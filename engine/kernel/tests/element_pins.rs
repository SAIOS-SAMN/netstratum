//! THE ELEMENT LAW — the element ledger enters conservation with strict
//! symmetry to the banked ledger (operator ruling 2026-08-01).
//!
//! ONT-2: an assay mints the vein's DERIVED flow into its element — sign-
//! guarded (amount ≥ 0) and bounded by the geometry minus the PRIOR
//! extraction integral, both re-derivable from (prior, step), so the bound is
//! deterministic and needs no stored richness scalar. The vein book holds
//! still: a prospect pushes a FRESH vein (extracted nothing), an assay grows
//! that vein's integral by exactly the banked amount, and every other deed
//! leaves the book untouched — an undeclared change refuses.

use cfe::dop::{ClosureEvent, ColonyOrb};
use cfe::ipsal::Entity;
use cfe::ratio::{ratio, Ratio};
use cfe::world::{Committed, Prospected, World, SOVEREIGN};
use kernel::{admits, Refused, Step};

const ELEMENT: [u8; 32] = [3u8; 32];

/// ELEMENT's geometric endowment (`natural`) under a G star — the hash's own
/// reading, the number the extraction bound derives against. Pinned: 1/2.
fn natural() -> Ratio {
    cfe::dop::element_properties(&ELEMENT, cfe::thermal::StarClass::G).2
}

/// A one-orb state under a G sun with a vein at ELEMENT that has EXTRACTED
/// `extracted` so far, last assayed at `last_assay`. Sovereign seated. The
/// star stands so the bound (natural − extracted, decayed over the span) is
/// derivable — the vein carries no richness scalar.
fn with_vein(extracted: (i64, i64), last_assay: u64) -> Committed {
    let mut committed = World::new().committed();
    committed.star = Some(cfe::thermal::StarClass::G);
    committed.orbs = vec![ColonyOrb { orb_id: 1, name: "site".into(), ..Default::default() }];
    committed.entities = vec![Entity {
        entity_id: SOVEREIGN,
        handle: "sovereign".into(),
        principal: None,
        grants: vec![],
    }];
    committed.prospects = vec![Prospected {
        orb: 1,
        stratum: 0,
        element: ELEMENT,
        extracted: ratio(extracted.0, extracted.1),
        last_assay,
    }];
    committed
}

/// The vein's derived flow over the span to `stamp` — the exact bound the
/// gate re-derives (call-not-copy, the one home `vein_flow`).
fn flow_to(prior: &Committed, stamp: u64) -> Ratio {
    prior
        .prospects
        .iter()
        .find(|v| v.element == ELEMENT)
        .and_then(|v| {
            let class = prior.star?;
            let n = cfe::dop::element_properties(&ELEMENT, class).2;
            let elapsed = stamp.saturating_sub(v.last_assay);
            let richness = cfe::dop::vein_richness(&n, &v.extracted);
            Some(cfe::dop::vein_flow(&richness, &n, elapsed))
        })
        .unwrap_or_else(|| ratio(0, 1))
}

fn banked_element(state: &Committed, element: [u8; 32], delta: Ratio) -> Committed {
    let mut next = state.clone();
    next.orbs[0].bank_element(element, delta);
    next
}

fn assay(amount: Ratio, stamp: u64) -> Step {
    Step {
        event: ClosureEvent::AssayStruck { orb_id: 1, element: ELEMENT, amount, richness: None },
        actor: SOVEREIGN,
        stamp,
    }
}

/// The full ONT-2 assay effect on the book: the element banks, the span
/// anchor advances, and the extraction integral grows by exactly the amount.
fn worked(prior: &Committed, amount: Ratio, stamp: u64) -> (Committed, Step) {
    let mut next = banked_element(prior, ELEMENT, amount.clone());
    if let Some(vein) = next.prospects.iter_mut().find(|v| v.element == ELEMENT) {
        vein.last_assay = stamp;
        vein.extracted += amount.clone();
    }
    (next, assay(amount, stamp))
}

#[test]
fn an_honest_assay_mints_exactly_the_vein_flow() {
    // A fresh vein (extracted 0) under G runs at natural 1/2. Over a 3-tick
    // span the derived flow is min(1/2·1/2·3, 1/2) = 1/2 (the span would take
    // more than stands, so it caps at the remaining richness). The gate
    // re-derives that exact number; banking it, advancing the anchor and
    // growing the integral admits.
    assert_eq!(natural(), ratio(1, 2), "the hash's reading, pinned");
    let prior = with_vein((0, 1), 1);
    let flow = flow_to(&prior, 4);
    assert_eq!(flow, ratio(1, 2), "a fresh vein pays its whole richness this span");
    let (next, step) = worked(&prior, flow, 4);
    assert_eq!(admits(&prior, &next, &step), Ok(()));
}

#[test]
fn an_assay_that_leaves_the_span_anchor_unmoved_refuses() {
    // The element ledger balanced and the bound honoured, but the vein left
    // as it was, so the span stands drawable again → the book law refuses.
    let prior = with_vein((0, 1), 1);
    let flow = flow_to(&prior, 4);
    let unmoved = banked_element(&prior, ELEMENT, flow.clone());
    assert_eq!(
        admits(&prior, &unmoved, &assay(flow, 4)),
        Err(Refused::VeinBookBroken { at: 0 }),
        "an assay moves its vein's anchor and integral, or it does not stand"
    );
}

#[test]
fn an_assay_past_the_vein_flow_refuses_bounded() {
    let prior = with_vein((0, 1), 1);
    let flow = flow_to(&prior, 4);
    let over = flow.clone() + ratio(1, 1);
    let greedy = banked_element(&prior, ELEMENT, over.clone());
    assert_eq!(
        admits(&prior, &greedy, &assay(over.clone(), 4)),
        Err(Refused::AssayExceedsVein { bound: Box::new(flow), claimed: Box::new(over) }),
        "value enters the element ledger only up to the vein's derived flow"
    );
}

#[test]
fn a_negative_assay_refuses_by_sign() {
    let prior = with_vein((0, 1), 1);
    let inverted = banked_element(&prior, ELEMENT, ratio(-6, 1));
    assert_eq!(
        admits(&prior, &inverted, &assay(ratio(-6, 1), 4)),
        Err(Refused::ElementNotPositive { element: ELEMENT, amount: Box::new(ratio(-6, 1)) }),
        "an element mint is non-negative by declaration"
    );
}

#[test]
fn an_assay_with_no_vein_yields_nothing() {
    let mut prior = with_vein((0, 1), 1);
    prior.prospects.clear();
    let claimed = banked_element(&prior, ELEMENT, ratio(1, 1));
    assert_eq!(
        admits(&prior, &claimed, &assay(ratio(1, 1), 4)),
        Err(Refused::AssayExceedsVein { bound: Box::new(ratio(0, 1)), claimed: Box::new(ratio(1, 1)) }),
        "no vein, no yield"
    );
}

#[test]
fn an_assay_with_no_star_yields_nothing() {
    // ONT-2: the vein's nature derives from the star's spectrum; with no sun
    // the endowment is underivable, so the bound is zero and any positive
    // claim refuses. (The verb cannot reach this — prospecting requires a
    // sun — but the gate judges states, not paths.)
    let mut starless = with_vein((0, 1), 1);
    starless.star = None;
    let claimed = banked_element(&starless, ELEMENT, ratio(1, 1));
    assert_eq!(
        admits(&starless, &claimed, &assay(ratio(1, 1), 4)),
        Err(Refused::AssayExceedsVein { bound: Box::new(ratio(0, 1)), claimed: Box::new(ratio(1, 1)) }),
        "no sun, no nature, no yield"
    );
}

#[test]
fn the_integral_grows_by_exactly_the_banked_amount() {
    // A partly-worked vein (already extracted 1/8) is drawn again: the gate
    // admits only a next whose extraction integral grew by EXACTLY the banked
    // amount. A book that grows it by any other step — the old 'after-richness'
    // drift — is VeinBookBroken.
    let prior = with_vein((1, 8), 1);
    let flow = flow_to(&prior, 4);
    let (next, step) = worked(&prior, flow.clone(), 4);
    assert_eq!(admits(&prior, &next, &step), Ok(()));

    let mut drifted = banked_element(&prior, ELEMENT, flow.clone());
    if let Some(v) = drifted.prospects.iter_mut().find(|v| v.element == ELEMENT) {
        v.last_assay = 4;
        v.extracted += flow.clone() + ratio(1, 100);
    }
    assert_eq!(
        admits(&prior, &drifted, &assay(flow, 4)),
        Err(Refused::VeinBookBroken { at: 0 }),
        "the integral grows by the banked amount, never a declared drift"
    );
}

fn stmine(eaten: Ratio, stamp: u64) -> Step {
    Step {
        event: ClosureEvent::StapleMined { orb_id: 1, element: ELEMENT, eaten },
        actor: SOVEREIGN,
        stamp,
    }
}

/// The full ONT-2b staple-mine effect on the book: NO element banks (the food
/// goes vein → mouths, the bank bypassed), the span anchor advances, and the
/// extraction integral grows by exactly `eaten`.
fn eaten_from_vein(prior: &Committed, eaten: Ratio, stamp: u64) -> (Committed, Step) {
    let mut next = prior.clone();
    if let Some(vein) = next.prospects.iter_mut().find(|v| v.element == ELEMENT) {
        vein.last_assay = stamp;
        vein.extracted += eaten.clone();
    }
    (next, stmine(eaten, stamp))
}

#[test]
fn eating_mines_the_whole_vein_past_the_geometric_flow() {
    // ONT-2b: a DEMAND-driven mine is bounded by the vein's REMAINING RICHNESS
    // (natural − extracted), NOT the per-span geometric flow — a hungry band
    // may draw its whole larder in one pull. A fresh vein under G holds
    // richness 1/2; over a ONE-tick span the geometric flow is only 1/4, yet
    // the mine may take the whole 1/2. And NO element banks — the food is eaten.
    let prior = with_vein((0, 1), 3);
    assert_eq!(flow_to(&prior, 4), ratio(1, 4), "the geometric throttle this span");
    let (next, step) = eaten_from_vein(&prior, ratio(1, 2), 4);
    assert_eq!(admits(&prior, &next, &step), Ok(()), "eating draws up to the whole richness");
    // The element ledger held STILL — a delta-0 move (food went vein → mouths).
    assert_eq!(next.orbs[0].elements, prior.orbs[0].elements, "the bank is bypassed for food");
}

#[test]
fn a_mine_past_the_remaining_richness_refuses() {
    // Geometry BOUNDS the demand: the vein feeds no more than it holds.
    let prior = with_vein((0, 1), 3);
    let over = ratio(1, 2) + ratio(1, 100);
    let (next, step) = eaten_from_vein(&prior, over.clone(), 4);
    assert_eq!(
        admits(&prior, &next, &step),
        Err(Refused::AssayExceedsVein { bound: Box::new(ratio(1, 2)), claimed: Box::new(over) }),
        "no vein feeds more than its remaining richness"
    );
}

#[test]
fn a_staple_mine_that_banks_the_element_refuses() {
    // The mine is DELTA-0 on the element ledger — the food is consumed, never
    // banked. A next that ALSO banks the element declares an undeclared mint,
    // so the conservation law refuses it.
    let prior = with_vein((0, 1), 3);
    let eaten = ratio(1, 4);
    let mut banked = prior.clone();
    if let Some(v) = banked.prospects.iter_mut().find(|v| v.element == ELEMENT) {
        v.last_assay = 4;
        v.extracted += eaten.clone();
    }
    banked.orbs[0].bank_element(ELEMENT, eaten.clone());
    assert!(
        matches!(
            admits(&prior, &banked, &stmine(eaten, 4)),
            Err(Refused::ElementBroken { .. })
        ),
        "eating mines the vein but banks nothing — the element ledger holds still"
    );
}

#[test]
fn a_prospect_pushes_a_fresh_vein() {
    let prior = with_vein((0, 1), 1);
    let struck = ClosureEvent::ProspectStruck {
        orb_id: 1,
        stratum: 7,
        element: [11u8; 32],
        family: 1,
        tier: 2,
        richness: ratio(3, 4),
        name: "keening lode".into(),
    };
    let step = Step { event: struck, actor: SOVEREIGN, stamp: 9 };
    let mut next = prior.clone();
    next.prospects.push(Prospected {
        orb: 1,
        stratum: 7,
        element: [11u8; 32],
        extracted: ratio(0, 1),
        last_assay: 9,
    });
    assert_eq!(admits(&prior, &next, &step), Ok(()));

    // A vein pushed already part-drained — the ground a world stands on is
    // what its receipts struck, and a prospect strikes a FRESH vein.
    let mut smuggled = prior.clone();
    smuggled.prospects.push(Prospected {
        orb: 1,
        stratum: 7,
        element: [11u8; 32],
        extracted: ratio(9, 1),
        last_assay: 9,
    });
    assert_eq!(
        admits(&prior, &smuggled, &step),
        Err(Refused::VeinBookBroken { at: 1 }),
        "a prospect pushes a fresh vein — extracted nothing"
    );
}

#[test]
fn a_neutral_deed_holds_the_vein_book_still() {
    let prior = with_vein((0, 1), 1);
    let mut drained = prior.clone();
    drained.prospects[0].extracted = ratio(9, 1);
    let neutral = Step {
        event: ClosureEvent::OverheatCleared { orb_id: 1 },
        actor: 999,
        stamp: 4,
    };
    assert_eq!(
        admits(&prior, &drained, &neutral),
        Err(Refused::VeinBookBroken { at: 0 }),
        "no deed drains a vein undeclared"
    );
}
