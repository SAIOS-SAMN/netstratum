//! **THE MEASURED SYSTEM BOUNDS** (P2, operator ruling 2026-08-06:
//! *execute the conductor thresholds as measured system bounds*).
//!
//! Re-measured 2026-08-07 after GROUND STEP (prospect/assay + regenerator
//! ecology) and the sovereign founding band (Stelahold · Tallykeep · Glyphward).
//! The autonomous loop opens veins and may place Regenerators when veins
//! wear, so the settlement footprint moved. The pin records the new
//! measurement deliberately — not a silent nudge.
//!
//! The industry half of the bounds (smelter/synthesizer gates behind
//! research rungs) is measured where it always was:
//! `research_unlocks_industry` pins the 45-beat outcome; this file
//! deliberately stays inside the cheap 15-beat window the settlement
//! thresholds govern.

use conductor::run_era;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "measured-bounds-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// Measured 2026-08-07 on the founding roster (incl. sovereign band),
/// 15 beats, under derived settlement gates + ground_step.
/// Prior pin was 50 (2026-08-06, pre-ground, no T2 seats).
/// RE-MEASURED 2026-08-12 to 73: the taxon yield scales + endowment
/// gifts (STAR-ENDOW-1/ORBIT-ENDOW-1) + the deposit law changed what
/// the reserve gates fund, so the era's footprint grew — a deliberate
/// re-measure under the ruled economy, the divergence property still
/// holds below (the temperaments still build different cities).
/// RE-MEASURED 2026-08-14 to 91: the G2 TRACE-ELEMENT (micronutrient)
/// factor lands — a line's harvest now scales with how well it meets the
/// trace-element need its own veins bear ("you eat what your world
/// grows"), so a well-nourished roster settles more. This also FEEDS
/// `tunborn`, which had been famishing at 3 zones (the settlement debt
/// shrank by one). A deliberate re-measure under the ruled economy.
/// RE-MEASURED 2026-08-14 to 92: the G2 harvesting CATALYST (§3) lands —
/// a line's favoured elements now flux its EXTRACTION (boost-only, on the
/// extractor line), so a reagent-holding line mines enough to settle one
/// more zone. A deliberate re-measure under the ruled economy.
/// RE-MEASURED 2026-08-14 to 90: the COURT RENAME (#5) — the Earth-culture
/// line names retired for coined netstratum-native ones. A world's founding
/// digest carries its name, so the renamed lines draw different veins and
/// the placement walk resolves differently, shifting the whole board's
/// footprint deterministically. A pure identity re-measure — no law changed.
/// RE-MEASURED 2026-08-14 to 89: MULTI-ORB EXPANSION (INTELLIGENCE ARC A1)
/// — a line that has climbed to rung 3 now CHARTS colony orbs and weaves an
/// internal supply ring, and `chart_as` MOVES founding reserves from the
/// home orb to the colony. So a fast-climbing line diverts inside the
/// window what it would have spent on home zones — settlement trades
/// against expansion, exactly as it should. The colonies' own zones are
/// not counted here (this pins the HOME orb), so the footprint falls by the
/// one home zone the reserves expansion drew (90 → 89). A deliberate
/// re-measure; the divergence property still holds (the temperaments still
/// build different cities).
/// RE-MEASURED 2026-08-15 to 88: **THE AWAKENING** (COG Slice 2 activation).
/// The first habitat on a colony now seats a FOUNDING BAND (`FOUNDING_BAND`),
/// so the population is LIVE. Because the founding bands live by GATHERING —
/// their whole demand sits under the planet's syntropic WILD flow, which is
/// eaten first and never touches the granary (COG Slice 2c, the bank-draw
/// netting) — they draw NOTHING from the cultivated biomass the builds spend.
/// So the footprint barely moves (89 → 88): a single marginal zone tips across
/// the whole roster as the live population perturbs one digest-seeded decision,
/// no more. (Were the bands eating the granary, this fell to 67 — a broken
/// bootstrap that also starved the smelter loop; the wild-flow-first law is
/// what keeps a founding band from taxing the industrial economy.) The per-line
/// bootstrap law is UNCHANGED and the temperaments still diverge. A deliberate
/// re-measure under the now-live population.
const MEASURED_ZONE_TOTAL_AT_15: usize = 88;

/// **THE ENERGY-BOOTSTRAP DEBT** — the lines the settlement law cannot
/// reach, pinned so the set can only SHRINK:
///
/// - cold-siting lines freeze-lock in joule famine at one CryoArray
///   (the Frost branch's own doc calls the freeze-lock "legitimate
///   drama, not a bug");
/// - Furnace / thermal holds and new sovereign seats that famish inside
///   the window (thresholder, tallykeep, glyphward) — recorded so a cure shrinks
///   this list rather than silently green-washing the debt.
const FROZEN_AT_15: [&str; 8] = [
    "corebound",
    "stillcold",
    "umbrid",
    "frostwright",
    "darlok",
    "thresholder",
    "tallykeep",
    "glyphward",
];

/// **THE SETTLEMENT BOUNDS, MEASURED.** Fifteen beats over the whole
/// founding roster; per line, the city its gates actually built.
#[test]
fn the_settlement_thresholds_measure_and_pin() {
    let root = scratch("settlement");
    let worlds = run_era(&root.to_string_lossy(), 15);
    assert!(worlds.len() >= 3, "the roster founded");

    let mut census: Vec<(String, usize)> = Vec::new();
    for polity in &worlds {
        let sigil = polity.spec.civilization.sigil.clone();
        let orb = polity.world.orb().expect("every polity holds its orb");
        let zones = orb.zones.len();
        let famished = !orb.famine.is_empty();
        census.push((sigil.clone(), zones));

        if FROZEN_AT_15.contains(&sigil.as_str()) {
            assert!(
                famished,
                "{sigil}: recorded frozen, measured fed — the energy debt \
                 moved; re-measure and shrink the pin"
            );
            continue;
        }
        // THE BOOTSTRAP LAW, held: every line the energy debt does not
        // claim builds past its founding inside the window…
        assert!(
            zones >= 2 && !famished,
            "{sigil}: stranded at {zones} zones (famine: {famished}) — \
             a gate stopped satisfying the bootstrap law"
        );
        // …and none runs past the largest city cap.
        assert!(zones <= 20, "{sigil}: {zones} zones exceeds every cap");
    }

    // The temperaments DISTINGUISH — the reserve factors buy
    // divergence or they buy nothing.
    let counts: Vec<usize> = census.iter().map(|(_, zones)| *zones).collect();
    assert!(
        counts.windows(2).any(|pair| pair[0] != pair[1]),
        "every temperament built the same city: {census:?}"
    );

    // THE MEASUREMENT, PINNED WHOLE: the era's zone total — the
    // reserve factors' aggregate footprint at beat 15, deterministic,
    // exact. A factor change lands here first and answers for its
    // whole-board effect in review.
    let total: usize = counts.iter().sum();
    assert_eq!(
        total, MEASURED_ZONE_TOTAL_AT_15,
        "the settlement footprint moved — a bound changed; re-measure \
         deliberately and move this pin with it: {census:?}"
    );
}
