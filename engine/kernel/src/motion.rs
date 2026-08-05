//! THE MOVE ALGEBRA (`19_KERNEL` §3a.2, operator ruling 2026-08-02) —
//! the multi-axial transition, its inversion axiom, and the schedule
//! that makes non-commutative composition safe across nodes.
//!
//! **WHY THIS EXISTS.** `admits(prior, next, step)` accepts `next` and
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
//! `admitted_deltas` (`lib.rs:611`) returns `Vec<(Resource, Ratio)>` —
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
    /// THE ACCUMULATOR AXIS — additive, keyed. Covers every per-orb
    /// running total `replay` maintains (`entropy_vented`, `bond_flow`,
    /// the crisis and morality books, `severance_load`, …). Additive,
    /// therefore COMMUTING.
    pub books: Vec<(BookKey, Ratio)>,
    /// THE RECORD AXIS — keyed collections that are INSERTED INTO and
    /// REMOVED FROM rather than added to (bonds, forwards, councils,
    /// offers, debts, prospects, instruments, routes). Ordered, and
    /// NON-COMMUTING: `remove ∘ insert` is not `insert ∘ remove`.
    pub records: Vec<RecordOp>,
}

/// An accumulator's address — `(orb, book)`. The book index is opaque
/// to the algebra on purpose: which concrete field a number belongs to
/// is the WIRING's business, and the composition law is the algebra's.
pub type BookKey = (u64, u16);

/// A keyed collection's address — `(kind, id)`.
pub type RecordKey = (u16, u64);

/// ONE RECORD OPERATION. Not a delta — a record either stands or does
/// not, so the axis composes by REPLACEMENT and the order is the whole
/// content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordOp {
    /// The record stands, carrying this opaque body. Re-inserting an
    /// existing key REPLACES it (an amended bond is not a second bond).
    Insert { key: RecordKey, body: Vec<u8> },
    /// The record no longer stands.
    Remove { key: RecordKey },
}

impl RecordOp {
    /// The address this operation acts on.
    pub fn key(&self) -> RecordKey {
        match self {
            RecordOp::Insert { key, .. } | RecordOp::Remove { key } => *key,
        }
    }
}

/// HOW AN AXIS COMPOSES — the taxonomy the measurement forced.
///
/// Collapsing `replay`'s arms onto this algebra needs the algebra to
/// SPAN what the arms do. Measured against `world.rs::replay`'s 63
/// arms, every axis they touch composes in exactly one of two ways,
/// and which one decides whether that axis commutes:
///
/// ```text
/// ADDITIVE      banked · gauge · the accumulator books
///               compose by +, and therefore COMMUTE
/// REPLACEMENT   seats · orb fields · world flags · records
///               last write stands, and therefore DO NOT COMMUTE
/// ```
///
/// **Non-commutativity is not a property of the system — it is a
/// property of particular axes**, which is exactly why a move must be
/// carried per axis and never summed. An algebra that knew only one
/// law would either forbid reordering everywhere or permit it
/// everywhere, and both are wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Law {
    /// Composes by addition. Commutes.
    Additive,
    /// Last write stands. Does not commute.
    Replacement,
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
            && self.books.iter().all(|(_, d)| *d == ratio(0, 1))
            && self.records.is_empty()
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
    /// commutator needs it, and `23_TRAVERSAL` §2k.1 established that the
    /// commutator IS the torsion. Without inverses the torsion is not
    /// expressible at all.
    pub fn inverse(&self) -> Self {
        Self {
            banked: self.banked.iter().map(|(k, d)| (*k, -d.clone())).collect(),
            gauge: self.gauge.iter().map(|d| -d.clone()).collect(),
            // Replacement axes have NO additive inverse: restoring a
            // prior office or re-inserting a removed record is a NEW
            // move, never an undoing. Dropped so `a ∘ a⁻¹ = e` holds
            // on exactly the axes where it can.
            seats: Vec::new(),
            books: self.books.iter().map(|(k, d)| (*k, -d.clone())).collect(),
            records: Vec::new(),
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
        banked.sort_by_key(|entry| entry.0);
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

        let mut books = self.books.clone();
        for (key, delta) in &next.books {
            match books.iter_mut().find(|(k, _)| k == key) {
                Some((_, held)) => *held += delta.clone(),
                None => books.push((*key, delta.clone())),
            }
        }
        books.sort_by_key(|entry| entry.0);
        books.retain(|(_, d)| *d != ratio(0, 1));

        let mut records = self.records.clone();
        records.extend(next.records.iter().cloned());

        Self { banked, gauge, seats, books, records }
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
    /// Accumulators. Unlike `banked` these are NOT floored — a book is
    /// a running total of what happened, and clamping it would erase
    /// history rather than bound a holding.
    pub books: Vec<(BookKey, Ratio)>,
    /// Standing records, ascending by key.
    pub records: Vec<(RecordKey, Vec<u8>)>,
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
        banked.sort_by_key(|entry| entry.0);
        banked.retain(|(_, held)| *held != ratio(0, 1));

        let mut seats = self.seats.clone();
        for (line, station) in &step.seats {
            match seats.iter_mut().find(|(held, _)| held == line) {
                Some((_, office)) => *office = *station,
                None => seats.push((line.clone(), *station)),
            }
        }
        seats.sort();

        let mut books = self.books.clone();
        for (key, delta) in &step.books {
            match books.iter_mut().find(|(k, _)| k == key) {
                Some((_, held)) => *held += delta.clone(),
                None => books.push((*key, delta.clone())),
            }
        }
        books.sort_by_key(|entry| entry.0);

        // RECORDS APPLY IN ORDER, and the order is the content: an
        // insert after a remove leaves the record standing, the reverse
        // leaves it gone.
        let mut records = self.records.clone();
        for op in &step.records {
            match op {
                RecordOp::Insert { key, body } => {
                    match records.iter_mut().find(|(k, _)| k == key) {
                        Some((_, held)) => *held = body.clone(),
                        None => records.push((*key, body.clone())),
                    }
                }
                RecordOp::Remove { key } => records.retain(|(k, _)| k != key),
            }
        }
        records.sort_by_key(|entry| entry.0);

        Self { banked, seats, books, records }
    }

    /// Does a record stand at this address?
    pub fn record(&self, key: RecordKey) -> Option<&[u8]> {
        self.records.iter().find(|(k, _)| *k == key).map(|(_, body)| body.as_slice())
    }

    /// An accumulator's reading.
    pub fn book(&self, key: BookKey) -> Ratio {
        self.books
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, held)| held.clone())
            .unwrap_or_else(|| ratio(0, 1))
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
/// (operator ruling 2026-08-02, superseding `22_CONTAINMENT` §2n.3's
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

// ── THE ONLY THING THAT MAKES THIS ALGEBRA REAL ──────────────────────

use cfe::dop::ClosureEvent;
use cfe::world::Committed;

/// PROJECT a committed world onto a standing — the banked and seat axes
/// read straight out of real state.
///
/// **Without this the algebra is fiction.** A `Move` that cannot be
/// checked against what the engine actually does is an island, and
/// `rfa-core` was already that island once (`bridge.rs` §head). Every
/// law in this module is worth exactly as much as this projection and
/// [`move_of`] together can reproduce.
pub fn project(state: &Committed) -> Standing {
    let mut banked = Vec::new();
    for orb in &state.orbs {
        for (resource, held) in &orb.banked {
            banked.push(((orb.orb_id, *resource), held.clone()));
        }
    }
    banked.sort_by_key(|entry| entry.0);
    banked.retain(|(_, held)| *held != ratio(0, 1));

    let mut seats: Vec<(String, u8)> = state.stations.clone();
    seats.sort();

    Standing { banked, seats, books: Vec::new(), records: Vec::new() }
}

/// THE MOVE ONE DEED MAKES — `event → Move`.
///
/// **DELIBERATELY PARTIAL.** `None` means *this algebra cannot yet
/// express that deed*, and that is a measurement rather than a hole to
/// paper over: the honest coverage number is how many events return
/// `Some` AND reproduce `replay` on a real world, not how many source
/// lines mention an axis.
///
/// Every arm added here must arrive with a witness that applies it to
/// real committed state and compares against what the engine did
/// (`motion_certification.rs`). An arm without one is a claim, not a
/// rule.
pub fn move_of(event: &ClosureEvent) -> Option<Move> {
    match event {
        // A transfer is TWO banked legs. When source and target are the
        // same orb they share a key, and a Move carrying one key twice
        // is not canonical — `Standing::then` would apply the legs
        // SEQUENTIALLY and could floor between them, turning a no-op
        // into a loss. Built through `then_free`, which merges.
        ClosureEvent::ResourceTransferred { from_orb, to_orb, resource, amount } => {
            let out = Move::banked_of(*from_orb, *resource, -amount.clone());
            let into = Move::banked_of(*to_orb, *resource, amount.clone());
            Some(out.then_free(&into))
        }
        ClosureEvent::StationSeated { line, station } => {
            Some(Move { seats: vec![(line.clone(), *station)], ..Move::default() })
        }
        ClosureEvent::DemurrageSettled { orb_id, resource, amount } => {
            Some(Move::banked_of(*orb_id, *resource, -amount.clone()))
        }
        // YIELD-1: a collection banks what the zones produced. Added
        // only after a live run showed 0/2 coverage — the deeds a world
        // actually writes are not the deeds that read well on paper.
        ClosureEvent::YieldCollected { orb_id, resource, amount } => {
            Some(Move::banked_of(*orb_id, *resource, amount.clone()))
        }
        // NOT YET EXPRESSIBLE. Not a default arm that guesses — an
        // explicit refusal, so coverage stays measurable.
        _ => None,
    }
}
