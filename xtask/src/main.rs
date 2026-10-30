//! **The board gate** — `cargo board`.
//!
//! Runs the whole workspace with `--no-fail-fast`, tallies per suite,
//! and compares the failing set against the KNOWN-red ledger below.
//! Exit 0 = no new reds (known reds reported, never hidden). Exit 1 =
//! a new red entered the tree.
//!
//! Born 2026-08-12 from the fail-fast blindness: `cargo test` stops at
//! the first failing target, and with one standing red every suite
//! ordered after it silently never ran — boards were reported green
//! that had skipped half the tree. This gate makes the red set a
//! counted ledger (the ratchet's discipline): adding a red is a
//! reviewed edit here, wearing its owning task, never an accident.

mod reds;

use std::process::Command;

fn main() {
    // One stream: cargo names targets on stderr and prints results on
    // stdout, and attribution needs their true interleaving — merge at
    // the shell, not by concatenation (which un-orders them).
    let output = Command::new("sh")
        .args(["-c", "cargo test --workspace --no-fail-fast 2>&1"])
        .output()
        .expect("cargo test runs");
    let text = String::from_utf8_lossy(&output.stdout).into_owned();

    // Attribute each `test result:` line to the target named by the
    // nearest preceding `Running …` / `Doc-tests …` line. The three
    // shapes cargo emits:
    //   Running tests/NAME.rs (target/…/deps/NAME-hash)
    //   Running unittests src/lib.rs (target/…/deps/CRATE-hash)
    //   Doc-tests crate
    let mut suite = String::from("?");
    let mut ok = 0usize;
    let mut failed: Vec<String> = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("Running ") {
            let head = rest.split(" (").next().unwrap_or(rest).trim();
            if let Some(stem) = head.strip_prefix("tests/") {
                suite = stem.trim_end_matches(".rs").to_string();
            } else {
                // unittests / bins: name by the deps binary's crate.
                let bin = rest
                    .rsplit('/')
                    .next()
                    .unwrap_or(rest)
                    .trim_end_matches(')');
                let crate_name = match bin.rsplit_once('-') {
                    Some((name, _hash)) => name,
                    None => bin,
                };
                suite = format!("{crate_name} (unittests)");
            }
        } else if let Some(crate_name) = trimmed.strip_prefix("Doc-tests ") {
            suite = format!("{} (doc-tests)", crate_name.trim());
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
