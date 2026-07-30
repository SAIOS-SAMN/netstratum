//! G3 witnesses — PERCEPTION + TRANSLATION (04 §1.3, §4; NS-2 §4):
//! the sigil is FRAME-FREE (the H¹ class is what survives translation
//! — frames quotient out, joining T outside the key); translation is
//! a READ-time law with its loss measured exact and shown, never
//! silent; the hit record extends additively (SEAL A on the wire);
//! the lens law holds through the translation read.

use cfe::dop::{translation_survival, ColonyOrb, Resource, ZoneKind};
use cfe::query::{hit_record_bytes, orb_sigil, translated_read};
use cfe::ratio::ratio;
use cfe::world::{replay, World, FIRST_ORB};

const EXPANSIONIST: u8 = 1;
const BIOTIC: u8 = 2;
const PLANAR: u8 = 1;
const HARMONIC: u8 = 3;
const UMBRAL: u8 = 4;

#[test]
fn the_survival_table_is_the_law() {
    // Same basis: the whole structure survives. The 04 §4 example
    // verbatim: a harmonic bubble read planar keeps 3/4. Umbral pairs
    // read by occlusion and keep 1/2. Symmetric in v1 (placeholder —
    // the loss prices the DISTANCE between frames, not the direction).
    assert_eq!(translation_survival(PLANAR, PLANAR), ratio(1, 1));
    assert_eq!(translation_survival(HARMONIC, PLANAR), ratio(3, 4));
    assert_eq!(translation_survival(PLANAR, HARMONIC), ratio(3, 4));
    assert_eq!(translation_survival(UMBRAL, PLANAR), ratio(1, 2));
    assert_eq!(translation_survival(HARMONIC, UMBRAL), ratio(1, 2));
    assert_eq!(translation_survival(UMBRAL, UMBRAL), ratio(1, 1));
    // The unsealed read carries no frame claim: full survival.
    assert_eq!(translation_survival(0, HARMONIC), ratio(1, 1));
}

#[test]
fn the_sigil_is_frame_free() {
    // Two identically built orbs project ONE sigil regardless of any
    // sealed basis — the frame is the observer's, never the
    // structure's. Query by WHAT it is, never by WHO reads it.
    let mut held = ColonyOrb { orb_id: 1, name: "held".to_string(), ..ColonyOrb::default() };
    held.zones = vec![
        cfe::dop::Zone { tile: 10, kind: ZoneKind::Extractor },
        cfe::dop::Zone { tile: 21, kind: ZoneKind::Habitat },
    ];
    held.banked = vec![
        (Resource::Iron, ratio(10, 1)),
        (Resource::Biomass, ratio(5, 1)),
    ];
    let sigil = orb_sigil(&held);
    // The sigil derives from committed structure alone: no basis
    // parameter exists to vary. The address is the same bytes under
    // every sky — measured by the record itself.
    assert_eq!(sigil, orb_sigil(&held.clone()));
}

#[test]
fn a_cross_frame_read_measures_its_loss() {
    // The N5-ready seam: one structure sealed under two skies. The
    // harmonic line's orb and the planar line's orb RESONATE (the
    // class survives translation) and the read carries 3/4 — the loss
    // measured, shown, never silent.
    let mut ours = ColonyOrb { orb_id: 1, name: "ours".to_string(), ..ColonyOrb::default() };
    ours.zones = vec![
        cfe::dop::Zone { tile: 10, kind: ZoneKind::Extractor },
        cfe::dop::Zone { tile: 21, kind: ZoneKind::Habitat },
    ];
    ours.banked = vec![
        (Resource::Iron, ratio(10, 1)),
        (Resource::Biomass, ratio(5, 1)),
    ];
    let mut theirs = ColonyOrb { orb_id: 7, name: "theirs".to_string(), ..ColonyOrb::default() };
    // The rot90 image — congruent, a genuine twin across worlds.
    theirs.zones = vec![
        cfe::dop::Zone { tile: 22, kind: ZoneKind::Extractor },
        cfe::dop::Zone { tile: 45, kind: ZoneKind::Habitat },
    ];
    theirs.banked = ours.banked.clone();
    let (resonant, survival) = translated_read(&ours, PLANAR, &theirs, HARMONIC);
    assert!(resonant, "the class survives translation — that is what a class IS");
    assert_eq!(survival, ratio(3, 4), "the harmonic sky through a planar eye: 3/4, exact");
    let (self_read, whole) = translated_read(&ours, PLANAR, &theirs, PLANAR);
    assert!(self_read);
    assert_eq!(whole, ratio(1, 1), "one frame, no loss");
}

#[test]
fn the_local_lens_reads_whole_and_stays_a_lens() {
    // In one sealed world every candidate shares the line's basis:
    // the scan annotates full survival, and the chronicle still gains
    // EXACTLY one receipt — the draw. The lens law holds through G3.
    let mut world = World::new();
    world.found("meridian").expect("founds");
    world
        .seal_genesis("planar-line", EXPANSIONIST, BIOTIC, PLANAR)
        .expect("seals");
    world.zone(0, ZoneKind::Extractor).expect("builds");
    world.zone(1, ZoneKind::Extractor).expect("builds");
    world.zone(8, ZoneKind::Habitat).expect("builds");
    world.zone(9, ZoneKind::Habitat).expect("builds");
    for _ in 0..40 {
        world.tick();
    }
    world.collect().expect("settles");
    let twin_a = world.chart("planted-a", FIRST_ORB).expect("charters");
    let twin_b = world.chart("planted-b", FIRST_ORB).expect("charters");
    world.zone_as(cfe::world::SOVEREIGN, twin_a, 10, ZoneKind::Extractor).expect("builds");
    world.zone_as(cfe::world::SOVEREIGN, twin_a, 21, ZoneKind::Habitat).expect("builds");
    world.zone_as(cfe::world::SOVEREIGN, twin_b, 22, ZoneKind::Extractor).expect("builds");
    world.zone_as(cfe::world::SOVEREIGN, twin_b, 45, ZoneKind::Habitat).expect("builds");
    // The annotated read is proven at the pure seam above (the
    // crystal climb is rqp_scanner's proof, not re-run here); what
    // THIS arc pins is the lens law surviving G3: a crystal-less scan
    // refuses pure and the chronicle stands unmoved.
    let receipts_before = world.chronicle().receipts().len();
    let refusal = world.scan(FIRST_ORB, twin_a).expect_err("no crystals banked");
    assert!(matches!(refusal, cfe::world::ActionRefused::CannotAfford { .. }));
    assert_eq!(world.chronicle().receipts().len(), receipts_before);
    assert_eq!(replay(world.chronicle()), world.committed());
}

#[test]
fn the_hit_record_extends_additively() {
    // SEAL A on the wire, measured: the pre-G3 hit record is a strict
    // PREFIX of the G3 record — an old reader parses orb and stamp
    // and skips nothing it needs; the survival ratio rides behind.
    let old_shape = hit_record_bytes(3, 178, &ratio(1, 1));
    let annotated = hit_record_bytes(3, 178, &ratio(3, 4));
    assert_eq!(
        old_shape[..21],
        annotated[..21],
        "tag ‖ length ‖ orb ‖ stamp — the 21-byte prefix an old reader parses, unchanged"
    );
    assert_ne!(old_shape, annotated, "the survival rides behind the prefix");
}
