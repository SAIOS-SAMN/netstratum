//! N-I5 witness — the survival seat reads the ration book through the ONE organ.
//! HAVE and OWE are mouths-as-sites over the species-ration fiber; `reduce_pair`
//! closes the book (Fed) when the larder covers every mouth and withholds
//! (Short) when a mouth is unfed. The species is the COLUMN — the owed direction
//! is passed in, never a hardcoded Biomass; the same reading serves any ration.

use solver::{ration_reads, think_survival, RationRead, SurvivalBook, SurvivalDeed};

const BIOMASS: u8 = 2; // a biotic line's ration direction (Resource discriminant)
const JOULE: u8 = 5; // a plasmic line owes joule, not biomass
const ELEMENT: u8 = 8; // an element-eater's reserved ration slot

/// A book that CLOSES on the ration seat (every mouth fed, a source stands, no
/// matter opened) — the starting point each E/F/H witness perturbs.
fn fed_book() -> SurvivalBook {
    SurvivalBook {
        ration_source_sites: 1,
        mouths: 4,
        fed: 4,
        ration_dir: BIOMASS,
        accrued: 0,
        habitat_support: 8,
        habitats: 2,
        extractors: 1,
        matter_opened: false,
        alloy: 0,
        iron: 0,
    }
}

#[test]
fn fed_mouths_close_the_book() {
    assert_eq!(ration_reads(4, 4, BIOMASS), RationRead::Fed, "every mouth fed → the book closes");
}

#[test]
fn a_short_mouth_withholds() {
    assert_eq!(
        ration_reads(4, 3, BIOMASS),
        RationRead::Short,
        "one unfed mouth → the book does not close (Pin G withholds the beat)",
    );
}

#[test]
fn a_surplus_larder_still_closes() {
    // The larder covers ten mouths but only four eat — surplus feeds no phantom.
    assert_eq!(ration_reads(4, 10, BIOMASS), RationRead::Fed, "surplus closes, never over-owes");
}

#[test]
fn zero_mouths_close_trivially() {
    // A line with no working force owes nothing — an empty pair closes.
    assert_eq!(ration_reads(0, 0, BIOMASS), RationRead::Fed, "no mouths, nothing to feed");
}

#[test]
fn the_species_is_the_column() {
    // The SAME reading serves any ration direction — a plasmic line owes joule,
    // an element-eater owes its element; neither is biomass, and the verdict is
    // identical to the biotic case for the same fed/mouths. No Biomass hardcoded.
    for dir in [BIOMASS, JOULE, ELEMENT] {
        assert_eq!(ration_reads(3, 3, dir), RationRead::Fed, "dir {dir}: covered closes");
        assert_eq!(ration_reads(3, 1, dir), RationRead::Short, "dir {dir}: short withholds");
    }
}

#[test]
fn one_short_mouth_in_a_large_force_still_withholds() {
    // The book is exact at mouth granularity: a single unfed mouth among many is
    // a Remainder, not rounded away.
    assert_eq!(ration_reads(64, 63, JOULE), RationRead::Short, "63/64 fed still withholds");
    assert_eq!(ration_reads(64, 64, JOULE), RationRead::Fed, "64/64 closes");
}

// ───────────────────────── N-I6 — the residual names the deed ─────────────────

#[test]
fn a_fed_closed_book_rests() {
    // Ration fed, no matter opened — the survival book names nothing; ARC runs.
    assert_eq!(think_survival(&fed_book()), SurvivalDeed::Rest, "a closed book rests");
}

#[test]
fn founding_seats_the_ration_source() {
    // Charter §3: a settled orb yields the ration nowhere → seat the source,
    // whatever the mouths (labor_force 0 → the mouths peel would read Fed empty).
    let book = SurvivalBook { ration_source_sites: 0, mouths: 0, fed: 0, ..fed_book() };
    assert_eq!(
        think_survival(&book),
        SurvivalDeed::SeatRationSource,
        "no ration-yielding site → the leftover names the source (not Rest, not Policy)",
    );
}

#[test]
fn pin_h_accrued_covers_the_shortfall_names_collect() {
    // Short by two mouths, but the accrued larder covers two — the cycle collect
    // IS the deed; no zone.
    let book = SurvivalBook { mouths: 4, fed: 2, accrued: 2, ..fed_book() };
    assert_eq!(think_survival(&book), SurvivalDeed::Collect, "accrued covers → Collect (Pin H)");
}

#[test]
fn pin_e_housing_short_names_habitat_before_a_raze() {
    // Short, accrued does NOT cover, and housing support is below the mouths →
    // Habitat. A third Extractor is never in the vocabulary.
    let book = SurvivalBook {
        mouths: 6,
        fed: 2,
        accrued: 0,
        habitat_support: 3, // < 6 mouths
        habitats: 2,
        extractors: 1,
        ..fed_book()
    };
    assert_eq!(think_survival(&book), SurvivalDeed::Habitat, "housing short → Habitat (Pin E)");
}

#[test]
fn pin_e_extractors_outweigh_habitats_names_raze() {
    // Short, accrued short, housing COVERS the mouths, but extractors outweigh
    // the habitats and drain the food column → raze one (the inverse of build).
    let book = SurvivalBook {
        mouths: 3,
        fed: 1,
        accrued: 0,
        habitat_support: 9, // covers the 3 mouths
        habitats: 2,
        extractors: 5, // outweigh the 2 habitats
        ..fed_book()
    };
    assert_eq!(
        think_survival(&book),
        SurvivalDeed::RazeExtractor,
        "extractors > habitats → raze (Pin E), never a third extractor",
    );
}

#[test]
fn pin_f_fed_matter_leftover_names_industry() {
    // Fed, matter opened. Alloy over iron → Synthesizer; iron over alloy →
    // Smelter; balanced → Rest. The matter book is a SEPARATE seat from ration.
    let synth = SurvivalBook { matter_opened: true, alloy: 20, iron: 5, ..fed_book() };
    let smelt = SurvivalBook { matter_opened: true, alloy: 5, iron: 20, ..fed_book() };
    let balanced = SurvivalBook { matter_opened: true, alloy: 7, iron: 7, ..fed_book() };
    assert_eq!(think_survival(&synth), SurvivalDeed::Synthesizer, "alloy > iron → Synthesizer");
    assert_eq!(think_survival(&smelt), SurvivalDeed::Smelter, "iron > alloy → Smelter");
    assert_eq!(think_survival(&balanced), SurvivalDeed::Rest, "balanced matter → no leftover");
}

#[test]
fn a_fed_line_does_not_open_matter_unbidden() {
    // Even with a matter imbalance, a Fed line whose Need did NOT open matter
    // rests — ration and matter are sequential seats, matter is not free.
    let book = SurvivalBook { matter_opened: false, alloy: 20, iron: 5, ..fed_book() };
    assert_eq!(think_survival(&book), SurvivalDeed::Rest, "matter shut → Rest, ARC runs");
}
