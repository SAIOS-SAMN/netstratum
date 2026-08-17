//! **RCP-2 F3 — THE EMERGENT DOING-LAYER GROWS.** The beat searches a society's
//! own almanac (the elements it holds) for the one that fills its BINDING drive's
//! deficit role, gated by the manifold's topological reach (`recipe::discover`),
//! and raises the recipe — no ZoneKind, no level.
//!
//! Two facts, witnessed here:
//! 1. THE SEAM composes: given a species manifold and a held almanac, the exact
//!    discover→build_recipe_as path build_step runs raises a real recipe that
//!    enters the fold, yields into the economy, and replays exactly.
//! 2. THE TRIGGER is scarcity: recipe discovery is gated on a BINDING drive (a
//!    tier left wanting). A founded era of FED lines (the sated, prosperous
//!    world) carries no binding deficit, so the deficit path stays dormant —
//!    correctly, necessity has not struck. (Proactive recipe choice by a fed
//!    society is F4, the workbench draw.) The era assertion here is the
//!    replay-safety guard: any recipe a line DOES raise replays exactly.

use conductor::run_era;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("recipe-grow-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

#[test]
fn the_discover_then_build_seam_grows_the_doing_layer() {
    // The exact composition build_step performs when a drive binds: read the
    // species manifold, search the society's almanac for the element that fills
    // the deficit role, and raise the recipe it finds.
    let mut w = cfe::world::World::new();
    w.found("seam").expect("founds");
    let orb = w.orb().expect("founded").orb_id;

    // A biotic species and its manifold, as build_step reads them.
    let species = cfe::morphogen::archetype(2);
    let manifold = conductor::soc::SpeciesManifold::of(2, "biotic", false);
    let rank = manifold.algebraic_rank();

    // The society's almanac — the elements it holds. Complexity tier 1
    // (byte 2 = 0) so the topological gate admits them at any reach; enough
    // salts that one is sure to project into FOOD for a biotic line (the diag
    // confirmed held elements project into FOOD for every founded polity).
    let almanac: Vec<[u8; 32]> = (0..32u8)
        .map(|salt| {
            let mut e = *blake3::hash(&[salt]).as_bytes();
            e[2] = 0;
            e
        })
        .collect();

    let recipe = conductor::recipe::discover(cfe::morphogen::FOOD, &almanac, &species, &rank)
        .expect("the almanac offers a food recipe to a biotic line");
    // Raise it exactly as build_step does on a Metabolic deficit.
    w.build_recipe_as(
        cfe::world::SOVEREIGN,
        orb,
        recipe.inputs.clone(),
        recipe.role,
        recipe.yield_of_role.clone(),
    )
    .expect("the recipe raises");

    // It entered the fold, and the doing-layer replays exactly.
    let built = cfe::vocabulary::recipes_built(w.chronicle());
    assert_eq!(built.len(), 1, "the raised recipe stands in the fold");
    assert_eq!(built[0].role, cfe::morphogen::FOOD);
    assert_eq!(cfe::world::replay(w.chronicle()), w.committed(), "SEAL B");
}

#[test]
fn a_fed_roster_proactively_grows_recipes_on_the_workbench() {
    // RCP-2 F4 — recipes compete in the workbench partition, so a FED line running
    // a surplus (leisure > 0) grows its doing-layer PROACTIVELY, not only under
    // scarcity. Over a founded era a real fraction of the roster raises a recipe,
    // and every recipe-bearing line replays exactly (SEAL B holds across the wire).
    let root = scratch("era");
    let worlds = run_era(&root.to_string_lossy(), 20);
    assert!(worlds.len() >= 3, "the roster founded");
    let mut with_recipe = 0usize;
    for polity in &worlds {
        let built = cfe::vocabulary::recipes_built(polity.world.chronicle());
        if built.is_empty() {
            continue;
        }
        with_recipe += 1;
        for recipe in &built {
            assert!(!recipe.inputs.is_empty(), "a raised recipe consumes matter");
            assert!(recipe.yield_of_role > cfe::ratio::ratio(0, 1), "it delivers yield");
        }
        assert_eq!(
            cfe::world::replay(polity.world.chronicle()),
            polity.world.committed(),
            "a recipe-bearing line replays exactly (SEAL B holds across the wire)"
        );
    }
    eprintln!("RECIPE ERA: {with_recipe} of {} lines raised a recipe", worlds.len());
    // The doing-layer is LIVE: the emergent recipe space is not a dormant wire but
    // a thing fed societies actually build from their own almanac.
    assert!(
        with_recipe >= 1,
        "no line grew a recipe — the workbench never drew the emergent doing-layer"
    );
}
