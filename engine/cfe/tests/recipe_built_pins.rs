//! RCP-2 — the chain's side of the emergent recipe (tag 181): the door's
//! refusals, the built-recipe fold, the wire, and SEAL B. The doing-layer
//! becomes a FOLD of receipts — no `ZoneKind` enum, no committed field.

use cfe::dop::{ClosureEvent, Resource};
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

#[test]
fn a_built_recipe_yields_its_role_into_the_economy_each_beat() {
    // F2: a built recipe delivers its role's resource per beat, folded into
    // the collect touch over the elapsed span. An ENERGY recipe yields into
    // Joule — the universal energy stock, which nothing else in a bare world
    // banks — so the recipe's contribution reads EXACTLY (food would be
    // muddied by regrowth; Joule is clean).
    let mut w = founded();
    let orb = w.orb().expect("founded").orb_id;
    let per_beat = ratio(2, 1);
    w.build_recipe_as(cfe::world::SOVEREIGN, orb, vec![[5u8; 32]], cfe::morphogen::ENERGY, per_beat.clone())
        .expect("the energy recipe builds");
    let t0 = w.now();
    for _ in 0..3 {
        w.tick();
    }
    // The yield settles at the touch, riding YieldCollected — no new deed.
    w.collect_as(cfe::world::SOVEREIGN, orb).expect("collect banks the recipe yield");
    let dt = w.now() - t0;
    assert_eq!(dt, 3, "three beats elapsed since the founding touch");
    let banked = w.orb().expect("founded").banked_amount(Resource::Joule);
    assert_eq!(
        banked,
        per_beat * ratio(dt as i64, 1),
        "the recipe yielded its role's resource each beat over the span"
    );
    // A role with no bankable home yet (STRUCT) yields nothing — the honest
    // hole; the build stands, the economy stays untouched by it.
    let before = w.orb().expect("founded").banked_amount(Resource::Iron);
    w.build_recipe_as(cfe::world::SOVEREIGN, orb, vec![[6u8; 32]], cfe::morphogen::STRUCT, ratio(9, 1))
        .expect("the structure recipe builds");
    for _ in 0..3 {
        w.tick();
    }
    w.collect_as(cfe::world::SOVEREIGN, orb).expect("collect");
    assert_eq!(
        w.orb().expect("founded").banked_amount(Resource::Iron),
        before,
        "an unbankable role is an honest hole — it yields nothing yet"
    );
    // SEAL B holds: the yield is a fold of existing receipts.
    assert_eq!(cfe::world::replay(w.chronicle()), w.committed());
}
