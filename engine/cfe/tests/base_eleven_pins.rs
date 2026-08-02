//! THE UPLIFT TO BASE-11 (operator ruling 2026-08-02).
//!
//! The stack was base-7 and full — conditions `0..=6`, the crown gated
//! on the solar leap plus all five. It is now ELEVEN, in two tiers:
//!
//! ```text
//!     0        the solar leap
//!     1..=5    the five crystallized conditions
//!     6        the gifted crown — completes the BASE
//!     7..=10   THE UPLIFT TIER — one node per chartered work
//! ```
//!
//! **The crown's gate does not move.** That was the exact risk of
//! changing the base, and these pins are mostly here to hold it still.

use cfe::literature::WORKS;
use cfe::ratio::ratio;
use cfe::world::survey::{FIRST_WORK, LAST_CONDITION};
use cfe::world::World;

fn literate_world() -> (World, u64) {
    let mut world = World::default();
    world.found("home").expect("founds");
    let orb = world.orb().expect("founded").orb_id;
    let kind = world.mint_book(orb, "Ledger of Deeds").expect("mints");
    world
        .post_book(orb, kind, ratio(1, 1), ratio(0, 1))
        .expect("posts");
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
    assert_eq!(LAST_CONDITION, 10);
    assert_eq!(FIRST_WORK, 7);
    let seen: Vec<u8> = sky.iter().map(|(condition, ..)| *condition).collect();
    assert_eq!(seen, (0u8..=10).collect::<Vec<_>>(), "no gaps, in order");
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
    let valor = FIRST_WORK; // the charter's order: VALOR first.
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
    for condition in 0..FIRST_WORK {
        assert_eq!(World::work_at(condition), None, "{condition} is base tier");
    }
    let mapped: Vec<&str> = (FIRST_WORK..=LAST_CONDITION)
        .map(|condition| World::work_at(condition).expect("a work"))
        .collect();
    assert_eq!(mapped, WORKS.to_vec(), "the charter's order, in order");
    assert_eq!(World::work_at(LAST_CONDITION + 1), None, "nothing above");
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
    assert!((FIRST_WORK..=LAST_CONDITION).all(|c| world.work_claimed(orb, c)));
    // ...and the full stack still refuses, because the BASE is not done.
    assert!(
        !world.base_eleven_complete(orb),
        "deeds do not substitute for the base tier"
    );
    assert!(!world.base_seven_complete());
}
