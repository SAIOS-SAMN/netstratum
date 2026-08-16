//! **SOC — THE SPECIES MANIFOLD & THE OPEN WORKBENCH.**
//!
//! SOC-1 opened `build_step`'s surplus phase into a stochastic draw over an
//! option surface (was a fixed `industrial.or(refine).or(settlement)`
//! priority). SOC-2 (operator directive 2026-08-16, *the algebra of affinity*)
//! replaces the scalar `match disposition` weight — a dimensional collapse — with
//! a genuine **homological pairing** against the species' own metabolic manifold.
//!
//! **The manifold `M_civ`.** A species' nature is not a tensor of scalars (a
//! flat vector space cannot describe a *hole* — a metabolism a clade cannot run).
//! It is a chain complex kept graded, never flattened. Here `M_civ` is the
//! species' **metabolic graph**: `C₀` = the resource traits the clade can touch
//! (an environment source `⊥` plus every resource reachable from it), `C₁` = the
//! conversion pathways its metabolism can run (`industry_relevant`, input→output
//! from `cost`/`zone_yield`). Its boundary `∂₁` is exact-integer, and its
//! homology is read off the substrate's Smith Normal Form — `Ker(∂₁)`, `Im(∂₁)`,
//! and the ranks — never a scalar comparison.
//!
//! **The mapping operator `𝒯`.** An infrastructure project `s` requires a
//! pathway from an input trait to an output trait — a class `[ω]` in `H₀(M_civ)`,
//! namely `[1_out − 1_in]`. The affinity is the operator's step function on the
//! vanishing of that obstruction:
//!
//! ```text
//!   w_s = 𝒯([ω]) = { W_base · genus(M_civ)   if [ω] = 0  (native bounding)
//!                   { 0                        if [ω] ≠ 0  (a hole in the phase space)
//! ```
//!
//! `[ω] = 0` iff `1_out − 1_in ∈ Im(∂₁)` — input and output are both present and
//! metabolically **connected**: the society's own geometry bounds the
//! requirement. `[ω] ≠ 0` iff a trait is absent or unreachable (a plasmic line
//! reaching for the iron forge: `Alloy` is not in its complex, the pairing
//! returns a non-trivial harmonic representative, `w_s → 0`) — the building falls
//! out of the partition `Z` with **no boolean override and no species table**.
//!
//! **Intelligence is genus.** `genus = Σ βₖ` (`β₀` components + `β₁` independent
//! metabolic cycles). A recycling (regenerative) metabolism closes loops — its
//! `β₁` rises — so a topologically richer society scores its native
//! infrastructure higher, the geometric truth that a system cannot deploy a
//! structure more complex than its own topological degree. `genus` is common to
//! every native candidate in one beat, so it cancels in the draw `P(s)=w_s/Z`
//! (natives are equiprobable, non-natives excluded) and surfaces where it
//! belongs: the reach/depth coupling (`bridge::read_entangled`).
//!
//! **Deterministic niche construction.** The draw among the native set is seeded
//! by the line's `chronicle_digest`, domain-tagged `b"workbench"` to decorrelate
//! from the Mind's `b"instinct"` draw. Replay-exact, `ℤ/ℚ` throughout.

use cfe::dop::{metabolic_harvest, Resource, ZoneKind};
use num_bigint::BigInt;
use substrate::snf::{smith_normal_form, Smith};

/// The base affinity a natively-bounded building carries before the genus scale.
/// A bare integer (no `ratio` dial — the ratchet is unmoved); the draw needs only
/// the proportions, and this common base cancels in `P(s)=w_s/Z`.
const W_BASE: u64 = 1;

/// The universe of metabolic conversions, each a directed pathway `input → output`
/// (`None` input = drawn from the environment source `⊥`). Assembled from the
/// standing tables — `ZoneKind::cost` names the input, `yield_per_tick` the
/// output — so a change to the economy's chemistry flows here with no restatement.
/// `Habitat`/`Regenerator` restore the metabolism's RATION (species-relative), so
/// they are resolved against `metabolic_harvest`, not listed here.
fn conversions() -> [(ZoneKind, Option<Resource>, Resource); 6] {
    [
        (ZoneKind::Extractor, None, Resource::Iron),
        (ZoneKind::Smelter, Some(Resource::Iron), Resource::Alloy),
        (ZoneKind::Synthesizer, Some(Resource::Alloy), Resource::Diamond),
        (ZoneKind::Resonator, Some(Resource::Alloy), Resource::Crystal),
        (ZoneKind::Encasement, Some(Resource::Iron), Resource::Joule),
        (ZoneKind::CryoArray, Some(Resource::Iron), Resource::Sink),
    ]
}

/// The pathway a buildable REQUIRES: `(input trait, output trait)`. A source
/// building draws its output from the environment (`None` input). Buildings with
/// no metabolic conversion (civic/refinery infrastructure) return `None` — they
/// carry no thermodynamic obstruction and are natively bounded by definition.
fn requirement(kind: ZoneKind, ration: Resource) -> Option<(Option<Resource>, Resource)> {
    match kind {
        ZoneKind::Extractor => Some((None, Resource::Iron)),
        ZoneKind::Habitat => Some((None, ration)),
        ZoneKind::Regenerator => Some((None, ration)),
        ZoneKind::Smelter => Some((Some(Resource::Iron), Resource::Alloy)),
        ZoneKind::Synthesizer => Some((Some(Resource::Alloy), Resource::Diamond)),
        ZoneKind::Resonator => Some((Some(Resource::Alloy), Resource::Crystal)),
        ZoneKind::Encasement => Some((Some(Resource::Iron), Resource::Joule)),
        ZoneKind::CryoArray => Some((Some(Resource::Iron), Resource::Sink)),
        _ => None, // civic infrastructure — no metabolic pathway to obstruct
    }
}

/// **`M_civ` — THE SPECIES METABOLIC COMPLEX.** Its vertices (`⊥` + the reachable
/// resource traits), its exact boundary `∂₁` factored to Smith Normal Form once,
/// and its genus. Every affinity query pairs against this without refactoring.
pub struct SpeciesManifold {
    /// Vertex 0 is the environment source `⊥`; the rest are the reachable traits.
    traits: Vec<Resource>,
    /// The SNF of `∂₁` (rows = |V|, cols = |E|) — `Im`, `Ker`, and rank in one.
    boundary: Smith,
    /// `genus = Σ βₖ = β₀ + β₁` — the society's topological degree.
    pub genus: u64,
    ration: Resource,
}

impl SpeciesManifold {
    /// Assemble the complex for a species. `metabolism` (u8) sets the ration
    /// trait; `metabolism_name` gates which conversions are relevant (a plasmic
    /// line never forges); `recycles` (a regenerative economy) adds the return
    /// pathway that closes a metabolic loop and raises `β₁`.
    pub fn of(metabolism: u8, metabolism_name: &str, recycles: bool) -> Self {
        let ration = metabolic_harvest(metabolism).0;

        // Reachability from ⊥: the traits the clade can actually touch. Iron (the
        // universal extractor) and the ration seed it; relevant conversions
        // extend it to a fixpoint. A conversion whose input is unreachable, or
        // that its metabolism cannot run, never contributes its output — that is
        // how a hole (a missing trait) forms, with no exclusion table.
        let mut reachable: Vec<Resource> = vec![Resource::Iron];
        if !reachable.contains(&ration) {
            reachable.push(ration);
        }
        loop {
            let mut grew = false;
            for (kind, input, output) in conversions() {
                if !crate::industry_relevant(metabolism_name, kind) {
                    continue;
                }
                let input_ready = match input {
                    None => true,
                    Some(r) => reachable.contains(&r),
                };
                if input_ready && !reachable.contains(&output) {
                    reachable.push(output);
                    grew = true;
                }
            }
            if !grew {
                break;
            }
        }
        reachable.sort_by_key(|r| *r as u8);

        // The edges: every relevant conversion whose endpoints are reachable,
        // plus the ration source edge (Habitat), plus the recycling return edge
        // (ration → ⊥) when the economy is regenerative.
        let idx = |r: Resource, traits: &[Resource]| -> usize {
            traits.iter().position(|t| *t == r).map(|p| p + 1).unwrap_or(0)
        };
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for (kind, input, output) in conversions() {
            if !crate::industry_relevant(metabolism_name, kind) {
                continue;
            }
            let ok = match input {
                None => true,
                Some(r) => reachable.contains(&r),
            };
            if ok && reachable.contains(&output) {
                let from = input.map(|r| idx(r, &reachable)).unwrap_or(0);
                edges.push((from, idx(output, &reachable)));
            }
        }
        edges.push((0, idx(ration, &reachable))); // the habitat source
        if recycles {
            edges.push((idx(ration, &reachable), 0)); // the loop the recycler closes
        }

        // ∂₁ : C₁ → C₀, an exact integer matrix (rows = |V|, cols = |E|); column
        // e carries −1 at its tail and +1 at its head. SNF gives Im/Ker/rank.
        let n_vertices = reachable.len() + 1;
        let mut boundary_matrix: Vec<Vec<BigInt>> =
            vec![vec![BigInt::from(0); edges.len()]; n_vertices];
        for (e, (from, to)) in edges.iter().enumerate() {
            boundary_matrix[*from][e] += BigInt::from(-1);
            boundary_matrix[*to][e] += BigInt::from(1);
        }
        let boundary = smith_normal_form(&boundary_matrix);
        let rank = boundary.rank();
        // A graph carries no homological torsion: β₀ = |V| − rank, β₁ = |E| − rank.
        let beta0 = (n_vertices - rank) as u64;
        let beta1 = (edges.len() - rank) as u64;
        let genus = beta0 + beta1;

        SpeciesManifold { traits: reachable, boundary, genus, ration }
    }

    /// The vertex index of a trait (`None` = the environment source `⊥` at 0),
    /// or `None` if the trait is absent from the complex (unreachable — a hole).
    fn vertex(&self, trait_: Option<Resource>) -> Option<usize> {
        match trait_ {
            None => Some(0),
            Some(r) => self.traits.iter().position(|t| *t == r).map(|p| p + 1),
        }
    }

    /// **THE HOMOLOGICAL PAIRING.** `[ω] = [1_out − 1_in] = 0` in `H₀` iff the
    /// class lies in `Im(∂₁)` — input and output present and connected. Exact
    /// membership via the SNF witnesses: in the `U` basis, `b ∈ Im(∂₁)` iff each
    /// coordinate below the rank vanishes and each on-rank coordinate is divisible
    /// by its invariant factor (all `1` for a torsion-free graph, so the test is
    /// exactly connectivity — computed, not assumed).
    pub fn bounds(&self, kind: ZoneKind) -> bool {
        let Some((input, output)) = requirement(kind, self.ration) else {
            return true; // civic infrastructure carries no metabolic obstruction
        };
        let (Some(in_v), Some(out_v)) = (self.vertex(input), self.vertex(Some(output))) else {
            return false; // a trait is absent — the pathway cannot even be expressed
        };
        if in_v == out_v {
            return true; // the trait bounds itself (a source into its own store)
        }
        let rows = self.boundary.rows;
        let mut b = vec![BigInt::from(0); rows];
        b[out_v] += BigInt::from(1);
        b[in_v] += BigInt::from(-1);
        let ub = matvec(&self.boundary.u, &b);
        let factors = self.boundary.invariant_factors();
        for (i, coord) in ub.iter().enumerate() {
            match factors.get(i) {
                Some(d) => {
                    if (coord % d) != BigInt::from(0) {
                        return false;
                    }
                }
                None => {
                    if *coord != BigInt::from(0) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// **THE MAPPING OPERATOR `𝒯`.** `W_base · genus` for a natively-bounded
    /// building, `0` for an obstructed one. The whole SOC-1 scalar `match
    /// disposition` collapses to this pairing.
    pub fn affinity(&self, kind: ZoneKind) -> u64 {
        if self.bounds(kind) {
            W_BASE.saturating_mul(self.genus)
        } else {
            0
        }
    }
}

/// Exact integer `A · x`.
fn matvec(a: &[Vec<BigInt>], x: &[BigInt]) -> Vec<BigInt> {
    a.iter()
        .map(|row| {
            row.iter()
                .zip(x)
                .fold(BigInt::from(0), |acc, (aij, xj)| acc + aij * xj)
        })
        .collect()
}

/// **THE WORKBENCH CHOOSER** — the civic partition-function draw. Over the
/// candidate set `S_valid` (viability resolved upstream by `build_step`'s gates),
/// weight each by the manifold's `𝒯` affinity and sample `P(s)=w_s/Z` from the
/// line's `chronicle_digest`. A non-native building weighs `0` — excluded from
/// `Z` with no boolean override. `None` only when no candidate is natively
/// bounded (the beat builds nothing, exactly as the old priority returned `None`).
pub fn choose_build(
    candidates: &[ZoneKind],
    manifold: &SpeciesManifold,
    seed: [u8; 32],
) -> Option<ZoneKind> {
    let total: u64 = candidates
        .iter()
        .fold(0u64, |sum, k| sum.saturating_add(manifold.affinity(*k)));
    if total == 0 {
        return None;
    }
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed);
    hasher.update(b"workbench");
    let block = *hasher.finalize().as_bytes();
    let roll = u64::from_le_bytes(block[0..8].try_into().expect("32 >= 8")) % total;
    let mut acc = 0u64;
    for kind in candidates {
        acc = acc.saturating_add(manifold.affinity(*kind));
        if roll < acc {
            return Some(*kind);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed(n: u8) -> [u8; 32] {
        *blake3::hash(&[n]).as_bytes()
    }

    // A balanced (biomass, metabolism 2) line runs the full forge chain.
    fn balanced() -> SpeciesManifold {
        SpeciesManifold::of(2, "aqueous", false)
    }

    #[test]
    fn a_native_pathway_pairs_to_zero_and_scores_positive() {
        // THE 𝒯 LAW: a balanced line touches Iron and Alloy and the Iron→Alloy
        // conversion is relevant, so the forge pathway is bounded — its class
        // vanishes, its affinity is W_base·genus > 0.
        let m = balanced();
        assert!(m.bounds(ZoneKind::Smelter), "the iron forge is native to a balanced line");
        assert!(m.affinity(ZoneKind::Smelter) > 0);
        // The sources are always native — the environment bounds them.
        assert!(m.bounds(ZoneKind::Extractor) && m.bounds(ZoneKind::Habitat));
    }

    #[test]
    fn a_plasmic_line_is_homologically_excluded_from_the_iron_forge() {
        // THE HOLE, WITNESSED (the operator's lithophage): a plasmic metabolism
        // cannot run the Smelter/Synthesizer, so Alloy and Diamond are never
        // reachable — they are ABSENT from its complex. The forge pathway cannot
        // even be expressed; the pairing is a non-trivial obstruction; 𝒯 → 0.
        // No boolean gate, no species table — a topological fact.
        let plasmic = SpeciesManifold::of(3, "plasmic", false);
        assert!(!plasmic.bounds(ZoneKind::Smelter), "the iron forge is a hole for a plasmic line");
        assert_eq!(plasmic.affinity(ZoneKind::Smelter), 0, "obstructed → excluded from Z");
        assert_eq!(plasmic.affinity(ZoneKind::Synthesizer), 0, "the deeper forge too");
        // Yet its own sources remain native — it still settles and extracts.
        assert!(plasmic.affinity(ZoneKind::Extractor) > 0);
    }

    #[test]
    fn recycling_raises_the_genus_a_regenerative_society_is_topologically_richer() {
        // INTELLIGENCE IS GENUS: closing the ration loop (a regenerative economy)
        // adds an independent cycle β₁ — the manifold's genus rises. The same
        // clade, recycling, is a topologically more complex society.
        let extractive = SpeciesManifold::of(2, "aqueous", false);
        let regenerative = SpeciesManifold::of(2, "aqueous", true);
        assert!(
            regenerative.genus > extractive.genus,
            "the recycling loop lifts the genus ({} > {})",
            regenerative.genus,
            extractive.genus
        );
    }

    #[test]
    fn the_draw_excludes_obstructions_and_stays_replayable() {
        let plasmic = SpeciesManifold::of(3, "plasmic", false);
        // Offered the forge and a settlement, a plasmic line can only draw the
        // settlement — the forge is weight 0, out of the partition.
        let surface = [ZoneKind::Smelter, ZoneKind::Extractor];
        for n in 0u8..16 {
            assert_eq!(
                choose_build(&surface, &plasmic, seed(n)),
                Some(ZoneKind::Extractor),
                "the obstructed forge is never drawn"
            );
        }
        // And the draw is replayable: same digest, same building.
        let m = balanced();
        let full = [ZoneKind::Smelter, ZoneKind::Extractor, ZoneKind::Habitat];
        assert_eq!(choose_build(&full, &m, seed(5)), choose_build(&full, &m, seed(5)));
    }

    #[test]
    fn among_natives_the_workbench_is_open() {
        // With every candidate native (equal W_base·genus), the draw spreads
        // across the surface as the digest varies — the workbench is genuinely
        // open, not a fixed priority.
        let m = balanced();
        let surface = [ZoneKind::Smelter, ZoneKind::Extractor, ZoneKind::Habitat];
        let mut seen: Vec<ZoneKind> = Vec::new();
        for n in 0u8..48 {
            if let Some(k) = choose_build(&surface, &m, seed(n)) {
                if !seen.contains(&k) {
                    seen.push(k);
                }
            }
        }
        assert!(seen.len() >= 2, "the draw explores more than one native building");
    }
}
