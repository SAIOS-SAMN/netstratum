//! THE BRIDGE — the projection from committed state to the relational
//! tensor (Task #25, operator ruling 2026-08-01).
//!
//! Before this module the algebra was an ISLAND: `coboundary_reduce`,
//! `for_each_residual`, `orbit_lock` and `rcf_signature` were all
//! landed and witnessed in `rfa-core`, and not one of them had ever run
//! on a universe. `Δ(Ω)` — "the relation V expresses" (`18_TORSION`
//! §2a, `EQUATIONS` D0) — was not a computation that existed. This is
//! that computation.
//!
//! **THE COBOUNDARY TRAP, and why the relation is not a bank
//! difference.** If `Δ_ij,l = scalar(i,l) − scalar(j,l)` — the obvious
//! bridge, per-orb banks differenced — then `Δ` is a PURE COBOUNDARY.
//! `reduce.rs` phase 1 anchors every relation at vertex 0 and sends
//! exactly that shape to zero (it is the same construction
//! `rfa-core/tests/fold_pins.rs` uses to MAKE a gauge shift), so `D`
//! would read zero for every universe forever and Article II would
//! deny standing to everything. Node-local scalars carry no structural
//! content by construction.
//!
//! **What is projected instead: the irreducibly PAIRWISE obligations.**
//! A standing obligation between two orbs is not derivable from any
//! per-orb property, so it survives the reduction as real structure.
//! The cocycle condition then asks whether obligation COMPOSES: if `i`
//! owes `j` and `j` owes `k`, does that equal what `i` owes `k`? A
//! triangle that does not compose is an economic fact, not an error.
//!
//! Entities `(i, j, k)` are ORBS, indexed by position in
//! `Committed::orbs`. The fiber coordinate `l` is the obligation
//! CHANNEL. (Note for readers of the relayed design: `k` is NOT a free
//! axis to assign — `Delta` is `dim × dim × fiber` and
//! `for_each_residual` ranges `i`, `j`, `k` all over `dim`. The
//! relational class therefore rides `l`, the only free coordinate.)
//!
//! COUNCILS ARE OMITTED, deliberately: a council binds three or more
//! members alike (`COUNCIL-1` — one boundary reckoning judging all
//! together), and projecting it onto pairs would MANUFACTURE pairwise
//! structure that the deed never created. A multi-party obligation
//! needs its own carrier; inventing edges to fit this one is exactly
//! the cosmetic move `18_TORSION` §3 Article III forbids.

use cfe::ratio::{ratio, Ratio};
use cfe::world::Committed;
use rfa_core::{
    dual_pole, holonomies, boundary_form, Degree, Delta, Flow, BoundaryForm, Support, Vacuity,
};

/// BOND-1: the standing order, `source` → `target` at `rate`/tick.
pub const CHANNEL_BOND: usize = 0;
/// COVENANT-1: the UNDELIVERED REMAINDER of a standing delivery
/// obligation, obligor → beneficiary (`quota − delivered`, floored at
/// zero — a fully delivered obligation owes nothing and contributes
/// nothing).
///
/// **What the channel carries, stripped of the spec's vocabulary:** one
/// party is bound to deliver a quantity of a resource to another by a
/// deadline, with collateral posted against failure. `quota` is the
/// amount owed, `delivered` the progress, and the projection carries
/// the difference. Nothing in the mechanism is ceremonial — "covenant"
/// and "pact" are inherited names (COVENANT-1, BUILD_ORDER §3), not
/// descriptions, and this channel means the remainder and only that.
pub const CHANNEL_COVENANT: usize = 1;
/// ABSORB-1: the open reciprocity window's remainder, debtor →
/// creditor.
pub const CHANNEL_DEBT: usize = 2;
/// The fiber arity of the projection.
pub const CHANNELS: usize = 3;

/// WHAT IS KNOWN about a relation (operator ruling 2026-08-01: the
/// complex is SPARSE — an absent relation is ABSENT, not zero).
///
/// Three values, never two. A dense tensor cannot tell "these two do
/// not relate" from "these two relate by zero", and that distinction is
/// exactly what carries topology: with every pair present, every
/// cocycle is a coboundary and `D` collapses (measured in
/// `bridge_pins`). Sparsity is what lets a cycle exist.
///
/// **ABSENCE IS A CLAIM, AND CLAIMS CAN BE WRONG.** This is not
/// epistemic politeness — it is the forgery gate. A mistaken absence
/// creates a cycle that is not there, and a phantom cycle carries
/// phantom holonomy, which reads as capacity that was never earned.
/// **Unverified absence mints.** So an unobserved pair may never be
/// counted as absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Observed {
    /// A receipted obligation stands between the pair on this channel.
    Present,
    /// VERIFIED absent: the observer's record is complete over this
    /// pair, and no obligation stands. Within one book this is the
    /// ordinary case — a world's chronicle is complete over its OWN
    /// orbs, so silence there is a fact.
    Absent,
    /// NOT SEEN. The pair may relate beyond the observer's reach. Never
    /// counted as `Absent`; it must first be resolved (see [`Resolve`]).
    /// Across books this is the ordinary case — *worlds cannot see
    /// neighbours* (WANT-1 kind 1, the epistemological want: a short
    /// reach reads as truth-starvation, by design).
    Unknown,
}

/// How an `Unknown` is discharged before capacity may be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolve {
    /// TELESCOPE FURTHER OUT — widen the reach and look again. The
    /// relation may stand outside the current horizon (reach is a
    /// wrapped-L1 distance; a short reach hides real structure).
    Telescope,
    /// LOCK ON — confirm the absence at the present reach, turning
    /// `Unknown` into `Absent` by observation rather than assumption.
    LockOn,
}

/// ONE RESIDUAL, CARRIED WHOLE — `R_ijk,l = Δ_ik,l − Δ_ij,l − Δ_jk,l`
/// with the triple and coordinate that produced it.
///
/// **These are the first formations of symmetry** (operator, 2026-08-01;
/// `KERNEL_RELAY` §5.3: *the residue forms the symmetry* — frame
/// differences that compose across many convergences into a genuine
/// gauge group). Each residual is a candidate generator, and it is
/// meaningless without its indices: a residual says *these three, on
/// this coordinate, do not compose, by this much*.
///
/// **Summing them destroys them.** `C(Δ) = Σ|R|` is a scalar aggregate,
/// and `18_TORSION` §2b bars exactly that where the system computes —
/// L1 survives only at a human lens. Worse, the residue composition
/// law (the design owed at `KERNEL_RELAY` §7.4) has nothing to compose
/// if the residuals were already added together, and `EQUATIONS` D3
/// makes these the source of capacity: *the residue EMITS, D rises by
/// the plane*. A summed residual cannot emit anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Residual {
    pub i: usize,
    pub j: usize,
    pub k: usize,
    pub l: usize,
    pub value: Ratio,
}

/// THE ENTANGLED READ (`18_TORSION` §2f). The rank side and the
/// potential side, computed in ONE pass over ONE prior state. They are
/// not separately obtainable from this module by construction: a
/// caller that could take `D` alone and consult the strain separately
/// would have broken the entanglement and reintroduced the two-object
/// attack surface the law closes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntangledRead {
    /// `D` — the RANK of the relational structure: how many independent
    /// cycles carry charge no frame explains. Frame contributes nothing.
    ///
    /// **NOT A CAPACITY** (operator ruling 2026-08-01: *why do we have
    /// capacity? that should not be the case in an expanding universe*).
    /// "Capacity" names a container and implies a ceiling, and there is
    /// no ceiling here — not in this module, not in `rfa-core`, not
    /// anywhere in the workspace. `holonomies().len()` rises without
    /// bound as structure emerges, exactly as a matrix's rank rises when
    /// an independent row is added. Nobody asks a matrix for its rank
    /// capacity.
    ///
    /// The name also contradicted the law it serves: `EQUATIONS` D2
    /// already rules that *D is a READING of the universe's own
    /// structure, not a quota anything must fit under* — while the word
    /// said quota. It kept pulling the design back toward the container
    /// D2 struck.
    ///
    /// **Where a real capacity does live:** seat widths and occupancy
    /// caps (`12_UNION` §3, `11_AEON` §7) ARE bounds, and they are
    /// DERIVED from this rank. That is how the misnomer arrived — the
    /// reading that sets a bound was named after the bound it sets.
    /// `PhiK I^(3)` had it right all along: *rank — dim of the
    /// relational structure*.
    ///
    /// **THIS COUNTS WHAT STANDS ABOVE THE GROUND, and it is NOT `D`**
    /// (operator ruling 2026-08-01, maximal inscription). It is the
    /// circulation count, and at the floor it reads ZERO — correctly,
    /// because nothing circulates there. `D` is the ELEVATION, which
    /// includes the point the state occupies by existing, and it is
    /// [`EntangledRead::degree`]. Reading this field as `D` is what put
    /// `D = 0` back into a system whose law says D is never zero.
    ///
    /// It stays a raw count because [`rfa_core::dual_pole`] classifies
    /// on "does anything circulate" — a question about what is ABOVE the
    /// floor, where zero is the right answer and the floor would only
    /// blur it.
    pub rank: usize,
    /// The residual field, WHOLE — every non-composing triple with its
    /// indices intact. Never pre-summed: these are what emit (D3) and
    /// what the composition law will compose.
    pub residuals: Vec<Residual>,
    /// `∂Δ` — the SECOND POLE, carried whole as its non-zero
    /// components: which vertices carry net flow, and how much. Empty
    /// means pure circulation — every vertex balances, nothing sources
    /// or sinks, and the field does no work however large it is.
    pub flows: Vec<Flow>,
    /// Pairs whose relation was NOT observed. While any stands, the
    /// complex has holes of unknown shape and no rank may be read
    /// off it.
    pub unknown: usize,
}

impl EntangledRead {
    /// **`D` is well-defined only on a fully OBSERVED complex.** One
    /// condition, and it is a refusal to read rank out of something
    /// that has none to give: `unknown == 0`. An unresolved absence
    /// would let phantom topology mint phantom rank, so discharge
    /// every `Unknown` by [`Resolve`] first.
    ///
    /// **THE RESIDUAL CLAUSE IS STRUCK** (operator ruling 2026-08-01).
    /// It read `&& self.residuals.is_empty()`, barring the rank while
    /// any triple failed to compose, on the reasoning that
    /// `H¹ = Z¹/B¹` is defined on cocycles alone. Three faults:
    ///
    /// 1. That reasoning is about cohomology as a QUOTIENT. This engine
    ///    never forms the quotient — [`rfa_core::holonomies`] reduces
    ///    against a spanning forest and reads the charge each remaining
    ///    edge carries. It is TOTAL and needs no cocycle precondition.
    /// 2. It gave `C` a gate. Structural phrasing (`is_empty`) rather
    ///    than scalar (`defect == 0`) does not change what it does: a
    ///    non-composing state was barred from being read.
    /// 3. It contradicts this module's own header — *a triangle that
    ///    does not compose is an economic fact, not an error* — and THE
    ///    BIFURCATION LAW, under which non-composition ROOTS and opens
    ///    capacity by receipt. Non-composition is the SIGNAL, and the
    ///    old clause silenced exactly the states worth reading.
    ///
    /// **A RECORDED OBLIGATION, NOT A LANDED MUST** (operator
    /// ratification 2026-08-01). This read *"callers that gate on rank
    /// MUST consult this"* — and no caller does; the only non-test
    /// reference is this definition. A `MUST` with no enforcer reads as
    /// landed law and is not one, which is the same shape as a constant
    /// that is declared and never computed.
    ///
    /// The obligation stands and is owed: **when Article II standing or
    /// Article IV monotonicity gets its gate, that gate consults this
    /// first** — an unresolved absence would let phantom topology mint
    /// phantom rank. Until the gate exists, this is an available
    /// reading, recorded in `18_TORSION` §6 as debt rather than
    /// asserted here as enforcement.
    pub fn rank_is_defined(&self) -> bool {
        self.unknown == 0
    }

    /// **`D` — THE DEGREE, ON THE GROUND** (operator ruling 2026-08-01,
    /// maximal inscription). The elevation this state stands at: the one
    /// degree it occupies by existing, plus every independent
    /// circulation above it.
    ///
    /// This is the reading Article II consults, and it **can never
    /// return zero** — [`rfa_core::Degree`] has no representation for a
    /// below-ground value. A world with no obligations at all reads
    /// `Degree::GROUND`, not nothing: it stands on the surface, holds
    /// its place, and is never deleted.
    ///
    /// Use this wherever the question is *how high does this state
    /// stand*. Use [`EntangledRead::rank`] only where the question is
    /// *does anything circulate above the floor* — the `dual_pole`
    /// question.
    pub fn degree(&self) -> Degree {
        Degree::above_ground(self.rank)
    }

    /// THE VERDICT — both poles read at once (`rfa_core::dual_pole`).
    /// Only `Entangled` stands: structure WITH gradient. `Stasis` is
    /// rank without flow — a wheel turning against nothing, at any
    /// size.
    ///
    /// Classifies on the ABOVE-GROUND circulation, never on the degree:
    /// every state stands at or above the ground, so a floored reading
    /// would make `Void` and `Stasis` unreachable and collapse the
    /// four rows to two.
    pub fn verdict(&self) -> Vacuity {
        dual_pole(self.rank, &BoundaryForm { flows: self.flows.clone() })
    }
}

/// THE VERDICT PER DIMENSION — the absolute, witnessed in EVERY one
/// (operator ruling 2026-08-01: *D must always be greater than 0 and
/// not in stasis and is entangled with V permanently, witnessed in
/// every dimension*).
///
/// **The aggregate verdict is not this.** `dual_pole` takes the TOTAL
/// rank and the WHOLE form, so a universe can read `Entangled` on the
/// sum while no single channel is entangled at all — rank standing in
/// one dimension, flow standing in another, and neither dimension
/// carrying both. That is the aggregate lying about the parts, and it is
/// exactly what a per-dimension witness catches.
///
/// **Absolute, not rate.** This is decidable at ONE state: no history,
/// no limit, no approach. A rate never attains its bound at any step, so
/// a rate can never be an invariant — it can only be a lens. The
/// stability reading (support expanding against gradient flattening)
/// belongs there; THIS belongs in the judge.
pub fn verdict_per_dimension(state: &Committed) -> Vec<Vacuity> {
    let (delta, support) = project_complex(state);
    let charges = holonomies(&delta, &support);
    let form = boundary_form(&delta, &support);
    (0..CHANNELS)
        .map(|l| {
            let rank = charges.iter().filter(|held| held.l == l).count();
            let flows =
                form.flows.iter().filter(|flow| flow.l == l).cloned().collect::<Vec<_>>();
            dual_pole(rank, &BoundaryForm { flows })
        })
        .collect()
}

/// Which dimensions carry any relation at all. A channel no deed has
/// ever used is UNOCCUPIED — it reads `Void`, and that is a fact about
/// the world's shape rather than a failure of it.
pub fn occupied_dimensions(state: &Committed) -> Vec<usize> {
    let (_, support) = project_complex(state);
    let dim = state.orbs.len();
    (0..CHANNELS)
        .filter(|&l| (0..dim).any(|i| (0..dim).any(|j| j != i && support.has(i, j, l))))
        .collect()
}

/// Project committed state onto the relational tensor: `Δ(Ω)`.
///
/// Pure and total. Orbs are entities in `Committed::orbs` order;
/// obligations naming an unknown orb are skipped (they relate nothing
/// in THIS book), and a self-directed obligation is skipped (the
/// diagonal is identically zero — `Delta::set_antisym` would panic,
/// and the kernel is total).
///
/// Opposed obligations NET: where `i` owes `j` and `j` owes `i` on one
/// channel, the relation carries the difference. That keeps
/// antisymmetry meaningful rather than double-counting a settled pair.
pub fn project(state: &Committed) -> Delta {
    project_complex(state).0
}

/// Project committed state onto the SPARSE COMPLEX: the relation AND
/// its support.
///
/// The support is built from the OBLIGATIONS THEMSELVES, never from
/// which entries came out non-zero. That distinction is the whole point
/// of sparsity: two orbs that owe each other the same amount net to
/// zero, and they are still RELATED. Reading support off the values
/// would put them back in the same bucket as two orbs that have never
/// dealt with each other — the exact conflation `Observed` exists to
/// end.
pub fn project_complex(state: &Committed) -> (Delta, Support) {
    let dim = state.orbs.len();
    let mut delta = Delta::zero(dim, CHANNELS);
    let mut support = Support::new(dim, CHANNELS);
    if dim < 2 {
        // One orb relates to nothing; zero orbs still less. Both are
        // lawful and both read as no structure.
        return (delta, support);
    }
    let seat = |orb: u64| state.orbs.iter().position(|held| held.orb_id == orb);
    let zero = ratio(0, 1);
    let mut owed = vec![vec![vec![zero.clone(); CHANNELS]; dim]; dim];

    for bond in &state.bonds {
        if let (Some(i), Some(j)) = (seat(bond.source), seat(bond.target)) {
            if i != j {
                owed[i][j][CHANNEL_BOND] += bond.rate.clone();
                support.connect(i, j, CHANNEL_BOND);
            }
        }
    }
    for pact in &state.covenants {
        if let (Some(i), Some(j)) = (seat(pact.obligor), seat(pact.beneficiary)) {
            let outstanding = pact.quota.clone() - pact.delivered.clone();
            if i != j && outstanding > zero {
                owed[i][j][CHANNEL_COVENANT] += outstanding;
                support.connect(i, j, CHANNEL_COVENANT);
            }
        }
    }
    for debt in &state.debts {
        if let (Some(i), Some(j)) = (seat(debt.debtor), seat(debt.creditor)) {
            if i != j {
                owed[i][j][CHANNEL_DEBT] += debt.remainder.clone();
                support.connect(i, j, CHANNEL_DEBT);
            }
        }
    }

    for (i, row) in owed.iter().enumerate() {
        for (j, back) in owed.iter().enumerate().skip(i + 1) {
            let coords: Vec<Ratio> = row[j]
                .iter()
                .zip(back[i].iter())
                .map(|(out, in_)| out.clone() - in_.clone())
                .collect();
            delta.set_antisym(i, j, coords);
        }
    }
    (delta, support)
}

/// THE REFOLD CERTIFICATE — what the fold took, kept exactly, so the
/// fold can be undone (operator ruling 2026-08-01: *fold is too flat —
/// this is folding a BLANKET, not a piece of paper*).
///
/// A crease in paper is permanent: the sheet goes flat and the fold
/// line destroys what was there. A blanket keeps its loft — folding
/// rearranges it and nothing is lost, so it unfolds whole. The
/// reduction must be the blanket.
///
/// `coboundary_reduce` returns only the core. What it stripped is
/// simply gone, and that is where **capacity is lost to silly
/// mistakes**: if a relation was mis-observed — an `Unknown` wrongly
/// taken as `Absent`, a reach too short — then some of what the fold
/// discarded as "frame" was structure, and there is no way back to it.
/// The certificate closes that door: the fold is EXACTLY INVERTIBLE, so
/// a correction to the observation re-runs the fold instead of having
/// to re-earn the capacity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refold {
    /// `p_i,l` — the frame potentials anchored at entity 0. The
    /// coboundary the fold stripped is exactly `p_j − p_i`, which is
    /// the one shape that carries no structural content (witnessed in
    /// `bridge_pins`).
    pub potentials: Vec<Vec<Ratio>>,
}

/// Fold with a certificate: strip the frame anchored at entity 0 and
/// KEEP what was stripped. The returned core is the projection before
/// integer normalization, so the pair is exactly invertible by
/// [`refold`]. For the canonical scale-normalized representative the
/// caller still uses the sealed `coboundary_reduce` — this door exists
/// to make the fold reversible, never to replace it.
pub fn fold_certified(delta: &Delta) -> (Delta, Refold) {
    let dim = delta.dim();
    let fiber = delta.fiber();
    let potentials: Vec<Vec<Ratio>> = (0..dim)
        .map(|i| (0..fiber).map(|l| delta.entry(0, i, l).clone()).collect())
        .collect();
    let mut core = Delta::zero(dim, fiber);
    for i in 0..dim {
        for j in (i + 1)..dim {
            let coords: Vec<Ratio> = (0..fiber)
                .map(|l| {
                    delta.entry(i, j, l).clone() - potentials[j][l].clone()
                        + potentials[i][l].clone()
                })
                .collect();
            core.set_antisym(i, j, coords);
        }
    }
    (core, Refold { potentials })
}

/// Undo a certified fold: re-add the frame the fold took. Exact — the
/// blanket unfolds whole.
pub fn refold(core: &Delta, cert: &Refold) -> Delta {
    let dim = core.dim();
    let fiber = core.fiber();
    let mut whole = Delta::zero(dim, fiber);
    for i in 0..dim {
        for j in (i + 1)..dim {
            let coords: Vec<Ratio> = (0..fiber)
                .map(|l| {
                    core.entry(i, j, l).clone() + cert.potentials[j][l].clone()
                        - cert.potentials[i][l].clone()
                })
                .collect();
            whole.set_antisym(i, j, coords);
        }
    }
    whole
}

/// What is KNOWN about the relation between two orbs on one channel,
/// within ONE committed book.
///
/// A world's chronicle is complete over its own orbs, so silence about
/// a pair it holds is a VERIFIED absence, not an unobserved one. The
/// `Unknown` case belongs to the cross-book reading, where a book
/// cannot see a neighbour's obligations and the mesh must present them
/// — `read_entangled` reports zero unknowns for exactly this reason,
/// and `read_entangled_observed` is the door for the mesh case.
/// **Reads the SUPPORT, never a value** (operator ruling 2026-08-01:
/// no `==` in deterministic code). This previously decided presence by
/// `*delta.entry(i, j, channel) != ratio(0, 1)` — comparing a NETTED
/// value against zero. But `project_complex` nets opposed obligations,
/// so two orbs owing each other equally net to zero and read ABSENT,
/// which is exactly the conflation this module's own header names as
/// fatal: *a dense tensor cannot tell "these two do not relate" from
/// "these two relate by zero", and that distinction is what carries
/// topology.* A balanced pair RELATES; erasing its edge deletes a real
/// cycle and with it real capacity.
///
/// Presence is tracked as the obligations are read and consulted here;
/// it is never recovered afterwards by a comparison.
pub fn observe(state: &Committed, i: usize, j: usize, channel: usize) -> Observed {
    let (_, support) = project_complex(state);
    if i >= support.dim() || j >= support.dim() || channel >= CHANNELS {
        return Observed::Unknown;
    }
    match support.has(i, j, channel) {
        true => Observed::Present,
        false => Observed::Absent,
    }
}

/// The residual field over FILLED TRIANGLES, enumerated from the
/// support rather than filtered out of every triple.
///
/// **THROUGHPUT.** `for_each_residual` visits every ordered distinct
/// triple — `O(n³·f)` — and on a sparse complex nearly all of them name
/// a triangle that does not exist. Measured on a ring: 40 orbs 61ms,
/// 80 orbs 409ms, 160 orbs 3.1s, 320 orbs 22.9s, growing ~7.3x per
/// doubling. The two-pole reading beside it is quadratic and takes
/// 127ms at the same 320 orbs.
///
/// Walking the support's own adjacency instead visits only triples
/// whose three edges are all present — `O(E·d)` — so a ring, which
/// fills no triangle at all, costs nothing to scan. Same residuals,
/// same order of discovery, same arithmetic:
/// `R_ijk,l = Δ_ik,l − Δ_ij,l − Δ_jk,l`.
fn filled_residuals(delta: &Delta, support: &Support) -> Vec<Residual> {
    let dim = delta.dim().min(support.dim());
    let fiber = delta.fiber().min(support.fiber());
    let zero = ratio(0, 1);
    let mut found = Vec::new();
    for l in 0..fiber {
        let adjacent: Vec<Vec<usize>> = (0..dim)
            .map(|i| (0..dim).filter(|&j| j != i && support.has(i, j, l)).collect())
            .collect();
        for (i, from) in adjacent.iter().enumerate() {
            for &j in from {
                for &k in &adjacent[j] {
                    if k == i || !support.has(i, k, l) {
                        continue;
                    }
                    let value = delta.entry(i, k, l).clone()
                        - delta.entry(i, j, l)
                        - delta.entry(j, k, l);
                    if value != zero {
                        found.push(Residual { i, j, k, l, value });
                    }
                }
            }
        }
    }
    found
}

/// Read the entangled pair from one prior state, in one pass
/// (`18_TORSION` §2f's binding build constraint).
///
/// Single-book reading: every pair among the book's own orbs is
/// observed, so `unknown` is zero by construction. For a reading that
/// spans books, carry the unobserved count through
/// [`read_entangled_observed`].
pub fn read_entangled(state: &Committed) -> EntangledRead {
    read_entangled_observed(state, 0)
}

/// The cross-book door: the same one-pass read, carrying the count of
/// pairs the observer could NOT see. While `unknown > 0` the complex
/// has holes of unknown shape and [`EntangledRead::capacity_is_defined`]
/// refuses — discharge them by [`Resolve::Telescope`] or
/// [`Resolve::LockOn`] before reading capacity, never by assuming
/// absence.
pub fn read_entangled_observed(state: &Committed, unknown: usize) -> EntangledRead {
    let (delta, support) = project_complex(state);
    // THE RESIDUAL FIELD, WHOLE, over FILLED TRIANGLES ONLY. Never
    // `coherence_functional` here — that sums the field into one scalar
    // and discards which triple produced each residual, the exact place
    // the first formations of symmetry were being lost (§2b bars the
    // collapse; D3 needs these to emit; the composition law needs them
    // to compose). And a triple with any absent edge is NOT a triangle:
    // checking it invents strain from a relation that does not exist,
    // which is the false gauge collapse.
    let residuals = filled_residuals(&delta, &support);
    let flows = boundary_form(&delta, &support).flows;
    // D — the independent cycles carrying charge no frame explains.
    let rank = holonomies(&delta, &support).len();
    EntangledRead { rank, residuals, flows, unknown }
}
