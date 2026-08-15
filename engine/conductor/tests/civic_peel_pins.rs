//! **THE FIRST LIVE PEEL** (COG Slice 1) — the homological economy read
//! against REAL committed state, not a constructed fixture.
//!
//! `civic.rs` is proven in isolation by its own unit tests; this drives it
//! over worlds a full era actually founded. For each founded line it builds
//! the demand/supply vectors from committed state — the population's
//! metabolic food demand (`pop × per_capita_ration`) against the aggregate
//! net capacity of the zones standing — draws the buildable generators
//! STRICTLY from that species' own `Axis::Zone` fold, and runs
//! `civic::civic_reading`. The pins assert the peel is coherent on live
//! data: production names a structure the civilization knows, and the
//! coefficients/residual split conserves the deficit exactly
//! (`generators·coefficients + residual = deficit`). The beat is NOT yet
//! driven by this — Slice 1 reads and peels; a later slice hands it the
//! wheel and re-verifies the goldens.

use conductor::civic;
use conductor::run_era;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("civic-peel-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// The metabolic food demand a line's population places this beat, as an
/// exact `(carrier, rate)` load: `pop × per_capita_ration` on the line's
/// ration carrier. Empty when the line is unpopulated or a rate-0 capture
/// line (a plasmoid rides the energy economy, #29 deferred).
fn food_demand(world: &cfe::world::World, orb: u64) -> Vec<(cfe::dop::Resource, cfe::ratio::Ratio)> {
    let pop = world.population_of(orb);
    if pop == 0 {
        return Vec::new();
    }
    let per_capita = world.per_capita_ration();
    if per_capita <= cfe::ratio::ratio(0, 1) {
        return Vec::new();
    }
    let metabolism = world
        .committed()
        .genesis
        .as_ref()
        .map(|held| held.metabolism)
        .unwrap_or(2);
    let carrier = cfe::dop::metabolic_ration(metabolism);
    vec![(carrier, cfe::ratio::ratio(pop as i64, 1) * per_capita)]
}

#[test]
fn the_live_peel_reads_real_state_and_names_known_structures() {
    let root = scratch("live");
    let worlds = run_era(&root.to_string_lossy(), 15);
    assert!(worlds.len() >= 3, "the roster founded");

    let primary = cfe::world::FIRST_ORB;
    let mut peeled = 0usize; // lines that ran the live peel over real state
    let mut had_deficit = 0usize; // lines carrying a real physical deficit
    let mut named_a_build = 0usize; // lines whose Production named a structure
    let mut hit_a_wall = 0usize; // lines carrying an irreducible research residual

    for polity in &worlds {
        let world = &polity.world;
        let Some(orb) = world.orb() else {
            continue;
        };
        let metabolism = world
            .committed()
            .genesis
            .as_ref()
            .map(|held| held.metabolism)
            .unwrap_or(2);
        let fold = cfe::vocabulary::fold(world.chronicle(), cfe::vocabulary::Axis::Zone);
        let standing: Vec<cfe::dop::ZoneKind> = orb.zones.iter().map(|z| z.kind).collect();
        // The demand a founding line places is twofold, and BOTH are real
        // committed state: (1) the population's caloric draw (`pop ×
        // per_capita`, dormant until a line clones — population is opt-in,
        // never auto-seeded), and (2) the implicit SELF-SUFFICIENCY demand
        // — a colony must cover its own standing upkeep from its own yields.
        // The second needs no explicit load: `peel_economy` surfaces any
        // axis where the standing zones run a NET DEFICIT (negative supply)
        // as a positive deficit against a zero demand. So every founding
        // line peels; the caloric load only adds to it.
        let demand = food_demand(world, primary);
        peeled += 1;

        // The seed is a function of committed state (the world clock stands
        // in for the state digest here) — replayable, divergent per line.
        let seed = *blake3::hash(&world.now().to_le_bytes()).as_bytes();
        let reading = civic::civic_reading(&fold, metabolism, &standing, &demand, seed);

        // The buildable generators are drawn from THIS line's fold — every
        // structure Production names must be one the fold actually holds.
        let generators = civic::fold_generators(&fold, metabolism, &civic::resource_axes());
        let known: Vec<cfe::dop::ZoneKind> = generators.iter().map(|(k, _)| *k).collect();
        if let Some(kind) = reading.build {
            assert!(
                known.contains(&kind),
                "Production named a structure the fold does not hold: {kind:?}"
            );
            named_a_build += 1;
        }
        if !reading.peel.covered {
            hit_a_wall += 1;
        }

        // CONSERVATION — the peel splits the deficit exactly. Rebuild the
        // deficit from the committed vectors and check
        // `generators·coefficients + residual = deficit` axis by axis.
        let axes = civic::resource_axes();
        let supply = civic::standing_supply(&standing, metabolism, &axes);
        let mut want = vec![num_bigint::BigInt::from(0); axes.len()];
        for (res, rate) in &demand {
            if let Some(i) = axes.iter().position(|r| r == res) {
                // Mirror civic's lattice quantization (LATTICE·rate rounded).
                let q = (rate * cfe::ratio::ratio(civic::LATTICE, 1)).round().to_integer();
                want[i] += q;
            }
        }
        let mut any_deficit = false;
        for i in 0..axes.len() {
            let deficit = {
                let gap = &want[i] - &supply[i];
                if gap < num_bigint::BigInt::from(0) {
                    num_bigint::BigInt::from(0)
                } else {
                    gap
                }
            };
            if deficit > num_bigint::BigInt::from(0) {
                any_deficit = true;
            }
            let mut spanned = reading.peel.residual[i].clone();
            for (gi, (_, vec)) in generators.iter().enumerate() {
                spanned += &vec[i] * &reading.peel.coefficients[gi];
            }
            assert_eq!(
                spanned, deficit,
                "peel conservation broke on axis {i}: generators·coeff + residual ≠ deficit"
            );
        }
        if any_deficit {
            had_deficit += 1;
        }
    }

    eprintln!(
        "LIVE PEEL: {peeled} lines peeled · {had_deficit} ran a real deficit · \
         {named_a_build} named a build · {hit_a_wall} hit a research wall"
    );
    // The era must have exercised the live peel on the real founded roster.
    assert!(peeled >= 3, "the live peel ran over at least three founded lines");
    // And on real committed state at least some lines carry a genuine
    // physical deficit — a colony whose standing zones cannot yet cover
    // their own upkeep, the drive to build. Where a line has a deficit the
    // peel either named a known structure to close it (Production) or left
    // an irreducible residual (Research); either is lawful, but a deficit
    // that produced NEITHER would mean the peel silently swallowed a
    // shortfall.
    assert!(
        had_deficit >= 1,
        "at least one founded line runs a real physical deficit against its own upkeep"
    );
    assert!(
        named_a_build + hit_a_wall >= 1,
        "a line with a deficit either named a build or surfaced a research wall on real state"
    );
}
