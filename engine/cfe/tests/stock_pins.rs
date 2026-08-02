//! THE REGENERATING STOCK — 20_FOUNDING §6g.
//!
//! The first pin is the one that matters: an exact-rational logistic
//! recurrence explodes its denominators, and this engine has already
//! ruled on a gentler version of that failure (demurrage is FLAT
//! because `banked × (31/32)^n` measured ~2^1100 after 220 beats). If
//! the lattice ever stops bounding denominators, the model becomes
//! unrunnable long before it becomes wrong.

use cfe::ratio::{ratio, Ratio};
use cfe::world::stock::ResourceStock;

fn tended() -> ResourceStock {
    ResourceStock {
        current: ratio(10, 1),
        ceiling: ratio(100, 1),
        rate: ratio(1, 8),
        seed_floor: ratio(1, 4),
        residue: ratio(0, 1),
    }
}

fn denom_bits(value: &Ratio) -> u64 {
    value.denom().bits()
}

/// THE LATTICE HOLDS. 400 ticks and every denominator stays bounded by
/// the lattice — the pin that would have caught the unquantized design.
#[test]
fn denominators_stay_bounded_forever() {
    let mut stock = tended();
    let zero = ratio(0, 1);
    let mut worst = 0;
    for _ in 0..400 {
        stock.step(&zero);
        worst = worst.max(denom_bits(&stock.current));
    }
    assert!(
        worst <= 11,
        "denominator reached {worst} bits — the lattice is not bounding the recurrence, \
         and an exact-rational logistic explodes (measured: 414,305 bits by tick 16)"
    );
}

/// It is still LOGISTIC: the curve rises and saturates at the ceiling
/// rather than growing without bound or stalling.
#[test]
fn the_curve_rises_and_saturates() {
    let mut stock = tended();
    let zero = ratio(0, 1);
    let start = stock.current.clone();
    for _ in 0..20 {
        stock.step(&zero);
    }
    let mid = stock.current.clone();
    for _ in 0..200 {
        stock.step(&zero);
    }
    assert!(mid > start, "the stock did not grow");
    assert!(stock.current > mid, "the stock stalled before the ceiling");
    assert!(stock.current <= stock.ceiling, "the stock overran its ceiling");
    assert!(
        stock.ceiling.clone() - stock.current.clone() < ratio(1, 10),
        "the stock never approached its ceiling"
    );
}

/// IT COMPOUNDS — a larger tended stock outgrows a smaller one over the
/// same span. This is the property a flat faucet cannot have.
#[test]
fn a_tended_stock_outgrows_a_small_one() {
    let zero = ratio(0, 1);
    let grow = |start: Ratio| {
        let mut s = ResourceStock { current: start.clone(), ..tended() };
        for _ in 0..40 {
            s.step(&zero);
        }
        s.current - start
    };
    let small = grow(ratio(1, 1));
    let large = grow(ratio(10, 1));
    assert!(
        large > small,
        "a bigger stock gained no more than a smaller one — the model is not compounding"
    );
}

/// RESEARCH RAISES THE RATE, and a raised rate compounds. The whole
/// point of §6g: research must not be a one-time multiplier on a flow.
#[test]
fn research_on_the_rate_pays_more_than_once() {
    let zero = ratio(0, 1);
    let run = |boost: Ratio| {
        let mut s = tended();
        s.research_rate(&boost);
        let mut path = Vec::new();
        for _ in 0..30 {
            s.step(&zero);
            path.push(s.current.clone());
        }
        path
    };
    let plain = run(ratio(0, 1));
    let researched = run(ratio(1, 8));
    assert!(researched[5] > plain[5], "research bought nothing early");
    let early_gap = researched[5].clone() - plain[5].clone();
    let later_gap = researched[15].clone() - plain[15].clone();
    assert!(
        later_gap > early_gap,
        "the research gap did not widen — a rate rise must compound, not step once"
    );
}

/// OVER-EXTRACTION BITES, and the seed floor holds: a stock is driven
/// down to its seed core and never below, so scorched earth is punished
/// without permanent death.
#[test]
fn over_extraction_falls_to_the_seed_floor_and_no_further() {
    let mut stock = tended();
    let heavy = ratio(4, 1);
    for _ in 0..60 {
        stock.step(&heavy);
    }
    assert!(stock.current <= stock.seed_floor.clone() + ratio(1, 1024), "the floor did not hold");
    assert!(stock.current >= stock.seed_floor, "the stock breached its seed core");
    // Left fallow, the seed core climbs back — the stock is not dead.
    let zero = ratio(0, 1);
    for _ in 0..60 {
        stock.step(&zero);
    }
    assert!(
        stock.current > stock.seed_floor,
        "a floored stock did not recover when left fallow — that is extinction, not a floor"
    );
}

/// A SUSTAINABLE DRAW leaves the stock standing; a draw past it does
/// not. This is the operand §6a wants for capture capacity.
#[test]
fn drawing_the_regeneration_holds_the_stock_steady() {
    let mut stock = tended();
    for _ in 0..25 {
        stock.step(&ratio(0, 1));
    }
    let settled = stock.current.clone();
    let draw = stock.sustainable_draw();
    stock.step(&draw);
    let after = stock.current.clone();
    let drift = match after >= settled {
        true => after.clone() - settled.clone(),
        false => settled.clone() - after.clone(),
    };
    assert!(drift < ratio(1, 100), "drawing exactly the regeneration moved the stock");
    let mut greedy = ResourceStock { current: settled.clone(), ..tended() };
    greedy.step(&(draw + ratio(1, 1)));
    assert!(greedy.current < settled, "drawing past regeneration did not shrink the stock");
}

/// FLOURISH reinvests into the base and never past the ceiling.
#[test]
fn reinvested_surplus_raises_the_base_within_the_ceiling() {
    let mut stock = tended();
    let before = stock.current.clone();
    stock.reinvest(&ratio(15, 1));
    assert!(stock.current > before, "reinvestment did not raise the stock");
    stock.reinvest(&ratio(10_000, 1));
    assert_eq!(stock.current, stock.ceiling, "reinvestment overran the ceiling");
}

/// CONSERVATION: flooring must not leak matter. `current + residue` is
/// exactly the pre-floor quantity at every step, so the total the stock
/// accounts for equals what regeneration added minus what was drawn.
///
/// Without the residue this leaked 0.347 units over 400 ticks — 0.346%
/// of the total — which the kernel's I1 arm would refuse the moment a
/// stock reached committed state.
#[test]
fn flooring_leaks_no_matter() {
    let mut stock = tended();
    let mut books = stock.current.clone();
    let draw = ratio(1, 3);
    for _ in 0..400 {
        let regen = stock.regeneration();
        let drawn = stock.step(&draw);
        books = books + regen - drawn;
    }
    assert_eq!(
        stock.current.clone() + stock.residue.clone(),
        books,
        "matter entered or left the system across 400 ticks — the residue is not carrying"
    );
}

/// The residue stays small and its denominator stays bounded, so
/// carrying it cannot reintroduce the explosion the lattice prevents.
#[test]
fn the_residue_stays_bounded() {
    let mut stock = tended();
    let draw = ratio(1, 7);
    let mut worst_bits = 0;
    for _ in 0..400 {
        stock.step(&draw);
        worst_bits = worst_bits.max(denom_bits(&stock.residue));
        let size = match stock.residue >= ratio(0, 1) {
            true => stock.residue.clone(),
            false => ratio(0, 1) - stock.residue.clone(),
        };
        assert!(size < ratio(1, 1024), "the residue grew past one lattice step");
    }
    assert!(worst_bits <= 64, "the residue denominator reached {worst_bits} bits");
}

/// A draw larger than the land holds returns what the land holds —
/// never more, and never by conjuring stock up to the floor.
#[test]
fn a_draw_past_the_land_returns_what_the_land_holds() {
    let mut stock = tended();
    let drawn = stock.step(&ratio(10_000, 1));
    assert!(drawn < ratio(10_000, 1), "the draw was not capped at what stands");
    assert!(drawn > ratio(0, 1), "a stock above its floor yielded nothing");
    assert!(stock.current >= stock.seed_floor, "the seed core was breached");
}

/// COVERAGE is defined everywhere — the gap that made a whole class of
/// world uninhabitable is closed, and the bands are plateaus now.
#[test]
fn coverage_is_positive_on_the_whole_line() {
    use cfe::thermal::{coverage, SubstrateBand};
    let probes = [
        ratio(1, 1),                    // frozen dark
        ratio(50_000, 1),               // inside cryogenic
        ratio(1_000_000_000, 1),        // between cryo and mesic
        ratio(6_000_000_000, 1),        // inside mesic
        ratio(13_000_000_000, 1),       // THE FURNACE GAP — derived seats land here
        ratio(500_000_000_000, 1),      // inside hyperthermal
        ratio(90_000_000_000_000, 1),   // past every band
    ];
    for t4 in probes {
        let read = coverage(&t4);
        assert!(read > ratio(0, 1), "coverage read zero at {t4:?} — the line is not covered");
        assert!(read <= ratio(1, 1), "coverage ran past one at {t4:?}");
    }
    // Inside a band is a PLATEAU, not a peak.
    let (lo, hi) = SubstrateBand::Mesic.bounds4();
    assert_eq!(coverage(&lo), ratio(1, 1), "a band edge is not full coverage");
    assert_eq!(coverage(&hi), ratio(1, 1), "a band edge is not full coverage");
}

/// The furnace gap reads POOR, never impossible — the defect
/// 20_FOUNDING §4 named, closed.
#[test]
fn the_furnace_gap_is_poor_and_not_impossible() {
    use cfe::thermal::{band_of, coverage};
    let seats = [
        ratio(12_118_000_000i64, 1),
        ratio(13_330_000_000i64, 1),
        ratio(14_542_000_000i64, 1),
    ];
    for seat in seats {
        assert_eq!(band_of(&seat), None, "fixture drifted — the seat found a band");
        let read = coverage(&seat);
        assert!(read > ratio(0, 1), "a founding seat reads uninhabitable");
        assert!(read < ratio(1, 1), "a gap seat reads as rich as a band core");
    }
}

/// Coverage falls with distance FROM THE NEAREST BAND — not with
/// temperature. Between two bands it descends into the valley and
/// climbs the far side, which is the correct shape and was worth
/// getting wrong once: the first version of this pin assumed monotone
/// in T⁴ and failed at 100e9, where a world is closer to hyperthermal
/// than to mesic.
#[test]
fn coverage_falls_with_distance_from_the_nearest_band() {
    use cfe::thermal::coverage;
    // Walking away from the mesic ceiling, distance strictly grows.
    let a = coverage(&ratio(11_000_000_000i64, 1));
    let b = coverage(&ratio(20_000_000_000i64, 1));
    let c = coverage(&ratio(40_000_000_000i64, 1));
    assert!(a > b && b > c, "coverage did not fall while distance grew");
    // And it CLIMBS again approaching the hyperthermal floor.
    let d = coverage(&ratio(120_000_000_000i64, 1));
    assert!(d > c, "coverage did not recover approaching the next band");
}

/// RECURRING VALUE — what a stock yields forever, priced by where it
/// sits. Two identical deposits on different worlds are worth different
/// amounts, which is why location is an asset.
#[test]
fn recurring_value_prices_the_world_the_stock_sits_on() {
    use cfe::thermal::coverage;
    let mut stock = tended();
    for _ in 0..20 {
        stock.step(&ratio(0, 1));
    }
    let rich = coverage(&ratio(6_000_000_000i64, 1));       // inside mesic
    let poor = coverage(&ratio(13_330_000_000i64, 1));      // the furnace gap
    let on_rich = stock.recurring_value(&rich);
    let on_poor = stock.recurring_value(&poor);
    assert!(on_rich > on_poor, "location did not price the deposit");
    assert!(on_poor > ratio(0, 1), "a gap world yields nothing — that is the old defect");
    assert!(on_rich <= stock.regeneration(), "coverage minted value past regeneration");
}
