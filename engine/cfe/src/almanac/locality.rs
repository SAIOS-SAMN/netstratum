//! Locality and pressure — what a star and a planet make reachable.
//!
//! Two independent readings, and an element needs both:
//!
//!   the star  says how deep a system's strata run  -> [`local_to`]
//!   the orbit says how hot the orb is              -> the thermal
//!                                                     region, in
//!                                                     [`super::thermal`]
//!
//! Neither implies the other. A shallow element under a heavy star can
//! still be unworkable in the wrong band, and that is what makes a
//! resource a place rather than a number.

use super::periodic::{periodic_of, LINES};
use crate::ratio::{ratio, Ratio};
use crate::thermal::StarClass;

/// Which lines a WORLD can reach at all.
///
/// A heavy, bright class forges and disperses down to the deep lines; a
/// dim one never reaches them. So the same element id is reachable in
/// one system and absent from another.
pub fn lines_of(class: StarClass) -> u8 {
    match class {
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

/// Is this element present in a system of this class? Locality as a
/// predicate: deep lines simply do not occur under a shallow star.
pub fn local_to(element: &[u8; 32], class: StarClass) -> bool {
    periodic_of(element).line <= lines_of(class)
}

/// **CIV-REACH-1** — deepest periodic LINE a people may open.
///
/// The sky (`lines_of`) says what the system *carries*. Civilization
/// reach says what the people may *work*.
///
/// - **No star** → 0 (prospect refuses `NoStarCommissioned` first).
/// - **Star known, no grown tech** (solar leap alone is the sun, not
///   an age) → **2** (bronze surface lines 1..=2).
/// - Each crystallized base condition **1..=5** raises the ceiling by
///   one. Never exceeds the star's forge depth or `LINES`.
///
/// ```text
/// grown conditions (1..=5)   civ_reach (before star cap)
/// 0  (bronze under a sun)    2
/// 1                          3
/// …
/// 5                          7  (= LINES)
/// ```
///
/// Condition 0 (solar leap, often granted at commission) does **not**
/// deepen the mine — knowing the sun is prerequisite, not industry.
pub fn civ_reach(held_conditions: impl IntoIterator<Item = u8>, star: Option<StarClass>) -> u8 {
    // Growth ladder: insulation…stores (1..=5). Solar (0) is not depth.
    let mut growth = [false; 5];
    for c in held_conditions {
        if (1..=5).contains(&c) {
            growth[(c - 1) as usize] = true;
        }
    }
    let n = growth.iter().filter(|&&h| h).count() as u8;
    // Bronze surface: lines 1..=2. One grown condition → 3, …, five → 7.
    let grown = (2u8.saturating_add(n)).min(LINES);
    match star {
        Some(class) => grown.min(lines_of(class)),
        None => 0,
    }
}

/// May this people open this LINE under this sky?
pub fn civ_may_work(line: u8, held_conditions: impl IntoIterator<Item = u8>, star: Option<StarClass>) -> bool {
    line > 0 && line <= civ_reach(held_conditions, star)
}

/// **The deposit law** — which LINES a stratum of this system can hold.
///
/// The forge that deposited a vein is the same pressure that releases
/// it, so a stratum carries exactly the lines its own pressure refines:
/// the deepest `l` with `pressure(stratum) >= l/LINES`, at least 1,
/// never past the star's forge depth. This is what makes "bronze digs
/// shallow" yield shallow MATTER rather than unworkable curios — and it
/// is locality made total: a line the system does not carry is never
/// charted in it.
pub fn strata_lines(stratum: u8, class: StarClass) -> u8 {
    let forge = lines_of(class);
    if forge == 0 {
        return 1;
    }
    let releasable = (u16::from(stratum.min(forge)) * u16::from(LINES) / u16::from(forge)) as u8;
    releasable.clamp(1, forge)
}

/// **The pressure of A system** — the stratum an orb has opened,
/// against the deepest line its star offers.
pub fn pressure(stratum: u8, class: StarClass) -> Ratio {
    let reach = lines_of(class);
    match reach {
        0 => ratio(0, 1),
        _ => ratio(i64::from(stratum.min(reach)), i64::from(reach)),
    }
}

/// **Refinement** — does this pressure release this element?
///
/// It refines when the system brings at least the pressure its line
/// demands. Exact comparison; no tolerance. Locality dominates
/// pressure: no stratum releases what the system does not carry.
pub fn refines(element: &[u8; 32], stratum: u8, class: StarClass) -> bool {
    local_to(element, class) && pressure(stratum, class) >= periodic_of(element).refines_at
}

/// The band form of the workability read, kept for callers that already
/// hold a `SubstrateBand`. The total form is
/// [`super::thermal::workable_at`], which answers for every orb rather
/// than only those landing inside a band.
pub fn workable_in(element: &[u8; 32], band: crate::thermal::SubstrateBand) -> bool {
    let phase = periodic_of(element).phase;
    if phase == super::periodic::PHASES {
        return true;
    }
    let wants = match band {
        crate::thermal::SubstrateBand::Cryogenic => 1,
        crate::thermal::SubstrateBand::Mesic => 2,
        crate::thermal::SubstrateBand::HyperThermal => 3,
    };
    phase == wants
}
