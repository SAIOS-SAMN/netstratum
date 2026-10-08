//! The proposer on the court's own tasks, offline: how many it closes at home
//! (the judge's own steps on the train pairs), and its time per task.
//!   cargo run --release -p ns-hypha --example propose_court [-- <dir> <tries>]
use std::time::Instant;
type G = Vec<Vec<u8>>;

fn grid(lines: &[&str]) -> G {
    lines.iter().map(|l| l.bytes().map(|b| b - b'0').collect()).collect()
}

fn read(text: &str) -> Option<(Vec<(G, G)>, Vec<G>)> {
    let lines: Vec<&str> = text.lines().collect();
    let (mut train, mut tests, mut i) = (Vec::new(), Vec::new(), 0);
    while i < lines.len() {
        match lines[i] {
            "train" => {
                let sep = lines[i + 1..].iter().position(|l| *l == "--")? + i + 1;
                let end = lines[sep + 1..].iter().position(|l| *l == "train" || *l == "test").map_or(lines.len(), |p| p + sep + 1);
                train.push((grid(&lines[i + 1..sep]), grid(&lines[sep + 1..end])));
                i = end;
            }
            "test" => {
                let end = lines[i + 1..].iter().position(|l| *l == "train" || *l == "test").map_or(lines.len(), |p| p + i + 1);
                tests.push(grid(&lines[i + 1..end]));
                i = end;
            }
            _ => i += 1,
        }
    }
    Some((train, tests))
}

fn main() {
    let dir = std::env::args().nth(1).unwrap_or_else(|| "/opt/netstratum/var/uplink/tasks".into());
    let tries: usize = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(400);
    let secs: u64 = std::env::args().nth(3).and_then(|s| s.parse().ok()).unwrap_or(20);
    let limit: usize = std::env::args().nth(4).and_then(|s| s.parse().ok()).unwrap_or(40);
    let mut files: Vec<_> = std::fs::read_dir(&dir).expect("archive").flatten().map(|e| e.path()).collect();
    files.sort();
    let (mut tasks, mut closed) = (0, 0);
    let started = Instant::now();
    for path in files.into_iter().take(limit) {
        let Some((train, tests)) = std::fs::read_to_string(&path).ok().and_then(|t| read(&t)) else { continue };
        tasks += 1;
        let at = Instant::now();
        let got = ns_hypha::propose::propose_within(&train, &tests, tries, std::time::Duration::from_secs(secs));
        let secs = at.elapsed().as_secs_f64();
        if let Some(p) = &got {
            closed += 1;
            println!("{:.2}s CLOSED {:?}", secs, p.steps);
        } else {
            println!("{:.2}s -", secs);
        }
    }
    println!("tasks {tasks}: closed at home by the proposer {closed}; total {:.1}s", started.elapsed().as_secs_f64());
}
