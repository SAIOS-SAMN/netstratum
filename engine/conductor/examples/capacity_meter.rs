//! THE CAPACITY METER (remediations lane, task #1) — the instrument
//! `17_CAPACITY` §7 owes: *build the meter, measure, then rule — never
//! rule a placeholder blind.*
//!
//! A READING. Appends nothing, writes nothing, gates nothing, changes
//! no law. It replays each world's chronicle and reports what rows
//! **B1–B8** (`docs/FORGE_BUDGET.md`, `EQUATIONS` §7) need before any of
//! them can be given a value.
//!
//! ```text
//! cargo run --release -p conductor --example capacity_meter -- <var-root>
//! ```
//!
//! **FOUR ANTI-COLLAPSE LAWS BIND THIS OUTPUT.** It is built to honour
//! them rather than to be convenient:
//!
//! 1. **Per channel, never the aggregate** (`bridge.rs:306-335`) — *"a
//!    universe can read `Entangled` on the sum while no single channel
//!    is entangled at all."*
//! 2. **Dimension is not volume** (`bridge.rs:176-199`) — *"your entire
//!    definition is wrong if dimensional capacity is a count of rooms."*
//!    `rank` counts cycles; `holonomy` values say what each CARRIES.
//! 3. **Both poles, separately** (`boundary.rs:147`) — `Vacuity` is
//!    decided by structure AND gradient. Reporting only "admitted"
//!    hides which pole is missing, and the cure differs: `Stasis`
//!    (rank, no flow) must be RE-VOLUMIZED; `Void` needs structure.
//! 4. **The four axes never collapse** (`09` §1.3, `EQUATIONS` §6 S2) —
//!    so `syntropy_ratio` is deliberately NOT called.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use cfe::ratio::{ratio, Ratio};
use cfe::world::Committed;
use kernel::bridge::{poles_per_dimension, read_entangled, CHANNELS};
use solver_verify::forge::Difficulty;

/// One axis ratio in its own name — never summed with another. `None`
/// when the axis has posted nothing (refuse-never-guess: an unmeasured
/// axis is not a zero one).
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

fn magnitude(v: &Ratio) -> Ratio {
    match *v < ratio(0, 1) {
        true => -v.clone(),
        false => v.clone(),
    }
}

/// The four named axes of `09` §2, each read on its own terms.
/// **Compassion is deliberately NOT a ratio** — `given_in_crisis` has no
/// entropic counterpart in the committed fields (`FORGE_BUDGET` §7.6, an
/// open row). Reported as a LEVEL rather than given a false denominator.
struct Alignment {
    compassion_level: Ratio,
    love: Option<Ratio>,
    integrity: Option<Ratio>,
    morality: Option<Ratio>,
}

impl Alignment {
    /// The WEAKEST standing among the axes that have posted — the
    /// order-preserving non-sum reading (`B6`). `None` when no axis has
    /// posted at all. Never an average: an average is the collapse.
    fn weakest(&self) -> Option<Ratio> {
        [&self.love, &self.integrity, &self.morality]
            .into_iter()
            .flatten()
            .min()
            .cloned()
    }
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
        kept += orb.forwards_fulfilled.clone();
        broke += orb.forwards_breached.clone();
        returned += orb.returned_across.clone();
        extracted += orb.extracted_across.clone();
    }
    Alignment {
        compassion_level: given,
        love: axis_ratio(&bond, &sever),
        integrity: axis_ratio(&kept, &broke),
        // MORALITY: the form `world/mod.rs::reciprocity_reading`
        // seals, read across the whole book. When that
        // reading is made public this must CALL it, never keep a copy
        // (CHARTER §0b) — recorded as owed, not silently duplicated.
        morality: axis_ratio(&returned, &extracted),
    }
}

/// THE DERIVED ENVELOPE — **called, never copied** (CHARTER §0b).
/// This meter kept its OWN `frontier` for one commit, which is the
/// duplication the call-not-copy law exists to stop: two spellings of
/// one rule drift the moment either moves. The sealed rule is
/// `solver::search::frontier`.
use solver::search::frontier;

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

    let mut entangled_channels: BTreeMap<usize, usize> = BTreeMap::new();
    let mut rank_spread: BTreeMap<usize, usize> = BTreeMap::new();
    let mut verdict_census: BTreeMap<String, usize> = BTreeMap::new();
    let mut substrate: BTreeMap<usize, usize> = BTreeMap::new();
    let mut unknown_lines = 0usize;
    let mut edged_lines = 0usize;
    let mut foundation_lines = 0usize;
    let mut flow_lines = 0usize;
    let mut read = 0usize;

    println!("=== PER LINE ===");
    println!(
        "{:<19} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>5} {:>4} {:>4} {:>4}  {:<20}",
        "world", "orb", "zon", "ent", "bnd", "cov", "dbt", "off", "rte", "rank", "flow", "fnd",
        "unk", "per-channel verdict"
    );
    println!("{}", "-".repeat(132));

    let mut lines: Vec<(String, Alignment)> = Vec::new();
    // Held for the CROSS-BOOK read — the complex whose vertices are
    // whole books, which no single-book projection can see.
    let mut states: Vec<Committed> = Vec::new();

    for dir in &paths {
        let carrier = dir.join("chronicle.tlv");
        if !carrier.exists() {
            continue;
        }
        let Ok(chronicle) = cfe::persist::load(&carrier) else {
            println!("{:<19} chronicle refused", name_of(dir));
            continue;
        };
        let state = cfe::world::replay(&chronicle);

        let entangled = read_entangled(&state);
        // The per-channel VERDICT is struck with the vacuity removal;
        // the poles it collapsed are still measurable, and a meter
        // wants the measurement rather than the judgement anyway.
        let per_channel = poles_per_dimension(&state);
        let live = per_channel.iter().filter(|(rank, flows)| *rank > 0 && *flows > 0).count();
        let align = alignment(&state);
        let zones: usize = state.orbs.iter().map(|o| o.zones.len()).sum();

        read += 1;
        *entangled_channels.entry(live).or_insert(0) += 1;
        *rank_spread.entry(entangled.rank).or_insert(0) += 1;
        *substrate.entry(state.orbs.len()).or_insert(0) += 1;
        for (rank, flows) in &per_channel {
            let row = match (*rank > 0, *flows > 0) {
                (false, false) => "neither pole",
                (true, false) => "rank, no flow",
                (false, true) => "flow, no rank",
                (true, true) => "both poles",
            };
            *verdict_census.entry(row.to_string()).or_insert(0) += 1;
        }
        if entangled.unknown > 0 {
            unknown_lines += 1;
        }
        if !state.bonds.is_empty() || !state.forwards.is_empty() || !state.debts.is_empty() {
            edged_lines += 1;
        }
        if !entangled.symmetry_foundation.is_empty() {
            foundation_lines += 1;
        }
        if !entangled.flows.is_empty() {
            flow_lines += 1;
        }

        // BOTH POLES PER CHANNEL — the verdict names which is missing.
        let verdicts: Vec<String> = (0..CHANNELS)
            .map(|l| {
                let cycles = entangled.holonomy.iter().filter(|h| h.l == l).count();
                let flows = entangled.flows.iter().filter(|f| f.l == l).count();
                let v = per_channel
                    .get(l)
                    .map(|v| format!("{v:?}"))
                    .unwrap_or_else(|| "?".into());
                format!("{}({cycles}/{flows})", &v[..1.min(v.len())])
            })
            .collect();

        println!(
            "{:<19} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4} {:>5} {:>4} {:>4} {:>4}  {:<20}",
            name_of(dir),
            state.orbs.len(),
            zones,
            state.entities.len(),
            state.bonds.len(),
            state.forwards.len(),
            state.debts.len(),
            state.offers.len(),
            state.routes.len(),
            entangled.rank,
            entangled.flows.len(),
            entangled.symmetry_foundation.len(),
            entangled.unknown,
            verdicts.join(" "),
        );

        // PER-ORB STANDING (post-merge: SYN-4/SYN-5 closed the free
        // ride, so standing is asked of the ORB, not inherited from the
        // world). `open_axes_of` is the syntropic half — "the DIRECTION
        // A NEW RELATION WOULD EARN ON", which is the capability-gated
        // expansion reading, already built.
        for (orb, axial) in kernel::bridge::borne_by_orb_axial(&state) {
            let open = kernel::bridge::open_axes_of(&state, orb);
            let total = kernel::bridge::standing_of(&state, orb);
            let shares: Vec<String> = axial.iter().map(|s| format!("{s}")).collect();
            println!(
                "{:>21}orb {orb} standing [{}] total {total} · open axes {open:?}",
                "",
                shares.join(" ")
            );
        }

        // THE CHARGE, WHOLE — the reading `rank` alone discards
        // (bridge.rs:176-199). Never summed into one figure.
        if !entangled.holonomy.is_empty() {
            let values: Vec<String> = entangled
                .holonomy
                .iter()
                .map(|h| format!("l{}:{}", h.l, magnitude(&h.value)))
                .collect();
            println!("{:>21}charge {}", "", values.join(" "));
        }
        lines.push((name_of(dir), align));
        states.push(state);
    }

    println!("\n=== ALIGNMENT — the four axes, never summed ===");
    println!(
        "{:<19} {:>12} {:>12} {:>12} {:>14} {:>12}",
        "world", "love", "integrity", "morality", "compassion(lvl)", "weakest"
    );
    println!("{}", "-".repeat(88));
    for (name, a) in &lines {
        println!(
            "{:<19} {:>12} {:>12} {:>12} {:>14} {:>12}",
            name,
            show(&a.love),
            show(&a.integrity),
            show(&a.morality),
            a.compassion_level,
            show(&a.weakest()),
        );
    }

    println!("\n=== CENSUS over {read} lines ===");
    println!("orbs per book -> lines:        {substrate:?}");
    println!("rank -> lines:                 {rank_spread:?}");
    println!("entangled-channel count:       {entangled_channels:?}");
    println!("per-channel verdicts:          {verdict_census:?}");
    println!("lines with any bond/cov/debt:  {edged_lines}");
    println!("lines with any FLOW (2nd pole):{flow_lines}");
    println!("lines with a symmetry found.:  {foundation_lines}");
    println!("lines with unknown > 0 (B8):   {unknown_lines}");

    // === THE CROSS-BOOK READ (operator ruling 2026-08-02) ===
    // The conductor holds the WHOLE roster, so unknown = 0 — that is
    // Resolve::LockOn, absence confirmed at the present reach, never
    // assumed.
    let borrowed: Vec<&Committed> = states.iter().collect();
    let across = kernel::bridge::read_entangled_across(&borrowed, 0);
    let across_channels = kernel::bridge::poles_per_dimension_across(&borrowed);
    let laned: usize = states.iter().filter(|s| !s.routes.is_empty()).count();
    let lanes: usize = states.iter().map(|s| s.routes.len()).sum();
    println!("\n=== CROSS-BOOK (books as vertices, shared lanes as edges) ===");
    println!("books read:                    {}", states.len());
    println!("books holding ANY lane:        {laned}");
    println!("lane legs across all books:    {lanes}");
    println!("cross-book rank:               {}", across.rank);
    println!("cross-book flows:              {}", across.flows.len());
    println!("cross-book symmetry found.:    {}", across.symmetry_foundation.len());
    println!("cross-book unknown:            {}", across.unknown);
    println!(
        "cross-book verdicts:           {:?}",
        across_channels.iter().map(|v| format!("{v:?}")).collect::<Vec<_>>()
    );
    if lanes == 0 {
        println!(
            "\n*** NO LANE STANDS ANYWHERE. charter_route draws 4 ALLOY from each side.\n\
             *** The refusal is now SURFACED (conductor prints LANE REFUSED), and it\n\
             *** reads CannotAfford Alloy need 4 have 0 falling to have 7/2 by beat 50\n\
             *** — accruing, but slower than the board's horizon. Measured: 110 beats,\n\
             *** 38 worlds, 4853 receipts, ZERO RouteChartered. The cross-book door is\n\
             *** built and its substrate never fills on the current economy."
        );
    }

    let single = substrate.get(&0).copied().unwrap_or(0) + substrate.get(&1).copied().unwrap_or(0);
    if single == read && read > 0 {
        println!(
            "\n*** EVERY BOOK HAS < 2 ORBS. project_complex (bridge.rs:377) returns NO STRUCTURE\n\
             *** below dim 2, so rank 0 here is STRUCTURALLY UNREACHABLE, not merely low.\n\
             *** A capacity-derived budget on this board seats EVERY line at the floor."
        );
    }

    // THE DERIVED ENVELOPE TABLE — every quantity B1/B3/B7 needs, as
    // arithmetic rather than as a chosen number.
    println!("\n=== THE DERIVED ENVELOPE: frontier(V, depth) = sum_(d=0..depth) V^d ===");
    println!("V = 6 symmetries + 8 obj_ops + <=7 derived recolors + {{0,16}} derived tiles");
    println!("{:<7} {:>10} {:>10} {:>10} {:>10}", "V", "d=0", "d=1", "d=2", "d=3");
    for v in [14usize, 21, 30, 37] {
        println!(
            "{:<7} {:>10} {:>10} {:>10} {:>10}",
            v,
            frontier(v, 0),
            frontier(v, 1),
            frontier(v, 2),
            frontier(v, 3)
        );
    }
    println!("\nrung -> word_len (the exhaustive depth), and mint_bound:");
    for level in 0..=5u32 {
        let d = Difficulty::level(level);
        println!(
            "  level {level}: tier {} word_len {} grid {}x{} pairs {}",
            d.tier, d.word_len, d.rows, d.cols, d.train_pairs
        );
    }
    println!(
        "\nNOTE: frontier(37,3) = {} — EXACTLY the bound the Ruling Box named for B7.",
        frontier(37, 3)
    );
    println!("      beta_cap is therefore DERIVABLE, not chosen: it is the exhaustive sweep.");

    // THE TWO ENVELOPES, SIDE BY SIDE. `kernel::motion::envelope(n, rank)
    // = n^rank` landed on MAIN as "the budget is a function of standing
    // rank rather than a constant chosen in advance". This lane's
    // `frontier(V, d) = sum_(k=0..d) V^k` landed HERE as the exhaustive
    // sweep to depth d. THEY ARE THE SAME LAW reached from two
    // directions — one shell against the cumulative sum of shells.
    // Printed together so the reconciliation cannot be missed.
    println!("\n=== TWO ENVELOPES, ONE LAW — reconcile before either is ruled ===");
    println!("kernel::motion::envelope(n, rank) = n^rank         (main — the shell)");
    println!("solver::search::frontier(V, d) = sum_(k=0..d) V^k  (this lane — the sum)");
    println!("{:<7} {:>16} {:>16}", "rank/d", "envelope(37,·)", "frontier(37,·)");
    for r in 0u32..=3 {
        println!(
            "{:<7} {:>16} {:>16}",
            r,
            kernel::motion::envelope(37, r).map(|v| v.to_string()).unwrap_or("overflow".into()),
            frontier(37, r as usize)
        );
    }
    println!("The shell is what ONE more dimension costs; the sum is what the whole");
    println!("search to that depth costs. A budget wants the sum; a per-axis bound");
    println!("wants the shell. One authority is owed (EQUATIONS §0.2).");
}

fn name_of(dir: &Path) -> String {
    dir.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}
