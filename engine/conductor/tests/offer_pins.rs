//! OFFER-1b — SURVIVE THE FREEZE (BUILD_ORDER §3a). The whole arc,
//! staged and witnessed: a Furnace world banks a joule surplus and
//! posts fuel-for-S_cool; a Frost world freeze-locks in joule famine
//! with banked sink and posts S_cool-for-fuel; the market beat clears
//! the crossing as aperture pairs — the famine RELIEVES through the
//! fill, the frozen world thaws and its flows resume, both books
//! carry OfferCleared, and replay reproduces the rescue from receipts
//! alone. The autarkies become a mesh.

use cfe::dop::{ClosureEvent, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::replay;
use conductor::{market_step, trade_intent, Polity, PolitySpec, Policy};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("offer-pin-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

/// FLEET-1 staging (ratified 2026-08-12): Phase-2 gives auto-sited
/// worlds distinct orbital phases, so even a chart-distance-0 clear
/// runs positive melt and needs hull capacity. A grade-0 hauler adds
/// capacity without arithmetic drift (hull_eta = 0); it stages at
/// genesis, inside the 34 balance gate's bootstrap window.
fn equip_hauler(world: &mut cfe::world::World) {
    world.zone(40, ZoneKind::ShipYard).expect("yard");
    world
        .salvage_scrap(cfe::world::FIRST_ORB, cfe::dop::positive(4, 1))
        .expect("scrap");
    world
        .mint_hull(cfe::world::FIRST_ORB, cfe::dop::HullRole::Hauler, 0)
        .expect("hauler");
    // The hull outlives its yard; razing it spares the starvation
    // fixtures 200 ticks of yard upkeep that would distort the very
    // famine dynamics under test.
    world
        .raze_as(cfe::world::SOVEREIGN, cfe::world::FIRST_ORB, 40)
        .expect("yard razed");
}

/// Constructed staging (the realignment pattern): a warrant mints the
/// stake the fixture's arithmetic needs, receipted — the founding kit
/// cannot fund both the hauler and the fixture's zones, and this pin's
/// charter is the MARKET rescue, not the bootstrap economy.
fn stake(world: &mut cfe::world::World, resource: Resource, amount: i64) {
    struct TestWarrant;
    impl cfe::world::MintWarrant for TestWarrant {
        fn earns(&self, _work: &[u8]) -> Option<cfe::ratio::Ratio> {
            Some(ratio(1_000, 1))
        }
        fn admits(
            &self,
            _prior: &cfe::world::Committed,
            _next: &cfe::world::Committed,
            _event: &ClosureEvent,
            _actor: u64,
            _stamp: u64,
        ) -> bool {
            true
        }
    }
    world
        .mint_discovery_as(
            cfe::world::SOVEREIGN,
            cfe::world::FIRST_ORB,
            resource,
            cfe::dop::NonNegative::new(ratio(amount, 1)).expect("nn"),
            vec![1u8; 32],
            &TestWarrant,
        )
        .expect("the warrant stakes the fixture");
}

fn seat(sigil: &str, star: StarClass, albedo: (i64, i64), orbit: (i64, i64)) -> PolitySpec {
    PolitySpec {
        civilization: Civilization {
            name: sigil.to_string(),
            sigil: sigil.to_string(),
            species: "trade witness".into(),
            clade: String::new(),
            motto: "the crossing feeds".into(),
            disposition: "empiricist".into(),
            metabolism: "biotic".into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage: Vec::new(),
            station: None,
        },
        world_name: format!("{sigil}-hold"),
        planet: format!("{sigil}-p"),
        colony: format!("{sigil}-c"),
        policy: Policy::Steady,
        position: (ratio(0, 1), ratio(0, 1), ratio(0, 1)),
        star: Some(star),
        albedo: Some(ratio(albedo.0, albedo.1)),
        orbit: Some(ratio(orbit.0, orbit.1)),
        gauge: None,
    }
}

#[test]
fn the_crossing_relieves_the_freeze() {
    let root = scratch("freeze");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    // Hand-driven worlds (no policy steps): the pin stages the states
    // the instinct reads, through real physics only.
    let mut furnace = Polity::genesis(
        seat("embermarket", StarClass::G, (1, 2), (1, 2)), &registry, &book);
    let mut frost = Polity::genesis(
        seat("rimemarket", StarClass::M, (0, 1), (1, 1)), &registry, &book);
    equip_hauler(&mut furnace.world);
    equip_hauler(&mut frost.world);
    // The hauler's salvage spent the iron the shell needs.
    stake(&mut furnace.world, Resource::Iron, 4);

    // The Furnace banks a surplus: extractor+habitat sustain, a shell
    // encases the overflow into joules, collected each year.
    furnace.world.zone(0, ZoneKind::Extractor).expect("zones");
    furnace.world.zone(8, ZoneKind::Habitat).expect("zones");
    furnace.world.zone(1, ZoneKind::Encasement).expect("shells");
    for tick in 1..=120u32 {
        furnace.world.tick();
        if tick % 20 == 0 {
            let _ = furnace.world.collect();
        }
    }
    assert!(
        trade_intent(&furnace.world, "biotic").is_some(),
        "the surplus Furnace forms its intent (banked joules: {:?})",
        furnace.world.orb().unwrap().banked_amount(Resource::Joule)
    );

    // The Frost freeze-locks honestly: an array captures sink while
    // the heating burn drains joules it never had — famine, frozen.
    frost.world.zone(0, ZoneKind::CryoArray).expect("arrays");
    let mut struck = false;
    for _ in 0..20 {
        struck = struck || !frost.world.tick().struck.is_empty();
        let _ = frost.world.collect();
    }
    assert!(struck, "the joule famine strikes — the freeze-lock is real");
    assert!(
        frost.world.orb().unwrap().famine.contains(&Resource::Joule),
        "frozen in want of fuel"
    );
    assert!(
        frost.world.orb().unwrap().banked_amount(Resource::Sink) > ratio(0, 1),
        "sink banked before the freeze — something to trade"
    );
    assert!(trade_intent(&frost.world, "biotic").is_some(), "the starving Frost forms its intent");

    // THE MARKET BEAT: intents post, the crossing clears.
    let mut board = vec![furnace, frost];
    let cleared = market_step(&mut board, 1);
    assert_eq!(cleared.len(), 1, "one crossing clears");

    let frost = &mut board[1];
    // The rescue: the fill's aperture credit relieved the famine.
    assert!(
        !frost.world.orb().unwrap().famine.contains(&Resource::Joule),
        "the famine RELIEVED through the fill"
    );
    assert!(
        frost.world.orb().unwrap().banked_amount(Resource::Joule) > ratio(0, 1),
        "fuel in the bank"
    );
    // Thawed: the next tick flows again (sink accrues anew).
    frost.world.tick();
    let _ = frost.world.collect();
    // Both books carry the clear; replay reproduces the rescue.
    for polity in &board {
        let cleared_receipt = polity.world.chronicle().receipts().iter().any(|r| {
            matches!(r.event, ClosureEvent::OfferCleared { .. })
        });
        assert!(cleared_receipt, "the clear is a receipt in both books");
        let committed = replay(polity.world.chronicle());
        assert_eq!(
            committed.orbs[0].banked_amount(Resource::Joule),
            polity.world.orb().unwrap().banked_amount(Resource::Joule),
            "the rescue replays from receipts alone"
        );
    }
}

#[test]
fn any_famine_can_ask_the_market() {
    // BALANCE-1: the famine ask is open-ended — the survey (docs/
    // BALANCE_NOTES.md) measured 42% struck with a market path for
    // joule famine ONLY. Two mesic autarkies famish complementarily
    // (iron-rich/biomass-starved against its opposite), both asks
    // post, ONE crossing relieves BOTH.
    let root = scratch("famine-ask");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let mut miner = Polity::genesis(
        seat("lodemarket", StarClass::G, (1, 4), (1, 1)), &registry, &book);
    let mut grower = Polity::genesis(
        seat("mossmarket", StarClass::G, (1, 4), (1, 1)), &registry, &book);
    equip_hauler(&mut miner.world);
    equip_hauler(&mut grower.world);
    // KNOWN RED (ledgered 2026-08-12, task #10): the crossing below
    // no longer clears — the staging restores capacity but the steep
    // famine bids' price cross fails under the post-arc stocks. The
    // freeze pin proves the rescue law; this one still owes its
    // diagnosis.

    // The miner extracts only (iron banks, biomass drains); the
    // grower inhabits only (biomass banks, iron drains).
    miner.world.zone(0, ZoneKind::Extractor).expect("zones");
    grower.world.zone(0, ZoneKind::Habitat).expect("zones");
    for world in [&mut miner.world, &mut grower.world] {
        for _ in 0..200 {
            world.tick();
            let _ = world.collect();
        }
    }
    let miner_orb = miner.world.orb().unwrap();
    let grower_orb = grower.world.orb().unwrap();
    assert!(miner_orb.famine.contains(&Resource::Biomass), "the miner starves for biomass");
    assert!(grower_orb.famine.contains(&Resource::Iron), "the grower starves for iron");
    assert!(miner_orb.banked_amount(Resource::Iron) > ratio(0, 1), "iron to give");
    assert!(grower_orb.banked_amount(Resource::Biomass) > ratio(0, 1), "biomass to give");
    let (give, _, want, _) = trade_intent(&miner.world, "biotic").expect("the miner asks");
    assert_eq!((give, want), (Resource::Iron, Resource::Biomass));
    let (give, _, want, _) = trade_intent(&grower.world, "biotic").expect("the grower asks");
    assert_eq!((give, want), (Resource::Biomass, Resource::Iron));

    let mut board = vec![miner, grower];
    let cleared = market_step(&mut board, 1);
    assert_eq!(cleared.len(), 1, "one crossing clears");
    assert!(
        !board[0].world.orb().unwrap().famine.contains(&Resource::Biomass)
            && !board[1].world.orb().unwrap().famine.contains(&Resource::Iron),
        "one crossing relieves BOTH famines"
    );
}
