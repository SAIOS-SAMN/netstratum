//! THE KNOWLEDGE MARKET (ARC C, tag 178) — a licensed word enters the
//! library exactly as a minted one, the door is idempotent, and replay
//! carries nothing (the word is a FOLD of the receipt).

use cfe::world::{ActionRefused, World};

fn founded() -> World {
    let mut w = World::new();
    w.found("market-hold").expect("founds");
    w
}

#[test]
fn a_licensed_word_joins_the_library_and_the_door_holds() {
    let mut w = founded();
    let orbit = [7u8; 32];
    let expansion = vec![3u8, 1u8, 4u8];
    let cited = vec![[1u8; 32], [2u8; 32], [3u8; 32]];

    // Nothing enters unearned.
    assert_eq!(
        w.license_knowledge_as(cfe::world::SOVEREIGN, orbit, Vec::new(), cited.clone()),
        Err(ActionRefused::SolverWordEmpty)
    );

    // The lawful license lands, and the FOLD reads it beside a mint —
    // knowledge circulated into this line's own vocabulary.
    w.license_knowledge_as(cfe::world::SOVEREIGN, orbit, expansion.clone(), cited.clone())
        .expect("the word licenses");
    let folded = cfe::vocabulary::solver_words(w.chronicle());
    assert_eq!(folded, vec![(orbit, expansion.clone(), cited.clone())], "the licensed word is in the library");

    // A line never licenses what it already knows.
    assert_eq!(
        w.license_knowledge_as(cfe::world::SOVEREIGN, orbit, expansion.clone(), cited.clone()),
        Err(ActionRefused::SolverWordKnown { orbit })
    );

    // And a word it MINTED is likewise already held — the two sources
    // share one library.
    let minted = [9u8; 32];
    w.mint_solver_word_as(cfe::world::SOVEREIGN, minted, vec![1u8, 2u8], vec![[4u8; 32], [5u8; 32], [6u8; 32]])
        .expect("mints");
    assert_eq!(
        w.license_knowledge_as(cfe::world::SOVEREIGN, minted, vec![1u8, 2u8], vec![[4u8; 32], [5u8; 32], [6u8; 32]]),
        Err(ActionRefused::SolverWordKnown { orbit: minted }),
        "a minted word is not re-licensed"
    );

    // SEAL B: a licensed word is a fold of the receipt — replay carries
    // nothing, the committed state is unchanged.
    assert_eq!(cfe::world::replay(w.chronicle()), w.committed());
}
