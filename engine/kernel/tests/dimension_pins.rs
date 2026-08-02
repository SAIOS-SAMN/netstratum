//! WITNESSED IN EVERY DIMENSION — the absolute, and why the aggregate
//! could not carry it.
//!
//! The verdict must hold per dimension, not on the sum. These pins
//! measure a state where the SUM reads admitted while no single
//! dimension is.

use cfe::dop::ColonyOrb;
use cfe::ratio::ratio;
use cfe::world::{AbsorptionDebt, Bond, Committed, World};
use kernel::bridge::{
    occupied_dimensions, read_entangled, verdict_per_dimension, CHANNEL_BOND, CHANNEL_FORWARD,
    CHANNEL_DEBT,
};
use rfa_core::Vacuity;

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

fn debt(debtor: u64, creditor: u64, remainder: i64) -> AbsorptionDebt {
    AbsorptionDebt { debtor, creditor, remainder: ratio(remainder, 1), opened: 0 }
}

#[test]
fn the_aggregate_verdict_can_read_admitted_while_no_dimension_is() {
    // THE HOLE, measured. Bonds form a balanced ring — rank, no flow,
    // so that dimension is in STASIS. Debts form an open pair — flow,
    // no cycle, so that dimension is UNFRAMED. Neither carries both
    // poles, and neither is admitted.
    //
    // But the aggregate sums them: rank comes from bonds, flow comes
    // from debts, and `dual_pole` on the totals reads ENTANGLED. The
    // sum is admitted while every part of it is dead — one dimension
    // lending its rank to another that lends back its flow.
    let mut state = orbs(4);
    state.bonds = vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3)];
    state.debts = vec![debt(1, 2, 5)];

    let whole = read_entangled(&state);
    let per = verdict_per_dimension(&state);

    assert_eq!(per[CHANNEL_BOND], Vacuity::Stasis, "bonds: rank without gradient");
    assert_eq!(per[CHANNEL_DEBT], Vacuity::Unframed, "debts: gradient without rank");
    assert_eq!(per[CHANNEL_FORWARD], Vacuity::Void, "forwards: unoccupied");
    assert!(
        !per.iter().any(|seen| seen.is_admitted()),
        "NOT ONE dimension is admitted on its own"
    );
    // And the aggregate borrowed across them. Whatever the summed
    // verdict says, it is not a statement about any dimension.
    assert!(
        whole.rank > 0 && !whole.flows.is_empty(),
        "the sum holds both poles only because two dead dimensions each supplied one"
    );
}

#[test]
fn a_dimension_that_carries_both_poles_is_admitted_on_its_own() {
    // The honest case. The bond channel alone holds a ring plus a
    // chord: a second cycle AND unbalanced vertices, both poles in the
    // same dimension. It stands without borrowing from anywhere.
    let mut state = orbs(4);
    state.bonds =
        vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3), bond(1, 3, 1)];

    let per = verdict_per_dimension(&state);
    assert_eq!(per[CHANNEL_BOND], Vacuity::Entangled, "one dimension, both poles, on its own");
}

#[test]
fn an_unoccupied_dimension_reads_void_and_that_is_a_shape_not_a_failure() {
    // A channel no deed has ever used carries nothing, and saying so is
    // a fact about the world's shape. A universe that has never sealed
    // a forward is not misaligned for it — which is why the floor is
    // read per dimension and never as one global gate.
    let mut state = orbs(3);
    state.bonds = vec![bond(1, 2, 4)];

    assert_eq!(occupied_dimensions(&state), vec![CHANNEL_BOND], "one channel in use");
    let per = verdict_per_dimension(&state);
    assert_eq!(per[CHANNEL_FORWARD], Vacuity::Void);
    assert_eq!(per[CHANNEL_DEBT], Vacuity::Void);
}

#[test]
fn the_floor_is_a_classification_never_a_score() {
    // THE ANTI-ABSOLUTISM PIN. The reading returns an enum per
    // dimension — it does not rank states, order them, or produce a
    // number any rule could maximise, and it never says WHICH relation
    // to form. Two worlds three orders of magnitude apart in obligation
    // size land on the same row: the row carries no ordering to climb.
    let mut small = orbs(3);
    small.bonds = vec![bond(1, 2, 1), bond(2, 3, 1), bond(1, 3, 5)];
    let mut large = orbs(3);
    large.bonds = vec![bond(1, 2, 900), bond(2, 3, 900), bond(1, 3, 4000)];

    let (a, b) = (verdict_per_dimension(&small), verdict_per_dimension(&large));
    assert_eq!(a[CHANNEL_BOND], b[CHANNEL_BOND], "same row, wildly different magnitudes");
}

#[test]
fn the_reading_is_absolute_decidable_at_one_state_with_no_history() {
    // ABSOLUTE, NOT RATE. The verdict is a function of ONE committed
    // state: no prior, no window, no limit being approached — which is
    // what lets it be an invariant at all. A rate never attains its
    // bound at any single step, so a rate can only ever be a lens.
    let mut state = orbs(4);
    state.bonds =
        vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3), bond(1, 3, 1)];

    assert_eq!(
        verdict_per_dimension(&state),
        verdict_per_dimension(&state),
        "same state, same verdict, no history consulted"
    );
}
