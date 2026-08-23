//! TECH-1b deferred arc witnesses, paid (10 §3b's own open note):
//! Lattice Deepening and Long Stores measured end-to-end — the
//! condition earned through played history, the modifier read live.
//! Two laws inscribed by measurement: a vault colony runs on FUEL
//! (the ECON-J burn bites even beside the lattice — capture is not
//! free heat), and the deepening boost BINDS only beside a
//! doctrine-boosted radiator (base radiator 2 = base lattice 2, so
//! headroom alone never exceeds one array's throughput) — the
//! crystallized nodes COMPOSE; capture never outruns the physics.

use cfe::dop::{positive, unit, ClosureEvent, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{aperture_transfer, replay, World, FIRST_ORB};

/// The fuel minter (the world_tech_tree burning idiom): a hot pair
/// with an encasement shell seals overflow into banked joules.
fn furnace(ticks: u32) -> World {
    let mut world = World::new();
    world.found("cinder-reach").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world.site_planet(FIRST_ORB, unit(1, 2), positive(1, 2), None, None).expect("sites hot");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world.zone(8, ZoneKind::Habitat).expect("zones");
    world.zone(16, ZoneKind::Encasement).expect("the shell seals the overflow");
    for tick in 1..=ticks {
        world.tick();
        if tick % 30 == 0 {
            let _ = world.collect();
        }
    }
    world
}

#[test]
fn lattice_deepening_crystallizes_and_never_outruns_headroom() {
    // The vault world (the thermal_flow siting): G star, albedo 0,
    // orbit 100 — deep in the cryo vault (no leak, so condition 3's
    // demurrage gate holds free); one array captures headroom
    // 19999/10000 per tick. The ECON-J law bites here too: sub-mesic,
    // every active zone burns 1/4 joule per tick — the lattice runs
    // on FUEL, apertured in from the furnace before the first tick.
    let mut fuel = furnace(150);
    let minted = fuel.orb().expect("stands").banked_amount(Resource::Joule);
    assert!(minted >= ratio(46, 1), "the shell banked the fuel: {minted}");
    let mut vault = World::new();
    vault.found("vault-hold").expect("founds");
    vault.commission_star(StarClass::G).expect("commissions");
    vault.site_planet(FIRST_ORB, unit(0, 1), positive(100, 1), None, None).expect("sites the vault");
    vault.zone(0, ZoneKind::CryoArray).expect("zones");
    aperture_transfer(&mut fuel, FIRST_ORB, &mut vault, FIRST_ORB, Resource::Joule, cfe::dop::positive(45, 1), 78)
        .expect("the fuel crosses");
    // The array's build spent half the founding iron (20 − 10): its
    // 1/4-iron upkeep would strike at tick 41 (measured — the first
    // draft forgot the build cost). The furnace lends STEEL as well
    // as fire: +35 iron covers 180 ticks of upkeep.
    aperture_transfer(&mut fuel, FIRST_ORB, &mut vault, FIRST_ORB, Resource::Iron, cfe::dop::positive(35, 1), 80)
        .expect("the steel crosses");
    // 45 banked joules cover 180 ticks of the 1/4 burn: the array
    // stays awake through the whole arc. Sink banks at the year
    // collect; the boundary weighs banked stores.
    for tick in 1..=120u32 {
        vault.tick();
        if tick % 60 == 0 {
            let _ = vault.collect();
        }
    }
    let deepened = vault
        .chronicle()
        .receipts()
        .iter()
        .any(|receipt| matches!(
            receipt.event(),
            ClosureEvent::TechDiscovered { condition: 3, magnitude, name, .. }
                if *magnitude == ratio(5, 4) && name.ends_with(" deepening")
        ));
    assert!(deepened, "a hundred banked sink in a settled vault crystallizes the deepening");
    // The composition law, measured: with base radiator 2 the
    // headroom stays 19999/10000 < 2 < 5/2 — the deepened lattice
    // captures EXACTLY the headroom still. The boost binds only where
    // a doctrine-boosted radiator opens headroom past one array's
    // base throughput; capture never mints sink past the physics.
    vault.tick();
    let sink_now = vault
        .accrued()
        .into_iter()
        .find(|(resource, _)| *resource == Resource::Sink)
        .map(|(_, amount)| amount)
        .expect("the vault still captures");
    assert_eq!(
        sink_now,
        ratio(19_999, 10_000),
        "min(headroom, 5/2) = headroom here — the deepening is a wider gate, never a mint"
    );
    assert_eq!(replay(vault.chronicle()), vault.committed());
    assert_eq!(replay(fuel.chronicle()), fuel.committed());
}

#[test]
fn long_stores_halves_the_decay_measured_on_the_receipt() {
    let mut fuel = furnace(240);
    let minted = fuel.orb().expect("stands").banked_amount(Resource::Joule);
    assert!(minted >= ratio(66, 1), "the shell banked the fuel: {minted}");

    // The cold keeper: K star at (0, 2) — LEAKING (above the vault,
    // demurrage runs) and sub-mesic (condition 5's cold gate). One
    // habitat: the year bill is 1/4 × 60 = 15; four bills = 60.
    let mut keep = World::new();
    keep.found("still-larder").expect("founds");
    keep.commission_star(StarClass::K).expect("commissions");
    keep.site_planet(FIRST_ORB, unit(0, 1), positive(2, 1), None, None).expect("sites cold, leaking");
    keep.zone(0, ZoneKind::Habitat).expect("zones");
    aperture_transfer(&mut fuel, FIRST_ORB, &mut keep, FIRST_ORB, Resource::Joule, cfe::dop::positive(65, 1), 79)
        .expect("the fuel crosses");
    // Cross a year boundary untouched: banked stays 65 ≥ 60 (decay
    // settles only at a touch), the world is cold and active —
    // condition 5 weighs and crystallizes. The boundary at stamp 60
    // fires DURING the 61st call (the clock advances at tick end).
    for _ in 0..61 {
        keep.tick();
    }
    let stored = keep
        .chronicle()
        .receipts()
        .iter()
        .any(|receipt| matches!(
            receipt.event(),
            ClosureEvent::TechDiscovered { condition: 5, magnitude, name, .. }
                if *magnitude == ratio(1, 2) && name.ends_with(" stores")
        ));
    assert!(stored, "four banked winters crystallize the long stores");
    // The ease, measured on the receipt: the first touch settles the
    // mixed pre/post-node window (not asserted); then EXACTLY 32
    // ticks pass and the next touch settles 1/32 × 1/2 × 32 = 1/2 —
    // half the unabated bill, flat, exact.
    let _ = keep.collect();
    for _ in 0..32 {
        keep.tick();
    }
    let _ = keep.collect();
    let last_decay = keep
        .chronicle()
        .receipts()
        .iter()
        .rev()
        .find_map(|receipt| match receipt.event() {
            ClosureEvent::DemurrageTouched { decays, .. } => decays
                .iter()
                .find(|(resource, _)| *resource == Resource::Joule)
                .map(|(_, amount)| amount.clone()),
            _ => None,
        })
        .expect("the leak settles by receipt, never silently");
    assert_eq!(
        last_decay,
        ratio(1, 2),
        "32 ticks at the eased flat rate: 1/32 × 1/2 × 32 — half the unabated bill, exact"
    );
    assert_eq!(replay(keep.chronicle()), keep.committed());
    assert_eq!(replay(fuel.chronicle()), fuel.committed());
}
