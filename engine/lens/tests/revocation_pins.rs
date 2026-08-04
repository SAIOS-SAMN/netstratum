//! **D4 OVER A WINDOW** — the lens half (EQUATIONS D4).
//!
//! `deeds(w) > 0 ∧ ΔD(w) = 0` vacates the allocation claim. The degree
//! is untouched, nothing is scored, and this reading gates nothing.

use lens::revocation::{Claim, Revocation};

/// THE ROW ITSELF: deeds emitted while nothing grew.
#[test]
fn deeds_without_growth_vacate_the_claim() {
    let read = Revocation::from_counts(7, 2, 2);
    assert_eq!(read.claim, Claim::Vacated);
    assert_eq!(read.growth(), 0);
    assert!(!read.claim.stood());
    // The counts ride along so a reader can check the row rather than
    // trust it.
    assert_eq!((read.deeds, read.circulation_before, read.circulation_after), (7, 2, 2));
}

/// Growth earns the claim.
#[test]
fn deeds_with_growth_stand() {
    let read = Revocation::from_counts(3, 1, 4);
    assert_eq!(read.claim, Claim::Stood);
    assert_eq!(read.growth(), 3);
    assert!(read.claim.stood());
}

/// A STILL WINDOW IS NOT A DELINQUENT ONE. No deeds means no claim was
/// made, so there is nothing to vacate — the rule is `deeds(w) > 0 ∧
/// ΔD(w) = 0`, and the first conjunct is load-bearing.
#[test]
fn a_quiet_window_claims_nothing_and_vacates_nothing() {
    for circulation in [0usize, 1, 9] {
        let read = Revocation::from_counts(0, circulation, circulation);
        assert_eq!(read.claim, Claim::Quiet, "no deeds, no claim");
        assert!(read.claim.stood(), "a still world is not delinquent");
    }
}

/// THE VACUITY GUARD ON THE ROW ITSELF. Zero circulation on both sides
/// with deeds emitted is the sharpest case — and it must read Vacated
/// rather than being waved through by a comparison that treats "no
/// structure" as "no change to complain about".
#[test]
fn a_flat_field_with_deeds_vacates() {
    let read = Revocation::from_counts(1, 0, 0);
    assert_eq!(read.claim, Claim::Vacated);
    assert_eq!(read.growth(), 0);
}

/// D NEVER RETREATS (Article IV), so a fall is unrepresentable. If one
/// is ever observed it must read as NO GROWTH — the conservative
/// direction — never as a negative that could underflow or, worse, be
/// read as a debit against the degree.
#[test]
fn an_impossible_fall_reads_as_no_growth_never_as_a_debit() {
    let read = Revocation::from_counts(5, 4, 1);
    assert_eq!(read.growth(), 0, "saturating, and never below zero");
    assert_eq!(read.claim, Claim::Vacated, "no growth vacates");
}

/// NO SCALAR METER. The struck `Ξ(w) = deeds(w)/(1 + ΔD(w))` would rank
/// windows against one another; the reading is a typed row plus its
/// counts. Two windows with very different deed volumes and the same
/// growth read the SAME row — there is no ordering between them.
#[test]
fn the_reading_is_a_row_and_never_a_score() {
    let small = Revocation::from_counts(1, 3, 3);
    let vast = Revocation::from_counts(10_000, 3, 3);
    assert_eq!(small.claim, vast.claim, "the row does not grade by volume");
    assert_eq!(small.growth(), vast.growth());
    // The deed counts differ and are both visible — the reader may draw
    // their own conclusion; the lens does not draw it for them.
    assert_ne!(small.deeds, vast.deeds);
}

/// Every row states itself in plain terms, and none of them is a fault.
#[test]
fn every_row_reads_as_a_state_not_a_verdict() {
    for claim in [Claim::Quiet, Claim::Stood, Claim::Vacated] {
        let reads = claim.reads();
        assert!(!reads.is_empty());
        for accusation in ["fail", "illegal", "punish", "guilty", "bad"] {
            assert!(
                !reads.to_lowercase().contains(accusation),
                "{claim:?} reads as an accusation: {reads}"
            );
        }
    }
}
