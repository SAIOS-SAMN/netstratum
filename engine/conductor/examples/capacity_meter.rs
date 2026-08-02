//! THE CAPACITY METER (remediations lane, task #1) — the instrument
//! `17_CAPACITY` §7 owes: *build the meter, measure, then rule — never
//! rule a placeholder blind.*
//!
//! This is a READING. It appends nothing, writes nothing, judges
//! nothing, and changes no law. It replays each world's chronicle and
//! reports the readings that rows **B1–B8** (`docs/FORGE_BUDGET.md`,
//! `EQUATIONS` §7) need before any of them can be given a value.
//!
//! ```text
//! cargo run --release -p conductor --example capacity_meter -- <var-root>
//! ```
//!
//! **THREE ANTI-COLLAPSE LAWS BIND THIS OUTPUT**, and it is built to
//! honour all three rather than to be convenient:
//!
//! 1. **Per channel, never the aggregate** (`bridge.rs:306-335`) — *"a
//!    universe can read `Entangled` on the sum while no single channel
//!    is entangled at all."* So the verdict is reported per channel.
//! 2. **Dimension is not volume** (operator ruling 2026-08-02,
//!    `bridge.rs:176-199`) — *"your entire definition is wrong if
//!    dimensional capacity is a count of rooms."* `rank` counts the
//!    independent cycles; the holonomy VALUES say what each one carries.
//!    Both are reported; the charge is never replaced by its count.
//! 3. **The four axes never collapse** (`09` §1.3, `EQUATIONS` §6 S2) —
//!    so compassion, love, integrity and morality are reported
//!    separately and `syntropy_ratio` is deliberately NOT called.
//!
//! Reading the output: a line whose `rank` is equal to another's but
//! whose charge differs is NOT the same line, and a budget that treated
//! them alike would be the collapse the same ruling struck.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use cfe::ratio::{ratio, Ratio};
use cfe::world::Committed;
use kernel::bridge::{read_entangled, verdict_per_dimension, CHANNELS};

/// One axis ratio, kept in its own name — never summed with another.
/// `None` when the axis has posted nothing (refuse-never-guess: an
/// unmeasured axis is not a zero one).
fn axis_ratio(syntropic: &Ratio, entropic: &Ratio) -> Option<Ratio> {
    let whole = syntropic.clone() + entropic.clone();
    match whole > ratio(0, 1) {
        true => Some(syntropic.clone() / whole),
        false => None,
    }
}

fn show(r: &Option<Ratio>) -> String {
    match r {
        Some(v) => format!("{v}"),
        None => "—".into(),
    }
}

/// The four named axes of `09` §2, each read on its own terms.
/// **Compassion is deliberately NOT a ratio** — `given_in_crisis`
/// stands alone with no entropic counterpart in the committed fields
/// (`FORGE_BUDGET` §7.6, an open row). Reporting it as a LEVEL rather
/// than forcing a denominator is the honest reading.
struct Alignment {
    compassion_level: Ratio,
    love: Option<Ratio>,
    integrity: Option<Ratio>,
    morality: Option<Ratio>,
}

fn alignment(state: &Committed) -> Alignment {
    let zero = ratio(0, 1);
    let (mut given, mut bond, mut sever) = (zero.clone(), zero.clone(), zero.clone());
    let (mut kept, mut broke) = (zero.clone(), zero.clone());
    let (mut returned, mut extracted) = (zero.clone(), zero.clone());
    for orb in &state.orbs {
        given += orb.given_in_crisis.clone();
        bond += orb.bond_flow.clone();
        sever += orb.severance_load.clone();
        kept += orb.covenants_fulfilled.clone();
        broke += orb.covenants_breached.clone();
        returned += orb.returned_across.clone();
        extracted += orb.extracted_across.clone();
    }
    Alignment {
        compassion_level: given,
        love: axis_ratio(&bond, &sever),
        integrity: axis_ratio(&kept, &broke),
        // MORALITY: the same form `world::reciprocity_reading` seals
        // (`world.rs:4905`), read here across the whole book. When that
        // reading is made public this must CALL it, never keep a copy
        // (CHARTER §0b) — recorded as owed, not silently duplicated.
        morality: axis_ratio(&returned, &extracted),
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let root = match args.next() {
        Some(held) => PathBuf::from(held),
        None => {
            println!("usage: capacity_meter <var-root>");
            return;
        }
    };
    let worlds_dir = root.join("worlds");
    let Ok(entries) = std::fs::read_dir(&worlds_dir) else {
        println!("no worlds under {}", worlds_dir.display());
        return;
    };

    let mut paths: Vec<PathBuf> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    paths.sort();

    // Tallies — never a verdict, only a census (the harness law: tally
    // and report, never panic on a measurable outcome).
    let mut entangled_channels: BTreeMap<usize, usize> = BTreeMap::new();
    let mut rank_spread: BTreeMap<usize, usize> = BTreeMap::new();
    let mut unknown_lines = 0usize;
    let mut read = 0usize;

    // THE SUBSTRATE of the reading, so a zero explains itself:
    // `project_complex` (bridge.rs:373) builds the complex over the
    // book's OWN orbs, edged by bonds | covenants | debts, and returns
    // no structure at all when `dim < 2`. A rank of 0 beside `orbs 1`
    // is not a low reading — it is a STRUCTURALLY UNREACHABLE one.
    let mut substrate: BTreeMap<usize, usize> = BTreeMap::new();
    let mut edged_lines = 0usize;

    println!(
        "{:<20} {:>4} {:>4} {:>4} {:>4} {:>5} {:>7} {:>7}  {:<11}  {:>8} {:>9} {:>8} {:>10}",
        "world", "orbs", "bnd", "cov", "dbt", "rank", "degree", "unknown", "per-channel", "love",
        "integrity", "morality", "compassion"
    );
    println!("{}", "-".repeat(130));

    for dir in &paths {
        let carrier = dir.join("chronicle.tlv");
        if !carrier.exists() {
            continue;
        }
        let Ok(chronicle) = cfe::persist::load(&carrier) else {
            println!("{:<22} chronicle refused", name_of(dir));
            continue;
        };
        let state = cfe::world::replay(&chronicle);

        let entangled = read_entangled(&state);
        let per_channel = verdict_per_dimension(&state);
        let live = per_channel.iter().filter(|v| v.is_admitted()).count();
        let align = alignment(&state);

        read += 1;
        *entangled_channels.entry(live).or_insert(0) += 1;
        *rank_spread.entry(entangled.rank).or_insert(0) += 1;
        *substrate.entry(state.orbs.len()).or_insert(0) += 1;
        if entangled.unknown > 0 {
            unknown_lines += 1;
        }
        if !state.bonds.is_empty() || !state.covenants.is_empty() || !state.debts.is_empty() {
            edged_lines += 1;
        }

        // THE CHARGE, PER CHANNEL — the reading `rank` alone discards.
        let mut charge: Vec<String> = Vec::new();
        for l in 0..CHANNELS {
            let held: Vec<&Ratio> = entangled
                .holonomy
                .iter()
                .filter(|h| h.l == l)
                .map(|h| &h.value)
                .collect();
            let admitted = per_channel.get(l).map(|v| v.is_admitted()).unwrap_or(false);
            charge.push(format!(
                "{}{}",
                if admitted { "E" } else { "·" },
                held.len()
            ));
        }

        println!(
            "{:<20} {:>4} {:>4} {:>4} {:>4} {:>5} {:>7} {:>7}  {:<11}  {:>8} {:>9} {:>8} {:>10}",
            name_of(dir),
            state.orbs.len(),
            state.bonds.len(),
            state.covenants.len(),
            state.debts.len(),
            entangled.rank,
            format!("{:?}", entangled.degree()),
            entangled.unknown,
            charge.join(" "),
            show(&align.love),
            show(&align.integrity),
            show(&align.morality),
            align.compassion_level,
        );

        // The charge VALUES, whole, when any cycle stands — never
        // summed into one figure (18 §2b).
        if !entangled.holonomy.is_empty() {
            let values: Vec<String> = entangled
                .holonomy
                .iter()
                .map(|h| format!("l{}:{}", h.l, h.value))
                .collect();
            println!("{:>24}charge {}", "", values.join(" "));
        }
    }

    println!("\n=== CENSUS over {read} lines ===");
    println!("lines with unknown > 0 (cross-book, B8 epsilon_unknown): {unknown_lines}");
    println!("entangled-channel count -> lines: {entangled_channels:?}");
    println!("rank -> lines: {rank_spread:?}");
    println!("orbs per book -> lines: {substrate:?}");
    println!("lines carrying ANY bond/covenant/debt edge: {edged_lines}");
    let single = substrate.get(&0).copied().unwrap_or(0) + substrate.get(&1).copied().unwrap_or(0);
    if single == read && read > 0 {
        println!(
            "\n*** EVERY BOOK HAS < 2 ORBS. project_complex (bridge.rs:377) returns NO STRUCTURE\n\
             *** below dim 2, so rank 0 here is STRUCTURALLY UNREACHABLE, not merely low.\n\
             *** A capacity-derived budget built on this board seats EVERY line at beta_floor —\n\
             *** i.e. a constant again. The reading must become reachable before B2/B3 can bind."
        );
    }
    println!(
        "\nB2 rho_D candidates to rule: count-of-entangled-channels | min-rank-across | sum-over-entangled"
    );
    println!("NOTE: rank is DIMENSION, not volume. Two lines of equal rank and unequal charge");
    println!("      are not the same line (bridge.rs:176-199). Read the charge rows above.");
}

fn name_of(dir: &Path) -> String {
    dir.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}
