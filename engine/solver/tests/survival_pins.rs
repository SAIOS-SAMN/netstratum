//! N-I5 witness — the survival seat reads the ration book through the ONE organ.
//! HAVE and OWE are mouths-as-sites over the species-ration fiber; `reduce_pair`
//! closes the book (Fed) when the larder covers every mouth and withholds
//! (Short) when a mouth is unfed. The species is the COLUMN — the owed direction
//! is passed in, never a hardcoded Biomass; the same reading serves any ration.

use solver::{ration_reads, RationRead};

const BIOMASS: u8 = 2; // a biotic line's ration direction (Resource discriminant)
const JOULE: u8 = 5; // a plasmic line owes joule, not biomass
const ELEMENT: u8 = 8; // an element-eater's reserved ration slot

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
