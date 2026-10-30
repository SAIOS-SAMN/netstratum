//! Coboundary reduction: project onto the canonical H¹ representative,
//! then integer-normalize.
//!
//! Two Deltas that differ only by a coboundary (a coordinate-frame
//! distortion) reduce to the same representative; a pure coboundary
//! reduces to zero. What survives is the structural content — the
//! cocycle class.

use crate::delta::Delta;
use cfe::ratio::Ratio;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};

/// Which relations are present — the sparse complex's edge set
/// (operator ruling 2026-08-01: *an absent relation is absent, not
/// zero*).
///
/// [`coboundary_reduce`] assumes a complete relation: every pair is an
/// edge, and it anchors the whole reduction at vertex 0. On a complete
/// graph every cocycle is a coboundary, so the reduction annihilates
/// everything and `D ≡ 0` — measured in `kernel/tests/bridge_pins.rs`.
/// Topology needs absent edges: it is the cycles they leave unfilled
/// that carry `H¹`.
///
/// Symmetric by construction — a relation present from `i` to `j` is
/// present from `j` to `i`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Support {
    dim: usize,
    fiber: usize,
    present: Vec<bool>,
}

impl Support {
    /// An empty complex: entities, coordinates, and no edges at all.
    pub fn new(dim: usize, fiber: usize) -> Self {
        Self { dim, fiber, present: vec![false; dim * dim * fiber] }
    }

    fn at(&self, i: usize, j: usize, l: usize) -> usize {
        (i * self.dim + j) * self.fiber + l
    }

    /// Mark the relation between `i` and `j` on coordinate `l` present.
    /// The diagonal is never an edge (`Δ_ii = 0` is not a relation).
    pub fn connect(&mut self, i: usize, j: usize, l: usize) {
        if i == j || i >= self.dim || j >= self.dim || l >= self.fiber {
            return;
        }
        let (a, b) = (self.at(i, j, l), self.at(j, i, l));
        self.present[a] = true;
        self.present[b] = true;
    }

    /// Is this relation an edge of the complex?
    pub fn has(&self, i: usize, j: usize, l: usize) -> bool {
        match i >= self.dim || j >= self.dim || l >= self.fiber {
            true => false,
            false => self.present[self.at(i, j, l)],
        }
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn fiber(&self) -> usize {
        self.fiber
    }
}

/// One independent cycle's holonomy — the charge a loop carries that no
/// frame can explain. `(i, j)` is the edge that closes the cycle, `l`
/// its coordinate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Holonomy {
    pub i: usize,
    pub j: usize,
    pub l: usize,
    pub value: Ratio,
}

/// One layer's spanning forest — grow a forest over the present edges of
/// coordinate `l`, assigning each vertex the potential its forest path
/// accumulates (roots at zero). Returns the per-vertex potentials
/// (`None` for a vertex the layer never touches — no edge, no potential)
/// and the forest-edge mask (`in_forest[i*dim + j]`).
///
/// This is the FRAME: a spanning forest is a coboundary by construction —
/// each forest edge is exactly `p_j − p_i` — so the potentials explain
/// away precisely what a coordinate frame should, and what they cannot
/// explain is every remaining edge's holonomy. Both [`holonomies`] (the
/// charge each non-forest edge carries) and [`forest_potentials`] (the
/// reconciled worldview itself) read this one walk, so they can never
/// disagree about the frame. Deterministic: the DFS pops the highest
/// index first and the walk is a pure function of `(delta, support, l)`.
fn spanning_forest(
    delta: &Delta,
    support: &Support,
    l: usize,
    dim: usize,
) -> (Vec<Option<Ratio>>, Vec<bool>) {
    let mut potential: Vec<Option<Ratio>> = vec![None; dim];
    let mut in_forest = vec![false; dim * dim];
    for root in 0..dim {
        if potential[root].is_some() {
            continue;
        }
        potential[root] = Some(Ratio::zero());
        let mut frontier = vec![root];
        while let Some(here) = frontier.pop() {
            let base = potential[here].clone().unwrap_or_else(Ratio::zero);
            for next in 0..dim {
                if next == here || !support.has(here, next, l) || potential[next].is_some() {
                    continue;
                }
                potential[next] = Some(base.clone() + delta.entry(here, next, l));
                in_forest[here * dim + next] = true;
                in_forest[next * dim + here] = true;
                frontier.push(next);
            }
        }
    }
    (potential, in_forest)
}

/// Support-aware reduction — reduce against a spanning forest of the
/// present edges rather than against vertex 0.
///
/// This is the same act [`coboundary_reduce`] performs, done honestly on
/// a sparse complex. A spanning forest is a coboundary by construction:
/// give each root potential zero and walk, and every forest edge is
/// exactly `p_j − p_i`. So the forest is the FRAME, and it explains
/// away precisely what a frame should. What it cannot explain is every
/// remaining edge — each one closes exactly one independent cycle, and
/// the amount by which it disagrees with the forest's potentials is
/// that cycle's holonomy.
///
/// A non-zero holonomy is structure no coordinate change can remove:
/// it is `∮ ≠ 0`, the topological charge of `24_FORM` §2d's sign law and
/// holonomic pole, and it is what `D` counts.
///
/// Anchoring at vertex 0 was the degenerate case of this that assumed
/// one component and a complete graph. Exact rational throughout; no
/// float, no root.
pub fn holonomies(delta: &Delta, support: &Support) -> Vec<Holonomy> {
    let dim = delta.dim().min(support.dim());
    let fiber = delta.fiber().min(support.fiber());
    let mut found = Vec::new();

    for l in 0..fiber {
        let (potential, in_forest) = spanning_forest(delta, support, l, dim);
        // Every present edge outside the forest closes one independent
        // cycle. Its holonomy is what the forest's frame cannot explain.
        for i in 0..dim {
            for j in (i + 1)..dim {
                if !support.has(i, j, l) || in_forest[i * dim + j] {
                    continue;
                }
                let (Some(from), Some(to)) = (&potential[i], &potential[j]) else {
                    continue;
                };
                let charge = delta.entry(i, j, l).clone() - (to.clone() - from.clone());
                if !charge.is_zero() {
                    found.push(Holonomy { i, j, l, value: charge });
                }
            }
        }
    }
    found
}

/// **The reconciled worldview** — the spanning-forest potential of every
/// vertex, per coordinate. `[l][i]` is the potential coordinate `l`
/// assigns vertex `i`: the accumulated relation along the forest path
/// from its component root (rooted at zero), and zero for any vertex the
/// coordinate never touches.
///
/// This is the coherent frame the sparse reduction fits to the data —
/// the one globally-consistent reading of a web of pairwise relations
/// that a resolve produces. Where the pairwise relations do compose
/// (`C = 0`), the potential explains them entirely; where they do not,
/// [`holonomies`] carries exactly the irreducible remainder the
/// potential could not. The two are the two halves of the one resolve,
/// read off the same [`spanning_forest`] walk.
///
/// Exact rational throughout; no float, no root. A `None` potential
/// (isolated vertex) reads as zero — it stands at its own ground,
/// relating to nothing.
pub fn forest_potentials(delta: &Delta, support: &Support) -> Vec<Vec<Ratio>> {
    let dim = delta.dim().min(support.dim());
    let fiber = delta.fiber().min(support.fiber());
    (0..fiber)
        .map(|l| {
            let (potential, _) = spanning_forest(delta, support, l, dim);
            potential
                .into_iter()
                .map(|p| p.unwrap_or_else(Ratio::zero))
                .collect()
        })
        .collect()
}

/// Reduce `delta` to the canonical representative of its H¹ class.
///
/// **Assumes a complete relation.** Anchoring at vertex 0 is only
/// lawful when every pair is an edge and the graph is one component;
/// on a sparse complex use [`holonomies`], which anchors per component
/// against a spanning forest. Kept as the dense case it always was.
///
/// Phase 1 — projection: for every pair `i < j` and coordinate `l`,
/// `Δ'_ij,l = Δ_ij,l − Δ_0j,l + Δ_0i,l` — subtract the coboundary
/// induced by anchoring every relation at vertex 0. Every relation
/// touching vertex 0 lands exactly on zero; the surviving entries are
/// the frame-free content.
///
/// Phase 2 — integer normalization: multiply through by the lcm of all
/// denominators, then divide by the gcd of all numerators. Every
/// coordinate becomes an integer (denominator 1) and the representative
/// is scale-canonical. The zero tensor passes through unchanged.
pub fn coboundary_reduce(delta: &Delta) -> Delta {
    let dim = delta.dim();
    let fiber = delta.fiber();
    let mut reduced = Delta::zero(dim, fiber);

    for i in 0..dim {
        for j in (i + 1)..dim {
            let coords: Vec<Ratio> = (0..fiber)
                .map(|l| delta.entry(i, j, l) - delta.entry(0, j, l) + delta.entry(0, i, l))
                .collect();
            reduced.set_antisym(i, j, coords);
        }
    }

    // Integer normalization over the independent entries (i < j).
    let mut lcm_denom = BigInt::one();
    let mut gcd_numer = BigInt::zero();
    for i in 0..dim {
        for j in (i + 1)..dim {
            for l in 0..fiber {
                let e = reduced.entry(i, j, l);
                if e.is_zero() {
                    continue;
                }
                lcm_denom = lcm_denom.lcm(e.denom());
                gcd_numer = gcd_numer.gcd(&e.numer().abs());
            }
        }
    }
    if gcd_numer.is_zero() {
        // Every entry is zero — nothing to normalize.
        return reduced;
    }

    let scale = Ratio::new(lcm_denom, gcd_numer);
    let mut normalized = Delta::zero(dim, fiber);
    for i in 0..dim {
        for j in (i + 1)..dim {
            let coords: Vec<Ratio> = (0..fiber)
                .map(|l| reduced.entry(i, j, l) * &scale)
                .collect();
            normalized.set_antisym(i, j, coords);
        }
    }
    normalized
}
