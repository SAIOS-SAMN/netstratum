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
use conductor::{market_step, trade_intent, Egregore, EgregoreSpec, Policy};
use mirror::registry::{Civilization, Registry};
use mirror::Mirror;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("offer-pin-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn seat(slug: &str, star: StarClass, albedo: (i64, i64), orbit: (i64, i64)) -> EgregoreSpec {
    EgregoreSpec {
        civilization: Civilization {
            name: slug.to_string(),
            slug: slug.to_string(),
            species: "trade witness".into(),
            motto: "the crossing feeds".into(),
            disposition: "empiricist".into(),
            metabolism: "biotic".into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage: Vec::new(),
            station: None,
        },
        world_name: format!("{slug}-hold"),
        planet: format!("{slug}-p"),
        colony: format!("{slug}-c"),
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
    let mirror = Mirror::at(root.join("worlds"));
    // Hand-driven worlds (no policy steps): the pin stages the states
    // the instinct reads, through real physics only.
    let mut furnace = Egregore::genesis(
        seat("embermarket", StarClass::G, (1, 2), (1, 2)), &registry, &mirror);
    let mut frost = Egregore::genesis(
        seat("rimemarket", StarClass::M, (0, 1), (1, 1)), &registry, &mirror);

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
        trade_intent(&furnace.world).is_some(),
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
    assert!(trade_intent(&frost.world).is_some(), "the starving Frost forms its intent");

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
    for egregore in &board {
        let cleared_receipt = egregore.world.chronicle().receipts().iter().any(|r| {
            matches!(r.event, ClosureEvent::OfferCleared { .. })
        });
        assert!(cleared_receipt, "the clear is a receipt in both books");
        let committed = replay(egregore.world.chronicle());
        assert_eq!(
            committed.orbs[0].banked_amount(Resource::Joule),
            egregore.world.orb().unwrap().banked_amount(Resource::Joule),
            "the rescue replays from receipts alone"
        );
    }
}
