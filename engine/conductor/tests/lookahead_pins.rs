//! COGNITION-2 pin — a genuine probe, not an assumption: run an
//! extractor-only line (no biomass income) forward and record exactly
//! when it starves for real. Then check whether `lookahead::project_passive`,
//! asked at each beat "where does this line's current course lead two
//! beats out," sees the famine BEFORE it actually lands — the concrete,
//! testable claim that this is foresight, not just agreement after the
//! fact. Numbers here are measured against this run, never guessed.

use cfe::dop::ZoneKind;
use cfe::world::{World, FIRST_ORB, SOVEREIGN};
use conductor::lookahead::project_passive;

fn free_tile(world: &World) -> Option<u16> {
    let orb = world.orb()?;
    (0..64u16).find(|tile| !orb.zones.iter().any(|z| z.tile == *tile))
}

#[test]
fn the_lookahead_sees_an_extractor_only_famine_before_it_lands() {
    let mut world = World::new();
    world.found("probe").expect("founds");

    let mut real_famine_beat = None;
    let mut foresight_beat = None;

    for beat in 0..40 {
        if world.orb().unwrap().famine.is_empty() {
            // At every beat where the colony still stands, ask the
            // lookahead where two more beats of nothing-but-ticking
            // leads — the same "what does this line's current course
            // actually lead to" read `build_step` runs before it ever
            // commits to a build.
            if foresight_beat.is_none() {
                if let Some(projection) = project_passive(&world, 2) {
                    if projection.famished {
                        foresight_beat = Some(beat);
                    }
                }
            }
            if let Some(tile) = free_tile(&world) {
                let _ = world.zone_as(SOVEREIGN, FIRST_ORB, tile, ZoneKind::Extractor);
            }
        }
        world.tick();
        if !world.orb().unwrap().famine.is_empty() && real_famine_beat.is_none() {
            real_famine_beat = Some(beat);
            break;
        }
    }

    let real_famine_beat = real_famine_beat.expect("an extractor-only line starves within 40 beats");
    let foresight_beat = foresight_beat.expect("the lookahead flags the famine at some beat");

    eprintln!(
        "real famine at beat {real_famine_beat}, lookahead foresaw it at beat {foresight_beat}"
    );
    assert!(
        foresight_beat < real_famine_beat,
        "the lookahead only agreed after the fact (foresaw {foresight_beat}, \
         real famine {real_famine_beat}) — it should see it coming, not just \
         confirm it happened"
    );
}
