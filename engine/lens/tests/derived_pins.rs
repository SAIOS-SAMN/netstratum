//! THE MEDIUM IS DERIVED, NOT DIALED (`19_LENS` §5b — task 15).
//!
//! The observer's own instrument is pinned in `books_pins.rs`: it was
//! first built onto the harmonic channels, which was a layer error, and
//! it now stands on the OPEN book set instead.
//!
//! A quantity the law calls a FACT ABOUT THE WORLD was arriving as an
//! ARGUMENT ABOUT IT: `Medium::from_surround(retained, shed)` let a
//! caller pick η, so two observers reading one chain could read
//! different paces — which LAW 6 does not admit as evidence.
//!
//! These pins hold the derivation to committed state. They would have
//! failed to EXIST before this change rather than failed to pass —
//! there was no derivation to pin.

use cfe::dop::{ColonyOrb, Resource};
use cfe::ratio::ratio;
use cfe::world::Committed;
use lens::propagation::Medium;

fn orb(id: u64) -> ColonyOrb {
    ColonyOrb {
        orb_id: id,
        name: format!("orb-{id}"),
        ..ColonyOrb::default()
    }
}

fn world(orbs: Vec<ColonyOrb>) -> Committed {
    Committed {
        orbs,
        ..Committed::default()
    }
}

// ── THE MEDIUM ────────────────────────────────────────────────────────

/// The point of the whole change: one world, one η, no matter who asks.
/// A dialed medium could not have made this true.
#[test]
fn one_world_reads_one_medium_for_every_observer() {
    let mut a = orb(1);
    a.banked = vec![(Resource::Joule, ratio(6, 1))];
    a.entropy_vented = ratio(2, 1);
    let state = world(vec![a]);

    // Two observers, two separate derivations, no shared argument.
    let first = Medium::of(&state);
    let second = Medium::of(&state);
    assert_eq!(first, second);
    assert_eq!(*first.eta(), ratio(3, 1), "retained 6 against shed 2");
}

/// A world that keeps more than it bleeds pushes what moves through it;
/// one bleeding faster than it keeps drags. Both poles pinned, so a
/// polarity inversion cannot pass as it did in the fold gate.
#[test]
fn retention_accelerates_and_bleeding_damps() {
    let mut keeping = orb(1);
    keeping.banked = vec![(Resource::Sink, ratio(10, 1))];
    keeping.entropy_vented = ratio(1, 1);
    assert!(Medium::of(&world(vec![keeping])).accelerates());

    let mut bleeding = orb(2);
    bleeding.banked = vec![(Resource::Joule, ratio(1, 1))];
    bleeding.entropy_vented = ratio(10, 1);
    assert!(Medium::of(&world(vec![bleeding])).decelerates());
}

/// Both banked resources count, and the sum is across the whole world:
/// the medium is the surround, not one orb's corner of it.
#[test]
fn the_medium_sums_both_retained_resources_across_every_orb() {
    let mut a = orb(1);
    a.banked = vec![(Resource::Joule, ratio(3, 1)), (Resource::Sink, ratio(1, 1))];
    a.entropy_vented = ratio(1, 1);
    let mut b = orb(2);
    b.banked = vec![(Resource::Joule, ratio(4, 1))];
    b.entropy_vented = ratio(3, 1);

    // retained 3+1+4 = 8, shed 1+3 = 4.
    assert_eq!(*Medium::of(&world(vec![a, b])).eta(), ratio(2, 1));
}

/// A world that has vented nothing is INERT — the surround has taken
/// nothing away yet. Not infinitely fast, and not a division by zero.
#[test]
fn a_world_that_has_shed_nothing_is_inert_not_infinite() {
    let mut rich = orb(1);
    rich.banked = vec![(Resource::Joule, ratio(9_000, 1))];
    assert_eq!(Medium::of(&world(vec![rich])), Medium::inert());
    assert_eq!(Medium::of(&world(vec![])), Medium::inert());
}
