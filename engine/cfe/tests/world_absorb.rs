//! ABSORB-1 — THE ABSORPTION TRANSACTION (09_IDEOLOGY, BUILD_ORDER §3).
//! A hot orb's un-sealed overflow routes into a partner's banked sink
//! instead of the void: the 05 §1.4 invariant `sink_consumed =
//! heat_absorbed = vented_after_shed` made inter-orb. Every absorption
//! opens a reciprocity window (the MORALITY books): compensation
//! flowing back inside the window posts `returned_across`; the unpaid
//! remainder lapses by receipt to `extracted_across` — one mechanic,
//! two outcomes, measured by behavior. The engine judges nothing.

use cfe::dop::{ClosureEvent, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB, SOVEREIGN};

fn banked(world: &World, orb: u64, resource: Resource) -> Ratio {
    world
        .orb_by_id(orb)
        .expect("orb exists")
        .banked_amount(resource)
}

/// The furnace↔coldstore fixture: a hot source (ext+hab+shell at
/// r=1/2, overflow 1/2 per tick) funds its vault-band charted colony's
/// heating by TRADE-1 (the ECON-J relief law inter-orb); the charted
/// captures sink. Ends at stamp 19: source holds 1 un-sealed pending
/// overflow (struck), the charted banks 59997/10000 sink.
fn furnace_and_coldstore() -> (World, u64) {
    let mut world = World::new();
    world.found("furnace").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("collects"); // 30 iron, 13 biomass @16
    let charted = world.chart("coldstore", FIRST_ORB).expect("charts");
    world.zone(16, ZoneKind::Encasement).expect("zones the shell");
    world
        .zone_as(SOVEREIGN, charted, 0, ZoneKind::CryoArray)
        .expect("zones the array");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 2), None, None)
        .expect("sites hot");
    world
        .site_planet(charted, ratio(0, 1), ratio(100, 1), None, None)
        .expect("sites cold");
    // Tick @16: the furnace mints 15/8 and overflows 1/2 (struck); the
    // array captures 19999/10000 before the unheated charted's joule
    // famine strikes.
    world.tick();
    world.collect().expect("banks the joules"); // @17: seals vent 1/2
    // ECON-J relief by TRADE-1: one joule covers the heating bill —
    // and posts both crisis books (struck giver, struck receiver).
    world
        .transfer(FIRST_ORB, charted, Resource::Joule, ratio(1, 1))
        .expect("relieves the freeze");
    world.tick(); // @17: thawed — captures, burns 1/4
    world.tick(); // @18
    world
        .collect_as(SOVEREIGN, charted)
        .expect("banks the sink"); // @19
    (world, charted)
}

/// THE THREE-WAY INVARIANT (05 §1.4), inter-orb and exact:
/// sink_consumed = heat_absorbed = vented_after_shed — the absorber's
/// ledger carries the shame, the source's stays clean; emptying the
/// pending CLEARS the strike; the debt paid inside the window posts
/// the returned book and nothing lapses.
#[test]
fn the_absorption_invariant_measures_across_two_orbs() {
    let (mut world, charted) = furnace_and_coldstore();
    assert_eq!(banked(&world, charted, Resource::Sink), ratio(59_997, 10_000));
    assert_eq!(world.vent_pending_of(FIRST_ORB), ratio(1, 1));
    assert!(world.orb_by_id(FIRST_ORB).expect("orb").overheat);
    // The fixture's relief gift already posted both crisis books.
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").given_in_crisis,
        ratio(1, 1)
    );
    assert_eq!(
        world.orb_by_id(charted).expect("orb").drawn_in_crisis,
        ratio(1, 1)
    );

    world.absorb(FIRST_ORB, charted, ratio(1, 1)).expect("absorbs"); // @19

    // sink_consumed = 1:
    assert_eq!(banked(&world, charted, Resource::Sink), ratio(49_997, 10_000));
    // vented_after_shed = 1, on the ABSORBER's ledger:
    assert_eq!(
        world.orb_by_id(charted).expect("orb").entropy_vented,
        ratio(1, 1)
    );
    // heat_absorbed = 1 — the source's pending empties, and its own
    // vented ledger never grew past the pre-absorption seal:
    assert_eq!(world.vent_pending_of(FIRST_ORB), ratio(0, 1));
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").entropy_vented,
        ratio(1, 2)
    );
    // The strike clears — absorption is relief, receipted once.
    assert!(!world.orb_by_id(FIRST_ORB).expect("orb").overheat);
    let cleared = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::OverheatCleared { .. }))
        .count();
    assert_eq!(cleared, 1);
    // Compassion: the absorber gave while the source stood struck.
    assert_eq!(
        world.orb_by_id(charted).expect("orb").given_in_crisis,
        ratio(1, 1)
    );
    // The draw opened one debt...
    assert_eq!(world.committed().debts.len(), 1);
    // ...paid inside the window: the returned book posts, no lapse.
    world.collect().expect("collects"); // @19 — pending 0: no new seal
    world
        .transfer(FIRST_ORB, charted, Resource::Joule, ratio(1, 1))
        .expect("pays the debt");
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").returned_across,
        ratio(1, 1)
    );
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").extracted_across,
        ratio(0, 1)
    );
    assert!(world.committed().debts.is_empty());
    // Exactly ONE VentSettled ever — the void was refused the second
    // load.
    let vents = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::VentSettled { .. }))
        .count();
    assert_eq!(vents, 1);
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES across the axes");
}

/// An unpaid window lapses by receipt at its exact stamp — the
/// world-driven famine-strike shape — and the remainder posts to the
/// debtor's extraction book.
#[test]
fn an_unpaid_window_lapses_to_the_extraction_book() {
    let (mut world, charted) = furnace_and_coldstore();
    world.absorb(FIRST_ORB, charted, ratio(1, 1)).expect("absorbs"); // debt @19, window 8
    for _ in 0..8 {
        world.tick(); // now runs 19..26 — the window still open
    }
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").extracted_across,
        ratio(0, 1)
    );
    assert_eq!(world.committed().debts.len(), 1);
    world.tick(); // now = 27 = opened 19 + window 8 → LAPSE
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").extracted_across,
        ratio(1, 1)
    );
    assert!(world.committed().debts.is_empty());
    let lapse = world
        .chronicle()
        .receipts()
        .iter()
        .find_map(|r| match &r.event {
            ClosureEvent::AbsorptionLapsed { debtor_orb, amount, .. } => {
                Some((*debtor_orb, amount.clone(), r.stamp))
            }
            _ => None,
        })
        .expect("the lapse is receipted");
    assert_eq!(lapse, (FIRST_ORB, ratio(1, 1), 27));
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES with the lapse");
}

/// Partial absorption leaves the strike standing (the pending is not
/// empty); the second half clears it. Two draws open two windows; a
/// partial payment retires the OLDEST first; the unpaid remainder of
/// the younger lapses — returned + extracted sum to the drawn whole.
#[test]
fn partial_absorption_and_partial_pay_measure_exactly() {
    let (mut world, charted) = furnace_and_coldstore();
    world
        .absorb(FIRST_ORB, charted, ratio(1, 2))
        .expect("absorbs half");
    assert!(world.orb_by_id(FIRST_ORB).expect("orb").overheat, "half is not relief");
    assert_eq!(world.vent_pending_of(FIRST_ORB), ratio(1, 2));
    world
        .absorb(FIRST_ORB, charted, ratio(1, 2))
        .expect("absorbs the rest");
    assert!(!world.orb_by_id(FIRST_ORB).expect("orb").overheat);
    assert_eq!(world.committed().debts.len(), 2);
    // Both gives landed while the source stood struck: 1/2 + 1/2 = 1.
    assert_eq!(
        world.orb_by_id(charted).expect("orb").given_in_crisis,
        ratio(1, 1)
    );
    // Pay 1/2: the oldest debt dies whole; the younger stands.
    world
        .transfer(FIRST_ORB, charted, Resource::Joule, ratio(1, 2))
        .expect("pays half");
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").returned_across,
        ratio(1, 2)
    );
    let debts = world.committed().debts;
    assert_eq!(debts.len(), 1);
    assert_eq!(debts[0].remainder, ratio(1, 2));
    // The younger lapses at its stamp: the books split the whole.
    for _ in 0..9 {
        world.tick();
    }
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").returned_across,
        ratio(1, 2)
    );
    assert_eq!(
        world.orb_by_id(FIRST_ORB).expect("orb").extracted_across,
        ratio(1, 2)
    );
    assert!(world.committed().debts.is_empty());
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES with the split");
}

/// The typed refusals: the absorbable pending named exactly; the sink
/// shortfall named exactly; shape errors refuse whole; no phantom orbs.
#[test]
fn absorption_refuses_typed() {
    let mut world = World::new();
    world.found("meridian").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    for _ in 0..16 {
        world.tick();
    }
    world.collect().expect("collects");
    let charted = world.chart("outpost", FIRST_ORB).expect("charts");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 2), None, None)
        .expect("sites hot");
    // No shell: heat_net 5/2 all overflows on one tick.
    world.tick();
    assert_eq!(world.vent_pending_of(FIRST_ORB), ratio(5, 2));
    assert_eq!(
        world.absorb(FIRST_ORB, charted, ratio(5, 1)),
        Err(ActionRefused::CannotAbsorb { pending: Box::new(ratio(5, 2)) })
    );
    assert_eq!(
        world.absorb(FIRST_ORB, charted, ratio(1, 1)),
        Err(ActionRefused::CannotAfford {
            resource: Resource::Sink,
            need: Box::new(ratio(1, 1)),
            have: Box::new(ratio(0, 1)),
        })
    );
    assert_eq!(
        world.absorb(FIRST_ORB, FIRST_ORB, ratio(1, 1)),
        Err(ActionRefused::BadTransfer)
    );
    assert_eq!(
        world.absorb(FIRST_ORB, charted, ratio(0, 1)),
        Err(ActionRefused::BadTransfer)
    );
    assert_eq!(
        world.absorb(FIRST_ORB, charted, ratio(-1, 1)),
        Err(ActionRefused::BadTransfer)
    );
    assert_eq!(
        world.absorb(99, charted, ratio(1, 1)),
        Err(ActionRefused::NoSuchOrb { orb: 99 })
    );
    assert_eq!(
        world.absorb(FIRST_ORB, 99, ratio(1, 1)),
        Err(ActionRefused::NoSuchOrb { orb: 99 })
    );
}

/// Tags 31 and 32 round-trip byte-exact (the VENT-1 witness shape).
#[test]
fn absorb_and_lapse_events_round_trip_byte_exact() {
    for (event, expected_tag) in [
        (
            ClosureEvent::OverflowAbsorbed {
                source_orb: 1,
                absorber_orb: 2,
                amount: ratio(7, 2),
            },
            31,
        ),
        (
            ClosureEvent::AbsorptionLapsed {
                debtor_orb: 1,
                creditor_orb: 2,
                amount: ratio(3, 4),
            },
            32,
        ),
    ] {
        let bytes = event.wire_bytes();
        let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
        assert_eq!(tag, expected_tag);
        let decoded = ClosureEvent::from_wire(tag, value).expect("decodes");
        assert_eq!(decoded, event);
        assert_eq!(decoded.wire_bytes(), bytes);
    }
}
