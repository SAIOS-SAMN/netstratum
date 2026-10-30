//! **The physical anchors** (P2, operator ruling 2026-08-05).
//!
//! A constant is a measured anchor only when its provenance is
//! structurally bound: each value here is reachable through a typed
//! declaration that names the empirical invariant it stands on, and
//! through nothing else. There is no bare `measured()` constructor in
//! this tree — that would be a dial in a lab coat.
//!
//! **The census law.** The tuned-constant ratchet walks every source
//! tree counting dials (tunables — target zero, ceiling only falls)
//! and holds this file to the opposite discipline: the anchor set is
//! finite and pinned exactly (`anchor_pins.rs`) — a value cannot enter,
//! leave, or change without going red against its own citation.
//!
//! **Honesty over count.** A value with no empirical citation does not
//! belong here, whatever that does to the dial ledger: the Luminal
//! White Hole is lore, so its luminosity stays a counted dial in
//! `thermal.rs`. Stylized values are declared as stylized — an anchor
//! cites what it stands near and says so, or it is not an anchor.

use crate::ratio::{ratio, Ratio};
use crate::thermal::StarClass;

/// **A thermal anchor** — a temperature the thermal law stands on, in
/// whole degrees. `thermal::anchor_degrees` accepts this type and nothing
/// else, so a bound cannot be built from a bare number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalAnchor {
    /// 279° — the home equilibrium: a yellow star's world at unit orbit,
    /// zero albedo, chartered whole.
    /// This is the calibration the folded constant `K = 279` encodes.
    HomeEquilibrium,
    /// 10° — the cryogenic floor: the coldest working band, where only
    /// the lightest working gas still flows; the regime of
    /// superconducting and quantum substrate.
    CryoFloor,
    /// 100° — the cryogenic ceiling: the top of the coldest working
    /// band, with margin; warmer than this, cryo-substrate decoheres
    /// and the ember vault leaks (05 §1.2).
    CryoCeiling,
    /// 250° — the mesic floor: the cold edge of the liquid band of
    /// biotic life, chartered wider at the cold edge for brine and
    /// subsurface melt.
    MesicFloor,
    /// 320° — the mesic ceiling: the warm edge of the liquid band of
    /// biotic life; carbon biochemistry fails above it.
    MesicCeiling,
    /// 600° — the foundry floor: where soft metals melt — the entry
    /// to the foundry band.
    FoundryFloor,
    /// 1500° — the foundry ceiling: where lithic matter melts; above
    /// it there is no lithic substrate to stand on.
    FoundryCeiling,
}

impl ThermalAnchor {
    /// The chartered value, in whole degrees.
    pub fn degrees(self) -> i64 {
        match self {
            ThermalAnchor::HomeEquilibrium => 279,
            ThermalAnchor::CryoFloor => 10,
            ThermalAnchor::CryoCeiling => 100,
            ThermalAnchor::MesicFloor => 250,
            ThermalAnchor::MesicCeiling => 320,
            ThermalAnchor::FoundryFloor => 600,
            ThermalAnchor::FoundryCeiling => 1500,
        }
    }
}

/// **The stellar luminosity anchors** — representative luminosities
/// per class, relative to the yellow class = 1, each stated against the
/// range its class spans. `None` for the lore classes: fiction has no
/// empirical citation, so its value stays a dial where it is used.
/// Main-sequence and remnant mass in yellow-star masses (the yellow
/// class = 1) — the second stellar anchor, the one Kepler's third law
/// reads (`P² ∝ r³/M`). Read along the colour classes' mass sequence;
/// the lore White Hole has no citation and its value lives as a dial
/// where it is used.
pub fn mass(class: StarClass) -> Option<Ratio> {
    match class {
        // Blue: 16–90; 20 representative of the massive, fast-burning hot end.
        StarClass::Blue => Some(ratio(20, 1)),
        // Blue-white: 2.1–16; 5 mid-class.
        StarClass::BlueWhite => Some(ratio(5, 1)),
        // White: 1.4–2.1; 2 representative.
        StarClass::White => Some(ratio(2, 1)),
        // Yellow-white: 1.0–1.4; 3/2 mid-class.
        StarClass::YellowWhite => Some(ratio(3, 2)),
        // Yellow: the unit itself.
        StarClass::Yellow => Some(ratio(1, 1)),
        // Orange: 0.45–0.8; 3/4 representative.
        StarClass::Orange => Some(ratio(3, 4)),
        // Red: 0.08–0.45; 1/5 the dim red star.
        StarClass::Red => Some(ratio(1, 5)),
        // Brown dwarf: 0.013–0.08 (sub-stellar); 1/20.
        StarClass::BrownDwarf => Some(ratio(1, 20)),
        // White dwarf: 0.5–0.7 (degenerate remnant); 3/5.
        StarClass::WhiteDwarf => Some(ratio(3, 5)),
        // Red giant: 0.5–8 up the giant branch; 2 representative.
        StarClass::RedGiant => Some(ratio(2, 1)),
        // Neutron star: ~1.4 (the degenerate-core limit); 3/2.
        StarClass::NeutronStar => Some(ratio(3, 2)),
        // Wind-stripped: 10–25; 20 representative.
        StarClass::WindStripped => Some(ratio(20, 1)),
        // Lore — no citation exists; the dial lives in thermal.rs.
        StarClass::LuminalWhiteHole => None,
    }
}

pub fn luminosity(class: StarClass) -> Option<Ratio> {
    match class {
        // Blue: 30,000–1,000,000 measured range; 10⁵ representative.
        StarClass::Blue => Some(ratio(100_000, 1)),
        // Blue-white: 25–30,000 across the class; 2×10⁴ takes the hot end.
        StarClass::BlueWhite => Some(ratio(20_000, 1)),
        // White: 5–25; 20 representative.
        StarClass::White => Some(ratio(20, 1)),
        // Yellow-white: 1.5–5; 3 representative.
        StarClass::YellowWhite => Some(ratio(3, 1)),
        // Yellow: the unit itself.
        StarClass::Yellow => Some(ratio(1, 1)),
        // Orange: 0.08–0.6; 3/10 representative.
        StarClass::Orange => Some(ratio(3, 10)),
        // Red: 10⁻⁴–0.08; 1/100 takes the bright end.
        StarClass::Red => Some(ratio(1, 100)),
        // Brown dwarf: 10⁻⁶–10⁻⁴ bolometric; the bright end.
        StarClass::BrownDwarf => Some(ratio(1, 10_000)),
        // White dwarf: 10⁻⁴–10⁻² for field dwarfs; the bright end.
        StarClass::WhiteDwarf => Some(ratio(1, 100)),
        // Red giant: 10²–10³ up the giant branch; 10³ representative.
        StarClass::RedGiant => Some(ratio(1_000, 1)),
        // Neutron star: young pulsar spin-down luminosity reaches
        // 10²–10⁴ (thermal surface emission is far dimmer);
        // 100 takes the spin-down reading, stylized bright.
        StarClass::NeutronStar => Some(ratio(100, 1)),
        // Wind-stripped: 10⁵–10⁶ measured; 2×10⁵ representative.
        StarClass::WindStripped => Some(ratio(200_000, 1)),
        // Lore — no citation exists; the dial lives in thermal.rs.
        StarClass::LuminalWhiteHole => None,
    }
}
