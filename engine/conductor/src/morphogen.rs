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

use cfe::ratio::{ratio, Ratio};

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

/// **THE THERMODYNAMIC FILTER `F_thermo`** — the orbit's diagonal scaling of the
/// raw material, `heat ∈ [0, 1]` rising with insolation. A scorched world
/// (`heat → 1`) maximises Cohesion and drives Volatility and resonance to zero
/// (matter bakes to rock/metal); a frozen world (`heat → 0`) loses Cohesion but
/// keeps Volatility and resonance (ices, gas, plasma). Syntropy and thermal
/// capacitance ride neutral. This is why inner worlds are rocky and outer worlds
/// gaseous — the same duality our own system shows.
fn thermo_scale(heat: &Ratio) -> Vec<Ratio> {
    let one = ratio(1, 1);
    let cold = one.clone() - heat.clone();
    let mut scale = vec![ratio(0, 1); 5];
    scale[C] = heat.clone(); // cohesion bakes in with heat
    scale[V] = cold.clone(); // volatility boils off with heat
    scale[S] = one.clone(); // syntropy neutral
    scale[F] = cold; // resonance survives in the cold
    scale[H] = one; // capacitance neutral
    scale
}

/// **THE PLANETARY SUBSTRATE `S_planet = (F_thermo · m̄) ⊕ Φ`** — the orbit-filtered
/// mean material vector of the planet's element endowment, joined with the
/// massless ambient fluxes. `phi_lum` is exact insolation `L/r²`; `phi_therm` the
/// thermal-band gradient. A planet with no elements still offers its fluxes — a
/// phototroph can live on light alone.
pub fn planet_substrate(
    elements: &[[u8; 32]],
    heat: &Ratio,
    phi_lum: &Ratio,
    phi_therm: &Ratio,
) -> Vec<Ratio> {
    let scale = thermo_scale(heat);
    let n = elements.len().max(1) as i64;
    // Mean material over the endowment, then the orbit filter.
    let mut mat = vec![ratio(0, 1); 5];
    for element in elements {
        let m = material_of(element);
        for i in 0..5 {
            mat[i] = mat[i].clone() + m[i].clone();
        }
    }
    for i in 0..5 {
        mat[i] = scale[i].clone() * (mat[i].clone() / ratio(n, 1));
    }
    let mut env = vec![ratio(0, 1); ENV_DIM];
    env[..5].clone_from_slice(&mat);
    env[LUM] = phi_lum.clone();
    env[THERM] = phi_therm.clone();
    env
}

/// **`S_planet` FROM THE ORBIT ALONE** — the thermodynamic filter applied to a
/// uniform unit material base (element specifics are a later refinement), joined
/// with the ambient fluxes. This is the substrate as siting sees it before a
/// planet's elements are seeded: a hot inner orbit is cohesive rock (high `c`, no
/// `v`/`f`), a cold outer orbit is volatile and resonant (no `c`, high `v`/`f`).
pub fn planet_from_orbit(heat: &Ratio, phi_lum: &Ratio, phi_therm: &Ratio) -> Vec<Ratio> {
    let scale = thermo_scale(heat); // uniform unit base ⇒ the filter IS the material
    let mut env = vec![ratio(0, 1); ENV_DIM];
    env[..5].clone_from_slice(&scale);
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
/// - **biotic (2):** eats Syntropy fed by light; builds from Cohesion (mines its
///   tools) — the generalist.
/// - **plasmic (3):** feeds on the thermal/energy flux; structures itself by
///   harmonic resonance and containment (Volatility), never by Cohesion.
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
            row(&[(C, 1)]),           // Structure: cohesion
            row(&[(LUM, 1)]),         // Energy: light
            row(&[(F, 1)]),           // Cognition: resonance
            row(&[(H, 1)]),           // Sink: capacitance
        ],
    }
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
pub fn fitness(metabolism: &Metabolism, substrate: &[Ratio]) -> Ratio {
    let u = project(metabolism, substrate);
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
pub fn survives(metabolism: &Metabolism, substrate: &[Ratio]) -> bool {
    fitness(metabolism, substrate) > ratio(0, 1)
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

    // The planet as siting sees it, from the orbit alone. A hot rocky inner world
    // (flatlander earth): heat at the scorched extreme → cohesion baked in,
    // volatility and resonance driven off; bathed in light. A cold outer gas
    // giant: heat at the frozen extreme → cohesion lost, volatility and resonance
    // preserved; little light, a live thermal gradient. A warm mesic world sits
    // between (heat = 1/2), where the subsistence floor decides the marginal case.
    fn rocky_earth() -> Vec<Ratio> {
        planet_from_orbit(&ratio(1, 1), &ratio(1, 1), &q(1, 2))
    }
    fn gas_giant() -> Vec<Ratio> {
        planet_from_orbit(&ratio(0, 1), &q(1, 8), &ratio(1, 1))
    }
    fn mesic_world() -> Vec<Ratio> {
        planet_from_orbit(&q(1, 2), &ratio(1, 1), &q(1, 2))
    }

    #[test]
    fn the_biotic_generalist_lives_on_rocky_earth_not_a_gas_giant() {
        let biotic = archetype(2);
        assert!(survives(&biotic, &rocky_earth()), "biotic thrives on the rocky lit world");
        // On a gas giant its Structure role (Cohesion) falls below subsistence —
        // it cannot build a body from gas.
        assert!(project(&biotic, &gas_giant())[FOOD] > ratio(0, 1), "it can still find food");
        assert!(!survives(&biotic, &gas_giant()), "but it cannot hold structure in gas — culled");
    }

    #[test]
    fn the_plasmic_dies_on_earth_and_is_free_on_the_gas_giant() {
        // THE OPERATOR'S LAW: a plasmic metabolism structures itself by resonance
        // and volatility — both baked out of a hot silicate earth — so its
        // Structure falls to zero: empty intersection, Starvation Cull.
        let plasmic = archetype(3);
        assert_eq!(project(&plasmic, &rocky_earth())[STRUCT], ratio(0, 1), "no resonance or volatility on a baked rock");
        assert!(!survives(&plasmic, &rocky_earth()), "plasmic on a flatlander earth is culled on beat one");
        // On the cold gas giant volatility and resonance survive — it is free.
        assert!(survives(&plasmic, &gas_giant()), "the plasmic is free on the gas giant");
    }

    #[test]
    fn the_subsistence_floor_bites_at_an_intermediate_orbit() {
        // ONT-1b: the cull is not only at the extremes. On a warm mesic world
        // (heat = 1/2), a plasmic's Structure = v + f = 1/2 + 1/2 = 1, but its
        // ceiling is 2, so subsistence is 1 — it sits EXACTLY at the floor and
        // barely clings on, while a biotic (Structure = cohesion 1/2, ceiling 1,
        // subsistence 1/2) also just clears. The floor is the wall the operator
        // asked for: derived, not dialled, and it decides the marginal world.
        let plasmic = archetype(3);
        // Nudge the mesic world a touch warmer: v, f drop below their floor.
        let warmer = planet_from_orbit(&q(3, 4), &ratio(1, 1), &q(1, 2));
        assert!(!survives(&plasmic, &warmer), "a warmer-than-mesic orbit already starves the plasmic's structure");
        // A marginal survivor reads a proportional fitness in [1/2, 1], not 1.
        let f = fitness(&archetype(2), &mesic_world());
        assert!(f > ratio(0, 1) && f <= ratio(1, 1), "a mesic biotic lives, at a graded fitness: {f}");
    }

    #[test]
    fn the_lithophage_eats_the_geothermal_gradient() {
        // A mineral metabolism draws Food from Cohesion + the thermal gradient,
        // so a warm rocky world with a live gradient sustains it.
        assert!(survives(&archetype(1), &rocky_earth()), "the lithophage eats rock and heat");
    }

    #[test]
    fn a_pure_phototroph_reads_massless_flux_and_starves_in_the_dark() {
        // THE FLUX DECOUPLING, WITNESSED: an autotroph draws Food from the
        // luminous flux ALONE (no material term) and structure from cohesion.
        // Metabolism as extraction of order from a gradient that is not matter.
        let autotroph: Metabolism = vec![
            row(&[(LUM, 1)]), // Food: pure light, no material consumption
            row(&[(C, 1)]),   // Structure: local cohesion
            row(&[(LUM, 1)]), // Energy: light
            row(&[(F, 1)]),   // Cognition
            row(&[(H, 1)]),   // Sink
        ];
        // A lit rocky world: it lives on sunlight.
        assert!(survives(&autotroph, &rocky_earth()), "the phototroph lives on light");
        // A sunless rogue world (φ_lum = 0) but with rock and a thermal gradient:
        // there is matter to stand on, but no light to eat — Food is zero.
        let dark_rogue = planet_from_orbit(&ratio(1, 1), &ratio(0, 1), &ratio(1, 1));
        assert_eq!(project(&autotroph, &dark_rogue)[FOOD], ratio(0, 1), "no light, no food, however much rock stands");
        assert!(!survives(&autotroph, &dark_rogue), "the phototroph starves in the dark — flux, not matter, feeds it");
    }

    #[test]
    fn fitness_is_replay_exact() {
        let earth = rocky_earth();
        assert_eq!(fitness(&archetype(2), &earth), fitness(&archetype(2), &earth), "same inputs, same fitness");
    }
}
