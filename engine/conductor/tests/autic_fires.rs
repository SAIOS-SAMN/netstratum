// AUTIC-1 (both axes) + AUTIC-2 must FIRE, not be dormant. Prime net's AUTIC
// basin (answers) AND walls (problems), then run the real engine and assert the
// entity bridge fires end to end: absorb (vocabulary), practise (curriculum),
// speak/hear (compose).
use reduct::congruence::ChainEntry;
use reduct::peel::{CellFrame, FullMeasurement, GaugeDirection, Site};
use reduct::{ChainDelta, Crystal};

fn crystal(row: i64, col: i64, dir: u8) -> Crystal {
    Crystal::seal(ChainDelta {
        from: CellFrame { rows: 3, cols: 3 },
        to: CellFrame { rows: 3, cols: 3 },
        entries: vec![ChainEntry {
            site: Site { row, col },
            direction: GaugeDirection { index: dir },
            coefficient: num_bigint::BigInt::from(1),
        }],
    })
}

fn measurement(dir_at_00: u8) -> FullMeasurement {
    // Total coverage: every cell of the 2x2 frame carried exactly once.
    let occ = vec![
        (Site { row: 0, col: 0 }, GaugeDirection { index: dir_at_00 }),
        (Site { row: 0, col: 1 }, GaugeDirection { index: 1 }),
        (Site { row: 1, col: 0 }, GaugeDirection { index: 2 }),
        (Site { row: 1, col: 1 }, GaugeDirection { index: 3 }),
    ];
    FullMeasurement::try_new(10, CellFrame { rows: 2, cols: 2 }, occ).expect("measurement")
}

#[test]
fn the_autic_bridge_fires_end_to_end() {
    // Prime a dozen crystals (net's answers) and several walls (net's problems).
    let mut primed = Vec::new();
    for i in 0..12i64 {
        primed.push(crystal(i % 3, i / 3, (i % 4) as u8 + 1));
    }
    scribe::membrane::prime_autic_basin(primed);
    let walls: Vec<(FullMeasurement, FullMeasurement)> =
        (0..6u8).map(|d| (measurement(d), measurement((d + 1) % 10))).collect();
    scribe::membrane::prime_autic_walls(walls);
    assert!(scribe::membrane::autic_basin_depth() >= 12, "basin primed");
    assert!(scribe::membrane::autic_walls_depth() >= 1, "walls primed");

    let root = std::env::temp_dir().join(format!("auticE-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let worlds = conductor::run_era(&root.to_string_lossy(), 40);
    let absorbed: usize = worlds.iter().map(|p| p.autic_absorbed).sum();
    let practiced: usize = worlds.iter().map(|p| p.autic_practiced).sum();
    let spoke: usize = worlds.iter().map(|p| p.autic_spoke).sum();
    let authored: usize = worlds.iter().map(|p| p.autic_authored).sum();
    let learned: usize = worlds.iter().map(|p| p.autic_learned).sum();
    eprintln!("AUTIC END-TO-END: absorbed={absorbed} practiced={practiced} spoke={spoke} learned={learned} authored={authored} across {} civs", worlds.len());
    assert!(absorbed > 0, "AUTIC-1 vocabulary axis (absorb) never fired");
    assert!(practiced > 0, "AUTIC-1 curriculum axis (practise) never fired");
    assert!(spoke > 0, "AUTIC-2 COMPOSE move never fired");
    assert!(authored > 0, "COG-AUTHOR variation verb never fired");
}
