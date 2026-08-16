//! **THE SETTLEMENT LAW, WITNESSED** (P2, operator ruling 2026-08-06 as
//! rebuilt 2026-08-16).
//!
//! This began as an EXACT-count pin — a fixed 15-beat zone total and a
//! hard-coded frozen list. Under the species-relative physics (DIO-1/2/3a)
//! the chronicle digest reseeds the board on every change, so those numbers
//! churned every commit and could not distinguish an intended physics change
//! from a bug; they witnessed the retired flatlander-mesic era, not the law.
//! Per the operator's ruling — *don't bend the logic to esotericism; move the
//! guards to the logic we're implementing and remove the antiquated
//! methodology* — the exact pins are torn down. What remains is the LAW, which
//! is stable under any reseed because it IS the physics: no line runs away,
//! every FAMINE is explained by the species' own habitability (η < ½), a
//! viable line bootstraps, a majority of the roster bootstraps, and species
//! diverge. See `the_settlement_law_holds_under_species_physics`.

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

/// **THE SETTLEMENT LAW, WITNESSED** (rebuilt at DIO-3a per the operator's
/// ruling: *don't bend the logic to esotericism — move the guards to the logic
/// we're implementing and remove the antiquated methodology*).
///
/// The old form pinned the EXACT zone total and an EXACT hard-coded frozen
/// list. Under the species-relative physics (DIO-1/2/3a) the digest reseeds the
/// board on every change, so those numbers churned every commit and could not
/// tell an intended physics change from a bug — they were witnesses of the
/// retired flatlander-mesic era, not of the law. They are torn down.
///
/// What this witnesses instead is stable under any reseed because it is the
/// physics itself:
///   1. **No runaway** — no line exceeds the largest city cap.
///   2. **Famine is EXPLAINED** — every famished line reads habitability
///      η < ½: its own species cannot take its own orbit (a cold-pinned Frost
///      seat, or a temperate clade drawn hot). A famine at η ≥ ½ would be a
///      broken gate, not a species-thermal cull — THAT is the guard.
///   3. **Bootstrap** — a line whose species CAN take its world settles past
///      its founding (≥ 2 zones), and a strong MAJORITY of the roster does.
///   4. **Species diverge** — the physics builds different cities, never one
///      roster of identical settlements.
#[test]
fn the_settlement_law_holds_under_species_physics() {
    let root = scratch("settlement");
    let worlds = run_era(&root.to_string_lossy(), 15);
    assert!(worlds.len() >= 3, "the roster founded");

    let orb_id = cfe::world::FIRST_ORB;
    let viable = cfe::ratio::ratio(1, 2);
    let mut zones_seen: Vec<usize> = Vec::new();
    let (mut fed, mut famished) = (0usize, 0usize);

    for polity in &worlds {
        let sigil = &polity.spec.civilization.sigil;
        let world = &polity.world;
        let orb = world.orb().expect("every polity holds its orb");
        let zones = orb.zones.len();
        zones_seen.push(zones);

        // 1. NO RUNAWAY.
        assert!(zones <= 20, "{sigil}: {zones} zones exceeds every cap");

        if orb.famine.is_empty() {
            // 3. A viable line BOOTSTRAPS.
            fed += 1;
            assert!(zones >= 2, "{sigil}: fed but stranded at {zones} zones — a gate stopped bootstrapping");
        } else {
            // 2. A famine must be EXPLAINED by the species-relative habitability.
            famished += 1;
            let t_eq4 = world.insolation_of(orb_id) * cfe::thermal::thermal_k();
            let eta = cfe::almanac::habitability_flux(world.rejuvenation_clade(), &t_eq4);
            assert!(
                eta < viable,
                "{sigil}: famished at η = {eta} (≥ ½) — famine on a HABITABLE orbit \
                 is a broken gate, not a species-thermal cull"
            );
        }
    }

    // 3. The roster bootstraps a strong majority; the thermal debt is the minority.
    assert!(fed > famished, "the roster failed to bootstrap: {fed} fed vs {famished} famished");

    // 4. Species diverge.
    assert!(
        zones_seen.windows(2).any(|pair| pair[0] != pair[1]),
        "every line built the same city — the physics stopped distinguishing species"
    );
}
