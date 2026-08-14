//! COGNITION-2 (operator ruling 2026-08-14): the tier above utility
//! SCORING — utility SEARCH. `cognition::settlement_choice` scores the
//! CURRENT beat with a static formula; this module clones the world,
//! actually builds a candidate on the clone, ticks it forward, and reads
//! back what that choice really leads to. It is real search-and-evaluate
//! planning, not another if/else — and it costs nothing the engine
//! doesn't already pay: `World` derives `Clone` (every field already
//! did), `tick()` is the same pure state transition `build_step` calls
//! for real, `zone_as` is the same build call. The clone is thrown away;
//! nothing here ever touches the live world or its chronicle.
//!
//! Only engaged where `build_step` already has a genuine multi-candidate
//! choice (both Extractor and Habitat viable) — every cap, reserve gate,
//! and bootstrap hold upstream is untouched.

use cfe::dop::ZoneKind;
use cfe::ratio::Ratio;
use cfe::world::World;

/// What a candidate build actually leads to, `depth` beats out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Projection {
    pub zones_total: usize,
    pub ext: i64,
    pub hab: i64,
    pub iron: Ratio,
    pub bio: Ratio,
    pub famished: bool,
}

/// Clone `world`, build `kind` as `actor` on `orb`/`tile`, tick `depth`
/// times, and read back the result. `None` only if the build itself
/// refuses on the clone — `build_step` already checked viability on the
/// live world moments earlier, so this is a defensive fallback, not the
/// expected path.
pub fn project(
    world: &World,
    actor: u64,
    orb: u64,
    tile: u16,
    kind: ZoneKind,
    depth: u32,
) -> Option<Projection> {
    let mut shadow = world.clone();
    shadow.zone_as(actor, orb, tile, kind).ok()?;
    for _ in 0..depth {
        shadow.tick();
    }
    read_projection(&shadow)
}

/// Clone `world` and tick `depth` times with no build — "if this line
/// does nothing more, where does it stand." Used where no candidate is
/// affordable yet doom may already be underway (a starving line that
/// cannot build its way out this beat still deserves to know it).
pub fn project_passive(world: &World, depth: u32) -> Option<Projection> {
    let mut shadow = world.clone();
    for _ in 0..depth {
        shadow.tick();
    }
    read_projection(&shadow)
}

fn read_projection(world: &World) -> Option<Projection> {
    let held = world.orb()?;
    Some(Projection {
        zones_total: held.zones.len(),
        ext: held.zones.iter().filter(|z| z.kind == ZoneKind::Extractor).count() as i64,
        hab: held.zones.iter().filter(|z| z.kind == ZoneKind::Habitat).count() as i64,
        iron: held
            .banked
            .iter()
            .find(|(r, _)| *r == cfe::dop::Resource::Iron)
            .map(|(_, amount)| amount.clone())
            .unwrap_or_else(|| cfe::ratio::ratio(0, 1)),
        bio: held
            .banked
            .iter()
            .find(|(r, _)| *r == cfe::dop::Resource::Biomass)
            .map(|(_, amount)| amount.clone())
            .unwrap_or_else(|| cfe::ratio::ratio(0, 1)),
        famished: !held.famine.is_empty(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cfe::world::{World, FIRST_ORB, SOVEREIGN};

    fn founded_world() -> World {
        let mut world = World::new();
        world.found("shadow-test").expect("founds");
        world
    }

    #[test]
    fn project_passive_ticks_without_building() {
        let world = founded_world();
        let zones_before = world.orb().expect("orb").zones.len();
        let projection = project_passive(&world, 2).expect("a founded orb stands");
        assert_eq!(projection.zones_total, zones_before);
    }

    #[test]
    fn project_leaves_the_real_world_untouched() {
        let world = founded_world();
        let before = world.orb().expect("orb").zones.len();
        let tile = crate::next_free_tile(&world).expect("a free tile stands");
        let _ = project(&world, SOVEREIGN, FIRST_ORB, tile, ZoneKind::Extractor, 2);
        assert_eq!(world.orb().expect("orb").zones.len(), before);
    }

    #[test]
    fn project_reports_the_built_zone() {
        let world = founded_world();
        let ext_before = world
            .orb()
            .expect("orb")
            .zones
            .iter()
            .filter(|z| z.kind == ZoneKind::Extractor)
            .count() as i64;
        let tile = crate::next_free_tile(&world).expect("a free tile stands");
        let projection = project(&world, SOVEREIGN, FIRST_ORB, tile, ZoneKind::Extractor, 0)
            .expect("the build is affordable at founding");
        assert_eq!(projection.ext, ext_before + 1);
    }

    #[test]
    fn an_unaffordable_build_refuses_on_the_clone_too() {
        let world = founded_world();
        let tile = crate::next_free_tile(&world).expect("a free tile stands");
        // Synthesizer is never in the founding kit's reach.
        assert!(project(&world, SOVEREIGN, FIRST_ORB, tile, ZoneKind::Synthesizer, 2).is_none());
    }
}
