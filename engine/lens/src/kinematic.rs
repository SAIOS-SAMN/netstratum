//! The kinematic pressure — the strain a twist generates
//! (`19_LENS` §11, operator ruling 2026-08-02).
//!
//! The ancestor draws the layer this system was missing, and draws it
//! exactly (`/opt/ipsal-autic-v0.1`, `umd_at_cognition_layer.md` §2):
//!
//! | layer | content |
//! |---|---|
//! | **manifold** | **torsion = the twist event** — a non-trivial element of `π₁(ℳ)` |
//! | **kinematic** | **the pressure of that twist** — the strain it generates |
//!
//! *"Dual witnesses of the same event."*
//!
//! **Netstratum had the first and not the second.** `orbit_lock`,
//! `holonomies` and the fold gate all detect that a twist is. Nothing
//! read how hard it pushes. A structure could be enormous and utterly
//! static and no reading distinguished it from one straining to move.
//!
//! # The pressure is the curvature, and it was already landed
//!
//! `rfa_core::fiber` (Directive 17) carries the whole primitive set, and
//! its own header states why a flat fiber could never have shown this:
//!
//! > *With a scalar connection the curvature is `F = dA` and the bracket
//! > vanishes identically: `[A, A] = 0`. A closed loop of scalars
//! > carries no field at all — walking it sums to zero and the loop is
//! > a flat, dead circle. With a matrix connection the curvature keeps
//! > its second term, `F = dA + A ∧ A`, and `[A_i, A_j] ≠ 0` in general.
//! > **The circulation is the field.**
//!
//! So the pressure of a twist is `plaquette_curvature(a, b)` — the
//! commutator — and a reading of zero is not "no measurement" but the
//! honest statement that these two directions commute and the cell holds
//! no flux.
//!
//! **The holonomy is an ordered product, not a sum.** A loop that
//! returns the identity closed; one that does not is holding flux. That
//! distinction is what makes a ring a flywheel rather than a remnant, and
//! it is unavailable to any abelian reading.
//!
//! # What is translated, not transcribed
//!
//! The ancestor's kinematic layer reports
//! `KineticStep::Converged { action_magnitude }` and derives a velocity
//! `v(T) = 1/(1+‖Δₜ‖) − 1/(1+‖Δₜ₋₁‖)`. **Both are barred**: the second
//! is the `1/(1+x)` shape of the struck cosmetic meter, and both collapse
//! a directional quantity to a scalar magnitude, which `24_FORM` §2b
//! forbids wherever the system computes.
//!
//! The idea survives whole and is worth having — pressure and velocity
//! derived from the chain alone, with no future state. **The lawful form
//! carries them per component.** [`Pressure`] holds the curvature as the
//! `2×2` it is; [`Drift`] holds a per-component difference across two
//! chain points. Neither is ever a length. Where a magnitude is wanted
//! for display it goes through the operator display bound (§1.0) —
//! caged, rendered, never returned as a number.

use cfe::ratio::{ratio, Ratio};
use rfa_core::{loop_holonomy, plaquette_curvature, MatrixFiber2x2};

/// The pressure of A twist — the curvature of a plaquette, carried
/// whole.
///
/// Read on the smallest closed cell a connection can carry. A zero
/// reading is a fact, not an absence: these two directions commute, and
/// the cell holds no flux.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pressure {
    /// `F = dA + A ∧ A` on this cell, per component. **Never a norm** —
    /// a caller that wants to know which way the cell strains needs the
    /// matrix, and a length cannot say.
    pub curvature: MatrixFiber2x2,
}

impl Pressure {
    /// Read the pressure between two directions of a connection.
    pub fn between(a: &MatrixFiber2x2, b: &MatrixFiber2x2) -> Self {
        Pressure {
            curvature: plaquette_curvature(a, b),
        }
    }
}

/// The phase lock — what a closed walk is holding.
///
/// The holonomy of a loop is the ordered product of its connections.
/// **Identity means the walk closed**: it returned where it began and
/// the lock costs nothing to hold. Anything else is flux the loop is
/// carrying, and carrying it is what the pressure measures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhaseLock {
    /// The accumulated transport around the walk, whole.
    pub holonomy: MatrixFiber2x2,
}

impl PhaseLock {
    /// Walk a closed loop and keep what it accumulated.
    ///
    /// An empty walk holds the identity, lawfully — nowhere to go,
    /// nothing acquired.
    pub fn around(walk: &[MatrixFiber2x2]) -> Self {
        PhaseLock {
            holonomy: loop_holonomy(walk),
        }
    }

    /// **Did the walk close?** The loop returned exactly where it began
    /// and is holding no flux.
    pub fn closed(&self) -> bool {
        self.holonomy.is_identity()
    }
}

/// The drift — how a reading moved between two points on the chain.
///
/// The lawful form of the ancestor's coherence velocity. It is derived
/// from the chain alone with no future state, exactly as Theorem β
/// intended — but it is carried **per component** rather than collapsed
/// to a signed magnitude, so a reader learns which way the structure
/// moved and not merely how far.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Drift {
    /// `later − earlier`, component by component.
    pub delta: MatrixFiber2x2,
}

impl Drift {
    /// Read the drift between two chain points.
    ///
    /// **Two points from the chain, never a private history** — a lens
    /// that kept its own timeline would be a second record (§1a). The
    /// caller supplies both readings; this holds neither.
    pub fn between(earlier: &MatrixFiber2x2, later: &MatrixFiber2x2) -> Self {
        Drift {
            delta: later.minus(earlier),
        }
    }

    /// One component of the movement, exact. `(row, col)` outside the
    /// `2×2` reads as no movement — the reading is total.
    pub fn component(&self, row: usize, col: usize) -> Ratio {
        match row < 2 && col < 2 {
            true => self.delta.m[row][col].clone(),
            false => ratio(0, 1),
        }
    }
}
