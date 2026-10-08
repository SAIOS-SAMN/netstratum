//! THE MINDS ON THE COURT'S OWN TASKS, offline: every task the seat kept
//! (`var/uplink/tasks/*.task`), thought by netstratum's reduce organ and by
//! restore 10's object perception; how many each closes at home.
//!   cargo run --release -p ns-hypha --example court_tasks [-- <dir>]
use solver::{think_task, Grid, SolveRead, Task, TaskPair};

fn grid(lines: &[&str]) -> Option<Grid> {
    let rows = lines.len();
    let cols = lines.first()?.len();
    let cells: Vec<u8> = lines.iter().flat_map(|l| l.bytes().map(|b| b - b'0')).collect();
    Grid::new(rows, cols, cells)
}

fn read(text: &str) -> Option<(Vec<TaskPair>, Vec<Grid>)> {
    let mut train = Vec::new();
    let mut tests = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        match lines[i] {
            "train" => {
                let sep = lines[i + 1..].iter().position(|l| *l == "--")? + i + 1;
                let end = lines[sep + 1..].iter().position(|l| *l == "train" || *l == "test").map_or(lines.len(), |p| p + sep + 1);
                train.push(TaskPair { input: grid(&lines[i + 1..sep])?, output: grid(&lines[sep + 1..end])? });
                i = end;
            }
            "test" => {
                let end = lines[i + 1..].iter().position(|l| *l == "train" || *l == "test").map_or(lines.len(), |p| p + i + 1);
                tests.push(grid(&lines[i + 1..end])?);
                i = end;
            }
            _ => i += 1,
        }
    }
    Some((train, tests))
}

fn main() {
    let dir = std::env::args().nth(1).unwrap_or_else(|| "var/uplink/tasks".into());
    let (mut tasks, mut organ, mut objects, mut either, mut unread) = (0, 0, 0, 0, 0);
    let mut why: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for entry in std::fs::read_dir(&dir).expect("the task archive").flatten() {
        let Ok(text) = std::fs::read_to_string(entry.path()) else { continue };
        let Some((train, tests)) = read(&text) else { unread += 1; continue };
        let Some(test_in) = tests.first().cloned() else { continue };
        tasks += 1;
        let pairs: Vec<(Grid, Grid)> = train.iter().map(|p| (p.input.clone(), p.output.clone())).collect();
        let task = Task { name: "court".into(), train, test_in, test_expected: None };
        let by_organ = matches!(think_task(&task, &mut Vec::new()), SolveRead::Solved { .. });
        let by_objects = solver::percept::learn(&pairs).is_some();
        organ += usize::from(by_organ);
        objects += usize::from(by_objects);
        either += usize::from(by_organ || by_objects);
        *why.entry(solver::percept::why_not(&pairs)).or_insert(0usize) += 1;
    }
    for (w, n) in &why {
        println!("  {n:3}  {w}");
    }
    println!("court tasks {tasks} (unread {unread}): closed at home by the reduce organ {organ}, by object perception {objects}, by either {either}");
}
