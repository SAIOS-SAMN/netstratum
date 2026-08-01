//! THE CAGE — containment for everything that tried to be universal
//! (operator ruling 2026-08-01).
//!
//! This system is ALL-ENCOMPASSING: it must be able to represent dogma,
//! narrative, scalar summaries and transcendental readings, because
//! they exist and a universe that cannot represent them is not
//! all-encompassing. **The error was never that such things exist. It
//! was granting them UNIVERSAL SCOPE.**
//!
//! Every construct that has leaked into the deterministic core this
//! session announced itself the same way — by claiming to be THE
//! measure, THE driver, THE master:
//!
//! - `C(Δ)` claimed to be the measure of convergence, and became a
//!   scoring gate in `validate_step` and a readability gate in the
//!   kernel bridge.
//! - `ζ∇V` claimed to be the driver, was demoted to "a hint", and the
//!   objective leaked around it through C-scoring instead.
//! - "the master equation" claimed to be the one composition, and was
//!   struck outright.
//!
//! **THE CONTAINMENT LAW: a claim of universality is the warning, not
//! the credential.** Anything asserting universal scope is moved here,
//! where it may be computed and SHOWN and may govern nothing.
//!
//! ## The cage is ONE-WAY, and that is the whole mechanism
//!
//! Values enter as exact rationals and leave only as `String`. There is
//! deliberately NO accessor returning the inner value — no
//! `into_inner`, no `Deref`, no `as_ratio`. A single such method would
//! reopen every path this module closes, because a caller could escape,
//! compare, and gate exactly as before.
//!
//! The exit being `String` is the point: no rule can numerically branch
//! on a rendered string, so the boundary is enforced by types rather
//! than by a docstring asking politely. Docstrings saying *do not gate
//! on this* failed TWICE in one day — `validate_step`'s productivity
//! gate and the bridge's `capacity_is_defined` — and in both cases the
//! prose was right while the code did the opposite.
//!
//! ## What may live here
//!
//! Scalar aggregates (`C`, conservation distance, any L1 collapse), and
//! — because nothing escapes — readings a deterministic path could
//! never lawfully hold at all: floats, roots, transcendentals, sampled
//! or approximate values destined for a chart. **Inside the cage those
//! are permitted precisely because they cannot leak.** That is why
//! containment beats prohibition: it lets the system SAY more while
//! guaranteeing determinism, instead of pretending the un-sayable does
//! not exist.
//!
//! Nothing in a computing path may import from this module.

use cfe::ratio::Ratio;

/// A caged reading: showable, comparable for identity, and ordered by
/// nothing.
///
/// `Lens` derives `PartialEq` and deliberately NOT `PartialOrd`, and
/// that single omission is the cage's teeth:
///
/// - **Ordering is what descent needs.** `a < b` does not compile.
///   Neither does `> epsilon`, `<= bound`, `.min()`, `.max()`, or
///   `.sort()`. Every gate, threshold and penalty curve struck this
///   session is now a TYPE ERROR rather than a review comment.
/// - **Identity is what witnesses need.** `assert_eq!(c, Lens::of(..))`
///   still states exactly what a state measures. Knowing WHAT a reading
///   is cannot be descended; knowing which SIDE of something it falls
///   on can.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lens(Ratio);

impl Lens {
    /// The door IN. Exact rationals enter freely; nothing leaves.
    pub fn of(value: Ratio) -> Self {
        Lens(value)
    }

    /// The door OUT, and the only one: a rendering for a human surface.
    /// It lands in a `String` on purpose — no rule can numerically
    /// branch on it, so the cage holds without anyone having to
    /// remember that it should.
    pub fn show(&self) -> String {
        format!("{}", self.0)
    }
}

impl std::fmt::Display for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
