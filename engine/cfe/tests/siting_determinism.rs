//! FOUNDING SITING IS DETERMINISTIC (probe, 2026-08-02).
//!
//! `conductor::derived_siting` draws one of three seats from the
//! beacon. The draw is NOT what places a line: all three seats of a
//! metabolism land in the SAME thermal outcome. Metabolism is the law;
//! the draw is orbital detail. Pinned so a future seat edit that breaks
//! that cannot land quietly.
use cfe::ratio::Ratio;
use cfe::thermal::{band_of, t_eq4, StarClass, SubstrateBand};

fn r(n: i64, d: i64) -> Ratio { Ratio::new(n.into(), d.into()) }

/// Furnace seats (mineral | plasmic): every draw lands ABOVE the mesic
/// ceiling and BELOW the hyperthermal floor — founding is BANDLESS.
#[test]
fn every_furnace_seat_founds_bandless() {
    for (i, albedo) in [r(1, 2), r(9, 20), r(2, 5)].iter().enumerate() {
        let t4 = t_eq4(&StarClass::G.luminosity(), albedo, &r(1, 2));
        assert_eq!(band_of(&t4), None, "furnace seat {i} acquired a band at founding");
        assert!(t4 > SubstrateBand::Mesic.bounds4().1, "furnace seat {i} not past mesic");
        assert!(t4 < SubstrateBand::HyperThermal.bounds4().0, "furnace seat {i} reached hyperthermal");
    }
}

/// Mesic seats (biotic): every draw lands INSIDE the mesic band.
#[test]
fn every_mesic_seat_founds_mesic() {
    for (i, (albedo, orbit)) in
        [(r(0, 1), r(1, 1)), (r(1, 10), r(1, 1)), (r(0, 1), r(6, 5))].iter().enumerate()
    {
        let t4 = t_eq4(&StarClass::G.luminosity(), albedo, orbit);
        assert_eq!(band_of(&t4), Some(SubstrateBand::Mesic), "mesic seat {i} left the band");
    }
}
