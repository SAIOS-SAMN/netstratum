//! THE ALMANAC PINS — five axes, and each must DECIDE something no
//! other one can. An axis that changes no outcome is decoration.

use cfe::almanac::{
    compounds, gem_of, knowledge_of, lines_of, local_to, periodic_of, pressure, refines,
    workable_in, CELLS, LANES, LINES, PHASES, SHELLS, SPINS,
};
use cfe::ratio::ratio;
use cfe::thermal::{StarClass, SubstrateBand};

/// An element whose cell is chosen by construction, one byte per axis.
fn at(line: u8, lane: u8, shell: u8, phase: u8, spin: u8) -> [u8; 32] {
    let mut id = [0u8; 32];
    id[3] = line - 1;
    id[4] = lane - 1;
    id[5] = shell - 1;
    id[6] = phase - 1;
    id[7] = spin;
    id
}

#[test]
fn the_table_is_stable_and_total_across_every_axis() {
    for b in 0..=255u8 {
        let mut id = [0u8; 32];
        for slot in 3..=7 {
            id[slot] = b;
        }
        let p = periodic_of(&id);
        assert!((1..=LINES).contains(&p.line));
        assert!((1..=LANES).contains(&p.lane));
        assert!((1..=SHELLS).contains(&p.shell));
        assert!((1..=PHASES).contains(&p.phase));
        assert!(p.spin < SPINS);
        assert_eq!(p.arity, 1 + p.shell, "arity follows the shell");
        assert_eq!(p, periodic_of(&id), "the same id reads the same cell");
    }
    assert_eq!(CELLS, 7 * 8 * 4 * 4 * 2, "five axes, multiplied");
}

#[test]
fn properties_recur_along_a_lane_which_is_what_makes_it_periodic() {
    // THE LOAD-BEARING PIN, and it must hold across the NEW axes too:
    // a lane's character depends on the lane alone.
    for lane in 1..=LANES {
        let first = periodic_of(&at(1, lane, 1, 1, 0));
        for line in 1..=LINES {
            for shell in 1..=SHELLS {
                for phase in 1..=PHASES {
                    let other = periodic_of(&at(line, lane, shell, phase, 0));
                    assert_eq!(other.binds, first.binds, "lane {lane} binds alike everywhere");
                }
            }
        }
    }
}

#[test]
fn every_axis_decides_something_no_other_can() {
    let base = at(3, 2, 2, 2, 0);
    // LINE moves the pressure and nothing else.
    assert_ne!(periodic_of(&at(5, 2, 2, 2, 0)).refines_at, periodic_of(&base).refines_at);
    // LANE moves the binds.
    assert_ne!(periodic_of(&at(3, 3, 2, 2, 0)).binds, periodic_of(&base).binds);
    // SHELL moves the arity.
    assert_ne!(periodic_of(&at(3, 2, 3, 2, 0)).arity, periodic_of(&base).arity);
    // PHASE moves where it is workable.
    assert!(workable_in(&at(3, 2, 2, 1, 0), SubstrateBand::Cryogenic));
    assert!(!workable_in(&at(3, 2, 2, 1, 0), SubstrateBand::Mesic));
    // SPIN moves the GEM and nothing before it — same composition,
    // opposite handedness, different identity.
    let left = [at(1, 1, 1, 1, 0), at(1, 4, 1, 1, 0)];
    let right = [at(1, 1, 1, 1, 0), at(1, 4, 1, 1, 1)];
    assert!(compounds(&left) && compounds(&right), "spin does not gate the bond");
    assert_ne!(
        gem_of(&left).expect("compounds"),
        gem_of(&right).expect("compounds"),
        "opposite handedness is a DIFFERENT gem"
    );
}

#[test]
fn pressure_rises_with_the_line_and_never_past_one() {
    let mut last = ratio(0, 1);
    for line in 1..=LINES {
        let p = periodic_of(&at(line, 1, 1, 1, 0));
        assert!(p.refines_at > last);
        assert!(p.refines_at <= ratio(1, 1));
        last = p.refines_at;
    }
    assert_eq!(periodic_of(&at(LINES, 1, 1, 1, 0)).refines_at, ratio(1, 1));
}

#[test]
fn locality_is_two_independent_readings_the_star_and_the_orbit() {
    // The star says how DEEP.
    let deep = at(7, 3, 1, 4, 0);
    assert!(local_to(&deep, StarClass::O));
    assert!(!local_to(&deep, StarClass::M));
    assert!(lines_of(StarClass::O) > lines_of(StarClass::M));

    // The orbit says how HOT, and it is INDEPENDENT: a shallow element
    // under a heavy star can still be unworkable in the wrong band.
    let shallow_cold = at(1, 3, 1, 1, 0);
    assert!(local_to(&shallow_cold, StarClass::O), "present");
    assert!(!workable_in(&shallow_cold, SubstrateBand::HyperThermal), "and unworkable");

    // The refractory phase works anywhere — locality bars, it does not
    // strand.
    let refractory = at(1, 3, 1, PHASES, 0);
    for band in [SubstrateBand::Cryogenic, SubstrateBand::Mesic, SubstrateBand::HyperThermal] {
        assert!(workable_in(&refractory, band));
    }
}

#[test]
fn refinement_needs_the_pressure_the_line_demands() {
    let element = at(4, 2, 1, 4, 0);
    assert!(!refines(&element, 3, StarClass::O), "3/7 short of 4/7");
    assert!(refines(&element, 4, StarClass::O), "met exactly — no tolerance");
    assert!(!refines(&at(7, 2, 1, 4, 0), 7, StarClass::M), "no line 7 to press");
    assert_eq!(pressure(9, StarClass::M), ratio(1, 1), "saturates at the star's reach");
}

#[test]
fn a_compound_can_be_wider_than_a_pair_which_is_the_multi_party_case() {
    // Shell 2 takes THREE bodies. Saturation is arity * (LANES/2+1) / 2
    // = 3 * 5 / 2, so the binds must sum to 15/2 — impossible in whole
    // numbers, and the shell therefore admits no 3-body compound at
    // this LANES. Recorded rather than tuned away: the arity axis is
    // real, and which arities can saturate is a property of LANES.
    let ternary = [at(1, 1, 2, 1, 0), at(1, 4, 2, 1, 0), at(1, 2, 2, 1, 0)];
    assert_eq!(ternary.len(), 3, "shell 2 wants three");
    assert!(!compounds(&ternary), "15/2 is not reachable in whole binds");

    // Shell 3 takes FOUR: saturation 4 * 5 / 2 = 10, and 1+4+1+4 = 10.
    let quaternary = [
        at(1, 1, 3, 1, 0),
        at(1, 4, 3, 1, 0),
        at(2, 1, 3, 1, 0),
        at(2, 4, 3, 1, 0),
    ];
    assert!(compounds(&quaternary), "four bodies, binds saturate at 10");
    assert!(gem_of(&quaternary).is_some(), "and it names a gem");

    // Arity is enforced: the same four at shell 1 are not a compound.
    let wrong_arity = [
        at(1, 1, 1, 1, 0),
        at(1, 4, 1, 1, 0),
        at(2, 1, 1, 1, 0),
        at(2, 4, 1, 1, 0),
    ];
    assert!(!compounds(&wrong_arity), "shell 1 takes two, not four");
}

#[test]
fn composition_agrees_on_shell_and_phase_and_the_gem_is_order_free() {
    let a = at(1, 1, 1, 2, 0);
    let b = at(1, 4, 1, 2, 0);
    assert!(compounds(&[a, b]), "1 + 4 saturates at arity 2");
    assert!(!compounds(&[a, a]), "1 + 1 does not");

    // Mismatched geometry and mismatched state both refuse.
    assert!(!compounds(&[a, at(1, 4, 2, 2, 0)]), "shells must agree");
    assert!(!compounds(&[a, at(1, 4, 1, 3, 0)]), "phases must agree");

    // The recurrence: the same lane pair compounds at ANY lines.
    for line_a in 1..=LINES {
        for line_b in 1..=LINES {
            assert!(compounds(&[at(line_a, 1, 1, 2, 0), at(line_b, 4, 1, 2, 0)]));
        }
    }

    // Order-free, and a different set is a different gem.
    let gem = gem_of(&[a, b]).expect("compounds");
    assert_eq!(gem, gem_of(&[b, a]).expect("either way"));
    assert_ne!(gem, gem_of(&[at(2, 1, 1, 2, 0), b]).expect("compounds"));
    assert!(gem_of(&[a, a]).is_none(), "no gem where nothing compounds");
}

#[test]
fn knowledge_is_the_depth_and_the_width_that_were_won() {
    let shallow = knowledge_of(&[at(1, 1, 1, 1, 0), at(1, 4, 1, 1, 0)]).expect("compounds");
    let deep = knowledge_of(&[at(7, 1, 1, 1, 0), at(7, 4, 1, 1, 0)]).expect("compounds");
    assert_eq!(shallow, 2);
    assert_eq!(deep, 14);
    assert!(deep > shallow, "depth is won");

    // WIDTH is won too: four bodies at the same depth carry more than
    // two, because arity is earned.
    let wide = knowledge_of(&[
        at(1, 1, 3, 1, 0),
        at(1, 4, 3, 1, 0),
        at(1, 1, 3, 1, 0),
        at(1, 4, 3, 1, 0),
    ])
    .expect("compounds");
    assert!(wide > shallow, "a wider compound carries more: {wide} vs {shallow}");

    assert!(knowledge_of(&[at(1, 1, 1, 1, 0)]).is_none(), "no compound, no knowledge");
}
