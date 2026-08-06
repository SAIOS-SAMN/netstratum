//! N4 witnesses — NS-2 RQP + the scanner (NS-2 §3): the sigil is
//! frame-invariant; the scan finds the planted match and refuses the
//! planted near-miss with its delta whole; the lens leaves exactly one
//! receipt (the crystal draw); a crystal-less scan refuses pure; the
//! resonator converts on the knife edge; replay MATCHES.

use cfe::dop::{ColonyOrb, Resource, ZoneKind};
use cfe::query::orb_sigil;
use cfe::ratio::ratio;
use cfe::vrp::ChainRead;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB, SOVEREIGN};

/// The witness board: the genesis prober climbs the whole pipeline to
/// a banked crystal; four charted colonies plant the vectors. A (the
/// exemplar): extractor@(1,2)=10, habitat@(2,5)=21. B (the planted
/// match): the same pattern under rot90 — extractor@(2,6)=22,
/// habitat@(5,5)=45, banked equal (charter reserves minus the same
/// costs, never collected). C (the kind-swap near-miss): B's tiles,
/// kinds exchanged — banked EQUAL (the swap costs the same), so its
/// C(Δ) is zero while its delta is real. D (the banked near-miss):
/// B's exact pattern, but collected once at +4 ticks — 1E+1H nets
/// iron +3/2 − 1/4 = +5/4 and biomass +1 − 1/2 = +1/2 per tick, so D
/// banks 15 iron, 7 biomass against the twins' 10, 5.
fn scanner_board() -> (World, u64, u64, u64, u64) {
    let mut world = World::new();
    world.found("meridian").expect("founding closes");
    // Stage 1 — the bootstrap: 2E+2H spend the whole reserve; nets
    // +5/2 iron, +1 biomass per tick.
    world.zone(0, ZoneKind::Extractor).expect("10 iron");
    world.zone(1, ZoneKind::Extractor).expect("10 iron");
    world.zone(8, ZoneKind::Habitat).expect("5 biomass");
    world.zone(9, ZoneKind::Habitat).expect("5 biomass");
    for _ in 0..40 {
        world.tick();
    }
    world.collect().expect("the charter fund settles");
    // The four charted colonies: 20 iron + 10 biomass move to each.
    let a = world.chart("planted-a", FIRST_ORB).expect("charters");
    let b = world.chart("planted-b", FIRST_ORB).expect("charters");
    let c = world.chart("planted-c", FIRST_ORB).expect("charters");
    let d = world.chart("planted-d", FIRST_ORB).expect("charters");
    world.zone_as(SOVEREIGN, a, 10, ZoneKind::Extractor).expect("builds");
    world.zone_as(SOVEREIGN, a, 21, ZoneKind::Habitat).expect("builds");
    world.zone_as(SOVEREIGN, b, 22, ZoneKind::Extractor).expect("builds");
    world.zone_as(SOVEREIGN, b, 45, ZoneKind::Habitat).expect("builds");
    world.zone_as(SOVEREIGN, c, 22, ZoneKind::Habitat).expect("builds");
    world.zone_as(SOVEREIGN, c, 45, ZoneKind::Extractor).expect("builds");
    world.zone_as(SOVEREIGN, d, 22, ZoneKind::Extractor).expect("builds");
    world.zone_as(SOVEREIGN, d, 45, ZoneKind::Habitat).expect("builds");
    for _ in 0..4 {
        world.tick();
    }
    world.collect_as(SOVEREIGN, d).expect("the near-miss banks its 4 ticks");
    // Stage 2 — the prober's climb, 7E+7H: earn the remaining five of
    // each (50 iron, 25 biomass beyond the 40 iron in hand).
    for _ in 0..34 {
        world.tick();
    }
    world.collect().expect("settles");
    for tile in 2..7 {
        world.zone(tile, ZoneKind::Extractor).expect("10 iron, earned");
    }
    for tile in 10..15 {
        world.zone(tile, ZoneKind::Habitat).expect("5 biomass, earned");
    }
    // Stage 3 — four smelters (60 iron + 40 biomass); 7E+7H nets
    // +35/4 iron, +7/2 biomass.
    for _ in 0..14 {
        world.tick();
    }
    world.collect().expect("settles");
    for tile in 16..20 {
        world.zone(tile, ZoneKind::Smelter).expect("15 iron + 10 biomass, earned");
    }
    // Stage 4 — the synthesizer (20 alloy + 15 biomass); 7E+7H+4S nets
    // +3/4 iron, +3/2 biomass, +4 alloy.
    for _ in 0..13 {
        world.tick();
    }
    world.collect().expect("settles");
    world.zone(20, ZoneKind::Synthesizer).expect("20 alloy + 15 biomass, earned");
    // Stage 5 — the resonator (10 alloy + 5 diamond); alloy on the
    // knife edge (+4 − 4 = 0), diamond +1/4.
    for _ in 0..20 {
        world.tick();
    }
    world.collect().expect("the first diamonds bank");
    world.zone(21, ZoneKind::Resonator).expect("10 alloy + 5 diamond, earned");
    // Stage 6 — the crystal: diamond on the knife edge (+1/4 − 1/4 =
    // 0), crystal +1/8; eight ticks, exactly one.
    for _ in 0..8 {
        world.tick();
    }
    world.collect().expect("the first crystal banks");
    (world, a, b, c, d)
}

#[test]
fn the_sigil_is_frame_invariant() {
    // Pure — no world, no chronicle: two hand-built orbs whose zones
    // are the same pattern under rot90 project the SAME sigil; the
    // kind-swap projects a different one.
    let mut original = ColonyOrb {
        orb_id: 1,
        name: "original".to_string(),
        ..ColonyOrb::default()
    };
    original.zones = vec![
        cfe::dop::Zone { tile: 10, kind: ZoneKind::Extractor, base: None },
        cfe::dop::Zone { tile: 21, kind: ZoneKind::Habitat, base: None },
    ];
    original.banked = vec![
        (Resource::Iron, ratio(10, 1)),
        (Resource::Biomass, ratio(5, 1)),
    ];
    let mut rotated = ColonyOrb {
        orb_id: 2,
        name: "rotated".to_string(),
        ..ColonyOrb::default()
    };
    rotated.zones = vec![
        cfe::dop::Zone { tile: 22, kind: ZoneKind::Extractor, base: None },
        cfe::dop::Zone { tile: 45, kind: ZoneKind::Habitat, base: None },
    ];
    rotated.banked = original.banked.clone();
    let mut swapped = ColonyOrb {
        orb_id: 3,
        name: "swapped".to_string(),
        ..ColonyOrb::default()
    };
    swapped.zones = vec![
        cfe::dop::Zone { tile: 22, kind: ZoneKind::Habitat, base: None },
        cfe::dop::Zone { tile: 45, kind: ZoneKind::Extractor, base: None },
    ];
    swapped.banked = original.banked.clone();
    assert_eq!(
        orb_sigil(&original),
        orb_sigil(&rotated),
        "congruent structure, one sigil — the lens does not see the frame"
    );
    assert_ne!(
        orb_sigil(&original),
        orb_sigil(&swapped),
        "the kind swap is a different joint — the marginals must not collide it"
    );
    // Same address, though: the swap lives in the SAME bucket (that is
    // what makes it a near-miss and not a stranger).
    assert_eq!(orb_sigil(&original).address, orb_sigil(&swapped).address);
}

#[test]
fn the_scan_finds_the_match_and_refuses_the_near_misses_delta_whole() {
    let (mut world, a, b, c, d) = scanner_board();
    let receipts_before = world.chronicle().receipts().len();
    let read = world.scan(FIRST_ORB, a).expect("one banked crystal affords the lens");
    // The planted match: B alone resonates, through rot90.
    assert_eq!(read.hits, vec![b], "the twin is found through the lens");
    // The near-misses refuse with their deltas whole, bucket order.
    assert_eq!(read.refusals.len(), 2);
    let kind_swap = &read.refusals[0];
    assert_eq!(kind_swap.orb, c);
    assert_eq!(kind_swap.banked_residuals, vec![], "the swap costs the same");
    assert_eq!(
        kind_swap.zone_deltas,
        vec![(10, 1, 2), (21, 2, 1)],
        "canonical tiles 10 and 21: extractor for habitat, habitat for extractor"
    );
    assert_eq!(
        kind_swap.coherence,
        ratio(0, 1),
        "C(Δ) is zero while the delta is real — the scalar is the summary, never the payload"
    );
    let banked_miss = &read.refusals[1];
    assert_eq!(banked_miss.orb, d);
    assert_eq!(
        banked_miss.banked_residuals,
        vec![
            (Resource::Iron, ratio(-5, 1)),
            (Resource::Biomass, ratio(-2, 1)),
        ],
        "probe minus candidate, the full list — never a truncation"
    );
    assert_eq!(banked_miss.zone_deltas, vec![], "the pattern matches; the bank does not");
    assert_eq!(banked_miss.coherence, ratio(7, 1), "C(Δ) = |−5| + |−2| = 7, exact — the proportional law");
    // The lens law: exactly ONE receipt — the draw; blind to the asking.
    let receipts_after = world.chronicle().receipts().len();
    assert_eq!(receipts_after, receipts_before + 1, "one lens, one receipt");
    let prober = world.orb_by_id(FIRST_ORB).expect("stands");
    assert_eq!(
        prober.banked_amount(Resource::Crystal),
        ratio(0, 1),
        "the crystal is spent on the looking"
    );
    // The reading is dated by its own power draw.
    assert_eq!(read.stamp, world.chronicle().receipts().last().expect("draws").stamp);
    assert_eq!(world.chronicle().verify_chain(), ChainRead::Intact);
    assert_eq!(
        replay(world.chronicle()),
        world.committed(),
        "the chain still holds after the lens — replay MATCHES"
    );
}

#[test]
fn an_empty_bucket_is_its_own_verdict_and_the_exemplar_never_matches_itself() {
    let (mut world, _, _, _, _) = scanner_board();
    // The prober's own 22-zone structure is alone in its bucket; the
    // exemplar is excluded from its own candidates.
    let read = world.scan(FIRST_ORB, FIRST_ORB).expect("the board's crystal affords it");
    assert_eq!(read.hits, vec![], "nothing resonates");
    assert_eq!(read.refusals, vec![], "no delta against the whole population — that would be the scan this protocol refuses");
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn a_crystalless_scan_refuses_pure_and_orphans_nothing() {
    let mut world = World::new();
    world.found("outpost").expect("founding closes");
    let charted = world.chart("bare", FIRST_ORB).expect("the reserve affords one");
    let receipts_before = world.chronicle().receipts().len();
    let refusal = world.scan(FIRST_ORB, charted).expect_err("no crystals, no lens");
    assert_eq!(
        refusal,
        ActionRefused::CannotAfford {
            resource: Resource::Crystal,
            need: Box::new(ratio(1, 1)),
            have: Box::new(ratio(0, 1)),
        }
    );
    assert_eq!(
        world.chronicle().receipts().len(),
        receipts_before,
        "a refusal orphans nothing — no receipt, no draw"
    );
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn the_resonator_converts_on_the_knife_edge() {
    let (mut world, _, _, _, _) = scanner_board();
    // 7E+7H+4S+1Syn+1Res: alloy +4 − 4 = 0; diamond +1/4 − 1/4 = 0;
    // crystal +1/8; iron +3/4; biomass +7 − 7/2 − 2 − 1 = +1/2.
    // Eight ticks: exactly one crystal, and the knife edges accrue
    // NOTHING — a zero flow never enters the workspace ledger.
    for _ in 0..8 {
        world.tick();
    }
    assert_eq!(
        world.accrued(),
        &[
            (Resource::Iron, ratio(6, 1)),
            (Resource::Biomass, ratio(4, 1)),
            (Resource::Crystal, ratio(1, 1)),
        ]
    );
    let orb = world.orb().expect("stands");
    assert!(orb.famine.is_empty(), "a knife edge is fed, not starving");
    world.collect().expect("the second crystal banks");
    assert_eq!(
        world.orb().expect("stands").banked_amount(Resource::Crystal),
        ratio(2, 1),
        "one banked at the board, one more here — exact, no drift"
    );
    assert_eq!(replay(world.chronicle()), world.committed());
}
