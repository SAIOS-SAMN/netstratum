//! The thermal core (05_THERMAL_BALANCE.md §1–§3): the exact-rational
//! equilibrium baseline and substrate-band membership.
//!
//! **The law, era 1 (docs/78 design law 2, refactored in place 2026-10-15):**
//! no square and no fixed dimension states a law. Flux through a
//! fluid-dimension shell falls linearly, `I = L (1 − A) / r`, and the
//! equilibrium temperature is held as `T` itself, in degrees, `T = K · I`:
//! exact for rational inputs, with no root anywhere. (Era 0 held `T⁴` from
//! a `1/r²` flux, a flat 3-dimensional law; the names `t_eq4`, `bounds4`
//! and `ceiling4` keep their call sites but now carry `T`.)
//!
//! All numbers here are placeholder parameters: `K` is anchored so a
//! yellow star's world (`L` = 1) at `r` = 1, albedo 0 sits at
//! `T` = 279° (mesic), i.e. `K = 279`.

use crate::ratio::{ratio, Ratio};
use num_traits::ToPrimitive;

/// A cited thermal anchor, exact — the only way a band bound or the folded
/// constant enters this module (P2, operator ruling 2026-08-05): the
/// parameter is a [`crate::anchor::ThermalAnchor`], a typed declaration
/// carrying its own band meaning. Era 1 holds temperature itself (no
/// fourth power).
fn anchor_degrees(anchor: crate::anchor::ThermalAnchor) -> Ratio {
    ratio(anchor.degrees(), 1)
}

/// The thermal constant `K`: the home-equilibrium anchor in degrees, so
/// `T(L=1, A=0, r=1) = K` lands mesic by calibration.
pub fn thermal_k() -> Ratio {
    anchor_degrees(crate::anchor::ThermalAnchor::HomeEquilibrium)
}

/// `T = K · L (1 − A) / r` — the equilibrium temperature, in degrees, from
/// the linear flux (era 1). Exact for rational inputs.
pub fn t_eq4(luminosity: &Ratio, albedo: &Ratio, orbit_r: &Ratio) -> Ratio {
    insolation(luminosity, albedo, orbit_r) * thermal_k()
}

/// **The continuous insolation** `I = L·(1−A)/r` (era 1: linear) — the absorbed stellar
/// flux per unit area, an exact rational (COG Slice 2b). This is the
/// physical vector the 8-way [`ThermalRegion`](crate::almanac::ThermalRegion)
/// buckets discard: `t_eq4` collapses it into a temperature and bins it, but
/// primary production reads it natively, so a bright or close star delivers
/// more food than a dim or distant one — continuously, not in temperature
/// steps. `L` is the star's own luminosity ([`StarClass::luminosity`]), so
/// the star class injects its exact energy output straight into the food
/// economy; the temperature bucket becomes an efficiency term (η), never the
/// magnitude of the light.
pub fn insolation(luminosity: &Ratio, albedo: &Ratio, orbit_r: &Ratio) -> Ratio {
    let one_minus_a = ratio(1, 1) - albedo.clone();
    luminosity.clone() * one_minus_a / orbit_r.clone()
}

/// The substrate equilibrium bands (05 §2): each is an inclusive interval
/// in degrees. A body's occupant substrate must match its band.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubstrateBand {
    /// 10° – 100°: quantum/superconducting/high-density compute.
    Cryogenic,
    /// 250° – 320°: carbon-based organics, hydro-biospheres.
    Mesic,
    /// 600° – 1500°: high-flux foundries, plasma, lithic life.
    HyperThermal,
}

impl SubstrateBand {
    /// The inclusive `[lo, hi]` bounds in degrees, from the cited anchors.
    pub fn bounds4(self) -> (Ratio, Ratio) {
        use crate::anchor::ThermalAnchor as At;
        match self {
            SubstrateBand::Cryogenic => (
                anchor_degrees(At::CryoFloor),
                anchor_degrees(At::CryoCeiling),
            ),
            SubstrateBand::Mesic => (
                anchor_degrees(At::MesicFloor),
                anchor_degrees(At::MesicCeiling),
            ),
            SubstrateBand::HyperThermal => (
                anchor_degrees(At::FoundryFloor),
                anchor_degrees(At::FoundryCeiling),
            ),
        }
    }
}

/// The falloff length for [`coverage`], in degrees.
///
/// **Derived, not chosen.** It is the width of the mesic band — the
/// span a temperate world already occupies — so a world one band-width
/// outside the nearest band reads half coverage. Using a measured span
/// rather than a tuned literal is what keeps this a reading (§7 of
/// `20_FOUNDING`).
pub fn coverage_scale4() -> Ratio {
    let (lo, hi) = SubstrateBand::Mesic.bounds4();
    hi - lo
}

/// Coverage — how well a world at `t_eq4` supports substrate, as an
/// exact rational in `(0, 1]`, defined everywhere on the positive line.
///
/// **This replaces a wall with a slope.** [`band_of`] answers whether a
/// world falls inside one of three disjoint windows, so a world one
/// degree outside a bound reads nothing while one degree inside reads
/// everything — and the three windows do not cover the line, leaving
/// 320–600° supported by nothing at all. The derived furnace seats
/// (mineral and plasmic foundings) land in that gap, which made a whole
/// class of world uninhabitable by construction rather than by design
/// (`20_FOUNDING` §4, §6c).
///
/// The bands remain, as the regions where coverage is rich:
///
/// ```text
/// inside any band          coverage = 1        a plateau, not a peak
/// outside every band       1 / (1 + d/scale)   d = degrees to the nearest edge
/// ```
///
/// **Monotone in distance, strictly positive everywhere, and free of
/// squares** — the difference enters to the first power only, so the
/// proportional law is untouched (`EQUATIONS` ch.2: squares are barred
/// as aggregators of difference; quartics survive only as physical law,
/// and `T⁴` is already the carrier here).
///
/// A gap world now reads poor rather than impossible, which is a game
/// where the alternative was a defect.
pub fn coverage(t_eq4: &Ratio) -> Ratio {
    let zero = ratio(0, 1);
    let one = ratio(1, 1);
    let mut nearest: Option<Ratio> = None;
    for band in [
        SubstrateBand::Cryogenic,
        SubstrateBand::Mesic,
        SubstrateBand::HyperThermal,
    ] {
        let (lo, hi) = band.bounds4();
        let gap = if *t_eq4 < lo {
            lo - t_eq4.clone()
        } else if *t_eq4 > hi {
            t_eq4.clone() - hi
        } else {
            return one;
        };
        nearest = Some(match nearest {
            Some(held) if held <= gap => held,
            _ => gap,
        });
    }
    match nearest {
        Some(distance) if distance > zero => one.clone() / (one + distance / coverage_scale4()),
        _ => one,
    }
}

/// THERMAL-1, the cryo law (05 §1.2): at or below this temperature the
/// store is a lossless vault — the cryogenic band's ceiling, 100°.
pub fn cryo_vault_max4() -> Ratio {
    SubstrateBand::Cryogenic.bounds4().1
}

/// Does an accrued ember store leak at this temperature? False in the vault
/// (cryo band and colder), true everywhere warmer (05 §1.2).
pub fn leaks(t_eq4: &Ratio) -> bool {
    *t_eq4 > cryo_vault_max4()
}

/// The band a temperature falls in, or `None` when it sits between/outside the
/// known substrate bands (no known substrate holds equilibrium there).
pub fn band_of(t_eq4: &Ratio) -> Option<SubstrateBand> {
    for band in [
        SubstrateBand::Cryogenic,
        SubstrateBand::Mesic,
        SubstrateBand::HyperThermal,
    ] {
        let (lo, hi) = band.bounds4();
        if *t_eq4 >= lo && *t_eq4 <= hi {
            return Some(band);
        }
    }
    None
}

/// The enumerator of known stars (NS-6 §7): each class fixes its star's
/// luminosity `L` (relative to the yellow class = 1), which drives every bound body's
/// `T_eq⁴`. Discriminants are the permanent wire codes (NS-6 §7).
/// Values are placeholder parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StarClass {
    // Main sequence, by colour, hottest to coolest.
    Blue = 1,
    BlueWhite = 2,
    White = 3,
    YellowWhite = 4,
    Yellow = 5,
    Orange = 6,
    Red = 7,
    // Remnants, giants, and the lore exotic.
    BrownDwarf = 8,
    WhiteDwarf = 9,
    RedGiant = 10,
    NeutronStar = 11,
    WindStripped = 12,
    LuminalWhiteHole = 13,
}

impl StarClass {
    pub fn from_wire(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Blue),
            2 => Some(Self::BlueWhite),
            3 => Some(Self::White),
            4 => Some(Self::YellowWhite),
            5 => Some(Self::Yellow),
            6 => Some(Self::Orange),
            7 => Some(Self::Red),
            8 => Some(Self::BrownDwarf),
            9 => Some(Self::WhiteDwarf),
            10 => Some(Self::RedGiant),
            11 => Some(Self::NeutronStar),
            12 => Some(Self::WindStripped),
            13 => Some(Self::LuminalWhiteHole),
            _ => None,
        }
    }

    /// **Magnetism — the field grade of this class** (operator ruling
    /// 2026-08-06: *how many planets can orbit a star should be based
    /// on the magnetism of the star associated with the star system*).
    ///
    /// Derived from the dynamo, the way the abundance spine wears
    /// metallicity: a star's field comes from its convective dynamo,
    /// and convection deepens down the main sequence — blue, blue-white and white are
    /// radiative (weak fossil fields), yellow-white begins shallow convection,
    /// yellow is the reference dynamo, orange runs deeper, red is fully convective and
    /// carries the strongest main-sequence fields (the packed systems
    /// are the measured case). Remnants: the white dwarf keeps a fossil
    /// megagauss field; the neutron star is the strongest field in
    /// nature but compact — a tight magnetosphere sweeps clean, so its
    /// grade is anchored to the measured case (three planets about a
    /// pulsar); the wind-stripped star
    /// strips its own disk with wind. Grades are placeholder dials
    /// ruled at balance, like every figure in this table.
    pub fn magnetism(self) -> u8 {
        match self {
            StarClass::Blue => 2,
            StarClass::BlueWhite => 3,
            StarClass::White => 3,
            StarClass::YellowWhite => 5,
            StarClass::Yellow => 8,
            StarClass::Orange => 9,
            StarClass::Red => 12,
            StarClass::BrownDwarf => 10,
            StarClass::WhiteDwarf => 6,
            StarClass::RedGiant => 4,
            StarClass::NeutronStar => 3,
            StarClass::WindStripped => 2,
            StarClass::LuminalWhiteHole => 12,
        }
    }

    /// **The ring REACH — how many rings this star's field anchors.**
    /// The law the ruling asked for: planet capacity is based on the
    /// magnetism, and at v1 the derivation is identity — one ring per
    /// field grade. Yellow reaches eight, which lands the home calibration
    /// exactly: the home star's dynamo holds the home count. A ring is a distinct
    /// `orbit_r`; slots within a ring stay the turn law's business
    /// (60 × 60 per ring, ψ separating) — magnetism bounds how many
    /// rings exist, never how a ring fills.
    pub fn ring_reach(self) -> u8 {
        self.magnetism()
    }

    /// Luminosity relative to the yellow class — the cited anchor table
    /// (`crate::anchor::luminosity`, operator ruling 2026-08-05) for
    /// every astronomical class; the lore class alone keeps a dial
    /// here, because fiction has no empirical citation and the census
    /// counts it honestly.
    pub fn luminosity(self) -> Ratio {
        crate::anchor::luminosity(self).unwrap_or_else(|| match self {
            StarClass::LuminalWhiteHole => ratio(1_000_000, 1),
            anchored => unreachable!("{anchored:?} is anchored"),
        })
    }

    /// The star's mass in yellow-star masses (the yellow class = 1) — the anchor Kepler's third
    /// law reads. The lore White Hole has no citation, so it takes the yellow
    /// baseline (trivial, no dial).
    pub fn mass(self) -> Ratio {
        crate::anchor::mass(self).unwrap_or_else(|| match self {
            StarClass::LuminalWhiteHole => ratio(1, 1),
            anchored => unreachable!("{anchored:?} is anchored"),
        })
    }
}

/// The base orbital period: a reference yellow-star world at `r = 1` completes its
/// 360° circuit in this many universal beats — one beat per degree of arc, the
/// Base-60/360 tie-in. Every other period scales off it by Kepler's law.
const BASE_ORBIT_BEATS: i64 = 360; // the beat↔degree definition (360° circuit)

/// **The keplerian period, squared** (the Keplerian clock) — Kepler's third law
/// `P² = (360)²·r³/M` in exact rationals (`r` in unit orbits, `M` in yellow-star masses, `P` in beats).
/// Squared because the period itself, `√(r³/M)`, is irrational and a ℚ-engine
/// cannot hold it — but the semantic quantity, the count of completed orbits, is
/// exact from `P²` alone (see [`local_years`]). A close orbit or a heavy star
/// gives a small `P²` (a short year); a distant orbit or a light star a large one.
pub fn orbital_period_squared(orbit_r: &Ratio, mass: &Ratio) -> Ratio {
    let base = ratio(BASE_ORBIT_BEATS, 1);
    let r3 = orbit_r.clone() * orbit_r.clone() * orbit_r.clone();
    (base.clone() * base) * r3 / mass.clone()
}

/// The exact rational square root of `x`, or `None` when `x` is not a perfect
/// rational square. `√(p/q) = √p/√q` is exact iff both the numerator and
/// denominator (of the reduced fraction) are perfect integer squares — the
/// `BigRational` is always reduced, so this test is total and exact. This is
/// the diophantine window: a track sited by `siting.rs` at `r = M·k²` has
/// `P² = BASE²·M²·k⁶`, a perfect rational square, so its period `P` is an
/// exact rational and no root is ever approximated.
fn exact_sqrt(x: &Ratio) -> Option<Ratio> {
    if *x < ratio(0, 1) {
        return None;
    }
    let n = x.numer();
    let d = x.denom();
    let sn = n.sqrt();
    let sd = d.sqrt();
    if &(sn.clone() * sn.clone()) == n && &(sd.clone() * sd.clone()) == d {
        Some(Ratio::new(sn, sd))
    } else {
        None
    }
}

/// **The LOCAL YEAR** (the Keplerian clock, DIO-1) — how many complete 360°
/// orbits a planet has traced in `beats` universal ticks: `⌊beats / P⌋`.
///
/// **The last irrational is gone.** A Diophantine track (`siting.rs`,
/// `r = M·k²`, `P = M·k³·BASE`) has a perfect-square `P²`, so `P` is an exact
/// rational and the year is a pure rational floor `⌊beats / P⌋` — the exact
/// angular step `Δθ = 360/P` walked `beats` times, no integer square root. For
/// any other `r` (an operator-pinned off-track world) `P` may be irrational;
/// there the computation falls back to the exact integer-sqrt floor `⌊√(beats²/P²)⌋`, which
/// is the same value computed without forming the root. The beat is the
/// absolute cosmic tick; the year is local geometry — a tight inner world
/// rolls its calendar often, a distant one seldom, on the very same beat.
pub fn local_years(orbit_r: &Ratio, mass: &Ratio, beats: u64) -> u64 {
    let p2 = orbital_period_squared(orbit_r, mass);
    if p2 <= ratio(0, 1) {
        return 0;
    }
    // The diophantine path: P² is a perfect rational square ⇒ P is exact ⇒
    // the year is a direct rational floor, no isqrt.
    if let Some(period) = exact_sqrt(&p2) {
        let years = (ratio(beats as i64, 1) / period).floor();
        return years.to_integer().to_u128().unwrap_or(u128::MAX as u128) as u64;
    }
    // The fallback (off-track r): the exact integer-sqrt floor of beats²/P².
    let beats2 = ratio(beats as i64, 1) * ratio(beats as i64, 1);
    let x = (beats2 / p2)
        .floor()
        .to_integer()
        .to_u128()
        .unwrap_or(u128::MAX);
    let mut lo = 0u64;
    let mut hi = 1u64;
    while (hi as u128).saturating_mul(hi as u128) <= x {
        hi = hi.saturating_mul(2);
    }
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if (mid as u128).saturating_mul(mid as u128) <= x {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(n: i64, d: i64) -> Ratio {
        ratio(n, d)
    }

    #[test]
    fn stellar_mass_ranks_the_main_sequence() {
        // Massive hot stars burn heavy; dim red stars are tiny.
        assert!(
            StarClass::Blue.mass() > StarClass::Yellow.mass(),
            "blue outweighs yellow"
        );
        assert_eq!(StarClass::Yellow.mass(), r(1, 1), "yellow is the mass unit");
        assert!(
            StarClass::Yellow.mass() > StarClass::Red.mass(),
            "yellow outweighs red"
        );
        assert_eq!(StarClass::Red.mass(), r(1, 5), "red is the tiny red star");
    }

    #[test]
    fn the_keplerian_clock_ages_planets_by_their_own_geometry() {
        use crate::thermal::{local_years, orbital_period_squared};
        let g = StarClass::Yellow.mass();
        // Kepler: a distant orbit is a longer year than a close one (same star).
        let close = orbital_period_squared(&r(1, 2), &g);
        let far = orbital_period_squared(&r(4, 1), &g);
        assert!(far > close, "a distant world has a longer period");
        // A heavier star pulls a shorter year at the same orbit.
        let round_heavy = orbital_period_squared(&r(1, 1), &StarClass::Blue.mass());
        let round_home = orbital_period_squared(&r(1, 1), &g);
        assert!(round_heavy < round_home, "a heavier star shortens the year");
        // The reference yellow-star world at r=1 completes its 360-beat circuit exactly:
        // 0 years before beat 360, exactly 1 at beat 360, 2 at 720.
        assert_eq!(local_years(&r(1, 1), &g, 359), 0, "not yet a full orbit");
        assert_eq!(
            local_years(&r(1, 1), &g, 360),
            1,
            "one 360-beat circuit = one local year"
        );
        assert_eq!(local_years(&r(1, 1), &g, 720), 2, "two circuits, two years");
        // Two planets, same beat, different ages: the close-and-heavy world has
        // lived far more local years than a distant one on the same cosmic tick.
        let beat = 3600;
        let inner = local_years(&r(1, 4), &StarClass::Blue.mass(), beat);
        let outer = local_years(&r(9, 1), &StarClass::Red.mass(), beat);
        assert!(
            inner > outer,
            "same beat, but the inner-heavy world is far older: {inner} vs {outer}"
        );
    }

    #[test]
    fn the_diophantine_clock_is_an_exact_rational_floor() {
        use crate::thermal::{exact_sqrt, local_years, orbital_period_squared};
        let g = StarClass::Yellow.mass();
        // A Diophantine track r = M·k² has a perfect-square P², so the period
        // is an exact rational and the year is a pure floor ⌊T/P⌋ — no isqrt.
        let k = r(3, 2); // an off-lattice continuous k
        let rr = g.clone() * k.clone() * k.clone(); // r = M·k²
        let p2 = orbital_period_squared(&rr, &g);
        let period = exact_sqrt(&p2).expect("a Diophantine track has an exact rational period");
        // P = base·M·k³ exactly (base = 360, M = 1): 360·(3/2)³ = 360·27/8.
        assert_eq!(
            period,
            r(360, 1) * k.clone() * k.clone() * k,
            "P = BASE·M·k³ exact"
        );
        // The year count is the exact rational floor ⌊beats/P⌋.
        let p_int = period.to_integer().to_u128().unwrap() as u64; // P here is integral
        assert_eq!(
            local_years(&rr, &g, p_int),
            1,
            "one exact period = one year"
        );
        assert_eq!(local_years(&rr, &g, p_int - 1), 0, "not yet a full period");
        assert_eq!(
            local_years(&rr, &g, 3 * p_int),
            3,
            "three periods, three years"
        );
        // A non-Diophantine r (P² not a perfect square) has no exact period —
        // the isqrt fallback still answers, and agrees at the reference world.
        assert_eq!(
            exact_sqrt(&orbital_period_squared(&r(2, 1), &g)),
            None,
            "r=2 is off-track"
        );
        assert_eq!(
            local_years(&r(1, 1), &g, 720),
            2,
            "the reference world still ages exactly"
        );
    }

    #[test]
    fn home_equilibrium_body_sits_mesic_at_exactly_279() {
        // Yellow star, L=1, albedo 0, r=1 → T == K == 279°, the mesic anchor.
        let t = t_eq4(&StarClass::Yellow.luminosity(), &r(0, 1), &r(1, 1));
        assert_eq!(t, r(279, 1));
        assert_eq!(band_of(&t), Some(SubstrateBand::Mesic));
    }

    #[test]
    fn albedo_cools_by_exactly_one_minus_a() {
        let full = t_eq4(&r(1, 1), &r(0, 1), &r(1, 1));
        let half = t_eq4(&r(1, 1), &r(1, 2), &r(1, 1));
        assert_eq!(half, full / r(2, 1)); // exact halving, no rounding
    }

    #[test]
    fn linear_in_orbit_radius() {
        // Era 1 (design law 2): flux falls linearly, no square.
        let near = t_eq4(&r(1, 1), &r(0, 1), &r(1, 1));
        let far = t_eq4(&r(1, 1), &r(0, 1), &r(2, 1));
        assert_eq!(far, near / r(2, 1)); // r doubles ⇒ flux halves, exact
    }

    #[test]
    fn red_star_at_one_unit_is_cold() {
        // A red star's world at r=1 is far colder than mesic: the warm
        // orbits hug a dim star.
        let t = t_eq4(&StarClass::Red.luminosity(), &r(0, 1), &r(1, 1));
        assert!(t < SubstrateBand::Mesic.bounds4().0);
    }

    #[test]
    fn blue_star_at_one_unit_exceeds_every_band() {
        // Too hot for any substrate at r=1 — the equilibrium band is pushed
        // far outward (05 §3.1). Farther out is strictly cooler.
        let hot = t_eq4(&StarClass::Blue.luminosity(), &r(0, 1), &r(1, 1));
        assert_eq!(band_of(&hot), None);
        let far = t_eq4(&StarClass::Blue.luminosity(), &r(0, 1), &r(100, 1));
        assert!(far < hot);
    }

    #[test]
    fn band_boundaries_are_exact_and_inclusive() {
        assert_eq!(band_of(&r(250, 1)), Some(SubstrateBand::Mesic)); // 250° lo
        assert_eq!(band_of(&r(320, 1)), Some(SubstrateBand::Mesic)); // 320° hi
        assert_eq!(band_of(&r(321, 1)), None); // one degree above ⇒ no band
    }

    #[test]
    fn main_sequence_luminosity_is_monotonic() {
        let seq = [
            StarClass::Blue,
            StarClass::BlueWhite,
            StarClass::White,
            StarClass::YellowWhite,
            StarClass::Yellow,
            StarClass::Orange,
            StarClass::Red,
        ];
        for pair in seq.windows(2) {
            assert!(pair[0].luminosity() > pair[1].luminosity());
        }
    }
}
