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
/// RE-MEASURED 2026-08-16 to 86: COG-5 (cognitive demurrage). The leisure bank
/// now decays each beat (fill − bank·¼) instead of only filling, so the
/// `LeisureSettled` values differ — and since the chronicle digest seeds the
/// stochastic build/divergence choices, two marginal zones tip across the whole
/// roster (88 → 86). No minting has begun by beat 15 (rung 3 lands later), so
/// this is purely the digest reseed from the changed leisure values, not a
/// physical effect on builds. Per-line bootstrap law unchanged. Deliberate.
/// RE-MEASURED 2026-08-16 to 83: **THE DIOPHANTINE GENERATOR** (DIO-1). The
/// derived siting no longer hands mesic lines one of three fixed orbits
/// (r ∈ {1, 1, 6/5}); it draws a CONTINUOUS exact-rational scale k and sites
/// each world on its own track r = M·k² (siting.rs). So the derived biotic
/// roster now SPREADS across real thermal diversity — measured this era at
/// r ≈ 0.65 … 1.17, insolation ≈ 0.66 … 2.39 — where before every mesic line
/// sat at essentially unit orbit. Two effects move the total from 86 to 83:
/// the reseeded orbits perturb the digest-seeded build walk (as every
/// re-measure before), AND one line — `deep-chorus` — draws a HOT orbit
/// (r ≈ 0.646, insolation ≈ 2.39, ABOVE the mesic ceiling ≈ 1.73) whose
/// over-heat coverage penalty pulls its primary production below what feeds
/// its founding band, so it famishes. That is the hot-tail analogue of the
/// cold freeze-lock, and it is the exact phenomenon DIO-2's species-relative
/// flux η will formalize (a carbon-water line cannot take that heat; a
/// thermophile sited there would thrive). A deliberate re-measure under the
/// now-continuous orbital physics; the per-line bootstrap law is UNCHANGED.
/// RE-MEASURED 2026-08-16 to 81: **THE SPECIES-RELATIVE η** (DIO-2). Habitability
/// is no longer the shared 8-way band-distance halving; it is a CONTINUOUS
/// flux tent read on the exact `t_eq4` against EACH clade's own preferred-band
/// optimum (`habitability_flux`). The absolute habitable zone is gone, and the
/// board proves it: `deep-chorus` — DIO-1's hot-tail casualty — is now FED,
/// because its clade prefers the warm (Torrid) band, so the very orbit that
/// starved it under the shared zone SUITS it (the exact vindication the DIO-1
/// note predicted). In its place `coralline`, a temperate-preferring clade that
/// also drew a hot orbit (insolation ≈ 2.1), denatures below its founding
/// band — it cannot take the heat its neighbour loves. Same sun, opposite
/// outcomes. The famished COUNT is unchanged (9); the sharper, coherent tent
/// (now shared by primary_production AND the regrowth ceiling) trims two
/// marginal zones (83 → 81). A deliberate re-measure; bootstrap law UNCHANGED.
const MEASURED_ZONE_TOTAL_AT_15: usize = 81;

/// **THE ENERGY-BOOTSTRAP DEBT** — the lines the settlement law cannot
/// reach, pinned so the set can only SHRINK:
///
/// - cold-siting lines freeze-lock in joule famine at one CryoArray
///   (the Frost branch's own doc calls the freeze-lock "legitimate
///   drama, not a bug");
/// - Furnace / thermal holds and new sovereign seats that famish inside
///   the window (thresholder, tallykeep, glyphward) — recorded so a cure shrinks
///   this list rather than silently green-washing the debt;
/// - `coralline` (2026-08-16, DIO-2): the hot-orbit casualty under the
///   SPECIES-RELATIVE η. DIO-1 recorded `deep-chorus` here as the hot-tail
///   casualty and predicted its cure was species-relative habitability — and
///   that is exactly what happened: under DIO-2's flux tent `deep-chorus`
///   (its clade preferring the warm band) now THRIVES on that same hot orbit
///   and LEAVES this list, while `coralline` — a temperate-preferring clade
///   that likewise drew a hot track — denatures in its place. Same debt slot,
///   different occupant: the heat one clade loves is the heat another cannot
///   take. Its cure is not a settlement gate but its own orbit (a cooler
///   Diophantine draw) or a warm-adapted lineage.
const FROZEN_AT_15: [&str; 9] = [
    "corebound",
    "stillcold",
    "umbrid",
    "frostwright",
    "darlok",
    "thresholder",
    "tallykeep",
    "glyphward",
    "coralline",
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
