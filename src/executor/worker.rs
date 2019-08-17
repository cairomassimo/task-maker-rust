use crate::execution::*;
use crate::executor::*;
use task_maker_store::*;
use failure::{Error, Fail};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

/// The identifier of a worker, it's globally unique and identifies the worker
/// during a single connection.
pub type WorkerUuid = Uuid;

/// The information about the current job the worker is doing
struct WorkerCurrentJob {
    /// Job currently waiting for, when there is a job running this should be
    /// None
    current_job: Option<Box<WorkerJob>>,
    /// Currently running sandbox
    current_sandbox: Option<Sandbox>,
    /// The number of missing files that are required for the sandbox setup
    missing_deps: usize,
}

/// The worker is the component that receives the work from the server and
/// sends the results back.
pub struct Worker {
    /// The identifier of this worker
    uuid: WorkerUuid,
    /// The name of this worker
    name: String,
    /// The channel that sends messages to the server
    sender: ChannelSender,
    /// The channel that receives messages from the server
    receiver: ChannelReceiver,
    /// A reference to the FileStore
    file_store: Arc<Mutex<FileStore>>,
    /// Job the worker is currently workin on
    current_job: Arc<Mutex<WorkerCurrentJob>>,
}

/// An handle of the connection to the worker
pub struct WorkerConn {
    /// The identifier of the worker
    pub uuid: WorkerUuid,
    /// The name of the worker
    pub name: String,
    /// The channel that sends messages to the worker
    pub sender: ChannelSender,
    /// The channel that receives messages from the server
    pub receiver: ChannelReceiver,
}

/// An error generated by the worker
#[derive(Debug, Fail)]
pub enum WorkerError {
    #[fail(display = "missing key for dependency {}", uuid)]
    MissingDependencyKey { uuid: Uuid },
}

impl WorkerCurrentJob {
    /// Make a new WorkerCurrentJob
    fn new() -> WorkerCurrentJob {
        WorkerCurrentJob {
            current_job: None,
            current_sandbox: None,
            missing_deps: 0,
        }
    }

    /// Keeps track that a new dependency is ready, will panic if no new file
    /// were expected. Will return true if all the deps are ready
    fn dependency_received(&mut self) -> bool {
        assert!(
            self.missing_deps > 0,
            "A new dep is ready but no deps were waiting"
        );
        self.missing_deps -= 1;
        self.missing_deps == 0
    }
}

impl Worker {
    /// Make a new worker attached to a FileStore, will return a pair with the
    /// actual Worker and an handle with the channels to connect to communicate
    /// with the worker.
    pub fn new(name: &str, file_store: Arc<Mutex<FileStore>>) -> (Worker, WorkerConn) {
        let (tx, rx_worker) = channel();
        let (tx_worker, rx) = channel();
        let uuid = Uuid::new_v4();
        (
            Worker {
                uuid,
                name: name.to_owned(),
                sender: tx_worker,
                receiver: rx_worker,
                file_store,
                current_job: Arc::new(Mutex::new(WorkerCurrentJob::new())),
            },
            WorkerConn {
                uuid,
                name: name.to_owned(),
                sender: tx,
                receiver: rx,
            },
        )
    }

    /// The worker body, this function will block.
    pub fn work(self) -> Result<(), Error> {
        trace!("Worker {} ready, asking for work", self);
        serialize_into(&WorkerClientMessage::GetWork, &self.sender)?;

        let start_job = || -> Result<(), Error> {
            let mut store = self.file_store.lock().unwrap();
            let sandbox = execute_job(self.current_job.clone(), &self.sender, &mut store)?;
            self.current_job.lock().unwrap().current_sandbox = Some(sandbox);
            Ok(())
        };

        loop {
            let message = deserialize_from::<WorkerServerMessage>(&self.receiver);
            match message {
                Ok(WorkerServerMessage::Work(job)) => {
                    trace!("Worker {} got job: {:?}", self, job);
                    assert!(self.current_job.lock().unwrap().current_job.is_none());
                    let mut missing_deps = 0;
                    for input in job.execution.dependencies().iter() {
                        let mut store = self.file_store.lock().unwrap();
                        let key = job
                            .dep_keys
                            .get(&input)
                            .ok_or(WorkerError::MissingDependencyKey { uuid: *input })?;
                        if !store.has_key(&key) {
                            serialize_into(&WorkerClientMessage::AskFile(*input), &self.sender)?;
                            missing_deps += 1;
                        }
                    }
                    {
                        let mut current_job = self.current_job.lock().unwrap();
                        current_job.missing_deps = missing_deps;
                        current_job.current_job = Some(job);
                    }
                    if missing_deps == 0 {
                        start_job()?;
                    }
                }
                Ok(WorkerServerMessage::ProvideFile(uuid, key)) => {
                    info!("Server sent file {} {:?}", uuid, key);
                    let mut store = self.file_store.lock().unwrap();
                    let reader = ChannelFileIterator::new(&self.receiver);
                    store.store(&key, reader)?;
                    if self.current_job.lock().unwrap().dependency_received() {
                        start_job()?;
                    }
                }
                Err(e) => {
                    let cause = e.find_root_cause().to_string();
                    if cause == "receiving on a closed channel" {
                        trace!("Connection closed: {}", cause);
                        if let Some(sandbox) =
                            self.current_job.lock().unwrap().current_sandbox.as_ref()
                        {
                            sandbox.kill();
                        }
                        break;
                    } else {
                        error!("Connection error: {}", cause);
                    }
                }
            }
        }
        Ok(())
    }
}

/// Spawn a new thread that will start the sandbox and will send the results
/// back to the server
fn execute_job(
    current_job: Arc<Mutex<WorkerCurrentJob>>,
    sender: &ChannelSender,
    file_store: &mut FileStore,
) -> Result<Sandbox, Error> {
    let job = current_job.lock().unwrap().current_job.clone().unwrap();
    let sandbox = Sandbox::new(
        std::path::Path::new("/tmp/sandboxes"),
        &job.execution,
        &job.dep_keys,
        file_store,
    )?;
    let thread_sender = sender.clone();
    let thread_sandbox = sandbox.clone();
    let thread_job = job.clone();
    thread::Builder::new()
        .name(format!("Sandbox of {}", job.execution.description))
        .spawn(move || {
            let sender = thread_sender;
            let sandbox = thread_sandbox;
            let job = thread_job;

            let result = sandbox.run().unwrap();
            let result = compute_execution_result(&job.execution, result);
            let status = result.status.clone();

            serialize_into(
                &WorkerClientMessage::WorkerDone(WorkerResult { result }),
                &sender,
            )
            .unwrap();

            let send_file = |uuid: FileUuid, path: PathBuf| {
                serialize_into(
                    &WorkerClientMessage::ProvideFile(
                        uuid,
                        FileStoreKey::from_file(&path).unwrap(),
                    ),
                    &sender,
                )
                .unwrap();
                ChannelFileSender::send(&path, &sender).unwrap();
            };

            if let ExecutionStatus::Success = status {
                if let Some(stdout) = job.execution.stdout {
                    let path = sandbox.stdout_path();
                    send_file(stdout.uuid, path);
                }
                if let Some(stderr) = job.execution.stderr {
                    let path = sandbox.stderr_path();
                    send_file(stderr.uuid, path);
                }
                for (path, file) in job.execution.outputs.iter() {
                    let path = sandbox.output_path(Path::new(path));
                    send_file(file.uuid, path);
                }
            }
            current_job.lock().unwrap().current_job = None;
            current_job.lock().unwrap().current_sandbox = None;
            serialize_into(&WorkerClientMessage::GetWork, &sender).unwrap();
        })?;
    Ok(sandbox)
}

/// Compute the ExecutionResult based on the result of the sandbox
fn compute_execution_result(execution: &Execution, result: SandboxResult) -> ExecutionResult {
    match result {
        SandboxResult::Success {
            exit_status,
            signal,
            resources,
        } => ExecutionResult {
            uuid: execution.uuid,
            status: compute_execution_status(execution, exit_status, signal, &resources),
            resources,
        },
        SandboxResult::Failed { error } => ExecutionResult {
            uuid: execution.uuid,
            status: ExecutionStatus::InternalError(error.to_string()),
            resources: ExecutionResourcesUsage {
                cpu_time: 0.0,
                sys_time: 0.0,
                wall_time: 0.0,
                memory: 0,
            },
        },
    }
}

/// Compute the ExecutionStatus based on the result of the sandbox, checking
/// the signals, the return code and the time/memory constraints
fn compute_execution_status(
    execution: &Execution,
    exit_status: u32,
    signal: Option<u32>,
    resources: &ExecutionResourcesUsage,
) -> ExecutionStatus {
    // it's important to check those before the signals because exceeding those
    // limits may trigger a SIGKILL from the sandbox
    if let Some(cpu_time_limit) = execution.limits.cpu_time {
        if resources.cpu_time > cpu_time_limit {
            return ExecutionStatus::TimeLimitExceeded;
        }
    }
    if let Some(sys_time_limit) = execution.limits.sys_time {
        if resources.sys_time > sys_time_limit {
            return ExecutionStatus::SysTimeLimitExceeded;
        }
    }
    if let Some(wall_time_limit) = execution.limits.wall_time {
        if resources.wall_time > wall_time_limit {
            return ExecutionStatus::WallTimeLimitExceeded;
        }
    }
    if let Some(memory_limit) = execution.limits.memory {
        if resources.memory > memory_limit {
            return ExecutionStatus::MemoryLimitExceeded;
        }
    }
    if let Some(signal) = signal {
        return ExecutionStatus::Signal(signal, strsignal(signal));
    }
    if exit_status != 0 {
        return ExecutionStatus::ReturnCode(exit_status);
    }
    ExecutionStatus::Success
}

impl std::fmt::Display for WorkerConn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "'{}' ({})", self.name, self.uuid)
    }
}

impl std::fmt::Display for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "'{}' ({})", self.name, self.uuid)
    }
}
