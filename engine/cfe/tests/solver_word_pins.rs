//! NOVELTY-GATE — the chain's side of the mint (tag 166): the door's
//! refusals, the fold, and SEAL B.

use cfe::world::{ActionRefused, World};

fn founded() -> World {
    let mut w = World::new();
    w.found("word-hold").expect("founds");
    w
}

#[test]
fn the_door_holds_and_the_fold_reads() {
    let mut w = founded();
    let orbit = [4u8; 32];
    let expansion = vec![1u8, 4u8];
    let cited = vec![[1u8; 32], [2u8; 32], [3u8; 32]];
    // Nothing enters unearned.
    assert_eq!(
        w.mint_solver_word_as(cfe::world::SOVEREIGN, orbit, Vec::new(), cited.clone()),
        Err(ActionRefused::SolverWordEmpty)
    );
    // COMM-1 — CONGRUENCE THRESHOLD: one vantage is not yet agreement (refuses);
    // two independent vantages converging on the orbit IS (mints).
    assert_eq!(
        w.mint_solver_word_as(cfe::world::SOVEREIGN, orbit, expansion.clone(), vec![[1u8; 32]]),
        Err(ActionRefused::SolverWordUnconfirmed { confirms: 1 })
    );
    // The lawful mint lands and folds — two vantages agree.
    w.mint_solver_word_as(cfe::world::SOVEREIGN, orbit, expansion.clone(), cited.clone())
        .expect("the word mints");
    let folded = cfe::vocabulary::solver_words(w.chronicle());
    assert_eq!(folded, vec![(orbit, expansion.clone(), cited.clone())]);
    // A re-spelling can never re-mint.
    assert_eq!(
        w.mint_solver_word_as(cfe::world::SOVEREIGN, orbit, expansion, cited),
        Err(ActionRefused::SolverWordKnown { orbit })
    );
    // SEAL B: the receipt replays; the fold is the register.
    assert_eq!(cfe::world::replay(w.chronicle()), w.committed());
}
