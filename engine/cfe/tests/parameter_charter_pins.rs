//! **THE CHARTERED PARAMETERS, PINNED** (P2, tag 103).
//!
//! The law in four claims: the band is a DOOR (a value outside the
//! fourfold band of its seed cannot be constructed — the sharper
//! claim); chartering is a TIER 0 power (the standing FACT at the
//! verb); the fold reads seed ∪ charters, last-writer-wins, and the
//! TICK OBEYS IT (a chartered rate demonstrably moves the world — the
//! vacuity bar, 27 LAW 8); and the whole arc replays and resumes
//! (SEAL B — the receipt is the state).

use cfe::parameter::{chartered, of, Chartered, Parameter};
use cfe::ratio::ratio;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

/// A world with a T0 progenitor seated, ready to charter.
fn seated() -> World {
    let mut world = World::new();
    world.found("charter-hold").expect("founds");
    world
        .seat_station(
            cfe::dop::name("tunborn"),
            cfe::dop::Station::from_wire(1).expect("a band office"),
        )
        .expect("seats the progenitor");
    world
}

/// **THE BAND IS A DOOR** — outside the fourfold band of the seed, the
/// case cannot be constructed; inside, the literal door vouches.
#[test]
fn the_band_cannot_be_left() {
    // HeatingPerZone seeds at 1/4: the band is [1/16, 1].
    assert!(Chartered::new(Parameter::HeatingPerZone, ratio(2, 1)).is_none());
    assert!(Chartered::new(Parameter::HeatingPerZone, ratio(1, 32)).is_none());
    assert!(Chartered::new(Parameter::HeatingPerZone, ratio(1, 2)).is_some());
    // The band anchors to the SEED — charters cannot compound past it.
    let _ = chartered(Parameter::RadiatorBase, ratio(8, 1)); // 4 × seed, the edge
}

/// **CHARTERING IS A TIER 0 POWER** — the standing fact, the same law
/// as the vocabulary mint.
#[test]
fn only_a_progenitor_charters() {
    let mut world = seated();
    assert!(matches!(
        world.charter_parameter("nobody", chartered(Parameter::VeilToll, ratio(1, 2))),
        Err(ActionRefused::MintWithoutStanding { .. })
    ));
    assert!(world
        .charter_parameter("tunborn", chartered(Parameter::VeilToll, ratio(1, 2)))
        .is_ok());
}

/// **THE FOLD READS SEED ∪ CHARTERS, LAST-WRITER-WINS** — and an
/// uncharted key reads its seed, exactly.
#[test]
fn the_fold_is_seed_then_charters() {
    let mut world = seated();
    assert_eq!(
        of(world.chronicle(), Parameter::HeatingPerZone),
        Parameter::HeatingPerZone.seed(),
        "unchartered reads the seed"
    );
    world
        .charter_parameter("tunborn", chartered(Parameter::HeatingPerZone, ratio(1, 2)))
        .expect("charters");
    assert_eq!(of(world.chronicle(), Parameter::HeatingPerZone), ratio(1, 2));
    world
        .charter_parameter("tunborn", chartered(Parameter::HeatingPerZone, ratio(1, 8)))
        .expect("re-charters");
    assert_eq!(
        of(world.chronicle(), Parameter::HeatingPerZone),
        ratio(1, 8),
        "the last charter rules"
    );
    // Other keys are untouched — one charter moves one rule.
    assert_eq!(
        of(world.chronicle(), Parameter::RadiatorBase),
        Parameter::RadiatorBase.seed()
    );
}

/// **THE TICK OBEYS THE CHARTER** — the vacuity bar (27 LAW 8): a
/// chartered encasement throughput demonstrably changes what a hot
/// world encases. Two worlds identical but for the charter diverge in
/// their accrued joules.
#[test]
fn a_chartered_rate_reaches_the_tick() {
    let run = |charter: bool| -> cfe::ratio::Ratio {
        let mut world = seated();
        world.commission_star(cfe::thermal::StarClass::G).expect("commissions");
        world
            .site_planet(FIRST_ORB, cfe::dop::unit(1, 2), cfe::dop::positive(1, 2), None, None)
            .expect("sites hot — heat_net stands positive");
        // One pair + one shell runs heat_net 1/2 under the seeded
        // radiator (2). Chartering the radiator UP to 5/2 closes the
        // budget exactly — nothing overflows, nothing encases — so the
        // two worlds' accrued joules diverge by the whole flow.
        world.zone(0, cfe::dop::ZoneKind::Extractor).expect("zones");
        world.zone(8, cfe::dop::ZoneKind::Habitat).expect("zones");
        world.zone(16, cfe::dop::ZoneKind::Encasement).expect("zones");
        if charter {
            world
                .charter_parameter(
                    "tunborn",
                    chartered(Parameter::RadiatorBase, ratio(5, 2)),
                )
                .expect("charters");
        }
        for _ in 0..8 {
            world.tick();
        }
        world
            .accrued()
            .into_iter()
            .find(|(resource, _)| *resource == cfe::dop::Resource::Joule)
            .map(|(_, amount)| amount)
            .unwrap_or_else(|| ratio(0, 1))
    };
    let seeded = run(false);
    let radiated = run(true);
    assert!(
        seeded > radiated,
        "a chartered radiator must shed what the shell would encase: {seeded} vs {radiated}"
    );
}

/// **SEAL B** — the charter arc replays and resumes; and tag 103 rides
/// the wire byte-exact.
#[test]
fn the_charter_replays_and_rides_the_wire() {
    let mut world = seated();
    world
        .charter_parameter("tunborn", chartered(Parameter::FrictionRate, ratio(1, 4)))
        .expect("charters");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.committed(), world.committed(), "and resumes");
    assert_eq!(
        of(resumed.chronicle(), Parameter::FrictionRate),
        ratio(1, 4),
        "the resumed fold carries the charter"
    );

    let event = cfe::dop::ClosureEvent::ParameterChartered {
        key: Parameter::SinkThroughput.code(),
        value: ratio(22, 7),
    };
    let bytes = event.wire_bytes();
    let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
    assert_eq!(tag, 103);
    let decoded = cfe::dop::ClosureEvent::from_wire(tag, value).expect("decodes");
    assert_eq!(decoded, event);
    assert_eq!(decoded.wire_bytes(), bytes, "byte-faithful");
    // An unknown key refuses at decode — never a guess.
    assert!(cfe::dop::ClosureEvent::from_wire(103, &[99u8, 1, 1]).is_none());
}
