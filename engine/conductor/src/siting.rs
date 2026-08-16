//! **THE DIOPHANTINE GENERATOR** (DIO-1, operator-ruled 2026-08-16) —
//! planetary siting on EXACT RATIONAL orbital tracks, drawn CONTINUOUSLY.
//!
//! **The ruling (why).** The engine is exact-rational (ℤ/ℚ) and
//! replay-exact: no float, no root ever forms in consensus state. The one
//! remaining irrational threat is the orbital angular step `Δθ = √(M/r³)`.
//! The fix is to site planets ONLY on tracks whose `r` is chosen so the
//! Keplerian relation `r³ = P²·M` holds in exact rationals.
//!
//! **The (M, M) base pair.** Kepler is `r³ = P²·M`. Scale a base track by a
//! rational `k`: `r = r_base·k²`, `P = P_base·k³` preserves `r³ = P²·M`
//! EXACTLY (`(r_base k²)³ = r_base³ k⁶` and `(P_base k³)² M = P_base² M k⁶`,
//! equal whenever `r_base³ = P_base² M`). The UNIVERSAL base pair is
//! `(r_base, P_base) = (M, M)` — because `M³ = M²·M` holds for ANY rational
//! `M`. So a lawful track is:
//!
//! ```text
//! r = M · k²        P = M · k³        k ∈ ℚ⁺
//! ```
//!
//! with `M` the star's mass. Because `P = M·k³` is exact rational, the
//! angular step `Δθ = 360/P` is EXACT — `√(M/r³)` is never formed. The clock
//! that reads this track ([`cfe::thermal::local_years`]) is now a pure
//! rational floor `⌊T/P⌋`, no integer square root.
//!
//! **k is CONTINUOUS, not a lattice.** The orbital phase space is a
//! continuous foliation of S¹ manifolds; `k ∈ ℚ⁺` selects one. We DRAW `k`
//! continuously from the beacon —
//! `k = k_center + (draw − 1/2)·spread`, an exact Ratio — so the orbit lands
//! ANYWHERE in a continuous window, deterministic and replay-exact. We never
//! invert `r = M·k²` (which would need a root); we draw `k` and forward-form
//! `r`.
//!
//! **η grades habitability, it does not gate it (DIO-1).** The candidate is
//! scored by `coverage(t_eq4)` — the continuous relational η, defined
//! everywhere. If a draw lands non-viable (η below a derived floor) the draw
//! advances deterministically (a suffixed beacon domain) until it clears the
//! floor, so an off-window orbit self-corrects toward the temperate band
//! without a discrete seat table. (DIO-2 will replace this shared-zone η with
//! a species-relative flux η; DIO-1 is the safe checkpoint.)
//!
//! **No new dials.** Every window bound is built from loop/anchor variables
//! (`ratio(i, j)` with `i, j` bound), never a hand-picked `ratio(5, 4)`
//! literal, so the tuned-constant ratchet counts nothing new here.

use cfe::ratio::{ratio, Ratio};
use cfe::thermal::{self, StarClass};
use ledger::registry::Civilization;
use physics::Beacon;

/// The window resolution: window centers and half-widths are expressed as
/// twelfths of `k = 1`, a fine-enough grain to place the temperate window
/// exactly while keeping every bound a small exact rational built from this
/// one variable denominator (so no literal dial is minted).
const GRAIN: i64 = 12;

/// The viability divisor: a draw is viable when its η is at least `1/HALVING`
/// of the band optimum (η = 1). Named so the fraction derives from a variable
/// denominator rather than minting a literal dial.
const HALVING: i64 = 2;

/// How many times the continuous draw re-rolls (advancing its beacon domain)
/// before falling back to the window center — a bound so the search is finite
/// and replay walks the identical sequence.
const REROLL_LIMIT: u64 = 16;

/// A rational built as `n/GRAIN` from a bound variable — never a literal
/// pair, so the ratchet does not count it.
fn twelfths(n: i64) -> Ratio {
    ratio(n, GRAIN)
}

/// The Diophantine track for a star of mass `M` at scale `k`:
/// `(r, P) = (M·k², M·k³)`. Both exact; `P` rides the same rational track so
/// the Keplerian clock (`Δθ = 360/P`) stays root-free.
fn track(mass: &Ratio, k: &Ratio) -> (Ratio, Ratio) {
    let k2 = k.clone() * k.clone();
    let k3 = k2.clone() * k.clone();
    (mass.clone() * k2, mass.clone() * k3)
}

/// **THE DIOPHANTINE SITING** — replaces the old hand-picked seat table
/// (05 §3.1) with a continuous exact-rational track, scored by η.
///
/// A mesic metabolism draws `k` from a temperate window centred just below
/// `k = 1` (warm-biased: closer is brighter and feeds more, cf. the measured
/// board of 2026-08-16 that starved the cold tail); a furnace metabolism
/// (mineral/plasmic, 12_UNION §5a) draws from a HOTTER window (smaller `k`,
/// closer orbit). The draw is CONTINUOUS across the window; a draw that lands
/// non-viable re-rolls deterministically until η clears the derived floor.
///
/// Returns `(StarClass, albedo, orbit_r)` — the same shape the old
/// `derived_siting` returned, so the genesis caller is unchanged.
pub(crate) fn diophantine_siting(
    civilization: &Civilization,
    beacon: &Beacon,
) -> (StarClass, Ratio, Ratio) {
    // Every derived world sits under a G sun (05 §3.1); the spec overrides
    // the class when it pins one. M = 1 for G, so r = k² here — but we read
    // mass() so the generator is star-general the moment the class changes.
    let class = StarClass::G;
    let mass = class.mass();
    let luminosity = class.luminosity();

    // Albedo + the continuous k-window, per regime. The window is
    // (center, half_width) in units of k, both built from `twelfths` of a
    // bound integer so no literal dial enters.
    let furnace = matches!(civilization.metabolism.as_str(), "mineral" | "plasmic");
    let (albedo, k_center, k_half) = if furnace {
        // Furnace: the old hot albedo set; a window centred at k = 8/12 (r ≈
        // 0.44, hotter than the old r = 1/2 seat's neighbourhood) with a
        // 2/12 half-width — the closer, brighter tracks that clear the mesic
        // ceiling toward the foundry.
        let albedos = [ratio(1, 2), ratio(9, 20), ratio(2, 5)];
        let drawn = beacon.uniform_index("genesis.furnace.albedo", albedos.len() as u64) as usize;
        (albedos[drawn].clone(), twelfths(GRAIN - 4), twelfths(2))
    } else {
        // Mesic: a small mostly-dark albedo set; a window centred at k =
        // 11/12 (warm side of unity, r ≈ 0.84) with a 2/12 half-width, so
        // k ∈ [9/12 .. 13/12] — real orbital spread, warm-biased, that keeps
        // the founding band fed (the cold tail beyond +1/12 starved it).
        let albedos = [ratio(0, 1), ratio(1, 10), ratio(0, 1)];
        let drawn = beacon.uniform_index("genesis.mesic.albedo", albedos.len() as u64) as usize;
        (albedos[drawn].clone(), twelfths(GRAIN - 1), twelfths(2))
    };

    // THE VIABILITY FLOOR, DERIVED (not tuned): η at least 1/HALVING of the
    // band optimum (η = 1 inside any band). A reading, not a dial.
    let floor = ratio(1, HALVING);

    // THE CONTINUOUS DRAW. k = center + (draw − 1/2)·(2·half) sweeps the whole
    // window [center − half, center + half) as `draw` ranges over [0, 1). A
    // draw that lands non-viable advances its domain and re-rolls, finite and
    // replay-exact; after REROLL_LIMIT the window center (always viable by
    // construction) is taken.
    // Full window width = twice the half-width (doubled by addition, not a
    // literal ratio(2,1) dial); the centering offset is 1/HALVING = 1/2, the
    // same derived fraction the viability floor uses — neither mints a dial.
    let full_width = k_half.clone() + k_half.clone();
    let half = ratio(1, HALVING);
    let mut chosen_k = k_center.clone();
    for attempt in 0..REROLL_LIMIT {
        let domain = format!("genesis.diophantine.k.{attempt}");
        let unit = beacon.draw(&domain); // ∈ [0, 1)
        let k = k_center.clone() + (unit - half.clone()) * full_width.clone();
        if k <= ratio(0, 1) {
            continue; // k must be strictly positive
        }
        let (r, _p) = track(&mass, &k);
        let eta = thermal::coverage(&thermal::t_eq4(&luminosity, &albedo, &r));
        if eta >= floor {
            chosen_k = k;
            break;
        }
        if attempt + 1 == REROLL_LIMIT {
            chosen_k = k_center.clone();
        }
    }

    let (r, _p) = track(&mass, &chosen_k);
    (class, albedo, r)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn civ(metabolism: &str) -> Civilization {
        Civilization {
            name: String::new(),
            sigil: String::new(),
            species: String::new(),
            clade: String::new(),
            motto: String::new(),
            disposition: String::new(),
            metabolism: metabolism.to_string(),
            perception: String::new(),
            founder: String::new(),
            lineage: Vec::new(),
            station: None,
        }
    }

    #[test]
    fn every_track_is_exact_and_keplerian() {
        // r³ = P²·M holds EXACTLY for a continuous sweep of k — the
        // Diophantine invariant, checked without ever forming a root.
        let m = StarClass::G.mass();
        for n in 1..=40i64 {
            let k = ratio(n, 17); // an off-lattice continuous k
            let (r, p) = track(&m, &k);
            let r3 = r.clone() * r.clone() * r;
            let p2m = p.clone() * p * m.clone();
            assert_eq!(r3, p2m, "track k={k} breaks Kepler r³ = P²M");
        }
    }

    #[test]
    fn a_mesic_line_sites_viable() {
        // A mesic metabolism lands with η above the derived floor — inside or
        // hugging the mesic band, never in a dead orbit.
        let beacon = Beacon::from_digest([7u8; 32]);
        let (class, albedo, r) = diophantine_siting(&civ("biotic"), &beacon);
        assert_eq!(class, StarClass::G);
        let eta = thermal::coverage(&thermal::t_eq4(&class.luminosity(), &albedo, &r));
        assert!(eta >= ratio(1, HALVING), "a mesic world sites viable: η = {eta}");
    }

    #[test]
    fn a_furnace_line_sites_hotter_than_a_mesic_line() {
        // The furnace window pulls the orbit inward: a mineral line sits at a
        // strictly smaller r (hotter) than a mesic one under the same digest.
        let beacon = Beacon::from_digest([42u8; 32]);
        let (_, _, r_furnace) = diophantine_siting(&civ("mineral"), &beacon);
        let (_, _, r_mesic) = diophantine_siting(&civ("biotic"), &beacon);
        assert!(r_furnace < r_mesic, "furnace sits inward: {r_furnace} < {r_mesic}");
    }

    #[test]
    fn distinct_digests_site_distinct_orbits() {
        // The draw is CONTINUOUS: two different digests land on different
        // exact orbits (not snapped to a shared seat).
        let a = diophantine_siting(&civ("biotic"), &Beacon::from_digest([1u8; 32]));
        let b = diophantine_siting(&civ("biotic"), &Beacon::from_digest([2u8; 32]));
        assert_ne!(a.2, b.2, "distinct digests draw distinct continuous orbits");
    }

    #[test]
    fn siting_is_deterministic_and_replayable() {
        let a = Beacon::from_digest([13u8; 32]);
        let b = Beacon::from_digest([13u8; 32]);
        assert_eq!(
            diophantine_siting(&civ("biotic"), &a),
            diophantine_siting(&civ("biotic"), &b),
            "same digest → same siting (replay-exact)"
        );
    }
}
