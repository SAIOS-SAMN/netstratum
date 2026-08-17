//! Wave-6 offices as play — O6.3 Exchange · O6.4 Assay · O6.5 Vault.

use cfe::dop::{Name, Positive, Resource, Station};
use cfe::ratio::ratio;
use cfe::world::{
    replay, Committed, MintWarrant, World, CLEAR_SEAL_DEPTH, FIRST_ORB, SOVEREIGN,
};

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

fn stamp_world(name: &str) -> World {
    let mut w = World::new();
    w.found(name).expect("found");
    w.mint_discovery_as(
        SOVEREIGN,
        FIRST_ORB,
        Resource::Crystal,
        cfe::dop::NonNegative::new(ratio(16, 1)).expect("nn"),
        vec![16u8; 32],
        &Tw,
    )
    .expect("crystal");
    let convert = w.orb().unwrap().banked_amount(Resource::Crystal)
        / (ratio(1, 1) + ratio(1, 1));
    cfe::world::mint_stamp_from_crystal(
        &mut w,
        FIRST_ORB,
        Positive::new(convert).expect("p"),
    )
    .expect("stamp");
    w
}

#[test]
fn o6_3_exchange_mm_lists_instruments() {
    let mut w = stamp_world("mm");
    let face = Positive::new(ratio(2, 1)).expect("f");
    let coll = Positive::new(ratio(1, 1)).expect("c");
    assert!(matches!(
        w.issue_note(FIRST_ORB, SOVEREIGN, face.clone(), coll.clone(), 4),
        Err(cfe::world::ActionRefused::ExchangeRequired)
    ));
    w.seat_station(Name::new("desk").unwrap(), Station::Exchange)
        .expect("seat");
    for _ in 0..CLEAR_SEAL_DEPTH {
        w.record_spot_clear(
            FIRST_ORB,
            Resource::Crystal,
            Positive::new(ratio(1, 1)).unwrap(),
        )
        .expect("clear");
    }
    let id = w
        .issue_note(FIRST_ORB, SOVEREIGN, face, coll, 4)
        .expect("issue under MM");
    assert!(w.notes().iter().any(|n| n.note_id == id && n.open));
    assert_eq!(replay(w.chronicle()), w.committed(), "SEAL B note under MM");
}

#[test]
fn o6_5_vault_deposit_withdraw_and_margin_hold() {
    let mut w = stamp_world("vault");
    let amt = Positive::new(ratio(2, 1)).expect("a");
    assert!(matches!(
        w.vault_deposit(FIRST_ORB, amt.clone()),
        Err(cfe::world::ActionRefused::VaultRequired)
    ));
    w.seat_station(Name::new("custody").unwrap(), Station::Vault)
        .expect("vault");
    w.seat_station(Name::new("desk").unwrap(), Station::Exchange)
        .expect("mm");
    let bal = w.vault_deposit(FIRST_ORB, amt.clone()).expect("dep");
    assert_eq!(bal, ratio(2, 1));
    assert_eq!(w.vault_balance_of(FIRST_ORB), ratio(2, 1));
    // Lock a note against vault free (earmark, not free-bank debit).
    for _ in 0..CLEAR_SEAL_DEPTH {
        w.record_spot_clear(
            FIRST_ORB,
            Resource::Crystal,
            Positive::new(ratio(1, 1)).unwrap(),
        )
        .expect("clear");
    }
    let face = Positive::new(ratio(2, 1)).expect("f");
    let coll = Positive::new(ratio(1, 1)).expect("c");
    w.issue_note(FIRST_ORB, SOVEREIGN, face, coll, 4)
        .expect("note on vault margin");
    assert_eq!(w.vault_reserved(FIRST_ORB), ratio(1, 1));
    // Free vault = 2 − 1 = 1; withdraw 2 refuses margin hold.
    assert!(matches!(
        w.vault_withdraw(FIRST_ORB, Positive::new(ratio(2, 1)).unwrap()),
        Err(cfe::world::ActionRefused::VaultMarginHeld { .. })
    ));
    let left = w
        .vault_withdraw(FIRST_ORB, Positive::new(ratio(1, 1)).unwrap())
        .expect("free withdraw");
    assert_eq!(left, ratio(1, 1));
    assert_eq!(replay(w.chronicle()), w.committed(), "SEAL B vault");
}

// ONT-2: o6_4_assay_consilience_match is DELETED. The assay-consilience court
// existed only to catch a stored richness scalar drifting from the geometric
// truth; richness is now derived (natural − extracted) and cannot drift, so
// the court — and this test — no longer have anything to challenge.
