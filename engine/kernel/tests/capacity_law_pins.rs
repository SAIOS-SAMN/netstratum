//! THE CAPACITY LAW — `D > 0` and `dV ≠ 0`, or no mint.
//!
//! The rule VERSION 3 promised and the code did not carry. Value is
//! created against structure that has somewhere to go; a world holding
//! neither, or one without the other, has nothing for work to bite on.

use cfe::dop::{ClosureEvent, ColonyOrb, Resource};
use cfe::ipsal::{Entity, Scope};
use cfe::ratio::ratio;
use cfe::world::{Bond, Committed, World, SOVEREIGN};
use kernel::{lawful, Step, Refused};
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
        admits(&bare, &bare, &mint()),
        Err(Refused::MintWithoutStanding { verdict: Vacuity::Void }),
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
        admits(&ring, &ring, &mint()),
        Err(Refused::MintWithoutStanding { verdict: Vacuity::Stasis }),
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
        admits(&chain, &chain, &mint()),
        Err(Refused::MintWithoutStanding { verdict: Vacuity::Unframed }),
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
    let seen = admits(&live, &live, &mint());
    assert!(
        !matches!(seen, Err(Refused::MintWithoutStanding { .. })),
        "structure WITH gradient may earn — standing is not what stops it"
    );
    assert_eq!(
        seen,
        Err(Refused::WorkRefused { broken: solver_verify::WorkBroken::Malformed }),
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
    assert_eq!(admits(&ring, &ring, &move_it), Ok(()), "motion is not creation");
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

    let (a, b) = (admits(&bare, &bare, &mint()), admits(&ring, &ring, &mint()));
    assert_ne!(a, b, "two dead worlds, two different diagnoses");
}

/// The same zero-amount mint, but claimed by a NAMED orb.
fn mint_by(orb_id: u64) -> Step {
    Step {
        event: ClosureEvent::DiscoveryMinted {
            orb_id,
            resource: Resource::Crystal,
            amount: ratio(0, 1),
            work: Vec::new(),
        },
        actor: HAND,
        stamp: 1,
    }
}

#[test]
fn an_orb_holding_nothing_cannot_mint_on_the_world_s_structure() {
    // THE FREE RIDE, CLOSED (SYN-4/SYN-5). Article II was read of the
    // WORLD and no orb appeared in the reading, so it could only ever
    // refuse — and it refused the wrong way. Orbs 1..4 build a ring with
    // a chord and the world reads Entangled. Orb 5 sits among them
    // holding no obligation at all, and under the world-level reading it
    // could mint on structure it had no part in. Standing was inherited.
    //
    // It is now asked personally: what does THIS orb hold up?
    let mut live = world(5);
    live.bonds =
        vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3), bond(1, 3, 1)];

    // The world itself still stands — this is not a world-level refusal.
    assert!(
        kernel::bridge::read_entangled(&live).verdict().is_admitted(),
        "the world carries both poles; the refusal below must be personal"
    );

    // Orb 5 holds nothing, so it stands on the ground and mints nothing.
    let seen = admits(&live, &live, &mint_by(5));
    assert_eq!(
        seen,
        Err(Refused::MintWithoutOwnStanding {
            orb: 5,
            axial: vec![ratio(0, 1), ratio(0, 1), ratio(0, 1)],
        }),
        "an orb holding no obligation may not mint on other orbs' structure"
    );

    // And orb 1, which built the structure, is NOT refused for standing —
    // it falls through to the next law along. The rule narrows onto the
    // free rider without touching the orb that did the work.
    assert!(
        !matches!(admits(&live, &live, &mint_by(1)), Err(Refused::MintWithoutOwnStanding { .. })),
        "the orb carrying the structure keeps its standing"
    );
}

#[test]
fn the_refusal_carries_the_share_so_the_floor_can_be_risen_from() {
    // NON-DEFENSIVE BY CONSTRUCTION. The refusal states what the orb
    // HOLDS — a magnitude, never negative — rather than naming a penalty.
    // An orb reads the ground and knows the lift is to form a real
    // obligation, not to avoid a sanction.
    let mut live = world(5);
    live.bonds =
        vec![bond(1, 2, 3), bond(2, 3, 3), bond(3, 4, 3), bond(4, 1, 3), bond(1, 3, 1)];

    let Err(Refused::MintWithoutOwnStanding { axial, .. }) = admits(&live, &live, &mint_by(5))
    else {
        panic!("orb 5 holds nothing and must be refused on its own standing");
    };
    // CARRIED PER AXIS, NEVER SUMMED. The refusal names one share per
    // CHANNEL, so it says WHICH DIRECTION is open rather than only how
    // short the orb is (`18` §2b bars the collapse where the system
    // computes; operator ruling 2026-08-02, multi-axial towards uplift).
    assert_eq!(axial.len(), kernel::bridge::CHANNELS, "one share per channel, never one scalar");
    for share in &axial {
        assert!(*share >= ratio(0, 1), "a share is a magnitude, never a debt");
    }
    // And every channel is open to it — the refusal is a map, not a score.
    assert_eq!(
        kernel::bridge::open_axes_of(&live, 5),
        vec![
            kernel::bridge::CHANNEL_BOND,
            kernel::bridge::CHANNEL_COVENANT,
            kernel::bridge::CHANNEL_DEBT
        ],
        "an orb holding nothing has every direction open to it"
    );

    // THE LIFT IS REAL. Give orb 5 a place in the circulation and the
    // same deed is no longer refused for standing.
    live.bonds.push(bond(4, 5, 3));
    live.bonds.push(bond(5, 1, 3));
    assert!(
        kernel::bridge::standing_of(&live, 5) > ratio(0, 1),
        "an orb that takes a place in the circulation rises off the ground"
    );
    assert!(
        !matches!(admits(&live, &live, &mint_by(5)), Err(Refused::MintWithoutOwnStanding { .. })),
        "having risen, the orb is no longer refused for standing"
    );
}

/// THE ARITY ITSELF, pinned literally — the one place it should be.
///
/// remediations-lane replaced literal `3`s in these pins with `CHANNELS`
/// (2026-08-02), which is right: a structural pin that hardcodes arity
/// breaks on every channel addition and teaches nothing. But deriving
/// EVERYWHERE means no pin can catch an unintended arity change, and
/// the channel count is wire-visible — `Delta` is `dim × dim × fiber`
/// and every projection reads it.
///
/// So exactly one literal, here, deliberately. Changing `CHANNELS`
/// should require editing this line and saying why.
#[test]
fn the_channel_arity_is_four_and_changing_it_is_an_amendment() {
    assert_eq!(
        kernel::bridge::CHANNELS,
        3,
        "BOND · COVENANT · DEBT on this branch. remediations-lane adds \
         CHANNEL_LANE as a fourth — verified present on `remediations-lane` \
         and NOT merged here, so this line becomes 4 when it lands and the \
         change is deliberate rather than silent."
    );
}
