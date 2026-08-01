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
use rfa_core::{coboundary_reduce, coherence_functional, Delta};

/// BOND-1: the standing order, `source` → `target` at `rate`/tick.
pub const CHANNEL_BOND: usize = 0;
/// COVENANT-1: the vitrified pact's OUTSTANDING quota, obligor →
/// beneficiary (`quota − delivered`, floored at zero — a fulfilled
/// pact owes nothing and contributes nothing).
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

/// THE ENTANGLED READ (`18_TORSION` §2f). The capacity side and the
/// potential side, computed in ONE pass over ONE prior state. They are
/// not separately obtainable from this module by construction: a
/// caller that could take `D` alone and consult the strain separately
/// would have broken the entanglement and reintroduced the two-object
/// attack surface the law closes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntangledRead {
    /// `D` — the structure surviving `Π_G(Δ(Ω))`: the count of
    /// independent entries (i<j, every channel) still standing after
    /// coboundary reduction. Frame contributes nothing.
    pub capacity: usize,
    /// The cocycle defect `C(Δ) = Σ|R|` — the potential-side strain,
    /// naming how far the obligations are from composing.
    pub defect: Ratio,
    /// Pairs whose relation was NOT observed. While any stands, the
    /// complex has holes of unknown shape and no capacity may be read
    /// off it.
    pub unknown: usize,
}

impl EntangledRead {
    /// **`D` is well-defined only on a fully observed cocycle.** Two
    /// independent conditions, and both are refusals to read capacity
    /// out of something that has none to give:
    ///
    /// 1. `unknown == 0` — an unresolved absence would let phantom
    ///    topology mint phantom capacity. Discharge every `Unknown` by
    ///    [`Resolve`] first.
    /// 2. `defect == 0` — `H¹ = Z¹/B¹` is defined on cocycles alone;
    ///    where the defect is non-zero `Δ ∉ Z¹` and there is no
    ///    cohomology class for the reduction to land in.
    ///
    /// Callers that gate on capacity (Article II standing, Article IV
    /// monotonicity) MUST consult this first.
    pub fn capacity_is_defined(&self) -> bool {
        self.unknown == 0 && self.defect == ratio(0, 1)
    }
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
    let dim = state.orbs.len();
    let mut delta = Delta::zero(dim, CHANNELS);
    if dim < 2 {
        // One orb relates to nothing; zero orbs still less. Both are
        // lawful and both read as no structure.
        return delta;
    }
    let seat = |orb: u64| state.orbs.iter().position(|held| held.orb_id == orb);
    let zero = ratio(0, 1);
    let mut owed = vec![vec![vec![zero.clone(); CHANNELS]; dim]; dim];

    for bond in &state.bonds {
        if let (Some(i), Some(j)) = (seat(bond.source), seat(bond.target)) {
            if i != j {
                owed[i][j][CHANNEL_BOND] += bond.rate.clone();
            }
        }
    }
    for pact in &state.covenants {
        if let (Some(i), Some(j)) = (seat(pact.obligor), seat(pact.beneficiary)) {
            let outstanding = pact.quota.clone() - pact.delivered.clone();
            if i != j && outstanding > zero {
                owed[i][j][CHANNEL_COVENANT] += outstanding;
            }
        }
    }
    for debt in &state.debts {
        if let (Some(i), Some(j)) = (seat(debt.debtor), seat(debt.creditor)) {
            if i != j {
                owed[i][j][CHANNEL_DEBT] += debt.remainder.clone();
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
    delta
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
pub fn observe(state: &Committed, i: usize, j: usize, channel: usize) -> Observed {
    let delta = project(state);
    match delta.dim() > i.max(j) && channel < CHANNELS {
        false => Observed::Unknown,
        true => match *delta.entry(i, j, channel) != ratio(0, 1) {
            true => Observed::Present,
            false => Observed::Absent,
        },
    }
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
    let delta = project(state);
    let defect = coherence_functional(&delta);
    let reduced = coboundary_reduce(&delta);
    let zero = ratio(0, 1);
    let mut capacity = 0;
    for i in 0..reduced.dim() {
        for j in (i + 1)..reduced.dim() {
            for l in 0..reduced.fiber() {
                if *reduced.entry(i, j, l) != zero {
                    capacity += 1;
                }
            }
        }
    }
    EntangledRead { capacity, defect, unknown }
}
