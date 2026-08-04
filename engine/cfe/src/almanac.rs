//! THE ALMANAC — the PERIODIC law of elements, the locality a star and
//! a planet give it, and the refinement pressure turns into knowledge.
//!
//! `14_ALMANAC.md` rules the element grammar; `dop::element_properties`
//! landed its first half — a family drawn from the star's spine bands,
//! a tier, a richness. What it has no notion of is PERIODICITY: two
//! elements of the same family are unrelated draws, so nothing recurs
//! and nothing can be predicted from position.
//!
//! **THE LINE AND THE LANE** (operator ruling 2026-08-04: *open up
//! lane/line on elements — periodicity*). The two axes of a periodic
//! table, and they mean different things:
//!
//!   LINE (the period, the row)  — DEPTH. Which stratum an element
//!                                 sits at, and therefore the PRESSURE
//!                                 at which it refines. Rises with the
//!                                 line.
//!   LANE (the group, the column) — CHARACTER. Properties RECUR along a
//!                                 lane: every element in a lane
//!                                 refines the same way and compounds
//!                                 by the same rule, whatever its line.
//!
//! That recurrence is the whole point of a periodic table — it lets a
//! civilization that has refined one element in a lane PREDICT the next,
//! and prediction is what makes discovery cumulative rather than a
//! lottery.
//!
//! **EVERYTHING HERE IS A DERIVATION.** Pure, total, exact-rational, no
//! I/O and no deed. It reads an element id, a star and an orb, and
//! returns what they imply. It changes no committed state and no
//! existing behaviour: `element_properties` is untouched and still
//! rules what a prospect yields. This is the reading that has to exist
//! before a refinement deed can be written against it.

use crate::ratio::{ratio, Ratio};
use crate::thermal::StarClass;

/// How many lanes the table carries. A lane is a CHARACTER class and
/// the count is the vocabulary's width, not a tuning knob.
pub const LANES: u8 = 8;
/// How many lines stand. The line is depth; the deepest is the most
/// pressured and the rarest.
pub const LINES: u8 = 7;

/// WHERE AN ELEMENT SITS, and what that implies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Periodic {
    /// The period — depth. `1..=LINES`.
    pub line: u8,
    /// The group — character. `1..=LANES`. Properties recur here.
    pub lane: u8,
    /// The pressure this element needs before it will refine, rising
    /// with the line. Exact, never a float.
    pub refines_at: Ratio,
    /// How many of its own lane an element BINDS when it compounds —
    /// the lane's recurring valence, identical for every line.
    pub binds: u8,
}

/// Read an element's position. Pure: the same id always reads the same
/// cell, so the table is stable across every world that meets it.
pub fn periodic_of(element: &[u8; 32]) -> Periodic {
    // Bytes 3 and 4 are unused by `dop::element_properties` (which
    // reads 1 and 2), so the table is independent of the landed
    // family/tier draw rather than a re-slicing of it.
    let line = 1 + (element[3] % LINES);
    let lane = 1 + (element[4] % LANES);
    Periodic {
        line,
        lane,
        // Pressure rises with depth: line 1 refines at 1/7, line 7 at 1.
        refines_at: ratio(i64::from(line), i64::from(LINES)),
        // The lane's valence recurs: 1..=4, and lanes 5..=8 mirror
        // 4..=1 — the fold that makes a table periodic rather than a
        // list.
        binds: match lane {
            l if l <= 4 => l,
            l => LANES + 1 - l,
        },
    }
}

/// **LOCALITY** — which lines a WORLD can reach at all.
///
/// A star's class fixes how deep its planets' strata run: a heavy,
/// bright class forges and disperses down to the deep lines; a dim one
/// never reaches them. So the same element id is REACHABLE in one
/// system and absent from another, which is what makes a resource a
/// place rather than a number.
pub fn lines_of(class: StarClass) -> u8 {
    match class {
        // The heavy forges reach every line.
        StarClass::O | StarClass::B => LINES,
        StarClass::A | StarClass::F => 6,
        StarClass::G => 5,
        StarClass::K => 4,
        StarClass::M => 3,
        // The remnants: a collapsed star is the deepest forge there is,
        // and a dwarf the shallowest.
        StarClass::NeutronStar | StarClass::WolfRayet => LINES,
        StarClass::RedGiant | StarClass::WhiteDwarf => 5,
        StarClass::BrownDwarf => 2,
        StarClass::LuminalWhiteHole => LINES,
    }
}

/// Is this element PRESENT in a system of this class? Locality as a
/// predicate: deep lines simply do not occur under a shallow star.
pub fn local_to(element: &[u8; 32], class: StarClass) -> bool {
    periodic_of(element).line <= lines_of(class)
}

/// **THE PRESSURE OF A SYSTEM** — what an orb can bring to bear.
///
/// Pressure is the orb's own depth of working: the stratum it has
/// opened, against the deepest line its star offers. An orb that has
/// opened nothing brings nothing; one working the deepest stratum of a
/// heavy star brings full pressure.
pub fn pressure(stratum: u8, class: StarClass) -> Ratio {
    let reach = lines_of(class);
    match reach {
        0 => ratio(0, 1),
        _ => ratio(i64::from(stratum.min(reach)), i64::from(reach)),
    }
}

/// **REFINEMENT** — does this pressure release this element?
///
/// The element refines when the system brings at least the pressure its
/// line demands. Exact comparison; no tolerance, no float.
pub fn refines(element: &[u8; 32], stratum: u8, class: StarClass) -> bool {
    local_to(element, class) && pressure(stratum, class) >= periodic_of(element).refines_at
}

/// **COMPOSITION** — two refined elements COMPOUND into a gem.
///
/// The rule is the lane's, not the pair's: two elements compound only
/// when their binds SATURATE — `binds(a) + binds(b) == LANES/2 + 1` —
/// which is why the table is periodic. Any two elements from the same
/// pair of lanes compound alike, whatever their lines, so a
/// civilization that has made one gem KNOWS the recipe for a family of
/// them.
pub fn compounds(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let (pa, pb) = (periodic_of(a), periodic_of(b));
    u16::from(pa.binds) + u16::from(pb.binds) == u16::from(LANES / 2 + 1)
}

/// The gem two elements make — content-addressed, so the SAME pair
/// always names the same gem and a "new" gem is new by its identity
/// rather than by anyone's say-so.
///
/// Order-free: `gem(a, b) == gem(b, a)`, because a compound is not a
/// direction.
pub fn gem_of(a: &[u8; 32], b: &[u8; 32]) -> Option<[u8; 32]> {
    if !compounds(a, b) {
        return None;
    }
    let (low, high) = match a <= b {
        true => (a, b),
        false => (b, a),
    };
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"netstratum.almanac.gem.v1");
    hasher.update(low);
    hasher.update(high);
    Some(*hasher.finalize().as_bytes())
}

/// **KNOWLEDGE** — the depth a gem is worth.
///
/// A compound made of deep elements is harder-won than a shallow one,
/// so the knowledge it carries is the SUM of its lines. This is the
/// seam a discovery deed would price against; it mints nothing on its
/// own.
pub fn knowledge_of(a: &[u8; 32], b: &[u8; 32]) -> Option<u16> {
    compounds(a, b).then(|| {
        u16::from(periodic_of(a).line) + u16::from(periodic_of(b).line)
    })
}
