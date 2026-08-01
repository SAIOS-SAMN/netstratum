//! MESH-1b #2 — THE PEN (the consumer slice). The full arc, one
//! witness: a conductor world beats; the CLI's ask lands on the
//! control file; the beat boundary seals the pen over (walk names
//! the seat); the seat's deeds append lawfully ON THE FILE, hand
//! carried, seal cited; the yield back returns the pen; the
//! conductor re-resumes FROM THE RECEIPTS with the seat's deeds
//! kept. The walk is the judge throughout — a mis-stamped funnel
//! would refuse USURPED/UNCITED and fail this pin.

use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::vrp::{hand_of, CONDUCTOR_HAND};
use cfe::world::World;
use conductor::{pen_service, Polity, PolitySpec, Policy};
use ledger::registry::{Civilization, Registry};
use ledger::Ledger;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pen-pin-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn seat(sigil: &str) -> PolitySpec {
    PolitySpec {
        civilization: Civilization {
            name: sigil.to_string(),
            sigil: sigil.to_string(),
            species: "pen witness".into(),
            clade: String::new(),
            motto: "one writer per clock".into(),
            disposition: "empiricist".into(),
            metabolism: "biotic".into(),
            perception: "planar".into(),
            founder: "conductor".into(),
            lineage: Vec::new(),
            station: None,
        },
        world_name: format!("{sigil}-hold"),
        planet: format!("{sigil}-p"),
        colony: format!("{sigil}-c"),
        policy: Policy::Steady,
        position: (ratio(0, 1), ratio(0, 1), ratio(0, 1)),
        star: Some(StarClass::G),
        albedo: Some(ratio(1, 4)),
        orbit: Some(ratio(1, 1)),
        gauge: None,
    }
}

#[test]
fn the_pen_passes_writes_and_returns() {
    let root = scratch("arc");
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let control = root.join("conductor.ctl");
    let mut board = vec![Polity::genesis(seat("penhold"), &registry, &book)];
    for _ in 0..3 {
        board[0].cycle(&book, &registry);
    }
    let beats_before = board[0].world.now();

    // The ask lands beside paused/cadence lines; only the yield is
    // consumed by the service.
    std::fs::write(&control, "cadence 5\nyield penhold-hold visible-hand\n").unwrap();
    let log = pen_service(&mut board, &registry, &book, &control, 100);
    assert!(log.iter().any(|l| l.contains("the pen passes")), "sealed: {log:?}");
    assert!(board[0].penned, "beats wait while the pen is out");
    assert_eq!(
        std::fs::read_to_string(&control).unwrap(),
        "cadence 5\n",
        "the yield line is consumed; the operator's lines stay"
    );

    // The taking hand works ON THE FILE — the CLI's exact moves.
    let carrier = book.world_dir("penhold-hold").join("chronicle.tlv");
    let chronicle = cfe::persist::load(&carrier).expect("the sealed arbor verifies");
    let hand = hand_of("visible-hand");
    assert_eq!(mesh::authority(&chronicle), Ok(Some(hand)), "the walk names the seat");
    let mut held = World::resume(chronicle);
    held.hold_pen(Some(hand));
    held.tick();
    let _ = held.collect();
    // The first append cited the seal and every receipt carries the
    // hand — or this walk refuses.
    assert_eq!(mesh::authority(held.chronicle()), Ok(Some(hand)), "the deeds walk lawful");

    // The yield back, synced to the file.
    let stamp = held.chronicle().receipts().last().unwrap().stamp;
    mesh::yield_authority(held.chronicle_mut(), hand, CONDUCTOR_HAND, stamp)
        .expect("the return seal");
    let deeds_len = held.chronicle().receipts().len();
    cfe::persist::sync(held.chronicle(), &carrier).expect("synced home");

    // The poll notices; the conductor re-resumes FROM the receipts.
    let log = pen_service(&mut board, &registry, &book, &control, 101);
    assert!(log.iter().any(|l| l.contains("the pen returns")), "returned: {log:?}");
    assert!(!board[0].penned, "beats resume");
    assert!(
        board[0].world.chronicle().receipts().len() >= deeds_len,
        "the seat's deeds are KEPT — the file is the world (SEAL B)"
    );
    assert!(board[0].world.now() > beats_before, "the held tick survived the return");

    // And the next conductor beat appends lawfully (cites the return
    // seal through the funnel) — the walk stays green.
    board[0].cycle(&book, &registry);
    assert_eq!(
        mesh::authority(board[0].world.chronicle()),
        Ok(Some(CONDUCTOR_HAND)),
        "the conductor's own appends carry its hand and cite the return"
    );
}
