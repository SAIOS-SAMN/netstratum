//! **TREATIES (#42) — the political spectrum, proven.**
//!
//! Four kinds, one law. What separates them is not four bespoke
//! branches but the SIGN of what they bind and what they cost to hold:
//! trade and defense ease the crossing, war burdens and walls it, peace
//! binds nothing and bars war. A defense pact additionally spends the
//! option to attack.
//!
//! The pins that matter most are the ones about what a pact CANNOT do:
//! a peace cannot smuggle terms, a war cannot ease, a world sworn to
//! defend cannot declare war, and one signature moves nothing.

use cfe::dop::{positive, Resource};
use cfe::ratio::ratio;
use cfe::treaty::{TreatyFault, TreatyKind, TreatyTerms, PAIR_MELT};
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

fn eased() -> Vec<(u8, cfe::ratio::Ratio)> {
    vec![(PAIR_MELT, ratio(1, 2))]
}
fn burdened() -> Vec<(u8, cfe::ratio::Ratio)> {
    vec![(PAIR_MELT, ratio(2, 1))]
}

fn world(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world
}

#[test]
fn the_kind_is_the_sign_of_what_it_binds() {
    // CONCORD eases, DISCORD burdens, and a pact that contradicts its
    // own kind cannot be CONSTRUCTED — the door, not a check.
    assert!(TreatyTerms::new(TreatyKind::Trade, eased()).is_ok());
    assert!(TreatyTerms::new(TreatyKind::Defense, eased()).is_ok());
    assert!(TreatyTerms::new(TreatyKind::War, burdened()).is_ok());

    assert_eq!(
        TreatyTerms::new(TreatyKind::Trade, burdened()),
        Err(TreatyFault::TermsContradictKind { channel: PAIR_MELT }),
        "a trade pact that raises the melt is not a trade pact"
    );
    assert_eq!(
        TreatyTerms::new(TreatyKind::War, eased()),
        Err(TreatyFault::TermsContradictKind { channel: PAIR_MELT }),
        "a war that eases the crossing is not a war"
    );
}

#[test]
fn peace_binds_nothing_and_that_is_the_whole_point() {
    // A peace carrying terms would be a trade pact wearing peace's
    // name — and it would let a pair bar war for free.
    assert!(TreatyTerms::new(TreatyKind::Peace, vec![]).is_ok());
    assert_eq!(
        TreatyTerms::new(TreatyKind::Peace, eased()),
        Err(TreatyFault::PeaceBindsNothing)
    );
    // And every OTHER kind must bind something: a pact that changes
    // nothing between two worlds is not a pact.
    assert_eq!(TreatyTerms::new(TreatyKind::Trade, vec![]), Err(TreatyFault::TermsEmpty));
    assert_eq!(TreatyTerms::new(TreatyKind::War, vec![]), Err(TreatyFault::TermsEmpty));
    // The band is the same fourfold pair every chartered factor sits
    // inside — a treaty cannot walk a rate anywhere a class could not.
    assert_eq!(
        TreatyTerms::new(TreatyKind::Trade, vec![(PAIR_MELT, ratio(1, 100))]),
        Err(TreatyFault::FactorOutOfBounds { channel: PAIR_MELT })
    );
    // And only the PAIR table's channels: a treaty may not reach into
    // the world channels a class binds.
    assert_eq!(
        TreatyTerms::new(TreatyKind::Trade, vec![(99, ratio(1, 2))]),
        Err(TreatyFault::ChannelUnknown { channel: 99 })
    );
}

#[test]
fn peace_ends_a_war_for_free_and_war_breaks_a_peace_at_a_cost() {
    // THE DISPLACEMENT LAW, and its deliberate ASYMMETRY: a world may
    // always stop fighting for free, and never start for free.
    let mut here = world("tunborn");
    here.treaty(1, "farhold", TreatyTerms::new(TreatyKind::War, burdened()).expect("war"), 0)
        .expect("war is declared");
    assert!(here.at_war_with("farhold"));

    // PEACE over the war ends it, and nothing is owed for stopping.
    let load_before = here.orb().expect("stands").severance_load.clone();
    here.treaty(2, "farhold", TreatyTerms::new(TreatyKind::Peace, vec![]).expect("peace"), 0)
        .expect("peace is sealed");
    assert!(!here.at_war_with("farhold"), "the war is ended by the peace");
    assert_eq!(
        here.orb().expect("stands").severance_load,
        load_before,
        "stopping costs nothing"
    );

    // WAR over the peace BREAKS it, and the load posts to the breaker's
    // own entropic book — the same book a bond cut without notice does.
    here.treaty(3, "farhold", TreatyTerms::new(TreatyKind::War, burdened()).expect("war"), 0)
        .expect("war again");
    assert!(here.at_war_with("farhold"));
    assert!(
        here.orb().expect("stands").severance_load > load_before,
        "breaking a peace posts against the breaker"
    );
    assert_eq!(replay(here.chronicle()), here.committed(), "replay MATCHES");
}

#[test]
fn a_world_sworn_to_defend_cannot_declare_war() {
    // WHAT A DEFENSE PACT COSTS: it spends the option to attack. The
    // oath must be RENOUNCED first, and the chronicle then shows it was
    // broken deliberately rather than quietly.
    let mut here = world("tunborn");
    here.treaty(1, "farhold", TreatyTerms::new(TreatyKind::Defense, eased()).expect("pact"), 0)
        .expect("sworn");
    assert_eq!(
        here.treaty(2, "farhold", TreatyTerms::new(TreatyKind::War, burdened()).expect("war"), 0),
        Err(ActionRefused::SwornToDefend { partner: "farhold".into() })
    );
    // Renounce, and the option returns.
    here.renounce(1, "farhold").expect("the oath is renounced");
    here.treaty(2, "farhold", TreatyTerms::new(TreatyKind::War, burdened()).expect("war"), 0)
        .expect("now war may be declared");
    assert!(here.at_war_with("farhold"));
    assert_eq!(replay(here.chronicle()), here.committed(), "replay MATCHES");
}

#[test]
fn the_oath_and_the_war_bar_each_other_both_ways() {
    // FOUND BY PLAYING IT: the war bar alone let a world hold a defense
    // pact AND a war with the same partner — not a position, two
    // contradictory ones. Peace comes first, in either direction.
    let mut here = world("tunborn");
    here.treaty(1, "farhold", TreatyTerms::new(TreatyKind::War, burdened()).expect("war"), 0)
        .expect("war");
    assert_eq!(
        here.treaty(2, "farhold", TreatyTerms::new(TreatyKind::Defense, eased()).expect("d"), 0),
        Err(ActionRefused::AtWar { partner: "farhold".into() }),
        "you cannot swear to defend a world you are at war with"
    );
    // Peace ends the war, and THEN the oath may be sworn.
    here.treaty(3, "farhold", TreatyTerms::new(TreatyKind::Peace, vec![]).expect("p"), 0)
        .expect("peace");
    here.treaty(4, "farhold", TreatyTerms::new(TreatyKind::Defense, eased()).expect("d"), 0)
        .expect("and now the oath stands");
    assert_eq!(replay(here.chronicle()), here.committed(), "replay MATCHES");
}

#[test]
fn a_pact_needs_both_books_and_a_world_owns_only_its_own_leg() {
    let mut here = world("tunborn");
    let mut there = world("farhold");
    // One signature: the pact does not yet move the crossing.
    here.treaty(7, "farhold", TreatyTerms::new(TreatyKind::Trade, eased()).expect("trade"), 0)
        .expect("our leg");
    assert_eq!(here.pair_factor("farhold", PAIR_MELT), ratio(1, 2));
    assert_eq!(
        there.pair_factor("tunborn", PAIR_MELT),
        ratio(1, 1),
        "the partner's book is not ours to write"
    );
    // Both signatures: the pact stands on the pair.
    there
        .treaty(7, "tunborn", TreatyTerms::new(TreatyKind::Trade, eased()).expect("trade"), 0)
        .expect("their leg");
    assert_eq!(there.pair_factor("tunborn", PAIR_MELT), ratio(1, 2));

    // Either side may end its OWN leg without the other's leave — and
    // only its own.
    assert_eq!(here.renounce(99, "farhold"), Err(ActionRefused::NoSuchTreaty { deal: 99 }));
    here.renounce(7, "farhold").expect("ours ends");
    assert_eq!(here.pair_factor("farhold", PAIR_MELT), ratio(1, 1));
    assert_eq!(
        there.pair_factor("tunborn", PAIR_MELT),
        ratio(1, 2),
        "theirs still stands — a pact is not a decree"
    );
}

#[test]
fn one_live_pact_of_a_kind_and_never_with_oneself() {
    let mut here = world("tunborn");
    here.treaty(1, "farhold", TreatyTerms::new(TreatyKind::Trade, eased()).expect("trade"), 0)
        .expect("sealed");
    assert_eq!(
        here.treaty(2, "farhold", TreatyTerms::new(TreatyKind::Trade, eased()).expect("trade"), 0),
        Err(ActionRefused::TreatyAlreadyStands {
            partner: "farhold".into(),
            kind: TreatyKind::Trade.wire()
        }),
        "a second is ambiguous, not stronger — the factors multiply"
    );
    // A trade pact and a DEFENSE pact may both stand: different kinds,
    // and both ease.
    here.treaty(3, "farhold", TreatyTerms::new(TreatyKind::Defense, eased()).expect("pact"), 0)
        .expect("both stand");
    assert_eq!(
        here.pair_factor("farhold", PAIR_MELT),
        ratio(1, 4),
        "two easing pacts COMPOSE — allies trade cheapest of all"
    );
    assert_eq!(
        here.treaty(4, "tunborn", TreatyTerms::new(TreatyKind::Trade, eased()).expect("t"), 0),
        Err(ActionRefused::TreatyWithSelf)
    );
}

#[test]
fn a_pact_runs_out_by_the_clock_and_needs_no_receipt_to_stop() {
    let mut here = world("tunborn");
    here.treaty(1, "farhold", TreatyTerms::new(TreatyKind::Trade, eased()).expect("trade"), 4)
        .expect("sealed for four");
    assert_eq!(here.pair_factor("farhold", PAIR_MELT), ratio(1, 2));
    for _ in 0..6 {
        here.tick();
    }
    assert_eq!(
        here.pair_factor("farhold", PAIR_MELT),
        ratio(1, 1),
        "the term ran out — the bond's own shape, read from the clock"
    );
    assert!(here.treaties().is_empty(), "and it no longer stands");
    // A termless pact never lapses.
    let mut standing = world("tunborn");
    standing
        .treaty(1, "farhold", TreatyTerms::new(TreatyKind::Trade, eased()).expect("t"), 0)
        .expect("sealed standing");
    for _ in 0..50 {
        standing.tick();
    }
    assert_eq!(standing.pair_factor("farhold", PAIR_MELT), ratio(1, 2));
}

#[test]
fn the_wall_is_what_makes_war_expensive() {
    // WAR COSTS THE TRADE, not a number chosen to make it costly. A
    // pair at war exchanges nothing, in either direction, exactly as
    // the elder's ward bars a crossing.
    let mut here = world("tunborn");
    let mut there = world("farhold");
    for held in [&mut here, &mut there] {
        held.commission_star(cfe::thermal::StarClass::G).expect("commissions");
        held
            .site_planet(FIRST_ORB, cfe::dop::unit(0, 1), positive(1, 1), None, None)
            .expect("sites — and meeting its sun is the solar leap");
    }
    // Peaceful: the crossing carries.
    cfe::world::aperture_transfer(
        &mut here,
        FIRST_ORB,
        &mut there,
        FIRST_ORB,
        Resource::Iron,
        positive(1, 1),
        1,
    )
    .expect("the aperture carries");

    here.treaty(9, "farhold", TreatyTerms::new(TreatyKind::War, burdened()).expect("war"), 0)
        .expect("war is declared");
    assert_eq!(
        cfe::world::aperture_transfer(
            &mut here,
            FIRST_ORB,
            &mut there,
            FIRST_ORB,
            Resource::Iron,
            positive(1, 1),
            2,
        ),
        Err(ActionRefused::AtWar { partner: "farhold".into() }),
        "a pair at war exchanges nothing"
    );
    // AND ONE SIDE'S WAR IS ENOUGH: it takes two to trade and one to
    // refuse. The far world never signed anything.
    assert!(there.treaties().is_empty());
    assert_eq!(
        cfe::world::aperture_transfer(
            &mut there,
            FIRST_ORB,
            &mut here,
            FIRST_ORB,
            Resource::Iron,
            positive(1, 1),
            3,
        ),
        Err(ActionRefused::AtWar { partner: "tunborn".into() }),
        "the wall stands in both directions"
    );
    assert_eq!(replay(here.chronicle()), here.committed(), "replay MATCHES");
}

#[test]
fn the_pact_rides_the_wire_and_the_snapshot() {
    use cfe::dop::ClosureEvent;
    let deed = ClosureEvent::TreatySealed {
        deal: 7,
        kind: TreatyKind::Trade.wire(),
        partner: "farhold".into(),
        terms: vec![(PAIR_MELT, ratio(1, 2))],
        term: 40,
    };
    let bytes = deed.wire_bytes();
    let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
    assert_eq!(tag, 105);
    assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), deed);
    // A kind the vocabulary does not name refuses rather than guessing.
    let mut broken = value.to_vec();
    broken[8] = 99;
    assert_eq!(ClosureEvent::from_wire(105, &broken), None);

    let struck = ClosureEvent::TreatyStruck { deal: 7, partner: "farhold".into(), broken: true };
    let bytes = struck.wire_bytes();
    let (tag, value) = cfe::tlv::records(&bytes).next().expect("one record");
    assert_eq!(tag, 106);
    assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), struck);
}
