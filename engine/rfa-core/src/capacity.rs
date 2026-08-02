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
//! reading is STRUCK. A universe may carry enormous arity and still
//! stand ON THE GROUND when every difference it holds is pure frame,
//! which is exactly the dogmatic loop Article III prices at zero.
//!
//! **THE ELEVATION.** Two readings, and they answer different
//! questions. [`Extent::coordinates`] counts the CIRCULATION — what
//! stands above the floor — and zero is a lawful answer for it.
//! [`Extent::degree`] is `D` itself, the elevation, and it is
//! [`Degree`]: `GROUND + circulation`, never zero, with no
//! representation for a below-ground value.
//!
//! **NEVER ONE SCALAR.** [`Extent`] reports per index. `18_TORSION`
//! §2b bars collapsing a directional quantity to a scalar aggregate
//! wherever the system computes; a single "capacity number" would be
//! the `C = x` substandard character (`17_CAPACITY` §2a.2) returning
//! under a new name. The extent says WHERE structure stands.
//!
//! **THE PAIRWISE LAW — the meter's one precondition.** The input must
//! be irreducibly pairwise (bonds, forwards, absorption debts), so
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
//! so such a source carries NO CIRCULATION. It has not vanished: it
//! sits at the GROUND (see [`Extent::GROUND`]).
//!
//! **D IS NEVER ZERO** (operator ruling 2026-08-01). Zero is an
//! ABSOLUTE and this system carries none — the T∞ purge already barred
//! the reciprocal pole, and a capacity of zero says the thing has no
//! space in which to be. The floor is the POINT: one degree, itself.
//!
//! **WHY THE SOURCED POLE IS NOT CAPACITY, measured.** A first attempt
//! at this ruling folded the divergence (the egress reading) into `D` so that a
//! pure coboundary would read non-zero. It fails: divergence is
//! GAUGE-DEPENDENT — `div(Δ + grad p) = div(Δ) − n·p` — so a node could
//! inflate its own capacity by adding a pure coboundary, which is the
//! forgery this meter exists to refuse. Capacity counts the
//! gauge-INVARIANT content (circulation) above a floor that is never
//! zero. The divergence stays a first-class reading ([`egress`]) for
//! THE SIGN LAW; it is simply not `D`.
//!
//! The cosmological naming that once wrapped these two readings is
//! PURGED (`18_TORSION` §2d): the organs are `DiscoveryMinted` (tag 86)
//! and `mesh::spine::seal`, they are not summands of `Δ`, and reading
//! the mint as raw divergence was the over-reach.
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
    /// Independent CIRCULATION coordinates: pairs `(i < j)` × layer
    /// carrying holonomy (`∮ ≠ 0`).
    pub coordinates: usize,
}

/// **`D` — THE DEGREE, STANDING ON THE GROUND BY CONSTRUCTION.**
///
/// The elevation reading, and the one type any `D` claim must pass
/// through:
///
/// ```text
///   D > GROUND   ABOVE GROUND — circulation stands above the point
///   D = GROUND   ON THE GROUND — the point state (α ≅ Ω), at rest
///   D < GROUND   IN THE GROUND — UNREPRESENTABLE, not merely refused
/// ```
///
/// **WHY A TYPE AND NOT A CHECK** (operator ruling 2026-08-01, maximal
/// inscription). `Extent::GROUND` was declared, doc-commented and
/// asserted, and *no computation ever read it*: `ground()` returned all
/// zeros, `is_ground()` tested `coordinates == 0`, and the pin that
/// claimed to prove the floor compared the constant to its own literal.
/// The ruling said D is never zero; the arithmetic still returned zero.
/// That is the failure `18_TORSION` §2h records and LAW 8 forbids — a
/// term that resonates but computes nothing.
///
/// The floor is now load-bearing:
///
/// - the field is **private**, so no caller anywhere can write a value
///   through it;
/// - the only constructor is [`Degree::above_ground`], which adds
///   circulation to `GROUND` — `usize` addition cannot go below its
///   own start, so underflow is not a case that exists;
/// - there is no `From<usize>`, no public tuple construction, and
///   [`Degree::default`] is the GROUND, never zero.
///
/// A below-ground degree is therefore not a state the judge refuses at
/// runtime — it is a state that cannot be built. That is the house's
/// own standing defense against the vacuous arm (`17_CAPACITY` §10b
/// AV-4: *degenerate arms are made structurally unrepresentable where
/// possible*).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Degree(usize);

impl Degree {
    /// The floor itself: the one degree a committed state occupies by
    /// existing. `α ≅ Ω` is not nothing — it is a POINT
    /// (`18_TORSION` §2e), which is one dimension, never none.
    pub const GROUND: Degree = Degree(Extent::GROUND);

    /// THE ONLY CONSTRUCTOR. The point the state occupies, plus the
    /// independent circulation standing above it.
    ///
    /// `GROUND + circulation` — total, and below-ground is arithmetically
    /// out of reach rather than guarded against.
    pub fn above_ground(circulation: usize) -> Self {
        Degree(Extent::GROUND + circulation)
    }

    /// The reading as a number, for a lens. Always `≥ GROUND`.
    pub fn get(self) -> usize {
        self.0
    }

    /// How much stands ABOVE the ground — the gauge-invariant
    /// circulation, which is what grows.
    pub fn above(self) -> usize {
        self.0 - Extent::GROUND
    }

    /// ON THE GROUND: the point state, at rest on the floor. It holds
    /// its place and its structural existence; it is never deleted and
    /// never barred. **Article II's operand is this predicate, never a
    /// zero.**
    pub fn is_ground(self) -> bool {
        self.0 == Extent::GROUND
    }

    /// ABOVE THE GROUND: expanded topological volume carrying
    /// circulation.
    pub fn is_above_ground(self) -> bool {
        self.0 > Extent::GROUND
    }
}

impl Default for Degree {
    /// The default is the GROUND, never zero — a `Degree` that came from
    /// nowhere still stands on the floor.
    fn default() -> Self {
        Degree::GROUND
    }
}

impl std::fmt::Display for Degree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Extent {
    /// **THE GROUND — `D` IS NEVER ZERO** (operator ruling 2026-08-01).
    ///
    /// Zero capacity is an ABSOLUTE, and this system does not carry
    /// absolutes. The T∞ purge already barred the other pole —
    /// *"infinity is disallowed, the torus wraps finitely"* — and zero
    /// is its reciprocal: a dimensional capacity of zero says the thing
    /// has no space in which to be, which is incoherent for a state
    /// that is committed and receipted.
    ///
    /// The floor is **the point**: a universe that exists occupies one
    /// degree, itself. `α ≅ Ω` does not mean nothing — it means the
    /// traverse is a POINT (`18_TORSION` §2e), which is one dimension,
    /// never none.
    ///
    /// **This constant is now CONSUMED, not merely declared** — see
    /// [`Degree`], which is the only way a `D` reading enters the
    /// system.
    pub const GROUND: usize = 1;

    /// **`D` — the degree this extent stands at.** The point, plus the
    /// circulation above it. Never zero, by the type's construction.
    pub fn degree(&self) -> Degree {
        Degree::above_ground(self.coordinates)
    }

    /// At the GROUND: the state carries no structure above the point.
    /// This REPLACES the former `is_vacuous` — the predicate is the
    /// same set, the MEANING is not. A state here has not vanished; it
    /// stands at the floor, holding one degree, itself.
    ///
    /// Article II's operand is this predicate, never a zero.
    pub fn is_ground(&self) -> bool {
        self.degree().is_ground()
    }

    /// ABOVE the ground: circulation stands, and it is what grows.
    pub fn is_above_ground(&self) -> bool {
        self.degree().is_above_ground()
    }

    /// Does the relation carry holonomy — `∮ ≠ 0`? This is the
    /// GAUGE-INVARIANT content, and it is what `D` counts above the
    /// ground.
    pub fn circulates(&self) -> bool {
        self.coordinates > 0
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
/// **THIS IS THE SIGNED READING, and the sign is the law**
/// (`18_TORSION` §2d, THE SIGN LAW):
///
/// - `div_i > 0` — an EGRESS. The vertex sources: it emits more than it
///   absorbs.
/// - `div_i < 0` — an INGRESS. The vertex sinks: it absorbs more than
///   it emits.
/// - `div_i = 0` — BALANCED. Whatever the vertex carries circulates —
///   it returns what it takes, and the content is holonomic.
///
/// **The books close.** Antisymmetry (`Δ_ji = −Δ_ij`) gives
/// `Σ_i div_i,l = 0` on every layer, so an egress anywhere implies an
/// ingress somewhere and NO RELATION IS ALL-SOURCE. That conservation
/// is why the reading is a statement about a boundary rather than about
/// an isolated vertex.
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
/// SINK, how many are balanced. Counted over any fiber layer carrying a
/// non-zero divergence at that vertex (`18_TORSION` §2d, THE SIGN LAW).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Egress {
    /// EGRESS vertices: `div > 0` on some layer.
    pub sources: usize,
    /// INGRESS vertices: `div < 0` on some layer.
    pub trapped: usize,
    /// `div = 0` on every layer — the vertex circulates, sourcing and
    /// sinking nothing.
    pub balanced: usize,
}

impl Egress {
    /// **Is the relation DIVERGENCE-FREE — `div Δ = 0` at every vertex?**
    ///
    /// Equivalently: pure circulation. Nothing sources, nothing sinks,
    /// and every content is holonomic.
    ///
    /// **This replaces a cosmological predicate name that overclaimed
    /// twice** (operator ratification 2026-08-01). The fault was never
    /// the borrowed word's provenance — it was LAW 8: the predicate did
    /// not compute the object it named.
    ///
    /// The proof it could not. The books close, `Σ_i div_i = 0`, so
    /// letting `V⁺ = {i : div_i > 0}` and `V⁻ = {i : div_i < 0}`:
    ///
    /// ```text
    ///     Σ_{V⁺} div_i  =  Σ_{V⁻} |div_i|
    ///     |V⁻| = 0   ⟹   Σ_{V⁺} div_i = 0   ⟹   |V⁺| = 0
    /// ```
    ///
    /// So `trapped == 0` forces `sources == 0` and every vertex
    /// balanced. The predicate could never distinguish an emitting
    /// region from ordinary balanced circulation — an anti-trapped
    /// configuration does not exist in a closed antisymmetric relation.
    /// It now carries the name of what it actually decides. (It reads
    /// both counts rather than `trapped == 0` alone, so the meaning is
    /// legible without the reader having to re-derive the conservation.)
    pub fn is_pure_circulation(&self) -> bool {
        self.sources == 0 && self.trapped == 0
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
/// `(6/5, 1, 1)` carries a coboundary standing ON THE GROUND to one
/// circulating in 3 coordinates.
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

/// The GROUND extent — what a state carrying nothing above the point
/// reads.
///
/// Its component counts are zero because NOTHING CIRCULATES: no pair
/// carries holonomy, no entity stands in a surviving relation, no layer
/// is occupied. That is a statement about what is above the floor.
///
/// **Its DEGREE is not zero.** `ground().degree() == Degree::GROUND`,
/// and that is the reading Article II consults. The state has not
/// vanished; it stands on the surface holding one degree, itself.
pub fn ground() -> Extent {
    Extent { entities: 0, layers: 0, coordinates: 0 }
}
