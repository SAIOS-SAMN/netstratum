//! **THE CRAFTABLE TAX SYSTEM** (`49_TAXATION`, #54/#55). The levy model as
//! data, and the continuity tax — a demurrage on KNOWLEDGE keyed to
//! lifespan turnover: the Locust churns, the Custodian accumulates.

use cfe::dop::{positive, unit, ClosureEvent, Levy, LevyBase, LevySink, LevyTrigger, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB, SOVEREIGN};

/// A world sealed to a lifespan class (1 short-lived, 2 long-lived).
fn lineage(class: u8) -> World {
    let mut world = World::new();
    world.found("levy-hold").expect("founds");
    world.seal_taxon(class, 1).expect("seals the lifespan class");
    world
}

/// A world with a producing economy (an Extractor over a stocked bank),
/// ready to raise civic zones and drive a taxed collect.
fn producing_world() -> World {
    let mut world = World::new();
    world.found("tax-hold").expect("founds");
    world.commission_star(StarClass::G).expect("commissions star");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites the planet");
    let stamp = world.now();
    world.chronicle_mut().append(
        ClosureEvent::YieldCollected {
            orb_id: FIRST_ORB,
            resource: Resource::Biomass,
            amount: ratio(100_000, 1),
        },
        stamp,
        vec![SOVEREIGN],
    );
    let mut world = World::resume(world.chronicle().clone());
    world.zone(0, ZoneKind::Extractor).expect("builds an extractor");
    world
}

/// Drive `rounds` collect touches, each after 4 ticks of accrual.
fn run_collects(world: &mut World, rounds: usize) {
    for _ in 0..rounds {
        for _ in 0..4 {
            world.tick();
        }
        let _ = world.collect();
    }
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

// ── §5: the Sovereign Treasury Bank (the fiscal-loop keystone) ──

#[test]
fn the_treasury_deed_round_trips_both_directions() {
    for delta in [ratio(5, 1), ratio(-2, 1), ratio(3, 4), ratio(-3, 4)] {
        let deed = ClosureEvent::TreasurySettled { orb_id: 5, delta: delta.clone() };
        let bytes = deed.wire_bytes();
        let mut records = cfe::tlv::records(&bytes);
        let (tag, value) = records.next().expect("one record");
        assert_eq!(tag, 171, "TreasurySettled is tag 171");
        assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), deed);
    }
}

#[test]
fn a_fresh_treasury_reads_zero() {
    let world = lineage(2);
    assert_eq!(
        world.treasury_of(FIRST_ORB),
        ratio(0, 1),
        "no levy has ever been collected — the treasury is empty"
    );
}

#[test]
fn collected_levies_accumulate_and_welfare_draws_them_down_replay_safe() {
    let mut world = lineage(2);
    // Two collections accumulate into the reserve.
    world.settle_treasury(FIRST_ORB, ratio(4, 1));
    world.settle_treasury(FIRST_ORB, ratio(3, 1));
    assert_eq!(world.treasury_of(FIRST_ORB), ratio(7, 1), "collections accumulate");
    // A welfare spend draws the reserve down.
    world.settle_treasury(FIRST_ORB, ratio(-2, 1));
    assert_eq!(world.treasury_of(FIRST_ORB), ratio(5, 1), "welfare spends the reserve");
    // The whole history replays byte-exact and survives a resume.
    assert_eq!(replay(world.chronicle()), world.committed(), "treasury replay MATCHES");
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.treasury_of(FIRST_ORB), ratio(5, 1), "the balance survives a resume");
}

#[test]
fn a_spend_floors_the_treasury_at_zero_never_overdrawn() {
    let mut world = lineage(2);
    world.settle_treasury(FIRST_ORB, ratio(3, 1));
    // Spend more than stands — the treasury empties, it does not go negative.
    world.settle_treasury(FIRST_ORB, ratio(-10, 1));
    assert_eq!(world.treasury_of(FIRST_ORB), ratio(0, 1), "a spend cannot overdraw the treasury");
    assert_eq!(replay(world.chronicle()), world.committed(), "floored replay MATCHES");
}

#[test]
fn a_zero_delta_settles_nothing_so_a_taxless_world_is_byte_identical() {
    let mut taxed = lineage(2);
    let mut untaxed = lineage(2);
    // The baseline chronicle length before any treasury touch.
    let before = untaxed.chronicle().receipts().len();
    taxed.settle_treasury(FIRST_ORB, ratio(0, 1));
    untaxed.settle_treasury(FIRST_ORB, ratio(0, 1));
    assert_eq!(
        taxed.chronicle().receipts().len(),
        before,
        "a zero settle appends no receipt — the world that never taxes is untouched"
    );
    assert_eq!(taxed.committed(), untaxed.committed(), "byte-identical to a world that never called it");
}

// ── §5: levy collection at the touch (governance reach, GovernmentOffice) ──

#[test]
fn governance_reach_is_the_office_share_of_the_active_economy() {
    // No office: the fiscal apparatus reaches nothing.
    let mut world = producing_world();
    assert_eq!(world.governance_reach(FIRST_ORB), ratio(0, 1), "no office reaches nothing");
    // One office among two active zones (extractor + office) → half the
    // economy is within fiscal reach. Structural, derived from the
    // office/zone ratio — no dialed rate.
    world.zone(1, ZoneKind::GovernmentOffice).expect("builds a government office");
    assert_eq!(world.governance_reach(FIRST_ORB), ratio(1, 2), "one office among two zones reaches half");
    // A second office lifts the reach toward the whole economy.
    world.zone(2, ZoneKind::GovernmentOffice).expect("builds a second office");
    assert_eq!(world.governance_reach(FIRST_ORB), ratio(2, 3), "more civic build, more reach");
}

#[test]
fn a_touch_with_an_office_fills_the_treasury_by_reach_times_yield() {
    let mut gov = producing_world();
    gov.zone(1, ZoneKind::GovernmentOffice).expect("office");
    assert_eq!(gov.governance_reach(FIRST_ORB), ratio(1, 2), "reach is one half");
    // One collect: the extractor's measured 4-tick yield is 5; the levy
    // takes reach (1/2) of it → 5/2 into the treasury.
    run_collects(&mut gov, 1);
    assert_eq!(gov.treasury_of(FIRST_ORB), ratio(5, 2), "the touch collects reach × yield");
    // A second collect accumulates — the treasury is a standing reserve.
    run_collects(&mut gov, 1);
    assert_eq!(gov.treasury_of(FIRST_ORB), ratio(5, 1), "collections accumulate across touches");
    // The whole taxed history replays byte-exact.
    assert_eq!(replay(gov.chronicle()), gov.committed(), "taxed-collect replay MATCHES");
}

#[test]
fn a_touch_without_an_office_collects_no_levy_and_is_byte_identical() {
    // The same producing economy, but no GovernmentOffice: reach is zero,
    // so the collect emits NO TreasurySettled deed at all — a world that
    // never stands a civic office is byte-identical to the pre-taxation era.
    let mut bare = producing_world();
    run_collects(&mut bare, 2);
    assert_eq!(bare.treasury_of(FIRST_ORB), ratio(0, 1), "no office → an empty treasury");
    assert!(
        !bare
            .chronicle()
            .receipts()
            .iter()
            .any(|r| matches!(r.event, ClosureEvent::TreasurySettled { .. })),
        "no reach → no receipt: the untaxed collect is byte-identical"
    );
    assert_eq!(replay(bare.chronicle()), bare.committed(), "untaxed replay MATCHES");
}
