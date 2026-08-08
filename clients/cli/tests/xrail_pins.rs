//! X7 strand / isthmus / datum / Anchor import pins.
//!
//! Pure schemas always test. Export/import deeds need
//! `deep-rail-strand` + `deep-rail-court` features on cfe.

use cfe::ratio::ratio;
use cfe::world::{World, FIRST_ORB};
use cfe::xrail::{
    decode_strand, encode_note_packet, encode_share_packet, verify_court_claim, CourtClaim,
    StrandKind,
};

#[test]
fn strand_note_share_round_trip() {
    let n = encode_note_packet(9, &ratio(2, 1));
    let d = decode_strand(&n).expect("note");
    assert_eq!(d.kind, StrandKind::Note);
    assert_eq!(d.instrument_id, 9);
    let s = encode_share_packet(3, &ratio(1, 4));
    let ds = decode_strand(&s).expect("share");
    assert_eq!(ds.kind, StrandKind::Share);
}

#[test]
fn datum_and_isthmus_compose() {
    let claim = CourtClaim {
        axes: [2, 1, 4],
        work: [9u8; 32],
        amount: ratio(7, 2),
    };
    let frame = World::wrap_court_claim_isthmus(claim.axes, claim.work, claim.amount.clone());
    let isth = cfe::xrail::decode_isthmus(&frame).expect("isthmus");
    let v = verify_court_claim(&isth.payload).expect("datum");
    assert_eq!(v.amount, ratio(7, 2));
}

#[test]
fn anchor_export_import_gated_without_feature() {
    let mut w = World::new();
    w.found("x7").expect("f");
    let amt = cfe::dop::Positive::new(ratio(1, 1)).expect("p");
    // Default build: feature off → DeepRailGated (or AnchorRequired first).
    let r = w.export_stamp_strand(FIRST_ORB, amt);
    assert!(
        matches!(
            r,
            Err(cfe::world::ActionRefused::DeepRailGated { .. })
                | Err(cfe::world::ActionRefused::AnchorRequired { .. })
                | Err(cfe::world::ActionRefused::CannotAffordStamp { .. })
        ),
        "got {r:?}"
    );
}

#[cfg(feature = "xrail-test")]
mod with_features {
    use super::*;
    use cfe::dop::{Name, Station};
    use cfe::world::{replay, Committed, MintWarrant, SOVEREIGN};

    struct Tw;
    impl MintWarrant for Tw {
        fn earns(&self, work: &[u8]) -> Option<cfe::ratio::Ratio> {
            Some(ratio(work.first().copied().unwrap_or(8).max(1) as i64, 1))
        }
        fn admits(
            &self,
            _: &Committed,
            _: &Committed,
            _: &cfe::dop::ClosureEvent,
            _: u64,
            _: u64,
        ) -> bool {
            true
        }
    }

    fn stamp_and_anchor() -> World {
        let mut w = World::new();
        w.found("x7-full").expect("f");
        w.mint_discovery_as(
            SOVEREIGN,
            FIRST_ORB,
            cfe::dop::Resource::Crystal,
            cfe::dop::NonNegative::new(ratio(8, 1)).expect("nn"),
            vec![8u8; 32],
            &Tw,
        )
        .expect("crystal");
        let convert = w.orb().unwrap().banked_amount(cfe::dop::Resource::Crystal)
            / (ratio(1, 1) + ratio(1, 1));
        cfe::world::mint_stamp_from_crystal(
            &mut w,
            FIRST_ORB,
            cfe::dop::Positive::new(convert).expect("p"),
        )
        .expect("stamp");
        let line = Name::new("x7-line").expect("name");
        w.seat_station(line, Station::Anchor).expect("anchor");
        w
    }

    #[test]
    fn export_import_stamp_round_trip() {
        let mut w = stamp_and_anchor();
        let before = w.orb().unwrap().stamp_amount();
        let amt = cfe::dop::Positive::new(ratio(1, 1)).expect("p");
        let pkt = w.export_stamp_strand(FIRST_ORB, amt).expect("export");
        assert!(w.orb().unwrap().stamp_amount() < before);
        let credited = w.import_anchor_strand(FIRST_ORB, &pkt).expect("import");
        assert_eq!(credited, ratio(1, 1));
        assert_eq!(replay(w.chronicle()), w.committed());
    }

    #[test]
    fn bad_proof_refuses_with_residual() {
        let mut w = stamp_and_anchor();
        let bad = b"not-a-packet";
        assert!(matches!(
            w.import_anchor_strand(FIRST_ORB, bad),
            Err(cfe::world::ActionRefused::AnchorProofInvalid { reason: 1 })
        ));
        assert!(w.chronicle().receipts().iter().any(|r| {
            matches!(
                r.event,
                cfe::dop::ClosureEvent::AnchorImportRefused { reason: 1, .. }
            )
        }));
    }

    #[test]
    fn court_import_credits_on_multi_axial() {
        let mut w = stamp_and_anchor();
        let frame =
            World::wrap_court_claim_isthmus([1, 2, 3], [5u8; 32], ratio(2, 1));
        let c = w.import_anchor_court(FIRST_ORB, &frame).expect("court");
        assert_eq!(c, ratio(2, 1));
        assert_eq!(replay(w.chronicle()), w.committed());
    }
}
