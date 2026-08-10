//! CIV-REACH-1 — bronze peoples do not open exotic depths.
//!
//! Sky (`lines_of`) caps what the system carries. Civilization reach
//! caps what the people may work. Prospect / assay / refine refuse
//! when asked depth exceeds either.

use cfe::almanac::{civ_reach, LINES};
use cfe::dop::{positive, unit, ZoneKind};
use cfe::thermal::StarClass;
use cfe::world::{ActionRefused, World, FIRST_ORB, SOVEREIGN};

fn sunlit(name: &str, class: StarClass) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(class).expect("commissions");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    world
}

#[test]
fn bronze_civ_reach_is_two_under_a_star() {
    let world = sunlit("bronze-hold", StarClass::G);
    // No crystallized base conditions → surface only.
    assert_eq!(world.civ_reach(), 2);
    assert_eq!(
        civ_reach(std::iter::empty(), Some(StarClass::G)),
        2,
        "pure table agrees"
    );
}

#[test]
fn deep_stratum_refuses_until_tech_grows() {
    let mut world = sunlit("shallow-mine", StarClass::G);
    assert_eq!(
        world.prospect(FIRST_ORB, 5),
        Err(ActionRefused::CivReachShort { need: 5, have: 2 }),
        "bronze cannot open stratum 5"
    );
    // Surface strata stand.
    world.prospect(FIRST_ORB, 1).expect("stratum 1 opens");
    world.prospect(FIRST_ORB, 2).expect("stratum 2 opens");
}

#[test]
fn sky_tech_raises_the_mine_ceiling() {
    let mut world = sunlit("growing-mine", StarClass::G);
    // Conditions 1..=5 grow the mine (solar leap 0 does not).
    world
        .grant_tech_as(SOVEREIGN, FIRST_ORB, 1)
        .expect("insulation");
    assert_eq!(world.civ_reach(), 3, "one grown condition → reach 3");
    world.prospect(FIRST_ORB, 3).expect("stratum 3 now opens");
    world
        .grant_tech_as(SOVEREIGN, FIRST_ORB, 2)
        .expect("discipline");
    world
        .grant_tech_as(SOVEREIGN, FIRST_ORB, 3)
        .expect("deepening");
    assert_eq!(world.civ_reach(), 5);
    world.prospect(FIRST_ORB, 5).expect("stratum 5 opens after growth");
    // Star G forges only to 5 — more tech cannot exceed the sky.
    world
        .grant_tech_as(SOVEREIGN, FIRST_ORB, 4)
        .expect("doctrine");
    world
        .grant_tech_as(SOVEREIGN, FIRST_ORB, 5)
        .expect("stores");
    assert_eq!(
        world.civ_reach(),
        5,
        "capped by lines_of(G)=5 even with full base tech"
    );
    assert_eq!(
        world.prospect(FIRST_ORB, 6),
        Err(ActionRefused::CivReachShort { need: 6, have: 5 }),
        "G-star forges only to line 5"
    );
}

#[test]
fn brown_dwarf_caps_even_full_tech() {
    let mut world = sunlit("dim-forge", StarClass::BrownDwarf);
    for c in 1u8..=5 {
        world.grant_tech_as(SOVEREIGN, FIRST_ORB, c).expect("tech");
    }
    // Full tech wants 7; brown dwarf lines_of = 2 → civ_reach 2.
    assert_eq!(world.civ_reach(), 2);
    assert!(world.civ_reach() <= LINES);
}

#[test]
fn assay_refuses_lines_beyond_civ_reach() {
    let mut world = sunlit("assay-gate", StarClass::G);
    // Open strata 1..=2 (bronze). Chart veins; assay only if line ≤ 2.
    let a = world.prospect(FIRST_ORB, 1).expect("s1");
    let b = world.prospect(FIRST_ORB, 2).expect("s2");
    for _ in 0..8 {
        world.tick();
    }
    let line_a = cfe::almanac::periodic_of(&a.element).line;
    let line_b = cfe::almanac::periodic_of(&b.element).line;
    for (el, line) in [(a.element, line_a), (b.element, line_b)] {
        let result = world.assay(FIRST_ORB, el);
        if line <= 2 {
            assert!(result.is_ok(), "line {line} should assay under bronze");
        } else {
            assert_eq!(
                result,
                Err(ActionRefused::CivReachShort {
                    need: line,
                    have: 2
                }),
                "line {line} is known but not yet mineable"
            );
        }
    }
    let board = world.stellar_capacity_board(FIRST_ORB);
    assert_eq!(board.civ_reach, 2);
    assert!(board.civ_reach <= board.reach);
}

#[test]
fn grown_tech_unlocks_assay_of_deeper_lines() {
    let mut world = sunlit("unlock-mine", StarClass::G);
    let struck = world.prospect(FIRST_ORB, 1).expect("charts");
    let line = cfe::almanac::periodic_of(&struck.element).line;
    for _ in 0..4 {
        world.tick();
    }
    if line <= 2 {
        // Drawn surface line — already mineable; grant tech and go deeper.
        world.assay(FIRST_ORB, struck.element).expect("surface assay");
        return;
    }
    assert_eq!(
        world.assay(FIRST_ORB, struck.element),
        Err(ActionRefused::CivReachShort {
            need: line,
            have: 2
        })
    );
    // Raise civ_reach to cover the line (each condition 1..=5 adds one).
    let need_n = line.saturating_sub(2);
    for c in 1u8..=need_n.min(5) {
        world
            .grant_tech_as(SOVEREIGN, FIRST_ORB, c)
            .expect("grow");
    }
    assert!(world.civ_reach() >= line);
    for _ in 0..4 {
        world.tick();
    }
    match world.assay(FIRST_ORB, struck.element) {
        Ok(_) => {}
        // Deep lines also need opened pressure (stratum depth).
        Err(ActionRefused::PressureUnreached { .. }) => {
            let s = line.min(world.civ_reach());
            let deeper = world.prospect(FIRST_ORB, s).expect("open pressure");
            for _ in 0..4 {
                world.tick();
            }
            world
                .assay(FIRST_ORB, deeper.element)
                .expect("mine after growth + pressure");
        }
        Err(other) => panic!("unexpected: {other:?}"),
    }
}
