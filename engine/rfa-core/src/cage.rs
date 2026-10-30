//! The cage — containment for everything that tried to be universal
//! (operator ruling 2026-08-01).
//!
//! This system is ALL-encompassing: it must be able to represent dogma,
//! narrative, scalar summaries and transcendental readings, because
//! they exist and a universe that cannot represent them is not
//! all-encompassing. **The error was never that such things exist. It
//! was granting them universal scope.**
//!
//! Every construct that has leaked into the deterministic core this
//! session announced itself the same way — by claiming to be the
//! measure, the driver, the master:
//!
//! - `C(Δ)` claimed to be the measure of convergence, and became a
//!   scoring gate in `validate_step` and a readability gate in the
//!   kernel bridge.
//! - the damped-gradient term claimed to be the driver, was demoted to "a hint", and the
//!   objective leaked around it through C-scoring instead.
//! - "the master equation" claimed to be the one composition, and was
//!   struck outright.
//!
//! **The containment law: a claim of universality is the warning, not
//! the credential.** Anything asserting universal scope is moved here,
//! where it may be computed and shown and may govern nothing.
//!
//! ## The cage is one-way, and that is the whole mechanism
//!
//! Values enter as exact rationals and leave only as `String`. There is
//! deliberately no accessor returning the inner value — no
//! `into_inner`, no `Deref`, no `as_ratio`. A single such method would
//! reopen every path this module closes, because a caller could escape,
//! compare, and gate exactly as before.
//!
//! The exit being `String` is the point: no rule can numerically branch
//! on a rendered string, so the boundary is enforced by types rather
//! than by a docstring asking politely. Docstrings saying *do not gate
//! on this* failed twice in one day — `validate_step`'s productivity
//! gate and the bridge's readability check — and in both cases the
//! prose was right while the code did the opposite.
//!
//! ## The Charter — what this box is for
//!
//! **The cage holds variables that must never impact the totality of
//! the mathematics** (operator ruling 2026-08-01). That is the whole
//! test, and it is a corruption test, not a shape test:
//!
//! - **floats and approximations** — they are not exact, so a single one
//!   admitted anywhere makes the universe non-reproducible;
//! - **sampled, estimated or truncated values** — same failure, dressed
//!   as precision;
//! - **dogma** — unfalsifiable by construction (Article III law 6), so
//!   no experiment can refute it and no rule may rest on it;
//! - **aggregates used as scores** — `C`, the conservation distance:
//!   exact, but they collapse many into one and what remains is
//!   rankable, which is a descent waiting for a caller.
//!
//! Inside the cage these are permitted, precisely because they cannot
//! leak. That is why containment beats prohibition: the system may say
//! them — render a float to a chart, name a belief — while determinism
//! is guaranteed by the boundary rather than by everyone's care.
//!
//! ## What is categorically not A candidate
//!
//! **The manifold's own quantities are never caged, because they are
//! the mathematics this box exists to protect.** Enclosing one would be
//! enclosing the thing the enclosure is for. They are not enumerated
//! here on purpose — see below.
//!
//! This is enforced structurally, not by care: `cage_arrow` pins that
//! this module names no manifold machinery, in code or in prose. The
//! reasoning about what is and is not a candidate lives in the law
//! documents, deliberately — a box that discusses its neighbours
//! acquires footholds, and a foothold is how the wrong thing ends up
//! inside.
//!
//! ## The two empty cells
//!
//! The off-diagonal cells of the containment vector hold nothing. They
//! were formerly reserved for fragments of the damped-gradient term;
//! that term is struck, nothing computes it, and **a slot held open for
//! a struck quantity carries it just as a named symbol would** (operator
//! ruling 2026-08-01, the same ruling that excised the symbol itself).
//! The reservation is withdrawn. The cells are empty because maximum
//! inverse opposition needs only two of the four, which is a property of
//! the layout and owes nothing to anything struck.
//!
//! ## The door — calling in, withdrawing out
//!
//! (Operator ruling 2026-08-01.) The cage is not only a quarantine, it
//! is a workshop. Worlds may call into it to run algorithmic potentials
//! — float pipelines, GPU kernels, heuristics, sampling, anything a
//! deterministic path could never hold — and withdraw answers, without
//! the approximation spreading outward.
//!
//! **The law that makes this safe is one sentence: approximation may
//! propose; only exactness may dispose.**
//!
//! This is U1a's prover/verifier split generalised from solving to
//! compute. There, `solver` searches by any means it likes and
//! `verify_work` re-forges, re-applies and re-chains exactly — the
//! search may be a GPU grinding floats, and the judgment never touches
//! one. The same shape holds here:
//!
//! ```text
//! call in    →  the inside may use ANY means. No purity is asked of
//!               it, because nothing it produces is believed.
//! withdraw   →  what leaves is a PROPOSAL: an exact candidate the
//!               outside re-derives for itself.
//! ```
//!
//! **A proposal is a guess, never evidence.** The outside does not
//! accept it because the cage produced it; it re-checks it under its own
//! exact rules, exactly as a mesh peer re-gates an incoming step rather
//! than trusting its sender. A proposal that fails verification costs
//! nothing, because nothing was ever trusted.
//!
//! **The disease cannot cross, and it is the type that stops it, not
//! care.** [`Proposal`] carries only `T: Exact`, and [`Exact`] is
//! implemented for the exact carriers alone. A float has no `Exact`
//! impl, so `Proposal<f64>` does not exist and an approximation cannot
//! be named at the door — never mind carried through it.
//!
//! Nothing in a computing path may import from this module.

use cfe::ratio::Ratio;

/// A caged reading: showable, comparable for identity, and ordered by
/// nothing.
///
/// `Lens` derives `PartialEq` and deliberately not `PartialOrd`, and
/// that single omission is the cage's teeth:
///
/// - **Ordering is what descent needs.** `a < b` does not compile.
///   Neither does `> epsilon`, `<= bound`, `.min()`, `.max()`, or
///   `.sort()`. Every gate, threshold and penalty curve struck this
///   session is now a type error rather than a review comment.
/// - **Identity is what witnesses need.** `assert_eq!(c, Lens::of(..))`
///   still states exactly what a state measures. Knowing what a reading
///   is cannot be descended; knowing which side of something it falls
///   on can.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lens {
    /// The containment vector, flattened to 2×2 (operator ruling
    /// 2026-08-01). The caged value is never held in a cell.
    ///
    /// A canonical `Ratio` is `p/q` with `gcd(p,q) = 1` and `q > 0`, so
    /// the pair `(p, q)` is unique. The cage stores them at maximum
    /// inverse opposition — numerator and denominator on opposite
    /// diagonal corners — so that:
    ///
    /// - no single cell approximates the value, in magnitude or sign;
    /// - reassembly requires the assembly RULE, which exists only
    ///   inside [`Lens::show`] and is never exported.
    ///
    /// This is the cage's second line. The first is that nothing
    /// escapes; the second is that what is inside is not a usable
    /// scalar even if it did. A leak yields components, never a number.
    ///
    /// ```text
    ///     ┌               ┐
    ///     │  p      ·     │   ·  = empty (see above)
    ///     │  ·      q     │   p  = numerator   ┐ maximally
    ///     └               ┘   q  = denominator ┘ separated
    /// ```
    cell: [[Ratio; 2]; 2],
}

impl Lens {
    /// The door in. Exact rationals enter freely; nothing leaves.
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

    /// The door out, and the only one: a rendering for an operator surface.
    /// It lands in a `String` on purpose — no rule can numerically
    /// branch on it, so the cage holds without anyone having to
    /// remember that it should.
    ///
    /// This is the only place the assembly rule exists.
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

/// The CLOSED algebra — C's own `V`, sealed (operator ruling
/// 2026-08-01: *C cannot leak into D and needs to find his own V*).
///
/// A cage with no interior life invites someone to open it, because the
/// value inside is still wanted for something. So `C` is not merely
/// held — it is given its own universe, closed under its own
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
/// obstructive. C is not starved; it is isolated with everything it needs.
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
    /// space. Note what this cannot feed: no engine path accepts a
    /// `Lens`, so an accumulated C has nowhere to go but a rendering.
    fn sum<I: Iterator<Item = Lens>>(iter: I) -> Lens {
        iter.fold(Lens::of(Ratio::from(num_bigint::BigInt::from(0))), |a, b| a + b)
    }
}

/// Exactly representable — the marker that may cross the door.
///
/// Implemented for the exact carriers and for nothing else. A float, a
/// sampled reading or any approximate type simply has no impl, so it
/// cannot be placed in a [`Proposal`] and therefore cannot be withdrawn.
/// The boundary is enforced by the type system rather than by anyone
/// remembering that it should be.
pub trait Exact {}

impl Exact for Ratio {}
impl Exact for i64 {}
impl Exact for u64 {}
impl Exact for usize {}
impl Exact for bool {}
impl<T: Exact> Exact for Vec<T> {}
impl<T: Exact> Exact for Option<T> {}

/// A candidate produced inside the cage by any means — and believed by
/// nobody.
///
/// The inside may have used floats, a GPU, sampling or a heuristic to
/// find this. None of that crosses: what crosses is an exact value the
/// outside verifies for itself. `withdraw` deliberately hands back a
/// bare candidate with no attestation attached, because attaching one
/// would invite a caller to trust it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal<T: Exact> {
    candidate: T,
}

impl<T: Exact> Proposal<T> {
    /// Offered from inside. No purity is asked of whatever produced it.
    pub fn offer(candidate: T) -> Self {
        Proposal { candidate }
    }

    /// The way out — **and it demands a verifier.**
    ///
    /// The first cut of this returned the candidate bare, with a
    /// docstring asking the caller to check it. That is protection by
    /// prose, and prose failed twice in one day (`validate_step`'s
    /// productivity gate, the bridge's readability check). So the
    /// obligation is now in the signature: **the value cannot be
    /// obtained without supplying the rule that re-derives it**, and a
    /// candidate that fails yields `None`.
    ///
    /// Honest limit: the compiler forces a verifier to exist, not to be
    /// sound — `|_| true` compiles. What it buys is that a bypass must
    /// be written, and a written bypass is visible in review, where an
    /// omitted check is invisible by construction.
    pub fn withdraw(self, verify: impl FnOnce(&T) -> bool) -> Option<T> {
        match verify(&self.candidate) {
            true => Some(self.candidate),
            false => None,
        }
    }
}

/// Consult the sector — `C(·)` as a call, not a score.
///
/// (Operator ruling 2026-08-01.) `C` is the culmination of a whole
/// family of acts — congruence, coherence, cohesion, conversation,
/// communication, comingling, consilience — and every one of them is a
/// reconciliation: *are these the same? does this hold together? can
/// these exchange? do independent lines agree?* Those questions cannot
/// be answered exactly in the general case. They need search,
/// comparison, heuristics, sampling — the whole dirty toolbox.
///
/// **So `C` is not a prisoner here. `C` is the resident**, and this is
/// the door a caller knocks on. C is consulted rather than excluded, because
/// the tools its work requires are exactly the tools that may never
/// touch the outside.
///
/// `tool` may do anything internally — floats, a GPU kernel, a sampled
/// search, an approximation that would be unlawful anywhere else. Only
/// its answer crosses, only if that answer is [`Exact`], and even then
/// it crosses as a [`Proposal`] — a guess the caller re-derives under
/// its own rules. Nothing consultation produces is believed.
///
/// **The loop this closes.** Charter §11 excised the reconciliation
/// instruments — consilience, the manifold, the census, the triangle —
/// because a second readable truth is not a measurement. They were
/// right to go as law. They return here as tools, and they cannot
/// become a second truth in this room, because a proposal carries no
/// authority: reconciliation that must be re-verified to be used is
/// advice, and advice was all it was ever fit to be.
pub fn consult<Q, A: Exact>(question: Q, tool: impl FnOnce(Q) -> A) -> Proposal<A> {
    Proposal::offer(tool(question))
}
