use crate::task_types::ioi::*;
use failure::{Error, bail};
use pest::Parser;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

#[derive(Parser)]
#[grammar = "task_types/ioi/formats/GEN.pest"]
struct GENParser;

pub type IOISubtasksInfo = HashMap<IOISubtaskId, IOISubtaskInfo>;
pub type IOITestcasesInfo = HashMap<IOISubtaskId, HashMap<IOITestcaseId, IOITestcaseInfo>>;

/// Parse the gen/GEN file extracting the subtasks and the testcases
pub fn parse_gen_gen(
    path: &Path,
    official_solution: Option<Arc<Solution<IOISubtaskId, IOITestcaseId>>>,
) -> Result<(IOISubtasksInfo, IOITestcasesInfo), Error> {
    let task_dir = path.parent().unwrap().parent().unwrap();
    let content = std::fs::read_to_string(path)?;
    let mut file = GENParser::parse(Rule::file, &content)?;
    let file = file.next().unwrap(); // extract the real file
    let mut subtasks = HashMap::new();
    let mut testcases = HashMap::new();
    let mut last_subtask = None;
    let mut last_testcases: HashMap<IOITestcaseId, IOITestcaseInfo> = HashMap::new();
    let mut testcase_count = 0;

    let mut default_subtask = Some(IOISubtaskInfo { max_score: 100.0 });

    let generator = find_source_file(
        &task_dir,
        vec![
            "gen/generator.*",
            "gen/generatore.*",
            "gen/generator",
            "gen/generatore",
        ],
        None,
    )
    .map(Arc::new);

    let validator = find_source_file(
        &task_dir,
        vec![
            "gen/validator.*",
            "gen/valida.*",
            "gen/validator",
            "gen/valida",
        ],
        None,
    )
    .map(Arc::new);

    let get_validator = |st: IOISubtaskId| {
        validator.as_ref().map(|v| {
            Arc::new(IOIValidator::new(
                v.clone(),
                vec!["input.txt".to_string(), st.to_string()],
            )) as Arc<Validator<IOISubtaskId, IOITestcaseId>>
        })
    };

    let get_solution = |st: IOITestcaseId| {
        if let Some(official) = official_solution.as_ref() {
            Ok(official.clone())
        } else {
            let static_file = task_dir.join("output").join(format!("output{}.txt", st));
            if !static_file.exists() {
                bail!("Static output file does not exists! {:?}", static_file);
            }
            Ok(Arc::new(StaticFileProvider::new(
                format!("Static output of testcase {}", st),
                static_file,
            )) as Arc<Solution<IOISubtaskId, IOITestcaseId>>)
        }
    };

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::line => {
                let line = line.into_inner().next().unwrap();
                match line.as_rule() {
                    Rule::subtask => {
                        if let Some(subtask) = last_subtask {
                            let subtask_id = subtasks.len() as IOISubtaskId;
                            subtasks.insert(subtask_id, subtask);
                            testcases.insert(subtask_id, last_testcases);
                            last_testcases = HashMap::new();
                        }
                        let score = line.into_inner().next().unwrap().as_str();
                        last_subtask = Some(IOISubtaskInfo {
                            max_score: score.parse::<f64>().unwrap(),
                        });
                    }
                    Rule::copy => {
                        if last_subtask.is_none() {
                            last_subtask = default_subtask.take();
                        }
                        let what = line.into_inner().next().unwrap().as_str();
                        last_testcases.insert(
                            testcase_count,
                            IOITestcaseInfo {
                                testcase: testcase_count,
                                static_output: None,
                                generator: Arc::new(StaticFileProvider::new(
                                    format!("Static input of testcase {}", testcase_count),
                                    task_dir.join(what),
                                )),
                                validator: get_validator(subtasks.len() as IOISubtaskId),
                                solution: get_solution(testcase_count)?,
                            },
                        );
                        testcase_count += 1;
                    }
                    Rule::command => {
                        if last_subtask.is_none() {
                            last_subtask = default_subtask.take();
                        }
                        let cmd: Vec<String> =
                            line.into_inner().map(|x| x.as_str().to_owned()).collect();
                        last_testcases.insert(
                            testcase_count,
                            IOITestcaseInfo {
                                testcase: testcase_count,
                                static_output: None,
                                generator: Arc::new(IOIGenerator::new(
                                    generator.clone().unwrap(),
                                    cmd,
                                )),
                                validator: get_validator(subtasks.len() as IOISubtaskId),
                                solution: get_solution(testcase_count)?,
                            },
                        );
                        testcase_count += 1;
                    }
                    Rule::comment => {}
                    Rule::empty => {}
                    _ => unreachable!(),
                }
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }
    let subtask_id = subtasks.len() as IOISubtaskId;
    subtasks.insert(subtask_id, last_subtask.unwrap());
    testcases.insert(subtask_id, last_testcases);
    Ok((subtasks, testcases))
}
