initSidebarItems({"enum":[["Checker","Which tool to use to compute the score on a testcase given the input file, the correct output file and the output file to evaluate."],["CompilationStatus","The status of the compilation of a file."],["InputGenerator","The source of the input files. It can either be a statically provided input file or a custom command that will generate an input file."],["InputValidator","An input file validator is responsible for checking that the input file follows the format and constraints defined by the task."],["OutputGenerator","The source of the output files. It can either be a statically provided output file or a custom command that will generate an output file."],["Tag","Tags of the various executions inside a IOI task."],["TaskType","The type of the task. This changes the behaviour of the solutions."],["TestcaseEvaluationStatus","Status of the evaluation of a solution on a testcase."],["TestcaseGenerationStatus","Status of the generation of a testcase input and output."],["TestcaseScoreAggregator","The aggregator of testcase scores for computing the subtask score."]],"fn":[["make_booklets","Find all the `Booklet` it makes sense to build."]],"struct":[["Booklet","A `Booklet` is a pdf file containing the statements of some tasks. It is compiled from a series of `.tex` files defined by `Statement` objects. The compiled pdf file is then copied somewhere."],["BookletConfig","Configuration of a `Booklet`, including the setting from the contest configuration."],["BookletDependencyState","The status of the compilation of a dependency of a booklet."],["BookletState","The status of the compilation of a booklet."],["BookletTemplate","Template to use to render the `booklet.tex` file."],["PrintUI","A simple UI that will print to stdout the human readable messages. Useful for debugging or for when curses is not available."],["SolutionEvaluationState","State of the evaluation of a solution."],["SolutionSubtaskEvaluationState","State of the evaluation of a subtask."],["SolutionTestcaseEvaluationState","State of the evaluation of a testcase."],["Statement","A statement is a `.tex` file with all the other assets included in its directory."],["StatementConfig","The configuration of a `Statement`."],["SubtaskGenerationState","State of the generation of a subtask."],["SubtaskInfo","A subtask of a IOI task."],["Task","Information about a generic IOI task."],["TestcaseGenerationState","State of the generation of a testcases."],["TestcaseInfo","A testcase of a IOI task."],["UIState","The state of a IOI task, all the information for the UI are stored here."]],"type":[["SubtaskId","In IOI tasks the subtask numbers are non-negative 0-based integers."],["TestcaseId","In IOI tasks the testcase numbers are non-negative 0-based integers."]]});