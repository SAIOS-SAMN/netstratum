//! **RCP-2 — the emergent recipe SPACE** (operator directive 2026-08-16: *the
//! doing-layer must be emergent, never an enum or a scalar*).
//!
//! The map (RCP-1) found the doing-layer was a closed `ZoneKind` enum priced over
//! seven fixed Resources, unlocked by an integer level (`level ≥ 3`). Both are
//! anthropocentric vestiges. RCP-2 dissolves them, honouring three immutable laws:
//!
//! 1. **No enum expansion.** A recipe is not a Rust variant. It is a
//!    dynamically-synthesised transformation from open-almanac elements into the
//!    universal roles, identified by a blake3 hash of its own content — emergent,
//!    replayable, and unlimited, with no authored dispatch.
//! 2. **No scalar level gate.** "Technology" is not a counter. Advancement is
//!    topological: a civilization can work an element only when its manifold's
//!    algebraic rank (`genus`, `Σ βₖ`) spans the element's own intrinsic
//!    complexity. Reach is the gate, not a number that ticks up.
//! 3. **Modular truncation from line one.** The recipe yields are grained onto
//!    the melt lattice at synthesis, so a per-recipe rational can never seed the
//!    denominator explosions the accumulators are prone to. (The nonlinear
//!    `ℚ[x]/I` composition of multiple elements — `xᵢ·xⱼ` — is the Gröbner
//!    frontier; when it lands it carries the same lazy-truncation discipline.)
//!
//! **Discovery (the conductor's bridge).** When a society hits an irreducible
//! deficit `γ` in a role its current `C₀` cannot span, the engine does not fall
//! back to a static table. It searches the open blake3 element almanac for the
//! element whose morphodynamic vector `m` projects the strongest non-zero yield
//! into the unspanned role (`morphogen::project_material`), gated by topological
//! reach, and mints that mapping as a `MintedRecipe`. A role no element can fill
//! for this species yields nothing — the honest hole, not a forced fallback.

use cfe::morphogen;
use cfe::ratio::{ratio, Ratio};

/// The melt lattice recipe yields grain onto — the same `64` the market, regrowth,
/// and orbit grids speak. A bare structural constant (bounds a per-recipe
/// denominator), not a balance dial.
const RECIPE_GRAIN: i64 = 64;

/// **An emergent recipe** — a hashed transformation `∏ elements → role`, not a
/// `ZoneKind` variant. Its `id` is the blake3 of its content, so the same
/// discovery always mints the same recipe (replay-exact) and two distinct
/// transformations never collide, across an unbounded space with no enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MintedRecipe {
    /// blake3(inputs ‖ role) — the recipe's cryptographic identity.
    pub id: [u8; 32],
    /// The open-almanac element ids this recipe consumes.
    pub inputs: Vec<[u8; 32]>,
    /// The universal role it fills (`morphogen::FOOD … SINK`) — the fungible slot,
    /// the only fixed vocabulary, shared by the market.
    pub role: usize,
    /// How much of the role it delivers, grained onto the melt lattice.
    pub yield_of_role: Ratio,
}

/// An element's intrinsic complexity, read from its own blake3 bytes (the tier the
/// almanac already speaks) — the topological rank a civilization must reach to
/// work it. This is what replaces `level ≥ 3`: a deep element demands a
/// higher-genus manifold, and advancement earns reach rather than ticking a
/// counter.
fn element_complexity(element: &[u8; 32]) -> u64 {
    1 + (element[2] as u64 % 3)
}

/// Grain a yield onto the melt lattice (modular truncation, from line one).
fn grain(x: Ratio) -> Ratio {
    let grid = ratio(RECIPE_GRAIN, 1);
    (x * grid.clone()).round() / grid
}

/// **Discover an emergent recipe** closing a deficit in `role`. Searches the open
/// `almanac` for the element whose morphodynamic vector projects the strongest
/// yield into the role under the species' projection `metabolism`, admitting only
/// elements whose complexity the manifold's `rank` (algebraic dimension `Σ βₖ`)
/// can span. Returns the minted recipe, or `None` — an honest hole (no element the
/// species can turn into that role) or insufficient reach (topological, not a
/// level). Deterministic; the winning element and its grained yield are a pure
/// function of the inputs, so the mint replays exactly.
pub fn discover(
    role: usize,
    almanac: &[[u8; 32]],
    metabolism: &morphogen::Metabolism,
    rank: &crate::soc::AlgebraicRank,
) -> Option<MintedRecipe> {
    let reach = rank.dimension();
    let zero = ratio(0, 1);
    let mut best: Option<([u8; 32], Ratio)> = None;
    for element in almanac {
        // Topological gate: the manifold must reach the element's complexity.
        if reach < element_complexity(element) {
            continue;
        }
        let delivered = grain(
            morphogen::project_material(metabolism, &morphogen::material_of(element))
                .get(role)
                .cloned()
                .unwrap_or_else(|| zero.clone()),
        );
        if delivered <= zero {
            continue; // this element carries nothing of the deficit role
        }
        // The strongest projection wins; ties break to the lexicographically
        // smaller id so the choice is deterministic, never order-dependent.
        let better = match &best {
            None => true,
            Some((be, bd)) => delivered > *bd || (delivered == *bd && element < be),
        };
        if better {
            best = Some((*element, delivered));
        }
    }
    best.map(|(element, yield_of_role)| {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&element);
        hasher.update(&[role as u8]);
        hasher.update(b"recipe");
        MintedRecipe {
            id: *hasher.finalize().as_bytes(),
            inputs: vec![element],
            role,
            yield_of_role,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cfe::morphogen::archetype;

    // A rank high enough to work any tier (≤ 3).
    fn full_reach() -> crate::soc::AlgebraicRank {
        crate::soc::AlgebraicRank {
            components: 3,
            cycles: 1,
            relations: vec![],
        }
    }
    // A rank too shallow to work a tier-2+ element.
    fn shallow_reach() -> crate::soc::AlgebraicRank {
        crate::soc::AlgebraicRank {
            components: 1,
            cycles: 0,
            relations: vec![],
        }
    }

    // An almanac element engineered to project strongly into a target axis: byte 0
    // (cohesion) high → Structure for a cohesion-reading metabolism.
    fn element(salt: u8) -> [u8; 32] {
        let mut e = *blake3::hash(&[salt]).as_bytes();
        e[2] = 0; // complexity tier 1 (workable at any reach)
        e
    }

    #[test]
    fn a_deficit_finds_an_emergent_recipe_in_the_almanac() {
        // A mineral (cohesion-structured) society deficient in Structure searches
        // its almanac and mints a recipe from the element that best fills it —
        // no ZoneKind, no authored table.
        let mineral = archetype(1);
        let almanac: Vec<[u8; 32]> = (0..8u8).map(element).collect();
        let recipe = discover(morphogen::STRUCT, &almanac, &mineral, &full_reach());
        assert!(
            recipe.is_some(),
            "the almanac offers a structure recipe to a mineral line"
        );
        let r = recipe.unwrap();
        assert_eq!(r.role, morphogen::STRUCT);
        assert_eq!(r.inputs.len(), 1, "a single-element recipe");
        assert!(r.yield_of_role > ratio(0, 1), "it delivers real structure");
    }

    #[test]
    fn a_role_no_element_can_fill_yields_no_recipe() {
        // Energy for a mineral line is drawn from the thermal flux, not from any
        // material axis — so no element projects into it. The search returns an
        // honest hole, never a forced fallback.
        let mineral = archetype(1);
        let almanac: Vec<[u8; 32]> = (0..8u8).map(element).collect();
        assert!(
            discover(morphogen::ENERGY, &almanac, &mineral, &full_reach()).is_none(),
            "no element fills the flux-fed energy role — an honest hole"
        );
    }

    #[test]
    fn advancement_is_topological_not_a_scalar_level() {
        // The level SCALAR, DISSOLVED. A deep (tier-2) element is unworkable until
        // the manifold's algebraic rank spans its complexity — reach is the gate.
        let mineral = archetype(1);
        let mut deep = element(1);
        deep[2] = 1; // tier 2 — needs reach ≥ 2
        let shallow_almanac = vec![deep];
        assert!(
            discover(
                morphogen::STRUCT,
                &shallow_almanac,
                &mineral,
                &shallow_reach()
            )
            .is_none(),
            "a low-genus society cannot yet work the deep element (topological gate, not a level)"
        );
        assert!(
            discover(morphogen::STRUCT, &shallow_almanac, &mineral, &full_reach()).is_some(),
            "once its reach deepens, the same element becomes workable"
        );
    }

    #[test]
    fn the_recipe_is_a_hashed_transformation_replayable_and_collision_free() {
        let mineral = archetype(1);
        let almanac: Vec<[u8; 32]> = (0..8u8).map(element).collect();
        let a = discover(morphogen::STRUCT, &almanac, &mineral, &full_reach()).unwrap();
        let b = discover(morphogen::STRUCT, &almanac, &mineral, &full_reach()).unwrap();
        assert_eq!(
            a.id, b.id,
            "the same discovery mints the same recipe — replay-exact"
        );
        // The same element into a different role is a different recipe (no collision).
        let single = vec![a.inputs[0]];
        let struct_recipe = discover(morphogen::STRUCT, &single, &mineral, &full_reach());
        let food_recipe = discover(morphogen::FOOD, &single, &mineral, &full_reach());
        if let (Some(s), Some(f)) = (struct_recipe, food_recipe) {
            assert_ne!(
                s.id, f.id,
                "element→Structure and element→Food are distinct recipes"
            );
        }
    }
}
