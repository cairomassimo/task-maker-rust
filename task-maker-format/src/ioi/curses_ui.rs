use std::path::Path;

use itertools::Itertools;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};

use task_maker_dag::ExecutionStatus;

use crate::ioi::finish_ui::{FinishUI, YELLOW_RESOURCE_THRESHOLD};
use crate::ioi::{
    SolutionTestcaseEvaluationState, SubtaskId, TestcaseEvaluationStatus, TestcaseGenerationStatus,
    UIState,
};
use crate::ui::curses::{
    compilation_status_text, draw_compilations, inner_block, render_block, render_server_status,
    CursesDrawer, CursesUI as GenericCursesUI, FrameType, GREEN, ORANGE, RED, YELLOW,
};
use crate::ui::UIExecutionStatus;

/// An animated UI for IOI tasks, dynamically refreshing using curses as a backend.
pub(crate) type CursesUI = GenericCursesUI<UIState, Drawer, FinishUI>;

/// The drawer of the IOI CursesUI.
pub(crate) struct Drawer;

impl CursesDrawer<UIState> for Drawer {
    fn draw(state: &UIState, frame: FrameType, loading: char, frame_index: usize) {
        draw_frame(state, frame, loading, frame_index);
    }
}

/// Draw a frame of interface to the provided `Frame`.
fn draw_frame(state: &UIState, mut f: FrameType, loading: char, frame_index: usize) {
    let size = f.size();
    if size.width < 16 || size.height < 16 {
        let error = Text::styled("Too small", Style::default().modifier(Modifier::BOLD));
        Paragraph::new([error].iter())
            .wrap(false)
            .render(&mut f, size);
        return;
    }
    let header = [
        Text::styled(
            state.task.title.clone(),
            Style::default().modifier(Modifier::BOLD),
        ),
        Text::raw(" ("),
        Text::raw(state.task.name.clone()),
        Text::raw(")\n"),
    ];
    let header_len = 2;
    let num_compilations = state
        .compilations
        .iter()
        .filter(|(k, _)| !state.evaluations.contains_key(*k))
        .count();
    let compilations_len = if num_compilations > 0 {
        num_compilations as u16 + 2
    } else {
        0
    };
    let booklet_len = if state.booklets.is_empty() {
        0
    } else {
        state
            .booklets
            .values()
            .map(|s| s.dependencies.len() as u16 + 1)
            .sum::<u16>()
            + 2
    };
    let generations_len = if state.generations.is_empty() { 0 } else { 3 };
    let evaluations_len = state.evaluations.len() as u16 + 2;
    let mut workers_len = state
        .executor_status
        .as_ref()
        .map(|s| s.connected_workers.len())
        .unwrap_or(0) as u16
        + 2;
    let total_height = f.size().height;
    // fixed size section heights
    let top_height = header_len + compilations_len + booklet_len + generations_len;
    // if the sections don't just fit, reduce the size of the workers until they fit but
    // without shortening it more than 3 lines (aka box + 1 worker).
    if top_height + evaluations_len + workers_len > total_height {
        workers_len = std::cmp::max(
            3,
            total_height as i16 - top_height as i16 - evaluations_len as i16,
        ) as u16;
    }
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(header_len),
                Constraint::Length(compilations_len),
                Constraint::Length(booklet_len),
                Constraint::Length(generations_len),
                Constraint::Min(0),
                Constraint::Length(workers_len),
            ]
            .as_ref(),
        )
        .split(f.size());
    Paragraph::new(header.iter())
        .block(Block::default().borders(Borders::NONE))
        .wrap(false)
        .render(&mut f, chunks[0]);
    if compilations_len > 0 {
        render_block(&mut f, chunks[1], " Compilations ");
        draw_compilations(
            &mut f,
            inner_block(chunks[1]),
            state
                .compilations
                .iter()
                .filter(|(p, _)| !state.evaluations.contains_key(*p)),
            loading,
        );
    }
    if !state.booklets.is_empty() {
        render_block(&mut f, chunks[2], " Statements ");
        draw_booklets(&mut f, inner_block(chunks[2]), state, loading);
    }
    if !state.generations.is_empty() {
        render_block(&mut f, chunks[3], " Generation ");
        draw_generations(&mut f, inner_block(chunks[3]), state, loading);
    }
    if !state.evaluations.is_empty() {
        render_block(&mut f, chunks[4], " Evaluations ");
        draw_evaluations(&mut f, inner_block(chunks[4]), state, loading);
    }
    render_server_status(
        &mut f,
        chunks[5],
        state.executor_status.as_ref(),
        loading,
        frame_index,
    );
}

/// Draw the content of the booklet box.
fn draw_booklets(frame: &mut FrameType, rect: Rect, state: &UIState, loading: char) {
    let text: Vec<Text> = state
        .booklets
        .keys()
        .sorted()
        .flat_map(|name| {
            let booklet = &state.booklets[name];
            let mut res = vec![
                Text::raw(format!("{:<20} ", name)),
                ui_execution_status_text(&booklet.status, loading),
                Text::raw("\n"),
            ];
            for name in booklet.dependencies.keys().sorted() {
                let dep = &booklet.dependencies[name];
                res.push(Text::raw(format!("  {:<18} ", name)));
                res.push(Text::raw("["));
                for step in dep.iter() {
                    res.push(ui_execution_status_text(&step.status, loading));
                }
                res.push(Text::raw("]\n"));
            }
            res
        })
        .collect();
    Paragraph::new(text.iter()).wrap(false).render(frame, rect);
}

fn ui_execution_status_text(status: &UIExecutionStatus, loading: char) -> Text {
    match status {
        UIExecutionStatus::Pending => Text::raw("."),
        UIExecutionStatus::Started { .. } => Text::raw(format!("{}", loading)),
        UIExecutionStatus::Skipped => Text::raw("S"),
        UIExecutionStatus::Done { result } => match &result.status {
            ExecutionStatus::Success => Text::styled("S", *GREEN),
            ExecutionStatus::InternalError(_) => Text::raw("I"),
            _ => Text::styled("F", *RED),
        },
    }
}

/// Draw the content of the generation box.
fn draw_generations(frame: &mut FrameType, rect: Rect, state: &UIState, loading: char) {
    let text: Vec<Text> = state
        .generations
        .iter()
        .sorted_by_key(|(k, _)| *k)
        .flat_map(|(_, subtask)| {
            let mut testcases: Vec<Text> = subtask
                .testcases
                .iter()
                .sorted_by_key(|(k, _)| *k)
                .map(|(_, tc)| generation_status_text(&tc.status, loading))
                .collect();
            let mut res = vec![Text::raw("[")];
            res.append(&mut testcases);
            res.push(Text::raw("]"));
            res
        })
        .collect();
    Paragraph::new(text.iter()).wrap(false).render(frame, rect);
}

/// Get the colored character corresponding to the status of the generation of a testcase.
fn generation_status_text(status: &TestcaseGenerationStatus, loading: char) -> Text {
    match status {
        TestcaseGenerationStatus::Pending => Text::raw("."),
        TestcaseGenerationStatus::Generating => Text::raw(format!("{}", loading)),
        TestcaseGenerationStatus::Generated => Text::styled("G", *GREEN),
        TestcaseGenerationStatus::Validating => Text::raw(format!("{}", loading)),
        TestcaseGenerationStatus::Validated => Text::styled("V", *GREEN),
        TestcaseGenerationStatus::Solving => Text::raw(format!("{}", loading)),
        TestcaseGenerationStatus::Solved => Text::styled("S", *GREEN),
        TestcaseGenerationStatus::Failed => Text::styled("F", *RED),
        TestcaseGenerationStatus::Skipped => Text::styled("s", *YELLOW),
    }
}

/// Draw the content of the evaluation box.
fn draw_evaluations(frame: &mut FrameType, rect: Rect, state: &UIState, loading: char) {
    let max_len = state
        .evaluations
        .keys()
        .map(|k| k.file_name().expect("Invalid file name").len())
        .max()
        .unwrap_or(0)
        + 4;
    let text: Vec<Text> = state
        .evaluations
        .keys()
        .sorted()
        .flat_map(|solution| {
            let mut texts = vec![Text::raw(format!(
                "{:<max_len$} ",
                solution
                    .file_name()
                    .expect("Invalid file name")
                    .to_string_lossy(),
                max_len = max_len
            ))];
            if let Some(comp_status) = state.compilations.get(solution) {
                texts.push(compilation_status_text(comp_status, loading));
            } else {
                texts.push(Text::raw("    "));
            }
            texts.push(Text::raw(" "));
            texts.push(evaluation_score(state, solution, loading));
            texts.append(&mut evaluation_line(state, solution, loading));
            texts.push(Text::raw("\n"));
            texts
        })
        .collect();
    Paragraph::new(text.iter()).wrap(false).render(frame, rect);
}

/// Get the colored score of a solution.
fn evaluation_score<'a>(state: &'a UIState, solution: &Path, loading: char) -> Text<'a> {
    let sol_state = if let Some(state) = state.evaluations.get(solution) {
        state
    } else {
        return Text::raw("  ?  ");
    };
    if let Some(score) = sol_state.score {
        if score == 0.0 {
            Text::styled(format!(" {:>3.0} ", score), *RED)
        } else if (score - state.max_score).abs() < 0.001 {
            Text::styled(format!(" {:>3.0} ", score), *GREEN)
        } else {
            Text::styled(format!(" {:>3.0} ", score), *YELLOW)
        }
    } else {
        let has_skipped = sol_state.subtasks.values().any(|st| {
            st.testcases
                .values()
                .any(|tc| tc.status == TestcaseEvaluationStatus::Skipped)
        });
        if has_skipped {
            Text::raw("  X  ")
        } else {
            Text::raw(format!("  {}  ", loading))
        }
    }
}

/// Get the line at the right of the score of a solution.
fn evaluation_line<'a>(state: &'a UIState, solution: &Path, loading: char) -> Vec<Text<'a>> {
    state
        .task
        .subtasks
        .keys()
        .sorted()
        .flat_map(|st| subtask_evaluation_status_text(state, solution, *st, loading))
        .collect()
}

/// Get the status of a subtask, like `[AATTR]` where each letter corresponds to
/// the status of a single testcase.
fn subtask_evaluation_status_text<'a>(
    state: &'a UIState,
    solution: &Path,
    subtask: SubtaskId,
    loading: char,
) -> Vec<Text<'a>> {
    let mut texts = vec![];
    let solution = &state.evaluations[solution];
    if !solution.subtasks.contains_key(&subtask) {
        return vec![Text::raw("[---]")];
    }
    let subtask = &solution.subtasks[&subtask];
    let par_style = if let Some(normalized_score) = subtask.normalized_score {
        if abs_diff_eq!(normalized_score, 1.0) {
            *GREEN
        } else if abs_diff_eq!(normalized_score, 0.0) {
            *RED
        } else {
            *YELLOW
        }
    } else {
        Style::default()
    };
    texts.push(Text::styled("[", par_style));
    for (_, testcase) in subtask.testcases.iter().sorted_by_key(|(k, _)| *k) {
        texts.push(testcase_evaluation_status_text(testcase, loading, state));
    }
    texts.push(Text::styled("]", par_style));
    texts
}

/// Get the colored character corresponding to the status of the evaluation of a testcase.
fn testcase_evaluation_status_text<'a>(
    testcase: &'a SolutionTestcaseEvaluationState,
    loading: char,
    state: &'a UIState,
) -> Text<'a> {
    let time_limit = state.task.time_limit;
    let memory_limit = state.task.memory_limit;
    let extra_time = state.config.extra_time;
    let close_color = if testcase.is_close_to_limits(
        time_limit,
        extra_time,
        memory_limit,
        YELLOW_RESOURCE_THRESHOLD,
    ) {
        Some(*ORANGE)
    } else {
        None
    };
    match &testcase.status {
        TestcaseEvaluationStatus::Pending => Text::raw("."),
        TestcaseEvaluationStatus::Solving => Text::raw(format!("{}", loading)),
        TestcaseEvaluationStatus::Solved => Text::raw("s"),
        TestcaseEvaluationStatus::Checking => Text::raw(format!("{}", loading)),
        TestcaseEvaluationStatus::Accepted(_) => Text::styled("A", close_color.unwrap_or(*GREEN)),
        TestcaseEvaluationStatus::WrongAnswer(_) => Text::styled("W", *RED),
        TestcaseEvaluationStatus::Partial(_) => Text::styled("P", *YELLOW),
        TestcaseEvaluationStatus::TimeLimitExceeded => {
            Text::styled("T", close_color.unwrap_or(*RED))
        }
        TestcaseEvaluationStatus::WallTimeLimitExceeded => Text::styled("T", *RED),
        TestcaseEvaluationStatus::MemoryLimitExceeded => {
            Text::styled("M", close_color.unwrap_or(*RED))
        }
        TestcaseEvaluationStatus::RuntimeError => Text::styled("R", *RED),
        TestcaseEvaluationStatus::Failed => Text::styled(
            "F",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Red)
                .modifier(Modifier::BOLD),
        ),
        TestcaseEvaluationStatus::Skipped => Text::raw("X"),
    }
}
