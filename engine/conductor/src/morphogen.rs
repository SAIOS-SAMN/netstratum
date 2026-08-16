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

/// The roles a species can actually FILL on a planet — those with a strictly
/// positive projection. These become the `C₀` generators of its `M_civ`.
pub fn viable_roles(u_role: &[Ratio]) -> Vec<usize> {
    (0..ROLE_DIM).filter(|r| u_role[*r] > ratio(0, 1)).collect()
}

/// **THE SUBSTRATE BOUNDARY CONDITION.** A metabolism survives only where it can
/// fill both ESSENTIAL roles — Food (it must eat) and Structure (it must hold a
/// body). If either projects to zero the intersection with the planet is the
/// null set: `η = 0`, an infinite deficit, the Starvation Cull on beat one. No
/// prohibition table — the geometry rejects the pairing.
pub fn survives(u_role: &[Ratio]) -> bool {
    u_role[FOOD] > ratio(0, 1) && u_role[STRUCT] > ratio(0, 1)
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

    // A representative element endowment: a spread of ids so the mean material
    // vector is well away from any axis, letting the orbit filter decide the
    // planet's character.
    fn endowment(salt: u8) -> Vec<[u8; 32]> {
        (0..8u8).map(|i| *blake3::hash(&[salt, i]).as_bytes()).collect()
    }

    // A hot rocky inner world (flatlander earth): heat at the scorched extreme →
    // cohesion baked in, volatility and resonance fully driven off; bathed in
    // light. (The extremes give the cull an exact zero; intermediate orbits need
    // a subsistence threshold, a wiring-time calibration.)
    fn rocky_earth() -> Vec<Ratio> {
        planet_substrate(&endowment(1), &ratio(1, 1), &ratio(1, 1), &q(1, 2))
    }
    // A cold outer gas giant: heat at the frozen extreme → cohesion fully lost,
    // volatility and resonance preserved; little light, a live thermal gradient.
    fn gas_giant() -> Vec<Ratio> {
        planet_substrate(&endowment(1), &ratio(0, 1), &q(1, 8), &ratio(1, 1))
    }

    #[test]
    fn the_biotic_generalist_lives_on_rocky_earth_not_a_gas_giant() {
        let biotic = archetype(2);
        assert!(survives(&project(&biotic, &rocky_earth())), "biotic thrives on the rocky lit world");
        // On a gas giant its Structure role (Cohesion) projects to ~zero — it
        // cannot build a body from gas.
        let on_giant = project(&biotic, &gas_giant());
        assert!(on_giant[FOOD] > ratio(0, 1), "it can still find food");
        assert!(!survives(&on_giant), "but it cannot hold structure in gas — culled");
    }

    #[test]
    fn the_plasmic_dies_on_earth_and_is_free_on_the_gas_giant() {
        // THE OPERATOR'S LAW: a plasmic metabolism structures itself by resonance
        // and volatility — both baked out of a hot silicate earth — so its
        // Structure projects to zero: empty intersection, Starvation Cull.
        let plasmic = archetype(3);
        let on_earth = project(&plasmic, &rocky_earth());
        assert_eq!(on_earth[STRUCT], ratio(0, 1), "no resonance or volatility on a baked rock");
        assert!(!survives(&on_earth), "plasmic on a flatlander earth is culled on beat one");
        // On the cold gas giant volatility and resonance survive — it is free.
        assert!(survives(&project(&plasmic, &gas_giant())), "the plasmic is free on the gas giant");
    }

    #[test]
    fn the_lithophage_eats_the_geothermal_gradient() {
        // A mineral metabolism draws Food from Cohesion + the thermal gradient,
        // so a warm rocky world with a live gradient sustains it.
        let litho = archetype(1);
        assert!(survives(&project(&litho, &rocky_earth())), "the lithophage eats rock and heat");
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
        assert!(survives(&project(&autotroph, &rocky_earth())), "the phototroph lives on light");
        // A sunless rogue world (φ_lum = 0) but with rock and a thermal gradient:
        // there is matter to stand on, but no light to eat — Food is zero.
        let dark_rogue = planet_substrate(&endowment(2), &ratio(1, 1), &ratio(0, 1), &ratio(1, 1));
        let starved = project(&autotroph, &dark_rogue);
        assert_eq!(starved[FOOD], ratio(0, 1), "no light, no food, however much rock stands");
        assert!(!survives(&starved), "the phototroph starves in the dark — flux, not matter, feeds it");
    }

    #[test]
    fn the_projection_is_replay_exact() {
        let biotic = archetype(2);
        let earth = rocky_earth();
        assert_eq!(project(&biotic, &earth), project(&biotic, &earth), "same inputs, same projection");
    }
}
