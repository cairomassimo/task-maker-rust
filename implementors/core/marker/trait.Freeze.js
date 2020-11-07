(function() {var implementors = {};
implementors["task_maker"] = [{"text":"impl Freeze for Evaluation","synthetic":true,"types":[]},{"text":"impl Freeze for Opt","synthetic":true,"types":[]},{"text":"impl Freeze for ServerOptions","synthetic":true,"types":[]},{"text":"impl Freeze for WorkerOptions","synthetic":true,"types":[]},{"text":"impl Freeze for Remote","synthetic":true,"types":[]},{"text":"impl Freeze for Node","synthetic":true,"types":[]},{"text":"impl Freeze for SelfExecSandboxRunner","synthetic":true,"types":[]}];
implementors["task_maker_cache"] = [{"text":"impl Freeze for Cache","synthetic":true,"types":[]},{"text":"impl Freeze for CacheResult","synthetic":true,"types":[]}];
implementors["task_maker_dag"] = [{"text":"impl Freeze for ExecutionDAGConfig","synthetic":true,"types":[]},{"text":"impl Freeze for ExecutionDAGData","synthetic":true,"types":[]},{"text":"impl Freeze for ExecutionDAG","synthetic":true,"types":[]},{"text":"impl Freeze for ExecutionTag","synthetic":true,"types":[]},{"text":"impl Freeze for ExecutionInput","synthetic":true,"types":[]},{"text":"impl Freeze for ExecutionCallbacks","synthetic":true,"types":[]},{"text":"impl Freeze for Execution","synthetic":true,"types":[]},{"text":"impl Freeze for ExecutionLimits","synthetic":true,"types":[]},{"text":"impl Freeze for ExecutionResourcesUsage","synthetic":true,"types":[]},{"text":"impl Freeze for ExecutionResult","synthetic":true,"types":[]},{"text":"impl Freeze for Fifo","synthetic":true,"types":[]},{"text":"impl Freeze for ExecutionGroup","synthetic":true,"types":[]},{"text":"impl Freeze for WriteToCallback","synthetic":true,"types":[]},{"text":"impl Freeze for FileCallbacks","synthetic":true,"types":[]},{"text":"impl Freeze for File","synthetic":true,"types":[]},{"text":"impl Freeze for CacheMode","synthetic":true,"types":[]},{"text":"impl Freeze for ProvidedFile","synthetic":true,"types":[]},{"text":"impl Freeze for ExecutionCommand","synthetic":true,"types":[]},{"text":"impl Freeze for ExecutionStatus","synthetic":true,"types":[]}];
implementors["task_maker_exec"] = [{"text":"impl Freeze for ExecutorClient","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Freeze for ExecutorStatus&lt;T&gt;","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Freeze for ExecutorWorkerStatus&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Freeze for WorkerCurrentJobStatus&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Freeze for ErrorSandboxRunner","synthetic":true,"types":[]},{"text":"impl Freeze for SuccessSandboxRunner","synthetic":true,"types":[]},{"text":"impl Freeze for UnsafeSandboxRunner","synthetic":true,"types":[]},{"text":"impl Freeze for ClientInfo","synthetic":true,"types":[]},{"text":"impl !Freeze for Worker","synthetic":true,"types":[]},{"text":"impl !Freeze for WorkerConn","synthetic":true,"types":[]},{"text":"impl Freeze for RawSandboxResult","synthetic":true,"types":[]},{"text":"impl Freeze for LocalExecutor","synthetic":true,"types":[]},{"text":"impl Freeze for RemoteExecutor","synthetic":true,"types":[]},{"text":"impl Freeze for RemoteEntityMessage","synthetic":true,"types":[]},{"text":"impl Freeze for RemoteEntityMessageResponse","synthetic":true,"types":[]},{"text":"impl&lt;'a, T&gt; Freeze for ChannelFileIterator&lt;'a, T&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for ChannelFileSender","synthetic":true,"types":[]},{"text":"impl Freeze for ExecutorClientMessage","synthetic":true,"types":[]},{"text":"impl Freeze for ExecutorServerMessage","synthetic":true,"types":[]},{"text":"impl Freeze for WorkerClientMessage","synthetic":true,"types":[]},{"text":"impl Freeze for WorkerServerMessage","synthetic":true,"types":[]}];
implementors["task_maker_format"] = [{"text":"impl Freeze for SourceFile","synthetic":true,"types":[]},{"text":"impl Freeze for VALID_TAGS","synthetic":true,"types":[]},{"text":"impl Freeze for DATA_DIR","synthetic":true,"types":[]},{"text":"impl Freeze for EvaluationConfig","synthetic":true,"types":[]},{"text":"impl Freeze for EvaluationData","synthetic":true,"types":[]},{"text":"impl Freeze for Tag","synthetic":true,"types":[]},{"text":"impl Freeze for TaskInfo","synthetic":true,"types":[]},{"text":"impl Freeze for BatchTypeData","synthetic":true,"types":[]},{"text":"impl Freeze for CommunicationTypeData","synthetic":true,"types":[]},{"text":"impl Freeze for BookletConfig","synthetic":true,"types":[]},{"text":"impl Freeze for BookletTemplate","synthetic":true,"types":[]},{"text":"impl Freeze for Booklet","synthetic":true,"types":[]},{"text":"impl Freeze for StatementConfig","synthetic":true,"types":[]},{"text":"impl Freeze for Statement","synthetic":true,"types":[]},{"text":"impl Freeze for IOITaskInfo","synthetic":true,"types":[]},{"text":"impl Freeze for TaskInfoLimits","synthetic":true,"types":[]},{"text":"impl Freeze for TaskInfoAttachment","synthetic":true,"types":[]},{"text":"impl Freeze for TaskInfoSubtask","synthetic":true,"types":[]},{"text":"impl Freeze for TaskInfoScoring","synthetic":true,"types":[]},{"text":"impl Freeze for TaskInfoStatement","synthetic":true,"types":[]},{"text":"impl Freeze for TestcaseGenerationState","synthetic":true,"types":[]},{"text":"impl Freeze for SubtaskGenerationState","synthetic":true,"types":[]},{"text":"impl Freeze for SolutionTestcaseEvaluationState","synthetic":true,"types":[]},{"text":"impl Freeze for SolutionSubtaskEvaluationState","synthetic":true,"types":[]},{"text":"impl Freeze for SolutionEvaluationState","synthetic":true,"types":[]},{"text":"impl Freeze for BookletDependencyState","synthetic":true,"types":[]},{"text":"impl Freeze for BookletState","synthetic":true,"types":[]},{"text":"impl Freeze for UIState","synthetic":true,"types":[]},{"text":"impl Freeze for ScoreManager","synthetic":true,"types":[]},{"text":"impl Freeze for IOITask","synthetic":true,"types":[]},{"text":"impl Freeze for SubtaskInfo","synthetic":true,"types":[]},{"text":"impl Freeze for TestcaseInfo","synthetic":true,"types":[]},{"text":"impl Freeze for Checker","synthetic":true,"types":[]},{"text":"impl Freeze for InputGenerator","synthetic":true,"types":[]},{"text":"impl Freeze for InputValidator","synthetic":true,"types":[]},{"text":"impl Freeze for OutputGenerator","synthetic":true,"types":[]},{"text":"impl Freeze for TaskType","synthetic":true,"types":[]},{"text":"impl Freeze for TestcaseScoreAggregator","synthetic":true,"types":[]},{"text":"impl Freeze for TestcaseGenerationStatus","synthetic":true,"types":[]},{"text":"impl Freeze for TestcaseEvaluationStatus","synthetic":true,"types":[]},{"text":"impl Freeze for TerryTaskInfo","synthetic":true,"types":[]},{"text":"impl Freeze for TerryTask","synthetic":true,"types":[]},{"text":"impl Freeze for SolutionOutcome","synthetic":true,"types":[]},{"text":"impl Freeze for SolutionValidation","synthetic":true,"types":[]},{"text":"impl Freeze for SolutionValidationCase","synthetic":true,"types":[]},{"text":"impl Freeze for SolutionFeedback","synthetic":true,"types":[]},{"text":"impl Freeze for SolutionFeedbackCase","synthetic":true,"types":[]},{"text":"impl Freeze for SolutionAlert","synthetic":true,"types":[]},{"text":"impl Freeze for CaseStatus","synthetic":true,"types":[]},{"text":"impl Freeze for JsonUI","synthetic":true,"types":[]},{"text":"impl Freeze for PrintUI","synthetic":true,"types":[]},{"text":"impl Freeze for RawUI","synthetic":true,"types":[]},{"text":"impl Freeze for SilentUI","synthetic":true,"types":[]},{"text":"impl Freeze for RED","synthetic":true,"types":[]},{"text":"impl Freeze for GREEN","synthetic":true,"types":[]},{"text":"impl Freeze for YELLOW","synthetic":true,"types":[]},{"text":"impl Freeze for BLUE","synthetic":true,"types":[]},{"text":"impl Freeze for BOLD","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Freeze for FinishUIUtils&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl !Freeze for UIMessageSender","synthetic":true,"types":[]},{"text":"impl Freeze for UIMessage","synthetic":true,"types":[]},{"text":"impl Freeze for UIExecutionStatus","synthetic":true,"types":[]},{"text":"impl Freeze for CompilationStatus","synthetic":true,"types":[]},{"text":"impl Freeze for UIType","synthetic":true,"types":[]}];
implementors["task_maker_gen_autocompletion"] = [{"text":"impl Freeze for Opt","synthetic":true,"types":[]},{"text":"impl Freeze for ServerOptions","synthetic":true,"types":[]},{"text":"impl Freeze for WorkerOptions","synthetic":true,"types":[]},{"text":"impl Freeze for Remote","synthetic":true,"types":[]}];
implementors["task_maker_lang"] = [{"text":"impl Freeze for GraderMap","synthetic":true,"types":[]},{"text":"impl Freeze for Dependency","synthetic":true,"types":[]},{"text":"impl Freeze for SourceFile","synthetic":true,"types":[]},{"text":"impl Freeze for LanguageManager","synthetic":true,"types":[]}];
implementors["task_maker_rust"] = [{"text":"impl Freeze for Evaluation","synthetic":true,"types":[]},{"text":"impl Freeze for Opt","synthetic":true,"types":[]},{"text":"impl Freeze for ServerOptions","synthetic":true,"types":[]},{"text":"impl Freeze for WorkerOptions","synthetic":true,"types":[]},{"text":"impl Freeze for Remote","synthetic":true,"types":[]},{"text":"impl Freeze for SelfExecSandboxRunner","synthetic":true,"types":[]}];
implementors["task_maker_store"] = [{"text":"impl Freeze for ReadFileIterator","synthetic":true,"types":[]},{"text":"impl Freeze for FileStore","synthetic":true,"types":[]},{"text":"impl Freeze for FileStoreKey","synthetic":true,"types":[]},{"text":"impl Freeze for FileStoreHandle","synthetic":true,"types":[]}];
implementors["task_maker_test"] = [{"text":"impl Freeze for TestInterface","synthetic":true,"types":[]},{"text":"impl Freeze for TestInterfaceSuccessful","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()