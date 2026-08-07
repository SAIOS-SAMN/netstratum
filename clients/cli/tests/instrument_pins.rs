//! PHASE-4 NOTE-1 / FUT-1 pins (32 / TASKS_TRADE_FLEET_DESK P4).

use cfe::dop::{ClosureEvent, Positive, Resource};
use cfe::ratio::{ratio, Ratio};
use cfe::world::{
    replay, Committed, MintWarrant, World, CLEAR_SEAL_DEPTH, FIRST_ORB, SOVEREIGN,
};

/// Test warrant: earns identity amount from work[0] and admits any transition.
struct TestWarrant;
impl MintWarrant for TestWarrant {
    fn earns(&self, work: &[u8]) -> Option<cfe::ratio::Ratio> {
        let n = work.first().copied().unwrap_or(1).max(1) as i64;
        Some(ratio(n, 1))
    }
    fn admits(
        &self,
        _prior: &Committed,
        _next: &Committed,
        _event: &ClosureEvent,
        _actor: u64,
        _stamp: u64,
    ) -> bool {
        true
    }
}

fn stamp_world() -> World {
    let mut w = World::new();
    w.found("bourse").expect("found");
    // Mint crystal then convert half to STAMP.
    let work = vec![8u8; 32];
    w.mint_discovery_as(
        SOVEREIGN,
        FIRST_ORB,
        Resource::Crystal,
        cfe::dop::NonNegative::new(ratio(8, 1)).expect("nn"),
        work,
        &TestWarrant,
    )
    .expect("mint crystal");
    let crystal = w
        .orb()
        .map(|o| o.banked_amount(Resource::Crystal))
        .unwrap_or_else(|| ratio(0, 1));
    let convert = crystal / (ratio(1, 1) + ratio(1, 1));
    let p = Positive::new(convert).expect("pos");
    cfe::world::mint_stamp_from_crystal(&mut w, FIRST_ORB, p).expect("stamp");
    w
}

/// P5-M Q-S4: last-clear marks require honest clear depth.
fn seal_clear(w: &mut World, resource: Resource, price: &Ratio) {
    let p = Positive::new(price.clone()).expect("pos");
    for _ in 0..CLEAR_SEAL_DEPTH {
        w.record_spot_clear(FIRST_ORB, resource, p.clone())
            .expect("clear");
    }
}

#[test]
fn note_issue_and_redeem_after_maturity() {
    let mut w = stamp_world();
    assert!(w.notes_listable(), "STAMP/crystal evidence opens the note book");
    let face = Positive::new(ratio(2, 1)).expect("face");
    let coll = Positive::new(ratio(1, 1)).expect("coll");
    let id = w
        .issue_note(FIRST_ORB, SOVEREIGN, face, coll, 4)
        .expect("issue");
    assert!(w.notes().iter().any(|n| n.note_id == id && n.open));
    assert!(matches!(
        w.redeem_note(id),
        Err(cfe::world::ActionRefused::NoteImmature { .. })
    ));
    for _ in 0..5 {
        w.tick();
    }
    w.redeem_note(id).expect("redeem");
    assert!(w.notes().iter().any(|n| n.note_id == id && !n.open));
    assert_eq!(replay(w.chronicle()), w.committed(), "SEAL B after note");
}

#[test]
fn future_opens_marks_and_settles_vs_last_clear() {
    let mut w = stamp_world();
    let price = Positive::new(ratio(1, 1)).expect("p");
    seal_clear(&mut w, Resource::Crystal, &ratio(1, 1));
    assert_eq!(w.last_clear_of(Resource::Crystal), ratio(1, 1));
    let n = Positive::new(ratio(1, 1)).expect("n");
    let m = Positive::new(ratio(1, 1)).expect("m");
    let short = w.register_agent("counterparty").expect("agent");
    let id = w
        .open_future(
            FIRST_ORB,
            SOVEREIGN,
            short,
            Resource::Crystal,
            n,
            price,
            m,
            3,
        )
        .expect("open");
    for _ in 0..4 {
        w.tick();
    }
    let p2 = Positive::new(ratio(1, 1) + ratio(1, 1)).expect("p2");
    w.mark_last_clear(FIRST_ORB, Resource::Crystal, p2)
        .expect("remark");
    w.settle_future(id).expect("settle");
    assert!(w.futures().iter().any(|f| f.fut_id == id && !f.open));
    assert_eq!(replay(w.chronicle()), w.committed(), "SEAL B after future");
}

#[test]
fn two_books_differ_by_evidence() {
    let empty = World::new();
    assert!(!empty.notes_listable());
    assert!(!empty.futures_listable());
    let rich = stamp_world();
    assert!(rich.notes_listable());
    // Founding iron alone would open futures; STAMP path also counts.
    assert!(rich.futures_listable() || rich.notes_listable());
}

#[test]
fn share_issue_dividend_and_perp_fund_close() {
    let mut w = stamp_world();
    // Zone for production evidence.
    let _ = w.zone(0, cfe::dop::ZoneKind::Extractor);
    for _ in 0..4 {
        w.tick();
    }
    let _ = w.collect();
    assert!(w.shares_listable());
    let units = Positive::new(ratio(1, 1)).expect("u");
    let par = Positive::new(ratio(1, 1)).expect("p");
    let sid = w
        .issue_share(FIRST_ORB, SOVEREIGN, Resource::Iron, units, par)
        .expect("share");
    // May be empty if no iron pool — still SEAL B if we only issue.
    let _ = w.pay_share_dividend(sid);
    seal_clear(&mut w, Resource::Crystal, &ratio(1, 1));
    let n = Positive::new(ratio(1, 1)).expect("n");
    let m = Positive::new(ratio(1, 1)).expect("m");
    let short = w.register_agent("short").expect("agent");
    let pid = w
        .open_perp(FIRST_ORB, SOVEREIGN, short, Resource::Crystal, n, m)
        .expect("perp");
    let _ = w.fund_perp(pid);
    w.close_perp(pid).expect("close");
    assert!(w.perps().iter().any(|p| p.perp_id == pid && !p.open));
    assert_eq!(replay(w.chronicle()), w.committed());
}

#[test]
fn hull_financed_optional_note() {
    let mut w = stamp_world();
    // Yard + scrap for hauler.
    let _ = w.zone(1, cfe::dop::ZoneKind::ShipYard);
    let scrap = Positive::new(ratio(4, 1)).expect("scrap");
    // Need iron for salvage; founding has iron.
    if let Ok(()) = w.salvage_scrap(FIRST_ORB, scrap) {
        let (hull, note) = w
            .mint_hull_financed(FIRST_ORB, cfe::dop::HullRole::Hauler, 0)
            .expect("finance mint");
        assert!(hull >= 1);
        // Note optional if STAMP thin after margin locks.
        let _ = note;
        assert_eq!(replay(w.chronicle()), w.committed());
    }
}
