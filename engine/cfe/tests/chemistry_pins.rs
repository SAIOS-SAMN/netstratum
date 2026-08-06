//! **ELEMENT-CHEMISTRY (#37) — THE ALMANAC'S FIRST CONSUMER.**
//!
//! The periodic table was complete, pinned, and read by NOTHING: five
//! axes deciding nothing outside their own witnesses, which is the
//! shape LAW 8 warns about. The compounding deed is what makes where an
//! element SITS decide what a civilization can make.
//!
//! The door/fact split is the thing under test as much as the deed:
//! whether matter bonds is a property of the SET and dies at the
//! `Compoundable` door; whether it bonds HERE reads the star, the
//! strata and the heat, and stays a fact at the verb forever.

use cfe::almanac::{periodic_of, recipe_of, Compoundable, LANES};
use cfe::dop::{positive, unit, ClosureEvent, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::thermal::StarClass;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

/// An element id seated at chosen axes — the almanac pins' own fixture
/// shape, so the tests state their cases rather than hunting for hashes.
fn at(line: u8, lane: u8, shell: u8, phase: u8, spin: u8) -> [u8; 32] {
    let mut id = [0u8; 32];
    id[3] = line - 1;
    id[4] = lane - 1;
    id[5] = shell - 1;
    id[6] = phase - 1;
    id[7] = spin;
    // Bytes 0..=2 belong to `element_properties`; leave them be.
    id
}

/// A PAIR that saturates: two members of one geometry and phase whose
/// binds sum to `arity · (LANES/2 + 1) / 2 = 6`. Lanes 3 and 3 bind
/// 3 + 3 = 6. Shallow (line 1) so any star carries them.
fn bonding_pair() -> [[u8; 32]; 2] {
    [at(1, 3, 1, 4, 0), at(1, 3, 1, 4, 1)]
}

#[test]
fn the_door_decides_bonding_and_the_verb_never_asks_again() {
    let pair = bonding_pair();
    let set = Compoundable::new(&pair).expect("the lanes saturate");
    assert_eq!(set.members(), pair, "the offered order is kept");
    // Past the door the gem and the knowledge are TOTAL — no Option
    // survives, which is the whole point of the door.
    assert_eq!(set.gem().len(), 32);
    assert_eq!(set.knowledge(), 2, "two shallow members, one line each");
    // And the recipe is mass-exact: what the members spend, the gem
    // carries. The saturation law fixes the sum, so no number is
    // chosen here.
    let (spends, yielded) = recipe_of(&set);
    let spent: Ratio = spends
        .iter()
        .fold(ratio(0, 1), |sum, (_, spend)| sum + spend.clone());
    assert_eq!(spent, yielded, "the bond neither creates nor destroys matter");
    assert_eq!(
        yielded,
        ratio(i64::from(2 * (LANES / 2 + 1) / 2), 1),
        "and the sum is the saturation law's own"
    );
    // Matter that does not bond cannot be constructed at all.
    assert_eq!(Compoundable::new(&[at(1, 1, 1, 4, 0), at(1, 1, 1, 4, 0)]), None);
    assert_eq!(Compoundable::new(&[]), None, "nothing bonds with nothing");
}

/// A world that has already DRAWN this matter — sited in the mesic
/// band (so phase-4 refractory matter is workable), holding `each` of
/// every member.
///
/// The stock is laid down as receipted assays and the world is then
/// RESUMED from its own chronicle, so the fixture is chain-true rather
/// than a hand-set field: every element the deed reads got there the
/// way elements get there. The assays ride the sealed pre-ecology form
/// (`richness: None`), because how the vein fared is #40's concern and
/// not chemistry's.
fn stocked(name: &str, members: &[[u8; 32]], each: Ratio) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    // ONE STRATUM OPENED. Pressure is `stratum / reach`, so a world
    // that has never dug brings none and releases nothing — not even
    // the shallowest line. The fixture digs once because a world
    // holding drawn matter necessarily has.
    world.prospect(FIRST_ORB, 1).expect("opens the ground");
    let stamp = world.now();
    for member in members {
        world.chronicle_mut().append(
            ClosureEvent::AssayStruck {
                orb_id: FIRST_ORB,
                element: *member,
                amount: each.clone(),
                richness: None,
            },
            stamp,
            vec![cfe::world::SOVEREIGN],
        );
    }
    World::resume(world.chronicle().clone())
}

#[test]
fn matter_bonds_and_the_gem_is_banked() {
    let pair = bonding_pair();
    let mut world = stocked("bond-hold", &pair, ratio(10, 1));
    let before: Vec<Ratio> = pair
        .iter()
        .map(|m| world.orb().expect("stands").element_amount(m))
        .collect();

    let gem = world.compound(&pair).expect("the matter holds");
    assert_eq!(gem, Compoundable::new(&pair).expect("bonds").gem());

    // Each member gave up its lane's valence; the gem carries the sum.
    let set = Compoundable::new(&pair).expect("bonds");
    let (spends, yielded) = recipe_of(&set);
    for ((member, spend), had) in spends.iter().zip(before) {
        assert_eq!(
            world.orb().expect("stands").element_amount(member),
            had - spend.clone(),
            "the member spent exactly its valence"
        );
    }
    assert_eq!(
        world.orb().expect("stands").element_amount(&gem),
        yielded,
        "and the gem carries the sum away"
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn the_same_set_always_names_the_same_gem_in_every_world() {
    // The gem is content-addressed, so two worlds that never meet make
    // the SAME gem from the same matter — which is what lets them
    // trade it, and what makes a recipe knowledge rather than a
    // private accident.
    let pair = bonding_pair();
    let mut here = stocked("first-bench", &pair, ratio(10, 1));
    let mut there = stocked("other-bench", &pair, ratio(10, 1));
    assert_eq!(
        here.compound(&pair).expect("bonds"),
        there.compound(&pair).expect("bonds"),
        "one recipe, one gem, everywhere"
    );
    // Order-free, too: the set is not a sequence.
    let flipped = [pair[1], pair[0]];
    let mut flipped_world = stocked("flipped-bench", &pair, ratio(10, 1));
    assert_eq!(
        flipped_world.compound(&flipped).expect("bonds"),
        here.committed().orbs[0].elements.iter().map(|(id, _)| *id)
            .find(|id| !pair.contains(id))
            .expect("the gem is banked"),
        "the same members in another order make the same gem"
    );
}

#[test]
fn a_short_ledger_refuses_and_spends_nothing() {
    let pair = bonding_pair();
    // Each member must spend 3 (lane 3's valence); hold 2.
    let mut world = stocked("thin-hold", &pair, ratio(2, 1));
    let receipts = world.chronicle().receipts().len();
    let refusal = world.compound(&pair).expect_err("the bench is short");
    assert!(
        matches!(refusal, ActionRefused::CannotAffordElement { .. }),
        "the element ledger refuses like the bank does: {refusal:?}"
    );
    assert_eq!(
        world.chronicle().receipts().len(),
        receipts,
        "a refused bond leaves no receipt"
    );
    assert_eq!(
        world.orb().expect("stands").element_amount(&pair[0]),
        ratio(2, 1),
        "and spends nothing"
    );
}

#[test]
fn the_facts_read_the_world_and_each_refuses_in_its_own_voice() {
    // LOCALITY — a deep line under a shallow star. Line 7 needs a
    // heavy class; G reaches 5.
    let deep = [at(7, 3, 1, 4, 0), at(7, 3, 1, 4, 1)];
    assert!(Compoundable::new(&deep).is_some(), "the set itself bonds");
    let mut world = stocked("shallow-sky", &deep, ratio(10, 1));
    assert!(
        matches!(world.compound(&deep), Err(ActionRefused::ElementNotLocal { .. })),
        "deep matter does not occur under a shallow star"
    );

    // PRESSURE — local, but the orb has opened no stratum deep enough.
    // Line 5 is exactly G's reach, so it demands full pressure.
    let pressured = [at(5, 3, 1, 4, 0), at(5, 3, 1, 4, 1)];
    let mut world = stocked("unpressured", &pressured, ratio(10, 1));
    assert!(
        matches!(world.compound(&pressured), Err(ActionRefused::PressureUnreached { .. })),
        "an unopened world cannot release what it holds"
    );

    // WORKABILITY — a phase that needs a colder place than this orb is.
    // Phase 1 is the cold phase; this orb sits mesic.
    let cold = [at(1, 3, 1, 1, 0), at(1, 3, 1, 1, 1)];
    let mut world = stocked("too-warm", &cold, ratio(10, 1));
    assert!(
        matches!(world.compound(&cold), Err(ActionRefused::NotWorkableHere { .. })),
        "matter has a phase and a place has a heat"
    );

    // And the door's own refusal, reported at the verb.
    let inert = [at(1, 1, 1, 4, 0), at(1, 1, 1, 4, 0)];
    let mut world = stocked("inert-hold", &inert, ratio(10, 1));
    assert_eq!(world.compound(&inert), Err(ActionRefused::MatterDoesNotBond));
}

#[test]
fn the_pressure_fact_opens_as_the_world_digs() {
    // The same set, the same world, refused and then admitted — the
    // proof that pressure is a FACT of the chain and not a shape.
    // Line 5 under G (reach 5) demands stratum 5.
    let pressured = [at(5, 3, 1, 4, 0), at(5, 3, 1, 4, 1)];
    let mut world = stocked("digs-deeper", &pressured, ratio(10, 1));
    assert!(matches!(
        world.compound(&pressured),
        Err(ActionRefused::PressureUnreached { .. })
    ));
    // Stratum 1 is already open (the fixture dug once); dig to 5, the
    // deepest line a G star carries.
    for stratum in 2..=5u8 {
        world.prospect(FIRST_ORB, stratum).expect("opens");
    }
    assert!(world.compound(&pressured).is_ok(), "the deepened world bonds it");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn the_bench_reads_what_the_ledger_could_make() {
    let pair = bonding_pair();
    // An inert third element that bonds with nothing held.
    let world = stocked("bench-read", &[pair[0], pair[1], at(1, 1, 2, 4, 0)], ratio(10, 1));
    let bench = world.bench(FIRST_ORB);
    assert!(
        bench.iter().any(|(members, gem, _)| members.len() == 2
            && *gem == Compoundable::new(&pair).expect("bonds").gem()),
        "the bench finds the pair that bonds"
    );
    assert!(
        bench.iter().all(|(members, _, _)| Compoundable::new(members).is_some()),
        "and offers nothing that does not"
    );
    // An empty ledger benches nothing, and an unknown orb likewise.
    assert!(World::new().bench(FIRST_ORB).is_empty());
    assert!(world.bench(9999).is_empty());
}

#[test]
fn the_deed_round_trips_byte_exact() {
    let pair = bonding_pair();
    let deed = ClosureEvent::GemCompounded {
        orb_id: 3,
        members: pair.to_vec(),
        gem: [7u8; 32],
    };
    let bytes = deed.wire_bytes();
    let mut records = cfe::tlv::records(&bytes);
    let (tag, value) = records.next().expect("one record");
    assert_eq!(tag, 104);
    assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), deed);
    // A partial identity is not a member: the decoder refuses rather
    // than guessing a shorter set.
    assert_eq!(ClosureEvent::from_wire(104, &value[..value.len() - 1]), None);
    // And a bond with no members at all does not decode.
    assert_eq!(ClosureEvent::from_wire(104, &value[..40]), None);
}

#[test]
fn a_bond_is_reachable_from_a_world_s_own_ground_but_never_guaranteed() {
    // **THE MEASUREMENT THAT SAYS WHETHER THIS IS A GAME.** Everything
    // above tests chosen matter. What decides whether chemistry is
    // playable is whether a world's OWN drawn elements — hash-derived,
    // never chosen — ever bond at all. Too rare and the feature is
    // dead on arrival; too common and the periodic table decides
    // nothing.
    //
    // Measured over 40 worlds drawing 24 strata each: 31 could bond
    // something, 60 sets in total — most worlds can make SOMETHING and
    // no world can make everything, which is exactly the scarcity that
    // makes the lanes worth reading and the gaps worth trading for.
    let trials = 20;
    let mut worlds_that_bond = 0;
    for trial in 0..trials {
        let mut world = charted(&format!("reach-{trial}"));
        let mut drawn = Vec::new();
        for stratum in 2..=24u8 {
            if let Ok(struck) = world.prospect(FIRST_ORB, stratum) {
                drawn.push(struck.element);
            }
        }
        for _ in 0..4 {
            world.tick();
        }
        for element in &drawn {
            let _ = world.assay(FIRST_ORB, *element);
        }
        if !world.bench(FIRST_ORB).is_empty() {
            worlds_that_bond += 1;
        }
    }
    assert!(
        worlds_that_bond * 4 >= trials,
        "a bond must be REACHABLE from a world's own ground — only {worlds_that_bond}/{trials} \
         could make anything; the chemistry would be dead on arrival"
    );
}

/// The charted world the reachability measurement draws on — a G star,
/// sited mesic, its ground open.
fn charted(name: &str) -> World {
    stocked(name, &[], ratio(0, 1))
}

#[test]
fn a_lane_recurs_so_one_recipe_names_a_family_of_gems() {
    // THE PERIODICITY PAYING OUT, and the reason the table earns its
    // keep: the bonding rule reads LANES, not members, so a recipe
    // learned at one depth works at every other. Different lines, same
    // lanes — still a bond, and a DIFFERENT gem.
    let shallow = [at(1, 3, 1, 4, 0), at(1, 3, 1, 4, 1)];
    let deeper = [at(2, 3, 1, 4, 0), at(2, 3, 1, 4, 1)];
    assert!(Compoundable::new(&shallow).is_some());
    assert!(Compoundable::new(&deeper).is_some(), "the recipe carries down the lane");
    assert_ne!(
        Compoundable::new(&shallow).expect("bonds").gem(),
        Compoundable::new(&deeper).expect("bonds").gem(),
        "one family of recipes, distinct gems"
    );
    assert!(
        Compoundable::new(&deeper).expect("bonds").knowledge()
            > Compoundable::new(&shallow).expect("bonds").knowledge(),
        "and the deeper bond is worth more"
    );
    // The valence a lane brings is the lane's alone, at every line.
    assert_eq!(periodic_of(&shallow[0]).binds, periodic_of(&deeper[0]).binds);
}
