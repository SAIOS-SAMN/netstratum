//! D3 pins (02 §2.5–2.6): the seeded chart. The deed is opt-in and
//! once-forever; the seed is the FOUNDING receipt's digest; classes
//! derive pure; multipliers land on extraction and agriculture only;
//! adjacency applies after terrain; upkeep never scales; an uncharted
//! star is untouched (backward-lawful). Replay and resume carry it.

use cfe::dop::{tile_class, ClosureEvent, Resource, TileClass, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

fn banked(world: &World, resource: Resource) -> Ratio {
    world
        .orb()
        .map(|orb| orb.banked_amount(resource))
        .unwrap_or_else(|| ratio(0, 1))
}

fn neighbors(tile: u16) -> Vec<u16> {
    let (row, col) = (tile / 8, tile % 8);
    let mut out = Vec::new();
    if row > 0 { out.push(tile - 8); }
    if row < 7 { out.push(tile + 8); }
    if col > 0 { out.push(tile - 1); }
    if col < 7 { out.push(tile + 1); }
    out
}

/// The fixture: a founded, charted star — same name, same seed, same
/// chart, every run (the founding digest is deterministic).
fn charted() -> (World, [u8; 32]) {
    let mut world = World::new();
    world.found("seamworld").expect("founds");
    let seed = world.chart_terrain().expect("charts");
    (world, seed)
}

fn class(seed: &[u8; 32], tile: u16) -> TileClass {
    tile_class(seed, FIRST_ORB, tile)
}

/// Find a tile satisfying a predicate over the derived chart.
fn tile_where(_seed: &[u8; 32], pred: impl Fn(u16) -> bool) -> u16 {
    (0..64u16).find(|t| pred(*t)).expect("the chart holds such a tile")
}

#[test]
fn the_deed_is_once_and_the_seed_is_the_founding_digest() {
    // Containment precedes the world (NS-6 §1.2): before founding the
    // refusal is "no entity," never "no colony."
    let mut unfounded = World::new();
    assert!(matches!(
        unfounded.chart_terrain(),
        Err(ActionRefused::NoSuchEntity { .. })
    ));

    let (mut world, seed) = charted();
    assert_eq!(
        seed,
        world.chronicle().receipts()[0].own.0,
        "the seed IS the founding receipt's digest — chronicle-seeded"
    );
    assert!(matches!(
        world.chart_terrain(),
        Err(ActionRefused::TerrainAlreadyCharted)
    ));
    let charters = world
        .chronicle()
        .receipts()
        .iter()
        .filter(|r| matches!(r.event, ClosureEvent::WorldCharted { .. }))
        .count();
    assert_eq!(charters, 1, "one chart per star, forever");

    let rebuilt = replay(world.chronicle());
    assert_eq!(rebuilt.terrain_seed, Some(seed));
    assert_eq!(rebuilt, world.committed(), "replay MATCHES the charted star");
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.committed(), world.committed(), "resume carries the seed");
}

/// Backward-lawful: a star that never charts yields exactly the
/// ruleset numbers — the opt-in law protects every landed world.
#[test]
fn an_uncharted_star_stands_untouched() {
    let mut world = World::new();
    world.found("plainworld").expect("founds");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("collects");
    // 20 − 10 (cost) + 4 × 3/2 = 16 iron; 10 − 4 × 1/2 = 8 biomass.
    assert_eq!(banked(&world, Resource::Iron), ratio(16, 1));
    assert_eq!(banked(&world, Resource::Biomass), ratio(8, 1));
}

/// The class multiplies the yield: a rich-seam extractor runs 9/4, a
/// barren one 3/4 — and upkeep never scales (biomass drains 1/2
/// either way).
#[test]
fn the_class_multiplies_the_yield_and_never_the_upkeep() {
    let (mut world, seed) = charted();
    let rich_isolated = tile_where(&seed, |t| {
        class(&seed, t) == TileClass::Rich
            && neighbors(t).into_iter().all(|n| class(&seed, n) != TileClass::Rich)
    });
    world.zone(rich_isolated, ZoneKind::Extractor).expect("zones the seam");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("collects");
    // 20 − 10 + 4 × (3/2 × 3/2) = 19 iron; upkeep unscaled: 10 − 2 = 8.
    assert_eq!(banked(&world, Resource::Iron), ratio(19, 1));
    assert_eq!(banked(&world, Resource::Biomass), ratio(8, 1));

    let (mut world, seed) = charted();
    let barren_isolated = tile_where(&seed, |t| {
        class(&seed, t) == TileClass::Barren
            && neighbors(t).into_iter().all(|n| class(&seed, n) != TileClass::Rich)
    });
    world.zone(barren_isolated, ZoneKind::Extractor).expect("zones the waste");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("collects");
    // 20 − 10 + 4 × (3/2 × 1/2) = 13 iron.
    assert_eq!(banked(&world, Resource::Iron), ratio(13, 1));
    assert_eq!(banked(&world, Resource::Biomass), ratio(8, 1));
}

/// Adjacency lands AFTER terrain: a standard-tile extractor beside a
/// rich tile runs ×5/4 (the seam under the fence); a habitat beside
/// an active extractor runs ×3/4 (noise and dust).
#[test]
fn adjacency_applies_after_terrain() {
    let (mut world, seed) = charted();
    let beside_seam = tile_where(&seed, |t| {
        class(&seed, t) == TileClass::Standard
            && neighbors(t).into_iter().any(|n| class(&seed, n) == TileClass::Rich)
    });
    world.zone(beside_seam, ZoneKind::Extractor).expect("zones by the fence");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("collects");
    // 20 − 10 + 4 × (3/2 × 1 × 5/4) = 35/2 iron.
    assert_eq!(banked(&world, Resource::Iron), ratio(35, 2));

    let (mut world, seed) = charted();
    // A quiet standard block: an extractor and its habitat neighbor,
    // no rich tile touching either (isolating both multipliers).
    let pit = tile_where(&seed, |t| {
        class(&seed, t) == TileClass::Standard
            && neighbors(t).into_iter().all(|n| class(&seed, n) == TileClass::Standard)
            && neighbors(t).into_iter().any(|n| {
                neighbors(n).into_iter().all(|m| class(&seed, m) != TileClass::Rich)
                    && class(&seed, n) == TileClass::Standard
            })
    });
    let hearth = neighbors(pit)
        .into_iter()
        .find(|n| {
            class(&seed, *n) == TileClass::Standard
                && neighbors(*n).into_iter().all(|m| class(&seed, m) != TileClass::Rich)
        })
        .expect("a quiet neighbor stands");
    world.zone(pit, ZoneKind::Extractor).expect("zones the pit");
    world.zone(hearth, ZoneKind::Habitat).expect("zones the hearth");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("collects");
    // Extractor: standard, no rich fence → 4 × 3/2 = 6.
    // Habitat: standard, beside the pit → 4 × (1 × 3/4) = 3.
    // iron: 20 − 10 + 6 − 4 × 1/4 (habitat life support) = 15.
    // biomass: 10 − 5 − 4 × 1/2 (crew rations) + 3 = 6.
    assert_eq!(banked(&world, Resource::Iron), ratio(15, 1));
    assert_eq!(banked(&world, Resource::Biomass), ratio(6, 1));
}
