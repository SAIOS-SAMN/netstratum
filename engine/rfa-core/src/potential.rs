//! THE POTENTIAL 1-FORM — `dV`, uncaged (DIRECTIVE 10, operator ruling
//! 2026-08-01; `18_TORSION` §2b).
//!
//! `V` is the potential landscape a reduction assigns: walk a spanning
//! forest of the present edges, give each root potential zero, and
//! every vertex acquires an exact rational potential. `dV` is that
//! landscape's differential — the slope carried across each forest
//! edge:
//!
//! ```text
//! dV(i → j, l)  =  p_j − p_i        (exact rational, over ℚ)
//! ```
//!
//! **It is a FORM, never a magnitude.** `dV` is returned WHOLE as its
//! components with their positions intact. It is never summed, never
//! normed, never collapsed to `‖∇V‖`, and nothing here compares it to
//! anything — the law bars collapsing a directional quantity to a
//! scalar aggregate wherever the system computes (`18_TORSION` §2b),
//! and a 1-form reduced to one number is exactly that collapse.
//!
//! **KNOWN LIMITATION, recorded rather than hidden.** The spanning
//! forest is TRAVERSAL-DEPENDENT, and `dV` is read off the forest, so
//! the same state can read flat under one forest and sloping under
//! another (pinned in `potential_pins.rs`). Holonomy VALUES are class
//! invariants and do not suffer this; `dV`'s non-vacuity does. **This
//! reading may not become a kernel invariant until it is given a
//! forest-independent form.** It stands as a lens-side reading today.
//!
//! **THE DUAL-POLE LOCK.** Capacity and potential are the two poles of
//! one unpartitioned manifold (`Ω ≡ V`), and BOTH must stand
//! non-vacuous. Capacity alone is not enough: a universe may carry
//! `D > 0` entirely in closed loops while its potential landscape is
//! flat everywhere — every forest edge zero, no slope anywhere. That is
//! STATIC STASIS, and it is refused. Structure with no gradient does no
//! work.
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

/// ONE COMPONENT of the potential 1-form: the slope the landscape
/// carries from `i` to `j` on coordinate `l`. A position and an exact
/// value, never an aggregate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slope {
    pub i: usize,
    pub j: usize,
    pub l: usize,
    pub value: Ratio,
}

/// `dV` — the potential 1-form, carried WHOLE.
///
/// Holds every non-zero slope with its position. An empty form is a
/// FLAT landscape: the potential is constant across every component of
/// the complex, and the field does no work.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PotentialForm {
    pub slopes: Vec<Slope>,
}

impl PotentialForm {
    /// **`rank(dV)` — the count of independent directions the landscape
    /// actually slopes in.** Not a magnitude: adding two slopes of
    /// opposite sign does not cancel here, because they are different
    /// directions, not competing quantities.
    pub fn rank(&self) -> usize {
        self.slopes.len()
    }

    /// **THE NON-VACUITY READ (DIRECTIVE 10).** `dV ≢ 0` iff at least
    /// one component slopes: `rank(dV) ≥ 1`.
    ///
    /// This is a structural emptiness question about a set of
    /// positions, never a numeric comparison against zero. A form with
    /// no components IS flat; nothing is measured to discover it.
    pub fn is_non_vacuous(&self) -> bool {
        !self.slopes.is_empty()
    }
}

/// Derive `dV` from a state and its support.
///
/// Walks the same spanning forest [`crate::holonomies`] walks — give
/// each root potential zero, propagate along present edges — and
/// records the slope of every forest edge that is not flat. Forest
/// edges are exactly where the landscape is DEFINED; the remaining
/// edges carry holonomy instead, which is the capacity pole and is read
/// there, not here.
///
/// Pure and total. An empty complex yields an empty form, lawfully.
pub fn potential_form(delta: &Delta, support: &Support) -> PotentialForm {
    let dim = delta.dim().min(support.dim());
    let fiber = delta.fiber().min(support.fiber());
    let mut slopes = Vec::new();

    for l in 0..fiber {
        let mut potential: Vec<Option<Ratio>> = vec![None; dim];
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
                    let slope = delta.entry(here, next, l).clone();
                    potential[next] = Some(base.clone() + slope.clone());
                    if !slope.is_zero() {
                        slopes.push(Slope { i: here, j: next, l, value: slope });
                    }
                    frontier.push(next);
                }
            }
        }
    }
    PotentialForm { slopes }
}
