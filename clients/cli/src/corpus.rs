//! **THE CORPUS BENCH (#38a) — cognition as gameplay.**
//!
//! The economy's second head has been mintable but unreachable: the
//! carrier law prices a confirmed corpus solve (kernel 0.14.0, the
//! measured 34-row table), and nothing but the conductor's automaton
//! could ever submit one. This is the surface where a PLAYER — human
//! or agent — meets the same benchmark and earns from it.
//!
//! **WHAT THE GAME ACTUALLY IS.** A corpus proof is not an answer, it
//! is an EXPLANATION: a program that reproduces every training pair
//! and then produces the test output. `verify_corpus_work` re-hashes
//! the task, re-applies the program to all of it, and re-chains the
//! certificate — so typing the right grid earns nothing and cannot.
//! That makes the loop hypothesise → test → submit, which is the ARC
//! task as a puzzle rather than as a quiz.
//!
//! **ONLY PRICED TASKS ARE SERVED.** `carrier_bound` refuses an
//! unmeasured task `CarrierUnpriced`, so serving the whole 1243 would
//! be serving mostly dead ends. The bench serves exactly the measured
//! set — the tasks that pay — by content digest.

use kernel::carrier_table::MEASURED;
use solver_verify::{
    canonical_name, corpus::task_digest, encode_corpus_work, load_task, run, work::CorpusProof,
    Grid, Op, Program, SolveCertificate, Task, TaskPair, TileMirror,
};
use std::path::{Path, PathBuf};

/// Where the measured corpus lives. The carrier table names this path
/// as the population it was measured over; `NETSTRATUM_CORPUS`
/// overrides it for a machine that keeps the archive elsewhere.
pub fn corpus_dir() -> PathBuf {
    std::env::var("NETSTRATUM_CORPUS")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/opt/ipsal-autic-v0.4/tools/arc_full_corpus"))
}

/// One served task: the archive's own file, plus the identity the
/// carrier law prices it under.
pub struct Served {
    pub task: Task,
    pub task_id: [u8; 32],
    pub blind: bool,
}

/// **THE PRICED SET, found by CONTENT.** A task's identity is its
/// content, never its filename, so the only way to find the measured
/// tasks in the archive is to read and digest them. 1243 small files;
/// done once and held for the session.
///
/// Returns them in the measured table's own order, so `corpus <n>`
/// names the same task on every machine that holds the same archive.
pub fn priced(dir: &Path) -> Vec<Served> {
    let mut found: Vec<(usize, Served)> = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    for entry in entries.flatten() {
        let Ok(task) = load_task(&entry.path()) else {
            continue;
        };
        let task_id = task_digest(&task.train, &task.test_in);
        if let Some(at) = MEASURED.iter().position(|(id, _, _, _)| *id == task_id) {
            let blind = MEASURED[at].3;
            found.push((at, Served { task, task_id, blind }));
        }
    }
    found.sort_by_key(|(at, _)| *at);
    found.into_iter().map(|(_, served)| served).collect()
}

/// **THE WORD PARSER** — a player's program, written out.
///
/// Deliberately covers the operators a person can hold in their head
/// and type: the eight rigid motions and recolour, plus tiling. The
/// object-centric family (`Op::Object`) takes a segmentation argument
/// and is left to the solver until the bench has a way to SHOW what a
/// segmentation did — an operator nobody can see the effect of is not
/// a move a player can reason about.
pub fn parse_program(words: &[&str]) -> Result<Program, String> {
    let mut program = Vec::new();
    for word in words {
        let op = match *word {
            "rot90" => Op::Rotate90,
            "rot180" => Op::Rotate180,
            "rot270" => Op::Rotate270,
            "fliph" => Op::FlipH,
            "flipv" => Op::FlipV,
            "transpose" => Op::Transpose,
            held if held.starts_with("recolor:") => {
                // recolor:0>1,2>3 — every hue not named maps to itself.
                let mut map = [0u8; 10];
                for (hue, slot) in map.iter_mut().enumerate() {
                    *slot = hue as u8;
                }
                for term in held.trim_start_matches("recolor:").split(',') {
                    let Some((from, to)) = term.split_once('>') else {
                        return Err(format!("{term} is not a hue mapping (use 0>1)"));
                    };
                    match (from.trim().parse::<usize>(), to.trim().parse::<u8>()) {
                        (Ok(from), Ok(to)) if from < 10 && to < 10 => map[from] = to,
                        _ => return Err(format!("{term} names a hue outside 0–9")),
                    }
                }
                Op::Recolor(map)
            }
            held if held.starts_with("tile:") => {
                // tile:3x3 or tile:3x3:h:v — the mirrors are optional.
                let mut parts = held.trim_start_matches("tile:").split(':');
                let dims = parts.next().unwrap_or("");
                let Some((down, across)) = dims.split_once('x') else {
                    return Err(format!("{dims} is not a tiling (use 3x3)"));
                };
                let (Ok(down), Ok(across)) =
                    (down.trim().parse::<usize>(), across.trim().parse::<usize>())
                else {
                    return Err(format!("{dims} is not a tiling (use 3x3)"));
                };
                if down == 0 || across == 0 {
                    return Err("a tiling has at least one row and column".to_string());
                }
                let mirror = |word: Option<&str>| match word {
                    None | Some("") | Some("none") => Ok(TileMirror::None),
                    Some("rows") => Ok(TileMirror::Rows),
                    Some("cols") => Ok(TileMirror::Cols),
                    Some("both") => Ok(TileMirror::Both),
                    Some(other) => {
                        Err(format!("{other} is not a mirror (none|rows|cols|both)"))
                    }
                };
                Op::Tile {
                    down,
                    across,
                    odd_row: mirror(parts.next())?,
                    odd_col: mirror(parts.next())?,
                }
            }
            other => {
                return Err(format!(
                    "{other} is not a move — try rot90 rot180 rot270 fliph flipv transpose \
                     recolor:0>1 tile:3x3[:rows|cols|both][:…]"
                ))
            }
        };
        program.push(op);
    }
    Ok(program)
}

/// How a program fares on the training pairs — one verdict per pair,
/// in order. This is the whole feedback loop: the player sees exactly
/// which examples their hypothesis explains.
pub fn train_verdicts(task: &Task, program: &Program) -> Vec<bool> {
    task.train
        .iter()
        .map(|pair| run(program, &pair.input) == pair.output)
        .collect()
}

/// **SEAL THE PROVER'S HALF.** Identical in shape to the witness the
/// sealed math's own pins use: derive the identity from content, run
/// the program on the test input, and chain the certificate over the
/// task as the verifier will rebuild it (canonical name, no held-out
/// answer). What comes back is what `verify_corpus_work` re-checks.
pub fn seal(served: &Served, program: Program) -> CorpusProof {
    let output = run(&program, &served.task.test_in);
    let sealed = Task {
        name: canonical_name(&served.task_id),
        train: served.task.train.clone(),
        test_in: served.task.test_in.clone(),
        test_expected: None,
    };
    let certificate = SolveCertificate::seal(&sealed, &program, &output);
    CorpusProof {
        task_id: served.task_id,
        train: served.task.train.clone(),
        test_in: served.task.test_in.clone(),
        program,
        output,
        certificate,
    }
}

/// The payload the mint carries — opaque to `cfe` by law, built here
/// where the verifier lives.
pub fn payload(proof: &CorpusProof) -> Option<Vec<u8>> {
    encode_corpus_work(proof)
}

/// **HAS THIS WORLD ALREADY EARNED FROM THIS TASK?** (#38d.)
///
/// The once-per-task law, enforced at the EMITTER — the staleness
/// precedent, and for the same reason: the chronicle is what knows,
/// and the gate is handed committed state rather than receipts. `cfe`
/// may not parse a work payload (the opaque-payload law), so the
/// question is asked HERE, above the verifier, where reading it is
/// lawful.
///
/// **This is a real half-measure and is recorded as one.** A caller
/// that skips this check can still mint the same task twice; closing
/// that needs the solved set in committed state, which is a kernel
/// amendment and its own landing.
pub fn already_earned(chronicle: &cfe::vrp::Chronicle, task_id: &[u8; 32]) -> bool {
    chronicle.receipts().iter().any(|receipt| {
        let cfe::dop::ClosureEvent::DiscoveryMinted { work, .. } = receipt.event() else {
            return false;
        };
        matches!(
            solver_verify::work::decode_corpus_work(work),
            Some(held) if held.task_id == *task_id
        )
    })
}

/// Render a grid as coloured blocks — the ARC palette, so a task reads
/// as a picture rather than as a table of digits.
pub fn paint(grid: &Grid, color: bool) -> Vec<String> {
    (0..grid.rows)
        .map(|r| {
            (0..grid.cols)
                .map(|c| cell(grid.at(r, c), color))
                .collect::<Vec<String>>()
                .join("")
        })
        .collect()
}

fn cell(hue: u8, color: bool) -> String {
    if !color {
        return format!("{hue} ");
    }
    // The ARC hues, as 256-colour backgrounds. Hue 0 is the ground and
    // stays dark so shapes read against it.
    let code = match hue {
        0 => 234,
        1 => 27,
        2 => 196,
        3 => 46,
        4 => 226,
        5 => 245,
        6 => 201,
        7 => 208,
        8 => 51,
        9 => 88,
        _ => 15,
    };
    format!("\x1b[48;5;{code}m  \x1b[0m")
}

/// Two grids side by side with a marker between — how a training pair
/// is read.
pub fn beside(left: &Grid, right: &Grid, arrow: &str, color: bool) -> Vec<String> {
    let (a, b) = (paint(left, color), paint(right, color));
    let width = a.iter().map(|row| row.chars().count()).max().unwrap_or(0);
    let rows = a.len().max(b.len());
    let middle = rows / 2;
    (0..rows)
        .map(|r| {
            let blank = " ".repeat(width);
            let left = a.get(r).cloned().unwrap_or(blank);
            let joint = if r == middle { arrow } else { "   " };
            let right = b.get(r).cloned().unwrap_or_default();
            format!("{left}  {joint}  {right}")
        })
        .collect()
}

/// A pair's grids, for the bench's own rendering.
pub fn pair_grids(pair: &TaskPair) -> (&Grid, &Grid) {
    (&pair.input, &pair.output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use solver_verify::{verify_work_class, WorkBroken, WorkClass};

    fn grid(rows: usize, cols: usize, cells: &[u8]) -> Grid {
        Grid::new(rows, cols, cells.to_vec()).expect("a lawful grid")
    }

    /// A task a rigid motion explains: every pair is its input turned a
    /// quarter turn.
    fn turning_task() -> Task {
        let make = |cells: [u8; 4]| {
            let input = grid(2, 2, &cells);
            let output = run(&vec![Op::Rotate90], &input);
            TaskPair { input, output }
        };
        Task {
            name: "probe".into(),
            train: vec![make([1, 2, 3, 4]), make([5, 6, 7, 8]), make([9, 0, 1, 2])],
            test_in: grid(2, 2, &[3, 1, 4, 1]),
            test_expected: None,
        }
    }

    fn served_from(task: Task) -> Served {
        let task_id = task_digest(&task.train, &task.test_in);
        Served { task, task_id, blind: false }
    }

    #[test]
    fn the_parser_reads_every_move_a_player_can_type() {
        assert_eq!(
            parse_program(&["rot90", "rot180", "rot270", "fliph", "flipv", "transpose"]),
            Ok(vec![
                Op::Rotate90,
                Op::Rotate180,
                Op::Rotate270,
                Op::FlipH,
                Op::FlipV,
                Op::Transpose
            ])
        );
        // A recolour names only what MOVES; every other hue holds.
        let Ok(program) = parse_program(&["recolor:0>1,7>2"]) else {
            panic!("a lawful recolour");
        };
        let Op::Recolor(map) = program[0] else { panic!("a recolour") };
        assert_eq!(map[0], 1);
        assert_eq!(map[7], 2);
        assert_eq!(map[3], 3, "an unnamed hue maps to itself");
        // Tiling, with and without its mirrors.
        assert_eq!(
            parse_program(&["tile:3x3"]),
            Ok(vec![Op::Tile {
                down: 3,
                across: 3,
                odd_row: TileMirror::None,
                odd_col: TileMirror::None
            }])
        );
        assert_eq!(
            parse_program(&["tile:2x4:cols:rows"]),
            Ok(vec![Op::Tile {
                down: 2,
                across: 4,
                odd_row: TileMirror::Cols,
                odd_col: TileMirror::Rows
            }])
        );
    }

    #[test]
    fn the_parser_refuses_rather_than_guessing() {
        for bad in ["wiggle", "recolor:0", "recolor:0>77", "tile:zzz", "tile:0x3", "tile:3x3:sideways"] {
            assert!(
                parse_program(&[bad]).is_err(),
                "{bad} must refuse — a misread move would seal a proof the player did not mean"
            );
        }
    }

    #[test]
    fn a_rule_that_explains_the_pairs_seals_a_proof_the_sealed_math_accepts() {
        // THE WHOLE LOOP, end to end through the REAL verifier: the
        // bench seals exactly what `verify_work_class` re-hashes,
        // re-applies and re-chains.
        let served = served_from(turning_task());
        let program = parse_program(&["rot90"]).expect("a lawful move");
        assert_eq!(
            train_verdicts(&served.task, &program),
            vec![true, true, true],
            "the rule explains every pair"
        );
        let proof = seal(&served, program);
        let bytes = payload(&proof).expect("the proof encodes");
        assert_eq!(
            verify_work_class(&bytes),
            Ok(WorkClass::Corpus { task_id: served.task_id }),
            "what the bench seals, the gate accepts"
        );
    }

    #[test]
    fn a_rule_that_misses_one_pair_is_refused_by_the_gate() {
        // The bench blocks this before sealing (so the player is told
        // WHERE it missed), but the law is the gate's: a program that
        // does not reproduce the training pairs is not a proof, and
        // this pins that the two agree.
        let served = served_from(turning_task());
        let wrong = parse_program(&["rot180"]).expect("a lawful move");
        assert!(
            train_verdicts(&served.task, &wrong).iter().any(|held| !held),
            "the bench sees the miss"
        );
        let proof = seal(&served, wrong);
        let bytes = payload(&proof).expect("it still encodes — it is just wrong");
        assert!(
            matches!(
                verify_work_class(&bytes),
                Err(WorkBroken::TrainingMiss { .. })
            ),
            "and so does the gate"
        );
    }

    #[test]
    fn the_once_per_task_law_reads_the_chronicle() {
        // #38d at the emitter. A world that has minted against this
        // task's payload answers yes; one that has not, no. (The gate
        // half needs the solved set in committed state — recorded, not
        // pretended.)
        let served = served_from(turning_task());
        let program = parse_program(&["rot90"]).expect("a lawful move");
        let proof = seal(&served, program);
        let bytes = payload(&proof).expect("encodes");

        let mut chronicle = cfe::vrp::Chronicle::new();
        assert!(!already_earned(&chronicle, &served.task_id), "nothing earned yet");
        chronicle.append(
            cfe::dop::ClosureEvent::DiscoveryMinted {
                orb_id: 1,
                resource: cfe::dop::Resource::Crystal,
                amount: cfe::ratio::ratio(1, 1),
                work: bytes,
            },
            1,
            vec![cfe::world::SOVEREIGN],
        );
        assert!(already_earned(&chronicle, &served.task_id), "and now it has");
        // A DIFFERENT task is untouched by that receipt.
        let other = served_from(Task { name: "other".into(), ..turning_task() });
        let mut moved = other.task.clone();
        moved.test_in = grid(2, 2, &[8, 8, 8, 8]);
        let other = served_from(moved);
        assert!(
            !already_earned(&chronicle, &other.task_id),
            "the law is per TASK, not a blanket"
        );
    }

    #[test]
    fn the_painter_draws_every_hue_and_keeps_the_shape() {
        let held = grid(2, 3, &[0, 1, 2, 3, 4, 5]);
        let plain = paint(&held, false);
        assert_eq!(plain, vec!["0 1 2 ", "3 4 5 "]);
        assert_eq!(paint(&held, true).len(), 2, "colour keeps the row count");
        // A pair reads as two grids with a marker between them.
        let rows = beside(&held, &held, "→", false);
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().any(|row| row.contains('→')), "the marker lands once");
    }
}
