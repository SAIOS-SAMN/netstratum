//! **THE PHYSICAL ANCHORS** (P2, operator ruling 2026-08-05).
//!
//! A constant is a measured anchor ONLY when its provenance is
//! structurally bound: each value here is reachable through a typed
//! declaration that names the empirical invariant it stands on, and
//! through nothing else. There is no bare `measured()` constructor in
//! this tree — that would be a dial in a lab coat.
//!
//! **The census law.** The tuned-constant ratchet walks every source
//! tree counting DIALS (tunables — target zero, ceiling only falls)
//! and holds THIS file to the opposite discipline: the anchor set is
//! FINITE and PINNED EXACTLY (`anchor_pins.rs`) — a value cannot enter,
//! leave, or change without going red against its own citation.
//!
//! **Honesty over count.** A value with no empirical citation does not
//! belong here, whatever that does to the dial ledger: the Luminal
//! White Hole is lore, so its luminosity stays a counted dial in
//! `thermal.rs`. Stylized values are declared as stylized — an anchor
//! cites what it stands NEAR and says so, or it is not an anchor.

use crate::ratio::{ratio, Ratio};
use crate::thermal::StarClass;

/// **A KELVIN ANCHOR** — a temperature the thermal law stands on, in
/// whole kelvin. `thermal::kelvin4` accepts THIS TYPE and nothing
/// else, so a fourth-power bound cannot be built from a bare number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KelvinAnchor {
    /// 279 K — the blackbody equilibrium of a Sol-type body at 1 AU,
    /// zero albedo: `(L☉ / 16πσ·AU²)^¼ ≈ 278.6 K`, chartered whole.
    /// This is the calibration the folded constant `K = 279⁴` encodes.
    SolEquilibrium,
    /// 10 K — the cryogenic floor: below the condensation point of
    /// every working gas but helium (H₂ boils at 20 K); the regime of
    /// superconducting and quantum substrate.
    CryoFloor,
    /// 100 K — the cryogenic ceiling: above liquid-nitrogen range
    /// (77 K) with margin; warmer than this, cryo-substrate decoheres
    /// and the joule vault leaks (05 §1.2).
    CryoCeiling,
    /// 250 K — the mesic floor: the liquid-water/organics band
    /// (273–373 K at 1 atm), chartered wider at the cold edge for
    /// brine and subsurface melt.
    MesicFloor,
    /// 320 K — the mesic ceiling: protein denaturation onsets in the
    /// 310–330 K range; carbon biochemistry fails above it.
    MesicCeiling,
    /// 600 K — the hyperthermal floor: lead melts (600 K), sulfur
    /// boils (718 K) — the entry to the foundry regime.
    FoundryFloor,
    /// 1500 K — the hyperthermal ceiling: silicate rock melts
    /// (1400–1600 K); above it there is no lithic substrate to stand
    /// on.
    FoundryCeiling,
}

impl KelvinAnchor {
    /// The chartered value, in whole kelvin.
    pub fn kelvin(self) -> i64 {
        match self {
            KelvinAnchor::SolEquilibrium => 279,
            KelvinAnchor::CryoFloor => 10,
            KelvinAnchor::CryoCeiling => 100,
            KelvinAnchor::MesicFloor => 250,
            KelvinAnchor::MesicCeiling => 320,
            KelvinAnchor::FoundryFloor => 600,
            KelvinAnchor::FoundryCeiling => 1500,
        }
    }
}

/// **THE STELLAR LUMINOSITY ANCHORS** — representative luminosities
/// per class, relative to Sol = 1, each cited against the astronomy it
/// stands near. `None` for the lore classes: fiction has no empirical
/// citation, so its value stays a DIAL where it is used.
/// Main-sequence and remnant MASS in solar masses (`M☉ = 1`) — the second
/// stellar anchor, the one Kepler's third law reads (`P² ∝ r³/M`). Cited to the
/// Morgan–Keenan mass sequence; the lore White Hole has no citation and its
/// value lives as a dial where it is used.
pub fn mass(class: StarClass) -> Option<Ratio> {
    match class {
        // O V: 16–90 M☉; 20 representative of the massive, fast-burning hot end.
        StarClass::O => Some(ratio(20, 1)),
        // B V: 2.1–16 M☉; 5 mid-class.
        StarClass::B => Some(ratio(5, 1)),
        // A V: 1.4–2.1 M☉ (Sirius A ≈ 2); 2 representative.
        StarClass::A => Some(ratio(2, 1)),
        // F V: 1.0–1.4 M☉; 3/2 mid-class.
        StarClass::F => Some(ratio(3, 2)),
        // G V: Sol, the unit itself.
        StarClass::G => Some(ratio(1, 1)),
        // K V: 0.45–0.8 M☉; 3/4 representative.
        StarClass::K => Some(ratio(3, 4)),
        // M V: 0.08–0.45 M☉; 1/5 the dim red dwarf.
        StarClass::M => Some(ratio(1, 5)),
        // Brown dwarf: 0.013–0.08 M☉ (sub-stellar); 1/20.
        StarClass::BrownDwarf => Some(ratio(1, 20)),
        // White dwarf: 0.5–0.7 M☉ (degenerate remnant); 3/5.
        StarClass::WhiteDwarf => Some(ratio(3, 5)),
        // Red giant: 0.5–8 M☉ up the RGB; 2 representative.
        StarClass::RedGiant => Some(ratio(2, 1)),
        // Neutron star: ~1.4 M☉ (the Chandrasekhar mass); 3/2.
        StarClass::NeutronStar => Some(ratio(3, 2)),
        // Wolf–Rayet: 10–25 M☉; 20 representative.
        StarClass::WolfRayet => Some(ratio(20, 1)),
        // LORE — no citation exists; the dial lives in thermal.rs.
        StarClass::LuminalWhiteHole => None,
    }
}

pub fn luminosity(class: StarClass) -> Option<Ratio> {
    match class {
        // O V: 30,000–1,000,000 L☉ measured range; 10⁵ representative.
        StarClass::O => Some(ratio(100_000, 1)),
        // B V: 25–30,000 L☉ across the class; 2×10⁴ takes the hot end.
        StarClass::B => Some(ratio(20_000, 1)),
        // A V: 5–25 L☉ (Sirius A ≈ 25); 20 representative.
        StarClass::A => Some(ratio(20, 1)),
        // F V: 1.5–5 L☉ (Procyon ≈ 7); 3 representative.
        StarClass::F => Some(ratio(3, 1)),
        // G V: Sol, the unit itself.
        StarClass::G => Some(ratio(1, 1)),
        // K V: 0.08–0.6 L☉; 3/10 representative.
        StarClass::K => Some(ratio(3, 10)),
        // M V: 10⁻⁴–0.08 L☉ (Proxima ≈ 0.0017); 1/100 takes the
        // bright end.
        StarClass::M => Some(ratio(1, 100)),
        // Brown dwarf: 10⁻⁶–10⁻⁴ L☉ bolometric; the bright end.
        StarClass::BrownDwarf => Some(ratio(1, 10_000)),
        // White dwarf: 10⁻⁴–10⁻² L☉ for field dwarfs; the bright end.
        StarClass::WhiteDwarf => Some(ratio(1, 100)),
        // Red giant: 10²–10³ L☉ up the RGB; 10³ representative.
        StarClass::RedGiant => Some(ratio(1_000, 1)),
        // Neutron star: young pulsar SPIN-DOWN luminosity reaches
        // 10²–10⁴ L☉ (thermal surface emission is far dimmer);
        // 100 takes the spin-down reading, stylized bright.
        StarClass::NeutronStar => Some(ratio(100, 1)),
        // Wolf–Rayet: 10⁵–10⁶ L☉ measured; 2×10⁵ representative.
        StarClass::WolfRayet => Some(ratio(200_000, 1)),
        // LORE — no citation exists; the dial lives in thermal.rs.
        StarClass::LuminalWhiteHole => None,
    }
}
