//! CIV-REACH-1 — bronze peoples do not open exotic depths.
//!
//! Sky (`lines_of`) caps what the system carries. Civilization reach
//! caps what the people may work. Prospect / assay / refine refuse
//! when asked depth exceeds either.

use cfe::almanac::{civ_reach, lines_of, periodic_of, refines, strata_lines, LINES};
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

/// THE DEPOSIT LAW — a vein carries only lines its stratum's pressure
/// could have deposited under its star, and stratum 1 always carries a
/// workable line-1 vein (the bronze floor). Before this law the LINE
/// axis drew uniform 1..=7 regardless of depth, so ~61% of bronze
/// worlds could bank nothing at all and stratum 2 under G charted
/// line-7 veins the system does not carry (locality broken at the
/// draw).
#[test]
fn veins_carry_only_lines_their_stratum_deposits() {
    // The table itself: G forges 5, so the releasable ladder is exact.
    assert_eq!(strata_lines(1, StarClass::G), 1);
    assert_eq!(strata_lines(2, StarClass::G), 2);
    assert_eq!(strata_lines(3, StarClass::G), 4);
    assert_eq!(strata_lines(4, StarClass::G), 5);
    assert_eq!(strata_lines(5, StarClass::G), 5, "capped by the forge");
    // A shallow forge compresses the whole ladder into few strata:
    // under a brown dwarf (forge 2) stratum 1 already presses 1/2,
    // releasing both lines the system carries.
    assert_eq!(strata_lines(1, StarClass::BrownDwarf), 2);
    assert_eq!(strata_lines(2, StarClass::BrownDwarf), 2);
    // Drawn veins obey it — many worlds, every reachable stratum.
    for (name, class) in [
        ("deposit-a", StarClass::G),
        ("deposit-b", StarClass::G),
        ("deposit-c", StarClass::M),
        ("deposit-d", StarClass::O),
    ] {
        let mut world = sunlit(name, class);
        for c in 1u8..=5 {
            world.grant_tech_as(SOVEREIGN, FIRST_ORB, c).expect("tech");
        }
        for stratum in 1..=world.civ_reach() {
            let struck = world.prospect(FIRST_ORB, stratum).expect("opens");
            let line = periodic_of(&struck.element).line;
            assert!(
                line <= strata_lines(stratum, class),
                "{name} stratum {stratum} drew line {line} past the deposit \
                 ceiling {}",
                strata_lines(stratum, class)
            );
            assert!(line <= lines_of(class), "locality holds at the draw");
        }
    }
}

/// The bronze floor: every sunlit world's stratum-1 vein is line 1 and
/// refines at its own depth — a bronze people always has SOMETHING to
/// mine, and the growth path stays real for the deeper strata.
#[test]
fn bronze_always_banks_its_surface() {
    for name in ["floor-a", "floor-b", "floor-c"] {
        let mut world = sunlit(name, StarClass::G);
        let struck = world.prospect(FIRST_ORB, 1).expect("surface opens");
        assert_eq!(periodic_of(&struck.element).line, 1);
        assert!(refines(&struck.element, 1, StarClass::G));
        for _ in 0..4 {
            world.tick();
        }
        world.assay(FIRST_ORB, struck.element).expect("surface banks");
    }
}

/// Refinement PURIFIES — the refined identity inherits the raw's LINE
/// axis, so a people that can work a vein can always work what it
/// refines into. Before this law the refined line re-drew uniform
/// 1..=7, refusing ~5/7 of bronze refines at the CivReachShort gate.
#[test]
fn refinement_never_transmutes_depth() {
    for seed in 0u8..16 {
        let raw = [seed; 32];
        let refined = cfe::dop::refined_identity(&raw);
        assert_ne!(raw, refined, "refined is a new identity");
        assert_eq!(
            periodic_of(&raw).line,
            periodic_of(&refined).line,
            "depth survives the crucible"
        );
    }
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
