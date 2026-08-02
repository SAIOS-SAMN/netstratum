//! THE MEDIUM AND THE APERTURE ARE DERIVED, NOT DIALED
//! (`19_LENS` §5b, §16 — tasks 15 and 16).
//!
//! Both arms had the same fault in the same shape: a quantity the law
//! calls a FACT ABOUT THE WORLD was arriving as an ARGUMENT ABOUT IT.
//! `Medium::from_surround(retained, shed)` let a caller pick η, and
//! `Aperture` was per-observer sensitivity that no observer carried. In
//! both cases two observers reading one chain could read differently,
//! which LAW 6 does not admit as evidence.
//!
//! These pins hold the derivation to committed state. The first two
//! would have failed to exist before this change rather than failed to
//! pass — there was no derivation to pin.

use cfe::dop::{ColonyOrb, Resource};
use cfe::ratio::ratio;
use cfe::world::Committed;
use lens::luminal::Aperture;
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

// ── THE APERTURE ──────────────────────────────────────────────────────

/// An observer with no history has no earned bias. Silence must read as
/// FLAT rather than as zero sensitivity — an unposted book teaches
/// nothing, and reading it as a zero would invent a signal.
#[test]
fn an_observer_with_no_books_carries_the_flat_instrument() {
    assert_eq!(Aperture::of_orb(&orb(1)), Aperture::flat());
}

/// The bias comes from the books and from nowhere else. Two observers
/// with different histories see through different instruments — which
/// is the per-observer half of LAW 9, and was unexercised until now.
#[test]
fn different_histories_grind_different_instruments() {
    let mut giver = orb(1);
    giver.given_in_crisis = ratio(4, 1);
    giver.returned_across = ratio(2, 1);
    giver.extracted_across = ratio(0, 1);

    let mut taker = orb(2);
    taker.given_in_crisis = ratio(0, 1);
    taker.returned_across = ratio(0, 1);
    taker.extracted_across = ratio(6, 1);

    let (giver, taker) = (Aperture::of_orb(&giver), Aperture::of_orb(&taker));
    assert_ne!(giver, taker);
    // Channel 0 is the reciprocity axis: all-given against all-taken.
    assert_eq!(giver.weight(0), ratio(1, 1));
    assert_eq!(taker.weight(0), ratio(0, 1));
}

/// Each grounded channel reads its OWN pair. A bond history must not
/// bleed into the covenant channel — that would be the fabricated
/// correspondence the derivation deliberately refuses.
#[test]
fn each_grounded_channel_reads_only_its_own_books() {
    let mut bonded = orb(1);
    bonded.bond_flow = ratio(5, 1);
    bonded.severance_load = ratio(0, 1);
    let aperture = Aperture::of_orb(&bonded);

    assert_eq!(aperture.weight(3), ratio(1, 1), "the bond channel moved");
    assert_eq!(
        aperture.weight(4),
        Aperture::flat().weight(4),
        "the covenant channel has no books and must not have moved"
    );
}

/// **THE LIMIT, PINNED AS LAW.** Eight relational books cannot honestly
/// fill eleven harmonic channels. The unmapped ones stay flat, and this
/// pin fails the day someone invents a correspondence to fill them.
#[test]
fn the_ungrounded_channels_stay_flat_however_loud_the_books_are() {
    let mut loud = orb(1);
    loud.given_in_crisis = ratio(99, 1);
    loud.returned_across = ratio(99, 1);
    loud.extracted_across = ratio(1, 1);
    loud.bond_flow = ratio(99, 1);
    loud.severance_load = ratio(1, 1);
    loud.covenants_fulfilled = ratio(99, 1);
    loud.covenants_breached = ratio(1, 1);

    let aperture = Aperture::of_orb(&loud);
    let flat = Aperture::flat();
    for channel in [1, 2, 5, 6, 7, 8, 9, 10] {
        assert_eq!(
            aperture.weight(channel),
            flat.weight(channel),
            "channel {channel} is not grounded in any book and must stay flat"
        );
    }
    // While the three that ARE grounded all moved off flat.
    for channel in [0, 3, 4] {
        assert_ne!(aperture.weight(channel), flat.weight(channel));
    }
}

/// The derivation is a function of committed state, so it is
/// reproducible — the same observer read twice reads the same.
#[test]
fn the_same_observer_grinds_the_same_instrument_twice() {
    let mut orb = orb(1);
    orb.covenants_fulfilled = ratio(7, 1);
    orb.covenants_breached = ratio(3, 1);
    assert_eq!(Aperture::of_orb(&orb), Aperture::of_orb(&orb));
    assert_eq!(Aperture::of_orb(&orb).weight(4), ratio(7, 10));
}
