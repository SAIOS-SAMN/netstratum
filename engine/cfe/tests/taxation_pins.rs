//! **THE CRAFTABLE TAX SYSTEM** (`45_TAXATION`, #54/#55). The levy model as
//! data, and the continuity tax — a demurrage on KNOWLEDGE keyed to
//! lifespan turnover: the Locust churns, the Custodian accumulates.

use cfe::dop::{Levy, LevyBase, LevySink, LevyTrigger};
use cfe::ratio::ratio;
use cfe::world::World;

/// A world sealed to a lifespan class (1 short-lived, 2 long-lived).
fn lineage(class: u8) -> World {
    let mut world = World::new();
    world.found("levy-hold").expect("founds");
    world.seal_taxon(class, 1).expect("seals the lifespan class");
    world
}

#[test]
fn the_seeded_levies_name_their_base_trigger_and_sink() {
    // The continuity tax draws on KNOWLEDGE and lapses to the VOID; the
    // estate tax draws on the ESTATE and flows to the SOVEREIGN — both fire
    // at TURNOVER, so lifespan is their shared clock.
    let world = lineage(2);
    assert_eq!(
        world.continuity_levy(),
        Levy { base: LevyBase::Knowledge, trigger: LevyTrigger::Turnover, sink: LevySink::Void },
        "continuity: knowledge lapses to the void"
    );
    assert_eq!(
        world.estate_levy(),
        Levy { base: LevyBase::Estate, trigger: LevyTrigger::Turnover, sink: LevySink::Sovereign },
        "estate: the inheritance is taxed to the sovereign"
    );
}

#[test]
fn the_continuity_tax_falls_with_lifespan() {
    // A short-lived line (L = one grain) loses ALL its unconfirmed knowledge
    // each turnover — it must re-earn what the last generation knew. A
    // long-lived line loses only a quarter — its law compounds.
    let locust = lineage(1); // short-lived
    let custodian = lineage(2); // long-lived
    assert_eq!(locust.continuity_tax(), ratio(1, 1), "the Locust re-earns everything each generation");
    assert_eq!(custodian.continuity_tax(), ratio(1, 4), "the Custodian keeps three-quarters");
    assert!(
        locust.continuity_tax() > custodian.continuity_tax(),
        "endurance is the discount on the continuity tax"
    );
}
