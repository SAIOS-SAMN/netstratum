//! WHICH DIMENSIONS CARRY A RELATION AT ALL.
//!
//! **This file used to pin the per-dimension VERDICT** — that the
//! four-row classification had to hold in every channel rather than on
//! the sum, because a universe could read admitted in aggregate while
//! no single dimension was (bonds lending rank to debts, debts lending
//! flow back). Those pins went with the law they guarded: the
//! classification and its mint gate are struck (operator ruling
//! 2026-08-04 — identity is pegged to entity and is recycled without
//! bonds, so a standing test read off relations refused the lawful
//! recycled line by construction).
//!
//! The aggregation hazard they recorded is worth carrying forward in
//! words even though nothing now classifies: **a reading summed across
//! channels can hold a property no channel holds.** Anything that later
//! reads per-world structure should be measured per channel first.
//!
//! What remains testable here is the plain shape question:
//! [`occupied_dimensions`] — which channels any deed has ever used.

use cfe::dop::ColonyOrb;
use cfe::ratio::ratio;
use cfe::world::{AbsorptionDebt, Bond, Committed, World};
use kernel::bridge::{
    occupied_dimensions, project_complex, CHANNEL_BOND, CHANNEL_DEBT,
};

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
fn an_unused_channel_is_unoccupied_and_that_is_a_shape_not_a_failure() {
    // A channel no deed has ever used carries nothing, and saying so is
    // a fact about the world's shape. A universe that has never sealed a
    // covenant is not deficient for it.
    let mut state = orbs(3);
    state.bonds = vec![bond(1, 2, 4)];

    assert_eq!(occupied_dimensions(&state), vec![CHANNEL_BOND], "one channel in use");
}

#[test]
fn every_channel_a_deed_touched_is_occupied() {
    let mut state = orbs(4);
    state.bonds = vec![bond(1, 2, 3), bond(2, 3, 3)];
    state.debts = vec![debt(1, 2, 5)];

    let occupied = occupied_dimensions(&state);
    assert!(occupied.contains(&CHANNEL_BOND), "bonds stand");
    assert!(occupied.contains(&CHANNEL_DEBT), "debts stand");
    assert_eq!(occupied.len(), 2, "and nothing else: {occupied:?}");
}

#[test]
fn occupancy_reads_the_support_not_the_values() {
    // A relation carrying ZERO is still a relation — the support says
    // the pair relates, and that is the distinction sparsity exists to
    // carry (an absent relation is ABSENT, never a zero). A reading that
    // confused the two would erase exactly the topology `D` counts.
    let mut state = orbs(3);
    state.bonds = vec![bond(1, 2, 0)];

    let (_, support) = project_complex(&state);
    assert!(
        support.has(0, 1, CHANNEL_BOND),
        "the pair relates even though the obligation is zero"
    );
    assert_eq!(occupied_dimensions(&state), vec![CHANNEL_BOND]);
}

/// **THE ARITY ITSELF, pinned literally — the one place it should be.**
///
/// RESTORED 2026-08-04 at the integration audit. remediations-lane
/// replaced literal channel counts in these pins with `CHANNELS`, which
/// is right: a structural pin that hardcodes arity breaks on every
/// channel addition and teaches nothing. But deriving EVERYWHERE means
/// no pin can catch an unintended arity change, and the channel count
/// is wire-visible — `Delta` is `dim × dim × fiber` and every
/// projection reads it. So exactly one literal, here, deliberately.
///
/// **It lived in `capacity_law_pins.rs` and I deleted that file for its
/// LAW, taking this guard with it — which is the collateral the pin
/// existed to prevent.** Its author wrote that the count "becomes 4
/// when [CHANNEL_LANE] lands and the change is deliberate rather than
/// silent." That merge landed in this same integration, so the change
/// was made silently, exactly once, by the deletion. Recorded rather
/// than quietly corrected.
///
/// Changing `CHANNELS` should require editing this line and saying why.
#[test]
fn the_channel_arity_is_four_and_changing_it_is_an_amendment() {
    assert_eq!(
        kernel::bridge::CHANNELS,
        4,
        "BOND · FORWARD · DEBT · LANE. Was 3; remediations-lane added \
         CHANNEL_LANE as the fourth and it landed 2026-08-04. A fifth \
         channel edits this line and says why."
    );
}
