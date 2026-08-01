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
//! ## The reserved cells, and what is NOT in them
//!
//! Two cells of the containment vector stand EMPTY and reserved for the
//! corrupted `ζ(V)` fragments the ruling names. They are empty because
//! **`ζ∇V` is struck and nothing computes it** — there is no live
//! quantity to place, and filling them with invented data would be the
//! speculative apparatus this session has spent the day removing. The
//! slots are named so the shape is honest; the debt is recorded rather
//! than papered over.
//!
//! ## WHAT DOES **NOT** BELONG HERE — aggregate versus invariant
//!
//! (Operator question 2026-08-01: does the holonomy upgrade fit in this
//! containment? **No — and putting it here would strand exactly what it
//! was meant to free.**)
//!
//! The cage is for AGGREGATES USED AS SCORES. It is not for scalars in
//! general, and the difference is structural rather than a matter of
//! size:
//!
//! ```text
//! AGGREGATE   C = Σ|R| — collapses MANY into one and destroys which
//!             residual broke and by how much. The information is gone,
//!             and what remains is rankable. CAGED.
//!
//! INVARIANT   rcf_signature, a holonomy's conjugacy class — the exact
//!             canonical fingerprint of ONE object. Destroys nothing:
//!             two Deltas share it IFF they are conjugate. NOT CAGED.
//! ```
//!
//! **The proof that the line is real is already in this crate.**
//! [`Lens`] and [`crate::rcf::RcfSignature`] carry the SAME derives —
//! `PartialEq, Eq` and deliberately no `PartialOrd`. Their algebra is
//! identical. The only difference between them is `cage_arrow`, which
//! bars `Lens` from every computing path while `rcf_signature` is called
//! directly by `orbit_lock`.
//!
//! So the no-ordering property — the thing that actually stops descent —
//! is available WITHOUT containment. **A canonical invariant follows the
//! `RcfSignature` pattern: equality-only, uncaged, kernel-eligible.**
//! Containment is a second, heavier act reserved for readings that
//! compute nothing.
//!
//! **A note on SL₂, because the escape route does not exist.** For a
//! matrix of determinant 1 the characteristic polynomial is
//! `x² − Tr(A)·x + 1`, so the conjugacy class is essentially
//! parameterised BY the trace. "Keep the class instead of the trace"
//! does not avoid a scalar — it renames one. The lawful move was never
//! to avoid scalars; it is to forbid ORDERING them and to keep them as
//! IDENTITY. That is what both types already do.
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
pub struct Lens {
    /// THE CONTAINMENT VECTOR, flattened to 2×2 (operator ruling
    /// 2026-08-01). The caged value is NEVER held in a cell.
    ///
    /// A canonical `Ratio` is `p/q` with `gcd(p,q) = 1` and `q > 0`, so
    /// the pair `(p, q)` is unique. The cage stores them at MAXIMUM
    /// INVERSE OPPOSITION — numerator and denominator on opposite
    /// diagonal corners — so that:
    ///
    /// - no single cell approximates the value, in magnitude or sign;
    /// - reassembly requires the ASSEMBLY RULE, which exists only
    ///   inside [`Lens::show`] and is never exported.
    ///
    /// This is the cage's SECOND line. The first is that nothing
    /// escapes; the second is that what is inside is not a usable
    /// scalar even if it did. A leak yields components, never a number.
    ///
    /// ```text
    ///     ┌               ┐
    ///     │  p      ·     │   ·  = reserved (see below)
    ///     │  ·      q     │   p  = numerator   ┐ maximally
    ///     └               ┘   q  = denominator ┘ separated
    /// ```
    cell: [[Ratio; 2]; 2],
}

impl Lens {
    /// The door IN. Exact rationals enter freely; nothing leaves.
    ///
    /// The value is decomposed on entry and is not stored anywhere as
    /// itself.
    pub fn of(value: Ratio) -> Self {
        let zero = Ratio::from(num_bigint::BigInt::from(0));
        let numerator = Ratio::from(value.numer().clone());
        let denominator = Ratio::from(value.denom().clone());
        Lens {
            cell: [
                [numerator, zero.clone()],
                [zero, denominator],
            ],
        }
    }

    /// The door OUT, and the only one: a rendering for a human surface.
    /// It lands in a `String` on purpose — no rule can numerically
    /// branch on it, so the cage holds without anyone having to
    /// remember that it should.
    ///
    /// This is the ONLY place the assembly rule exists.
    pub fn show(&self) -> String {
        let numerator = self.cell[0][0].numer().clone();
        let denominator = self.cell[1][1].numer().clone();
        format!("{}", Ratio::new(numerator, denominator))
    }
}

impl std::fmt::Display for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.show())
    }
}

/// THE CLOSED ALGEBRA — C's own `V`, sealed (operator ruling
/// 2026-08-01: *C cannot leak into D and needs to find his own V*).
///
/// A cage with no interior life invites someone to open it, because the
/// value inside is still wanted for something. So `C` is not merely
/// held — it is given its OWN universe, closed under its own
/// operations, in which it may be combined, accumulated and rendered
/// freely and from which nothing reaches the manifold.
///
/// **The sealing property: every operation on a `Lens` yields a
/// `Lens`.** There is no combination, no fold, no accumulation that
/// produces a `Ratio`, a `bool`, an ordering, or anything the engine
/// computes on. C may have arbitrary structure inside its own space and
/// zero morphisms out of it.
///
/// This is what makes the containment total rather than merely
/// obstructive. C is not starved; it is exiled with everything it needs.
impl std::ops::Add for Lens {
    type Output = Lens;

    /// Combine two caged readings. Stays caged, forever.
    fn add(self, other: Lens) -> Lens {
        let left = Ratio::new(self.cell[0][0].numer().clone(), self.cell[1][1].numer().clone());
        let right = Ratio::new(other.cell[0][0].numer().clone(), other.cell[1][1].numer().clone());
        Lens::of(left + right)
    }
}

impl std::iter::Sum for Lens {
    /// Accumulate caged readings — C's own aggregation, inside C's own
    /// space. Note what this CANNOT feed: no engine path accepts a
    /// `Lens`, so an accumulated C has nowhere to go but a rendering.
    fn sum<I: Iterator<Item = Lens>>(iter: I) -> Lens {
        iter.fold(Lens::of(Ratio::from(num_bigint::BigInt::from(0))), |a, b| a + b)
    }
}
