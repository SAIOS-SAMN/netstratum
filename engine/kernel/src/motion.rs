//! THE MOVE ALGEBRA (`19_KERNEL` §3a.2, operator ruling 2026-08-02) —
//! the multi-axial transition, its inversion axiom, and the schedule
//! that makes non-commutative composition safe across nodes.
//!
//! **WHY THIS EXISTS.** `lawful(prior, next, step)` accepts `next` and
//! returns `Ok(())`: the transition is computed elsewhere and the
//! kernel grades it. Returning `Committed` instead would fix the
//! duplication and keep the LINEARITY — a whole world is a POINT, with
//! its axes flattened and no reading of which moved or in what order.
//!
//! A transition returns the **MOVE**, and the state is what you get by
//! composing moves:
//!
//! ```text
//! transition(prior, step) -> Result<Move, Refused>
//! prior ∘ move = next                 (∘ is NOT commutative)
//! ```
//!
//! **THE MOVE ALREADY EXISTS IN THE JUDGE AND IS DISCARDED.**
//! `lawful_deltas` (`lib.rs:611`) returns `Vec<(Resource, Ratio)>` —
//! that is the move — and `conservation` uses it only to compare
//! against the `next` it was handed, then drops it. This module gives
//! that value a type and a law.
//!
//! **THIS MODULE IS THE ALGEBRA, NOT THE WIRING.** `lawful` still has
//! its old signature; collapsing the 113 `replay` arms onto this is
//! #30's remaining half. Landed here so the wiring aims at a certified
//! shape rather than an invented one.

use cfe::dop::Resource;
use cfe::ratio::{ratio, Ratio};

/// ONE MULTI-AXIAL MOVE — what a deed does, carried per axis rather
/// than collapsed into a resulting world.
///
/// Axes are held as sparse position lists, never as a norm: the same
/// discipline `Flow{at, l, value}` follows in `rfa-core`. A summed move
/// would destroy exactly the per-axis reading this type exists to keep.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Move {
    /// The banked axis: `(orb, resource) → delta`, ascending and
    /// zero-free (one move, one encoding).
    pub banked: Vec<((u64, Resource), Ratio)>,
    /// The gauge axis: one delta per standing plane.
    pub gauge: Vec<Ratio>,
    /// The seat axis: `(line, station)` in application order. **Not
    /// additive** — a later seating replaces an earlier one, which is
    /// one of the two sources of non-commutativity here.
    pub seats: Vec<(String, u8)>,
}

/// WHY COMPOSITION DOES NOT COMMUTE — the two sources, both already in
/// the engine's semantics rather than introduced by this module.
///
/// 1. **THE FLOOR.** A bank cannot go negative, so composition clamps
///    at zero. `spend(10)` then `earn(5)` is not `earn(5)` then
///    `spend(10)` when the bank holds `5` — the first bottoms out and
///    loses the remainder, the second does not. Every clamped axis is
///    non-commutative by construction.
/// 2. **REPLACEMENT.** A seat is not a delta; the last seating stands
///    (`world.rs` STATION-1). Ordering two seatings of one line gives
///    two different offices.
///
/// Additive unclamped axes (the gauge) DO commute. Non-commutativity
/// is a property of particular axes, not a decoration on all of them —
/// which is exactly why the move is carried per axis.
impl Move {
    /// THE IDENTITY — the move that changes nothing. `e ∘ a = a ∘ e = a`.
    pub fn identity() -> Self {
        Self::default()
    }

    /// Is this the identity?
    pub fn is_identity(&self) -> bool {
        self.banked.iter().all(|(_, d)| *d == ratio(0, 1))
            && self.gauge.iter().all(|d| *d == ratio(0, 1))
            && self.seats.is_empty()
    }

    /// A single banked move, for construction and witness.
    pub fn banked_of(orb: u64, resource: Resource, delta: Ratio) -> Self {
        Self { banked: vec![((orb, resource), delta)], ..Self::default() }
    }

    /// THE INVERSION AXIOM — every move has a formal inverse: negate
    /// each additive axis, and drop the seats (a replacement has no
    /// additive inverse; restoring a prior office is a NEW seating, not
    /// an undoing).
    ///
    /// **INVERSES ARE ALGEBRA, NOT PERMISSION.** `a ∘ a⁻¹ = e` holds on
    /// the additive axes — and applying `a⁻¹` as a DEED is a separate
    /// question the law answers separately. ARTICLE IV (rank never
    /// retreats) forbids the inverse of a rank-raising move outright,
    /// and nothing un-mints. **The lawful moves form a MONOID, not a
    /// group**; the group exists only so the commutator can be written.
    ///
    /// That is the whole reason inversion is required here: the
    /// commutator needs it, and `18_TORSION` §2k.1 established that the
    /// commutator IS the torsion. Without inverses the torsion is not
    /// expressible at all.
    pub fn inverse(&self) -> Self {
        Self {
            banked: self.banked.iter().map(|(k, d)| (*k, -d.clone())).collect(),
            gauge: self.gauge.iter().map(|d| -d.clone()).collect(),
            seats: Vec::new(),
        }
    }

    /// UNCLAMPED COMPOSITION — the free group operation. Associative
    /// and, on the additive axes alone, commutative. This is the
    /// algebra the commutator is taken in.
    pub fn then_free(&self, next: &Self) -> Self {
        let mut banked = self.banked.clone();
        for (key, delta) in &next.banked {
            match banked.iter_mut().find(|(k, _)| k == key) {
                Some((_, held)) => *held += delta.clone(),
                None => banked.push((*key, delta.clone())),
            }
        }
        banked.sort_by(|a, b| a.0.cmp(&b.0));
        banked.retain(|(_, d)| *d != ratio(0, 1));

        let width = self.gauge.len().max(next.gauge.len());
        let gauge = (0..width)
            .map(|plane| {
                let left = self.gauge.get(plane).cloned().unwrap_or_else(|| ratio(0, 1));
                let right = next.gauge.get(plane).cloned().unwrap_or_else(|| ratio(0, 1));
                left + right
            })
            .collect();

        let mut seats = self.seats.clone();
        seats.extend(next.seats.iter().cloned());
        Self { banked, gauge, seats }
    }

    /// THE COMMUTATOR `[a, b] = a ∘ b ∘ a⁻¹ ∘ b⁻¹`, taken in the free
    /// group. The identity iff the two moves commute; anything else is
    /// the torsion the pair carries.
    ///
    /// Written with `then_free` deliberately: a clamped composition is
    /// not a group and the commutator would not be defined in it.
    pub fn commutator(&self, other: &Self) -> Self {
        self.then_free(other).then_free(&self.inverse()).then_free(&other.inverse())
    }

    /// Do these two moves commute?
    pub fn commutes_with(&self, other: &Self) -> bool {
        self.commutator(other).is_identity()
    }
}

/// A STANDING POSITION — what a move is applied TO. Sparse and
/// non-negative: a bank that cannot go below zero is what makes
/// application non-commutative.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Standing {
    pub banked: Vec<((u64, Resource), Ratio)>,
    pub seats: Vec<(String, u8)>,
}

impl Standing {
    /// APPLY — `standing ∘ move`. **Clamped at zero on every banked
    /// axis**, which is the floor that breaks commutativity, and
    /// last-write-wins on seats, which is the replacement that breaks
    /// it again.
    pub fn then(&self, step: &Move) -> Self {
        let mut banked = self.banked.clone();
        for (key, delta) in &step.banked {
            match banked.iter_mut().find(|(k, _)| k == key) {
                Some((_, held)) => *held = (held.clone() + delta.clone()).max(ratio(0, 1)),
                None => banked.push((*key, delta.clone().max(ratio(0, 1)))),
            }
        }
        banked.sort_by(|a, b| a.0.cmp(&b.0));
        banked.retain(|(_, held)| *held != ratio(0, 1));

        let mut seats = self.seats.clone();
        for (line, station) in &step.seats {
            match seats.iter_mut().find(|(held, _)| held == line) {
                Some((_, office)) => *office = *station,
                None => seats.push((line.clone(), *station)),
            }
        }
        seats.sort();
        Self { banked, seats }
    }

    /// The banked reading on one axis.
    pub fn held(&self, orb: u64, resource: Resource) -> Ratio {
        self.banked
            .iter()
            .find(|((o, r), _)| *o == orb && *r == resource)
            .map(|(_, held)| held.clone())
            .unwrap_or_else(|| ratio(0, 1))
    }
}

/// THE SCHEDULE — the canonical order moves compose in.
///
/// **THIS IS WHAT MAKES NON-COMMUTATIVITY SAFE.** If `a ∘ b ≠ b ∘ a`
/// and two nodes hold the same set of moves, they reach DIFFERENT
/// states unless the order is fixed. Non-commutativity without a
/// canonical schedule is a consensus break, not a feature.
///
/// The schedule is already landed and needs nothing new: a receipt
/// carries `sequence` (`vrp.rs:100`), which is monotone, unique within
/// a chain, and inside the frame hash. **Every node reconstructs the
/// same order from the chain itself.** `stamp` alone is insufficient —
/// several deeds share a tick — so `sequence` is the tie-break and the
/// primary key both.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Slot {
    /// The tick the deed closed on.
    pub stamp: u64,
    /// The receipt's position in its chain — total, unique, hashed.
    pub sequence: u64,
}

/// Compose a set of moves under the schedule. The input order is
/// IRRELEVANT: the schedule sorts, so two nodes handed the same moves
/// in any arrangement reach the same standing.
pub fn compose_scheduled(standing: &Standing, mut moves: Vec<(Slot, Move)>) -> Standing {
    moves.sort_by(|a, b| a.0.cmp(&b.0));
    moves.iter().fold(standing.clone(), |held, (_, step)| held.then(step))
}

/// THE COST ENVELOPE — `O(n^D)`, replacing the flat `O(1)` bound
/// (operator ruling 2026-08-02, superseding `18_TORSION` §2n.3's
/// per-axis statement for the WORK envelope).
///
/// A flat bound was a scalar collapse: it fixed the budget independent
/// of how much structure stands, which is the same flattening as
/// `C(Δ) = Σ|R|`. The honest envelope is the VOLUME of the space the
/// work ranges over — side `n` along `D` independent axes.
///
/// **It breathes with `D`.** Rank is earned, never granted, and
/// ARTICLE IV holds it monotone — so the envelope widens as the system
/// gains dimension and never narrows. That is the re-volumizer: the
/// budget is a function of standing rank rather than a constant chosen
/// in advance.
///
/// Returns `None` on overflow rather than wrapping — an envelope that
/// silently wrapped would report a huge space as a tiny one.
pub fn envelope(n: u64, rank: u32) -> Option<u128> {
    (n as u128).checked_pow(rank)
}
