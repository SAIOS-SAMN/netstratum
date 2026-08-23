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

/// A lifespan-classed line with a fed population, habitats + granary (so a
/// collect always settles), Iron stocked (so a civic office stays lit),
/// and optional practices adopted and a GovernmentOffice. `class` 1 is
/// short-lived (L=64), 2 long-lived (L=256).
fn generation_line(class: u8, office: bool, practices: &[u8]) -> World {
    let mut world = World::new();
    world.found("turnover-hold").expect("founds");
    world.commission_star(StarClass::G).expect("star");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites");
    let stamp = world.now();
    for res in [Resource::Biomass, Resource::Iron] {
        world.chronicle_mut().append(
            ClosureEvent::YieldCollected { orb_id: FIRST_ORB, resource: res, amount: ratio(1_000_000, 1) },
            stamp,
            vec![SOVEREIGN],
        );
    }
    let mut world = World::resume(world.chronicle().clone());
    for tile in 0..6 {
        world.zone(tile, ZoneKind::Habitat).expect("habitat");
    }
    world.zone(50, ZoneKind::Granary).expect("granary");
    if office {
        world.zone(20, ZoneKind::GovernmentOffice).expect("office");
    }
    world.seal_taxon(class, 1).expect("seals the lifespan class");
    for p in practices {
        world.adopt_practice(*p).expect("adopts a practice");
    }
    world.settle_population(FIRST_ORB, 100);
    world
}

/// The estate levy's setting: a short-lived line (L=64), fed, no practices.
fn generation_world(office: bool) -> World {
    generation_line(1, office, &[])
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
            .any(|r| matches!(r.event(), ClosureEvent::TreasurySettled { .. })),
        "no reach → no receipt: the untaxed collect is byte-identical"
    );
    assert_eq!(replay(bare.chronicle()), bare.committed(), "untaxed replay MATCHES");
}

// ── §4: the estate tax at the generation turnover (population-proportional) ──

#[test]
fn the_estate_tax_is_population_proportional_and_office_gated() {
    let world = generation_world(true);
    let pop = world.population_of(FIRST_ORB);
    assert!(pop > 0, "the line stands");
    // No boundary crossed → nothing due.
    assert_eq!(world.estate_due_at_touch(FIRST_ORB, 0), ratio(0, 1), "no turnover, no estate tax");
    // One generation: estate_tax × (population × per_capita) — the estate is
    // the generation's population-proportional worth (ruled 2026-08-13).
    let estate = ratio(pop as i64, 1) * world.per_capita_ration();
    let one = world.estate_tax() * estate;
    assert_eq!(world.estate_due_at_touch(FIRST_ORB, 1), one, "one turnover taxes the estate once");
    // Several boundaries in one span settle together — linear in the count.
    assert_eq!(
        world.estate_due_at_touch(FIRST_ORB, 3),
        one.clone() + one.clone() + one,
        "three turnovers settle at once"
    );
    // No office → uncollected, whatever the boundaries crossed.
    let bare = generation_world(false);
    assert_eq!(bare.estate_due_at_touch(FIRST_ORB, 5), ratio(0, 1), "no office → the estate tax slips");
}

#[test]
fn the_estate_tax_lands_only_at_the_generation_boundary() {
    // The estate tax is a DISCRETE jump at each generation turnover (L=64
    // beats). Since COG Slice 2c's wild-flow-first law, the founding-scale
    // population gathers and never taxes the granary, so the office world's
    // cultivated biomass yield stands TAXABLE — a steady per-touch revenue levy
    // trickles into the treasury between boundaries. The estate tax is then
    // isolated as the EXCESS: the treasury's increment across a span that
    // crosses a generation boundary exceeds its increment across an equal span
    // that does not. Flat-rate revenue within a generation; a leap at the turn.
    let mut gov = generation_world(true);
    // Walk collect-by-collect, recording each round's treasury increment and
    // whether that round CROSSED a generation boundary (its clock stepped over
    // a multiple of L). The steady revenue levy rises gently with the growing
    // population, so we isolate the estate tax LOCALLY: a boundary round's jump
    // against its immediate neighbour, which carries near-identical population
    // (hence near-identical revenue) but no turnover.
    let lifespan = gov.lifespan_of().max(1);
    let mut deltas: Vec<(cfe::ratio::Ratio, bool)> = Vec::new();
    let mut prev_treasury = gov.treasury_of(FIRST_ORB);
    let mut prev_beat = gov.now();
    for _ in 0..34 {
        run_collects(&mut gov, 1); // one collect round (≈4 beats)
        let now_treasury = gov.treasury_of(FIRST_ORB);
        let now_beat = gov.now();
        let crossed = now_beat / lifespan > prev_beat / lifespan;
        deltas.push((now_treasury.clone() - prev_treasury.clone(), crossed));
        prev_treasury = now_treasury;
        prev_beat = now_beat;
    }
    // At L=64 over ~136 beats the walk turns over TWICE (beats 64 and 128).
    let boundaries: Vec<usize> = deltas
        .iter()
        .enumerate()
        .filter_map(|(i, (_, crossed))| crossed.then_some(i))
        .collect();
    assert_eq!(boundaries.len(), 2, "two generation boundaries crossed: {boundaries:?}");
    for &i in &boundaries {
        // Off a boundary the treasury still climbs (the revenue levy is live),
        // so the neighbour delta is positive — the estate tax is the EXCESS the
        // turnover round carries over the very next, boundary-free round.
        let here = &deltas[i].0;
        let neighbour = &deltas[i + 1].0;
        assert!(!deltas[i + 1].1, "the round after a turnover holds no second boundary");
        assert!(
            here > neighbour,
            "the estate tax makes the turnover round ({here}) jump past its \
             boundary-free neighbour ({neighbour}) — the discrete generation levy"
        );
    }
    // And OFF the boundaries the reserve only ever rises (revenue), never a
    // spurious estate jump — every interior round is a gentle climb.
    assert!(
        deltas.iter().filter(|(_, c)| !c).all(|(d, _)| *d >= ratio(0, 1)),
        "interior rounds carry only the steady revenue levy, never a phantom levy"
    );
    assert_eq!(replay(gov.chronicle()), gov.committed(), "estate-tax replay MATCHES");
}

#[test]
fn a_generation_turns_without_an_office_and_the_treasury_stays_empty() {
    // The mirror: the same short-lived line turning over generation after
    // generation, but no GovernmentOffice — the estate slips uncollected,
    // so the treasury never fills and the world is byte-identical.
    let mut bare = generation_world(false);
    run_collects(&mut bare, 40); // ≈160 beats — two generations turned
    assert_eq!(bare.treasury_of(FIRST_ORB), ratio(0, 1), "no office → the estate tax collects nothing");
    assert!(
        !bare
            .chronicle()
            .receipts()
            .iter()
            .any(|r| matches!(r.event(), ClosureEvent::TreasurySettled { .. })),
        "no reach, no boundary receipt: byte-identical to the pre-taxation era"
    );
}

// ── §3: the continuity levy — practices lapse to the void at turnover ──

#[test]
fn the_continuity_deed_round_trips() {
    for count in [0u8, 1, 7, 255] {
        let deed = ClosureEvent::PracticesLapsed { count };
        let bytes = deed.wire_bytes();
        let mut records = cfe::tlv::records(&bytes);
        let (tag, value) = records.next().expect("one record");
        assert_eq!(tag, 172, "PracticesLapsed is tag 172");
        assert_eq!(ClosureEvent::from_wire(tag, value).expect("decodes"), deed);
    }
}

#[test]
fn the_continuity_lapse_falls_with_lifespan() {
    // The short-lived Locust (rate 1) forgets ALL four practices at a
    // turnover; the long-lived Custodian (rate 1/4) loses only one.
    let locust = generation_line(1, false, &[1, 2, 3, 4]);
    let custodian = generation_line(2, false, &[1, 2, 3, 4]);
    assert_eq!(locust.practice_count(), 4, "the line adopted four");
    assert_eq!(locust.continuity_lapse_at_touch(1), 4, "the Locust re-earns everything each generation");
    assert_eq!(custodian.continuity_lapse_at_touch(1), 1, "the Custodian keeps three of four");
    // No turnover, no lapse; no sealed lifespan class, no lapse.
    assert_eq!(locust.continuity_lapse_at_touch(0), 0, "no turnover, nothing lapses");
    let mut unsealed = World::new();
    unsealed.found("x").expect("founds");
    unsealed.adopt_practice(1).expect("adopts");
    assert_eq!(
        unsealed.continuity_lapse_at_touch(5),
        0,
        "a line with no sealed lifespan class never turns over → byte-identical"
    );
}

#[test]
fn a_short_lived_line_forgets_its_practices_at_the_turnover() {
    // The Locust adopts four practices; crossing its 64-beat generation
    // boundary lapses them all to the void. Replay drops the same.
    let mut locust = generation_line(1, false, &[1, 2, 3, 4]);
    assert_eq!(locust.practice_count(), 4, "four practices stand");
    // Within the first generation: nothing lapses yet.
    run_collects(&mut locust, 10); // now ≈ 40 (< 64)
    assert_eq!(locust.practice_count(), 4, "no turnover yet — knowledge held");
    // Cross the 64-beat boundary: the generation turns, all practices lapse.
    run_collects(&mut locust, 10); // now ≈ 80 (crossed 64)
    assert_eq!(locust.practice_count(), 0, "the generation turned — its unkept knowledge lapsed");
    assert!(
        locust
            .chronicle()
            .receipts()
            .iter()
            .any(|r| matches!(r.event(), ClosureEvent::PracticesLapsed { .. })),
        "the lapse is receipted"
    );
    assert_eq!(replay(locust.chronicle()), locust.committed(), "continuity-lapse replay MATCHES");
}

#[test]
fn a_line_with_no_practices_or_no_lifespan_class_never_lapses() {
    // A sealed line that adopted no practice emits no lapse deed.
    let mut no_practice = generation_line(1, false, &[]);
    run_collects(&mut no_practice, 30); // ≈120 beats — a generation turned
    assert!(
        !no_practice
            .chronicle()
            .receipts()
            .iter()
            .any(|r| matches!(r.event(), ClosureEvent::PracticesLapsed { .. })),
        "no practices → no lapse deed → byte-identical"
    );
}
