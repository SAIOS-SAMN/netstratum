//! GALACTIC-1 witnesses — the regional resource-weighting seed. A
//! world's charted (X,Y,Z) quantizes onto a coarse galactic grid cell
//! and the CELL (never the exact position) hashes into a seed folded
//! alongside the founding digest at prospect time: same-cell worlds
//! draw with the same regional character, a world one cell over draws
//! with none of it, and a world that never charts a region prospects
//! byte-identically to the pre-GALACTIC-1 draw.

use cfe::dop::{positive, region_seed_of, unit, ZoneKind, REGION_CELL};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB, SOVEREIGN};

fn charted(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world.site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None).expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    for c in 1u8..=5 {
        let _ = world.grant_tech_as(SOVEREIGN, FIRST_ORB, c);
    }
    world
}

#[test]
fn the_regional_seed_is_deterministic_and_grid_exact() {
    let a = region_seed_of(&ratio(10, 1), &ratio(10, 1), &ratio(10, 1));
    let b = region_seed_of(&ratio(10, 1), &ratio(10, 1), &ratio(10, 1));
    assert_eq!(a, b, "the same position always derives the same cell seed");

    // A position well inside the SAME cell (REGION_CELL wide) rounds to
    // the same cell index and so shares the seed exactly.
    let near = region_seed_of(
        &ratio(10 + REGION_CELL / 4, 1),
        &ratio(10, 1),
        &ratio(10, 1),
    );
    assert_eq!(a, near, "a nearby position in the same cell shares the seed");

    // A position a full cell over lands on a different cell index and
    // so derives a genuinely different seed — no interpolation.
    let far = region_seed_of(&ratio(10 + REGION_CELL * 4, 1), &ratio(10, 1), &ratio(10, 1));
    assert_ne!(a, far, "a position several cells over derives a different seed");
}

#[test]
fn charting_a_region_is_sealed_once_and_shapes_the_prospect() {
    let mut plain = charted("unregioned");
    let baseline = plain.prospect(FIRST_ORB, 1).expect("opens without a region");
    assert!(plain.committed().region_seed.is_none(), "no region charted — byte-identical draw");

    let mut regioned = charted("regioned");
    let seed = region_seed_of(&ratio(1, 1), &ratio(1, 1), &ratio(1, 1));
    regioned.chart_region(seed).expect("seals the region once");
    assert_eq!(
        regioned.chart_region(seed),
        Err(ActionRefused::RegionAlreadyCharted),
        "one seal per world; a system does not travel between galactic cells"
    );
    assert_eq!(regioned.committed().region_seed, Some(seed));
    let struck = regioned.prospect(FIRST_ORB, 1).expect("opens with a region charted");
    assert_ne!(
        struck.element, baseline.element,
        "the regional seed folds into the draw — a different seed, a different vein"
    );

    assert_eq!(replay(regioned.chronicle()), regioned.committed());
    let resumed = World::resume(regioned.chronicle().clone());
    assert_eq!(resumed.committed(), regioned.committed(), "resume carries the regional seed");
}

#[test]
fn same_cell_worlds_share_regional_character_a_far_cell_does_not() {
    // Two worlds charted at positions inside the SAME cell derive the
    // SAME regional seed — the topological gradient across the galaxy,
    // not per-world noise — even though their founding digests differ
    // (different colony names), so their prospects still diverge on
    // the LOCAL axis while sharing the regional one.
    let cell_seed = region_seed_of(&ratio(100, 1), &ratio(100, 1), &ratio(100, 1));
    let same_cell_other = region_seed_of(
        &ratio(100 + REGION_CELL / 3, 1),
        &ratio(100, 1),
        &ratio(100, 1),
    );
    assert_eq!(cell_seed, same_cell_other, "neighboring systems in one cell share a seed");

    let mut near_a = charted("near-a");
    near_a.chart_region(cell_seed).expect("seals");
    let mut near_b = charted("near-b");
    near_b.chart_region(same_cell_other).expect("seals");
    assert_eq!(
        near_a.committed().region_seed,
        near_b.committed().region_seed,
        "same-cell systems chart the identical regional seed"
    );

    // A world in a distant cell charts a DIFFERENT seed entirely.
    let far_seed = region_seed_of(&ratio(100 + REGION_CELL * 5, 1), &ratio(100, 1), &ratio(100, 1));
    assert_ne!(cell_seed, far_seed, "a distant system charts a different regional seed");
}
