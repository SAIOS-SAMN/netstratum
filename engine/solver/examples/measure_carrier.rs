//! **THE CARRIER MEASUREMENT** (P3 → #35): for every task the depth-3
//! sweep CONFIRMS, emit the pricing row the carrier table pins —
//! task_id, VisitedNodes₃ = frontier(vocabulary, 3), the found word's
//! length, and blind membership (second arg: the blind dir).
//!
//!     measure_carrier <corpus_dir> <blind_dir>

use solver::{frontier, load_task, solve, sweep_vocabulary, SolveRead};

fn main() {
    let mut args = std::env::args().skip(1);
    let dir = args.next().expect("usage: measure_carrier <corpus_dir> <blind_dir>");
    let blind_dir = args.next().expect("blind dir");
    let blind: std::collections::HashSet<String> = std::fs::read_dir(&blind_dir)
        .expect("blind dir")
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .collect();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("corpus dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "txt"))
        .collect();
    paths.sort();
    for path in &paths {
        let Ok(task) = load_task(path) else { continue };
        if let SolveRead::Solved { program, matches_expected: Some(true), .. } =
            solve(&task, 3)
        {
            let id = solver_verify::corpus::task_digest(&task.train, &task.test_in);
            let visited = frontier(sweep_vocabulary(&task).len(), 3);
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            println!(
                "ROW {} {} {} {}",
                hex(&id),
                visited,
                program.len().max(1),
                blind.contains(&name) as u8
            );
        }
    }
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
