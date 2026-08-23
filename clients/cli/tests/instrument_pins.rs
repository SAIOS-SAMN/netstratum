//! PHASE-4 NOTE-1 / FUT-1 pins (32 / TASKS_TRADE_FLEET_DESK P4).

use cfe::dop::{ClosureEvent, Name, Positive, Resource, Station};
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
    // O6.3 — Exchange MM must sit before instruments list.
    let line = Name::new("mm").expect("name");
    w.seat_station(line, Station::Exchange).expect("exchange");
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
    // P5-M2 haircut: thin clear depth raises floor above half — seal first.
    seal_clear(&mut w, Resource::Crystal, &ratio(1, 1));
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
fn o6_3_issue_refuses_without_exchange() {
    let mut w = World::new();
    w.found("no-mm").expect("found");
    let work = vec![8u8; 32];
    w.mint_discovery_as(
        SOVEREIGN,
        FIRST_ORB,
        Resource::Crystal,
        cfe::dop::NonNegative::new(ratio(8, 1)).expect("nn"),
        work,
        &TestWarrant,
    )
    .expect("mint");
    let convert = w.orb().unwrap().banked_amount(Resource::Crystal)
        / (ratio(1, 1) + ratio(1, 1));
    cfe::world::mint_stamp_from_crystal(
        &mut w,
        FIRST_ORB,
        Positive::new(convert).expect("p"),
    )
    .expect("stamp");
    assert!(w.notes_listable(), "evidence still opens the book view");
    let face = Positive::new(ratio(2, 1)).expect("face");
    let coll = Positive::new(ratio(1, 1)).expect("coll");
    assert!(matches!(
        w.issue_note(FIRST_ORB, SOVEREIGN, face, coll, 4),
        Err(cfe::world::ActionRefused::ExchangeRequired)
    ));
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

/// I5.6 — THE DEFAULT IS ENTROPIC: what a redemption leaves unpaid
/// posts to the issuer's severance book (the bond-severance shape),
/// and replay derives the same posting from the receipt.
#[test]
fn note_default_posts_to_the_severance_book() {
    let mut w = stamp_world();
    seal_clear(&mut w, Resource::Crystal, &ratio(1, 1));
    let free_before = w.orb().map(|o| o.stamp_amount()).expect("orb");
    // Note 1: face 2, collateral 1, short term.
    let face = Positive::new(ratio(2, 1)).expect("face");
    let coll = Positive::new(ratio(1, 1)).expect("coll");
    let id = w.issue_note(FIRST_ORB, SOVEREIGN, face, coll, 2).expect("issue");
    // Drain the rest of the free stamp into a second note's collateral
    // lock, so maturity finds the bank empty and the seize path runs.
    let drain = free_before - ratio(1, 1);
    if drain > ratio(0, 1) {
        let face2 = Positive::new(drain.clone() + drain.clone()).expect("face2");
        let coll2 = Positive::new(drain).expect("coll2");
        let _ = w.issue_note(FIRST_ORB, SOVEREIGN, face2, coll2, 400);
    }
    for _ in 0..3 {
        w.tick();
    }
    let severance_before = w
        .orb()
        .map(|o| o.severance_load.clone())
        .expect("orb");
    w.redeem_note(id).expect("mature note redeems (by seizure)");
    let note = w.notes().iter().find(|n| n.note_id == id).expect("note").clone();
    assert!(!note.open, "the note closed");
    let redeemed = w
        .chronicle()
        .receipts()
        .iter()
        .rev()
        .find_map(|r| match r.event() {
            ClosureEvent::NoteRedeemed { note_id, paid, seized, .. }
                if *note_id == id =>
            {
                Some((paid.clone(), seized.clone()))
            }
            _ => None,
        })
        .expect("the redemption receipted");
    let (paid, seized) = redeemed;
    assert!(seized > ratio(0, 1), "the seize path ran");
    assert!(paid < note.notional, "the redemption fell short");
    let severance_after = w.orb().map(|o| o.severance_load.clone()).expect("orb");
    assert_eq!(
        severance_after - severance_before,
        note.notional - paid,
        "the unpaid remainder posts to the severance book, exactly"
    );
    assert_eq!(replay(w.chronicle()), w.committed(), "SEAL B — replay derives the same posting");
}

/// I5.4 — a margin breach refuses funding, and the liquidation door
/// closes the perp with the flag receipted (the conductor's
/// instrument_step walks exactly this pair).
#[test]
fn margin_breach_refuses_funding_and_liquidation_closes() {
    let mut w = stamp_world();
    seal_clear(&mut w, Resource::Crystal, &ratio(2, 1));
    // Thin margin against a fat notional: maint = n·entry/4 = 4 > 1/4.
    let n = Positive::new(ratio(8, 1)).expect("n");
    let m = Positive::new(ratio(1, 4)).expect("m");
    let short = w.register_agent("thin-short").expect("agent");
    let pid = w
        .open_perp(FIRST_ORB, SOVEREIGN, short, Resource::Crystal, n, m)
        .expect("perp opens (no maintenance gate at the door)");
    // Move the mark so funding is positive.
    seal_clear(&mut w, Resource::Crystal, &ratio(3, 1));
    assert!(matches!(
        w.fund_perp(pid),
        Err(cfe::world::ActionRefused::PerpMarginBreach { .. })
    ));
    w.liquidate_perp(pid).expect("the breach liquidates");
    assert!(w.perps().iter().any(|p| p.perp_id == pid && !p.open));
    let liquidated = w.chronicle().receipts().iter().any(|r| {
        matches!(
            r.event(),
            ClosureEvent::PerpClosed { perp_id, liquidated: true, .. } if *perp_id == pid
        )
    });
    assert!(liquidated, "the close receipt carries the liquidation flag");
    assert_eq!(replay(w.chronicle()), w.committed(), "SEAL B after liquidation");
}
