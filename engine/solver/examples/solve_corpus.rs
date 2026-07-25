//! Run the harness over a corpus directory of pair-format tasks and
//! report the measured record — solved = program closed every training
//! pair AND the rendered test output matched the archived answer.
//!
//! Usage: cargo run -p solver --release --example solve_corpus -- <dir> [max_depth]

use solver::{load_task, solve, verify_certificate, CertificateRead, SolveRead};

fn main() {
    let mut args = std::env::args().skip(1);
    let dir = args.next().expect("usage: solve_corpus <corpus_dir> [max_depth]");
    let max_depth: usize = args.next().map(|d| d.parse().expect("depth")).unwrap_or(3);

    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("corpus dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "txt"))
        .collect();
    paths.sort();

    let (mut attempted, mut closed, mut confirmed) = (0u32, 0u32, 0u32);
    for path in &paths {
        let task = match load_task(path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("skip {path:?}: {e:?}");
                continue;
            }
        };
        attempted += 1;
        if let SolveRead::Solved {
            program,
            test_output,
            matches_expected,
            certificate,
        } = solve(&task, max_depth)
        {
            closed += 1;
            assert_eq!(
                verify_certificate(&certificate, &task, &program, &test_output),
                CertificateRead::Intact,
                "every emitted certificate must replay"
            );
            if matches_expected == Some(true) {
                confirmed += 1;
                let word: Vec<String> = program.iter().map(|op| op.describe()).collect();
                println!("SOLVED {} via [{}]", task.name, word.join(" → "));
            } else {
                println!(
                    "closed-but-diverged {} (train closed; test render did not match)",
                    task.name
                );
            }
        }
    }
    println!(
        "\nrecord: {attempted} attempted · {closed} closed training · {confirmed} confirmed against held-out answers (depth {max_depth})"
    );
}
