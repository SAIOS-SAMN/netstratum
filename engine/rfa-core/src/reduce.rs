//! Coboundary reduction: project onto the canonical H¹ representative
//! of the OBSERVED relation graph, then integer-normalize.
//!
//! Two Deltas that differ only by a coboundary (a coordinate-frame
//! distortion) reduce to the same representative; a pure coboundary
//! reduces to zero. What survives is the structural content — the
//! cocycle class.
//!
//! # ACTIVE SUPPORT MASKING (DIRECTIVE 05, operator ruling 2026-08-01)
//!
//! A [`Delta`] is dense: every pair `(i, j)` has a stored coordinate,
//! and an UNOBSERVED relation is indistinguishable from an observed
//! zero. The previous reduction anchored every pair at vertex 0 —
//! `Δ'_ij = Δ_ij − Δ_0j + Δ_0i` — which reads those absent entries as
//! real zeros and is wrong in both directions:
//!
//! - it MANUFACTURES structure on unobserved pairs (a path `0—1—2` with
//!   no `0—2` relation reduced to a non-zero entry at `(1,2)`, so a
//!   tree — which carries no `H¹` at all — read as capacity 1);
//! - it DESTROYS structure by folding open cocycle defects into trivial
//!   gauge when the anchor vertex is itself unobserved, collapsing
//!   `D → 0` on small entity configurations.
//!
//! The correction: reduce strictly over the ACTIVE SUPPORT — the pairs
//! that carry an actual observed relation. Per fiber layer the support
//! is a graph; the reduction builds a spanning FOREST of it, sets the
//! potentials along the forest, and subtracts. Forest edges land on
//! exactly zero and the survivors are the independent cycles — which is
//! precisely `H¹` of the observed graph, of rank `E − V + C`. Pairs
//! outside the support are never evaluated and stay absent.
//!
//! An unobserved triple is NOT a zero-valued relation. It is not a
//! relation.

use crate::delta::Delta;
use cfe::ratio::Ratio;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};

/// The ACTIVE SUPPORT of a relation tensor — which `(i, j)` pairs carry
/// an observed relation, per fiber layer. Symmetric in `(i, j)`: a
/// relation and its antisymmetric twin are one observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Support {
    dim: usize,
    fiber: usize,
    /// `present[i][j][l]`, maintained symmetrically.
    present: Vec<Vec<Vec<bool>>>,
}

impl Support {
    /// An empty support over `dim` entities and `fiber` channels —
    /// nothing observed.
    pub fn none(dim: usize, fiber: usize) -> Self {
        Self { dim, fiber, present: vec![vec![vec![false; fiber]; dim]; dim] }
    }

    /// The support IMPLIED by a Delta's non-zero entries.
    ///
    /// This is the honest default when no observation record is
    /// available, and it is what [`coboundary_reduce`] uses. Callers
    /// that KNOW which relations exist — the kernel bridge knows, because
    /// a bond either exists or does not — should build the support
    /// explicitly with [`Support::mark`] and call
    /// [`coboundary_reduce_over`]. An observed relation whose value is
    /// exactly zero is real structure that this inference cannot see.
    pub fn of(delta: &Delta) -> Self {
        let mut support = Self::none(delta.dim(), delta.fiber());
        for i in 0..delta.dim() {
            for j in (i + 1)..delta.dim() {
                for l in 0..delta.fiber() {
                    if !delta.entry(i, j, l).is_zero() {
                        support.mark(i, j, l);
                    }
                }
            }
        }
        support
    }

    /// Record that the pair `(i, j)` carries an observed relation on
    /// channel `l`. Symmetric; the diagonal is never a relation and is
    /// ignored. Out-of-range indices are ignored — the mask is total.
    pub fn mark(&mut self, i: usize, j: usize, l: usize) {
        if i == j || i >= self.dim || j >= self.dim || l >= self.fiber {
            return;
        }
        self.present[i][j][l] = true;
        self.present[j][i][l] = true;
    }

    /// Is `(i, j)` observed on channel `l`?
    pub fn is_present(&self, i: usize, j: usize, l: usize) -> bool {
        if i >= self.dim || j >= self.dim || l >= self.fiber {
            return false;
        }
        self.present[i][j][l]
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn fiber(&self) -> usize {
        self.fiber
    }
}

/// Reduce `delta` to the canonical representative of its H¹ class,
/// inferring the active support from the non-zero entries.
///
/// Equivalent to [`coboundary_reduce_over`] with [`Support::of`]. On a
/// COMPLETE support graph this reproduces the historical vertex-0
/// anchoring exactly (the spanning tree rooted at 0 has potentials
/// `p_v = Δ_0v`, so `Δ'_ij = Δ_ij − Δ_0j + Δ_0i` as before).
pub fn coboundary_reduce(delta: &Delta) -> Delta {
    coboundary_reduce_over(delta, &Support::of(delta))
}

/// Reduce `delta` over an EXPLICIT active support (DIRECTIVE 05).
///
/// Phase 1 — projection over the support only. Per fiber layer, build a
/// spanning forest of the support graph (deterministic BFS: vertices
/// ascending, neighbours ascending), set `p_root = 0` and
/// `p_v = p_u + Δ_uv` along each forest edge, then write
/// `Δ'_ij = Δ_ij − (p_j − p_i)` for every SUPPORTED pair. Forest edges
/// land exactly on zero; the surviving entries are the independent
/// cycles of the observed graph. Unsupported pairs are never evaluated
/// and stay zero-as-absent.
///
/// Phase 2 — integer normalization over the surviving entries: multiply
/// by the lcm of their denominators, divide by the gcd of their
/// numerators. Every coordinate becomes an integer and the
/// representative is scale-canonical. A wholly-zero result passes
/// through unchanged.
///
/// Pure and total: no panic on any well-formed `(delta, support)` pair,
/// including mismatched arities (out-of-range marks read as absent).
pub fn coboundary_reduce_over(delta: &Delta, support: &Support) -> Delta {
    let dim = delta.dim();
    let fiber = delta.fiber();
    // One spanning forest per fiber layer: each layer is its own
    // 1-cochain over its own observed graph.
    let potentials: Vec<Vec<Ratio>> =
        (0..fiber).map(|l| forest_potentials(delta, support, l)).collect();

    let mut projected = Delta::zero(dim, fiber);
    for i in 0..dim {
        for j in (i + 1)..dim {
            let coords: Vec<Ratio> = (0..fiber)
                .map(|l| match support.is_present(i, j, l) {
                    // Trivialize against the forest potential. A forest
                    // edge lands on exactly zero; a cycle edge keeps
                    // its defect.
                    true => delta.entry(i, j, l) - (&potentials[l][j] - &potentials[l][i]),
                    // Not a relation. Never evaluated, never a zero the
                    // algebra can mistake for an observation.
                    false => Ratio::zero(),
                })
                .collect();
            projected.set_antisym(i, j, coords);
        }
    }

    // Integer normalization over what actually survived.
    let mut lcm_denom = BigInt::one();
    let mut gcd_numer = BigInt::zero();
    for i in 0..dim {
        for j in (i + 1)..dim {
            for l in 0..fiber {
                let e = projected.entry(i, j, l);
                if e.is_zero() {
                    continue;
                }
                lcm_denom = lcm_denom.lcm(e.denom());
                gcd_numer = gcd_numer.gcd(&e.numer().abs());
            }
        }
    }
    if gcd_numer.is_zero() {
        // Every surviving entry is zero — nothing to normalize.
        return projected;
    }
    let scale = Ratio::new(lcm_denom, gcd_numer);
    let mut normalized = Delta::zero(dim, fiber);
    for i in 0..dim {
        for j in (i + 1)..dim {
            let coords: Vec<Ratio> =
                (0..fiber).map(|l| projected.entry(i, j, l) * &scale).collect();
            normalized.set_antisym(i, j, coords);
        }
    }
    normalized
}

/// The potentials that trivialize a spanning forest of the support
/// graph on one fiber layer.
///
/// Deterministic: components are opened by ascending root vertex and
/// explored breadth-first with ascending neighbours, so the same
/// `(delta, support, l)` always yields the same forest and the same
/// representative. Vertices carrying no observed relation keep
/// potential zero and are never read.
fn forest_potentials(delta: &Delta, support: &Support, l: usize) -> Vec<Ratio> {
    let dim = delta.dim();
    let mut potentials = vec![Ratio::zero(); dim];
    let mut seen = vec![false; dim];

    for root in 0..dim {
        if seen[root] {
            continue;
        }
        // Skip isolated vertices: opening a component at a vertex with
        // no observed relation would seat a potential nothing reads.
        if !(0..dim).any(|other| support.is_present(root, other, l)) {
            continue;
        }
        seen[root] = true;
        potentials[root] = Ratio::zero();
        let mut frontier = vec![root];
        while let Some(u) = frontier.pop() {
            for v in 0..dim {
                if v == u || seen[v] || !support.is_present(u, v, l) {
                    continue;
                }
                // The forest edge is trivialized: p_v = p_u + Δ_uv, so
                // Δ'_uv = Δ_uv − (p_v − p_u) = 0 exactly.
                seen[v] = true;
                potentials[v] = &potentials[u] + delta.entry(u, v, l);
                frontier.insert(0, v);
            }
        }
    }
    potentials
}
