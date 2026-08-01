//! THE POTENTIAL 1-FORM — `dV`, uncaged (DIRECTIVE 10, operator ruling
//! 2026-08-01; `18_TORSION` §2b).
//!
//! **NO TRAVERSAL. NO BASIS. NO WALK.** (operator correction
//! 2026-08-01: *we are working in polynomials, not graphs, matrices, or
//! lattices*.) The first cut of this module walked a spanning forest
//! and read slopes off forest edges. That was a graph algorithm, and it
//! was not merely non-canonical — it was WRONG: the same state read
//! flat or sloping depending on which edges the walk happened to take
//! into the forest, because "the value on a forest edge" is not "the
//! gradient part of `Δ`".
//!
//! The gradient part is an algebraic invariant and needs no walk. A
//! 1-form splits canonically into a gradient part and a circulation
//! part, and the gradient part vanishes exactly when the form is
//! DIVERGENCE-FREE:
//!
//! ```text
//! div(Δ)_i,l  =  Σ_j Δ_ij,l          over the present edges at i
//!
//! dV ≡ 0   ⟺   div(Δ) = 0 at every vertex   (Δ is pure circulation)
//! dV ≢ 0   ⟺   some vertex carries net flow
//! ```
//!
//! One pass over the support. Traversal order cannot change it, no
//! spanning structure is chosen, and every judge computing it arrives
//! at the same answer — which is what a kernel invariant requires and
//! what the forest version could never provide.
//!
//! **It is a FORM, never a magnitude.** The divergence is returned
//! WHOLE as its components with their positions intact. It is never
//! summed, never normed, never collapsed to `‖∇V‖` — the law bars
//! collapsing a directional quantity to a scalar aggregate wherever the
//! system computes (`18_TORSION` §2b), and a form reduced to one number
//! is exactly that collapse.
//!
//! **THE DUAL-POLE LOCK.** Capacity and potential are the two poles of
//! one unpartitioned manifold (`Ω ≡ V`), and BOTH must stand
//! non-vacuous. Rank alone is not enough: a universe may hold `D > 0`
//! entirely in closed circulation while no vertex carries net flow.
//! That is STATIC STASIS, and it is refused — structure with no
//! gradient does no work.
//!
//! ```text
//! D = 0 ∧ dV ≡ 0   vacuous void        — refused
//! D = 0 ∧ dV ≢ 0   unframed potential  — refused
//! D > 0 ∧ dV ≡ 0   static stasis       — refused
//! D > 0 ∧ dV ≢ 0   entangled           — ADMITTED
//! ```
//!
//! Exact rational throughout: no float, no root, no transcendental.

use crate::delta::Delta;
use crate::reduce::Support;
use cfe::ratio::Ratio;
use num_traits::Zero;

/// ONE COMPONENT of the potential form: the NET FLOW at one vertex on
/// one coordinate. A position and an exact value, never an aggregate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flow {
    pub at: usize,
    pub l: usize,
    pub value: Ratio,
}

/// `dV` — the potential form, carried WHOLE as its non-zero components.
///
/// An empty form is pure circulation: every vertex balances, nothing
/// sources or sinks, and the field does no work.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PotentialForm {
    pub flows: Vec<Flow>,
}

impl PotentialForm {
    /// **`rank(dV)` — how many positions carry net flow.** Not a
    /// magnitude: two flows of opposite sign at different vertices do
    /// not cancel here, because they are different positions, not
    /// competing quantities.
    pub fn rank(&self) -> usize {
        self.flows.len()
    }

    /// **THE NON-VACUITY READ (DIRECTIVE 10).** `dV ≢ 0` iff some
    /// vertex carries net flow.
    ///
    /// A structural emptiness question about a set of positions, never
    /// a numeric comparison against zero. A form with no components IS
    /// pure circulation; nothing is measured to discover it.
    pub fn is_non_vacuous(&self) -> bool {
        !self.flows.is_empty()
    }
}

/// Derive `dV` from a state and its support — the divergence at every
/// vertex, in ONE pass.
///
/// Canonical by construction: no spanning structure is chosen and no
/// order is walked, so two judges always agree. Antisymmetry makes the
/// sum well-formed — `Δ_ij = −Δ_ji`, so an edge contributes `+` at one
/// end and `−` at the other, and a pure circulation cancels at every
/// vertex exactly.
///
/// Pure and total. An empty complex yields an empty form, lawfully.
pub fn potential_form(delta: &Delta, support: &Support) -> PotentialForm {
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
                flows.push(Flow { at: i, l, value: net });
            }
        }
    }
    PotentialForm { flows }
}

/// THE DUAL-POLE VERDICT (DIRECTIVE 10). The four rows of the
/// non-vacuity matrix, as one typed reading over ONE state.
///
/// **`dV` CANNOT BE ZERO** (operator ruling 2026-08-01). A flat field is
/// not a lawful state to sit in: without gradient the manifold is static
/// flat space and any rank it holds is meaningless. This is an
/// invariant, not a preference.
///
/// Note what this verdict does NOT do: it does not discard. Under THE
/// BIFURCATION LAW a refused state is a ROOT, and [`Vacuity::Stasis`]
/// names exactly what must be re-volumized — the positions are carried
/// so the restoration has something to act on. A verdict that returned
/// only "no" would be the discard the law forbids.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vacuity {
    /// Neither pole stands: no structure and no gradient.
    Void,
    /// Gradient without structure — nothing to anchor the field to.
    Unframed,
    /// Structure with no gradient: pure circulation, doing no work.
    /// **The state that must be RE-VOLUMIZED**, never merely refused.
    Stasis,
    /// Both poles stand. The only admitted row.
    Entangled,
}

/// Read both poles of one state at once and classify it.
///
/// `rank` is the count of independent charged cycles
/// (`holonomies(..).len()`); `form` is this module's divergence read.
/// Neither is consulted alone — the entanglement is the point.
pub fn dual_pole(rank: usize, form: &PotentialForm) -> Vacuity {
    match (rank > 0, form.is_non_vacuous()) {
        (false, false) => Vacuity::Void,
        (false, true) => Vacuity::Unframed,
        (true, false) => Vacuity::Stasis,
        (true, true) => Vacuity::Entangled,
    }
}

impl Vacuity {
    /// Only the entangled row stands.
    pub fn is_admitted(&self) -> bool {
        matches!(self, Vacuity::Entangled)
    }
}
