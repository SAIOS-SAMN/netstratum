//! **THE TUNED-CONSTANT RATCHET** (P2, `29_CONSTRUCTION`).
//!
//! P2's law: every number DERIVES, MEASURES, or MINTS. Until a cluster
//! is converted, its literals are DEBT — and PROVABLE NOT POLICED
//! rules that a debt must be a COUNTED number whose count only falls,
//! never a policy hoping nobody adds another.
//!
//! This witness is the count. It walks every source tree in the
//! workspace and counts `ratio(p, q)` literals whose value is neither
//! zero nor one — the identity and the additive zero are lawful
//! algebra, not tuning. A literal written with variables
//! (`ratio(condition as i64, 1)`) does not parse as two integers and
//! is not counted: it already derives.
//!
//! **The ceiling moves one way.** Convert a cluster, watch the count
//! fall, LOWER the ceiling to the new measurement in the same commit.
//! A change that raises the count goes red here and must either derive
//! its number or answer for a new tunable in review. Doc-comment
//! examples count too — prose examples of tuning are still tuning
//! shown to the next coder, and stripping them costs one honest line.

use std::fs;
use std::path::Path;

/// Every source tree in the workspace, relative to `engine/cfe`.
const SOURCE_TREES: [&str; 11] = [
    "src",
    "../codec/src",
    "../conductor/src",
    "../kernel/src",
    "../ledger/src",
    "../lens/src",
    "../mesh/src",
    "../rfa-core/src",
    "../scribe/src",
    "../solver/src",
    "../../clients/cli/src",
];

/// **THE CEILING — measured 2026-08-05, and it only falls.**
const TUNED_LITERAL_CEILING: usize = 200;

/// Parse `ratio(<int>, <int>)` starting at `text` (just past the open
/// paren). Returns the pair only when both arguments are integer
/// literals — anything else is a derivation, not a tunable.
fn literal_pair(text: &str) -> Option<(i64, i64)> {
    let (numer_text, rest) = text.split_once(',')?;
    let (denom_text, _) = rest.split_once(')')?;
    // Rust spells big literals with underscores (`ratio(10_000, 1)`) —
    // still a dial; a bare parse would miss every one of them.
    let numer: i64 = numer_text.trim().replace('_', "").parse().ok()?;
    let denom: i64 = denom_text.trim().replace('_', "").parse().ok()?;
    Some((numer, denom))
}

fn tuned_in(line: &str) -> usize {
    let mut count = 0;
    let mut rest = line;
    while let Some(at) = rest.find("ratio(") {
        rest = &rest[at + "ratio(".len()..];
        if let Some((numer, denom)) = literal_pair(rest) {
            // Zero and one are algebra; everything else is a dial.
            let trivial = (numer == 0) || (numer == denom);
            if !trivial {
                count += 1;
            }
        }
    }
    count
}

fn walk(dir: &Path, total: &mut usize, files: &mut usize) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(|held| held.ok()) {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, total, files);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            *files += 1;
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            for line in text.lines() {
                *total += tuned_in(line);
            }
        }
    }
}

/// **THE DEBT IS COUNTED, AND THE COUNT ONLY FALLS.**
#[test]
fn the_tuned_constant_debt_only_falls() {
    let mut total = 0;
    let mut files = 0;
    for tree in SOURCE_TREES {
        walk(Path::new(tree), &mut total, &mut files);
    }
    assert!(files > 0, "the walk found no source files — the paths moved");
    assert!(
        total <= TUNED_LITERAL_CEILING,
        "{total} tuned ratio literals stand against a ceiling of \
         {TUNED_LITERAL_CEILING} — a new dial entered the tree. Derive \
         it, measure it, or mint it (P2); the ceiling does not rise."
    );
    // The other direction is a nudge, not a wall: when conversion work
    // drops the count well below the ceiling, the ceiling follows it
    // down in the same commit or this line asks for it.
    assert!(
        total + 25 > TUNED_LITERAL_CEILING,
        "the count fell to {total}, far under the ceiling of \
         {TUNED_LITERAL_CEILING} — lower the ceiling to the new \
         measurement so the ratchet holds what was won."
    );
}

/// The parser itself is pinned, so the census cannot drift by parsing.
#[test]
fn the_census_parser_reads_what_it_should() {
    assert_eq!(tuned_in("ratio(3, 4)"), 1);
    assert_eq!(tuned_in("ratio(0, 1) + ratio(1, 1)"), 0, "algebra is not tuning");
    assert_eq!(tuned_in("ratio(7, 7)"), 0, "one in any spelling");
    assert_eq!(tuned_in("ratio(-1, 2)"), 1, "sign does not hide a dial");
    assert_eq!(tuned_in("ratio(n, 1)"), 0, "a variable already derives");
    assert_eq!(tuned_in("ratio(condition as i64, 1)"), 0);
    assert_eq!(tuned_in("ratio(5, 4)) * ratio(3, 2)"), 2, "two dials on one line");
    assert_eq!(tuned_in("ratio(10_000, 1)"), 1, "underscores do not hide a dial");
}
