//! THE UPLIFT TO BASE-11 (operator ruling 2026-08-02).
//!
//! The stack was base-7 and full — conditions `0..=6`, the crown gated
//! on the solar leap plus all five. It is now ELEVEN, in two tiers:
//!
//! ```text
//!     0        the solar leap
//!     1..=5    the five crystallized conditions
//!     6        the gifted crown — completes the BASE
//!     7        NOT A SKY NODE — PRACTICE-1's re-emergence lesson
//!     8..=11   THE UPLIFT TIER — one node per chartered work
//! ```
//!
//! **The crown's gate does not move.** That was the exact risk of
//! changing the base, and these pins are mostly here to hold it still.

use cfe::literature::WORKS;
use cfe::ratio::ratio;
use cfe::world::survey::{first_work, last_condition};
use cfe::world::World;

fn literate_world() -> (World, u64) {
    let mut world = World::default();
    world.found("home").expect("founds");
    let orb = world.orb().expect("founded").orb_id;
    let kind = world.mint_book(orb, "Ledger of Deeds").expect("mints");
    world
        .post_book(orb, kind, ratio(1, 1), ratio(0, 1))
        .expect("posts");
    // PRACTICE COSTS TIME (GROWTH-1). A second posting after real chain
    // distance is what makes this line literate; both at one stamp
    // would be the leap this fixture used to encode.
    for _ in 0..4 {
        world.tick();
    }
    world
        .post_book(orb, kind, ratio(1, 1), ratio(1, 1))
        .expect("posts again");
    (world, orb)
}

fn a_receipt(world: &World) -> [u8; 32] {
    world.chronicle().receipts().first().expect("a receipt").own.0
}

/// The stack is eleven wide, and every node reports.
#[test]
fn the_stack_is_base_eleven() {
    let (world, orb) = literate_world();
    let sky = world.constellation(orb);
    assert_eq!(sky.len(), 11, "eleven nodes");
    assert_eq!(last_condition(), 11);
    assert_eq!(first_work(), 8);
    let seen: Vec<u8> = sky.iter().map(|(condition, ..)| *condition).collect();
    // 7 is PRACTICE-1's re-emergence lesson and is NOT a sky node.
    assert_eq!(
        seen,
        vec![0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11],
        "eleven nodes, and 7 is not one of them"
    );
}

/// **THE GATE THAT MUST NOT MOVE.** The crown still waits on `0..=5`
/// and still reports a threshold of six. Widening the base must not
/// silently change what the crown is waiting for.
#[test]
fn the_crowns_gate_did_not_move() {
    let (world, orb) = literate_world();
    let sky = world.constellation(orb);
    let (_, held, progress, threshold) =
        sky.iter().find(|(c, ..)| *c == 6).copied().expect("the crown");
    assert!(!held);
    assert_eq!(threshold, 6, "six earned nodes, exactly as it landed");
    assert_eq!(progress, 0);
    assert!(!world.base_seven_complete(), "unchanged reading");
}

/// Claiming a work must not move the crown one step closer. The uplift
/// tier sits ABOVE the crown, not among the nodes it waits for.
#[test]
fn claiming_a_work_does_not_advance_the_crown() {
    let (mut world, orb) = literate_world();
    let witness = a_receipt(&world);
    let before = world
        .constellation(orb)
        .into_iter()
        .find(|(c, ..)| *c == 6)
        .expect("crown");

    for name in WORKS {
        world.aspire(orb, name).expect("aspires");
    }
    world.claim_work(orb, "VALOR", &[witness]).expect("claims");

    let after = world
        .constellation(orb)
        .into_iter()
        .find(|(c, ..)| *c == 6)
        .expect("crown");
    assert_eq!(before, after, "the crown is untouched by a deed");
    assert!(!world.base_seven_complete());
}

/// **A DEED IS THE NODE.** The uplift tier is not granted into `techs`;
/// a work is held exactly when it has been claimed on evidence.
#[test]
fn an_uplift_node_is_held_by_the_claim_itself() {
    let (mut world, orb) = literate_world();
    let witness = a_receipt(&world);
    let valor = first_work(); // 8 — the charter's order: VALOR first.
    assert_eq!(World::work_at(valor), Some("VALOR"));

    let unheld = world.constellation(orb).into_iter().find(|(c, ..)| *c == valor);
    assert_eq!(unheld, Some((valor, false, 0, 1)));

    world.aspire(orb, "VALOR").expect("aspires");
    let reached = world.constellation(orb).into_iter().find(|(c, ..)| *c == valor);
    assert_eq!(
        reached,
        Some((valor, false, 0, 1)),
        "reaching is not holding — the deed still has to be witnessed"
    );

    world.claim_work(orb, "VALOR", &[witness]).expect("claims");
    let held = world.constellation(orb).into_iter().find(|(c, ..)| *c == valor);
    assert_eq!(held, Some((valor, true, 1, 1)), "the claim IS the node");
}

/// Every uplift node maps to a chartered work, and nothing outside the
/// tier does.
#[test]
fn the_uplift_tier_is_exactly_the_chartered_works() {
    for condition in 0..first_work() {
        assert_eq!(World::work_at(condition), None, "{condition} is base tier");
    }
    let mapped: Vec<&str> = (first_work()..=last_condition())
        .map(|condition| World::work_at(condition).expect("a work"))
        .collect();
    assert_eq!(mapped, WORKS.to_vec(), "the charter's order, in order");
    assert_eq!(World::work_at(last_condition() + 1), None, "nothing above");
}

/// The full stack needs the base complete AND every deed witnessed —
/// and it reads independently of the base-7 reading beneath it.
#[test]
fn base_eleven_needs_the_base_and_every_deed() {
    let (mut world, orb) = literate_world();
    let witness = a_receipt(&world);
    assert!(!world.base_eleven_complete(orb));

    for name in WORKS {
        world.aspire(orb, name).expect("aspires");
        world.claim_work(orb, name, &[witness]).expect("claims");
    }
    // Every deed stands...
    assert!((first_work()..=last_condition()).all(|c| world.work_claimed(orb, c)));
    // ...and the full stack still refuses, because the BASE is not done.
    assert!(
        !world.base_eleven_complete(orb),
        "deeds do not substitute for the base tier"
    );
    assert!(!world.base_seven_complete());
}

/// **THE COLLISION THAT WAS CAUGHT.** `7` is PRACTICE-1's re-emergence
/// lesson — landed law, held in `techs`, and what eases
/// `deposit_window` from 240 to 180. A first cut of the uplift put
/// VALOR there, which would have rendered a held lesson as an unreached
/// work and hidden the real node. The sky must never claim `7`.
#[test]
fn the_sky_never_claims_the_re_emergence_slot() {
    let (world, orb) = literate_world();
    assert_eq!(cfe::world::survey::RE_EMERGENCE, 7);
    assert!(
        world.constellation(orb).iter().all(|(c, ..)| *c != 7),
        "7 rides separately and is not a sky node"
    );
    assert_eq!(World::work_at(7), None, "7 is not a work");
    assert_eq!(World::work_at(8), Some("VALOR"), "the works begin at 8");
}

/// **THE STACK AND THE SOLVER'S FRAME ARE ONE NUMBER** (operator ruling
/// 2026-08-05: *tie spin to orbit … extrapolate down to base-11 per
/// entity*).
///
/// `solver_verify::shell::STACK_NODES` names base-11 from below the
/// `cfe` arrow, where it cannot import this constant. Two copies of a
/// number is the two-truths defect, so the equality is PINNED rather
/// than trusted — this test is what makes them one truth.
#[test]
fn the_solvers_stack_base_is_this_stack_base() {
    assert_eq!(
        i64::from(cfe::world::survey::last_condition()),
        solver_verify::shell::STACK_NODES,
        "the solver frames entities on a different base than the sky — \
         one of the two moved without the other"
    );
    assert_eq!(
        solver_verify::shell::FRAME_PERIOD,
        ratio::TURN_SLOTS * i64::from(cfe::world::survey::last_condition()),
        "the composite frame ring must be the orbital ring times the stack"
    );
}

/// **AND ELEVEN IS DERIVED ON BOTH SIDES OF THE ARROW.** The sky's own
/// parts sum to it, the solver's parts sum to it, and the two sums
/// agree — so there is no `11` left in the tree for anyone to edit in
/// one place.
#[test]
fn both_sides_derive_eleven_from_the_same_parts() {
    use cfe::world::survey as sky;
    use solver_verify::shell as frame;

    use cfe::world::survey::NodeKind;

    // THE SKY COUNTS ITS OWN NODES — nothing here is typed.
    assert_eq!(sky::count_of(NodeKind::Leap), sky::solar_leap(), "one star, one leap");
    assert_eq!(sky::count_of(NodeKind::Crystallized), 5, "the crystallized tier, counted");
    assert_eq!(sky::count_of(NodeKind::Crown), 1, "one gifted crown, counted");
    assert_eq!(sky::count_of(NodeKind::Lesson), 1, "the re-emergence lesson, counted");
    assert_eq!(
        sky::works(),
        cfe::literature::WORKS.len() as u8,
        "the works ARE the literature vocabulary, not a restatement of its size"
    );
    assert_eq!(sky::base_nodes(), 7, "leap + crystallized + crown");
    assert_eq!(sky::last_condition(), 11, "base + works");
    // The base occupies 0..=6 and the lesson rides at 7, so the works
    // begin at base + lesson = 8. The counts ARE the indices consumed —
    // an extra +1 here put VALOR at 9 and left 8 unnamed, which the
    // work_at pin caught.
    assert_eq!(
        sky::first_work(),
        sky::base_nodes() + sky::count_of(NodeKind::Lesson),
        "the works begin past the base AND past the lesson riding between"
    );
    assert_eq!(sky::first_work(), 8, "and that is eight");

    // PART FOR PART, the solver's declared parts match what the sky
    // COUNTS — not merely the totals. A drift names which part moved.
    assert_eq!(i64::from(sky::count_of(NodeKind::Leap)), frame::SOLAR_LEAP);
    assert_eq!(
        i64::from(sky::count_of(NodeKind::Crystallized)),
        frame::CRYSTALLIZED_CONDITIONS
    );
    assert_eq!(i64::from(sky::count_of(NodeKind::Crown)), frame::GIFTED_CROWN);
    assert_eq!(i64::from(sky::works()), frame::CHARTERED_WORKS);
    assert_eq!(i64::from(sky::base_nodes()), frame::BASE_NODES);
    assert_eq!(i64::from(sky::last_condition()), frame::STACK_NODES);
}
