//! THE OPENED GROUND — `D`'s floor is the VOLUME of the estate, not the
//! point (operator ruling 2026-08-05: *GROUND needs to be opened up into
//! V = Volume of orbs, weight on D as capacity*).
//!
//! The law, as arithmetic:
//!
//!     D  =  volume(orbs) × (GROUND + circulation)
//!
//! Derived from the code's own premises, not from a document:
//!
//! - every committed family keys on orbs (`Committed`: banks, zones,
//!   famine live ON `ColonyOrb`; bonds/debts/forwards key orb ids), so
//!   the orbs are the substance's extensive content and V's volume is
//!   their measure;
//! - `Degree` is never zero (2026-08-01) — preserved: volume ≥ 1 for
//!   any state, and the empty pre-founding state weighs 1;
//! - the gate reads `Degree::above()` — the circulation, deliberately
//!   never `get()` (`revocation_law`'s own comment) — so opening the
//!   floor changes NOTHING the gate admits: `above()` still returns the
//!   circulation, and the fold gate compares ranks. That derivation makes
//!   this a PATCH under the semver law, and the pins below hold it.
//!
//! At one orb the product is `1 × (GROUND + c)` — byte-identical to the
//! old reading, so the widening is provably inert at the base and grows
//! only as the estate does. Weight means MULTIPLY: founding an orb
//! multiplies D, which is the compounding every expansion ruling asked
//! for (E5, `ENDOWMENTS` §5).

use cfe::dop::{ColonyOrb, Resource};
use cfe::ratio::ratio;
use cfe::world::{Bond, Committed, World};
use kernel::bridge::read_entangled;

/// A state holding `orbs` colonies and nothing else — zero circulation.
fn estate(orbs: u64) -> Committed {
    let mut state: Committed = World::new().committed();
    state.orbs = (1..=orbs)
        .map(|orb_id| ColonyOrb { orb_id, name: format!("orb{orb_id}"), ..Default::default() })
        .collect();
    state
}

/// The same estate carrying one independent circulation: the
/// non-composing triangle (1 + 2 ≠ 7), holonomy no frame explains.
fn estate_with_loop(orbs: u64) -> Committed {
    let mut state = estate(orbs.max(3));
    let bond = |source, target, rate: i64| Bond {
        source,
        target,
        resource: Resource::Iron,
        rate: ratio(rate, 1),
        term: 0,
        sealed: 0,
        anchored: 0,
    };
    state.bonds = vec![bond(1, 2, 1), bond(2, 3, 2), bond(1, 3, 7)];
    state
}

#[test]
fn the_floor_is_the_volume_of_the_estate_not_the_point() {
    // Zero circulation everywhere; ONLY the estate differs.
    let one = read_entangled(&estate(1));
    let three = read_entangled(&estate(3));
    assert_eq!(one.degree().above(), 0, "nothing circulates");
    assert_eq!(three.degree().above(), 0, "nothing circulates here either");
    assert_eq!(one.degree().get(), 1, "one orb: the point survives as the base case");
    assert_eq!(
        three.degree().get(),
        3,
        "three orbs at rest stand on a floor of three — the ground OPENED"
    );
}

#[test]
fn weight_means_multiply_not_add() {
    // One independent loop on a 3-orb estate: D = 3 × (1 + 1) = 6.
    let read = read_entangled(&estate_with_loop(3));
    assert_eq!(read.degree().above(), 1, "one loop above the floor");
    assert_eq!(
        read.degree().get(),
        6,
        "the volume WEIGHTS the whole degree: 3 × (GROUND + 1)"
    );
}

#[test]
fn never_zero_survives_the_opening() {
    // The pre-founding state holds no orb and still weighs one.
    let empty = read_entangled(&World::new().committed());
    assert_eq!(empty.degree().get(), 1, "an empty state still stands on the floor");
    assert!(empty.degree().is_ground());
}

#[test]
fn the_gate_reads_circulation_and_nothing_admitted_changes() {
    // The invariance that made 0.5.1 a PATCH: `above()` is the
    // circulation regardless of volume, so every D4 comparison — and
    // any arm reading above() — is untouched by the estate's size.
    let small = read_entangled(&estate_with_loop(3));
    let mut wide_state = estate_with_loop(3);
    wide_state.orbs.push(ColonyOrb { orb_id: 9, name: "annex".into(), ..Default::default() });
    let wide = read_entangled(&wide_state);
    assert_eq!(
        small.degree().above(),
        wide.degree().above(),
        "the gauge-invariant part does not move with volume"
    );
    assert!(
        wide.degree().get() > small.degree().get(),
        "while the CAPACITY reading grows with the estate"
    );
}
