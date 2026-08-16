//! The thermal core (05_THERMAL_BALANCE.md §1–§3): the exact-rational
//! equilibrium baseline and substrate-band membership.
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
use num_traits::ToPrimitive;

/// The fourth power of a CITED kelvin anchor, exact — the ONLY way a
/// band bound or the folded constant enters this module (P2, operator
/// ruling 2026-08-05): the parameter is a [`crate::anchor::KelvinAnchor`]
/// — a typed declaration carrying its empirical citation — so a bare
/// number cannot become a thermal bound, and a flattened quartic
/// cannot drift from the anchor it encodes.
fn kelvin4(anchor: crate::anchor::KelvinAnchor) -> Ratio {
    let kelvin = anchor.kelvin();
    ratio(kelvin * kelvin * kelvin * kelvin, 1)
}

/// The folded thermal constant `K = 1/(16πσ)` as one exact rational:
/// the fourth power of the Sol-equilibrium anchor, so
/// `T_eq⁴(L=1, A=0, r=1) = K` lands mesic by calibration.
pub fn thermal_k() -> Ratio {
    kelvin4(crate::anchor::KelvinAnchor::SolEquilibrium)
}

/// `T_eq⁴ = L (1 − A) K / r²` — the equilibrium baseline in flux space
/// (05 §1.1). Exact for rational inputs; the root is never taken.
pub fn t_eq4(luminosity: &Ratio, albedo: &Ratio, orbit_r: &Ratio) -> Ratio {
    insolation(luminosity, albedo, orbit_r) * thermal_k()
}

/// **THE CONTINUOUS INSOLATION** `I = L·(1−A)/r²` — the absorbed stellar
/// flux per unit area, an EXACT rational (COG Slice 2b). This is the
/// physical vector the 8-way [`ThermalRegion`](crate::almanac::ThermalRegion)
/// buckets discard: `t_eq4` collapses it into a temperature and bins it, but
/// primary production reads it NATIVELY, so a bright or close star delivers
/// more food than a dim or distant one — continuously, not in temperature
/// steps. `L` is the star's own luminosity ([`StarClass::luminosity`]), so
/// the star class injects its exact energy output straight into the food
/// economy; the temperature bucket becomes an efficiency term (η), never the
/// magnitude of the light.
pub fn insolation(luminosity: &Ratio, albedo: &Ratio, orbit_r: &Ratio) -> Ratio {
    let one_minus_a = ratio(1, 1) - albedo.clone();
    let r2 = orbit_r.clone() * orbit_r.clone();
    luminosity.clone() * one_minus_a / r2
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
    /// The inclusive `[lo⁴, hi⁴]` bounds in K⁴ — DERIVED from the
    /// CITED kelvin anchors, never stored flattened (the anchors are
    /// the parameters; the quartics cannot drift from them).
    pub fn bounds4(self) -> (Ratio, Ratio) {
        use crate::anchor::KelvinAnchor as At;
        match self {
            SubstrateBand::Cryogenic => (kelvin4(At::CryoFloor), kelvin4(At::CryoCeiling)),
            SubstrateBand::Mesic => (kelvin4(At::MesicFloor), kelvin4(At::MesicCeiling)),
            SubstrateBand::HyperThermal => {
                (kelvin4(At::FoundryFloor), kelvin4(At::FoundryCeiling))
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

    /// **MAGNETISM — the field grade of this class** (operator ruling
    /// 2026-08-06: *how many planets can orbit a star should be based
    /// on the magnetism of the star associated with the solar system*).
    ///
    /// Derived from the DYNAMO, the way the abundance spine wears
    /// metallicity: a star's field comes from its convective dynamo,
    /// and convection deepens down the main sequence — O/B/A are
    /// radiative (weak fossil fields), F begins shallow convection,
    /// G is the solar dynamo, K runs deeper, M is FULLY convective and
    /// carries the strongest main-sequence fields (the packed systems
    /// are the measured case). Remnants: the white dwarf keeps a fossil
    /// megagauss field; the NEUTRON STAR is the strongest field in
    /// nature but COMPACT — a tight magnetosphere sweeps clean, so its
    /// grade is anchored to the measured case (three planets at
    /// PSR B1257+12, the first exoplanets ever found); Wolf–Rayet
    /// strips its own disk with wind. Grades are placeholder dials
    /// ruled at balance, like every figure in this table.
    pub fn magnetism(self) -> u8 {
        match self {
            StarClass::O => 2,
            StarClass::B => 3,
            StarClass::A => 3,
            StarClass::F => 5,
            StarClass::G => 8,
            StarClass::K => 9,
            StarClass::M => 12,
            StarClass::BrownDwarf => 10,
            StarClass::WhiteDwarf => 6,
            StarClass::RedGiant => 4,
            StarClass::NeutronStar => 3,
            StarClass::WolfRayet => 2,
            StarClass::LuminalWhiteHole => 12,
        }
    }

    /// **THE RING REACH — how many RINGS this star's field anchors.**
    /// The law the ruling asked for: planet capacity is BASED ON the
    /// magnetism, and at v1 the derivation is identity — one ring per
    /// field grade. G reaches EIGHT, which lands the home calibration
    /// exactly: Sol's dynamo holds Sol's count. A ring is a distinct
    /// `orbit_r`; slots WITHIN a ring stay the turn law's business
    /// (60 × 60 per ring, ψ separating) — magnetism bounds how many
    /// rings exist, never how a ring fills.
    pub fn ring_reach(self) -> u8 {
        self.magnetism()
    }

    /// Luminosity relative to Sol — the CITED anchor table
    /// (`crate::anchor::luminosity`, operator ruling 2026-08-05) for
    /// every astronomical class; the lore class alone keeps a DIAL
    /// here, because fiction has no empirical citation and the census
    /// counts it honestly.
    pub fn luminosity(self) -> Ratio {
        crate::anchor::luminosity(self)
            .unwrap_or_else(|| match self {
                StarClass::LuminalWhiteHole => ratio(1_000_000, 1),
                anchored => unreachable!("{anchored:?} is anchored"),
            })
    }

    /// The star's MASS in solar masses (`M☉ = 1`) — the anchor Kepler's third
    /// law reads. The lore White Hole has no citation, so it takes the solar
    /// baseline (trivial, no dial).
    pub fn mass(self) -> Ratio {
        crate::anchor::mass(self).unwrap_or_else(|| match self {
            StarClass::LuminalWhiteHole => ratio(1, 1),
            anchored => unreachable!("{anchored:?} is anchored"),
        })
    }
}

/// The base orbital period: a reference G-type world at `r = 1` completes its
/// 360° circuit in this many universal beats — one beat per degree of arc, the
/// Base-60/360 tie-in. Every other period scales off it by Kepler's law.
const BASE_ORBIT_BEATS: i64 = 360; // the beat↔degree definition (360° circuit)

/// **THE KEPLERIAN PERIOD, SQUARED** (the Keplerian clock) — Kepler's third law
/// `P² = (360)²·r³/M` in exact rationals (`r` in AU, `M` in `M☉`, `P` in beats).
/// SQUARED because the period itself, `√(r³/M)`, is irrational and a ℚ-engine
/// cannot hold it — but the SEMANTIC quantity, the count of completed orbits, is
/// exact from `P²` alone (see [`local_years`]). A close orbit or a heavy star
/// gives a small `P²` (a short year); a distant orbit or a light star a large one.
pub fn orbital_period_squared(orbit_r: &Ratio, mass: &Ratio) -> Ratio {
    let base = ratio(BASE_ORBIT_BEATS, 1);
    let r3 = orbit_r.clone() * orbit_r.clone() * orbit_r.clone();
    (base.clone() * base) * r3 / mass.clone()
}

/// **THE ORBITAL-PLANE NORMAL** (the galactic frame) — each solar system tilts
/// its own way relative to the galactic plane, so planets are NOT all drawn on
/// one flat sheet. The galactic position of a system is the S of the universal
/// `{S, G, T}` (a 3-vector already), and this normal `N` gives its orbital track
/// a 3D inclination off galactic-up `[0, 1, 0]`. Derived EXACTLY from the
/// system's founding digest — no stored state, deterministic, replayable: the
/// tilts are small signed rationals in `(-1/2, 1/2)` from disjoint digest bytes,
/// so most systems sit near the plane and some lean hard. The tilt shapes the
/// LOCAL orbit (insolation geometry, and seasons if ever); at galactic scale a
/// planet's orbital displacement (~AU) is sub-resolution against its star's
/// position (~parsecs), so a planet's `{S, G, T}` IS its star's.
pub fn orbital_normal(seed: [u8; 32]) -> (Ratio, Ratio, Ratio) {
    // A signed tilt in (-1/2, 1/2): byte 128 is flat, the extremes lean ±1/2.
    let tilt = |byte: u8| ratio(byte as i64 - 128, 256);
    (tilt(seed[0]), ratio(1, 1), tilt(seed[16]))
}

/// The EXACT rational square root of `x`, or `None` when `x` is not a perfect
/// rational square. `√(p/q) = √p/√q` is exact iff both the numerator and
/// denominator (of the reduced fraction) are perfect integer squares — the
/// `BigRational` is always reduced, so this test is total and exact. This is
/// the DIOPHANTINE window: a track sited by `siting.rs` at `r = M·k²` has
/// `P² = BASE²·M²·k⁶`, a perfect rational square, so its period `P` is an
/// EXACT rational and no root is ever approximated.
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

/// **THE LOCAL YEAR** (the Keplerian clock, DIO-1) — how many complete 360°
/// orbits a planet has traced in `beats` universal ticks: `⌊beats / P⌋`.
///
/// **The last irrational is gone.** A Diophantine track (`siting.rs`,
/// `r = M·k²`, `P = M·k³·BASE`) has a PERFECT-SQUARE `P²`, so `P` is an exact
/// rational and the year is a pure rational floor `⌊beats / P⌋` — the exact
/// angular step `Δθ = 360/P` walked `beats` times, no integer square root. For
/// any OTHER `r` (an operator-pinned off-track world) `P` may be irrational;
/// there we fall back to the exact integer-sqrt floor `⌊√(beats²/P²)⌋`, which
/// is the same value computed without forming the root. The beat is the
/// absolute cosmic tick; the year is local geometry — a tight inner world
/// rolls its calendar often, a distant one seldom, on the very same beat.
pub fn local_years(orbit_r: &Ratio, mass: &Ratio, beats: u64) -> u64 {
    let p2 = orbital_period_squared(orbit_r, mass);
    if p2 <= ratio(0, 1) {
        return 0;
    }
    // THE DIOPHANTINE PATH: P² is a perfect rational square ⇒ P is exact ⇒
    // the year is a direct rational floor, no isqrt.
    if let Some(period) = exact_sqrt(&p2) {
        let years = (ratio(beats as i64, 1) / period).floor();
        return years.to_integer().to_u128().unwrap_or(u128::MAX as u128) as u64;
    }
    // THE FALLBACK (off-track r): the exact integer-sqrt floor of beats²/P².
    let beats2 = ratio(beats as i64, 1) * ratio(beats as i64, 1);
    let x = (beats2 / p2).floor().to_integer().to_u128().unwrap_or(u128::MAX);
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
        // Massive hot stars burn heavy; dim red dwarfs are tiny.
        assert!(StarClass::O.mass() > StarClass::G.mass(), "O outweighs the Sun");
        assert_eq!(StarClass::G.mass(), r(1, 1), "G is the solar unit");
        assert!(StarClass::G.mass() > StarClass::M.mass(), "the Sun outweighs a red dwarf");
        assert_eq!(StarClass::M.mass(), r(1, 5), "M is the tiny red dwarf");
    }

    #[test]
    fn each_system_tilts_its_own_orbital_plane() {
        use crate::thermal::orbital_normal;
        // The median digest sits flat on the galactic plane: N = [0, 1, 0].
        let mut flat = [128u8; 32];
        assert_eq!(orbital_normal(flat), (r(0, 1), r(1, 1), r(0, 1)), "the median digest is flat");
        // Galactic-up is never lost — the plane leans, the Y reference holds.
        let mut leaning = [128u8; 32];
        leaning[0] = 255;
        let n = orbital_normal(leaning);
        assert_eq!(n.1, r(1, 1), "galactic-up (Y) holds; the lean is in X/Z");
        assert!(n.0 > r(0, 1), "a high digest byte leans the plane one way");
        // Distinct systems tilt distinctly (deterministic per digest).
        flat[16] = 40;
        assert_ne!(orbital_normal(flat), orbital_normal([128u8; 32]), "distinct digests, distinct tilt");
        assert_eq!(orbital_normal([200u8; 32]), orbital_normal([200u8; 32]), "same digest → same tilt (replayable)");
    }

    #[test]
    fn the_keplerian_clock_ages_planets_by_their_own_geometry() {
        use crate::thermal::{local_years, orbital_period_squared};
        let g = StarClass::G.mass();
        // Kepler: a distant orbit is a LONGER year than a close one (same star).
        let close = orbital_period_squared(&r(1, 2), &g);
        let far = orbital_period_squared(&r(4, 1), &g);
        assert!(far > close, "a distant world has a longer period");
        // A HEAVIER star pulls a SHORTER year at the same orbit.
        let round_heavy = orbital_period_squared(&r(1, 1), &StarClass::O.mass());
        let round_sol = orbital_period_squared(&r(1, 1), &g);
        assert!(round_heavy < round_sol, "a heavier star shortens the year");
        // The reference G world at r=1 completes its 360-beat circuit exactly:
        // 0 years before beat 360, exactly 1 at beat 360, 2 at 720.
        assert_eq!(local_years(&r(1, 1), &g, 359), 0, "not yet a full orbit");
        assert_eq!(local_years(&r(1, 1), &g, 360), 1, "one 360-beat circuit = one local year");
        assert_eq!(local_years(&r(1, 1), &g, 720), 2, "two circuits, two years");
        // Two planets, SAME beat, DIFFERENT ages: the close-and-heavy world has
        // lived far more local years than a distant one on the same cosmic tick.
        let beat = 3600;
        let inner = local_years(&r(1, 4), &StarClass::O.mass(), beat);
        let outer = local_years(&r(9, 1), &StarClass::M.mass(), beat);
        assert!(inner > outer, "same beat, but the inner-heavy world is far older: {inner} vs {outer}");
    }

    #[test]
    fn the_diophantine_clock_is_an_exact_rational_floor() {
        use crate::thermal::{exact_sqrt, local_years, orbital_period_squared};
        let g = StarClass::G.mass();
        // A Diophantine track r = M·k² has a PERFECT-SQUARE P², so the period
        // is an EXACT rational and the year is a pure floor ⌊T/P⌋ — no isqrt.
        let k = r(3, 2); // an off-lattice continuous k
        let rr = g.clone() * k.clone() * k.clone(); // r = M·k²
        let p2 = orbital_period_squared(&rr, &g);
        let period = exact_sqrt(&p2).expect("a Diophantine track has an exact rational period");
        // P = BASE·M·k³ exactly (BASE = 360, M = 1): 360·(3/2)³ = 360·27/8.
        assert_eq!(period, r(360, 1) * k.clone() * k.clone() * k, "P = BASE·M·k³ exact");
        // The year count is the exact rational floor ⌊beats/P⌋.
        let p_int = period.to_integer().to_u128().unwrap() as u64; // P here is integral
        assert_eq!(local_years(&rr, &g, p_int), 1, "one exact period = one year");
        assert_eq!(local_years(&rr, &g, p_int - 1), 0, "not yet a full period");
        assert_eq!(local_years(&rr, &g, 3 * p_int), 3, "three periods, three years");
        // A NON-Diophantine r (P² not a perfect square) has no exact period —
        // the isqrt fallback still answers, and agrees at the reference world.
        assert_eq!(exact_sqrt(&orbital_period_squared(&r(2, 1), &g)), None, "r=2 is off-track");
        assert_eq!(local_years(&r(1, 1), &g, 720), 2, "the reference world still ages exactly");
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
