//! Mind-link, measured before wiring: Mycelium's own extractor, offline (no
//! session, no key), on every court task the seat kept — its time per task
//! under the budget a voice uses (`extraction_spend`), and whether the rule
//! it extracts closes every training pair when spoken back (`speak_rule`).
//!   cargo run --release -p ns-hypha --example mycelium_minds [-- <dir>]
use ns_hypha::mycelium_sdk::rule::Physics;
use ns_hypha::mycelium_sdk::rule::{self, NoCrystal};
use std::time::Instant;

type G = Vec<Vec<u8>>;

fn grid(lines: &[&str]) -> G {
    lines
        .iter()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn read(text: &str) -> Option<(Vec<(G, G)>, Vec<G>)> {
    let lines: Vec<&str> = text.lines().collect();
    let (mut train, mut tests, mut i) = (Vec::new(), Vec::new(), 0);
    while i < lines.len() {
        match lines[i] {
            "train" => {
                let sep = lines[i + 1..].iter().position(|l| *l == "--")? + i + 1;
                let end = lines[sep + 1..]
                    .iter()
                    .position(|l| *l == "train" || *l == "test")
                    .map_or(lines.len(), |p| p + sep + 1);
                train.push((grid(&lines[i + 1..sep]), grid(&lines[sep + 1..end])));
                i = end;
            }
            "test" => {
                let end = lines[i + 1..]
                    .iter()
                    .position(|l| *l == "train" || *l == "test")
                    .map_or(lines.len(), |p| p + i + 1);
                tests.push(grid(&lines[i + 1..end]));
                i = end;
            }
            _ => i += 1,
        }
    }
    Some((train, tests))
}

fn main() {
    let dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "var/uplink/tasks".into());
    let (mut tasks, mut extracted, mut closed) = (0, 0, 0);
    let mut slowest = 0f64;
    let started = Instant::now();
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("the task archive")
        .flatten()
        .map(|e| e.path())
        .collect();
    files.sort();
    for path in files {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Some((train, _)) = read(&text) else {
            continue;
        };
        tasks += 1;
        let frames = family::perceive_family(&train);
        let spend = family::extraction_spend(&train, &frames);
        let mut budget = Physics::default().stroke_n(spend);
        let at = Instant::now();
        let rule = rule::extract_rule(&train, &mut budget);
        let secs = at.elapsed().as_secs_f64();
        slowest = slowest.max(secs);
        let closes = rule.as_ref().is_some_and(|r| {
            train.iter().all(|(a, b)| {
                rule::speak_rule(r, a, &NoCrystal)
                    .and_then(|s| s.grid())
                    .as_ref()
                    == Some(b)
            })
        });
        extracted += usize::from(rule.is_some());
        closed += usize::from(closes);
        println!(
            "{:.2}s {} {}",
            secs,
            if rule.is_some() { "rule" } else { "none" },
            if closes { "CLOSES" } else { "" }
        );
    }
    println!("tasks {tasks}: a rule extracted {extracted}, closing every train pair {closed}; slowest {slowest:.2}s, total {:.1}s", started.elapsed().as_secs_f64());
}
