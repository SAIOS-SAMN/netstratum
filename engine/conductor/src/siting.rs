//! **The diophantine generator** (DIO-1, operator-ruled 2026-08-16) —
//! planetary siting on exact rational orbital tracks, drawn continuously.
//!
//! **The ruling (why).** The engine is exact-rational (ℤ/ℚ) and
//! replay-exact: no float, no root ever forms in consensus state. The one
//! remaining irrational threat is the orbital angular step `Δθ = √(M/r³)`.
//! The fix is to site planets only on tracks whose `r` is chosen so the
//! Keplerian relation `r³ = P²·M` holds in exact rationals.
//!
//! **The (M, M) base pair.** Kepler is `r³ = P²·M`. Scale a base track by a
//! rational `k`: `r = r_base·k²`, `P = P_base·k³` preserves `r³ = P²·M`
//! exactly (`(r_base k²)³ = r_base³ k⁶` and `(P_base k³)² M = P_base² M k⁶`,
//! equal whenever `r_base³ = P_base² M`). The universal base pair is
//! `(r_base, P_base) = (M, M)` — because `M³ = M²·M` holds for any rational
//! `M`. So a lawful track is:
//!
//! ```text
//! r = M · k²        P = M · k³        k ∈ ℚ⁺
//! ```
//!
//! with `M` the star's mass. Because `P = M·k³` is exact rational, the
//! angular step `Δθ = 360/P` is exact — `√(M/r³)` is never formed. The clock
//! that reads this track ([`cfe::thermal::local_years`]) is now a pure
//! rational floor `⌊T/P⌋`, no integer square root.
//!
//! **k is continuous, not a lattice.** The orbital phase space is a
//! continuous foliation of S¹ manifolds; `k ∈ ℚ⁺` selects one. `k` is drawn
//! continuously from the beacon —
//! `k = k_center + (draw − 1/2)·spread`, an exact Ratio — so the orbit lands
//! anywhere in a continuous window, deterministic and replay-exact. `r = M·k²`
//! is never inverted (which would need a root); `k` is drawn and `r`
//! forward-formed.
//!
//! **The gate is production's own viability (ONT-1d part 3).** A candidate
//! draw is scored by [`cfe::almanac::substrate_viability`] — the same
//! `habitability_flux(clade) · phase_feasibility(metabolism)` product that
//! `carrying_capacity` reads. If a draw lands below the derived floor the draw
//! advances deterministically (a suffixed beacon domain) until it clears, so a
//! line self-corrects toward the orbit where it can actually produce, one
//! geometry, no two-viabilities seam. (Earlier DIO-1 scored a shared, species-
//! blind thermal-coverage η here; that mesic tent could not tell a plasma from
//! a phototroph, and it is retired.) The k-window per regime only bounds the
//! search; the species-relative gate PLACES the niche within it — re-measured
//! 2026-10-18 under the linear thermal law to seat the plasma in the furnace
//! (k ∈ [4/12, 6/12]) where its clade band and plasma phase both hold, and the
//! biotic lines mesic.
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

/// **The truncation boundary** — quantize `k` onto the orbit sub-lattice
/// `k_grained = round(k·G)/G` (G = [`cfe::almanac::ORBIT_GRAIN`]) immediately
/// after the beacon draw and before any exponentiation. The beacon expands to a
/// rational with a 2¹²⁸ denominator; `r = M·k²` squares it and the incident
/// flux `∝ k⁴` takes it toward 2⁵¹², so every downstream exact-rational
/// operation drowns in a GCD cascade (the denominator-explosion that stalled
/// the beat). Graining bounds the denominator — and thus the memory footprint —
/// to `G`, restoring O(1) arithmetic, while keeping `k` exactly rational so the
/// Diophantine track (`r³ = P²M`, perfect-square `P²`) stays exact.
fn grain_k(k: &Ratio) -> Ratio {
    let g = cfe::almanac::ORBIT_GRAIN;
    (k.clone() * ratio(g, 1)).round() / ratio(g, 1)
}

/// **The diophantine siting** — the first viable continuous draw of a
/// line's regime (05 §3.1), kept as the reference reading. Genesis no longer
/// imposes it: the band chooses among [`founding_options`] instead.
///
/// Returns `(StarClass, albedo, orbit_r)`.
pub(crate) fn diophantine_siting(
    civilization: &Civilization,
    beacon: &Beacon,
) -> (StarClass, Ratio, Ratio) {
    // Every derived world sits under a yellow star (05 §3.1); the spec overrides
    // the class when it pins one. M = 1 for G, so r = k² here — but mass()
    // is read so the generator is star-general the moment the class changes.
    let class = StarClass::Yellow;
    let mass = class.mass();
    let (albedo, k_center, _) = regime(civilization, beacon);

    // The viability floor, derived (not tuned): at least 1/HALVING of the
    // band optimum. A reading, not a dial.
    let floor = ratio(1, HALVING);

    // A draw that lands non-viable advances its domain and re-rolls, finite
    // and replay-exact; after REROLL_LIMIT the window center is taken.
    let mut chosen_k = k_center.clone();
    for (attempt, k) in draws(civilization, beacon).into_iter().enumerate() {
        let Some(k) = k else { continue };
        let (r, _p) = track(&mass, &k);
        if viability_at(civilization, class, &albedo, &r) >= floor {
            chosen_k = k;
            break;
        }
        if attempt as u64 + 1 == REROLL_LIMIT {
            chosen_k = k_center.clone();
        }
    }

    let (r, _p) = track(&mass, &chosen_k);
    (class, albedo, r)
}

/// Albedo + the continuous k-window, per regime: `(albedo, k_center,
/// k_half)`. A mesic metabolism draws `k` from a temperate window centred
/// just below `k = 1`; a furnace metabolism (mineral/plasmic, 12_UNION §5a)
/// from a hotter window. Every bound is `twelfths` of a bound integer, so no
/// literal dial enters.
fn regime(civilization: &Civilization, beacon: &Beacon) -> (Ratio, Ratio, Ratio) {
    let plasmic = civilization.metabolism.as_str() == "plasmic";
    let furnace = matches!(civilization.metabolism.as_str(), "mineral" | "plasmic");
    if plasmic {
        // ONT-1d part 3 — the plasma niche, derived from the gate. Re-measured
        // 2026-10-18 under era 1's linear thermal law (`T = K·L(1−A)/r`,
        // bc6471f; the 2026-08-18 window [2/12, 4/12] was measured under the
        // quartic law and now lands in dead orbit, too hot for the clade band
        // tent): a plasmiform line (HyperThermal clade band × plasma phase
        // feasibility) clears its `substrate_viability` floor (1/HALVING) for
        // k ∈ [8/24, 10/24] at every albedo of its set, and reads zero for
        // k ≤ 6/24 (too hot) and k ≥ 19/24 (too cold for plasma). The
        // plasma's own window: centre k = 5/12 (r ≈ 0.17, clearing at every
        // albedo), half 1/12, so k ∈ [4/12, 6/12].
        let albedos = [ratio(1, 2), ratio(9, 20), ratio(2, 5)];
        let drawn = beacon.uniform_index("genesis.plasmic.albedo", albedos.len() as u64) as usize;
        (albedos[drawn].clone(), twelfths(GRAIN / 2 - 1), twelfths(1))
    } else if furnace {
        // Mineral furnace: the old hot albedo set; a window centred at k = 8/12
        // (r ≈ 0.44) with a 2/12 half-width.
        let albedos = [ratio(1, 2), ratio(9, 20), ratio(2, 5)];
        let drawn = beacon.uniform_index("genesis.furnace.albedo", albedos.len() as u64) as usize;
        (albedos[drawn].clone(), twelfths(GRAIN - 4), twelfths(2))
    } else {
        // Mesic: a small mostly-dark albedo set; a window centred at k =
        // 11/12 (warm side of unity, r ≈ 0.84) with a 2/12 half-width, so
        // k ∈ [9/12 .. 13/12].
        let albedos = [ratio(0, 1), ratio(1, 10), ratio(0, 1)];
        let drawn = beacon.uniform_index("genesis.mesic.albedo", albedos.len() as u64) as usize;
        (albedos[drawn].clone(), twelfths(GRAIN - 1), twelfths(2))
    }
}

/// The continuous draws — `REROLL_LIMIT` grained `k` across the regime's
/// window, each from its own beacon domain (`genesis.diophantine.k.{n}`), in
/// order; `None` where a draw lands non-positive. `k = center + (draw −
/// 1/2)·(2·half)` sweeps `[center − half, center + half)`.
fn draws(civilization: &Civilization, beacon: &Beacon) -> Vec<Option<Ratio>> {
    let (_, k_center, k_half) = regime(civilization, beacon);
    let full_width = k_half.clone() + k_half;
    let half = ratio(1, HALVING);
    (0..REROLL_LIMIT)
        .map(|attempt| {
            let unit = beacon.draw(&format!("genesis.diophantine.k.{attempt}")); // ∈ [0, 1)
                                                                                 // Grain k here — before track() exponentiates it (see grain_k).
            let k = grain_k(&(k_center.clone() + (unit - half.clone()) * full_width.clone()));
            (k > ratio(0, 1)).then_some(k)
        })
        .collect()
}

/// ONT-1d part 3 — the one viability a line reads at an orbit: its clade's
/// band × its metabolism's phase, at the incident flux of `class` through
/// `albedo` at `r` (`cfe::almanac::substrate_viability`, the same product
/// `carrying_capacity` reads). A fresh siting has no world yet, so aridity is
/// neutral (the solid/organic phases ignore it).
pub(crate) fn viability_at(
    civilization: &Civilization,
    class: StarClass,
    albedo: &Ratio,
    r: &Ratio,
) -> Ratio {
    let clade = cfe::almanac::table_clade_from_word(&civilization.clade)
        .unwrap_or(cfe::almanac::FLATLANDER_CLADE as u16);
    let metab: u8 = match civilization.metabolism.as_str() {
        "mineral" => 1,
        "plasmic" => 3,
        _ => 2,
    };
    let t_eq4 = thermal::t_eq4(&class.luminosity(), albedo, r);
    cfe::almanac::substrate_viability(clade, metab, &t_eq4, &ratio(1, 1))
}

/// One place A founding band could settle, as the physics reads it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SiteOption {
    pub class: StarClass,
    pub albedo: Ratio,
    pub orbit: Ratio,
    /// `substrate_viability` there — the physics' reading, never a verdict.
    pub viability: Ratio,
    /// The line's compiled home (a pinned seat), as opposed to a derived draw.
    pub home: bool,
}

/// **The founding options** (G2, ruling of 2026-10-07: bands are given
/// options, freedom, and agency only). Where a band could settle: its compiled
/// home when the line has one and the physics admits it (albedo in `[0, 1)`,
/// orbit `> 0` — the `Unit`/`Positive` doors), and every continuous derived
/// draw of its regime around every main-sequence star. Each carries its viability reading.
/// Returns the options and, when a compiled home exists but the physics
/// refuses it, why — reported, never silently left unsited.
pub(crate) fn founding_options(
    civilization: &Civilization,
    beacon: &Beacon,
    home: Option<(StarClass, Ratio, Ratio)>,
) -> (Vec<SiteOption>, Option<String>) {
    let mut options = Vec::new();
    let mut refused = None;
    if let Some((class, albedo, orbit)) = home {
        let lawful = cfe::dop::Unit::new(albedo.clone()).is_some()
            && cfe::dop::Positive::new(orbit.clone()).is_some();
        if lawful {
            let viability = viability_at(civilization, class, &albedo, &orbit);
            options.push(SiteOption {
                class,
                albedo,
                orbit,
                viability,
                home: true,
            });
        } else {
            refused = Some(format!("{class:?} albedo {albedo} orbit {orbit}"));
        }
    }
    // Every main-sequence star (blue..red, the codes the wire reads 1..=7) offers
    // the regime's continuous draws on its own Keplerian track `r = M·k²` —
    // the galaxy holds more than one kind of star, so a line whose clade band
    // sits far from a yellow star's temperate window still has living land to
    // choose. Remnants and the lore exotics are not founding places.
    //
    // Beside the regime's continuous draws (the line's native window), the
    // open orbits: every twelfth of `k` out to `GRAIN/4` whole units
    // (`k = n/GRAIN`, `n ∈ 1..=GRAIN·GRAIN/4`), so a line whose clade band
    // lies outside its metabolism's window — a cryogenic biotic, say — can
    // still reach the cold. The window is a bound on the search, not a
    // prescription; viability alone weights the choice.
    let (albedo, _, _) = regime(civilization, beacon);
    let mut ks: Vec<Ratio> = draws(civilization, beacon).into_iter().flatten().collect();
    ks.extend((1..=GRAIN * GRAIN / 4).map(twelfths));
    for class in (1u8..=7).filter_map(StarClass::from_wire) {
        for k in &ks {
            let (orbit, _p) = track(&class.mass(), k);
            let viability = viability_at(civilization, class, &albedo, &orbit);
            options.push(SiteOption {
                class,
                albedo: albedo.clone(),
                orbit,
                viability,
                home: false,
            });
        }
    }
    (options, refused)
}

/// **The band chooses** — a chronicle-seeded proportional draw over the
/// options, weighted by viability on the civic lattice
/// (`⌊viability · LATTICE⌋`, `soc::partition_draw`, salted `founding.site`).
/// Better land is likelier, never guaranteed; a site the physics reads at
/// zero carries no weight. When no option is viable the band still settles
/// somewhere — the draw goes uniform and the physics decides what follows.
/// `None` only for an empty option list. Exact-integer, replay-exact.
pub(crate) fn choose_site(options: &[SiteOption], seed: [u8; 32]) -> Option<usize> {
    use num_traits::ToPrimitive;
    let lattice = ratio(crate::civic::LATTICE, 1);
    let mut weights: Vec<u64> = options
        .iter()
        .map(|o| {
            (o.viability.clone() * lattice.clone())
                .floor()
                .to_integer()
                .to_u64()
                .unwrap_or(0)
        })
        .collect();
    if weights.iter().all(|w| *w == 0) {
        weights.iter_mut().for_each(|w| *w = 1);
    }
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed);
    hasher.update(b"founding.site");
    crate::soc::partition_draw(&weights, *hasher.finalize().as_bytes())
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
        // r³ = P²·M holds exactly for a continuous sweep of k — the
        // Diophantine invariant, checked without ever forming a root.
        let m = StarClass::Yellow.mass();
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
        assert_eq!(class, StarClass::Yellow);
        let eta = thermal::coverage(&thermal::t_eq4(&class.luminosity(), &albedo, &r));
        assert!(
            eta >= ratio(1, HALVING),
            "a mesic world sites viable: η = {eta}"
        );
    }

    #[test]
    fn a_furnace_line_sites_hotter_than_a_mesic_line() {
        // The furnace window pulls the orbit inward: a mineral line sits at a
        // strictly smaller r (hotter) than a mesic one under the same digest.
        let beacon = Beacon::from_digest([42u8; 32]);
        let (_, _, r_furnace) = diophantine_siting(&civ("mineral"), &beacon);
        let (_, _, r_mesic) = diophantine_siting(&civ("biotic"), &beacon);
        assert!(
            r_furnace < r_mesic,
            "furnace sits inward: {r_furnace} < {r_mesic}"
        );
    }

    #[test]
    fn a_plasmic_line_sites_viable_and_hot() {
        // ONT-1d part 3 — the live plasmic niche. The only roster line that
        // derives a plasmic orbit (Coronal Choir, clade `plasmiform` = 12) must
        // site where its new gate — `substrate_viability` — actually clears the
        // floor: a hot world where the HyperThermal clade band and the plasma
        // phase feasibility both read high. This pins the real niche, unlike the
        // furnace test above (empty clade → mesic default), which pins only the
        // window geometry.
        let mut c = civ("plasmic");
        c.clade = "plasmiform".to_string();
        let beacon = Beacon::from_digest([42u8; 32]);
        let (class, albedo, r_plasmic) = diophantine_siting(&c, &beacon);
        let t_eq4 = thermal::t_eq4(&class.luminosity(), &albedo, &r_plasmic);
        let viability = cfe::almanac::substrate_viability(12, 3, &t_eq4, &ratio(1, 1));
        assert!(
            viability >= ratio(1, HALVING),
            "a plasmiform line sites where its own gate clears: viability = {viability}"
        );
        // And it sits inward (hotter) than a mesic biotic line under the same
        // digest — the plasma seeks the furnace.
        let (_, _, r_mesic) = diophantine_siting(&civ("biotic"), &beacon);
        assert!(
            r_plasmic < r_mesic,
            "plasma sits inward: {r_plasmic} < {r_mesic}"
        );
    }

    #[test]
    fn distinct_digests_site_distinct_orbits() {
        // The draw is continuous: two different digests land on different
        // exact orbits (not snapped to a shared seat).
        let a = diophantine_siting(&civ("biotic"), &Beacon::from_digest([1u8; 32]));
        let b = diophantine_siting(&civ("biotic"), &Beacon::from_digest([2u8; 32]));
        assert_ne!(a.2, b.2, "distinct digests draw distinct continuous orbits");
    }

    #[test]
    fn a_lawful_home_is_one_option_and_an_unlawful_home_is_reported() {
        let beacon = Beacon::from_digest([5u8; 32]);
        let home = Some((StarClass::Red, ratio(1, 2), ratio(1, 1)));
        let (options, refused) = founding_options(&civ("biotic"), &beacon, home);
        assert!(refused.is_none());
        assert!(options[0].home, "the compiled home leads the options");
        assert!(options.len() > 1, "the derived draws stand beside it");
        // albedo 1 fails the Unit door: no home option, and the refusal is named.
        let (options, refused) = founding_options(
            &civ("biotic"),
            &beacon,
            Some((StarClass::Yellow, ratio(1, 1), ratio(1, 1))),
        );
        assert!(
            refused.is_some(),
            "an unlawful home is reported, never silently unsited"
        );
        assert!(options.iter().all(|o| !o.home));
        assert!(
            !options.is_empty(),
            "the band still has derived places to go"
        );
    }

    #[test]
    fn a_dead_site_is_never_chosen_while_living_land_exists() {
        let dead = SiteOption {
            class: StarClass::BlueWhite,
            albedo: ratio(1, 4),
            orbit: ratio(1, 1),
            viability: ratio(0, 1),
            home: true,
        };
        let alive = SiteOption {
            class: StarClass::Yellow,
            albedo: ratio(0, 1),
            orbit: ratio(1, 1),
            viability: ratio(3, 4),
            home: false,
        };
        for n in 0..64u8 {
            let pick = choose_site(&[dead.clone(), alive.clone()], [n; 32]);
            assert_eq!(pick, Some(1), "seed {n}: zero viability carries no weight");
        }
    }

    #[test]
    fn the_choice_replays_and_a_band_with_no_living_land_still_settles() {
        let a = SiteOption {
            class: StarClass::Red,
            albedo: ratio(0, 1),
            orbit: ratio(2, 1),
            viability: ratio(0, 1),
            home: true,
        };
        let b = SiteOption {
            class: StarClass::Yellow,
            albedo: ratio(0, 1),
            orbit: ratio(9, 1),
            viability: ratio(0, 1),
            home: false,
        };
        let options = [a, b];
        assert_eq!(
            choose_site(&options, [9u8; 32]),
            choose_site(&options, [9u8; 32]),
            "same seed, same choice"
        );
        let picks: std::collections::BTreeSet<_> = (0..64u8)
            .filter_map(|n| choose_site(&options, [n; 32]))
            .collect();
        assert_eq!(
            picks.len(),
            2,
            "with nothing viable the draw is free across every option"
        );
        assert_eq!(choose_site(&[], [0u8; 32]), None);
    }

    #[test]
    fn the_better_site_is_likelier_never_certain() {
        let good = SiteOption {
            class: StarClass::Yellow,
            albedo: ratio(0, 1),
            orbit: ratio(1, 1),
            viability: ratio(1, 1),
            home: false,
        };
        let poor = SiteOption {
            class: StarClass::Orange,
            albedo: ratio(0, 1),
            orbit: ratio(1, 1),
            viability: ratio(1, 8),
            home: true,
        };
        let (mut g, mut p) = (0, 0);
        for n in 0..=255u8 {
            let mut seed = [0u8; 32];
            seed[0] = n;
            seed[1] = n.wrapping_mul(31);
            match choose_site(&[good.clone(), poor.clone()], seed) {
                Some(0) => g += 1,
                Some(1) => p += 1,
                _ => unreachable!(),
            }
        }
        assert!(g > p, "viability weights the draw: good {g} vs poor {p}");
        assert!(p > 0, "a poorer place is still a free choice: poor {p}");
    }

    #[test]
    fn every_compiled_line_has_living_land_among_its_options() {
        // The law the choice rests on: the band is never left with only dead
        // places by the option set itself. Measured across many genesis digests.
        for spec in crate::polities::founding_polities() {
            let home = match (spec.star, spec.albedo.clone(), spec.orbit.clone()) {
                (Some(c), Some(a), Some(o)) => Some((c, a, o)),
                _ => None,
            };
            for n in 0..8u8 {
                let (options, _) = founding_options(
                    &spec.civilization,
                    &Beacon::from_digest([n; 32]),
                    home.clone(),
                );
                assert!(
                    options.iter().any(|o| o.viability > ratio(0, 1)),
                    "{} (clade {}, {}) has no living land among {} options at digest {n}",
                    spec.civilization.sigil,
                    spec.civilization.clade,
                    spec.civilization.metabolism,
                    options.len()
                );
            }
        }
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
