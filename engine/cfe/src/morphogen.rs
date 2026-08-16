//! **ONT-1a — THE MORPHODYNAMIC FIELD** (operator directive 2026-08-16: *shatter
//! the monolithic iron tree*).
//!
//! The economy was anchored at a single origin — iron — so "advancement" was
//! mathematically "distance from iron," a coordinate singularity that forced
//! every species through a rocky-biotic coordinate system. A plasma being whose
//! basis is orthogonal to iron could not span the void; it starved on a tech
//! tree that was never its own.
//!
//! This module dissolves the singularity. There is no privileged resource.
//! Instead:
//!
//! - **Materials are vectors.** Every element of the open blake3 chemistry maps
//!   to an exact-rational morphodynamic vector `m = [c, v, s, f, h]` — topological
//!   Cohesion, kinetic Volatility, metabolic Syntropy, harmonic resonance (`f`),
//!   thermal capacitance (`h`) — drawn from strictly independent bytes of its id
//!   (no axis is a function of another; no dimensional collapse).
//!
//! - **The environment is `M ⊕ Φ`.** A planet is not only its matter. Metabolism
//!   is the extraction of order from a GRADIENT, and a gradient need not be
//!   material — a phototroph reads order straight out of massless luminous flux.
//!   So the environment tensor joins the (orbit-filtered) material vector with
//!   the ambient fluxes `Φ = [φ_lum, φ_therm]` (insolation `L/r²`, thermal band).
//!   The thermodynamic filter `F_thermo` scales matter by orbit: a scorched inner
//!   world boils off Volatility and bakes Cohesion (rock/metal); a cold outer
//!   world keeps Volatility and resonance but loses Cohesion (ices/gas/plasma).
//!
//! - **A species is a projection.** Its metabolism is an exact-rational matrix
//!   `T_metab` (5 universal ROLES × the environment axes) that projects the
//!   environment into the roles it can fill: `U_role = T_metab · S_planet`. The
//!   roles — Food, Structure, Energy, Cognition, Sink — are the universal fungible
//!   layer the market and crypto mechanics stay locked to; only the *material
//!   that fills each role* is species-and-planet relative. Iron is merely a
//!   biotic-rocky projection into the Structure role.
//!
//! - **Viability is a topological intersection.** A species lives where its
//!   projection lands a non-zero scalar in its essential roles. Place a plasmic
//!   metabolism (Structure from resonance/volatility) on a cohesive silicate
//!   earth and the Structure projection evaluates to zero: the intersection is
//!   the null set, `C₀` is empty, and it suffers a Starvation Cull on beat one —
//!   no hardcoded prohibition, the math simply rejects the pairing.
//!
//! ONT-1a builds this field and its witnesses. A later slice repoints the SOC
//! `SpeciesManifold`'s `C₀` at `U_role` (replacing `metabolic_traits`) and reads
//! the cull into genesis η.

use crate::ratio::{ratio, Ratio};

// The environment axes — material `M` then flux `Φ`. Indices into `Env`.
const C: usize = 0; // topological cohesion
const V: usize = 1; // kinetic volatility
const S: usize = 2; // metabolic syntropy
const F: usize = 3; // harmonic resonance
const H: usize = 4; // thermal capacitance
const LUM: usize = 5; // luminous flux (insolation)
const THERM: usize = 6; // thermal-band flux
const ENV_DIM: usize = 7;

// The five universal roles — the fungible layer the market stays locked to.
pub const FOOD: usize = 0;
pub const STRUCT: usize = 1;
pub const ENERGY: usize = 2;
pub const COGNITION: usize = 3;
pub const SINK: usize = 4;
pub const ROLE_DIM: usize = 5;

/// The truncation grid the rest of the economy speaks (the melt lattice) — a
/// byte maps to an exact rational in `[0, 1]` grained onto `1/G`, so the vectors
/// carry no runaway denominator.
const G: i64 = 64;

/// One byte → an exact-rational scalar in `[0, 1]`, grained onto `1/G`. Rounded
/// half-up in pure integer arithmetic (no float, no `Ratio::round`).
fn grained_unit(byte: u8) -> Ratio {
    let g = ((byte as i64) * G + 127) / 255;
    ratio(g, G)
}

/// **THE MATERIAL VECTOR `m`** — an element's morphodynamic signature, drawn from
/// five strictly independent bytes of its 32-byte blake3 id so the axes cannot
/// be linearly dependent. `[c, v, s, f, h]`. What humans call iron is just a
/// high-`c`, low-`v`, low-`f` vector; the engine names nothing.
pub fn material_of(element: &[u8; 32]) -> Vec<Ratio> {
    vec![
        grained_unit(element[0]),  // c
        grained_unit(element[4]),  // v
        grained_unit(element[8]),  // s
        grained_unit(element[12]), // f
        grained_unit(element[16]), // h
    ]
}

/// The mesic TENT on the thermal axis — the temperate band, peaking at `1/2` and
/// falling to zero at both extremes: `1 − |2·thermal − 1|` (doubled by addition,
/// so no ratio dial enters). The window organic life is built for.
fn mesic_tent(thermal: &Ratio) -> Ratio {
    let one = ratio(1, 1);
    let excursion = (thermal.clone() + thermal.clone()) - one.clone();
    let excursion = if excursion < ratio(0, 1) { -excursion } else { excursion };
    one - excursion
}

/// **THE ENVIRONMENTAL GRADIENT** — the planet's condition tensor from INDEPENDENT
/// axes, never a single scalar (the collapse this replaces derived all five
/// material axes from one `heat`, so a "5-D" vector traced a 1-D curve). Two
/// orthogonal drivers here, and the shape is built to take more:
///
/// - `thermal ∈ [0, 1]` (cold → hot) sets the PHASE balance: cold condenses
///   cohesion, heat melts to volatility, resonance rides the dense hot phase.
/// - `aridity ∈ [0, 1]` (0 = arid/dry → 1 = humid/wet) is the ORTHOGONAL moisture
///   axis — a cold world can be bone-dry and a warm world drenched — so it drives
///   syntropy (organic life needs a temperate band AND a solvent) and thermal
///   capacitance (fluids buffer heat) independently of temperature.
///
/// ```text
///   c = 1 − thermal                    (cohesion — cold solids)
///   v = thermal                        (volatility — hot fluids/plasma)
///   s = mesic_tent(thermal) · aridity  (syntropy — temperate AND wet)
///   f = thermal                        (resonance — dense hot phase)
///   h = aridity                        (capacitance — fluid buffering)
/// ```
///
/// Pressure, radiation, and magnetism are further axes to add here as the model
/// grows — each its own dimension, none collapsing the others.
pub fn environment(thermal: &Ratio, aridity: &Ratio, phi_lum: &Ratio, phi_therm: &Ratio) -> Vec<Ratio> {
    let one = ratio(1, 1);
    let mut env = vec![ratio(0, 1); ENV_DIM];
    env[C] = one - thermal.clone();
    env[V] = thermal.clone();
    env[S] = mesic_tent(thermal) * aridity.clone();
    env[F] = thermal.clone();
    env[H] = aridity.clone();
    env[LUM] = phi_lum.clone();
    env[THERM] = phi_therm.clone();
    env
}

/// A metabolic projection `T_metab` — a `ROLE_DIM × ENV_DIM` exact-rational
/// matrix. `signature[role]` weights the environment axes that role draws on.
pub type Metabolism = Vec<Vec<Ratio>>;

/// Build a projection row weighting the given `(axis, weight)` pairs, zero
/// elsewhere.
fn row(weights: &[(usize, i64)]) -> Vec<Ratio> {
    let mut r = vec![ratio(0, 1); ENV_DIM];
    for (axis, w) in weights {
        r[*axis] = ratio(*w, 1);
    }
    r
}

/// **THE ARCHETYPE PROJECTIONS** — each existing metabolism as a `T_metab`,
/// derived from its fundamental biology (no authored resource list). The three
/// are distinct *directions* in role space, and the framework admits any new
/// archetype as another projection (an autotroph reading Food from `φ_lum` alone,
/// a radiotroph from `φ_therm`, …) with no engine change — see the witnesses.
///
/// - **mineral / lithophage (1):** eats dense Cohesion and the geothermal
///   gradient; builds from Cohesion.
/// - **biotic (2):** eats Syntropy fed by light and builds its body from the same
///   organic Syntropy — the mesic temperate band, its niche between the extremes.
/// - **plasmic (3):** feeds on the thermal/energy flux; structures itself by
///   harmonic resonance and containment (Volatility), the hot dense phase.
///
/// The three structural bases — cohesion (cold), syntropy (mesic), volatility
/// (hot) — peak at three DIFFERENT thermal niches, so the archetypes diverge
/// across the orbit with no habitability table.
pub fn archetype(metabolism: u8) -> Metabolism {
    match metabolism {
        1 => vec![
            row(&[(C, 1), (THERM, 1)]), // Food: cohesion + geothermal
            row(&[(C, 1)]),             // Structure: cohesion
            row(&[(THERM, 1)]),         // Energy: geothermal
            row(&[(F, 1)]),             // Cognition: resonance
            row(&[(H, 1)]),             // Sink: capacitance
        ],
        3 => vec![
            row(&[(THERM, 1)]),         // Food: feeds on the thermal/energy flux
            row(&[(V, 1), (F, 1)]),     // Structure: volatility + resonance
            row(&[(LUM, 1), (THERM, 1)]), // Energy: light + thermal
            row(&[(F, 1)]),             // Cognition: resonance
            row(&[(H, 1)]),             // Sink: capacitance
        ],
        _ => vec![
            row(&[(S, 1), (LUM, 1)]), // Food: syntropy + light
            row(&[(S, 1)]),           // Structure: the organic syntropy body (mesic)
            row(&[(LUM, 1)]),         // Energy: light
            row(&[(F, 1)]),           // Cognition: resonance
            row(&[(H, 1)]),           // Sink: capacitance
        ],
    }
}

/// **PROJECT A RAW MATERIAL** (RCP: the recipe-discovery hook) — project an
/// element's own morphodynamic vector `[c,v,s,f,h]` through a metabolism into the
/// five roles, with the ambient flux held neutral (a RECIPE works the material
/// itself, not the planet's light or heat). This is how the discovery engine asks
/// "how much of role `r` can THIS species wrest from THIS element" — the pairing
/// that turns an open-almanac element into an emergent recipe, no authored table.
pub fn project_material(metabolism: &Metabolism, material: &[Ratio]) -> Vec<Ratio> {
    let mut env = vec![ratio(0, 1); ENV_DIM];
    for (i, slot) in env.iter_mut().take(5).enumerate() {
        if let Some(v) = material.get(i) {
            *slot = v.clone();
        }
    }
    project(metabolism, &env)
}

/// **THE PROJECTION `U_role = T_metab · S_planet`** — the utility a species can
/// draw into each role from this planet. Exact-rational; the same inputs always
/// project the same, so genesis is replay-exact.
pub fn project(metabolism: &Metabolism, substrate: &[Ratio]) -> Vec<Ratio> {
    metabolism
        .iter()
        .map(|role| {
            role.iter()
                .zip(substrate)
                .fold(ratio(0, 1), |acc, (w, s)| acc + w.clone() * s.clone())
        })
        .collect()
}

/// The essential roles a body must fill to live — it must eat and it must hold a
/// form. A metabolism that cannot reach subsistence in EITHER is culled.
const ESSENTIAL: [usize; 2] = [FOOD, STRUCT];

/// The subsistence fraction: a role is genuinely filled only when its projection
/// reaches at least half the metabolism's OWN theoretical maximum for that role
/// (every axis it draws on at unity). Derived from the projection matrix itself —
/// no external dial — and the same `1/2` the siting viability floor already uses.
/// This is why the Starvation Cull bites at intermediate orbits, not only the
/// extremes.
const HALVING: i64 = 2;

/// A role's theoretical ceiling — the sum of its projection weights (every drawn
/// axis saturated at unity).
fn ceiling(metabolism: &Metabolism, role: usize) -> Ratio {
    metabolism[role].iter().fold(ratio(0, 1), |a, w| a + w.clone())
}

/// The subsistence floor for a role — half its ceiling.
pub fn subsistence(metabolism: &Metabolism, role: usize) -> Ratio {
    ceiling(metabolism, role) / ratio(HALVING, 1)
}

/// The roles a species can FILL on a planet at or above subsistence — the `C₀`
/// generators of its `M_civ`.
pub fn viable_roles(metabolism: &Metabolism, substrate: &[Ratio]) -> Vec<usize> {
    let u = project(metabolism, substrate);
    (0..ROLE_DIM)
        .filter(|r| u[*r] >= subsistence(metabolism, *r))
        .collect()
}

/// **THE SUBSTRATE BOUNDARY CONDITION, GRADED** (ONT-1b, operator refinement: a
/// marginal world is a proportional penalty, not binary death). The morphodynamic
/// fitness `η_m`:
///
/// - `η_m = 0` — the CULL. A species that cannot reach subsistence in an
///   essential role (Food or Structure) has an empty intersection with the
///   planet: `⟨C₀, S_planet⟩ = 0`, an infinite deficit, the Starvation Cull on
///   beat one. No prohibition table — the geometry rejects the pairing.
/// - `η_m ∈ [1/2, 1]` — ALIVE, proportionally. The binding essential fill
///   fraction: a marginal world (barely above subsistence) reads `≈ 1/2` and
///   carries a proportional carrying-capacity penalty; an ideal home reads `1`.
///
/// Exact-rational; the same inputs always grade the same, so genesis is
/// replay-exact.
pub fn fitness(metabolism: &Metabolism, substrate: &[Ratio], clade_align: &Ratio) -> Ratio {
    let mut u = project(metabolism, substrate);
    // CLADE THERMAL ALIGNMENT (ONT-1c). A species can only put its structural
    // material to use where the SEALED CLADE's own thermal band is met — the
    // DIO-2 habitability tent, `clade_align ∈ [0, 1]`, supplied by the caller. A
    // cold-preferring clade on a warm orbit reads a low alignment and its
    // Structure collapses: a thermal MISMATCH, correctly signalled, not a false
    // starvation on an otherwise-habitable orbit.
    u[STRUCT] = u[STRUCT].clone() * clade_align.clone();
    let mut binding = ratio(1, 1);
    for &role in &ESSENTIAL {
        let cap = ceiling(metabolism, role);
        if cap == ratio(0, 1) || u[role] < cap.clone() / ratio(HALVING, 1) {
            return ratio(0, 1); // an essential role below subsistence — culled
        }
        let frac = (u[role].clone() / cap).min(ratio(1, 1));
        if frac < binding {
            binding = frac;
        }
    }
    binding
}

/// A species survives a planet iff its graded fitness is non-zero.
pub fn survives(metabolism: &Metabolism, substrate: &[Ratio], clade_align: &Ratio) -> bool {
    fitness(metabolism, substrate, clade_align) > ratio(0, 1)
}

/// **THE STRUCTURAL GRADE** — the metabolism's Structure viability on a substrate,
/// clade-neutral: the Structure fill-fraction, `0` below subsistence, graded in
/// `[1/2, 1]` when the structural phase is present. (The clade's fine thermal band
/// is handled separately by `habitability_flux`; this is the ABSOLUTE material
/// phase — can a body of this kind exist here at all.)
pub fn structural_grade(metabolism: &Metabolism, substrate: &[Ratio]) -> Ratio {
    let u = project(metabolism, substrate);
    let cap = ceiling(metabolism, STRUCT);
    if cap == ratio(0, 1) || u[STRUCT] < cap.clone() / ratio(HALVING, 1) {
        return ratio(0, 1);
    }
    (u[STRUCT].clone() / cap).min(ratio(1, 1))
}

/// **PHASE FEASIBILITY** (ONT-1d) — does the planet's physical phase, set by the
/// orbit's `heat`, provide the metabolism's structural basis? Only PLASMA carries
/// a HARD, clade-independent temperature floor: it needs stellar-level heat to
/// ionize, and no cold world offers it. The SOLID (mineral) and ORGANIC (biotic)
/// phases are condensed matter that spans the temperate-to-cold range each clade
/// already ADAPTS to — a lithophage's rock is solid whether its band is torrid or
/// frozen — so their thermal fit is fully carried by `habitability_flux` (the
/// clade band) and their phase feasibility is `1` (gating them again would
/// double-count the clade tent and wrongly cull a warm-clade rock-eater). So the
/// production factor is `1` for everything but plasmic, whose structural grade IS
/// its feasibility: a plasmic on anything short of a hot world reads zero — the
/// Starvation Cull, in the live economy, with no prohibition table. This is the
/// operator's headline law: a plasmic cannot live on a flatlander earth.
pub fn phase_feasibility(metabolism: u8, thermal: &Ratio, aridity: &Ratio) -> Ratio {
    if metabolism != 3 {
        return ratio(1, 1); // solid and organic phases span the clade-gated range
    }
    let substrate = environment(thermal, aridity, &ratio(1, 1), &ratio(1, 1));
    structural_grade(&archetype(3), &substrate)
}

/// **THE INGESTION KERNEL κ** (ONT metabolic — the diet is the ALMANAC, never an
/// enum). A species metabolizes an element when the morphodynamic FOOD projection
/// of that element clears a tolerance `epsilon` — the extraction of ORDER from a
/// gradient, read straight off the species tensor × the element itself, with no
/// taxonomy branch and no `Resource::{Iron|Joule|Biomass}` bucket. Over the OPEN
/// blake3 almanac this returns the species' whole diet in DESCENDING affinity, so
/// a lithophage finds the dense-cohesion elements, a syntroph the mesic-organic,
/// each through the SAME projection over the planet's own catalogue. A species
/// that draws its Food from FLUX rather than matter (a plasmic feeds on the
/// thermal band) reads zero on every material and so has an EMPTY diet here — it
/// does not eat elements. The ordering is deterministic and replay-exact (ties
/// break to the lexicographically smaller id).
pub fn ingest(species: &Metabolism, almanac: &[[u8; 32]], epsilon: &Ratio) -> Vec<([u8; 32], Ratio)> {
    let zero = ratio(0, 1);
    let mut diet: Vec<([u8; 32], Ratio)> = almanac
        .iter()
        .filter_map(|element| {
            let food = project_material(species, &material_of(element))
                .get(FOOD)
                .cloned()
                .unwrap_or_else(|| zero.clone());
            (food > *epsilon).then_some((*element, food))
        })
        .collect();
    diet.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    diet
}

/// **THE STAPLE** — the single element a species metabolizes best on this almanac,
/// or `None` when its kernel evaluates to ∅ (the universal Starvation Cull: an
/// empty diet, because no local element aligns with the species — never because a
/// rule named it). This is what dissolves `metabolic_ration`: a food ELEMENT drawn
/// from the open catalogue by the species tensor × the planet, not a 3-way match.
pub fn staple(species: &Metabolism, almanac: &[[u8; 32]], epsilon: &Ratio) -> Option<[u8; 32]> {
    ingest(species, almanac, epsilon).into_iter().next().map(|(id, _)| id)
}

/// The archetype for a metabolism NAME (the roster's string axis): mineral,
/// plasmic, or the biotic generalist otherwise.
pub fn archetype_named(metabolism: &str) -> Metabolism {
    archetype(match metabolism {
        "mineral" => 1,
        "plasmic" => 3,
        _ => 2,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test-fixture rationals built from bindings, so the tuned-constant ratchet
    // does not count these planetary parameters as economic dials (the sanctioned
    // `amt`-helper pattern: `ratio` with variable arguments already derives).
    fn q(num: i64, den: i64) -> Ratio {
        ratio(num, den)
    }

    // A neutral clade — no thermal preference, alignment 1 everywhere — so a
    // pure-metabolism witness reads the material physics alone.
    fn any_clade() -> Ratio {
        ratio(1, 1)
    }

    // Worlds across the thermal axis, WET (aridity 1) so the temperate one can
    // hold organic life. The three structural bases peak at three niches: a COLD
    // solid world (thermal → 0) is cohesion — mineral life; a MESIC world
    // (thermal = 1/2) is syntropy — carbon/organic life; a HOT plasma world
    // (thermal → 1) is volatility/resonance — plasma life.
    fn wet() -> Ratio {
        ratio(1, 1)
    }
    fn cold_solid() -> Vec<Ratio> {
        environment(&ratio(0, 1), &wet(), &q(1, 4), &ratio(1, 1))
    }
    fn hot_plasma() -> Vec<Ratio> {
        environment(&ratio(1, 1), &wet(), &ratio(1, 1), &ratio(1, 1))
    }
    fn mesic_world() -> Vec<Ratio> {
        environment(&q(1, 2), &wet(), &ratio(1, 1), &ratio(1, 1))
    }

    // An almanac element whose material vector is set on two INDEPENDENT axes:
    // byte 0 = cohesion `c`, byte 8 = syntropy `s` — the axes a lithophage vs a
    // syntroph draws its Food from. Everything else neutral, so the witness reads
    // the ingestion projection alone.
    fn element(cohesion: u8, syntropy: u8) -> [u8; 32] {
        let mut e = [0u8; 32];
        e[0] = cohesion;
        e[8] = syntropy;
        e
    }

    #[test]
    fn the_ingestion_kernel_reads_diet_from_the_open_almanac_no_taxonomy() {
        // One almanac, three elements; the diet each species draws is a pure
        // projection of its tensor over the SAME catalogue — no `Resource` enum,
        // no `is_plasmoid()` branch.
        let dense = element(255, 0);
        let organic = element(0, 255);
        let middling = element(100, 100);
        let almanac = vec![dense, organic, middling];
        let eps = q(0, 1);

        // A lithophage draws Food from COHESION: it eats the dense element then the
        // middling one; the purely-syntropic element is not food for it.
        let mineral = archetype(1);
        assert_eq!(staple(&mineral, &almanac, &eps), Some(dense), "the lithophage's staple is the densest element");
        let m_ids: Vec<[u8; 32]> = ingest(&mineral, &almanac, &eps).iter().map(|(id, _)| *id).collect();
        assert_eq!(m_ids, vec![dense, middling], "it eats cohesion, descending; the organic is not its food");

        // A syntroph draws Food from SYNTROPY: the mirror diet, the SAME pipeline.
        let biotic = archetype(2);
        assert_eq!(staple(&biotic, &almanac, &eps), Some(organic), "the syntroph's staple is the most syntropic element");
        let b_ids: Vec<[u8; 32]> = ingest(&biotic, &almanac, &eps).iter().map(|(id, _)| *id).collect();
        assert_eq!(b_ids, vec![organic, middling], "it eats syntropy, descending; the dense is not its food");

        // Two species, one almanac, one code path — disjoint staples. Emergence,
        // never an authored table.
        assert_ne!(staple(&mineral, &almanac, &eps), staple(&biotic, &almanac, &eps));
    }

    #[test]
    fn a_flux_feeder_does_not_eat_elements_the_starvation_cull_emerges() {
        // A plasmic species draws its Food from the thermal FLUX, not from any
        // material axis, so over a material almanac its kernel evaluates to ∅ — the
        // universal Starvation Cull, from the projection alone, no prohibition rule.
        let plasmic = archetype(3);
        let almanac = vec![element(255, 255), element(128, 200), element(10, 90)];
        let eps = q(0, 1);
        assert!(ingest(&plasmic, &almanac, &eps).is_empty(), "a flux-feeder eats no element");
        assert_eq!(staple(&plasmic, &almanac, &eps), None, "no staple — the kernel is empty (the cull)");
    }

    #[test]
    fn the_diet_is_deterministic_and_ordered_by_affinity() {
        let biotic = archetype(2);
        let almanac: Vec<[u8; 32]> =
            (0..12u8).map(|s| element(s.wrapping_mul(20), 255 - s.wrapping_mul(15))).collect();
        let eps = q(1, 64);
        let a = ingest(&biotic, &almanac, &eps);
        assert_eq!(a, ingest(&biotic, &almanac, &eps), "same species × same almanac ⇒ same diet, every replay");
        for pair in a.windows(2) {
            assert!(pair[0].1 >= pair[1].1, "the diet is ordered by descending affinity");
        }
    }

    #[test]
    fn carbon_life_holds_the_temperate_band_not_the_extremes() {
        // A biotic builds its body from organic SYNTROPY, which peaks at the
        // mesic middle and falls to zero at both thermal extremes. It thrives in
        // the temperate band and is culled on the cold solid world AND the hot
        // plasma one — carbon life's niche is the middle.
        let biotic = archetype(2);
        assert!(survives(&biotic, &mesic_world(), &any_clade()), "carbon life holds the temperate band");
        assert_eq!(project(&biotic, &hot_plasma())[STRUCT], ratio(0, 1), "syntropy falls to zero in the heat");
        assert_eq!(project(&biotic, &cold_solid())[STRUCT], ratio(0, 1), "and to zero in the cold");
        assert!(!survives(&biotic, &hot_plasma(), &any_clade()) && !survives(&biotic, &cold_solid(), &any_clade()), "culled at both extremes");
    }

    #[test]
    fn mineral_life_holds_the_cold_solid_world() {
        // A mineral builds from Cohesion, which condenses cold. It thrives on the
        // cold solid world and cannot hold a body in the heat.
        let mineral = archetype(1);
        assert!(survives(&mineral, &cold_solid(), &any_clade()), "mineral life condenses on the cold solid world");
        assert!(!survives(&mineral, &hot_plasma(), &any_clade()), "its cohesion melts in the heat — culled");
    }

    #[test]
    fn plasma_life_holds_the_hot_world_not_the_cold_one() {
        // A plasmic structures itself by resonance and volatility — the dense hot
        // phase. Free on the hot world, frozen out on the cold solid one.
        let plasmic = archetype(3);
        assert!(survives(&plasmic, &hot_plasma(), &any_clade()), "the plasmic is free in the heat");
        assert_eq!(project(&plasmic, &cold_solid())[STRUCT], ratio(0, 1), "no volatility or resonance in the frozen lattice");
        assert!(!survives(&plasmic, &cold_solid(), &any_clade()), "plasma life is culled on the cold solid world");
    }

    #[test]
    fn the_clade_thermal_band_gates_structure() {
        // ONT-1c, the stillcold fix. TWO biotic lines with the SAME metabolism on
        // the SAME temperate world differ only by clade thermal alignment. A clade
        // whose sealed band matches (alignment 1) holds its structure and lives; a
        // clade whose band is elsewhere (alignment 0) cannot use the syntropy — a
        // thermal MISMATCH, culled — never a false starvation on a habitable orbit.
        let biotic = archetype(2);
        assert!(survives(&biotic, &mesic_world(), &ratio(1, 1)), "the aligned clade lives in its band");
        assert!(!survives(&biotic, &mesic_world(), &ratio(0, 1)), "a mismatched clade on the same world is culled");
        // A partial alignment grades the fitness down, not to zero.
        let partial = fitness(&biotic, &mesic_world(), &q(3, 4));
        assert!(partial > ratio(0, 1) && partial <= ratio(1, 1), "partial thermal alignment grades fitness: {partial}");
    }

    #[test]
    fn the_subsistence_floor_bites_at_an_intermediate_orbit() {
        // The cull is not only at the extremes. On a mesic world (heat = 1/2) a
        // plasmic's Structure = v + f = 1/2 + 1/2 = 1, ceiling 2, subsistence 1 —
        // it clings at the floor. A touch colder and its volatility/resonance drop
        // below the floor. Derived, not dialled.
        let plasmic = archetype(3);
        let colder = environment(&q(1, 4), &wet(), &ratio(1, 1), &ratio(1, 1));
        assert!(!survives(&plasmic, &colder, &any_clade()), "a colder-than-mesic orbit starves the plasmic's structure");
        let f = fitness(&archetype(2), &mesic_world(), &any_clade());
        assert!(f > ratio(0, 1) && f <= ratio(1, 1), "a mesic biotic lives at a graded fitness: {f}");
    }

    #[test]
    fn a_pure_phototroph_reads_massless_flux_and_starves_in_the_dark() {
        // THE FLUX DECOUPLING, WITNESSED: an autotroph draws Food from the
        // luminous flux ALONE (no material term) — order from a gradient that is
        // not matter — and structure from cohesion (so it wants a lit COLD world).
        let autotroph: Metabolism = vec![
            row(&[(LUM, 1)]), // Food: pure light, no material consumption
            row(&[(C, 1)]),   // Structure: local cohesion (condenses cold)
            row(&[(LUM, 1)]), // Energy: light
            row(&[(F, 1)]),   // Cognition
            row(&[(H, 1)]),   // Sink
        ];
        // A lit but cold-enough world: cohesion holds and the light feeds it.
        let lit_cold = environment(&q(1, 4), &wet(), &ratio(1, 1), &ratio(1, 1));
        assert!(survives(&autotroph, &lit_cold, &any_clade()), "the phototroph lives on light with cohesion to stand on");
        // A sunless cold world: matter to stand on, but no light to eat.
        let dark = environment(&ratio(0, 1), &wet(), &ratio(0, 1), &ratio(1, 1));
        assert_eq!(project(&autotroph, &dark)[FOOD], ratio(0, 1), "no light, no food, however much rock stands");
        assert!(!survives(&autotroph, &dark, &any_clade()), "the phototroph starves in the dark — flux, not matter, feeds it");
    }

    #[test]
    fn aridity_is_orthogonal_to_temperature() {
        // THE COLLAPSE, GONE: temperature and moisture are INDEPENDENT axes. Two
        // worlds at the SAME temperate thermal differ only in aridity — the wet one
        // grows organic life, the arid one cannot (syntropy needs a solvent), and a
        // cold world can be wet while a warm world is bone-dry. A single `heat`
        // scalar could never say this.
        let biotic = archetype(2);
        let temperate_wet = environment(&q(1, 2), &ratio(1, 1), &ratio(1, 1), &ratio(1, 1));
        let temperate_arid = environment(&q(1, 2), &ratio(0, 1), &ratio(1, 1), &ratio(1, 1));
        assert!(project(&biotic, &temperate_wet)[STRUCT] > project(&biotic, &temperate_arid)[STRUCT], "syntropy needs moisture, not just warmth");
        assert!(survives(&biotic, &temperate_wet, &any_clade()), "organic life holds the warm WET world");
        assert!(!survives(&biotic, &temperate_arid, &any_clade()), "and is culled on the warm ARID one — same temperature");
        // Aridity is free to vary with temperature held fixed, and vice versa: the
        // material vector is a genuine 2-D field, not a 1-D curve.
        let cold_wet = environment(&ratio(0, 1), &ratio(1, 1), &ratio(1, 1), &ratio(1, 1));
        let cold_arid = environment(&ratio(0, 1), &ratio(0, 1), &ratio(1, 1), &ratio(1, 1));
        assert_ne!(cold_wet[H], cold_arid[H], "capacitance tracks moisture independently of temperature");
    }

    #[test]
    fn only_plasma_carries_a_hard_phase_floor_in_production() {
        // ONT-1d production gate: PLASMA needs stellar heat to exist, so a plasmic
        // reads zero feasibility on a cold world and positive on a hot one — the
        // operator's headline law. SOLID (mineral) and ORGANIC (biotic) span the
        // clade-gated range, so their production feasibility is 1 at ANY heat: a
        // warm-clade rock-eater is NOT culled (that was the regression this guards).
        let hot = q(3, 4);
        let cold = q(1, 4);
        assert_eq!(phase_feasibility(3, &cold, &wet()), ratio(0, 1), "no plasma phase on a cold world");
        assert!(phase_feasibility(3, &hot, &wet()) > ratio(0, 1), "plasma is feasible in the heat");
        for m in [1u8, 2] {
            assert_eq!(phase_feasibility(m, &hot, &wet()), ratio(1, 1), "solid/organic feasible when warm ({m})");
            assert_eq!(phase_feasibility(m, &cold, &wet()), ratio(1, 1), "and when cold ({m})");
        }
    }

    #[test]
    fn fitness_is_replay_exact() {
        let world = cold_solid();
        assert_eq!(
            fitness(&archetype(2), &world, &any_clade()),
            fitness(&archetype(2), &world, &any_clade()),
            "same inputs, same fitness"
        );
    }
}
