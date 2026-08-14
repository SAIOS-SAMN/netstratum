//! CONSILIENCE (38 §5, tag 177) — the chain's side of the shared axiom:
//! the door's refusals, idempotence, and SEAL B (replay).

use cfe::dop::ClosureEvent;
use cfe::world::{ActionRefused, World};

fn founded() -> World {
    let mut w = World::new();
    w.found("axiom-hold").expect("founds");
    w
}

#[test]
fn the_axiom_records_once_and_a_lone_line_is_no_consilience() {
    let mut w = founded();
    let signature = [5u8; 32];

    // A consilience is agreement BETWEEN lines: a mass below two is one
    // line agreeing with itself, and no measurement.
    assert_eq!(
        w.crystallize_knowledge_as(cfe::world::SOVEREIGN, signature, 1, 0),
        Err(ActionRefused::ConsilienceAlone { mass: 1 })
    );

    // The lawful crystallization lands.
    w.crystallize_knowledge_as(cfe::world::SOVEREIGN, signature, 3, 2)
        .expect("the axiom records");
    let recorded = w
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(
            &r.event,
            ClosureEvent::KnowledgeConverged { signature: held, mass: 3, hardness: 2 } if *held == signature
        ))
        .count();
    assert_eq!(recorded, 1, "one leg, recorded");

    // Convergence is recorded once per line — a re-record refuses.
    assert_eq!(
        w.crystallize_knowledge_as(cfe::world::SOVEREIGN, signature, 4, 1),
        Err(ActionRefused::KnowledgeKnown { signature })
    );

    // A DIFFERENT shared truth is its own axiom and records freely.
    w.crystallize_knowledge_as(cfe::world::SOVEREIGN, [6u8; 32], 2, 1)
        .expect("a distinct axiom records");

    // SEAL B: the receipts replay to the same committed state — a
    // knowledge axiom moves nothing bankable, so replay carries nothing.
    assert_eq!(cfe::world::replay(w.chronicle()), w.committed());
}
