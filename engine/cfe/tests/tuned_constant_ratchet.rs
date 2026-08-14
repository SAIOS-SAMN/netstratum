//! **THE TUNED-CONSTANT RATCHET** (P2, `29_CONSTRUCTION`; census law
//! per the operator ruling 2026-08-05).
//!
//! P2's law: every number DERIVES, MEASURES, or MINTS. Until a cluster
//! is converted, its literals are DEBT — and PROVABLE NOT POLICED
//! rules that a debt must be a COUNTED number whose count only falls,
//! never a policy hoping nobody adds another.
//!
//! **The census tracks two metrics, ruled distinct:**
//!
//! - **DIALS** — tunables that alter balance without physical
//!   grounding. Counted here; target ZERO; the ceiling only falls.
//! - **ANCHORS** — cited empirical invariants, permitted ONLY through
//!   the typed declarations in `cfe::anchor` (there is no bare
//!   `measured()` constructor — that would be laundering). The anchor
//!   set is FINITE and PINNED EXACTLY in `anchor_pins.rs`, so this
//!   walk excludes `anchor.rs`: its discipline is the opposite one —
//!   nothing enters, leaves, or changes without going red against its
//!   own citation.
//!
//! `conductor/src/polities.rs` is likewise excluded: the founding
//! roster is APPROVED as lawful compiled seed data under THE
//! DEFINITION LAW (same ruling) — genesis reads no chain, so the seed
//! is definition, not tuning. Its size is pinned below so seed growth
//! stays visible.
//!
//! This witness counts `ratio(p, q)` literals whose value is neither
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
/// Held at 112. The live count fell 170 → 163 on 2026-08-12 (task #3:
/// the seven chartered-parameter seeds moved to the SEALED class,
/// where they belong — see `the_sealed_class_...`). 163 still exceeds
/// 112: the remaining 51 excess dials are the BALANCE clusters
/// (`dop.rs` 59 · `vocabulary.rs` 37 · conductor) whose conversion is
/// operator-gated per R20 — deriving them by fiat is the laundering
/// the census exists to bar, so the ratchet stands red until ruled.
const TUNED_LITERAL_CEILING: usize = 112; // held at 112; balance clusters await operator rulings

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

/// The sealed-history marker. An EPOCH-0 or writer-rule constant is
/// not a tunable — editing it rewrites what history means — so a line
/// carrying this marker leaves the dial walk and enters the SEALED
/// class, whose size is pinned EXACTLY below (the anchor discipline:
/// nothing enters without going red here, wearing its seal, in review).
const SEALED_MARK: &str = "SEALED-RATE";

/// The approved-seed marker (operator ruling 2026-08-06: the founding
/// endowment is APPROVED SEED — definition, not tuning, and never to
/// drift into dial status or freeze as sealed history). Same
/// discipline as the sealed class: marked lines leave the dial walk
/// and the marker count is pinned exactly.
const SEED_MARK: &str = "APPROVED-SEED";

fn tuned_in(line: &str) -> usize {
    if line.contains(SEALED_MARK) || line.contains(SEED_MARK) {
        return 0;
    }
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

/// Files the DIAL walk does not enter, each under its own opposite
/// discipline (see the module doc): the anchors are pinned exactly,
/// the seed's size is pinned below. Both filenames are unique in the
/// workspace.
const NOT_DIALS: [&str; 2] = ["anchor.rs", "polities.rs"];

fn walk(dir: &Path, total: &mut usize, files: &mut usize) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(|held| held.ok()) {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, total, files);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            let name = path.file_name().and_then(|held| held.to_str()).unwrap_or("");
            if NOT_DIALS.contains(&name) {
                continue;
            }
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

/// **THE SEALED CLASS IS PINNED EXACTLY** — a sealed constant is
/// counted by its marker, and the count moves only in review: history
/// gains a sealed rule when an epoch closes, never silently. The
/// VALUES are held by the epoch pins (`parameter_pins` holds the
/// cliff, the flat sink rate rides `demurrage_per_tick` at an epoch-0
/// stamp, the writer rule rides the codec's own round trips).
#[test]
fn the_sealed_class_is_exactly_as_large_as_sealed() {
    let mut marks = 0usize;
    let mut files = 0usize;
    fn walk_marks(dir: &Path, mark: &str, marks: &mut usize, files: &mut usize) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.filter_map(|held| held.ok()) {
            let path = entry.path();
            if path.is_dir() {
                walk_marks(&path, mark, marks, files);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                *files += 1;
                if let Ok(text) = fs::read_to_string(&path) {
                    *marks += text.lines().filter(|line| line.contains(mark)).count();
                }
            }
        }
    }
    for tree in SOURCE_TREES {
        walk_marks(Path::new(tree), SEALED_MARK, &mut marks, &mut files);
    }
    assert!(files > 0, "the walk found no source files — the paths moved");
    assert_eq!(
        marks, 10,
        "the sealed class moved — an epoch closed (lawful, pin it here \
         in review) or a dial put on a seal it did not earn. \
         Grew 3 → 10 on 2026-08-12 (task #3): the SEVEN chartered-\
         parameter seeds (`parameter::seed`) are epoch-0 rates —\
         editing one recomputes every pre-charter chronicle's heat, so \
         they move only by a receipted ParameterChartered deed, which \
         is the SEALED-RATE property, not tuning."
    );
    let mut seeded = 0usize;
    let mut seed_files = 0usize;
    for tree in SOURCE_TREES {
        walk_marks(Path::new(tree), SEED_MARK, &mut seeded, &mut seed_files);
    }
    assert_eq!(
        seeded, 2,
        "the marked seed class moved — approval covers the founding \
         endowment's two values (operator ruling 2026-08-06)"
    );
}

/// **THE SEED'S SIZE IS PINNED** — polities.rs stands outside the dial
/// walk as APPROVED compiled seed (THE DEFINITION LAW, operator ruling
/// 2026-08-05), so its growth must stay visible HERE: a new seed value
/// goes red until this pin moves with it, deliberately, in review.
#[test]
fn the_approved_seed_is_exactly_as_large_as_approved() {
    let text = fs::read_to_string("../conductor/src/polities.rs")
        .expect("the approved seed stands where it stood");
    let seeded: usize = text.lines().map(tuned_in).sum();
    assert_eq!(
        seeded, 68,
        "the founding roster's seed count moved — approval covered 68 values \
         (Wave-1 added three T2 sovereign seats: Stelahold · Tallykeep · Glyphward)"
    );
}
