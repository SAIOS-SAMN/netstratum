//! **ZONE VOCABULARY (#41) slice 2 — a minted building that BUILDS.**
//!
//! Slice 1 gave a world the words. This is the half where a minted kind
//! costs something to raise, yields something once standing, refunds
//! what it can be salvaged for, and survives replay — the test of
//! whether the vocabulary is load-bearing or decoration.
//!
//! The economics are the operator's ruling made concrete: a minted
//! kind's numbers are its BASE's, and its binding modulates what the
//! building DOES. What it costs to raise is the base's alone, because
//! the binding prices operation rather than construction.

use cfe::dop::{name, Resource, Station, ZoneKind, ZONE_KINDS};
use cfe::ratio::ratio;
use cfe::vocabulary::{Axis, TradingBinding, CHANNEL_EXTRACTOR_UPKEEP, CHANNEL_YIELD};
use cfe::world::{replay, ActionRefused, World};

/// A world whose progenitor may mint, holding the iron an extractor
/// costs several times over.
fn t0_world(line: &str) -> World {
    let mut world = World::new();
    world.found(line).expect("founds");
    world
        .seat_station(name(line), Station::from_wire(1).expect("a band office"))
        .expect("seats the progenitor");
    world
}

/// The DEEPWORKS: an extractor variant that yields more and costs more
/// to run. One benefit, one friction — so it constructs.
fn deepworks() -> TradingBinding {
    TradingBinding::new(vec![
        (CHANNEL_YIELD, ratio(2, 1)),
        (CHANNEL_EXTRACTOR_UPKEEP, ratio(2, 1)),
    ])
    .expect("a building that trades")
}

#[test]
fn a_minted_building_is_raised_priced_worked_and_salvaged_on_its_base() {
    let mut world = t0_world("tunborn");
    let code = world
        .mint_vocabulary_based_as(
            cfe::world::SOVEREIGN,
            "tunborn",
            Axis::Zone,
            "⌂",
            name("deepworks"),
            deepworks(),
            Some(ZoneKind::Extractor.wire()),
        )
        .expect("the building enters the vocabulary");
    assert_eq!(code, ZONE_KINDS + 1, "the first minted kind takes the next free wire code");

    // RAISED — and PRICED on its base. The binding modulates what a
    // building does, never what it costs to raise.
    let iron_before = world.orb().expect("stands").banked_amount(Resource::Iron);
    world.zone(0, ZoneKind::Minted(code)).expect("the deepworks is raised");
    let spent = iron_before - world.orb().expect("stands").banked_amount(Resource::Iron);
    let base_cost = ZoneKind::Extractor
        .cost()
        .into_iter()
        .find(|(resource, _)| *resource == Resource::Iron)
        .map(|(_, cost)| cost)
        .expect("the extractor costs iron");
    assert_eq!(spent, base_cost, "raised at its base's price");

    // The zone carries the base it was raised on — committed state, so
    // the gate can price the salvage without reading the fold.
    let standing = world.orb().expect("stands").zones[0].clone();
    assert_eq!(standing.kind, ZoneKind::Minted(code));
    assert_eq!(standing.base, Some(ZoneKind::Extractor));
    assert_eq!(standing.priced_as(), ZoneKind::Extractor);

    // WORKED — the yield is the base's, doubled by the binding.
    for _ in 0..4 {
        world.tick();
    }
    let minted_iron = world
        .accrued()
        .into_iter()
        .find(|(resource, _)| *resource == Resource::Iron)
        .map(|(_, amount)| amount)
        .expect("the deepworks works");

    // The same world, same span, with a PLAIN extractor instead.
    let mut plain = t0_world("tunborn");
    plain.zone(0, ZoneKind::Extractor).expect("raised");
    for _ in 0..4 {
        plain.tick();
    }
    let plain_iron = plain
        .accrued()
        .into_iter()
        .find(|(resource, _)| *resource == Resource::Iron)
        .map(|(_, amount)| amount)
        .expect("the extractor works");
    assert!(
        minted_iron > plain_iron,
        "the deepworks out-yields the kind it varies: {minted_iron} vs {plain_iron}"
    );

    // SALVAGED — half the base's cost, exactly as the build spent it.
    let before_raze = world.orb().expect("stands").banked_amount(Resource::Iron);
    world.raze(0).expect("razed");
    let refund = world.orb().expect("stands").banked_amount(Resource::Iron) - before_raze;
    assert_eq!(refund, base_cost / ratio(2, 1), "salvaged on the base it was raised on");

    // SEAL B — the whole arc replays.
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn a_word_without_a_workshop_mints_but_never_builds() {
    // THE LADDER'S THIRD RUNG, ENFORCED. The binding names only the
    // GENERAL channels, which price every zone alike, so no base is
    // inferred and none was declared. The mint stands — the vocabulary
    // is richer — and the build refuses.
    let mut world = t0_world("tunborn");
    let code = world
        .mint_vocabulary(
            "tunborn",
            Axis::Zone,
            "◇",
            name("reverie"),
            TradingBinding::new(vec![
                (CHANNEL_YIELD, ratio(2, 1)),
                (cfe::vocabulary::CHANNEL_UPKEEP, ratio(2, 1)),
            ])
            .expect("it trades"),
        )
        .expect("the word enters the vocabulary");
    assert_eq!(
        world.zone(0, ZoneKind::Minted(code)),
        Err(ActionRefused::ZoneKindUnbuildable { code }),
        "a building with no base to modulate cannot be raised"
    );
    // And a code the fold never minted is its own refusal — a FACT of
    // the chain, which is why it lives at the verb.
    assert_eq!(
        world.zone(1, ZoneKind::Minted(code + 9)),
        Err(ActionRefused::ZoneKindUnknown { code: code + 9 })
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn an_inferred_base_builds_too() {
    // RUNG 2 end to end: no base declared, and the extractor channel
    // says which family this building belongs to.
    let mut world = t0_world("tunborn");
    let code = world
        .mint_vocabulary("tunborn", Axis::Zone, "⌁", name("pitworks"), deepworks())
        .expect("mints");
    world.zone(0, ZoneKind::Minted(code)).expect("the inferred base carries it");
    assert_eq!(
        world.orb().expect("stands").zones[0].base,
        Some(ZoneKind::Extractor),
        "inferred from the channel it binds, and fixed at build time"
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn a_minted_building_survives_the_state_wire() {
    // The base rides the zone's own record as a trailing byte, so a
    // saved world reopens knowing what its buildings were raised on.
    let mut world = t0_world("tunborn");
    let code = world
        .mint_vocabulary_based_as(
            cfe::world::SOVEREIGN,
            "tunborn",
            Axis::Zone,
            "⌂",
            name("deepworks"),
            deepworks(),
            Some(ZoneKind::Extractor.wire()),
        )
        .expect("mints");
    world.zone(0, ZoneKind::Minted(code)).expect("raised");
    let orb = world.orb().expect("stands").clone();
    let bytes = orb.wire_bytes();
    let back = cfe::dop::ColonyOrb::from_wire(&bytes).expect("the orb decodes");
    assert_eq!(back.zones, orb.zones, "the minted kind and its base both survive");
    assert_eq!(back.zones[0].base, Some(ZoneKind::Extractor));
}
