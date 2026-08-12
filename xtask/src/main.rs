//! **THE BOARD GATE** — `cargo board`.
//!
//! Runs the whole workspace with `--no-fail-fast`, tallies per suite,
//! and compares the failing set against the KNOWN-RED ledger below.
//! Exit 0 = no new reds (known reds reported, never hidden). Exit 1 =
//! a new red entered the tree.
//!
//! Born 2026-08-12 from the fail-fast blindness: `cargo test` stops at
//! the first failing TARGET, and with one standing red every suite
//! ordered after it silently never ran — boards were reported green
//! that had skipped half the tree. This gate makes the red set a
//! COUNTED ledger (the ratchet's discipline): adding a red is a
//! reviewed edit here, wearing its owning task, never an accident.

mod reds;

use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .args(["test", "--workspace", "--no-fail-fast"])
        .output()
        .expect("cargo test runs");
    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    // Attribute each `test result:` line to the suite named by the
    // nearest preceding `Running` line.
    let mut suite = String::from("?");
    let mut ok = 0usize;
    let mut failed: Vec<String> = Vec::new();
    for line in text.lines() {
        if let Some(at) = line.find("Running ") {
            let path = &line[at + "Running ".len()..];
            suite = path
                .rsplit('/')
                .next()
                .unwrap_or(path)
                .split('-')
                .next()
                .unwrap_or(path)
                .trim_start_matches("tests/")
                .to_string();
            if let Some(name) = path.split('(').next() {
                if let Some(stem) = name.trim().rsplit('/').next() {
                    suite = stem.trim_end_matches(".rs").to_string();
                }
            }
        }
        if line.contains("test result: ok.") {
            ok += 1;
        }
        if line.contains("test result: FAILED.") {
            failed.push(suite.clone());
        }
    }
    failed.sort();
    failed.dedup();

    let known: Vec<&str> = reds::KNOWN_REDS.iter().map(|(s, _)| *s).collect();
    let new_reds: Vec<&String> = failed.iter().filter(|s| !known.contains(&s.as_str())).collect();
    let cured: Vec<&&str> = known.iter().filter(|s| !failed.iter().any(|f| f == **s)).collect();

    println!("board: {ok} suites ok · {} failing", failed.len());
    for suite in &failed {
        let owner = reds::KNOWN_REDS
            .iter()
            .find(|(s, _)| s == suite)
            .map(|(_, task)| *task);
        match owner {
            Some(task) => println!("  known red: {suite}  ({task})"),
            None => println!("  NEW RED:   {suite}"),
        }
    }
    for suite in &cured {
        println!("  cured: {suite} — remove it from xtask/src/reds.rs in this change");
    }
    if new_reds.is_empty() {
        println!("board: no new reds");
    } else {
        println!("board: {} NEW red(s) — fix them or ledger them in review", new_reds.len());
        std::process::exit(1);
    }
}
