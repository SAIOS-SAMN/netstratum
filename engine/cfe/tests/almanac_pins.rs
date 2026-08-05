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
        for slot in id.iter_mut().take(8).skip(3) {
            *slot = b;
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
    assert_eq!(CELLS, 7 * 10 * 4 * 4 * 2, "five axes, multiplied");
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
    let left = [at(1, 1, 1, 1, 0), at(1, 5, 1, 1, 0)];
    let right = [at(1, 1, 1, 1, 0), at(1, 5, 1, 1, 1)];
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
    // **THE WIDENING PAYS OUT HERE.** Saturation is
    // `arity * (LANES/2 + 1) / 2`. At LANES = 8 that was `arity * 5 / 2`,
    // whole only for EVEN arities — so no three-body compound could
    // saturate, and the previous landing had to record that hole. At
    // LANES = 10 it is `arity * 3`, and every arity 2..=5 is reachable.
    //
    // Shell 2 takes THREE bodies; the binds must sum to 9.
    let ternary = [at(1, 1, 2, 1, 0), at(1, 3, 2, 1, 0), at(1, 5, 2, 1, 0)];
    assert_eq!(ternary.len(), 3, "shell 2 wants three");
    assert!(compounds(&ternary), "1 + 3 + 5 = 9 saturates at arity 3");
    assert!(gem_of(&ternary).is_some(), "and the three-body compound names a gem");

    // It is irreducibly NOT a pair: no two of its members compound.
    for pair in [[ternary[0], ternary[1]], [ternary[0], ternary[2]], [ternary[1], ternary[2]]] {
        assert!(!compounds(&pair), "a pairwise projection cannot express it");
    }

    // Shell 3 takes FOUR: saturation 4 * 3 = 12, and 1 + 5 + 1 + 5 = 12.
    let quaternary = [
        at(1, 1, 3, 1, 0),
        at(1, 5, 3, 1, 0),
        at(2, 1, 3, 1, 0),
        at(2, 5, 3, 1, 0),
    ];
    assert!(compounds(&quaternary), "four bodies, binds saturate at 12");
    assert!(gem_of(&quaternary).is_some(), "and it names a gem");

    // Shell 4 takes FIVE — the widest the table admits — needing 15.
    let quinary = [
        at(1, 1, 4, 1, 0),
        at(1, 2, 4, 1, 0),
        at(1, 3, 4, 1, 0),
        at(1, 4, 4, 1, 0),
        at(1, 5, 4, 1, 0),
    ];
    assert!(compounds(&quinary), "1+2+3+4+5 = 15 saturates at arity 5");

    // Arity is enforced: the same four at shell 1 are not a compound.
    let wrong_arity = [
        at(1, 1, 1, 1, 0),
        at(1, 5, 1, 1, 0),
        at(2, 1, 1, 1, 0),
        at(2, 5, 1, 1, 0),
    ];
    assert!(!compounds(&wrong_arity), "shell 1 takes two, not four");
}

#[test]
fn the_lane_width_is_derived_and_not_preferred() {
    // WHY 10. Saturation `arity * (LANES/2 + 1) / 2` is whole for every
    // arity exactly when `LANES/2 + 1` is even — that is, when
    // `LANES = 2 (mod 4)`. This is the pin that would fail if someone
    // widened the table to 12 by eye.
    assert_eq!(LANES % 4, 2, "the width admits every arity");
    for width in [8u32, 12, 16] {
        let odd_arity_whole = (3 * (width / 2 + 1)) % 2 == 0;
        assert!(!odd_arity_whole, "width {width} strands the three-body compound");
    }
    for arity in 2..=(1 + u32::from(SHELLS)) {
        assert_eq!(
            (arity * (u32::from(LANES) / 2 + 1)) % 2,
            0,
            "arity {arity} saturates in whole binds at LANES = {LANES}"
        );
    }
    // And the binds still fold at the half, which is what makes the
    // lane periodic rather than a ramp.
    assert_eq!(periodic_of(&at(1, 1, 1, 1, 0)).binds, periodic_of(&at(1, LANES, 1, 1, 0)).binds);
    assert_eq!(periodic_of(&at(1, LANES / 2, 1, 1, 0)).binds, LANES / 2, "the peak is the half");
}

#[test]
fn composition_agrees_on_shell_and_phase_and_the_gem_is_order_free() {
    let a = at(1, 1, 1, 2, 0);
    let b = at(1, 5, 1, 2, 0);
    assert!(compounds(&[a, b]), "1 + 4 saturates at arity 2");
    assert!(!compounds(&[a, a]), "1 + 1 does not");

    // Mismatched geometry and mismatched state both refuse.
    assert!(!compounds(&[a, at(1, 5, 2, 2, 0)]), "shells must agree");
    assert!(!compounds(&[a, at(1, 5, 1, 3, 0)]), "phases must agree");

    // The recurrence: the same lane pair compounds at ANY lines.
    for line_a in 1..=LINES {
        for line_b in 1..=LINES {
            assert!(compounds(&[at(line_a, 1, 1, 2, 0), at(line_b, 5, 1, 2, 0)]));
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
    let shallow = knowledge_of(&[at(1, 1, 1, 1, 0), at(1, 5, 1, 1, 0)]).expect("compounds");
    let deep = knowledge_of(&[at(7, 1, 1, 1, 0), at(7, 5, 1, 1, 0)]).expect("compounds");
    assert_eq!(shallow, 2);
    assert_eq!(deep, 14);
    assert!(deep > shallow, "depth is won");

    // WIDTH is won too: four bodies at the same depth carry more than
    // two, because arity is earned.
    let wide = knowledge_of(&[
        at(1, 1, 3, 1, 0),
        at(1, 5, 3, 1, 0),
        at(1, 1, 3, 1, 0),
        at(1, 5, 3, 1, 0),
    ])
    .expect("compounds");
    assert!(wide > shallow, "a wider compound carries more: {wide} vs {shallow}");

    assert!(knowledge_of(&[at(1, 1, 1, 1, 0)]).is_none(), "no compound, no knowledge");
}

// ── THE THERMAL CLOSURE ──────────────────────────────────────────────

use cfe::almanac::{ceiling4, region_of, workable_at, ThermalRegion};
use cfe::thermal::{band_of, t_eq4};

#[test]
fn the_band_reading_is_open_in_four_places_and_the_region_closes_it() {
    // The four silences `band_of` answers, each now a named place.
    let holes = [
        (ratio(1, 1), ThermalRegion::Quenched, "below the cryogenic floor"),
        (ratio(200_000_000, 1), ThermalRegion::Sublime, "between cryogenic and mesic"),
        (ratio(20_000_000_000, 1), ThermalRegion::Torrid, "between mesic and hyperthermal"),
        (ratio(6_000_000_000_000, 1), ThermalRegion::Incandescent, "above hyperthermal"),
    ];
    for (t, region, why) in holes {
        assert_eq!(band_of(&t), None, "band_of is silent here: {why}");
        assert_eq!(region_of(&t), region, "the region names it: {why}");
    }
}

#[test]
fn the_three_bands_are_carried_through_unchanged() {
    // Closure must not MOVE admission — every temperature that had a
    // band still reads the same place.
    for (t, region) in [
        (ratio(10_000, 1), ThermalRegion::Cryogenic),
        (ratio(3_906_250_000, 1), ThermalRegion::Mesic),
        (ratio(10_485_760_000, 1), ThermalRegion::Mesic),
        (ratio(129_600_000_000, 1), ThermalRegion::HyperThermal),
    ] {
        assert!(band_of(&t).is_some(), "this had a band");
        assert_eq!(region_of(&t), region, "and keeps its place");
    }
}

#[test]
fn the_region_is_total_where_the_band_is_not() {
    // Totality is the closure. Walk the line by orders of magnitude and
    // count how often each reading declines to answer.
    let mut silent = 0;
    let mut t = ratio(1, 1);
    for _ in 0..20 {
        if band_of(&t).is_none() {
            silent += 1;
        }
        // region_of is total by type — there is no None to count.
        let _ = region_of(&t);
        t *= ratio(10, 1);
    }
    assert!(silent > 0, "the band reading is genuinely open, not hypothetically");
    assert_eq!(
        region_of(&ratio(0, 1)),
        ThermalRegion::Quenched,
        "even absolute zero names a place"
    );
}

#[test]
fn the_top_is_open_until_an_orbit_floor_lands() {
    // THE MEASURED HOLE. `site_planet` refuses `orbit_r <= 0` and
    // admits everything above, so a lawful siting can exceed ANY
    // ceiling by shrinking the orbit.
    let hottest = cfe::thermal::StarClass::O.luminosity();
    let ceiling = ceiling4();
    // A lawful orbit one order smaller than the floor already passes it.
    let beyond = t_eq4(&hottest, &ratio(0, 1), &ratio(1, 100));
    assert!(beyond > ceiling, "a lawful siting outruns the ceiling");
    assert_eq!(
        region_of(&beyond),
        ThermalRegion::Beyond,
        "and the closure NAMES it rather than pretending otherwise"
    );
    // Nothing is workable there — the arm exists to become unreachable.
    let refractory = at(1, 1, 1, PHASES, 0);
    assert!(!workable_at(&refractory, &beyond), "no chemistry above the ceiling");
    assert!(workable_at(&refractory, &ratio(3_906_250_000, 1)), "and plenty below");
}

#[test]
fn a_shrinking_orbit_blows_the_denominator_which_is_the_old_scar() {
    // The same hole costs twice. `t_eq4` divides by r², so an orbit of
    // 1/10^k yields a denominator around 10^(2k) — the exact-rational
    // blowup that killed proportional demurrage at ~2^1100 and forced
    // the flat law.
    let hottest = cfe::thermal::StarClass::O.luminosity();
    let mut last = 0usize;
    for k in [1u32, 4, 8, 16] {
        let r = ratio(1, 10i64.pow(k));
        let t = t_eq4(&hottest, &ratio(0, 1), &r);
        let digits = t.numer().to_string().len() + t.denom().to_string().len();
        assert!(digits > last, "k={k}: the figure grows with every order");
        last = digits;
    }
    assert!(last > 30, "and it grows fast: {last} digits at r = 1/10^16");
}

// ── HEAT, AMPLIFIED BY DISTANCE, WITHOUT IRRATIONALITY ───────────────

use cfe::almanac::{grained_t_eq4, orbit_on_grain, ORBIT_GRAIN};

#[test]
fn heat_is_amplified_by_closeness_and_still_varies_across_the_grain() {
    // The law must keep its whole point: nearer is fiercer, and the
    // range is not flattened by quantising.
    let sun = cfe::thermal::StarClass::G.luminosity();
    let near = grained_t_eq4(&sun, &ratio(0, 1), &ratio(1, 4));
    let mid = grained_t_eq4(&sun, &ratio(0, 1), &ratio(1, 1));
    let far = grained_t_eq4(&sun, &ratio(0, 1), &ratio(4, 1));
    assert!(near > mid && mid > far, "closer is hotter, still");
    assert!(near > far * ratio(100, 1), "and the amplification is steep, not token");
}

#[test]
fn the_denominator_cannot_run_away_however_small_the_orbit() {
    // THE SCAR, closed. Ungrained, r = 1/10^k blew past 30 digits by
    // k = 16. Grained, every orbit's denominator is ORBIT_GRAIN, so
    // the heat's denominator is bounded no matter what is asked for.
    let hottest = cfe::thermal::StarClass::O.luminosity();
    let mut widest = 0usize;
    for k in [1u32, 4, 8, 16, 32] {
        let asked = ratio(1, 10i64.pow(k.min(18)));
        let on_grain = orbit_on_grain(&asked);
        // The invariant is DIVIDES, not equals: `Ratio::new` reduces,
        // so 2/64 is 1/32. Bounded by the grain is the claim, and it is
        // the one that bounds the arithmetic.
        let grain = num_bigint::BigInt::from(ORBIT_GRAIN);
        assert_eq!(
            grain.clone() % on_grain.denom().clone(),
            num_bigint::BigInt::from(0),
            "every grained orbit's denominator divides the grain: {on_grain}"
        );
        let t = grained_t_eq4(&hottest, &ratio(0, 1), &asked);
        widest = widest.max(t.numer().to_string().len() + t.denom().to_string().len());
    }
    assert!(widest < 30, "the figure stays holdable: {widest} digits at the widest");
}

#[test]
fn the_ceiling_is_derived_and_a_lawful_orbit_cannot_pass_it() {
    // No chosen bound on temperature: the ceiling falls out of the
    // luminosity table and the grain, and nothing grained exceeds it.
    let ceiling = ceiling4();
    for class in [
        cfe::thermal::StarClass::O,
        cfe::thermal::StarClass::B,
        cfe::thermal::StarClass::G,
        cfe::thermal::StarClass::M,
    ] {
        for orbit in [ratio(1, 1000000), ratio(1, 64), ratio(1, 2), ratio(9, 1)] {
            let t = grained_t_eq4(&class.luminosity(), &ratio(0, 1), &orbit);
            assert!(t <= ceiling, "{class:?} at {orbit} stays under the derived ceiling");
        }
    }
    // And the closure now has no reachable Beyond: the grained ceiling
    // IS the top of the named line.
    let hottest = grained_t_eq4(
        &cfe::thermal::StarClass::O.luminosity(),
        &ratio(0, 1),
        &ratio(0, 1),
    );
    assert_ne!(
        region_of(&hottest),
        ThermalRegion::Beyond,
        "with the grain, the open arm is unreachable"
    );
}

#[test]
fn a_sub_grain_orbit_is_not_a_nearer_orbit_it_is_not_an_orbit() {
    // Flooring, then lifting zero to one step. An orbit inside the
    // first grain does not get to be closer than the closest.
    let inside = orbit_on_grain(&ratio(1, 100000));
    let first = orbit_on_grain(&ratio(1, ORBIT_GRAIN));
    assert_eq!(inside, first, "everything inside the first step IS the first step");
    assert_eq!(orbit_on_grain(&ratio(0, 1)), first, "including nothing at all");
    // Above the grain it floors, never rounds up — no orbit is moved
    // closer to the star than it was asked to be.
    assert!(orbit_on_grain(&ratio(3, 2)) <= ratio(3, 2));
}
