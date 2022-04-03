use crate::dag::{ExecutionDAG, ExecutionDAGOptions, ExecutionGroup};
use crate::error::Error;
use crate::store::FileSetHandle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerStatus {
    pub num_workers: usize,
    pub queue_length: usize,
    // TODO: add more
}

#[tarpc::service]
pub trait Server {
    /// Asks the server to evaluate the given DAG. All the input files must already be available in
    /// the Store.
    async fn evaluate(dag: ExecutionDAG, options: ExecutionDAGOptions) -> Result<(), Error>;

    /// Asks the server for work to do. Returns a FileSetHandle to be used to store the
    /// outputs in the Store. id is an identifier of the worker that calls the method.
    async fn get_work(id: usize) -> (ExecutionGroup, ExecutionDAGOptions, FileSetHandle);

    /// Asks the server whether the given computation should be cancelled. Returns iff the
    /// computation should be cancelled; otherwise the request is dropped.
    async fn is_cancelled(computation: FileSetHandle);

    /// Retrieves information about the status of the server.
    async fn get_status() -> ServerStatus;
}