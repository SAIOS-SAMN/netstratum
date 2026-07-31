//! NS-5 §2.1 v2 witnesses (MESH-1d, the totality law): a structural
//! divergence is never invisible; a world-family divergence censuses
//! typed; two histories arriving at one state measure zero WHOLE (the
//! confluence precondition); the census rides the wire additively.

use cfe::dop::ClosureEvent;
use cfe::persist::{load, sync};
use cfe::ratio::ratio;
use cfe::tlv;
use cfe::vrp::Chronicle;
use cfe::world::World;
use mesh::{consile, MeshRefused, CENSUS_ORB, CENSUS_WANTS};

use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "netstratum-totality-{}-{name}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

/// The persist_boot session, synced and carried: two nodes, one
/// prefix, ready to diverge.
fn a_forked_pair(name: &str) -> (Chronicle, Chronicle, u64) {
    let dir = scratch(name);
    let a_path = dir.join("a").join("chronicle.tlv");
    let b_path = dir.join("b").join("chronicle.tlv");
    let mut world = World::new();
    world.found("meridian").expect("genesis");
    world.zone(0, cfe::dop::ZoneKind::Extractor).expect("builds");
    world.zone(1, cfe::dop::ZoneKind::Extractor).expect("builds");
    world.zone(8, cfe::dop::ZoneKind::Habitat).expect("builds");
    for _ in 0..4 {
        world.tick();
    }
    world.collect().expect("settles");
    sync(world.chronicle(), &a_path).expect("syncs");
    fs::create_dir_all(b_path.parent().expect("has parent")).expect("b dir");
    fs::copy(&a_path, &b_path).expect("the file carries");
    let fork = world.chronicle().receipts().len() as u64;
    let a = load(&a_path).expect("a returns");
    let b = load(&b_path).expect("b returns");
    (a, b, fork)
}

#[test]
fn a_structural_divergence_is_never_invisible() {
    // Each side razes a DIFFERENT extractor: equal refunds, so no
    // rational moves — before the totality law this divergence
    // measured C = 0 with an empty residual list and vanished. Now
    // the census names the orb.
    let (mut a, mut b, fork) = a_forked_pair("structural");
    a.append(ClosureEvent::ZoneRazed { orb_id: 1, tile: 0 }, 5, vec![100]);
    b.append(ClosureEvent::ZoneRazed { orb_id: 1, tile: 1 }, 5, vec![100]);
    let refused = consile(&a, &b).unwrap_err();
    let MeshRefused::Diverged { at, residuals, coherence, census } = refused else {
        panic!("must refuse DIVERGED");
    };
    assert_eq!(at, fork);
    assert!(residuals.is_empty(), "no rational moved");
    assert_eq!(coherence, ratio(0, 1), "C alone would have been blind");
    assert_eq!(census, vec![(CENSUS_ORB, 1)], "the structure is NAMED");
}

#[test]
fn a_world_family_divergence_censuses_typed() {
    // A's future strikes a want (a world family); B's razes a zone
    // (orb structure). Both differences arrive named, orb first,
    // families in code order.
    let (mut a, mut b, _) = a_forked_pair("family");
    a.append(
        ClosureEvent::WantStruck { orb_id: 1, kind: 2, reading: ratio(3, 1) },
        5,
        vec![100],
    );
    b.append(ClosureEvent::ZoneRazed { orb_id: 1, tile: 8 }, 5, vec![100]);
    let refused = consile(&a, &b).unwrap_err();
    let MeshRefused::Diverged { census, .. } = refused else {
        panic!("must refuse DIVERGED");
    };
    // B's raze refunds (a rational residual carries it); the census
    // still names BOTH structural differences, orb first then the
    // families in code order.
    assert_eq!(census, vec![(CENSUS_ORB, 1), (CENSUS_WANTS, 0)]);
}

#[test]
fn two_histories_one_state_measure_zero_whole() {
    // Each side strikes and relieves a DIFFERENT want: the spines
    // fork, the states land identical. The judgment still refuses
    // (the spine is the spine) but the measurement reads zero whole —
    // empty residuals AND empty census. The confluence precondition.
    let (mut a, mut b, fork) = a_forked_pair("one-state");
    a.append(
        ClosureEvent::WantStruck { orb_id: 1, kind: 2, reading: ratio(3, 1) },
        5,
        vec![100],
    );
    a.append(ClosureEvent::WantRelieved { orb_id: 1, kind: 2 }, 6, vec![100]);
    b.append(
        ClosureEvent::WantStruck { orb_id: 1, kind: 3, reading: ratio(3, 1) },
        5,
        vec![100],
    );
    b.append(ClosureEvent::WantRelieved { orb_id: 1, kind: 3 }, 6, vec![100]);
    let refused = consile(&a, &b).unwrap_err();
    let MeshRefused::Diverged { at, residuals, coherence, census } = refused else {
        panic!("must refuse DIVERGED");
    };
    assert_eq!(at, fork);
    assert!(residuals.is_empty(), "one state — nothing measured moved");
    assert!(census.is_empty(), "one state — nothing censused differs");
    assert_eq!(coherence, ratio(0, 1), "two histories, one state, zero whole");
}

#[test]
fn the_census_rides_the_wire_additively() {
    let refusal = MeshRefused::Diverged {
        at: 5,
        residuals: vec![(1, ratio(-3, 7))],
        coherence: ratio(54, 49),
        census: vec![(0, 0), (CENSUS_ORB, 7), (18, 0)],
    };
    let bytes = refusal.wire_bytes();
    let (tag, value) = tlv::records(&bytes).next().expect("one record");
    let decoded = MeshRefused::from_wire(tag, value).expect("decodes");
    assert_eq!(decoded, refusal);
    assert_eq!(decoded.wire_bytes(), bytes, "equal values, equal bytes");

    // The additive law: a pre-census record (the trailing bytes
    // absent) decodes census-empty — the landed meaning never moved.
    let mut pre_census = MeshRefused::Diverged {
        at: 5,
        residuals: vec![(1, ratio(-3, 7))],
        coherence: ratio(54, 49),
        census: Vec::new(),
    }
    .wire_bytes();
    // strip the empty census trailer (LE32 count = 4 bytes) from the
    // record value AND fix the TLV length prefix.
    let value_len = pre_census.len() - 5;
    pre_census.truncate(pre_census.len() - 4);
    let new_len = (value_len - 4) as u32;
    pre_census[1..5].copy_from_slice(&new_len.to_le_bytes());
    let (tag, value) = tlv::records(&pre_census).next().expect("one record");
    let decoded = MeshRefused::from_wire(tag, value).expect("the old shape decodes");
    let MeshRefused::Diverged { census, .. } = decoded else {
        panic!("still DIVERGED");
    };
    assert!(census.is_empty(), "absent trailer reads census-empty");
}
