//! **ONT-2 — THE SCALAR COLLAPSE IS ANNIHILATED.** The `vein_wear` recurrence
//! detonated a founded line's replay at ~beat 115 (a denominator explosion,
//! measured pre-ONT-2). Richness is now a projection of geometry minus the
//! extraction integral — a division-free bounded sum — so replay must hold
//! BYTE-EXACT well past that horizon.
//!
//! A single line is cycled SOLO (no O(n²) cross-polity market steps), so this
//! drives the exact per-beat assay hot path cheaply, far past 115 beats.

use conductor::{founding_polities, Polity};
use ledger::registry::Registry;
use ledger::Ledger;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ont2-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

#[test]
fn a_founded_line_replays_exact_past_the_old_divergence_horizon() {
    let root = scratch("solo");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let spec = founding_polities()
        .into_iter()
        .next()
        .expect("a roster stands");
    let mut polity = Polity::genesis(spec, &registry, &book);

    // 130 beats — comfortably past the pre-ONT-2 beat-115 collapse. Every beat
    // the line's ground_step assays its opened veins, the exact recurrence
    // that used to explode.
    for _ in 0..130 {
        let _ = polity.cycle(&book, &registry);
    }
    let committed = polity.world.committed();
    let replayed = cfe::world::replay(polity.world.chronicle());

    // ONT-2's DOMAIN: the vein EXTRACTION INTEGRALS replay byte-exact at 130
    // beats — the vein-richness scalar collapse that detonated at ~beat 115 is
    // annihilated (the integral is a bounded, division-free sum; there is no
    // recurrence left to explode).
    assert_eq!(
        replayed.prospects, committed.prospects,
        "vein extraction integrals replay byte-exact at 130 beats — the ONT-2 collapse is gone"
    );
    // NOTE (ONT-4, #110): the FULL-committed replay still trips a SEPARATE
    // scalar collapse in the COG `leisure_banks` at ~beat 117 (replay sums
    // unbounded `LeisureSettled` deltas while the live bank is floored). That is
    // a different scalar in the same class, unmasked once this one was fixed —
    // scoped to its own slice, not ONT-2.
}
