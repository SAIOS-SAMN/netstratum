//! **D4 OVER A WINDOW — the revocation reading** (EQUATIONS D4, the
//! lens half; landed 2026-08-04).
//!
//! D4 is the revocation law: the site where a claim on capacity that
//! never entangled with potential is VACATED from the frame. It fires
//! in two places and **they must never merge**:
//!
//! ```text
//! PER-STEP (kernel)    a deed that mints carries a non-zero capacity
//!                      increment, or REFUSES. Local, judgeable.
//!                      `kernel::revocation_law`.
//! OVER A WINDOW (lens) deeds(w) > 0 ∧ ΔD(w) = 0 — the allocation
//!                      claim VACATES; the degree is untouched.
//!                      THIS MODULE.
//! ```
//!
//! **Why the split is load-bearing and not tidiness.** A rate never
//! attains its bound at any single step, so a window can never be an
//! invariant — hoisting this reading into the judge would make the
//! kernel history-dependent, which it is not and must not become. The
//! kernel is pure over `(prior, next, step)`; a window is a story about
//! many steps and belongs to whoever reads them.
//!
//! **THIS REPORTS; IT NEVER GATES.** Rule 5. Delete this module and the
//! judge is unchanged — `kernel::revocation_law` reaches its own verdict
//! from its own read, per step. What is added here is only that an
//! operator can SEE a window in which deeds were emitted while nothing
//! grew.
//!
//! **THE CLAIM VACATES; THE DEGREE DOES NOT.** Nothing here lowers
//! anything. `D` never retreats (`18` §3 Article IV) and
//! [`rfa_core::Degree`] has no below-ground representation to retreat
//! into. A vacated window is a reading about a CLAIM, not a debit.
//!
//! **NO SCALAR METER.** `Ξ(w) = deeds(w)/(1 + ΔD(w))` was this row's
//! window arithmetic and is STRUCK — a single number would rank windows
//! against each other and re-introduce exactly the scalar authority
//! `18` §3 struck. The reading is a typed row plus the two counts it
//! was taken from, so a reader sees WHY and can check it.
//!
//! **Deliberately built on `holonomies` alone.** The circulation count
//! is the whole input; this module does not touch the dual-pole verdict
//! machinery, which is under strike elsewhere. The reading survives that
//! strike unchanged because it never depended on it.

use rfa_core::{holonomies, Delta, Support};

/// What happened to the capacity CLAIM over one window.
///
/// Three rows, and none of them is a score. Two of them are not faults:
/// a quiet window claimed nothing, and a grown window earned what it
/// claimed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Claim {
    /// No deeds in the window. Nothing was claimed, so nothing vacates.
    /// A still world is not a delinquent one.
    Quiet,
    /// Deeds were emitted AND circulation grew. The claim stood: this
    /// is D3's leg, seen from D4's side.
    Stood,
    /// **THE REVOCATION.** Deeds were emitted while the rank did not
    /// grow — `deeds(w) > 0 ∧ ΔD(w) = 0`. The allocation claim is
    /// vacated; the degree itself is untouched, and the entity is
    /// neither barred nor deleted (Article III's guardrail 6: it still
    /// acts, still emits receipts, still tells its story).
    Vacated,
}

impl Claim {
    /// Did the window's claim on capacity survive?
    ///
    /// A READING, never a gate — no caller may use this to refuse a
    /// deed. The judge refuses per step, from its own read.
    pub fn stood(&self) -> bool {
        matches!(self, Claim::Stood | Claim::Quiet)
    }

    /// What this row means, in one line, for an operator surface.
    /// Stated as a state rather than a fault — three rows, no verdicts
    /// on anybody's character.
    pub fn reads(&self) -> &'static str {
        match self {
            Claim::Quiet => "no deeds — nothing was claimed",
            Claim::Stood => "deeds emitted and circulation grew — the claim stood",
            Claim::Vacated => "deeds emitted while nothing grew — the allocation claim vacates",
        }
    }
}

/// One window's revocation reading, carried with the counts it came
/// from so a reader can check it rather than trust it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Revocation {
    pub claim: Claim,
    /// Deeds emitted across the window.
    pub deeds: usize,
    /// Independent circulation at the window's open — the gauge-
    /// invariant part, the part that grows.
    pub circulation_before: usize,
    /// Independent circulation at the window's close.
    pub circulation_after: usize,
}

impl Revocation {
    /// Read a window from its two ends and the deeds between them.
    ///
    /// Pure and total; derives at call and retains nothing.
    ///
    /// The increment is read on the CIRCULATION — what stands above the
    /// ground — never on the full `Degree`, which includes the floor.
    /// A comparison against the floor would be vacuous in exactly the
    /// way the struck `D > 0` gate was: a floor decides nothing.
    pub fn over(
        deeds: usize,
        open: (&Delta, &Support),
        close: (&Delta, &Support),
    ) -> Revocation {
        let circulation_before = holonomies(open.0, open.1).len();
        let circulation_after = holonomies(close.0, close.1).len();
        Revocation::from_counts(deeds, circulation_before, circulation_after)
    }

    /// The same reading from counts already in hand — for a caller that
    /// has measured circulation by another lawful route, and for pins.
    pub fn from_counts(
        deeds: usize,
        circulation_before: usize,
        circulation_after: usize,
    ) -> Revocation {
        // `saturating_sub` is not a clamp hiding a negative: D never
        // retreats (Article IV), so a fall is unrepresentable rather
        // than merely unexpected. If one is ever observed it reads as
        // no growth, which vacates — the conservative direction.
        let grew = circulation_after.saturating_sub(circulation_before) > 0;
        let claim = match (deeds > 0, grew) {
            (false, _) => Claim::Quiet,
            (true, true) => Claim::Stood,
            (true, false) => Claim::Vacated,
        };
        Revocation { claim, deeds, circulation_before, circulation_after }
    }

    /// The growth the window actually showed. Zero is a lawful and
    /// meaningful answer here — it is precisely what vacates a claim.
    pub fn growth(&self) -> usize {
        self.circulation_after.saturating_sub(self.circulation_before)
    }
}
