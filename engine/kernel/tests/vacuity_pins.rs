//! THE FOUR ROWS, measured on real committed state.
//!
//! `dual_pole(rank, form)` classifies a state on both poles at once.
//! These pins show that all four rows occur naturally in states this
//! tree already pinned for other reasons — and that the canonical
//! `D > 0` proof of `18_TORSION` §8 lands on **Stasis**, not on the
//! admitted row.

use cfe::dop::ColonyOrb;
use cfe::ratio::ratio;
use cfe::world::{Bond, Committed, World};
use kernel::bridge::{project_complex, read_entangled};
use rfa_core::{dual_pole, boundary_form, Vacuity};

fn orbs(n: u64) -> Committed {
    let mut committed = World::new().committed();
    committed.orbs = (1..=n)
        .map(|orb_id| ColonyOrb { orb_id, name: format!("orb{orb_id}"), ..Default::default() })
        .collect();
    committed
}

fn bond(source: u64, target: u64, rate: i64) -> Bond {
    Bond {
        source,
        target,
        resource: cfe::dop::Resource::Iron,
        rate: ratio(rate, 1),
        term: 0,
        sealed: 0,
        anchored: 0,
    }
}

fn verdict(state: &Committed) -> (usize, usize, Vacuity) {
    let (delta, support) = project_complex(state);
    let form = boundary_form(&delta, &support);
    let rank = read_entangled(state).rank;
    let seen = dual_pole(rank, &form);
    (rank, form.rank(), seen)
}

#[test]
fn the_canonical_proof_of_d_above_zero_is_stasis() {
    // THE FINDING. `18_TORSION` §8 proves D > 0 with a four-orb ring of
    // equal bonds. Each orb owes exactly what it is owed, so the
    // divergence cancels at EVERY vertex and `∂V ≡ 0`.
    //
    // So the simplest state carrying non-zero rank is pure circulation —
    // and pure circulation is precisely the row the dual-pole verdict
    // REFUSES. That is not a defect in either piece; it is the
    // anti-dogma law stated structurally: **going in circles produces
    // structure without gradient.** The §8 proof shows the rank reading
    // works; it does not exhibit an admissible universe.
    let mut ring = orbs(4);
    ring.bonds = vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3)];

    let (rank, flows, seen) = verdict(&ring);
    assert_eq!(rank, 1, "the ring carries one independent cycle");
    assert_eq!(flows, 0, "and every vertex nets to zero: pure circulation");
    assert_eq!(seen, Vacuity::Stasis, "rank without gradient — the state to re-volumize");
    assert!(!seen.is_admitted());
}

#[test]
fn the_admitted_row_was_already_pinned_it_is_the_chord() {
    // And the Entangled example was sitting in `bridge_pins` all along,
    // under a name that undersold it. Adding the missing 1—3 obligation
    // breaks the balance at two vertices AND closes a second cycle:
    // both poles non-vacuous, which is the only admitted row.
    let mut chord = orbs(4);
    chord.bonds =
        vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3), bond(1, 3, 1)];

    let (rank, flows, seen) = verdict(&chord);
    assert_eq!(rank, 2, "the chord closes a second independent cycle");
    assert_eq!(flows, 2, "and unbalances two vertices: dV ≢ 0");
    assert_eq!(seen, Vacuity::Entangled);
    assert!(seen.is_admitted(), "structure AND gradient — the only row that stands");
}

#[test]
fn an_open_chain_has_gradient_but_no_structure() {
    // Unframed: obligations flow from a source to a sink, so `dV ≢ 0`,
    // but a chain has no cycle and therefore no rank. Potential going
    // somewhere, with no shape to hold it.
    let mut chain = orbs(3);
    chain.bonds = vec![bond(1, 2, 3), bond(2, 3, 3)];

    let (rank, flows, seen) = verdict(&chain);
    assert_eq!(rank, 0);
    assert_eq!(flows, 2, "a source and a sink");
    assert_eq!(seen, Vacuity::Unframed);
    assert!(!seen.is_admitted());
}

#[test]
fn a_world_that_owes_nothing_is_void() {
    let (rank, flows, seen) = verdict(&orbs(3));
    assert_eq!((rank, flows), (0, 0));
    assert_eq!(seen, Vacuity::Void, "neither pole: nothing to stand on");
    assert!(!seen.is_admitted());
}

#[test]
fn the_certified_fold_cannot_be_the_re_volumizer() {
    // WHY refold IS NOT THE SEAM. `refold(fold_certified(x)) == x` is a
    // witnessed exact identity (`bridge_pins`), so the round trip
    // returns a pure-circulation field as a pure-circulation field. The
    // certificate's potentials are DERIVED FROM the state, so it can
    // never carry gradient the state did not already have.
    //
    // Restoration is not generation. Whatever lifts Stasis must ADD a
    // relation, not put back one that was folded away.
    let mut ring = orbs(4);
    ring.bonds = vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3)];
    let before = verdict(&ring);

    // The one landed move that lifts it: an asymmetric obligation.
    ring.bonds.push(bond(1, 3, 1));
    let after = verdict(&ring);

    assert_eq!(before.2, Vacuity::Stasis);
    assert_eq!(after.2, Vacuity::Entangled, "a new relation, not a restored one, lifts stasis");
}
