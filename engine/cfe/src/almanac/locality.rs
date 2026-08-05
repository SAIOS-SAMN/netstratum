//! LOCALITY AND PRESSURE — what a star and a planet make reachable.
//!
//! Two INDEPENDENT readings, and an element needs both:
//!
//!   the STAR  says how DEEP a system's strata run  -> [`local_to`]
//!   the ORBIT says how HOT the orb is              -> the thermal
//!                                                     region, in
//!                                                     [`super::thermal`]
//!
//! Neither implies the other. A shallow element under a heavy star can
//! still be unworkable in the wrong band, and that is what makes a
//! resource a PLACE rather than a number.

use super::periodic::{periodic_of, LINES};
use crate::ratio::{ratio, Ratio};
use crate::thermal::StarClass;

/// Which lines a WORLD can reach at all.
///
/// A heavy, bright class forges and disperses down to the deep lines; a
/// dim one never reaches them. So the same element id is REACHABLE in
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

/// Is this element PRESENT in a system of this class? Locality as a
/// predicate: deep lines simply do not occur under a shallow star.
pub fn local_to(element: &[u8; 32], class: StarClass) -> bool {
    periodic_of(element).line <= lines_of(class)
}

/// **THE PRESSURE OF A SYSTEM** — the stratum an orb has opened,
/// against the deepest line its star offers.
pub fn pressure(stratum: u8, class: StarClass) -> Ratio {
    let reach = lines_of(class);
    match reach {
        0 => ratio(0, 1),
        _ => ratio(i64::from(stratum.min(reach)), i64::from(reach)),
    }
}

/// **REFINEMENT** — does this pressure release this element?
///
/// It refines when the system brings at least the pressure its line
/// demands. Exact comparison; no tolerance. Locality DOMINATES
/// pressure: no stratum releases what the system does not carry.
pub fn refines(element: &[u8; 32], stratum: u8, class: StarClass) -> bool {
    local_to(element, class) && pressure(stratum, class) >= periodic_of(element).refines_at
}

/// The band form of the workability read, kept for callers that already
/// hold a `SubstrateBand`. The TOTAL form is
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
