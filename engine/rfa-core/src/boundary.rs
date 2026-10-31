//! The boundary 1-FORM — `∂Δ`, uncaged (directive 10, operator ruling
//! 2026-08-01; `24_FORM` §2b).
//!
//! **No traversal. No basis. No walk.** (operator correction
//! 2026-08-01: *the work is in polynomials, not graphs, matrices, or
//! lattices*.) The first cut of this module walked a spanning forest
//! and read slopes off forest edges. That was a graph algorithm, and it
//! was not merely non-canonical — it was wrong: the same state read
//! flat or sloping depending on which edges the walk happened to take
//! into the forest, because "the value on a forest edge" is not "the
//! gradient part of `Δ`".
//!
//! The gradient part is an algebraic invariant and needs no walk. A
//! 1-form splits canonically into a gradient part and a circulation
//! part, and the gradient part vanishes exactly when the form is
//! divergence-free:
//!
//! ```text
//! div(Δ)_i,l  =  Σ_j Δ_ij,l          over the present edges at i
//!
//! ∂Δ ≡ 0   ⟺   div(Δ) = 0 at every vertex   (Δ is pure circulation)
//! dV ≢ 0   ⟺   some vertex carries net flow
//! ```
//!
//! One pass over the support. Traversal order cannot change it, no
//! spanning structure is chosen, and every gate computing it arrives
//! at the same answer — which is what a kernel invariant requires and
//! what the forest version could never provide.
//!
//! **It is a form, never a magnitude.** The divergence is returned
//! whole as its components with their positions intact. It is never
//! summed, never normed, never collapsed to `‖∂Δ‖` — the law bars
//! collapsing a directional quantity to a scalar aggregate wherever the
//! system computes (`24_FORM` §2b), and a form reduced to one number
//! is exactly that collapse.
//!
//! **The dual-pole lock is struck** (operator ruling 2026-08-04). This
//! module classified a state on two poles — circulation and net flow —
//! into four rows (`Void`, `Unframed`, `Stasis`, `Entangled`), and the
//! kernel refused a work-anchored mint on any row but the last. Both
//! the type and its verdict are removed.
//!
//! **The ruling, in the operator's terms:** *identity is pegged to
//! entity, and identity gets recycled without bonds.* The classification
//! read standing off relations — bonds, covenants, absorption debts —
//! and so demanded of a world precisely what a recycled identity does
//! not carry. NOVEL-1 succession and INCURSION-1a both produce a line
//! owing nothing and owed nothing by design; the lock refused it for
//! being what it lawfully is.
//!
//! Measured before the ruling (`harness-lane`, 2026-08-04): the admitted
//! row needed a three-orb obligation triangle that does not compose, a
//! shape no world the game produces can form — 60 beats of the live
//! board gave 374 solves, 0 mints, 124 withheld.
//!
//! **What remains here is the reading, which was never the problem.**
//! [`boundary_form`] is `∂Δ` carried whole, and the divergence is a
//! first-class reading for the sign law (`24_FORM` §2d) and the
//! object `capacity.rs` splits against to build the circulation. A
//! reading is not a gate; removing the gate does not touch it.
//!
//! Exact rational throughout: no float, no root, no transcendental.

use crate::delta::Delta;
use crate::reduce::Support;
use cfe::ratio::Ratio;
use num_traits::Zero;

/// One component of the boundary form: the net flow at one vertex on
/// one coordinate. A position and an exact value, never an aggregate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flow {
    pub at: usize,
    pub l: usize,
    pub value: Ratio,
}

/// `∂Δ` — the boundary form, carried whole as its non-zero components.
///
/// An empty form is pure circulation: every vertex balances, nothing
/// sources or sinks, and the field does no work.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BoundaryForm {
    pub flows: Vec<Flow>,
}

impl BoundaryForm {
    /// **`rank(dV)` — how many positions carry net flow.** Not a
    /// magnitude: two flows of opposite sign at different vertices do
    /// not cancel here, because they are different positions, not
    /// competing quantities.
    pub fn rank(&self) -> usize {
        self.flows.len()
    }
}

/// Derive `∂Δ` from a state and its support — the divergence at every
/// vertex, in one pass.
///
/// Canonical by construction: no spanning structure is chosen and no
/// order is walked, so two gates always agree. Antisymmetry makes the
/// sum well-formed — `Δ_ij = −Δ_ji`, so an edge contributes `+` at one
/// end and `−` at the other, and a pure circulation cancels at every
/// vertex exactly.
///
/// Pure and total. An empty complex yields an empty form, lawfully.
pub fn boundary_form(delta: &Delta, support: &Support) -> BoundaryForm {
    let dim = delta.dim().min(support.dim());
    let fiber = delta.fiber().min(support.fiber());
    let mut flows = Vec::new();

    for i in 0..dim {
        for l in 0..fiber {
            let mut net = Ratio::zero();
            for j in 0..dim {
                if j != i && support.has(i, j, l) {
                    net += delta.entry(i, j, l);
                }
            }
            if !net.is_zero() {
                flows.push(Flow {
                    at: i,
                    l,
                    value: net,
                });
            }
        }
    }
    BoundaryForm { flows }
}
