//! The peel — magnification as a depth on the coboundary decomposition
//! (`19_LENS` §3, operator ruling 2026-08-02).
//!
//! A lens is turned on a relation to resolve it, and the only thing that
//! varies is how close the observer stands. This module is that choice, and it
//! invents no geometry to make it:
//!
//! ```text
//!     FAR    the gauge-invariant class      WHAT stands      telescopic
//!     MID    the charged cycles, located    WHERE it strains
//!     NEAR   every standing entry, exact    HOW assembled    microscopic
//! ```
//!
//! **Each outer depth constrains the inner.** The far view finds what
//! changed, the middle finds where, the near finds how — and they are
//! not three computations but three readings of one decomposition the
//! algebra already performs. `rfa_core::holonomies` reduces against a
//! spanning forest and returns the charge each surviving edge carries;
//! far counts those charges, mid names them, near reads past them to the
//! entries themselves.
//!
//! **Why zooming out is lawful rather than lossy.** The far view is not
//! a blurred copy of the near view — it is the GAUGE-invariant content
//! of it. Stepping back removes FRAME, never precision. That is what
//! makes magnification a reading instead of a rendering trick.
//!
//! # One DEPTH is asked for; one DEPTH is computed
//!
//! [`peel`] returns a [`View`] with exactly one arm populated. It does
//! not return a struct holding all three layers — the ancestor's shape
//! (`NestedPeelLayers { value, object, cell }`) computes every layer
//! eagerly, and eager computation of an unobserved depth is the thing
//! `19_LENS` §4 exists to forbid.
//!
//! **The saving is in materialisation, not in the reduction.** Stated
//! honestly: finding the charges costs the same whichever depth is
//! asked for. What differs by orders of magnitude is what gets built and
//! carried — `Far` is one number; `Near` is `O(dim² · fiber)` entries.
//! On a 320-orb book that is three numbers against hundreds of
//! thousands. An observable universe does not avoid looking; it avoids
//! materialising what nobody is looking at.
//!
//! # What this module may not do
//!
//! - It **stores nothing**. Every view derives at call and is discarded;
//!   a retained view is the second image `00_CHARTER` §3a bans.
//! - It **decides nothing**. No caller may branch on a view — held by
//!   `deletability::the_conductor_renders_but_never_decides_on_a_reading`.
//! - It **is lazy, never memoised** (`19_LENS` §4). No cache lives here,
//!   and none may be added: a cache is a copy outside the chain.

use rfa_core::capacity::Degree;
use rfa_core::{holonomies, Delta, Holonomy, Support};

use cfe::ratio::Ratio;

/// How close the observer is standing.
///
/// Not a quality setting — a POSITION. Each depth answers a different
/// question about the same relation, and the answers are consistent by
/// construction (pinned in `peel_pins::the_depths_agree`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Depth {
    /// Telescopic. What structure stands, as one elevation.
    Far,
    /// Where the relation strains — the charged cycles, located.
    Mid,
    /// Microscopic. Every standing entry, exactly as held.
    Near,
}

/// One standing coordinate of a relation, with the place that holds it.
///
/// Carried whole rather than summed: `24_FORM` §2b bars collapsing a
/// directional quantity to a scalar aggregate wherever the system
/// computes, and a lens that pre-summed its entries would hand the
/// caller a number it could not take apart again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub i: usize,
    pub j: usize,
    pub l: usize,
    pub value: Ratio,
}

/// A reading at one depth. Exactly one arm, because exactly one depth
/// was asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    /// Far — the elevation this relation stands at. Never zero: a
    /// relation that circulates nowhere still stands on the ground
    /// (`rfa_core::Degree`).
    Far(Degree),
    /// Mid — the independent charges, each with the pair and channel
    /// that carries it.
    Mid(Vec<Holonomy>),
    /// Near — every standing entry over the independent half `i < j`.
    /// `Δ_ji = −Δ_ij` carries no additional structure, so reporting it
    /// would be reporting the same fact twice.
    Near(Vec<Entry>),
}

/// **The peel.** Read `delta` at one magnification.
///
/// Pure, total, and stateless. Takes the relation it is given — the
/// projection from committed state belongs to whoever holds the state,
/// which keeps this crate free of any dependency on the gate.
///
/// The `support` is the observation record: which pairs carry an
/// observed relation. An absent pair is absent, not zero, and reading it
/// as zero would manufacture topology that was never receipted.
pub fn peel(delta: &Delta, support: &Support, depth: Depth) -> View {
    match depth {
        // Far — count what stands above the floor, and report the
        // elevation. Nothing else is built.
        Depth::Far => View::Far(Degree::above_ground(holonomies(delta, support).len())),

        // Mid — the same charges, kept whole with their indices. This is
        // where a reader learns which pair strains, which the elevation
        // alone cannot say.
        Depth::Mid => View::Mid(holonomies(delta, support)),

        // Near — past the reduction to the entries themselves. The only
        // depth that costs its own size.
        Depth::Near => {
            let dim = delta.dim().min(support.dim());
            let fiber = delta.fiber().min(support.fiber());
            let zero = Ratio::from_integer(0.into());
            let mut entries = Vec::new();
            for i in 0..dim {
                for j in (i + 1)..dim {
                    for l in 0..fiber {
                        if !support.has(i, j, l) {
                            continue; // absent is absent, never zero
                        }
                        let value = delta.entry(i, j, l);
                        if *value == zero {
                            continue; // observed, and carries nothing
                        }
                        entries.push(Entry { i, j, l, value: value.clone() });
                    }
                }
            }
            View::Near(entries)
        }
    }
}

impl View {
    /// The depth this view was read at — so a caller that passes a view
    /// onward cannot lose track of how close it was standing.
    pub fn depth(&self) -> Depth {
        match self {
            View::Far(_) => Depth::Far,
            View::Mid(_) => Depth::Mid,
            View::Near(_) => Depth::Near,
        }
    }

}
