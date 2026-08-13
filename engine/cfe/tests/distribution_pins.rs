//! **THE PROVISIONING LAYER — the granary gate** (`43_DISTRIBUTION`,
//! task #45). The granary is a buildable infrastructure zone whose
//! PRESENCE flips the provisioned economy on. It holds no stock of its
//! own (the depot is the orb's banked ration, ruled) and yields nothing —
//! it is the law's unlock, the REGEN-1 precedent. These witnesses pin the
//! gate: it builds like a habitat, produces nothing, and `has_granary`
//! reads it only while it stands active.

use cfe::dop::{positive, unit, ClosureEvent, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB, SOVEREIGN};

/// A mesic world with biomass banked to build on — enough to raise a
/// granary and still stand a ration.
fn provisioned_world() -> World {
    let mut world = World::new();
    world.found("granary-hold").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites in the mesic band");
    let stamp = world.now();
    world.chronicle_mut().append(
        ClosureEvent::YieldCollected {
            orb_id: FIRST_ORB,
            resource: Resource::Biomass,
            amount: ratio(100, 1),
        },
        stamp,
        vec![SOVEREIGN],
    );
    World::resume(world.chronicle().clone())
}

#[test]
fn the_granary_rides_the_habitat_bill_and_produces_nothing() {
    // Its economics DERIVE from the habitat (a ration house): same cost,
    // same upkeep line; and it yields nothing — infrastructure, not a
    // producer.
    assert_eq!(
        ZoneKind::Granary.cost(),
        ZoneKind::Habitat.cost(),
        "a granary costs the founding ration bill, like the habitat"
    );
    assert_eq!(
        ZoneKind::Granary.yield_per_tick().1,
        ratio(0, 1),
        "a granary produces nothing — it is the provisioning law's unlock"
    );
    assert_eq!(
        ZoneKind::Granary.upkeep_per_tick(),
        ZoneKind::Habitat.upkeep_per_tick(),
        "and is maintained like the ration house it is"
    );
}

#[test]
fn the_granary_wire_code_round_trips_and_is_not_minted() {
    // Code 11, the next compiled code past the ship-yard — a compiled
    // kind, never read as a minted one.
    assert_eq!(ZoneKind::Granary.wire(), 11, "the granary is compiled code 11");
    assert_eq!(ZoneKind::from_wire(11), Some(ZoneKind::Granary), "11 decodes to the granary");
    assert!(ZoneKind::Granary.compiled(), "it is a compiled kind, not minted");
    // The first minted code now floats one higher, above the granary.
    assert!(
        matches!(ZoneKind::from_wire(12), Some(ZoneKind::Minted(12))),
        "minted zone codes float above the compiled table"
    );
}

#[test]
fn has_granary_reads_the_gate_and_replay_matches() {
    let mut world = provisioned_world();
    assert!(!world.has_granary(FIRST_ORB), "a bare world stands no granary");
    world.zone(0, ZoneKind::Granary).expect("builds a granary like any zone");
    assert!(world.has_granary(FIRST_ORB), "the built granary flips the gate on");
    // The gate is derived from committed state — a fresh resume reads it
    // identically, and the chronicle replays byte-exact.
    let resumed = World::resume(world.chronicle().clone());
    assert!(resumed.has_granary(FIRST_ORB), "the gate survives a resume");
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

// ── L2+L3: THE PROVISIONED CONSUMPTION CYCLE (§2–4) ──

/// A mesic world stocked to run: biomass to eat, iron to keep structures
/// (and granaries) lit, and — where `leisure` — a deep Diamond stock (the
/// biotic line's leisure good) so the recreation tier reads covered.
/// `granaries` active granaries and a long-lived (class 2) biotic line.
fn society(habitats: u16, granaries: u16, leisure: bool, seed: i64) -> World {
    let mut world = World::new();
    world.found("society").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites mesic");
    let stamp = world.now();
    for (resource, amount) in [
        (Resource::Biomass, 400i64),
        (Resource::Iron, 100_000),
    ] {
        world.chronicle_mut().append(
            ClosureEvent::YieldCollected { orb_id: FIRST_ORB, resource, amount: ratio(amount, 1) },
            stamp,
            vec![SOVEREIGN],
        );
    }
    if leisure {
        world.chronicle_mut().append(
            ClosureEvent::YieldCollected {
                orb_id: FIRST_ORB,
                resource: Resource::Diamond,
                amount: ratio(1_000_000, 1),
            },
            stamp,
            vec![SOVEREIGN],
        );
    }
    let mut world = World::resume(world.chronicle().clone());
    for tile in 0..habitats {
        world.zone(tile, ZoneKind::Habitat).expect("habitat");
    }
    for g in 0..granaries {
        world.zone(50 + g, ZoneKind::Granary).expect("granary");
    }
    world.seal_taxon(2, 1).expect("long-lived biotic");
    world.settle_population(FIRST_ORB, seed);
    world
}

fn population_after(mut world: World, rounds: usize) -> (u64, World) {
    for _ in 0..rounds {
        for _ in 0..8 {
            world.tick();
        }
        let _ = world.collect();
    }
    (world.population_of(FIRST_ORB), world)
}

#[test]
fn a_granary_lifts_the_population_ceiling() {
    // The provisioning footprint adds to the population ceiling (never the
    // ration's own regrowth ceiling, which stays ecological).
    let bare = society(6, 0, false, 10);
    let provisioned = society(6, 2, false, 10);
    assert!(
        provisioned.population_capacity(FIRST_ORB) > bare.population_capacity(FIRST_ORB),
        "each granary sustains more people: {} > {}",
        provisioned.population_capacity(FIRST_ORB),
        bare.population_capacity(FIRST_ORB)
    );
    // But the ecological ration ceiling is untouched by the granary.
    assert_eq!(
        provisioned.ration_capacity_of(FIRST_ORB),
        bare.ration_capacity_of(FIRST_ORB),
        "a granary provisions people, not the food's own regrowth"
    );
}

#[test]
fn stability_reads_the_leisure_coverage_only_under_a_granary() {
    let stamp_ahead = |w: &World| w.now() + 8;
    // No granary: the leisure tier does not apply — full stability always,
    // whether or not any leisure good is banked.
    let subsistence = society(6, 0, false, 10);
    assert_eq!(
        subsistence.stability_of(FIRST_ORB, stamp_ahead(&subsistence)),
        ratio(1, 1),
        "with no granary the recreation tier does not gate anything"
    );
    // Granary, leisure fully stocked: full stability.
    let entertained = society(6, 2, true, 10);
    assert_eq!(
        entertained.stability_of(FIRST_ORB, stamp_ahead(&entertained)),
        ratio(1, 1),
        "a fed, entertained people is fully content"
    );
    // Granary, no leisure banked: zero stability — the recreation tier is
    // unmet.
    let bored = society(6, 2, false, 10);
    assert_eq!(
        bored.stability_of(FIRST_ORB, stamp_ahead(&bored)),
        ratio(0, 1),
        "a granary with no leisure good reads no contentment"
    );
}

#[test]
fn a_bored_people_stalls_where_an_entertained_one_grows() {
    // THE STALL (§3–4): under a granary, an unmet leisure tier halts births
    // — the population holds (a long-lived line neither grows nor dies),
    // while the entertained twin grows. Replay byte-exact for both.
    let (bored, bored_world) = population_after(society(6, 2, false, 10), 6);
    let (entertained, ent_world) = population_after(society(6, 2, true, 10), 6);
    assert_eq!(bored, 10, "the bored population stalls at its seed — births halt");
    assert!(entertained > 10, "the entertained population grows: {entertained}");
    assert!(
        entertained > bored,
        "leisure is the difference between growth and stall: {entertained} > {bored}"
    );
    assert_eq!(replay(bored_world.chronicle()), bored_world.committed(), "bored replay MATCHES");
    assert_eq!(replay(ent_world.chronicle()), ent_world.committed(), "entertained replay MATCHES");
}

#[test]
fn starvation_crashes_where_a_stall_merely_holds() {
    // The severity ordering (§2): a leisure-short people STALLS (holds,
    // does not die), but a calorie-short people STARVES (crashes to zero).
    // Stall is the milder tier, and it fires before any cull.
    let (stalled, _) = population_after(society(6, 2, false, 10), 5);
    let (starved, _) = population_after(society(6, 2, true, 6000), 5);
    assert!(stalled > 0, "a stalled (bored but fed) people endures: {stalled}");
    assert_eq!(starved, 0, "a starved (overpopulated) people crashes to nothing");
}

#[test]
fn a_world_with_no_granary_ignores_the_leisure_tier_entirely() {
    // The #40–42 subsistence path is untouched where no granary stands:
    // banking a leisure good changes NOTHING, because the tier never
    // applies — the net delta is identical with and without it.
    let with_leisure = society(6, 0, true, 10);
    let without = society(6, 0, false, 10);
    let d_with = with_leisure.population_touch_delta(FIRST_ORB, with_leisure.now() + 8);
    let d_without = without.population_touch_delta(FIRST_ORB, without.now() + 8);
    assert_eq!(
        d_with.map(|(delta, _, _)| delta),
        d_without.map(|(delta, _, _)| delta),
        "with no granary, a stocked leisure good does not touch the population delta"
    );
}

// ── L4: DISPOSITION FACTORS — the granary as an economic driver (§5) ──

/// A provisioned society (2 granaries) sealed to a DISPOSITION (1
/// expansionist · 2 empiricist · 3 epicurean), biotic so the leisure good
/// is Diamond, long-lived, seeded with people.
fn disposed_society(disposition: u8, leisure: bool, seed: i64) -> World {
    let mut world = society(6, 2, leisure, 0);
    world.seal_genesis("civ", disposition, 2, 1).expect("seals disposition");
    world.settle_population(FIRST_ORB, seed);
    world
}

#[test]
fn the_leisure_appetite_bands_by_disposition() {
    // The factor set (§5), built from 0/1/2: lean, base, deep.
    assert_eq!(
        disposed_society(1, false, 10).provision_leisure_factor(),
        ratio(0, 1),
        "an expansionist wants no luxury — military rations"
    );
    assert_eq!(
        disposed_society(2, false, 10).provision_leisure_factor(),
        ratio(1, 1),
        "an empiricist wants the base ration of leisure"
    );
    assert_eq!(
        disposed_society(3, false, 10).provision_leisure_factor(),
        ratio(2, 1),
        "an epicurean wants twice the leisure — the luxury appetite"
    );
}

#[test]
fn an_expansionist_grows_lean_whether_or_not_it_has_luxury() {
    // Factor 0 — no leisure tier applies, so an expansionist grows the same
    // fed with luxury or without it. Lean provisioning, maximum output.
    let (with_lux, w1) = population_after(disposed_society(1, true, 10), 8);
    let (no_lux, w2) = population_after(disposed_society(1, false, 10), 8);
    assert!(with_lux > 10, "the expansionist grows: {with_lux}");
    assert_eq!(with_lux, no_lux, "and grows the SAME without luxury — it needs none");
    assert_eq!(replay(w1.chronicle()), w1.committed(), "replay MATCHES (lux)");
    assert_eq!(replay(w2.chronicle()), w2.committed(), "replay MATCHES (lean)");
}

#[test]
fn an_epicurean_is_the_granary_engine_deep_luxury_or_stall() {
    // Factor 2 — the epicurean demands twice the leisure. Met, it thrives;
    // unmet, it stalls hard. This is the pull that makes the granary an
    // economic engine: to grow, the civilisation must PRODUCE luxury.
    let (fed_lux, w1) = population_after(disposed_society(3, true, 10), 8);
    let (starved_lux, w2) = population_after(disposed_society(3, false, 10), 8);
    assert!(fed_lux > 10, "deep luxury met — the epicurean grows: {fed_lux}");
    assert_eq!(starved_lux, 10, "luxury unmet — the epicurean stalls at its seed");
    assert_eq!(replay(w1.chronicle()), w1.committed(), "replay MATCHES (fed)");
    assert_eq!(replay(w2.chronicle()), w2.committed(), "replay MATCHES (stalled)");
}

#[test]
fn an_empiricist_spends_births_on_knowledge_and_grows_slower() {
    // Factor: baseline leisure, but HALF the surplus births are siphoned to
    // research — so a fed empiricist grows slower than a fed expansionist on
    // the same world, spending mouths for knowledge.
    let (empiricist, we) = population_after(disposed_society(2, true, 10), 8);
    let (expansionist, _) = population_after(disposed_society(1, true, 10), 8);
    assert!(empiricist > 10, "the empiricist still grows: {empiricist}");
    assert!(
        empiricist < expansionist,
        "but slower — half its births went to research: {empiricist} < {expansionist}"
    );
    assert_eq!(replay(we.chronicle()), we.committed(), "replay MATCHES");
}

#[test]
fn the_research_siphon_taxes_only_the_empiricist() {
    // The siphon redirects half of a positive birth cohort — for the
    // empiricist alone; every other disposition spends its surplus on people.
    let cohort = 8;
    assert_eq!(
        disposed_society(2, true, 10).research_siphon(cohort),
        cohort / 2,
        "the empiricist siphons half its births to knowledge"
    );
    for disposition in [1u8, 3] {
        assert_eq!(
            disposed_society(disposition, true, 10).research_siphon(cohort),
            0,
            "disposition {disposition} spends its surplus on people, not research"
        );
    }
    // A non-positive cohort is never taxed.
    assert_eq!(disposed_society(2, true, 10).research_siphon(0), 0, "no births, no siphon");
}

// ── L5: THE WORKBENCH (§7) — the deployed system made legible ──

use cfe::world::workbench::BenchKind;

#[test]
fn the_depot_bench_reads_the_whole_provisioning_society() {
    // A provisioned epicurean world with no leisure stocked — the depot
    // bench surfaces the society and flags the stall.
    let mut world = society(6, 2, false, 10);
    world.seal_genesis("civ", 3, 2, 1).expect("epicurean biotic");
    world.settle_population(FIRST_ORB, 10);
    let view = world.workbench(FIRST_ORB, BenchKind::Depot).expect("the orb stands");
    eprintln!("=== DEPOT BENCH ===");
    for c in &view.components { eprintln!("  · {} [{}]", c.label, if c.active {"lit"} else {"dark"}); }
    for (k, v) in &view.gauges { eprintln!("  {k:>16} : {v}"); }
    for a in &view.alerts { eprintln!("  ! {a}"); }

    assert!(!view.components.is_empty(), "the granary shows as a component");
    assert!(view.gauges.iter().any(|(k, _)| k == "castes L/C/A"), "the caste split is a gauge");
    assert!(view.gauges.iter().any(|(k, _)| k == "stability"), "stability is a gauge");
    assert!(
        view.alerts.iter().any(|a| a.contains("leisure short")),
        "an unmet leisure tier is flagged"
    );
}

#[test]
fn a_bare_world_bench_flags_no_granary() {
    let world = society(4, 0, false, 5);
    let view = world.workbench(FIRST_ORB, BenchKind::Depot).expect("stands");
    assert!(view.components.is_empty(), "no granary, no depot components");
    assert!(view.alerts.iter().any(|a| a.contains("no granary")), "the missing granary is flagged");
}

#[test]
fn every_bench_kind_reads_uniformly() {
    // The abstraction is general: each bench kind returns the same shape,
    // an empty deploy reading as a legible alert rather than an error.
    let world = society(4, 1, false, 5);
    for kind in [BenchKind::Depot, BenchKind::Refinery, BenchKind::ShipYard, BenchKind::Lab] {
        let view = world.workbench(FIRST_ORB, kind).expect("every bench reads");
        assert_eq!(view.kind, kind, "the view names its kind");
        // Refinery/yard/lab aren't built here — each says so, uniformly.
        if kind != BenchKind::Depot {
            assert!(!view.alerts.is_empty(), "{}: an undeployed bench reports it", kind.word());
        }
    }
}
