//! **PLANETARY REJUVENATION — the lazy READ, at the world** (task #28).
//!
//! A planet SCAN folds the ruled S-curve over the beats elapsed since
//! the orb's last touch and shows what its biomass has recovered to —
//! writing nothing, minting no receipt, advancing no clock. These
//! witnesses hold that read: capacity comes from the living footprint
//! and the place, a collapsed lush world climbs back, and looking
//! changes nothing.

use cfe::almanac::ThermalRegion;
use cfe::dop::{positive, unit, ClosureEvent, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, World, FIRST_ORB, SOVEREIGN};

/// A world sited in the mesic band under a G star, with `habitats`
/// habitats standing — the lush case. Biomass is topped up by receipt
/// first so the habitats can be built (5 each) and a remnant remains
/// banked below K.
fn greenhold(habitats: u16) -> World {
    let mut world = World::new();
    world.found("greenhold").expect("founds");
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
    let mut world = World::resume(world.chronicle().clone());
    for tile in 0..habitats {
        world.zone(tile, ZoneKind::Habitat).expect("builds a habitat");
    }
    world
}

#[test]
fn the_place_and_the_footprint_set_the_capacity() {
    let world = greenhold(8);
    assert_eq!(world.orb_region(FIRST_ORB), Some(ThermalRegion::Mesic), "sited in the life band");
    assert_eq!(world.living_zones_of(FIRST_ORB), 8, "eight habitats are the footprint");
    let k = world.ration_capacity_of(FIRST_ORB);
    assert!(k > ratio(100, 1), "a lush world holds a large K: {k}");
    // A world with no living zones holds no natural biomass ceiling.
    let bare = greenhold(0);
    assert_eq!(bare.ration_capacity_of(FIRST_ORB), ratio(0, 1), "no footprint, no capacity");
}

#[test]
fn a_scan_shows_biomass_climbing_toward_capacity_over_time() {
    let world = greenhold(8);
    let now = world.orb().expect("stands").banked_amount(Resource::Biomass);
    let k = world.ration_capacity_of(FIRST_ORB);
    assert!(now < k, "the young colony sits below its lush ceiling: {now} < {k}");

    // At the present beat, the scan reads today's stock unchanged.
    assert_eq!(world.regrown_ration(FIRST_ORB, world.now()), now, "no time, no growth");

    // As beats pass the scan reads MORE, monotonically, never past K.
    let mut last = now.clone();
    for stamp in [10u64, 100, 1_000, 100_000] {
        let grown = world.regrown_ration(FIRST_ORB, stamp);
        assert!(grown >= last, "regrowth is monotone in the scan: {grown} >= {last}");
        assert!(grown <= k, "and never past the ceiling: {grown} <= {k}");
        last = grown;
    }
    assert!(last > now, "over a long wait the stock has genuinely climbed");
}

#[test]
fn a_barren_world_never_regrows_and_a_rock_holds_nothing() {
    // A scorched rock (sited hot, no habitable band) carries no capacity
    // and shows no regrowth however long it waits.
    let mut world = World::new();
    world.found("cinder").expect("founds");
    world.commission_star(StarClass::O).expect("commissions hot");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 2), None, None)
        .expect("sites in the furnace");
    world.zone(0, ZoneKind::Habitat).expect("builds");
    assert_eq!(world.ration_capacity_of(FIRST_ORB), ratio(0, 1), "the furnace bears no life");
    let now = world.orb().expect("stands").banked_amount(Resource::Biomass);
    // Over capacity (any stock over K=0) settles toward zero, never up.
    assert!(world.regrown_ration(FIRST_ORB, 1_000_000) <= now, "a rock does not bloom");
}

#[test]
fn a_scan_is_not_a_deed_and_changes_nothing() {
    // Looking at a planet writes no receipt and moves no bank — the
    // read-only-audit law. The chronicle and the committed state are
    // untouched by any number of scans.
    let mut world = greenhold(8);
    let before_receipts = world.chronicle().receipts().len();
    let before_bank = world.orb().expect("stands").banked_amount(Resource::Biomass);
    for stamp in [1u64, 100, 10_000, 1_000_000] {
        let _ = world.regrown_ration(FIRST_ORB, stamp);
    }
    assert_eq!(
        world.chronicle().receipts().len(),
        before_receipts,
        "a scan mints no receipt"
    );
    assert_eq!(
        world.orb().expect("stands").banked_amount(Resource::Biomass),
        before_bank,
        "and moves no bank — the grown value settles only at a real touch"
    );
    // Belt and braces: a fresh append + resume still replays byte-exact,
    // proving nothing was silently mutated.
    let stamp = world.now();
    world.chronicle_mut().append(
        ClosureEvent::YieldCollected {
            orb_id: FIRST_ORB,
            resource: Resource::Biomass,
            amount: ratio(1, 1),
        },
        stamp,
        vec![SOVEREIGN],
    );
    let world = World::resume(world.chronicle().clone());
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

// ── the SETTLE: regrowth banks at a real collect touch ──

#[test]
fn a_collect_banks_the_regrown_ration_folded_into_its_yield() {
    // A depleted mesic world, left to run: at the collect touch its
    // natural biomass has climbed, and that growth banks WITH the farmed
    // yield — one receipt, no separate deed.
    let mut world = greenhold(8);
    for _ in 0..50 {
        world.tick();
    }
    let before = world.orb().expect("stands").banked_amount(Resource::Biomass);
    let farmed: cfe::ratio::Ratio = world
        .accrued()
        .iter()
        .filter(|(resource, _)| *resource == Resource::Biomass)
        .map(|(_, amount)| amount.clone())
        .fold(ratio(0, 1), |sum, amount| sum + amount);
    let grown = world.regrown_ration(FIRST_ORB, world.now()) - before.clone();
    assert!(grown > ratio(0, 1), "a depleted mesic world has regrowth to settle: {grown}");

    world.collect().expect("collects");
    let after = world.orb().expect("stands").banked_amount(Resource::Biomass);
    assert_eq!(
        after,
        before + farmed + grown,
        "the collect banks farmed yield AND the regrown stock, in one touch"
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn a_world_at_capacity_regrows_nothing_at_the_touch() {
    // Rejuvenation only aids RECOVERY: a stock already at or above its
    // ceiling settles no growth (farming may still push it higher, but
    // the natural law adds nothing above K).
    let world = greenhold(8);
    let k = world.ration_capacity_of(FIRST_ORB);
    assert!(k > ratio(0, 1), "a lush world has a ceiling");
    // Nothing has elapsed and the stock sits below K, but the SETTLE at
    // t=last_touch is zero regardless (no time, no growth).
    assert_eq!(world.rejuvenation_settle(FIRST_ORB, world.now()), None, "no time, no settle");
    // And an over-capacity stock never settles growth, however long it waits.
    // (Read the curve directly: a stock at K stays at K.)
    assert_eq!(
        cfe::almanac::grained_logistic(&k, &k, &cfe::almanac::intrinsic_rate(2, None), 10_000),
        k,
        "at capacity, the curve holds — the settle would bank nothing"
    );
}

// ── QUARANTINE (task #30): a sealed world starves instead of recovering ──

/// A lush mesic greenhold whose bio-pressure has been driven past the
/// quarantine threshold (cascade strikes by receipt), so it is sealed.
fn quarantined_greenhold(habitats: u16) -> World {
    let mut world = World::new();
    world.found("sealed-hold").expect("founds");
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
    for _ in 0..World::QUARANTINE_PRESSURE {
        world.chronicle_mut().append(
            ClosureEvent::ScienceExploded {
                orb_id: FIRST_ORB,
                path: 0,
                kind: cfe::almanac::ExplosionKind::BioChemCross.as_wire(),
                resource: Resource::Biomass as u8,
                slash: ratio(0, 1),
            },
            stamp,
            vec![SOVEREIGN],
        );
    }
    let mut world = World::resume(world.chronicle().clone());
    for tile in 0..habitats {
        world.zone(tile, ZoneKind::Habitat).expect("builds a habitat");
    }
    world
}

// ── POPULATION DYNAMICS (#40/#41/#42): the elastic people eat, breed,
//    and die at the collect touch — driven off the same lazy clock ──

/// A greenhold with a sealed lifespan class and a seeded population.
/// `class` 2 is long-lived (4 grains); 1 short-lived (one grain). The
/// metabolism stays the biotic default, so both eat Biomass at the same
/// rate — the ONLY difference between two classes is the lifespan `L`.
fn peopled(habitats: u16, class: u8, seed: i64) -> World {
    let mut world = greenhold(habitats);
    world.seal_taxon(class, 1).expect("seals the lifespan class");
    world.settle_population(FIRST_ORB, seed);
    world
}

#[test]
fn a_collect_applies_the_population_net_delta_and_debits_its_food() {
    // The touch settles what the reading predicts: the headcount moves by
    // exactly `population_touch_delta`'s Δ, and the food it ate leaves the
    // biomass bank as part of the same touch — one lazy settle, replay-exact.
    let mut world = peopled(8, 2, 10);
    for _ in 0..8 {
        world.tick();
    }
    let (delta, resource, consumed) = world
        .population_touch_delta(FIRST_ORB, world.now())
        .expect("a peopled, farming world drives population");
    assert_eq!(resource, Resource::Biomass, "a biotic line's people eat biomass");
    assert!(consumed > ratio(0, 1), "the people ate this span: {consumed}");
    // The TRUE mass — the integer headcount plus its fractional carry —
    // moves by the predicted rational delta, QUANTIZED onto the population
    // lattice (COG Slice 2: the whole part promotes to the headcount, the
    // fraction is carried on the 1024-lattice that bounds the #28 feedback
    // denominator). So the mass after equals the lattice-floor of
    // `before + delta` — exact and replayable, the carry off by at most one
    // lattice step from the raw prediction.
    let before = ratio(world.population_of(FIRST_ORB) as i64, 1) + world.pop_accrued_of(FIRST_ORB);
    world.collect().expect("collects");
    let after = ratio(world.population_of(FIRST_ORB) as i64, 1) + world.pop_accrued_of(FIRST_ORB);
    let lattice = cfe::world::stock::lattice();
    let expected = ((before.clone() + delta) * lattice.clone()).floor() / lattice;
    assert_eq!(
        after, expected,
        "the collect applies the predicted net delta, quantized onto the lattice"
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn a_fed_line_grows_and_an_overpopulated_one_is_culled() {
    // Below the food ceiling the fed population BIRTHS (net positive delta);
    // far above it the same world STARVES and culls. Both are the same law
    // read at different densities, and both replay byte-exact.
    let mut fed = peopled(8, 2, 10);
    for _ in 0..8 {
        fed.tick();
    }
    let (grow, _, _) = fed.population_touch_delta(FIRST_ORB, fed.now()).expect("driven");
    assert!(grow > ratio(0, 1), "a small fed population grows: Δ={grow}");
    fed.collect().expect("collects");
    let fed_mass =
        ratio(fed.population_of(FIRST_ORB) as i64, 1) + fed.pop_accrued_of(FIRST_ORB);
    assert!(fed_mass > ratio(10, 1), "the people multiplied: {fed_mass}");
    assert_eq!(replay(fed.chronicle()), fed.committed(), "fed replay MATCHES");

    // A world seeded far past what its habitats can feed collapses.
    let mut crowded = peopled(8, 2, 4000);
    for _ in 0..8 {
        crowded.tick();
    }
    let (cull, _, _) = crowded.population_touch_delta(FIRST_ORB, crowded.now()).expect("driven");
    assert!(cull < ratio(0, 1), "an overpopulated world culls: Δ={cull}");
    crowded.collect().expect("collects");
    assert!(crowded.population_of(FIRST_ORB) < 4000, "the famine took its toll");
    assert_eq!(replay(crowded.chronicle()), crowded.committed(), "cull replay MATCHES");
}

#[test]
fn a_small_fed_population_grows_instead_of_freezing() {
    // COG Slice 2 — the fractional-birth accumulator. A small fed band GROWS:
    // its sub-one births accrue on the exact-rational carry (lattice-bounded)
    // and promote whole colonists across collects, where the old FLOORED
    // logistic discarded any birth below one and froze the band at its seed
    // forever. Replay-exact — the carry rides its own receipt (tag 179).
    //
    // (The food ceiling that eventually bounds and culls an over-provisioned
    // line — grow AND die — is the separate population-balance work, tuned
    // with the seed activation; here the horizon stays inside the band's
    // sustainable growth so the accumulator itself is what is measured.)
    let mut world = peopled(8, 2, 2);
    for _ in 0..10 {
        for _ in 0..8 {
            world.tick();
        }
        world.collect().expect("collects");
    }
    assert!(
        world.population_of(FIRST_ORB) > 2,
        "the small fed band grew past its seed of 2: {}",
        world.population_of(FIRST_ORB)
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn an_overgrown_line_stabilizes_at_the_syntropic_limit_never_extinct() {
    // COG Slice 2c — the syntropic baseline caps the extinction vent. A band
    // that grows past what it can gather does NOT collapse: the star keeps
    // feeding the wild baseline (`syntropic_flow`), so the starvation cull
    // regresses the line to that limit and HOLDS it there, alive, forever — a
    // standing deficit, never a spiral to zero. (Before the baseline this
    // exact seed grew, ate the ambient stock to zero, and reached 0 clones by
    // the sixteenth collect — entropy winning an uncapped vent.)
    let mut world = peopled(8, 2, 2);
    let mut trajectory = Vec::new();
    for _ in 0..40 {
        for _ in 0..8 {
            world.tick();
        }
        world.collect().expect("the line never collapses — syntropy floors it");
        trajectory.push(world.population_of(FIRST_ORB));
    }
    let tail_min = *trajectory[30..].iter().min().expect("tail");
    assert!(
        tail_min > 0,
        "the line endures at the wild limit across the whole tail — no famine spiral: {trajectory:?}"
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn the_settle_denominator_stays_bounded_over_many_collects() {
    // THE #28 LATTICE GUARD. The regrowth logistic is an exact rational
    // whose denominator compounds each time its output banks and feeds
    // back as the next p0 — the 2^414305-after-16 explosion `stock.rs`
    // lattice-flooring exists to stop. Measured on the settle path at ~1
    // denominator-digit PER COLLECT before the floor. With it, the banked
    // ration stays on the lattice (denominator divides `LATTICE`) forever.
    let lattice = 1024u64; // cfe::world::stock::lattice() numerator
    // No population — the pure regrowth-settle path, so the habitats keep
    // farming and the only fractional injector is the logistic itself.
    let mut world = greenhold(8);
    for _ in 0..40 {
        for _ in 0..8 {
            world.tick();
        }
        world.collect().expect("collects");
        let bank = world.orb().expect("stands").banked_amount(Resource::Biomass);
        let denom: u64 = bank.denom().try_into().expect("a bounded denominator fits u64");
        assert!(
            lattice % denom == 0,
            "the banked ration stays on the lattice, never exploding: denom={denom}"
        );
    }
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}

#[test]
fn a_longer_lived_line_outgrows_a_short_lived_one() {
    // R7 — the ONLY difference is lifespan `L`: same world, same food, same
    // seed. The long-lived line dies more slowly (mortality `p·Δt/L`), so
    // its net births carry it to a HIGHER peak than the short-lived line,
    // which the mortality holds flat. Lifespan is a population lever.
    fn peak(class: u8) -> u64 {
        let mut world = peopled(8, class, 10);
        let mut top = world.population_of(FIRST_ORB);
        for _ in 0..6 {
            for _ in 0..8 {
                world.tick();
            }
            // A collapsed line leaves nothing to collect (NothingAccrued);
            // that is a clean no-op, so the peak still stands.
            let _ = world.collect();
            top = top.max(world.population_of(FIRST_ORB));
        }
        assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
        top
    }
    let short = peak(1);
    let long = peak(2);
    assert!(
        long > short,
        "the long-lived line reaches a higher population: long={long} > short={short}"
    );
}

#[test]
fn a_capture_line_population_rides_the_energy_economy_not_the_ration() {
    // A plasmoid FARMS nothing (harvest rate 0 — it captures heat, 04 §1.2);
    // there is no ration stock to feed a population from, so the ration path
    // does not drive it. Its people ride the energy economy (#29), so the
    // touch delta is None however many stand.
    let mut world = greenhold(8);
    world
        .seal_genesis("plasmoid", 1, 3, 1)
        .expect("seals a plasmic genesis");
    world.settle_population(FIRST_ORB, 50);
    for _ in 0..8 {
        world.tick();
    }
    assert_eq!(
        world.population_touch_delta(FIRST_ORB, world.now()),
        None,
        "a capture line's population is not driven by the ration path"
    );
}

#[test]
fn a_quarantined_world_starves_its_ration_at_the_touch() {
    let mut world = quarantined_greenhold(8);
    assert!(world.world_quarantined(), "the world is sealed by bio-pressure");
    for _ in 0..50 {
        world.tick();
    }
    let before = world.orb().expect("stands").banked_amount(Resource::Biomass);
    let farmed: cfe::ratio::Ratio = world
        .accrued()
        .iter()
        .filter(|(resource, _)| *resource == Resource::Biomass)
        .map(|(_, amount)| amount.clone())
        .fold(ratio(0, 1), |sum, amount| sum + amount);
    // A sealed world DECAYS where a free one would regrow.
    let (resource, decay) = world
        .quarantine_decay(FIRST_ORB, world.now())
        .expect("a sealed world starves");
    assert_eq!(resource, Resource::Biomass);
    assert!(decay > ratio(0, 1), "the ration is decaying: {decay}");
    // The TOUCH settles a starving DEBIT, not a recovery credit — the
    // quarantine branch overrides regrowth for a sealed world.
    assert_eq!(
        world.ration_touch_delta(FIRST_ORB, world.now()),
        Some((Resource::Biomass, -decay.clone())),
        "a sealed world's touch is a debit, never a credit"
    );

    world.collect().expect("collects — quarantine seals TRADE, not internal yield");
    let after = world.orb().expect("stands").banked_amount(Resource::Biomass);
    assert_eq!(
        after,
        before + farmed - decay,
        "the sealed world banks farmed yield MINUS the starving decay"
    );
    assert_eq!(replay(world.chronicle()), world.committed(), "replay MATCHES");
}
