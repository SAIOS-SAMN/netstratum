//! W-C / W-D slice — the banked-neutral WORLD-DRIVEN and MESH deeds
//! enter coverage (the coverage campaign, U_DIRECTIVES §5). Red-first.
//!
//! These deeds are consequences the tick computes (no actor) or
//! universe/mesh-ledger entries — their `world.rs::replay` arms set a
//! flag, a list, or are outright no-ops, touching NO bank. So
//! `scope_of` is `None` (containment carries no requirement — a
//! world-driven consequence is not attributed to a hand) and
//! conservation's non-constraint is CORRECT (ratified against the
//! replay reading). They are judged by I3 exactness alone. The
//! value-moving members of these families (FamineStruck −drawn,
//! CovenantSlashed +collateral, CouncilClosed +banked) stay held out
//! for the I1 value slice; the FOLD TRIO (Confluent/EraAnnexed/
//! EraFolded) stays held out for U2d's I6 gate — a fold must be
//! orbit-lock-judged, never admitted on exactness alone.

use cfe::dop::{ClosureEvent, Resource};
use cfe::ratio::ratio;
use cfe::world::{World, SOVEREIGN};
use kernel::{lawful, Step, Unlawful};

fn founded() -> World {
    let mut world = World::new();
    world.found("origin").expect("founds");
    world
}

/// Representative banked-neutral world-driven / mesh deeds (clear
/// fields). The full covered set is larger — ratified against replay
/// in `kernel/src/lib.rs`; these witness the pattern.
fn world_driven_deeds() -> Vec<ClosureEvent> {
    vec![
        ClosureEvent::FamineRelieved { orb_id: 1, resource: Resource::Iron },
        ClosureEvent::OverheatCleared { orb_id: 1 },
        ClosureEvent::OverheatStruck { orb_id: 1, overflow: ratio(1, 1) },
        ClosureEvent::VentSettled { orb_id: 1, amount: ratio(1, 1) },
        ClosureEvent::AbsorptionLapsed { debtor_orb: 1, creditor_orb: 2, amount: ratio(1, 1) },
        ClosureEvent::WantRelieved { orb_id: 1, kind: 1 },
        ClosureEvent::StarAged { class: 5 },
        ClosureEvent::RouteLapsed { deal: 1 },
        ClosureEvent::VoidShare { deal: 1, resource: Resource::Iron, amount: ratio(1, 1) },
        // Mesh / universe-ledger deeds (W-D non-fold): no bank, no actor.
        ClosureEvent::HandoffSealed { from: 1, to: 2 },
        ClosureEvent::GaugeShifted { gauge: ratio(1, 3) },
        ClosureEvent::Diverged { branch: "fork".into(), gauge: ratio(1, 4) },
    ]
}

fn judge(world: &World, event: &ClosureEvent, actor: u64) -> Result<(), Unlawful> {
    let state = world.committed();
    let step = Step { event: event.clone(), actor, stamp: world.now() };
    lawful(&state, &state, &step)
}

#[test]
fn world_driven_deeds_are_judged_without_a_scope() {
    let world = founded();
    for event in world_driven_deeds() {
        // Admitted — banked-neutral, exact, and carrying no scope
        // requirement (the tick's own consequence, no hand to check).
        assert_eq!(
            judge(&world, &event, SOVEREIGN),
            Ok(()),
            "the world-driven deed is judged and admitted: {event:?}"
        );
        // And admitted even from a NON-ENTITY: a world-driven
        // consequence is not attributed to an actor, so containment is
        // vacuous by design (scope_of = None). This is the deliberate
        // asymmetry — authority deeds gate on the hand, consequences
        // do not.
        assert_eq!(
            judge(&world, &event, 999),
            Ok(()),
            "no actor scope: the consequence admits regardless of hand: {event:?}"
        );
    }
}

#[test]
fn a_value_moving_consequence_stays_unjudged() {
    // FamineStruck is a world-driven deed too — but it DRAWS a bank
    // (world.rs banks −drawn). It moves value, so exactness alone does
    // not complete it; it must NOT have entered coverage here. The
    // sealed default holds it for the I1 value slice.
    let world = founded();
    let drawing = ClosureEvent::FamineStruck {
        orb_id: 1,
        resource: Resource::Iron,
        drawn: ratio(1, 1),
    };
    assert_eq!(
        judge(&world, &drawing, SOVEREIGN),
        Err(Unlawful::Unjudged),
        "a value-drawing consequence is not admitted by the neutral slice"
    );
}

#[test]
fn the_fold_trio_stays_unjudged_until_the_gate() {
    // Confluent / EraAnnexed / EraFolded are banked-neutral at world
    // replay (gauge/no-op) — but a FOLD is lawful only when the I6
    // orbit-lock gate says so (U2d). Admitting them on exactness alone
    // would be a fold with no gate. They stay Unjudged by design.
    let world = founded();
    for event in [
        ClosureEvent::Confluent { partner: "x".into(), gauge: ratio(1, 1) },
        ClosureEvent::EraFolded { era: 1, gauge: ratio(1, 1) },
    ] {
        assert_eq!(
            judge(&world, &event, SOVEREIGN),
            Err(Unlawful::Unjudged),
            "a fold waits on the I6 gate, never admitted by exactness: {event:?}"
        );
    }
}
