//! THE CAPACITY METER — `ext_x` and the D reading.
//!
//! Closes the first half of `18_TORSION.md` §6.2 ("`D` is not
//! measured"). Until this existed, Articles II, III and IV had no
//! operand and every capacity claim in the tree was unfalsifiable.
//!
//! **D is what SURVIVES reduction** (operator ruling 2026-08-01,
//! `17_CAPACITY` §4b, `18_TORSION` §5):
//!
//! ```text
//! D  =  ext_x( Π_G( Δ(Ω) ) )
//! ```
//!
//! The indices `(i, j, k, l)` and the arities `Delta{dim, fiber}` are
//! the CONTAINER the relation rides on — never the capacity. That
//! reading is STRUCK. A universe may carry enormous arity and near-zero
//! D when every difference it holds is pure frame, which is exactly the
//! dogmatic loop Article III prices at zero.
//!
//! **NEVER ONE SCALAR.** [`Extent`] reports per index. `18_TORSION`
//! §2b bars collapsing a directional quantity to a scalar aggregate
//! wherever the system computes; a single "capacity number" would be
//! the `C = x` substandard character (`17_CAPACITY` §2a.2) returning
//! under a new name. The extent says WHERE structure stands.
//!
//! **THE PAIRWISE LAW — the meter's one precondition.** The input must
//! be irreducibly pairwise (bonds, covenants, absorption debts), so
//! that non-transitive obligation triangles form genuine `H¹` cocycle
//! defects. A `Δ` built from node-local scalar differences —
//! `Δ_ij = p_j − p_i` for any potential `p` — is a PURE COBOUNDARY and
//! reduces to exactly zero everywhere:
//!
//! ```text
//! Δ'_ij = Δ_ij − Δ_0j + Δ_0i
//!       = (p_j − p_i) − (p_j − p_0) + (p_i − p_0)
//!       = 0
//! ```
//!
//! so such a source makes the meter read `D = 0` universally — a
//! working meter on a vacuous feed. `a_pure_coboundary_has_no_capacity`
//! pins this, and `the_equilibrium_anchor_is_a_coboundary` records that
//! the house's own canonical fixture is exactly that case.
//!
//! ALTITUDE: this is a READING, not a judge and not an objective
//! (`18_TORSION`, Ξ/ΔD are readings never objectives). It is `O(dim² ·
//! fiber)` over the committed relation and calls no search.

use crate::delta::Delta;
use crate::reduce::coboundary_reduce;
use num_traits::Zero;

/// The EXTENT of a relation, read PER INDEX — never collapsed to one
/// number.
///
/// Every field counts only entries that actually stand: a coordinate
/// that is zero contributes nothing to any of them. Over the
/// independent half of the tensor (`i < j`), since `Δ_ji = −Δ_ij`
/// carries no additional structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Extent {
    /// Entities standing in at least one surviving relation.
    pub entities: usize,
    /// Fiber layers carrying at least one surviving coordinate.
    pub layers: usize,
    /// Independent surviving coordinates: pairs `(i < j)` × layer.
    pub coordinates: usize,
}

impl Extent {
    /// The Article II reading: a vacuous extent claims no standing.
    /// `D = 0` — no open plane, no relation, nothing there to have
    /// standing (`18_TORSION` §3 Article II).
    pub fn is_vacuous(&self) -> bool {
        self.coordinates == 0
    }
}

/// `ext_x(Δ)` — the extent of each index over the entries that stand.
///
/// This is the RAW extent: it reads whatever `Δ` carries, frame
/// included. It is the "before" half of the meter. For capacity use
/// [`capacity`], which reduces first — frame contributes NOTHING to D.
pub fn ext_x(delta: &Delta) -> Extent {
    let dim = delta.dim();
    let fiber = delta.fiber();
    let mut entity_stands = vec![false; dim];
    let mut layer_stands = vec![false; fiber];
    let mut coordinates = 0usize;

    for i in 0..dim {
        for j in (i + 1)..dim {
            for (l, layer_stood) in layer_stands.iter_mut().enumerate() {
                if delta.entry(i, j, l).is_zero() {
                    continue;
                }
                coordinates += 1;
                entity_stands[i] = true;
                entity_stands[j] = true;
                *layer_stood = true;
            }
        }
    }

    Extent {
        entities: entity_stands.iter().filter(|s| **s).count(),
        layers: layer_stands.iter().filter(|s| **s).count(),
        coordinates,
    }
}

/// **`D` — THE CAPACITY READING.** `ext_x( Π_G( Δ ) )`: the extent of
/// the structure that SURVIVES coboundary reduction.
///
/// Frame artifacts contribute nothing; only structural content counts
/// (`EQUATIONS` D1). Because `Π_G` is idempotent, so is this reading —
/// `capacity(Δ) == capacity(coboundary_reduce(Δ))` — which is why `D`
/// is monotone by ALGEBRA rather than by decree (`EQUATIONS` D4).
pub fn capacity(delta: &Delta) -> Extent {
    ext_x(&coboundary_reduce(delta))
}
