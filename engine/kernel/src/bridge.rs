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
use rfa_core::{coboundary_reduce_over, coherence_functional, Delta, Support};

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
    ///
    /// **A READING, NEVER A GATE** (DIRECTIVE 03, operator ruling
    /// 2026-08-01). `C(Δ) = 0` is a post-reduction canonical property,
    /// not an operational gatekeeper. This crate once carried a
    /// `capacity_is_defined()` predicate asserting that `D` is
    /// meaningless where the defect is non-zero, and instructed Article
    /// II and Article IV callers to consult it first. That was the
    /// struck `C`-gate re-entering as a method: `coboundary_reduce_over`
    /// is TOTAL over any input tensor — it does not test its input, it
    /// projects it — and the engine never forms the `Z¹/B¹` quotient
    /// that would need a cocycle to be well-defined. It reduces to a
    /// canonical representative and reads its extent.
    ///
    /// Standing is governed EXCLUSIVELY by non-zero post-reduction
    /// extent (`capacity > 0`). Non-composition ROOTS — it is an
    /// economic fact that carries structure — it is never a refusal.
    pub defect: Ratio,
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
    projection(state).0
}

/// The ACTIVE SUPPORT of the projection (DIRECTIVE 05): which orb pairs
/// carry an OBSERVED obligation, per channel.
///
/// This is the observation record `rfa_core::Support::of` cannot infer.
/// Two opposed bonds that net to exactly zero are a real, standing
/// relation whose VALUE is zero — structure the non-zero inference would
/// silently drop. The kernel knows the difference because it holds the
/// obligation lists themselves; nothing downstream has to guess.
///
/// A FULFILLED covenant is deliberately NOT marked: its outstanding
/// quota is zero, it owes nothing, and the standing law (`COVENANT-1`)
/// is that it contributes nothing. It is a closed relation, not an
/// observed one.
pub fn support_of(state: &Committed) -> Support {
    projection(state).1
}

/// Build the relation tensor and its observation mask in ONE pass over
/// ONE state — the two are the same reading and are never taken apart.
fn projection(state: &Committed) -> (Delta, Support) {
    let dim = state.orbs.len();
    let mut delta = Delta::zero(dim, CHANNELS);
    let mut support = Support::none(dim, CHANNELS);
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
                support.mark(i, j, CHANNEL_BOND);
            }
        }
    }
    for pact in &state.covenants {
        if let (Some(i), Some(j)) = (seat(pact.obligor), seat(pact.beneficiary)) {
            let outstanding = pact.quota.clone() - pact.delivered.clone();
            if i != j && outstanding > zero {
                owed[i][j][CHANNEL_COVENANT] += outstanding;
                support.mark(i, j, CHANNEL_COVENANT);
            }
        }
    }
    for debt in &state.debts {
        if let (Some(i), Some(j)) = (seat(debt.debtor), seat(debt.creditor)) {
            if i != j {
                owed[i][j][CHANNEL_DEBT] += debt.remainder.clone();
                support.mark(i, j, CHANNEL_DEBT);
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

/// Read the entangled pair from one prior state, in one pass
/// (`18_TORSION` §2f's binding build constraint).
///
/// `capacity` is `D` — the count of independent relations surviving
/// `Π_G` over the ACTIVE SUPPORT (DIRECTIVE 05). On the observed graph
/// this is the cycle rank `E − V + C`: a tree of obligations carries no
/// capacity, and only genuine cycles do.
pub fn read_entangled(state: &Committed) -> EntangledRead {
    let (delta, support) = projection(state);
    let defect = coherence_functional(&delta);
    let reduced = coboundary_reduce_over(&delta, &support);
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
    EntangledRead { capacity, defect }
}
