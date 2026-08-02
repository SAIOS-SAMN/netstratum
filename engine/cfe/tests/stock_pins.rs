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
    assert_eq!(stock.current, stock.seed_floor, "the floor did not hold");
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
