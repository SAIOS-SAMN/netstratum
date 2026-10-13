//! THE CAPACITY METER — `ext_x` and the D reading.
//!
//! Closes the first half of `25_STANDING` §6.2 ("`D` is not
//! measured"). Until this existed, Articles II, III and IV had no
//! operand and every capacity claim in the tree was unfalsifiable.
//!
//! **D is what SURVIVES reduction** (operator ruling 2026-08-01,
//! `17_CAPACITY` §4b, `26_ARTICLES` §5):
//!
//! ```text
//! D  =  ext_x( Π_G( Δ(V) ) )
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
//! **NEVER ONE SCALAR.** [`Extent`] reports per index. `26_ARTICLES`
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
//! PURGED (`24_FORM` §2d): the organs are `DiscoveryMinted` (tag 86)
//! and `mesh::spine::seal`, they are not summands of `Δ`, and reading
//! the mint as raw divergence was the over-reach.
//!
//! ALTITUDE: this is a READING, not a gate and not an objective
//! (`26_ARTICLES`, Ξ/ΔD are readings never objectives). It is `O(dim² ·
//! fiber)` over the committed relation and calls no search.

use crate::delta::Delta;
use crate::reduce::Support;
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
/// That is the failure `24_FORM` §2h records and LAW 8 forbids — a
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
/// A below-ground degree is therefore not a state the gate refuses at
/// runtime — it is a state that cannot be built. That is the house's
/// own standing defense against the vacuous arm (`17_CAPACITY` §10b
/// AV-4: *degenerate arms are made structurally unrepresentable where
/// possible*).
#[derive(Debug, Clone, Copy)]
pub struct Degree {
    /// **THE OPENED GROUND** (operator ruling 2026-08-05: *GROUND needs
    /// to be opened up into V = Volume of orbs, weight on D as
    /// capacity*). The floor a state stands on is the VOLUME of its
    /// estate — the measure of the orbs it holds — never below one.
    /// The 2026-08-01 point survives as the base case: an estate of one
    /// weighs one, and the empty pre-founding state weighs one.
    volume: usize,
    /// The gauge-invariant circulation standing above the floor. This
    /// is what the gate reads (`above()`), and it is deliberately
    /// UNTOUCHED by the volume — which is the derivation that made the
    /// opening a patch: nothing the gate admits consults the floor.
    circulation: usize,
}

impl Degree {
    /// The floor's base case: one orb, nothing circulating. `α ≅ Ω` is
    /// not nothing — it is a POINT (`24_FORM` §2e), one dimension,
    /// never none. Wider estates stand on wider floors.
    pub const GROUND: Degree = Degree { volume: Extent::GROUND, circulation: 0 };

    /// The point-weighted constructor: volume one, plus circulation.
    /// This is the pre-opening reading and remains correct wherever no
    /// estate stands behind the extent (the bare algebra, a lens over a
    /// projection). Below-ground stays arithmetically out of reach.
    pub fn above_ground(circulation: usize) -> Self {
        Degree { volume: Extent::GROUND, circulation }
    }

    /// **THE OPENED CONSTRUCTOR** — the estate's volume weighting the
    /// degree. `volume` is clamped to one: a state that exists occupies
    /// at least the point, so never-zero survives the opening by
    /// construction rather than by guard.
    pub fn in_volume(volume: usize, circulation: usize) -> Self {
        Degree { volume: volume.max(Extent::GROUND), circulation }
    }

    /// The reading as a number, for a lens:
    /// `volume × (GROUND + circulation)`. Weight means MULTIPLY —
    /// founding an orb multiplies the whole degree, which is the
    /// compounding the expansion rulings ask for. Always `≥ GROUND`.
    pub fn get(self) -> usize {
        self.volume * (Extent::GROUND + self.circulation)
    }

    /// How much stands ABOVE the ground — the gauge-invariant
    /// circulation, which is what grows. **Volume-blind on purpose**:
    /// this is the gate's operand (`revocation_law` reads it and never
    /// `get()`), so the estate's size can never change what admits.
    pub fn above(self) -> usize {
        self.circulation
    }

    /// The estate's measure — the opened ground itself.
    pub fn volume(self) -> usize {
        self.volume
    }

    /// ON THE GROUND: at rest on the floor of its own estate, however
    /// wide that floor is. It holds its place and its structural
    /// existence; it is never deleted and never barred. **Article II's
    /// operand is this predicate, never a zero.**
    pub fn is_ground(self) -> bool {
        self.circulation == 0
    }

    /// ABOVE THE GROUND: circulation stands on the floor.
    pub fn is_above_ground(self) -> bool {
        self.circulation > 0
    }
}

/// A degree IS its magnitude: two readings of equal capacity are equal,
/// however that capacity decomposes into floor and circulation. The
/// decomposition stays separately readable (`volume()`, `above()`) —
/// the couple remains two individuals — but comparison, ordering and
/// hashing are on what stands.
impl PartialEq for Degree {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}
impl Eq for Degree {}
impl PartialOrd for Degree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Degree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get().cmp(&other.get())
    }
}
impl std::hash::Hash for Degree {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get().hash(state);
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
        write!(f, "{}", self.get())
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
    /// traverse is a POINT (`24_FORM` §2e), which is one dimension,
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
/// (`24_FORM` §2d, THE SIGN LAW):
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
/// non-zero divergence at that vertex (`24_FORM` §2d, THE SIGN LAW).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flux {
    /// DIVERGENCE — `div > 0` on some layer. The vertex SOURCES: it
    /// emits more than it takes. Egress.
    pub sources: usize,
    /// CONVERGENCE — `div < 0` on some layer. The vertex SINKS: it
    /// absorbs more than it emits, and **that is a capability, not a
    /// fault.**
    ///
    /// A sink is the scarce half of the economy. `05` §1.4: a dim-star
    /// system *"holds the rarest resource in a high-flux universe:
    /// radiator headroom"*, and sells it as `S_cool` — absorption
    /// measured in joule-commensurable units. The Frost's entire trade
    /// is convergence.
    ///
    /// **A SINK IS NOT A TRAPPED INTERIOR.** What `18` §2d forbids is
    /// convergence with NO egress — a closed region that takes and
    /// never sheds. Absorption is lawful precisely BECAUSE it sheds:
    /// `sink_consumed = heat_absorbed = vented_after_shed` (`05` §1.4).
    /// That judgment needs the shed ledger and is not visible in `Δ`
    /// alone, so this reading reports the pole and never convicts it.
    pub sinks: usize,
    /// `div = 0` on every layer — the vertex circulates, sourcing and
    /// sinking nothing.
    pub balanced: usize,
}

impl Flux {
}

/// THE FLUX CENSUS — both poles of the divergence, counted and
/// named: DIVERGENCE sources, CONVERGENCE sinks, and the balanced
/// circulate. Neither pole is a fault (see [`divergence`]).
pub fn flux(delta: &Delta) -> Flux {
    let mut census = Flux { sources: 0, sinks: 0, balanced: 0 };
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
                    census.sinks += 1;
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

/// **THE CIRCULATION ON THE SUPPORT** — the Hodge split taken on the
/// relations that ACTUALLY EXIST, not on the complete graph.
///
/// [`circulation`] solves the complete-graph system `p = −div/n`, which
/// silently treats every absent relation as a real one carrying zero.
/// For a READING of the whole that is tolerable; for ATTRIBUTION it is
/// not, and it fails in two measured ways:
///
/// ```text
/// four entities, ONE relation (0—1 = 1), nothing else:
///   complete-graph shares -> [1/2, 1/2, 1/4, 1/4]
/// ```
///
/// Entities 2 and 3 hold nothing, owe nothing, and are owed nothing,
/// and each draws a QUARTER of a relation they are not party to —
/// FARMING BY ABSENCE, which is worse than farming by strain because it
/// costs nothing at all. And the same defect inflates the rest: the
/// identical relation reads `circ_01 = 0` among two entities and `1/2`
/// among four, so an orb could enlarge every share in the world by
/// seating empty orbs beside it.
///
/// Both vanish here. The potential solves the SUPPORT graph's own
/// Laplacian system, `L·p = −div`, with `L = deg − adjacency` over
/// present relations only. Isolated entities form their own components,
/// carry `p = 0`, and hold no relation to sum — so they bear nothing,
/// and because components decouple, seating them moves no other share.
///
/// `L` is singular by construction (the constants lie in its kernel,
/// once per component) and that is the gauge freedom itself: `p` is
/// fixed to zero at one vertex per component, which changes `p` by a
/// constant there and `grad(p)` not at all. Exact rational elimination
/// (I3) — no floats, no iteration, no tolerance.
pub fn circulation_on(delta: &Delta, support: &Support) -> Delta {
    let dim = delta.dim().min(support.dim());
    let fiber = delta.fiber().min(support.fiber());
    let zero = Ratio::from_integer(0.into());
    let mut out = Delta::zero(delta.dim(), delta.fiber());
    // One potential per layer; the edges are written once, after, so
    // every fiber coordinate of a relation is set in a single call.
    let mut potentials: Vec<Vec<Ratio>> = Vec::with_capacity(fiber);

    for l in 0..fiber {
        // THE COMPONENTS. Each is its own gauge orbit; an isolated
        // entity is a component of one and decouples completely.
        let mut comp = vec![usize::MAX; dim];
        let mut count = 0;
        for seed in 0..dim {
            if comp[seed] != usize::MAX {
                continue;
            }
            comp[seed] = count;
            let mut stack = vec![seed];
            while let Some(v) = stack.pop() {
                for (w, seen) in comp.iter_mut().enumerate().take(dim) {
                    if w != v && support.has(v, w, l) && *seen == usize::MAX {
                        *seen = count;
                        stack.push(w);
                    }
                }
            }
            count += 1;
        }

        // THE DIVERGENCE, over present relations only.
        let div: Vec<Ratio> = (0..dim)
            .map(|i| {
                let mut acc = zero.clone();
                for j in 0..dim {
                    if i != j && support.has(i, j, l) {
                        acc += delta.entry(i, j, l);
                    }
                }
                acc
            })
            .collect();

        let mut potential = vec![zero.clone(); dim];
        for c in 0..count {
            let members: Vec<usize> = (0..dim).filter(|v| comp[*v] == c).collect();
            if members.len() < 2 {
                continue; // isolated: p = 0, and it holds nothing anyway
            }
            // GAUGE FIXED at the first member: p = 0 there. The
            // remaining rows are `(L·p)_v = −div_v`.
            let unknowns = &members[1..];
            let k = unknowns.len();
            let mut aug: Vec<Vec<Ratio>> = Vec::with_capacity(k);
            for &v in unknowns {
                let mut row = vec![zero.clone(); k + 1];
                let mut degree = zero.clone();
                for w in 0..dim {
                    if w != v && support.has(v, w, l) {
                        degree += Ratio::from_integer(1.into());
                        if let Some(idx) = unknowns.iter().position(|u| *u == w) {
                            row[idx] -= Ratio::from_integer(1.into());
                        }
                    }
                }
                if let Some(idx) = unknowns.iter().position(|u| *u == v) {
                    row[idx] += degree;
                }
                row[k] = -&div[v];
                aug.push(row);
            }
            // Exact Gauss-Jordan. Any free column is residual gauge and
            // takes zero — `grad(p)` is unmoved either way.
            let mut pivot_of = vec![usize::MAX; k];
            let mut r = 0;
            for col in 0..k {
                let Some(hit) = (r..k).find(|row| aug[*row][col] != zero) else {
                    continue;
                };
                aug.swap(r, hit);
                let lead = aug[r][col].clone();
                for x in aug[r].iter_mut() {
                    *x /= &lead;
                }
                for other in 0..k {
                    if other == r {
                        continue;
                    }
                    let factor = aug[other][col].clone();
                    if factor == zero {
                        continue;
                    }
                    let taken: Vec<_> =
                        aug[r].iter().take(k + 1).map(|cell| cell * &factor).collect();
                    for (cell, take) in aug[other].iter_mut().zip(taken) {
                        *cell -= take;
                    }
                }
                pivot_of[col] = r;
                r += 1;
                if r == k {
                    break;
                }
            }
            for (col, &v) in unknowns.iter().enumerate() {
                potential[v] =
                    if pivot_of[col] == usize::MAX { zero.clone() } else { aug[pivot_of[col]][k].clone() };
            }
        }

        potentials.push(potential);
    }

    for i in 0..dim {
        for j in (i + 1)..dim {
            let coords: Vec<Ratio> = (0..delta.fiber())
                .map(|l| match l < fiber && support.has(i, j, l) {
                    // Absent relations carry nothing. This is the whole
                    // difference from the complete-graph reading.
                    false => zero.clone(),
                    true => delta.entry(i, j, l) - (&potentials[l][j] - &potentials[l][i]),
                })
                .collect();
            out.set_antisym(i, j, coords);
        }
    }
    out
}

/// **THE BORNE SHARE, AXIS BY AXIS** — how much of the circulation each
/// entity holds up ON EACH COORDINATE. `D` says how much room stands;
/// this says WHO IS STANDING IN IT, and IN WHICH DIRECTION.
///
/// ```text
/// borne_i,l = ½ · Σ_{j ~ i} |circulation_on(Δ, S)_ij,l|
/// ```
///
/// **NEVER ONE SCALAR** (`24_FORM` §2b, operator ruling 2026-08-02:
/// *this needs to be multi-axial on vertices*). The first landing of
/// this share summed across every coordinate and returned ONE number
/// per entity. That is a directional quantity collapsed to a scalar
/// aggregate, which §2b bars wherever the system computes — and it
/// erased the only thing an entity can act on. Two entities reading the
/// same total can stand in completely different worlds:
///
/// ```text
/// borne = 3   as one scalar     — indistinguishable
///   → (3, 0, 0)  everything on one axis, two axes untouched
///   → (1, 1, 1)  spread evenly, nowhere to grow cheaply
/// ```
///
/// The aggregate lies about the parts in exactly the way
/// `verdict_per_dimension` was built to catch: a universe can read
/// whole on the sum while no single coordinate carries both poles.
///
/// **TOWARD UPLIFT.** A coordinate where an entity stands at the ground
/// is not a deficiency, it is the OPEN DIRECTION — the axis on which a
/// new relation would earn most. Read per axis, the share stops being a
/// score and becomes a map of where to grow, which is the whole
/// difference between a reading that withholds and one that gives
/// (D6, `18` Article II).
///
/// Indexed `[entity][coordinate]`. Exact rationals (I3).
pub fn borne_axial(delta: &Delta, support: &Support) -> Vec<Vec<Ratio>> {
    let dim = delta.dim().min(support.dim());
    let fiber = delta.fiber().min(support.fiber());
    let circ = circulation_on(delta, support);
    let half = Ratio::new(1.into(), 2.into());
    let zero = Ratio::from_integer(0.into());
    (0..delta.dim())
        .map(|i| {
            (0..delta.fiber())
                .map(|l| {
                    let mut total = zero.clone();
                    if i < dim && l < fiber {
                        for j in 0..dim {
                            if i == j || !support.has(i, j, l) {
                                continue;
                            }
                            let v = circ.entry(i, j, l);
                            total += if *v < zero { -v } else { v.clone() };
                        }
                    }
                    total * &half
                })
                .collect()
        })
        .collect()
}

/// **THE BORNE SHARE** — how much of the circulation each entity holds
/// up. `D` says how much room stands; this says WHO IS STANDING IN IT.
///
/// Every present relation carries some of the canonical circulation.
/// Its two ends bear that magnitude EQUALLY — half each — and an
/// entity's share is the sum over its OWN relations:
///
/// ```text
/// borne_i = ½ · Σ_{j ~ i} Σ_l |circulation_on(Δ, S)_ij,l|
/// ```
///
/// **THIS IS THE LENS FORM — the L1 over [`borne_axial`], and the axial
/// reading is the one that computes.** `24_FORM` §2b permits an L1
/// aggregate at a HUMAN LENS and bars it wherever the system computes,
/// so this is for display and for totals that must reconcile (the
/// conservation pin sums it against the whole circulation). **A gate
/// reads [`borne_axial`]**: collapsing first would hide which
/// coordinate an entity actually stands on, and that is the only part
/// of the reading it can act on.
///
/// **Why the relation and not the cycle.** The cycle is the intuitive
/// unit — charge is a property of a closed loop — but a *fundamental
/// cycle basis* is chosen by a spanning forest, so the node set
/// `Nodes(C)` and its size `N_C` MOVE WHEN THE ENTITIES ARE RELABELLED.
/// Measured on a theta graph: the same charge yields cycles
/// `{0,1,2},{0,2,3}` under one labelling and `{0,1,2,3},{0,1,2}` under
/// another, so `|c|/N_C` pays different entities different amounts for
/// identical structure. Article I voids it. Only the cycle-space
/// DIMENSION is invariant, never a chosen basis of it. The circulation
/// is the unique Hodge representative — no forest, no root — so a share
/// read off IT is basis-free, and this is pinned over every relabelling.
///
/// **Why the support and not the complete graph.** See
/// [`circulation_on`]: on the complete graph an entity holding NO
/// relations draws a quarter of a relation it is not party to. A share
/// is only ever read across relations that exist.
///
/// **Why circulation and not the residuals.** `symmetry_foundation`
/// stands exactly where obligations FAIL to compose. Paying on it would
/// pay for strain: manufacture an inconsistency, draw a share, do no
/// work. The circulation is coherent closed content — what actually
/// composes — so a share can only be earned by structure that holds.
///
/// **Conservation.** Each relation's magnitude is split in two and both
/// halves are handed out, so `Σ_i borne_i` is exactly the total
/// circulation standing on the support. Nothing is minted at the join
/// and nothing falls out of it.
///
/// Exact rationals throughout (I3). A pure coboundary bears nothing —
/// frame is not work.
pub fn borne(delta: &Delta, support: &Support) -> Vec<Ratio> {
    let zero = Ratio::from_integer(0.into());
    borne_axial(delta, support)
        .into_iter()
        .map(|axes| axes.into_iter().fold(zero.clone(), |a, b| a + b))
        .collect()
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
