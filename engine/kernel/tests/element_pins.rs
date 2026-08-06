//! THE ELEMENT LAW — the element ledger enters conservation with
//! strict symmetry to the banked ledger (operator ruling 2026-08-01).
//! Red-first.
//!
//! An assay mints `richness · elapsed` into a vein's element — sign-
//! guarded (amount ≥ 0) and bounded by the PRIOR vein's richness and
//! last_assay, both already committed state, so the bound is
//! deterministic from (prior, step). Every other deed holds every
//! element key still: an element total changed under a deed that
//! declares no element move refuses, exactly as a smuggled bank does.

use cfe::dop::{ClosureEvent, ColonyOrb};
use cfe::ipsal::Entity;
use cfe::ratio::{ratio, Ratio};
use cfe::world::{Committed, Prospected, World, SOVEREIGN};
use kernel::{admits, Step, Refused};

const ELEMENT: [u8; 32] = [3u8; 32];

/// A one-orb state with an opened vein: element ELEMENT, richness 2,
/// last assayed at stamp 1. Sovereign seated.
fn with_vein(richness: (i64, i64), last_assay: u64) -> Committed {
    let mut committed = World::new().committed();
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
        richness: ratio(richness.0, richness.1),
        last_assay,
    }];
    committed
}

fn banked_element(state: &Committed, element: [u8; 32], delta: Ratio) -> Committed {
    let mut next = state.clone();
    next.orbs[0].bank_element(element, delta);
    next
}

/// ECO-1 (#40): the assay's FULL effect on the vein book — the span
/// anchor advances to the deed's stamp, and the declared after-richness
/// lands. The sealed form (`None`) holds richness where it stood.
fn worked_vein(state: &Committed, stamp: u64, richness: Option<Ratio>) -> Committed {
    let mut next = state.clone();
    if let Some(vein) = next.prospects.iter_mut().find(|vein| vein.element == ELEMENT) {
        vein.last_assay = stamp;
        if let Some(after) = richness {
            vein.richness = after;
        }
    }
    next
}

fn assay(amount: (i64, i64), stamp: u64) -> Step {
    Step {
        event: ClosureEvent::AssayStruck {
            orb_id: 1,
            element: ELEMENT,
            amount: ratio(amount.0, amount.1),
            // The SEALED pre-ecology form: declares no after-richness,
            // so the vein's flow holds still and only the bound is
            // judged. Chains cut this way verify forever this way.
            richness: None,
        },
        actor: SOVEREIGN,
        stamp,
    }
}

#[test]
fn an_honest_assay_mints_exactly_the_vein_yield() {
    // richness 2 × elapsed (4 − 1 = 3) = 6, minted into the element.
    //
    // WITNESS TRUTH-UPDATE (ECO-1, #40): `next` now also carries the
    // advanced span anchor. It always SHOULD have — live and replay
    // both advance it — but until the vein book became judged state
    // the gate could not see it, and an assay that banked its element
    // while leaving the anchor where it stood left the same span
    // drawable again on the next step. That is now VeinBookBroken.
    let prior = with_vein((2, 1), 1);
    let next = worked_vein(&banked_element(&prior, ELEMENT, ratio(6, 1)), 4, None);
    assert_eq!(admits(&prior, &next, &assay((6, 1), 4)), Ok(()));
}

#[test]
fn an_assay_that_leaves_the_span_anchor_unmoved_refuses() {
    // THE HOLE THE BOOK LAW CLOSES: the element ledger balanced, the
    // bound honoured — and the vein left exactly as it was, so the
    // same three ticks of span stand ready to be drawn again.
    let prior = with_vein((2, 1), 1);
    let unmoved = banked_element(&prior, ELEMENT, ratio(6, 1));
    assert_eq!(
        admits(&prior, &unmoved, &assay((6, 1), 4)),
        Err(Refused::VeinBookBroken { at: 0 }),
        "an assay moves its vein's anchor or it does not stand"
    );
}

#[test]
fn an_assay_past_the_vein_yield_refuses_bounded() {
    // The same vein yields 6 at stamp 4; a claim of 7 refuses with the
    // bound and the claim carried whole.
    let prior = with_vein((2, 1), 1);
    let greedy = banked_element(&prior, ELEMENT, ratio(7, 1));
    assert_eq!(
        admits(&prior, &greedy, &assay((7, 1), 4)),
        Err(Refused::AssayExceedsVein {
            bound: Box::new(ratio(6, 1)),
            claimed: Box::new(ratio(7, 1)),
        }),
        "value enters the element ledger only up to the vein's yield"
    );
}

#[test]
fn a_negative_assay_refuses_by_sign() {
    let prior = with_vein((2, 1), 1);
    let inverted = banked_element(&prior, ELEMENT, ratio(-6, 1));
    assert_eq!(
        admits(&prior, &inverted, &assay((-6, 1), 4)),
        Err(Refused::ElementNotPositive {
            element: ELEMENT,
            amount: Box::new(ratio(-6, 1)),
        }),
        "an element mint is non-negative by declaration"
    );
}

#[test]
fn an_assay_with_no_vein_yields_nothing() {
    // No prior vein for this element: the bound is zero, so any
    // positive claim refuses.
    let mut prior = with_vein((2, 1), 1);
    prior.prospects.clear();
    let claimed = banked_element(&prior, ELEMENT, ratio(6, 1));
    assert_eq!(
        admits(&prior, &claimed, &assay((6, 1), 4)),
        Err(Refused::AssayExceedsVein {
            bound: Box::new(ratio(0, 1)),
            claimed: Box::new(ratio(6, 1)),
        }),
        "no vein, no yield"
    );
}

/// ECO-1 (#40): the vein's NATURAL flow for ELEMENT under a G star —
/// the hash's own reading, the number the wear law caps against.
fn natural() -> Ratio {
    let (_, _, richness) = cfe::dop::element_properties(&ELEMENT, cfe::thermal::StarClass::G);
    richness
}

/// A state whose sun stands (the wear law reads the star to derive the
/// spring's nature) and whose orb carries `regenerators` regenerators.
fn under_a_sun(richness: (i64, i64), last_assay: u64, regenerators: usize) -> Committed {
    let mut committed = with_vein(richness, last_assay);
    committed.star = Some(cfe::thermal::StarClass::G);
    committed.orbs[0].zones = (0..regenerators)
        .map(|tile| cfe::dop::Zone { tile: tile as u16, kind: cfe::dop::ZoneKind::Regenerator })
        .collect();
    committed
}

/// The full ecological assay: the element banked, the anchor advanced,
/// and the declared after-richness landed on the vein.
fn worked(prior: &Committed, amount: Ratio, stamp: u64, after: Ratio) -> (Committed, Step) {
    let mut next = banked_element(prior, ELEMENT, amount.clone());
    if let Some(vein) = next.prospects.iter_mut().find(|vein| vein.element == ELEMENT) {
        vein.last_assay = stamp;
        vein.richness = after.clone();
    }
    let step = Step {
        event: ClosureEvent::AssayStruck {
            orb_id: 1,
            element: ELEMENT,
            amount,
            richness: Some(after),
        },
        actor: SOVEREIGN,
        stamp,
    };
    (next, step)
}

#[test]
fn the_gate_re_derives_the_wear_and_admits_only_that_number() {
    // The vein runs at its nature (1/2 under G) and is drawn for its
    // whole three-tick span. The gate re-derives the wear by CALLING
    // the emitter's one home, so the ONLY admissible after-richness is
    // the one the verb would have settled.
    let natural = natural();
    assert_eq!(natural, ratio(1, 2), "the hash's reading, pinned");
    let prior = under_a_sun((1, 2), 1, 0);
    let drawn = ratio(3, 2);
    let expected = cfe::dop::vein_wear(&natural, &natural, &drawn, 0, 3);
    assert_eq!(expected, ratio(1, 8), "a draw of three times the nature guts the spring");

    let (next, step) = worked(&prior, drawn.clone(), 4, expected.clone());
    assert_eq!(admits(&prior, &next, &step), Ok(()));

    // A gentler wear than the law allows — the vein flattered by a
    // tick — refuses with both numbers whole.
    let flattered = ratio(1, 4);
    let (next, step) = worked(&prior, drawn, 4, flattered.clone());
    assert_eq!(
        admits(&prior, &next, &step),
        Err(Refused::VeinWearMisdeclared {
            expected: Box::new(expected),
            declared: Box::new(flattered),
        }),
        "the emitter cannot quote the ground one way and work it another"
    );
}

#[test]
fn the_gate_reads_the_regenerators_standing_on_the_orb() {
    // The SAME draw over the SAME span, judged against an orb that
    // tends its ground: the gate reads the regenerators from prior
    // state, so the number it admits rises with them — and a
    // declaration that claims the tended recovery on BARE ground
    // refuses.
    let natural = natural();
    let drawn = ratio(3, 2);
    let bare = under_a_sun((1, 2), 1, 0);
    let tended = under_a_sun((1, 2), 1, 1);
    let restored = cfe::dop::vein_wear(&natural, &natural, &drawn, 1, 3);
    assert_eq!(restored, ratio(37, 80), "one regenerator closes most of the gap");
    assert!(restored > ratio(1, 8), "and stands above the untended wear");

    let (next, step) = worked(&tended, drawn.clone(), 4, restored.clone());
    assert_eq!(admits(&tended, &next, &step), Ok(()));

    let (next, step) = worked(&bare, drawn, 4, restored.clone());
    assert_eq!(
        admits(&bare, &next, &step),
        Err(Refused::VeinWearMisdeclared {
            expected: Box::new(ratio(1, 8)),
            declared: Box::new(restored),
        }),
        "recovery is earned by standing regenerators, never by declaration"
    );
}

#[test]
fn a_wear_declaration_with_nothing_to_derive_from_refuses() {
    // No star: the spring's nature is underivable, so no after-richness
    // is lawful. (The verb cannot reach this — prospecting requires a
    // commissioned sun — but the gate judges states, not paths.)
    let mut starless = under_a_sun((1, 2), 1, 0);
    starless.star = None;
    let (next, step) = worked(&starless, ratio(3, 2), 4, ratio(1, 8));
    assert_eq!(
        admits(&starless, &next, &step),
        Err(Refused::VeinWearMisdeclared {
            expected: Box::new(ratio(0, 1)),
            declared: Box::new(ratio(1, 8)),
        }),
        "no sun, no nature, no lawful wear"
    );
}

#[test]
fn a_prospect_pushes_exactly_its_receipted_vein() {
    // The book law's other half: the mint deed's effect is declared by
    // its own receipt, and a book that grew some other vein refuses.
    let prior = under_a_sun((1, 2), 1, 0);
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
    next.prospects.push(cfe::world::Prospected {
        orb: 1,
        stratum: 7,
        element: [11u8; 32],
        richness: ratio(3, 4),
        last_assay: 9,
    });
    assert_eq!(admits(&prior, &next, &step), Ok(()));

    let mut smuggled = prior.clone();
    smuggled.prospects.push(cfe::world::Prospected {
        orb: 1,
        stratum: 7,
        element: [11u8; 32],
        // A richer vein than the receipt struck.
        richness: ratio(9, 1),
        last_assay: 9,
    });
    assert_eq!(
        admits(&prior, &smuggled, &step),
        Err(Refused::VeinBookBroken { at: 1 }),
        "the ground a world stands on is what its receipts struck"
    );
}

#[test]
fn a_neutral_deed_holds_the_vein_book_still() {
    let prior = under_a_sun((1, 2), 1, 0);
    let mut enriched = prior.clone();
    enriched.prospects[0].richness = ratio(9, 1);
    let neutral = Step {
        event: ClosureEvent::OverheatCleared { orb_id: 1 },
        actor: 999,
        stamp: 4,
    };
    assert_eq!(
        admits(&prior, &enriched, &neutral),
        Err(Refused::VeinBookBroken { at: 0 }),
        "no deed enriches a vein undeclared"
    );
}

#[test]
fn an_element_change_under_a_neutral_deed_refuses() {
    // Zero-invariance: a covered deed that declares no element move
    // must leave every element key still. A smuggled element mint
    // under a neutral deed refuses, exactly as a smuggled bank does.
    let prior = with_vein((2, 1), 1);
    let smuggled = banked_element(&prior, ELEMENT, ratio(6, 1));
    let neutral = Step {
        event: ClosureEvent::OverheatCleared { orb_id: 1 },
        actor: 999,
        stamp: 4,
    };
    assert_eq!(
        admits(&prior, &smuggled, &neutral),
        Err(Refused::ElementBroken {
            element: ELEMENT,
            expected: Box::new(ratio(0, 1)),
            actual: Box::new(ratio(6, 1)),
        }),
        "no deed mints an element undeclared"
    );
}
