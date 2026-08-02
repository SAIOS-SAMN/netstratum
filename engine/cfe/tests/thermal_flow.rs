//! THERMAL-1 witnesses (05 §1.1–§1.2): the equilibrium baseline drives the
//! flow model through the two-armed heat law — encase first, only the
//! overflow vents — with the opt-in guard, exact numbers end to end.

use cfe::dop::{ClosureEvent, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::{self, StarClass};
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

fn joules(world: &World) -> Ratio {
    world
        .accrued()
        .into_iter()
        .find(|(resource, _)| *resource == Resource::Joule)
        .map(|(_, amount)| amount)
        .unwrap_or_else(|| ratio(0, 1))
}

fn iron(world: &World) -> Ratio {
    world
        .accrued()
        .into_iter()
        .find(|(resource, _)| *resource == Resource::Iron)
        .map(|(_, amount)| amount)
        .unwrap_or_else(|| ratio(0, 1))
}

/// The opt-in guard (05 §1.2): a world with no sun ticks exactly as the
/// pre-thermal engine did — no joules, no overheat, nothing thermal.
#[test]
fn an_uncommissioned_world_is_thermally_dormant() {
    let mut world = World::new();
    world.found("dormant").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Encasement).expect("zones");
    for _ in 0..4 {
        let outcome = world.tick();
        assert!(outcome.overheat_struck.is_empty());
        assert!(outcome.overheat_cleared.is_empty());
    }
    assert_eq!(joules(&world), ratio(0, 1), "no sun ⇒ no minting");
    assert!(!world.orb().expect("orb").overheat);
}

/// A Sol-type mesic siting runs clean: heat_net = 1 + 1/2 − 2 < 0 —
/// no venting, no minting, full yields (05 §3.2).
#[test]
fn a_mesic_sol_world_runs_clean() {
    let mut world = World::new();
    world.found("meridian").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1), None, None)
        .expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    let outcome = world.tick();
    assert!(outcome.overheat_struck.is_empty());
    assert_eq!(joules(&world), ratio(0, 1));
    // Full extractor yield: +3/2 iron, −1/4 habitat upkeep = 5/4.
    assert_eq!(iron(&world), ratio(3, 2) - ratio(1, 4));
}

/// THE TWO-ARMED LAW, exact (05 §1.2): G star at r = 1/2 → stellar
/// load 4; extractor+habitat+shell factory heat 1/2; radiator 2 ⇒
/// heat_net 5/2. One shell encases 2 (the syntropic arm); overflow 1/2
/// vents (the entropic arm): efficiency 2/3, overheat strikes ONCE,
/// warm store leaks 1/8 ⇒ joules exactly 15/8 per tick.
#[test]
fn the_furnace_mints_and_only_the_overflow_vents() {
    let mut world = World::new();
    world.found("furnace").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 2), None, None)
        .expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    world.zone(16, ZoneKind::Encasement).expect("zones");

    let outcome = world.tick();
    assert_eq!(outcome.overheat_struck, vec![FIRST_ORB], "overheat strikes");
    assert!(world.orb().expect("orb").overheat);
    // Syntropic arm: 2 encased, minus the warm leak 1/8.
    assert_eq!(joules(&world), ratio(2, 1) - ratio(1, 8));
    // Entropic arm: yields degrade by exactly 2/3.
    // iron: 3/2 × 2/3 − 1/4 (hab) − 1/4 (shell upkeep) = 1/2.
    assert_eq!(iron(&world), ratio(1, 2));

    // Second tick: overheat persists (no second strike receipt), the
    // store accrues another exact 15/8.
    let outcome = world.tick();
    assert!(outcome.overheat_struck.is_empty(), "strike chronicles once");
    assert_eq!(joules(&world), ratio(15, 4));
    let strikes = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::OverheatStruck { .. }))
        .count();
    assert_eq!(strikes, 1);

    // Raze the extractor: factory drops 1/4 ⇒ heat_net 9/4, still over
    // one shell's throughput... raze the habitat too: heat_net 2 ⇒
    // overflow 0 ⇒ OverheatCleared chronicles, efficiency recovers.
    world.raze(0).expect("razes");
    world.raze(8).expect("razes");
    let outcome = world.tick();
    assert_eq!(outcome.overheat_cleared, vec![FIRST_ORB], "overheat clears");
    assert!(!world.orb().expect("orb").overheat);
    let cleared = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::OverheatCleared { .. }))
        .count();
    assert_eq!(cleared, 1);
}

/// The vault boundary (05 §1.2): the leak predicate flips exactly at
/// the cryo ceiling 100⁴ — lossless at and below, leaking above.
#[test]
fn the_vault_boundary_is_exact() {
    assert!(!thermal::leaks(&ratio(100_000_000, 1)), "100⁴ holds lossless");
    assert!(thermal::leaks(&ratio(100_000_001, 1)), "one K⁴ above leaks");
    // A brown-dwarf siting at r=1 lands in the vault.
    let t4 = thermal::t_eq4(
        &StarClass::BrownDwarf.luminosity(),
        &ratio(0, 1),
        &ratio(1, 1),
    );
    assert!(!thermal::leaks(&t4));
}

/// The thermal refusals are typed (05 status: the world refuses
/// instead of guessing).
#[test]
fn thermal_refusals_are_typed() {
    let mut world = World::new();
    world.found("refusals").expect("founds");
    assert_eq!(
        world.site_planet(FIRST_ORB, ratio(1, 2), ratio(1, 1), None, None),
        Err(ActionRefused::NoStarCommissioned)
    );
    world.commission_star(StarClass::K).expect("commissions");
    assert_eq!(
        world.commission_star(StarClass::G),
        Err(ActionRefused::StarAlreadyCommissioned)
    );
    assert_eq!(
        world.site_planet(FIRST_ORB, ratio(1, 1), ratio(1, 1), None, None),
        Err(ActionRefused::BadSiting),
        "albedo must be < 1"
    );
    assert_eq!(
        world.site_planet(FIRST_ORB, ratio(1, 2), ratio(0, 1), None, None),
        Err(ActionRefused::BadSiting),
        "orbit r must be > 0"
    );
    world
        .site_planet(FIRST_ORB, ratio(1, 2), ratio(1, 1), None, None)
        .expect("sites");
    assert_eq!(
        world.site_planet(FIRST_ORB, ratio(1, 2), ratio(2, 1), None, None),
        Err(ActionRefused::AlreadySited { orb: FIRST_ORB })
    );
}

/// SEAL B: the thermal world replays from receipts alone — star,
/// siting, overheat flag and all — and the orb round-trips byte-exact
/// with its thermal entry.
#[test]
fn the_thermal_world_replays_and_round_trips() {
    let mut world = World::new();
    world.found("replayed").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(1, 4), ratio(1, 2), None, None)
        .expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(16, ZoneKind::Encasement).expect("zones");
    world.tick();
    world.collect().expect("collects");

    let committed = world.committed();
    assert_eq!(committed.star, Some(StarClass::G));
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, committed, "replay MATCHES, thermal included");

    let orb = world.orb().expect("orb");
    assert!(orb.thermal.is_some());
    let bytes = orb.wire_bytes();
    let decoded = cfe::dop::ColonyOrb::from_wire(&bytes).expect("decodes");
    assert_eq!(&decoded, orb, "byte-exact round trip with tag 26");
    assert_eq!(decoded.wire_bytes(), bytes);

    // The banked joules are canonical: collected encasement persisted.
    assert!(orb.banked_amount(Resource::Joule) > ratio(0, 1));
}

/// VENT-1 conservation (05 §1.4): captured + vented = total excess,
/// exactly, and the vented ledger seals at collect via one receipt.
#[test]
fn the_vent_ledger_conserves_and_seals_at_collect() {
    let mut world = World::new();
    world.found("books").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 2), None, None)
        .expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    world.zone(16, ZoneKind::Encasement).expect("zones");
    // Per tick: excess 5/2 = encased 2 + vented 1/2. Two ticks.
    world.tick();
    world.tick();
    world.collect().expect("collects");
    let orb = world.orb().expect("orb");
    assert_eq!(orb.entropy_vented, ratio(1, 1), "2 × 1/2 vented, sealed");
    let banked_j = orb.banked_amount(Resource::Joule);
    assert_eq!(banked_j, ratio(15, 4), "2×2 encased − 2×1/8 warm leak");
    // The books: banked + leaked + vented = total excess. Exact.
    assert_eq!(
        banked_j + ratio(1, 4) + orb.entropy_vented.clone(),
        ratio(5, 1)
    );
    let vents = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::VentSettled { .. }))
        .count();
    assert_eq!(vents, 1, "one receipt, no per-tick bloat");
}

/// ECON-J (05 §1.2): a cold sited orb burns joules for heating; with
/// none banked, joule famine strikes and the orb FREEZE-LOCKS — no
/// flows at all until fuel arrives.
#[test]
fn a_cold_orb_burns_joules_and_freeze_locks() {
    let mut world = World::new();
    world.found("frost").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(0, 1), ratio(100, 1), None, None)
        .expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    let outcome = world.tick();
    assert!(
        outcome.struck.contains(&(FIRST_ORB, Resource::Joule)),
        "joule famine strikes: heating unpaid"
    );
    let iron_after_strike = iron(&world);
    world.tick();
    world.tick();
    assert_eq!(iron(&world), iron_after_strike, "frozen: no flows accrue");
}

/// SINK-1 (05 §1.4): a vault-band cryo array captures headroom as
/// bankable sink — exactly min(−heat_net, throughput) — and vaulted
/// banked stores pay NO demurrage.
#[test]
fn the_vault_captures_sink_and_charges_no_demurrage() {
    let mut world = World::new();
    world.found("vault").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(0, 1), ratio(100, 1), None, None)
        .expect("sites");
    world.zone(0, ZoneKind::CryoArray).expect("zones");
    world.tick();
    // headroom = 2 − 1/10000 = 19999/10000, one array (throughput 2).
    let sink = world
        .accrued()
        .into_iter()
        .find(|(resource, _)| *resource == Resource::Sink)
        .map(|(_, amount)| amount)
        .expect("sink accrued");
    assert_eq!(sink, ratio(19_999, 10_000));
    world.collect().expect("collects");
    assert_eq!(
        world.orb().expect("orb").banked_amount(Resource::Sink),
        ratio(19_999, 10_000)
    );
    // Let time pass; a second settle in the vault charges no demurrage.
    world.tick();
    world.tick();
    let _ = world.collect();
    let demurrage = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::DemurrageSettled { .. }))
        .count();
    assert_eq!(demurrage, 0, "the vault is lossless — no decay receipts");
}

/// SINK-1/Q2: un-vaulted banked stores decay by receipted demurrage at
/// touch — exact, open, never silent.
#[test]
fn warm_banked_joules_pay_receipted_demurrage_at_touch() {
    let mut world = World::new();
    world.found("demurrage").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 2), None, None)
        .expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    world.zone(16, ZoneKind::Encasement).expect("zones");
    world.tick();
    world.collect().expect("collects");
    let banked = world.orb().expect("orb").banked_amount(Resource::Joule);
    assert_eq!(banked, ratio(15, 8));
    let touch = world.orb().expect("orb").last_touch;
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("collects again");
    let elapsed = world.orb().expect("orb").last_touch - touch;
    assert_eq!(elapsed, 4);
    // FLAT decay = 1/32 × 4 = 1/8 (never proportional — the
    // denominator-explosion ruling), settled by receipt.
    let decayed = world
        .chronicle()
        .receipts()
        .iter()
        .find_map(|r| match &r.event {
            ClosureEvent::DemurrageSettled { resource: Resource::Joule, amount, .. } => {
                Some(amount.clone())
            }
            _ => None,
        })
        .expect("demurrage receipted");
    assert_eq!(decayed, ratio(1, 8));
    // And the whole story still replays.
    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt, world.committed(), "replay MATCHES with the dual ledger");
}

/// The two new events round-trip byte-exact (tags 27, 29).
#[test]
fn vent_and_demurrage_events_round_trip() {
    for event in [
        ClosureEvent::VentSettled { orb_id: 3, amount: ratio(7, 2) },
        ClosureEvent::DemurrageSettled {
            orb_id: 3,
            resource: Resource::Sink,
            amount: ratio(1, 8),
        },
    ] {
        let bytes = event.wire_bytes();
        let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
        assert!(tag == 27 || tag == 29);
        let decoded = ClosureEvent::from_wire(tag, value).expect("decodes");
        assert_eq!(decoded, event);
        assert_eq!(decoded.wire_bytes(), bytes);
    }
}
