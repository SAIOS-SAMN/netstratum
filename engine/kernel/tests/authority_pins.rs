//! W-A slice-1 — the banked-neutral authority deeds enter coverage
//! (the I1/I2/I3 coverage campaign, U_DIRECTIVES §5). Red-first.
//!
//! These deeds carry AUTHORITY, not value: their `world/replay.rs`
//! arms touch no bank (they set a flag, a seal, a list — terrain seed,
//! star, practice, ward, veil, route, offer, bond, genesis, prospect,
//! instrument, the succession boundary, a licence record). So the
//! completeness rule is satisfied by {I2 containment, I3 exactness}
//! ALONE — conservation's non-constraint (`_ => Ok(())`) is CORRECT
//! for them, ratified here against the replay reading: the deed is
//! judged by WHO may do it (Build/Raze scope) and that the state stays
//! exact, never by a value delta it does not carry. Value-moving
//! "authority" deeds (ColonyFounded/Charted, ZoneBuilt/Razed,
//! Forward/Council seals — all move banks) are W-A slice-2, gated on
//! their I1 arms; they are NOT admitted here.

use cfe::dop::ClosureEvent;
use cfe::ipsal::Scope;
use cfe::ratio::ratio;
use cfe::world::{World, SOVEREIGN};
use kernel::{lawful, Step, Unlawful};

/// A founded world (sovereign auto-registered, holds every scope) plus
/// a bare agent holding NOTHING — the containment test material.
fn founded_with_agent() -> (World, u64) {
    let mut world = World::new();
    world.found("origin").expect("founds");
    let agent = world.register_agent("hand").expect("registers");
    (world, agent)
}

/// The Build-scope banked-neutral authority deeds, representative set.
fn build_deeds() -> Vec<ClosureEvent> {
    vec![
        ClosureEvent::WorldCharted { seed: [7u8; 32] },
        ClosureEvent::StarCommissioned { class: 5 },
        ClosureEvent::PracticeAdopted { practice: 1 },
        ClosureEvent::ElderWarded {},
        ClosureEvent::VeilDrawn {},
        ClosureEvent::RouteChartered { deal: 1, eta: ratio(1, 1) },
        // THE MISSED SIXTH (arch-02's audit) entered coverage here as
        // GaugeShiftedAt — then U2d's GAUGE LAW landed and outran this
        // fixture: the deed DECLARES a key move, so judging it with
        // prior == next rightly refuses GaugeBroken now. WITNESS
        // TRUTH-UPDATE: its containment stays witnessed by the Build
        // family here in spirit; its full judgment (the exact resize,
        // the declared key at its grade) lives in gauge_pins.rs with
        // an honest next state.
    ]
}

/// The Raze-scope banked-neutral authority deeds (lift what was sealed).
fn raze_deeds() -> Vec<ClosureEvent> {
    vec![
        ClosureEvent::WardLifted { claimer: "elder".into() },
        ClosureEvent::VeilLifted { starved: false },
    ]
}

fn judge(world: &World, event: &ClosureEvent, actor: u64) -> Result<(), Unlawful> {
    // Banked-neutral: prior and next carry the SAME banks; the deed is
    // judged by scope and exactness, not by a value delta.
    let state = world.committed();
    let step = Step { event: event.clone(), actor, stamp: world.now() };
    lawful(&state, &state, &step)
}

#[test]
fn build_authority_deeds_gate_by_the_build_scope() {
    let (world, agent) = founded_with_agent();
    for event in build_deeds() {
        // The sovereign holds Build by construction: admitted (no longer
        // Unjudged — the coverage landed).
        assert_eq!(
            judge(&world, &event, SOVEREIGN),
            Ok(()),
            "the sovereign's authority deed is judged and admitted: {event:?}"
        );
        // A bare agent lacks Build: refused by containment.
        assert_eq!(
            judge(&world, &event, agent),
            Err(Unlawful::NotPermitted { actor: agent, scope: Scope::Build }),
            "authority is granted, never assumed: {event:?}"
        );
        // A deed from no one refuses before any state read.
        assert_eq!(
            judge(&world, &event, 999),
            Err(Unlawful::NoSuchEntity { actor: 999 }),
            "identity precedes the world: {event:?}"
        );
    }
}

#[test]
fn raze_authority_deeds_gate_by_the_raze_scope() {
    let (mut world, agent) = founded_with_agent();
    for event in raze_deeds() {
        assert_eq!(judge(&world, &event, SOVEREIGN), Ok(()), "sovereign razes: {event:?}");
        assert_eq!(
            judge(&world, &event, agent),
            Err(Unlawful::NotPermitted { actor: agent, scope: Scope::Raze }),
            "the lift needs Raze, granted: {event:?}"
        );
    }
    // Grant the agent Raze: now the lift is admitted — authority is a
    // scope, not a blanket.
    world.grant(agent, Scope::Raze).expect("grants raze");
    for event in raze_deeds() {
        assert_eq!(judge(&world, &event, agent), Ok(()), "granted, the agent lifts: {event:?}");
    }
}

#[test]
fn a_value_moving_authority_deed_is_held_to_its_charter() {
    // WITNESS TRUTH-UPDATE (the value-law wave): in slice-1 this pin
    // held ColonyFounded OUT as Unjudged. Its I1 arm has landed — the
    // founding declares EXACTLY the chartered reserves, so the same
    // smuggling attempt (a founding whose next state banks nothing)
    // now refuses by the STRONGER law: conservation, both totals
    // carried. The guard's spirit stands — no mint slips through the
    // authority door — the refusal just got teeth.
    let (world, _agent) = founded_with_agent();
    let minting = ClosureEvent::ColonyFounded { orb_id: 99, name: "smuggler".into() };
    assert!(
        matches!(
            judge(&world, &minting, SOVEREIGN),
            Err(Unlawful::ConservationBroken { .. })
        ),
        "a founding that does not bank its charter refuses by conservation"
    );
}
