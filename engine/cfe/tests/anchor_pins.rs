//! **THE ANCHOR SET IS FINITE AND PINNED EXACTLY** (P2, operator
//! ruling 2026-08-05).
//!
//! The dial ratchet excludes `cfe::anchor` from its walk because this
//! file holds it to the opposite discipline: every anchored value is
//! pinned here against the citation it carries, so a value cannot
//! enter, leave, or change without going red — the exact opposite of a
//! ceiling that only falls. Laundering a dial into the anchor set
//! means adding a line HERE, in review, wearing a citation.

use cfe::anchor::{luminosity, KelvinAnchor};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;

/// The seven kelvin anchors, value for value.
#[test]
fn the_kelvin_anchors_stand_exactly() {
    let table: [(KelvinAnchor, i64); 7] = [
        (KelvinAnchor::SolEquilibrium, 279),
        (KelvinAnchor::CryoFloor, 10),
        (KelvinAnchor::CryoCeiling, 100),
        (KelvinAnchor::MesicFloor, 250),
        (KelvinAnchor::MesicCeiling, 320),
        (KelvinAnchor::FoundryFloor, 600),
        (KelvinAnchor::FoundryCeiling, 1500),
    ];
    for (anchor, kelvin) in table {
        assert_eq!(anchor.kelvin(), kelvin, "{anchor:?} moved off its citation");
    }
}

/// The twelve luminosity anchors, value for value — and the lore class
/// is NOT among them: fiction has no citation, so its value stays a
/// counted dial where it is used.
#[test]
fn the_luminosity_anchors_stand_exactly() {
    let table = [
        (StarClass::O, ratio(100_000, 1)),
        (StarClass::B, ratio(20_000, 1)),
        (StarClass::A, ratio(20, 1)),
        (StarClass::F, ratio(3, 1)),
        (StarClass::G, ratio(1, 1)),
        (StarClass::K, ratio(3, 10)),
        (StarClass::M, ratio(1, 100)),
        (StarClass::BrownDwarf, ratio(1, 10_000)),
        (StarClass::WhiteDwarf, ratio(1, 100)),
        (StarClass::RedGiant, ratio(1_000, 1)),
        (StarClass::NeutronStar, ratio(100, 1)),
        (StarClass::WolfRayet, ratio(200_000, 1)),
    ];
    for (class, cited) in table {
        assert_eq!(
            luminosity(class),
            Some(cited),
            "{class:?} moved off its citation"
        );
    }
    assert_eq!(
        luminosity(StarClass::LuminalWhiteHole),
        None,
        "lore is not an anchor"
    );
}

/// The derivation chain holds end to end: the folded constant IS the
/// Sol anchor to the fourth, and every band bound IS its anchor's
/// quartic — the values the old flattened literals encoded, unchanged.
#[test]
fn the_thermal_law_reads_the_anchors_it_always_encoded() {
    assert_eq!(cfe::thermal::thermal_k(), ratio(6_059_221_281, 1));
    let (lo, hi) = cfe::thermal::SubstrateBand::Mesic.bounds4();
    assert_eq!((lo, hi), (ratio(3_906_250_000, 1), ratio(10_485_760_000, 1)));
    let (lo, hi) = cfe::thermal::SubstrateBand::Cryogenic.bounds4();
    assert_eq!((lo, hi), (ratio(10_000, 1), ratio(100_000_000, 1)));
    let (lo, hi) = cfe::thermal::SubstrateBand::HyperThermal.bounds4();
    assert_eq!((lo, hi), (ratio(129_600_000_000, 1), ratio(5_062_500_000_000, 1)));
    // And the lore dial still speaks through the same surface.
    assert_eq!(StarClass::LuminalWhiteHole.luminosity(), ratio(1_000_000, 1));
}
