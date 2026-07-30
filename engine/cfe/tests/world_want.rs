//! WANT-1 — THE FAMINES OF THE MIND (12_UNION §5b, tags 62–63). A
//! cognitive want is derived from the chronicle and typed by the
//! SEALED identity (G1) — no seal, no entity type, no want. The
//! resonance want starves a harmonic mind of links (240 ticks); the
//! novelty want starves an empiricist mind of signal (480) and HALTS
//! its crystallizer while struck — relief arrives by licensed leap,
//! the co-op pressure by design. Star-independent: a mind needs no
//! sky to starve.

use cfe::dop::{ClosureEvent, Resource};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{aperture_transfer, replay, World, FIRST_ORB};

fn sunlit(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world.site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1)).expect("sites");
    world
}

/// The resonance want: a harmonic line links once, then silence — the
/// strike lands at a year boundary with the drought measured; a new
/// link feeds the mind and the next boundary relieves. Replays whole.
#[test]
fn a_harmonic_mind_starves_without_links() {
    let mut world = sunlit("chorline");
    let mut partner = sunlit("farline");
    world
        .seal_genesis("abyssal-vocalis", 3, 2, 3)
        .expect("seals harmonic");
    // The link, then the silence.
    aperture_transfer(&mut world, FIRST_ORB, &mut partner, FIRST_ORB,
        Resource::Iron, ratio(1, 1), 1).expect("links");
    for _ in 0..300 {
        world.tick();
    }
    assert!(
        world.wants().iter().any(|(_, kind)| *kind == 2),
        "240 silent ticks starve the harmonic mind"
    );
    // A pre-identity partner never starves (no seal, no want).
    for _ in 0..300 {
        partner.tick();
    }
    assert!(partner.wants().is_empty(), "no seal, no entity type, no want");

    // Feed it: a fresh link, then the boundary relieves.
    aperture_transfer(&mut world, FIRST_ORB, &mut partner, FIRST_ORB,
        Resource::Iron, ratio(1, 1), 2).expect("feeds");
    for _ in 0..60 {
        world.tick();
    }
    assert!(world.wants().is_empty(), "the mind is fed; the want relieved");
    let relieved = world.chronicle().receipts().iter().any(|r| {
        matches!(r.event, ClosureEvent::WantRelieved { kind: 2, .. })
    });
    assert!(relieved, "relief is a receipt");
    assert_eq!(replay(world.chronicle()).wants, world.wants().to_vec());
}

/// The novelty want: an empiricist line with one discovery and a long
/// drought strikes — and its crystallizer RESTS while struck: a
/// condition that holds mints nothing until a licensed leap feeds the
/// mind, and then the tree resumes.
#[test]
fn a_starving_mind_mints_no_nodes() {
    // The empiricist, mesic (nothing crystallizes naturally there);
    // its solar leap at t0 is its one discovery.
    let mut world = sunlit("askline");
    world.seal_genesis("askline", 2, 2, 1).expect("seals empiricist");
    for _ in 0..541 {
        world.tick();
    }
    assert!(
        world.wants().iter().any(|(_, kind)| *kind == 3),
        "480 signal-less ticks starve the empiricist"
    );

    // Turn the sky hostile: the doctrine's condition will hold — and
    // the struck mind must NOT crystallize it.
    world.age_star(StarClass::RedGiant).expect("the star reddens");
    world.zone(0, cfe::dop::ZoneKind::Extractor).expect("zones");
    world.zone(8, cfe::dop::ZoneKind::Habitat).expect("zones");
    for tick in 1..=301u32 {
        world.tick();
        if tick % 60 == 0 {
            let _ = world.collect();
        }
    }
    let crystallized = world.chronicle().receipts().iter().any(|r| {
        matches!(&r.event, ClosureEvent::TechDiscovered { condition: 1.., .. })
    });
    assert!(!crystallized, "a starving mind mints no nodes");

    // The leap that feeds it: a licensed arrival (the co-op door).
    let mut grantor = sunlit("grantline");
    let b = grantor.chart("midhold", FIRST_ORB).expect("charts");
    let c = grantor.chart("farhold", b).expect("charts");
    grantor.transfer(c, FIRST_ORB, Resource::Iron, ratio(6, 1)).expect("spreads");
    grantor.transfer(c, b, Resource::Iron, ratio(6, 1)).expect("spreads");
    grantor
        .seal_council(1, vec![FIRST_ORB, b, c], Resource::Iron, ratio(2, 1), 600, ratio(1, 1))
        .expect("council");
    aperture_transfer(&mut grantor, FIRST_ORB, &mut world, FIRST_ORB,
        Resource::Iron, ratio(1, 1), 9).expect("crosses");
    cfe::world::license_tech(
        &mut grantor, FIRST_ORB, &mut world, FIRST_ORB,
        cfe::world::Leap { condition: 5, name: "granted stores".into(), deal: 20, council: 1 },
    )
    .expect("the leap lands");
    // The next boundary relieves; the one after can crystallize.
    for tick in 1..=121u32 {
        world.tick();
        if tick % 60 == 0 {
            let _ = world.collect();
        }
    }
    assert!(
        !world.wants().iter().any(|(_, kind)| *kind == 3),
        "the licensed leap fed the mind"
    );
    let crystallized_now = world.chronicle().receipts().iter().any(|r| {
        matches!(&r.event, ClosureEvent::TechDiscovered { condition: 4, .. })
    });
    assert!(crystallized_now, "fed, the tree resumes — the doctrine crystallizes");
}
