//! RCP-2 — the chain's side of the emergent recipe (tag 181): the door's
//! refusals, the built-recipe fold, the wire, and SEAL B. The doing-layer
//! becomes a FOLD of receipts — no `ZoneKind` enum, no committed field.

use cfe::dop::ClosureEvent;
use cfe::ratio::ratio;
use cfe::vocabulary::{recipes_built, BuiltRecipe};
use cfe::world::{ActionRefused, World};

fn founded() -> World {
    let mut w = World::new();
    w.found("recipe-hold").expect("founds");
    w
}

#[test]
fn the_door_holds_and_the_fold_reads() {
    let mut w = founded();
    let orb = w.orb().expect("founded").orb_id;
    let inputs = vec![[7u8; 32], [9u8; 32]];
    let role = cfe::morphogen::STRUCT;
    let y = ratio(3, 1);

    // Nothing enters unearned: an empty draw transforms nothing.
    assert_eq!(
        w.build_recipe_as(cfe::world::SOVEREIGN, orb, Vec::new(), role, y.clone()),
        Err(ActionRefused::RecipeInputsEmpty)
    );
    // A recipe that delivers nothing of its role is no technology — the
    // discovery engine returns `None` for such a hole; the door refuses in kind.
    assert_eq!(
        w.build_recipe_as(cfe::world::SOVEREIGN, orb, inputs.clone(), role, ratio(0, 1)),
        Err(ActionRefused::RecipeYieldEmpty)
    );
    // The lawful build lands and folds.
    w.build_recipe_as(cfe::world::SOVEREIGN, orb, inputs.clone(), role, y.clone())
        .expect("the recipe builds");
    assert_eq!(
        recipes_built(w.chronicle()),
        vec![BuiltRecipe { orb_id: orb, inputs: inputs.clone(), role, yield_of_role: y.clone() }]
    );
    // A recipe is minted once: the SAME (inputs ‖ role) on this orb refuses.
    assert_eq!(
        w.build_recipe_as(cfe::world::SOVEREIGN, orb, inputs.clone(), role, y.clone()),
        Err(ActionRefused::RecipeAlreadyBuilt { orb_id: orb })
    );
    // ...but the same inputs into a DIFFERENT role is a distinct recipe (no
    // collision — the transformation, not the ingredient, is the identity).
    w.build_recipe_as(cfe::world::SOVEREIGN, orb, inputs.clone(), cfe::morphogen::FOOD, y.clone())
        .expect("a different role is a different recipe");
    assert_eq!(recipes_built(w.chronicle()).len(), 2);

    // SEAL B: the receipts replay to the same committed state — the built set
    // is the FOLD, no committed field moves (the SolverWordMinted discipline).
    assert_eq!(cfe::world::replay(w.chronicle()), w.committed());
}

#[test]
fn the_recipe_built_deed_round_trips_on_the_wire() {
    // A multi-input recipe (the count prefix at work).
    let deed = ClosureEvent::RecipeBuilt {
        orb_id: 42,
        inputs: vec![[1u8; 32], [2u8; 32], [3u8; 32]],
        role: cfe::morphogen::COGNITION,
        yield_of_role: ratio(5, 8),
    };
    let bytes = deed.wire_bytes();
    let mut records = cfe::tlv::records(&bytes);
    let (tag, value) = records.next().expect("one record");
    assert_eq!(tag, 181, "RecipeBuilt is tag 181");
    assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), deed);

    // A single-input recipe round-trips too (the count prefix, boundary 1) —
    // the yield runs to the frame edge with no trailing ambiguity.
    let single = ClosureEvent::RecipeBuilt {
        orb_id: 1,
        inputs: vec![[8u8; 32]],
        role: cfe::morphogen::SINK,
        yield_of_role: ratio(1, 1),
    };
    let sb = single.wire_bytes();
    let mut sr = cfe::tlv::records(&sb);
    let (st, sv) = sr.next().expect("one record");
    assert_eq!(ClosureEvent::from_wire(st, sv).expect("decodes"), single);
}
