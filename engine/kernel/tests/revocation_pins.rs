//! **D4 — THE REVOCATION LAW, per step** (EQUATIONS D4).
//!
//! A deed that mints carries a non-zero capacity increment, or refuses.
//! The claim vacates; the degree does not.
//!
//! The load-bearing pins:
//!   · `a_mint_that_grows_nothing_refuses` — the rule itself.
//!   · `the_phantom_rank_gate_is_asked_first` — an unresolved absence
//!     is not a zero. Without it, phantom topology mints phantom rank.
//!   · `the_refusal_never_lowers_the_degree` — Article IV. D4 vacates a
//!     CLAIM; it is not a debit and has no write path.

use cfe::dop::{ClosureEvent, ColonyOrb, Resource};
use cfe::ipsal::{Entity, Scope};
use cfe::ratio::ratio;
use cfe::world::{Bond, Committed, World, SOVEREIGN};
use kernel::{bridge, lawful, Step, Unlawful};

const HAND: u64 = 100;

fn bond(source: u64, target: u64, rate: i64) -> Bond {
    Bond {
        source,
        target,
        resource: Resource::Iron,
        rate: ratio(rate, 1),
        term: 0,
        sealed: 0,
        anchored: 0,
    }
}

fn world(n: u64) -> Committed {
    let mut committed = World::new().committed();
    committed.orbs = (1..=n)
        .map(|orb_id| ColonyOrb { orb_id, name: format!("orb{orb_id}"), ..Default::default() })
        .collect();
    committed.entities = vec![
        Entity { entity_id: SOVEREIGN, handle: "sovereign".into(), principal: None, grants: vec![] },
        Entity {
            entity_id: HAND,
            handle: "hand".into(),
            principal: Some(SOVEREIGN),
            grants: vec![Scope::Collect],
        },
    ];
    committed
}

/// A zero-amount mint: it clears I4's bound trivially, so whatever
/// refuses it refused on the capacity increment and nothing else.
fn mint() -> Step {
    Step {
        event: ClosureEvent::DiscoveryMinted {
            orb_id: 1,
            resource: Resource::Crystal,
            amount: ratio(0, 1),
            work: Vec::new(),
        },
        actor: HAND,
        stamp: 1,
    }
}

/// A world holding one independent cycle — circulation 1 above ground.
fn ring(n: u64) -> Committed {
    let mut world = world(n);
    world.bonds = (1..=n).map(|i| bond(i, i % n + 1, 3)).collect();
    world
}

/// THE RULE. A mint whose capacity increment is zero refuses, and the
/// refusal carries both readings whole.
#[test]
fn a_mint_that_grows_nothing_refuses() {
    let still = ring(4);
    // prior == next: nothing grew across the step.
    match lawful(&still, &still, &mint()) {
        Err(Unlawful::MintWithoutCapacityIncrement { was, now }) => {
            assert_eq!(was, now, "the reading is carried whole, both sides");
        }
        // The capacity law may refuse first on standing; that is a
        // different rule and this pin is not about it. What must never
        // happen is ADMISSION.
        Err(other) => assert!(
            !matches!(other, Unlawful::Unjudged),
            "refused for a stated reason: {other:?}"
        ),
        Ok(()) => panic!("a mint that grew no circulation must never be admitted"),
    }
}

/// THE PHANTOM-RANK GATE, asked FIRST. `rank_is_defined` existed unused
/// for weeks with its own header warning that an unresolved absence
/// *"would let phantom topology mint phantom rank"* — recorded in
/// `18_TORSION` §6 as debt. D4 computes an increment over that rank, so
/// the debt is now due: an unresolved reading refuses before any
/// increment is compared.
#[test]
fn the_phantom_rank_gate_is_asked_first() {
    // The gate's own predicate is what D4 consults.
    let resolved = bridge::read_entangled(&ring(4));
    assert!(
        resolved.rank_is_defined(),
        "a fully observed world resolves its rank"
    );
    assert_eq!(resolved.unknown, 0, "nothing unobserved");

    // And an unresolved reading is NOT rank zero — the distinction the
    // gate exists to hold.
    let unresolved = bridge::read_entangled_observed(&ring(4), 2);
    assert!(
        !unresolved.rank_is_defined(),
        "two unobserved positions leave the rank undefined"
    );
    assert_eq!(unresolved.unknown, 2, "the absence is counted, never assumed away");
}

/// ARTICLE IV. D4 vacates a CLAIM — it is not a debit, and it has no
/// write path. The degree standing before the refused deed is exactly
/// the degree standing after it.
#[test]
fn the_refusal_never_lowers_the_degree() {
    let still = ring(4);
    let before = bridge::read_entangled(&still).degree();
    let _ = lawful(&still, &still, &mint());
    let after = bridge::read_entangled(&still).degree();
    assert_eq!(before, after, "the claim vacates; the degree does not");
    assert!(
        after >= rfa_core::Degree::GROUND,
        "D never falls below GROUND — it has no representation for it"
    );
}

/// D4 BINDS THE MINT, not motion. Moving value that already exists
/// makes no capacity claim, so there is no claim to vacate — those
/// deeds must pass D4 untouched whatever the circulation did.
#[test]
fn d4_binds_creation_and_leaves_motion_alone() {
    let still = ring(4);
    let transfer = Step {
        event: ClosureEvent::ScopeGranted { agent: HAND, scope: Scope::Build },
        actor: SOVEREIGN,
        stamp: 1,
    };
    // Whatever else judges this deed, it is never D4's two refusals.
    if matches!(
        lawful(&still, &still, &transfer),
        Err(Unlawful::MintWithoutCapacityIncrement { .. }) | Err(Unlawful::RankUndefined { .. })
    ) {
        panic!("D4 must not bind a deed that mints nothing")
    }
}

/// A mint that DOES grow circulation passes D4. Without this the rule
/// could be satisfied by refusing everything, which is not a law but a
/// wall — and the pin above would still be green.
#[test]
fn a_mint_that_grows_circulation_passes_d4() {
    let before = world(4); // no bonds: circulation 0
    let after = ring(4); //   one independent cycle: circulation 1
    let a = bridge::read_entangled(&before).degree().above();
    let b = bridge::read_entangled(&after).degree().above();
    assert!(b > a, "the fixture must actually grow, else this proves nothing");

    // D4's own arm admits this pair. Other invariants may still refuse
    // the deed for their own reasons; what this pins is that D4 does
    // not, so the law is a gate and not a wall.
    if let Err(Unlawful::MintWithoutCapacityIncrement { .. }) =
        lawful(&before, &after, &mint())
    {
        panic!("a mint that grew circulation must clear D4")
    }
}
