//! THE CAPACITY LAW — `D > 0` and `dV ≠ 0`, or no mint.
//!
//! The rule VERSION 3 promised and the code did not carry. Value is
//! created against structure that has somewhere to go; a world holding
//! neither, or one without the other, has nothing for work to bite on.

use cfe::dop::{ClosureEvent, ColonyOrb, Resource};
use cfe::ipsal::{Entity, Scope};
use cfe::ratio::ratio;
use cfe::world::{Bond, Committed, World, SOVEREIGN};
use kernel::{lawful, Step, Unlawful};
use rfa_core::Vacuity;

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

/// `n` orbs and the two entities every deed needs.
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

/// A zero-amount mint: it clears the I4 bound trivially, so whatever
/// refuses it refused on STANDING and nothing else.
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

#[test]
fn a_world_with_nothing_cannot_mint() {
    // VOID — no structure, no gradient. There is nothing here for work
    // to be done against, and the refusal names the row.
    let bare = world(3);
    assert_eq!(
        lawful(&bare, &bare, &mint()),
        Err(Unlawful::MintWithoutStanding { verdict: Vacuity::Void }),
        "a world holding no relations at all mints nothing"
    );
}

#[test]
fn a_perfectly_circulating_world_cannot_mint() {
    // STASIS — the load-bearing case, and the one rank alone would have
    // admitted. Four orbs in a balanced ring: real structure, one
    // independent cycle, and every orb owing exactly what it is owed so
    // the gradient cancels at every vertex.
    //
    // Article III LAW 4: a system in equilibrium performs no work no
    // matter how much it holds. The refusal is thermodynamic, not
    // moral — nothing is being punished for being idle, there is simply
    // no work to be had from a flat field.
    let mut ring = world(4);
    ring.bonds = vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3)];

    assert_eq!(
        lawful(&ring, &ring, &mint()),
        Err(Unlawful::MintWithoutStanding { verdict: Vacuity::Stasis }),
        "structure with no gradient does no work — at any size"
    );
}

#[test]
fn flow_with_nothing_holding_it_cannot_mint() {
    // UNFRAMED — a source and a sink, gradient with no cycle to hold
    // it. Potential going somewhere and no shape to carry it.
    let mut chain = world(3);
    chain.bonds = vec![bond(1, 2, 3), bond(2, 3, 3)];

    assert_eq!(
        lawful(&chain, &chain, &mint()),
        Err(Unlawful::MintWithoutStanding { verdict: Vacuity::Unframed }),
        "gradient with no structure mints nothing either"
    );
}

#[test]
fn a_world_carrying_both_poles_may_mint() {
    // ENTANGLED — the ring plus a chord: a second independent cycle AND
    // two vertices that do not balance. Both poles in one world, and
    // the mint passes the capacity law.
    let mut live = world(4);
    live.bonds =
        vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3), bond(1, 3, 1)];

    // The capacity law ADMITS it. The deed then fails on its own proof
    // (this fixture carries an empty payload deliberately), which is a
    // DIFFERENT law — and that separation is the point: standing and
    // proof are judged independently, so a refusal never conflates
    // "this world cannot earn" with "this claim is false".
    let seen = lawful(&live, &live, &mint());
    assert!(
        !matches!(seen, Err(Unlawful::MintWithoutStanding { .. })),
        "structure WITH gradient may earn — standing is not what stops it"
    );
    assert_eq!(
        seen,
        Err(Unlawful::WorkRefused { broken: solver_verify::WorkBroken::Malformed }),
        "it refuses on the empty proof instead, which is the next law along"
    );
}

#[test]
fn the_law_gates_creation_never_motion() {
    // A transfer moves value that already exists and needs no gradient
    // to justify it — the capacity law is about CREATION. A dead ring
    // may still move what it holds.
    let mut ring = world(4);
    ring.bonds = vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3)];
    ring.orbs[0].bank(Resource::Iron, ratio(10, 1));

    let move_it = Step {
        event: ClosureEvent::ResourceTransferred {
            from_orb: 1,
            to_orb: 2,
            resource: Resource::Iron,
            amount: ratio(0, 1),
        },
        actor: HAND,
        stamp: 1,
    };
    assert_eq!(lawful(&ring, &ring, &move_it), Ok(()), "motion is not creation");
}

#[test]
fn the_refusal_carries_which_row_failed() {
    // The house law: a refusal names its cure. Void, Stasis and
    // Unframed each need a DIFFERENT act to fix — form any relation,
    // form an asymmetric one, close a cycle — so a bare "refused" would
    // leave the caller guessing which.
    let bare = world(3);
    let mut ring = world(4);
    ring.bonds = vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3)];

    let (a, b) = (lawful(&bare, &bare, &mint()), lawful(&ring, &ring, &mint()));
    assert_ne!(a, b, "two dead worlds, two different diagnoses");
}
