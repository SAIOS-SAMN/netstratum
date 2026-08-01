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
use cfe::ratio::Ratio;
use num_traits::{Signed, Zero};

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

/// **THE DIVERGENCE** — `div(Δ)_i,l = Σ_j Δ_ij,l`, indexed `[i][l]`.
///
/// One pass, no basepoint, no spanning structure, no chosen basis. It
/// is the adjoint of the gradient, so it detects the gradient component
/// exactly: `Δ` has no gradient part iff `div(Δ) = 0` everywhere.
///
/// **THIS IS THE SIGNED READING, and the sign is the law** (18 §2d):
///
/// - `div_i > 0` — an EGRESS. The vertex emits more than it absorbs:
///   a source, `S⁺_out`, "strictly positive divergence, emission only".
/// - `div_i < 0` — an INGRESS. A trapped interior, which §2d FORBIDS:
///   the singularity is a white hole, never a black one.
/// - `div_i = 0` — balanced. Whatever the vertex carries is holonomic,
///   `S⁺_hol`: it returns what it takes and the content is circulation.
///
/// Divergence always sums to zero over the vertices (antisymmetry), so
/// a source anywhere implies an ingress somewhere: the books close.
pub fn divergence(delta: &Delta) -> Vec<Vec<Ratio>> {
    let dim = delta.dim();
    let fiber = delta.fiber();
    let mut out = vec![vec![Ratio::zero(); fiber]; dim];
    for (i, row) in out.iter_mut().enumerate() {
        for j in 0..dim {
            for (l, slot) in row.iter_mut().enumerate() {
                *slot += delta.entry(i, j, l);
            }
        }
    }
    out
}

/// The egress census of a relation — how many vertices SOURCE, how many
/// are trapped, how many are balanced. Counted over any fiber layer
/// carrying a non-zero divergence at that vertex.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Egress {
    /// `S⁺_out` candidates: `div > 0` on some layer.
    pub sources: usize,
    /// Trapped interior: `div < 0` on some layer. §2d forbids these.
    pub trapped: usize,
    /// `div = 0` on every layer — purely holonomic at this vertex.
    pub balanced: usize,
}

impl Egress {
    /// §2d's non-trapping law: a lawful egress has NO trapped interior.
    pub fn is_white_hole(&self) -> bool {
        self.trapped == 0
    }
}

/// The egress reading (see [`divergence`]).
pub fn egress(delta: &Delta) -> Egress {
    let mut census = Egress { sources: 0, trapped: 0, balanced: 0 };
    for row in divergence(delta) {
        let positive = row.iter().any(|d| d.is_positive());
        let negative = row.iter().any(|d| d.is_negative());
        match (positive, negative) {
            (false, false) => census.balanced += 1,
            _ => {
                if positive {
                    census.sources += 1;
                }
                if negative {
                    census.trapped += 1;
                }
            }
        }
    }
    census
}

/// **THE CIRCULATION — the POSITIVE construction of capacity.**
///
/// `Δ = grad(p) ⊕ circulation`, the discrete Hodge split. This function
/// returns the circulation summand, and it is built rather than left
/// over: instead of stripping a frame and counting the residue, it
/// PROJECTS onto the divergence-free subspace, where the structural
/// content actually lives.
///
/// Over the complete relation graph the solve is closed-form and exact.
/// The Laplacian is `L = n·I − J`, so on the zero-sum space
/// `p_i = −div_i / n`, and `circulation = Δ − grad(p)`. `O(n²)`, no
/// elimination, no anchor.
///
/// **Why this replaces the anchored representative.** `Π_G`'s
/// `Δ_ij − Δ_0j + Δ_0i` gauge-FIXES at vertex 0 — a spanning star, a
/// chosen basis — so its output depends on which vertex is labelled 0,
/// and Article I voids any reading that requires a chosen basis. The
/// circulation is UNIQUE: `p` is determined only up to a constant, but
/// `grad(p)` is not, so the summand returned here is canonical. Same
/// class, basis-free representative.
pub fn circulation(delta: &Delta) -> Delta {
    let dim = delta.dim();
    let fiber = delta.fiber();
    if dim == 0 {
        return Delta::zero(dim, fiber);
    }
    let div = divergence(delta);
    let n = Ratio::from_integer(dim.into());
    // p_i = −div_i / n  (the zero-sum potential whose gradient carries
    // exactly Δ's divergence).
    let potential: Vec<Vec<Ratio>> = div
        .iter()
        .map(|row| row.iter().map(|d| -(d / &n)).collect())
        .collect();

    let mut out = Delta::zero(dim, fiber);
    for i in 0..dim {
        for j in (i + 1)..dim {
            let coords: Vec<Ratio> = (0..fiber)
                .map(|l| delta.entry(i, j, l) - (&potential[j][l] - &potential[i][l]))
                .collect();
            out.set_antisym(i, j, coords);
        }
    }
    out
}

/// **`D` — THE CAPACITY READING**, built positively: the extent of the
/// CIRCULATION.
///
/// Frame artifacts contribute nothing; only structural content counts
/// (`EQUATIONS` D1). The circulation is divergence-free by construction
/// and canonical, so this reading needs no basepoint and cannot be
/// moved by relabelling the entities.
///
/// Idempotent, because the circulation of a circulation is itself.
pub fn capacity(delta: &Delta) -> Extent {
    ext_x(&circulation(delta))
}

/// **THE LAWFUL BASE GROUP** — does this congruence preserve the
/// Hodge split, and therefore the capacity reading?
///
/// A base congruence `A·Δ·Aᵀ` preserves `D` **iff `A` maps the
/// constants to the constants**: `A·1 = c·1` for some `c ≠ 0`, i.e.
/// every row of `A` sums to the same value.
///
/// The derivation is short and it is the whole seal. A gradient is the
/// matrix `P = 1·pᵀ − p·1ᵀ`, so
///
/// ```text
/// A·P·Aᵀ = (A1)(Ap)ᵀ − (Ap)(A1)ᵀ
/// ```
///
/// which is a gradient again ONLY when `A1 ∈ span(1)`, in which case it
/// equals `c · grad(Ap)`. If `A1` leaves that line, the image of a pure
/// gradient is NOT a gradient: circulation appears out of frame alone,
/// and capacity is forged. Measured — a shear with row sums
/// `(6/5, 1, 1)` carries a `D = 0` coboundary to `D = 3`.
///
/// **This is the residue of the `f05bec8` mint-leak.** The Hodge flip
/// removed the anchor, but not this: `1` spans `ker L`, so a
/// transformation that does not preserve the constants does not map
/// potentials to potentials, and "pure frame" stops being a
/// well-defined notion under it.
///
/// **Volume SURVIVES the restriction.** The condition is `A1 = c·1`,
/// not `det(A) = 1` — scaling by `c` has constant row sums and
/// `det = cⁿ`. So the volumetric scaling the expansion law needs is
/// admitted; only the constants-breaking directions are refused.
/// Permutations (`c = 1`) are the relabelling subgroup.
pub fn preserves_hodge_split(matrix: &[Vec<Ratio>]) -> bool {
    let mut rows = matrix.iter().map(|r| r.iter().fold(Ratio::zero(), |s, x| s + x));
    let Some(first) = rows.next() else {
        return true; // the empty matrix moves nothing
    };
    if first.is_zero() {
        return false; // A1 = 0 — the constants are annihilated, not preserved
    }
    rows.all(|s| s == first)
}

/// The former reading, kept for the drift record: `ext_x( Π_G( Δ ) )`
/// over the ANCHORED representative. Agrees with [`capacity`] on
/// whether capacity exists, and disagrees on its extent, because the
/// anchor concentrates the class onto the pairs that miss vertex 0.
pub fn capacity_anchored(delta: &Delta) -> Extent {
    ext_x(&coboundary_reduce(delta))
}
