//! The thermal core (05_THERMAL_BALANCE.md §1–§3): the exact-rational
//! master equation and substrate-band membership.
//!
//! **The law (05 §1.1):** the CFE never takes the fourth root and never
//! approximates. The canonical quantity is `T_eq⁴` — with `L`, albedo `A`,
//! and orbit radius `r` all exact rationals and the folded constant `K`
//! (= 1/(16πσ), the π carried in the symbolic-π extension, σ a rational
//! placeholder) inscribed as one exact rational, `T_eq⁴` is exact.
//! `T_eq` itself is display-only. Band thresholds are stored as fourth
//! powers, so band membership is an exact rational comparison — no root is
//! ever evaluated.
//!
//! All numbers here are PLACEHOLDER PARAMETERS (05 status header): `K` is
//! anchored so a Sol-type body (G star, `L`=1) at `r`=1, albedo 0 sits at
//! `T_eq`=279 K (mesic), i.e. `K = 279⁴`.

use crate::ratio::{ratio, Ratio};

/// The folded thermal constant `K = 1/(16πσ)` as one exact rational
/// (placeholder): `279⁴`, so `T_eq⁴(L=1, A=0, r=1) = K` lands mesic.
pub fn thermal_k() -> Ratio {
    ratio(6_059_221_281, 1)
}

/// `T_eq⁴ = L (1 − A) K / r²` — the master equation in flux space
/// (05 §1.1). Exact for rational inputs; the root is never taken.
pub fn t_eq4(luminosity: &Ratio, albedo: &Ratio, orbit_r: &Ratio) -> Ratio {
    let one_minus_a = ratio(1, 1) - albedo.clone();
    let r2 = orbit_r.clone() * orbit_r.clone();
    luminosity.clone() * one_minus_a * thermal_k() / r2
}

/// The substrate equilibrium bands (05 §2): each is an inclusive interval
/// in `T⁴` (K⁴). A body's occupant substrate must match its band.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubstrateBand {
    /// 10 K – 100 K: quantum/superconducting/high-density compute.
    Cryogenic,
    /// 250 K – 320 K: carbon-based organics, hydro-biospheres.
    Mesic,
    /// 600 K – 1500 K: high-flux foundries, plasma, lithic life.
    HyperThermal,
}

impl SubstrateBand {
    /// The inclusive `[lo⁴, hi⁴]` bounds in K⁴ (exact).
    pub fn bounds4(self) -> (Ratio, Ratio) {
        match self {
            // 10⁴ .. 100⁴
            SubstrateBand::Cryogenic => (ratio(10_000, 1), ratio(100_000_000, 1)),
            // 250⁴ .. 320⁴
            SubstrateBand::Mesic => (ratio(3_906_250_000, 1), ratio(10_485_760_000, 1)),
            // 600⁴ .. 1500⁴
            SubstrateBand::HyperThermal => {
                (ratio(129_600_000_000, 1), ratio(5_062_500_000_000, 1))
            }
        }
    }
}

/// The falloff length for [`coverage`], in `K⁴`.
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

/// COVERAGE — how well a world at `t_eq4` supports substrate, as an
/// exact rational in `(0, 1]`, DEFINED EVERYWHERE on the positive line.
///
/// **This replaces a wall with a slope.** [`band_of`] answers whether a
/// world falls inside one of three disjoint windows, so a world one
/// K⁴ outside a bound reads NOTHING while one K⁴ inside reads
/// everything — and the three windows do not cover the line, leaving
/// 320–600 K supported by nothing at all. The derived furnace seats
/// (mineral and plasmic foundings) land in that gap, which made a whole
/// class of world uninhabitable by construction rather than by design
/// (`20_FOUNDING` §4, §6c).
///
/// The bands remain, as the regions where coverage is RICH:
///
/// ```text
/// inside any band          coverage = 1        a plateau, not a peak
/// outside every band       1 / (1 + d/scale)   d = K⁴ to the nearest edge
/// ```
///
/// **Monotone in distance, strictly positive everywhere, and free of
/// squares** — the difference enters to the first power only, so THE
/// PROPORTIONAL LAW is untouched (`EQUATIONS` ch.2: squares are barred
/// as aggregators of difference; quartics survive only as physical law,
/// and `T⁴` is already the carrier here).
///
/// A gap world now reads POOR rather than IMPOSSIBLE, which is a game
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
        Some(distance) if distance > zero => {
            one.clone() / (one + distance / coverage_scale4())
        }
        _ => one,
    }
}

/// THERMAL-1, the cryo law (05 §1.2): at or below this `T⁴` the store
/// is a lossless vault — the cryogenic band's ceiling, `100⁴ K⁴`.
pub fn cryo_vault_max4() -> Ratio {
    SubstrateBand::Cryogenic.bounds4().1
}

/// Does an accrued joule store LEAK at this `T⁴`? False in the vault
/// (cryo band and colder), true everywhere warmer (05 §1.2).
pub fn leaks(t_eq4: &Ratio) -> bool {
    *t_eq4 > cryo_vault_max4()
}

/// The band a `T_eq⁴` falls in, or `None` when it sits between/outside the
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

/// The enumerator of known stars (NS-6 §7): each class fixes its sun's
/// luminosity `L` (relative to Sol = 1), which drives every bound body's
/// `T_eq⁴`. Discriminants are the PERMANENT wire codes (NS-6 §7).
/// Values are placeholder parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StarClass {
    // Main sequence (Morgan–Keenan), hottest to coolest.
    O = 1,
    B = 2,
    A = 3,
    F = 4,
    G = 5,
    K = 6,
    M = 7,
    // Remnants, giants, and the lore exotic.
    BrownDwarf = 8,
    WhiteDwarf = 9,
    RedGiant = 10,
    NeutronStar = 11,
    WolfRayet = 12,
    LuminalWhiteHole = 13,
}

impl StarClass {
    pub fn from_wire(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::O),
            2 => Some(Self::B),
            3 => Some(Self::A),
            4 => Some(Self::F),
            5 => Some(Self::G),
            6 => Some(Self::K),
            7 => Some(Self::M),
            8 => Some(Self::BrownDwarf),
            9 => Some(Self::WhiteDwarf),
            10 => Some(Self::RedGiant),
            11 => Some(Self::NeutronStar),
            12 => Some(Self::WolfRayet),
            13 => Some(Self::LuminalWhiteHole),
            _ => None,
        }
    }

    /// Luminosity relative to Sol (exact rational placeholder).
    pub fn luminosity(self) -> Ratio {
        match self {
            StarClass::O => ratio(100_000, 1),
            StarClass::B => ratio(20_000, 1),
            StarClass::A => ratio(20, 1),
            StarClass::F => ratio(3, 1),
            StarClass::G => ratio(1, 1),
            StarClass::K => ratio(3, 10),
            StarClass::M => ratio(1, 100),
            StarClass::BrownDwarf => ratio(1, 10_000),
            StarClass::WhiteDwarf => ratio(1, 100),
            StarClass::RedGiant => ratio(1_000, 1),
            StarClass::NeutronStar => ratio(100, 1),
            StarClass::WolfRayet => ratio(200_000, 1),
            StarClass::LuminalWhiteHole => ratio(1_000_000, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(n: i64, d: i64) -> Ratio {
        ratio(n, d)
    }

    #[test]
    fn sol_type_body_sits_mesic_at_exactly_279k_fourth() {
        // G star, L=1, albedo 0, r=1 → T_eq⁴ == K == 279⁴, the mesic anchor.
        let t = t_eq4(&StarClass::G.luminosity(), &r(0, 1), &r(1, 1));
        assert_eq!(t, r(6_059_221_281, 1));
        assert_eq!(band_of(&t), Some(SubstrateBand::Mesic));
    }

    #[test]
    fn albedo_cools_by_exactly_one_minus_a() {
        let full = t_eq4(&r(1, 1), &r(0, 1), &r(1, 1));
        let half = t_eq4(&r(1, 1), &r(1, 2), &r(1, 1));
        assert_eq!(half, full / r(2, 1)); // exact halving, no rounding
    }

    #[test]
    fn inverse_square_in_orbit_radius() {
        let near = t_eq4(&r(1, 1), &r(0, 1), &r(1, 1));
        let far = t_eq4(&r(1, 1), &r(0, 1), &r(2, 1));
        assert_eq!(far, near / r(4, 1)); // r doubles ⇒ flux quarters, exact
    }

    #[test]
    fn red_dwarf_at_one_unit_is_cold() {
        // M star (L=1/100) at r=1 → cryogenic, not mesic: the band hugs the
        // star, so an Earth-distance body is frozen (05 §2.1, §3.3).
        let t = t_eq4(&StarClass::M.luminosity(), &r(0, 1), &r(1, 1));
        assert_eq!(band_of(&t), Some(SubstrateBand::Cryogenic));
    }

    #[test]
    fn blue_o_star_at_one_unit_exceeds_every_band() {
        // Too hot for any substrate at r=1 — the equilibrium band is pushed
        // far outward (05 §3.1). Farther out is strictly cooler.
        let hot = t_eq4(&StarClass::O.luminosity(), &r(0, 1), &r(1, 1));
        assert_eq!(band_of(&hot), None);
        let far = t_eq4(&StarClass::O.luminosity(), &r(0, 1), &r(100, 1));
        assert!(far < hot);
    }

    #[test]
    fn band_boundaries_are_exact_and_inclusive() {
        assert_eq!(band_of(&r(3_906_250_000, 1)), Some(SubstrateBand::Mesic)); // 250⁴ lo
        assert_eq!(band_of(&r(10_485_760_000, 1)), Some(SubstrateBand::Mesic)); // 320⁴ hi
        assert_eq!(band_of(&r(10_485_760_001, 1)), None); // one K⁴ above ⇒ no band
    }

    #[test]
    fn main_sequence_luminosity_is_monotonic() {
        let seq = [
            StarClass::O,
            StarClass::B,
            StarClass::A,
            StarClass::F,
            StarClass::G,
            StarClass::K,
            StarClass::M,
        ];
        for pair in seq.windows(2) {
            assert!(pair[0].luminosity() > pair[1].luminosity());
        }
    }
}
