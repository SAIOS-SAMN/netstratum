//! THE REACH — what a civilization is asking for when it thinks.
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
//! **THE LAW THAT KEEPS THIS HONEST.** The need may ORDER and GATE the
//! reach; only the OBSERVATION may supply a candidate. A starving mind
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
    /// inside its window reaches for what RELATES two things.
    Resonance,
    /// Kind 3 — starved of signal. An empiricist mind with no node
    /// inside its window reaches for what MAKES structure.
    Novelty,
}

impl Hunger {
    /// From the wire's want kind. Kind 1 is reserved board-side and
    /// names no reach yet; unknown kinds are not guessed at.
    pub fn from_kind(kind: u8) -> Option<Hunger> {
        match kind {
            2 => Some(Hunger::Resonance),
            3 => Some(Hunger::Novelty),
            _ => None,
        }
    }
}

/// THE NEED a solve is answering.
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
    /// **ORBIT-DIRECTED HUNGER** (38 §6, task #15): the line's own
    /// families ordered THINNEST-FIRST by its minted library — the
    /// measured frontier. Empty = no census (a young or libraryless
    /// line), and the static preference stands. Ordering only, as
    /// ever: the observation still supplies every candidate.
    pub thin: Vec<Family>,
}

impl Need {
    /// Nothing struck: work the observation in its own order.
    pub fn sated() -> Need {
        Need::default()
    }

    /// One struck want with its reading.
    pub fn struck(hunger: Hunger, starved_for: u64) -> Need {
        Need { hungers: vec![hunger], starved_for, thin: Vec::new() }
    }

    /// Map the chronicle's `(kind, reading)` wants into a reach.
    /// Unknown kinds are dropped, never guessed.
    pub fn from_wants(wants: &[(u8, u64)]) -> Need {
        let mut hungers = Vec::new();
        let mut starved_for = 0u64;
        for &(kind, reading) in wants {
            if let Some(hunger) = Hunger::from_kind(kind) {
                if !hungers.contains(&hunger) {
                    hungers.push(hunger);
                }
                starved_for = starved_for.max(reading);
            }
        }
        Need { hungers, starved_for, thin: Vec::new() }
    }

    /// THE DEPTH THE NEED WILL COMPOSE TO.
    ///
    /// A sated mind takes single derived operators; a starving one will
    /// compose two, and a long-starved one three. This is the ONLY place
    /// hunger buys breadth, and it buys it over DERIVED candidates —
    /// never over the raw vocabulary, which is the enumeration that was
    /// struck.
    pub fn compose_limit(&self) -> usize {
        match (self.hungers.is_empty(), self.starved_for) {
            (true, _) => 1,
            (false, r) if r >= 480 => 3,
            (false, _) => 2,
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
            // ORBIT-DIRECTED (38 §6): with a census standing, the
            // starving mind reaches first for its THINNEST family —
            // where its own language has the fewest words — instead
            // of a fixed pair. No census, the fixed pair stands.
            Hunger::Novelty => match self.thin.first() {
                Some(thinnest) => family == *thinnest,
                None => matches!(family, Family::Tile | Family::Object),
            },
        })
    }
}

/// The family an operator belongs to — the census's classifier, one
/// home shared with the derivation module's own labels.
pub fn family_of(op: &solver_verify::grid::Op) -> Family {
    use solver_verify::grid::Op;
    match op {
        Op::Rotate90 | Op::Rotate180 | Op::Rotate270 | Op::FlipH | Op::FlipV | Op::Transpose => {
            Family::Symmetry
        }
        Op::Recolor(_) => Family::Recolor,
        Op::Tile { .. } => Family::Tile,
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
