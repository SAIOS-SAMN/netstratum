//! The REACH — what a civilization is asking for when it thinks.
//!
//! Operator ruling 2026-08-04: *never enumerate; the operators should be
//! called by actions based on the civilization's requirements, demands
//! and needs.*
//!
//! The engine already carries those needs as receipted deeds. WANT-1
//! (`cfe`, tags 62/63) strikes a `WantStruck { orb_id, kind, reading }`
//! when a mind starves and relieves it when the mind is fed:
//!
//! ```text
//!     kind 1  epistemological   (board-side reading, reserved)
//!     kind 2  resonance         a harmonic mind starved of LINKS
//!     kind 3  novelty           an empiricist mind starved of SIGNAL
//! ```
//!
//! The solver has never read them. It swept the same vocabulary for
//! every mind in every era regardless of what that mind lacked, which
//! is what made it brute force rather than cognition.
//!
//! **The law that keeps this honest.** The need may order and gate the
//! reach; only the observation may supply a candidate. A starving mind
//! reaches differently — it does not get to invent an operator the
//! observation does not support. Break that and "need-driven" is just
//! guessing under a better name, and the sector door would withdraw it
//! anyway.
//!
//! `solver` sits below `cfe` in the arrow law and cannot import its
//! types, so the want kinds arrive here as the plain integers the wire
//! already carries. The conductor maps `world.wants()` into a [`Need`].

/// What the mind is starved of, as the chronicle recorded it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hunger {
    /// Kind 2 — starved of correspondence. A harmonic mind with no link
    /// inside its window reaches for what relates two things.
    Resonance,
    /// Kind 3 — starved of signal. An empiricist mind with no node
    /// inside its window reaches for what makes structure.
    Novelty,
}

impl Hunger {}

/// The need a solve is answering.
///
/// `Need::sated()` is the honest neutral: nothing is struck, so nothing
/// is preferred and the reach is taken in its derived order. It is not
/// a "default difficulty" and carries no bias.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Need {
    /// The struck wants, strongest first. Empty = sated.
    pub hungers: Vec<Hunger>,
    /// How long the strongest want has starved (the `reading` WANT-1
    /// carries). A longer starve widens the reach — it never invents.
    pub starved_for: u64,
    /// **Orbit-directed hunger** (38 §6, task #15): the line's own
    /// families ordered thinnest-first by its minted library — the
    /// measured frontier. Empty = no census (a young or libraryless
    /// line), and the static preference stands. Ordering only, as
    /// ever: the observation still supplies every candidate.
    pub thin: Vec<Family>,
    /// **The DEPTH expansion buys** (intelligence ARC A2) — extra compose
    /// depth a line has earned through its relational structure (rank).
    /// The mind reaches this much deeper than hunger alone would take it:
    /// a civilization whose internal economy sustains obligation cycles
    /// (multi-orb, rank > 0) can hold more of the composition in view at
    /// once. Zero for a single-orb line — advancement, not a clock, buys
    /// this. Additive, never a cap: the baseline the rung climbs on is
    /// untouched, so a young line is never starved of the depth it needs
    /// to climb to the rung where it can expand (the B-E1 deadlock).
    pub reach_bonus: usize,
    /// **The infrastructure ceiling** (COG-6) — the deepest thought the
    /// civilization's built cognitive apparatus can contain. Leisure buys the
    /// time to think (COG-1/3); this bounds the size: an 11-dimensional bridge
    /// cannot be contained in a mud hut. Set by the conductor from the seated
    /// research sciences (an Academy in all but name). `0` means uncapped — the
    /// standing no-overfit frontier stands, so a line that never sets it (every
    /// test, the sweep path) is unchanged.
    pub capacity: usize,
}

impl Need {
    /// Nothing struck: work the observation in its own order.
    pub fn sated() -> Need {
        Need::default()
    }

    /// One struck want with its reading.
    pub fn struck(hunger: Hunger, starved_for: u64) -> Need {
        Need {
            hungers: vec![hunger],
            starved_for,
            thin: Vec::new(),
            reach_bonus: 0,
            capacity: 0,
        }
    }

    /// Does this need reach for `family` before the others?
    ///
    /// Ordering only. Every family stays reachable — a resonance-struck
    /// mind that meets a tiling task still solves it, it simply looks
    /// for correspondence first.
    pub fn prefers(&self, family: Family) -> bool {
        self.hungers.iter().any(|hunger| match hunger {
            // Correspondence: what maps a thing onto its counterpart.
            Hunger::Resonance => matches!(family, Family::Symmetry | Family::Recolor),
            // Structure: what makes something that was not there.
            // Orbit-directed (38 §6): with a census standing, the
            // starving mind reaches first for its thinnest family —
            // where its own language has the fewest words — instead
            // of a fixed pair. No census, the fixed pair stands.
            Hunger::Novelty => match self.thin.first() {
                Some(thinnest) => family == *thinnest,
                None => matches!(family, Family::Tile | Family::Object),
            },
        })
    }
}

#[cfg(test)]
mod tests {}

/// The family an operator belongs to — the census's classifier, one
/// home shared with the derivation module's own labels.
pub fn family_of(op: &solver_verify::grid::Op) -> Family {
    use solver_verify::grid::Op;
    match op {
        Op::Rotate90 | Op::Rotate180 | Op::Rotate270 | Op::FlipH | Op::FlipV | Op::Transpose => {
            Family::Symmetry
        }
        Op::Recolor(_) => Family::Recolor,
        Op::Tile { .. } | Op::Dilate { .. } => Family::Tile,
        Op::Shift { .. } => Family::Symmetry,
        Op::Project(_) => Family::Object,
        Op::Object(_) => Family::Object,
    }
}

/// The operator families, named so a need can speak about them without
/// naming individual operators (which only the observation may supply).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Family {
    Symmetry,
    Recolor,
    Tile,
    Object,
}
