//! THE ALMANAC PINS — periodicity, locality, pressure, composition.
//!
//! Each claim the module makes is a claim about RECURRENCE, and a
//! recurrence that is not pinned is a coincidence waiting to be edited
//! away.

use cfe::almanac::{
    compounds, gem_of, knowledge_of, lines_of, local_to, periodic_of, pressure, refines, LANES,
    LINES,
};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;

/// An element id whose table position is chosen by construction: byte 3
/// fixes the line, byte 4 the lane.
fn at(line: u8, lane: u8) -> [u8; 32] {
    let mut id = [0u8; 32];
    id[3] = line - 1;
    id[4] = lane - 1;
    id
}

#[test]
fn the_table_is_stable_and_total() {
    // Every byte pair lands in the table, and the same id always reads
    // the same cell. Totality first: a position that panicked would be
    // a hole in the vocabulary.
    for b3 in 0..=255u8 {
        for b4 in [0u8, 1, 7, 8, 200, 255] {
            let mut id = [0u8; 32];
            id[3] = b3;
            id[4] = b4;
            let p = periodic_of(&id);
            assert!((1..=LINES).contains(&p.line), "line in range: {}", p.line);
            assert!((1..=LANES).contains(&p.lane), "lane in range: {}", p.lane);
            assert_eq!(p, periodic_of(&id), "the same id reads the same cell");
        }
    }
}

#[test]
fn properties_recur_along_a_lane_which_is_what_makes_it_periodic() {
    // THE LOAD-BEARING PIN. A lane's character must not depend on its
    // line — that recurrence is the difference between a TABLE and a
    // list, and it is what lets a civilization predict the next element
    // from one it has already refined.
    for lane in 1..=LANES {
        let first = periodic_of(&at(1, lane));
        for line in 2..=LINES {
            let deeper = periodic_of(&at(line, lane));
            assert_eq!(
                deeper.binds, first.binds,
                "lane {lane} binds the same at every line: line {line}"
            );
            assert_eq!(deeper.lane, lane);
        }
    }
}

#[test]
fn pressure_rises_with_the_line_and_never_past_one() {
    // Depth is the OTHER axis and must vary where the lane does not.
    let mut last = ratio(0, 1);
    for line in 1..=LINES {
        let p = periodic_of(&at(line, 1));
        assert!(p.refines_at > last, "line {line} demands more than line {}", line - 1);
        assert!(p.refines_at <= ratio(1, 1), "pressure is a fraction of full");
        last = p.refines_at;
    }
    assert_eq!(periodic_of(&at(LINES, 1)).refines_at, ratio(1, 1), "the deepest line takes all of it");
}

#[test]
fn locality_is_the_star_and_a_deep_element_is_absent_under_a_dim_one() {
    // The same element id, two systems. This is what makes a resource a
    // PLACE rather than a number.
    let deep = at(7, 3);
    assert!(local_to(&deep, StarClass::O), "the heavy forge reaches line 7");
    assert!(!local_to(&deep, StarClass::M), "an M dwarf never does");
    assert!(!local_to(&deep, StarClass::BrownDwarf));

    // And a shallow element is everywhere — locality bars the deep, it
    // does not make worlds disjoint.
    let shallow = at(1, 3);
    for class in [StarClass::O, StarClass::G, StarClass::M, StarClass::BrownDwarf] {
        assert!(local_to(&shallow, class), "line 1 stands under every star");
    }
    assert!(lines_of(StarClass::O) > lines_of(StarClass::M));
}

#[test]
fn refinement_needs_the_pressure_the_line_demands() {
    let element = at(4, 2); // needs 4/7
    let class = StarClass::O; // reaches all 7 lines, so pressure = stratum/7
    assert!(!refines(&element, 3, class), "3/7 is short of 4/7");
    assert!(refines(&element, 4, class), "4/7 meets it exactly — no tolerance");
    assert!(refines(&element, 7, class));

    // Locality dominates pressure: no stratum releases what the system
    // does not carry.
    let deep = at(7, 2);
    assert!(!refines(&deep, 7, StarClass::M), "an M dwarf has no line 7 to press");
    assert_eq!(pressure(9, StarClass::M), ratio(1, 1), "pressure saturates at the star's reach");
}

#[test]
fn composition_is_the_lanes_rule_and_the_gem_is_order_free() {
    // binds: lanes 1..4 -> 1..4, lanes 5..8 -> 4..1. Saturation is
    // LANES/2 + 1 = 5, so lane 1 (binds 1) compounds with lane 4
    // (binds 4).
    let a = at(1, 1);
    let b = at(1, 4);
    assert!(compounds(&a, &b), "1 + 4 saturates");
    assert!(!compounds(&a, &a), "1 + 1 does not");

    // THE RECURRENCE, again: the same lane pair compounds at ANY lines.
    for line_a in 1..=LINES {
        for line_b in 1..=LINES {
            assert!(
                compounds(&at(line_a, 1), &at(line_b, 4)),
                "the recipe is the lanes', not the lines': {line_a}/{line_b}"
            );
        }
    }

    // The gem is content-addressed and order-free — a compound is not a
    // direction.
    let gem = gem_of(&a, &b).expect("compounds");
    assert_eq!(gem, gem_of(&b, &a).expect("compounds either way"));
    assert!(gem_of(&a, &a).is_none(), "no gem where nothing compounds");

    // A DIFFERENT pair is a DIFFERENT gem — discovery is by identity,
    // never by anyone's say-so.
    let other = gem_of(&at(2, 1), &at(1, 4)).expect("compounds");
    assert_ne!(gem, other, "a new composition is a new gem");
}

#[test]
fn knowledge_is_the_depth_that_was_won() {
    // A gem of deep elements carries more than a shallow one, and the
    // shallowest pair carries the least that is still something.
    let shallow = knowledge_of(&at(1, 1), &at(1, 4)).expect("compounds");
    let deep = knowledge_of(&at(7, 1), &at(7, 4)).expect("compounds");
    assert_eq!(shallow, 2, "two line-1 elements");
    assert_eq!(deep, 14, "two line-7 elements");
    assert!(deep > shallow, "depth is what was won");
    assert!(
        knowledge_of(&at(1, 1), &at(1, 1)).is_none(),
        "no compound, no knowledge — never a zero"
    );
}
