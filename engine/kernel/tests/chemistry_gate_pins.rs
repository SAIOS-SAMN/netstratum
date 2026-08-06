//! **ELEMENT-CHEMISTRY AT THE GATE (#37, kernel 0.17.0).** Red-first.
//!
//! A bond moves MANY element keys in one deed — each member spends its
//! lane's valence, the gem carries away the sum — so the element
//! declaration widens from one move to many. The gate trusts none of
//! those numbers: it re-derives the recipe AND the gem's identity from
//! the members the receipt names, by calling the emitter's own law.

use cfe::almanac::{recipe_of, Compoundable};
use cfe::dop::{ClosureEvent, ColonyOrb};
use cfe::ipsal::Entity;
use cfe::ratio::{ratio, Ratio};
use cfe::world::{Committed, World, SOVEREIGN};
use kernel::{admits, Refused, Step};

fn at(line: u8, lane: u8, shell: u8, phase: u8, spin: u8) -> [u8; 32] {
    let mut id = [0u8; 32];
    id[3] = line - 1;
    id[4] = lane - 1;
    id[5] = shell - 1;
    id[6] = phase - 1;
    id[7] = spin;
    id
}

/// Two members of one lane whose binds saturate a two-body geometry.
fn pair() -> [[u8; 32]; 2] {
    [at(1, 3, 1, 4, 0), at(1, 3, 1, 4, 1)]
}

/// A seated one-orb state holding `each` of every named element.
fn holding(members: &[[u8; 32]], each: Ratio) -> Committed {
    let mut committed = World::new().committed();
    let mut orb = ColonyOrb { orb_id: 1, name: "bench".into(), ..Default::default() };
    for member in members {
        orb.bank_element(*member, each.clone());
    }
    committed.orbs = vec![orb];
    committed.entities = vec![Entity {
        entity_id: SOVEREIGN,
        handle: "sovereign".into(),
        principal: None,
        grants: vec![],
    }];
    committed
}

/// The state a lawful bond produces, and the step that declares it.
fn bonded(prior: &Committed, members: &[[u8; 32]]) -> (Committed, Step) {
    let set = Compoundable::new(members).expect("bonds");
    let (spends, yielded) = recipe_of(&set);
    let mut next = prior.clone();
    for (element, spend) in &spends {
        next.orbs[0].bank_element(*element, -spend.clone());
    }
    next.orbs[0].bank_element(set.gem(), yielded);
    let step = Step {
        event: ClosureEvent::GemCompounded {
            orb_id: 1,
            members: members.to_vec(),
            gem: set.gem(),
        },
        actor: SOVEREIGN,
        stamp: 4,
    };
    (next, step)
}

#[test]
fn an_honest_bond_spends_the_valences_and_mints_the_sum() {
    let members = pair();
    let prior = holding(&members, ratio(10, 1));
    let (next, step) = bonded(&prior, &members);
    assert_eq!(admits(&prior, &next, &step), Ok(()));
}

#[test]
fn a_bond_that_underspends_a_member_refuses() {
    // The gate re-derives what each member owes. A world that banks
    // the gem while keeping one member's valence refuses, with the
    // element and both totals carried whole.
    let members = pair();
    let prior = holding(&members, ratio(10, 1));
    let (honest, step) = bonded(&prior, &members);
    let set = Compoundable::new(&members).expect("bonds");
    let (spends, _) = recipe_of(&set);
    let mut greedy = honest;
    // Put the first member's spend back.
    greedy.orbs[0].bank_element(spends[0].0, spends[0].1.clone());
    assert!(
        matches!(
            admits(&prior, &greedy, &step),
            Err(Refused::ElementBroken { element, .. }) if element == spends[0].0
        ),
        "a bond spends every valence it declares"
    );
}

#[test]
fn a_bond_that_overmints_the_gem_refuses() {
    let members = pair();
    let prior = holding(&members, ratio(10, 1));
    let (honest, step) = bonded(&prior, &members);
    let gem = Compoundable::new(&members).expect("bonds").gem();
    let mut inflated = honest;
    inflated.orbs[0].bank_element(gem, ratio(1, 1));
    assert!(
        matches!(
            admits(&prior, &inflated, &step),
            Err(Refused::ElementBroken { element, .. }) if element == gem
        ),
        "the gem carries the sum and not one grain more"
    );
}

#[test]
fn a_gem_the_members_do_not_make_refuses() {
    // The gem is content-addressed, so the gate re-derives it. A
    // receipt naming any other identity refuses — a compounding
    // cannot mint a name it did not earn.
    let members = pair();
    let prior = holding(&members, ratio(10, 1));
    let (mut next, _) = bonded(&prior, &members);
    let honest = Compoundable::new(&members).expect("bonds").gem();
    let invented = [42u8; 32];
    // Bank the invented gem instead, so only the identity is at issue.
    let (_, yielded) = recipe_of(&Compoundable::new(&members).expect("bonds"));
    next.orbs[0].bank_element(honest, -yielded.clone());
    next.orbs[0].bank_element(invented, yielded);
    let step = Step {
        event: ClosureEvent::GemCompounded {
            orb_id: 1,
            members: members.to_vec(),
            gem: invented,
        },
        actor: SOVEREIGN,
        stamp: 4,
    };
    assert_eq!(
        admits(&prior, &next, &step),
        Err(Refused::GemMisidentified {
            expected: Box::new(honest),
            declared: Box::new(invented),
        }),
        "the gem's name is derived, never declared"
    );
}

#[test]
fn matter_that_does_not_bond_declares_nothing_and_may_move_nothing() {
    // Lane 1 binds 1 apiece: two of them sum to 2, not the 6 a
    // two-body geometry saturates at.
    let inert = [at(1, 1, 1, 4, 0), at(1, 1, 1, 4, 1)];
    let prior = holding(&inert, ratio(10, 1));
    let step = Step {
        event: ClosureEvent::GemCompounded {
            orb_id: 1,
            members: inert.to_vec(),
            gem: [9u8; 32],
        },
        actor: SOVEREIGN,
        stamp: 4,
    };
    assert_eq!(
        admits(&prior, &prior.clone(), &step),
        Err(Refused::MatterDoesNotBond),
        "matter that does not compound may make no move at all"
    );
}

#[test]
fn a_repeated_member_spends_twice() {
    // THE HAZARD THE WIDENING INTRODUCED. A set may hold two elements
    // of one lane — and if it holds the SAME element twice, that key
    // is declared twice. The gate SUMS repeated declarations; taking
    // only the first match would let the second spend vanish and mint
    // matter out of nothing.
    //
    // One element, offered twice: its lane's binds sum to the
    // saturation, so the set bonds and the key is declared twice.
    let twin = at(1, 3, 1, 4, 0);
    let members = [twin, twin];
    let set = Compoundable::new(&members).expect("one lane, twice, saturates");
    let (spends, yielded) = recipe_of(&set);
    assert_eq!(spends.len(), 2, "the key is declared once per member");
    assert_eq!(spends[0].0, spends[1].0, "and both declarations are the same key");

    let prior = holding(&[twin], ratio(10, 1));
    let mut next = prior.clone();
    // The HONEST effect: both spends land.
    next.orbs[0].bank_element(twin, -(spends[0].1.clone() + spends[1].1.clone()));
    next.orbs[0].bank_element(set.gem(), yielded.clone());
    let step = Step {
        event: ClosureEvent::GemCompounded {
            orb_id: 1,
            members: members.to_vec(),
            gem: set.gem(),
        },
        actor: SOVEREIGN,
        stamp: 4,
    };
    assert_eq!(admits(&prior, &next, &step), Ok(()));

    // And spending it only ONCE refuses — the arithmetic the summing
    // fold exists to catch.
    let mut half = prior.clone();
    half.orbs[0].bank_element(twin, -spends[0].1.clone());
    half.orbs[0].bank_element(set.gem(), yielded);
    assert!(
        matches!(
            admits(&prior, &half, &step),
            Err(Refused::ElementBroken { element, .. }) if element == twin
        ),
        "a member offered twice is spent twice"
    );
}

#[test]
fn the_sealed_assay_still_judges_exactly_as_it_did() {
    // The declaration widened from one move to many. The assay's arm
    // rides the SAME widened machinery, so its old verdicts are pinned
    // here beside the new ones: a smuggled element under a neutral
    // deed still refuses ElementBroken with the same numbers.
    let element = [3u8; 32];
    let prior = holding(&[element], ratio(6, 1));
    let mut smuggled = prior.clone();
    smuggled.orbs[0].bank_element(element, ratio(6, 1));
    let neutral = Step {
        event: ClosureEvent::OverheatCleared { orb_id: 1 },
        actor: 999,
        stamp: 4,
    };
    assert_eq!(
        admits(&prior, &smuggled, &neutral),
        Err(Refused::ElementBroken {
            element,
            expected: Box::new(ratio(6, 1)),
            actual: Box::new(ratio(12, 1)),
        }),
        "one move or many, an undeclared element still refuses"
    );
}
