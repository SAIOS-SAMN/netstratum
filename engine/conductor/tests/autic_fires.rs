// AUTIC-1 + AUTIC-2 must FIRE, not be dormant. Prime net's AUTIC basin (the
// warm-start path, deterministic — no mesh needed), then run the real engine and
// assert the entity bridge fires end to end: vessels absorb the shared crystals
// (AUTIC-1) and speak/hear composed sentences against them (AUTIC-2).
use reduct::congruence::ChainEntry;
use reduct::peel::{CellFrame, GaugeDirection, Site};
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

#[test]
fn the_autic_bridge_fires_end_to_end() {
    // Prime a dozen distinct crystals as net's learned language.
    let mut primed = Vec::new();
    for i in 0..12i64 {
        primed.push(crystal(i % 3, i / 3, (i % 4) as u8 + 1));
    }
    scribe::membrane::prime_autic_basin(primed);
    assert!(scribe::membrane::autic_basin_depth() >= 12, "basin primed");

    let root = std::env::temp_dir().join(format!("auticE-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let worlds = conductor::run_era(&root.to_string_lossy(), 40);
    let absorbed: usize = worlds.iter().map(|p| p.autic_absorbed).sum();
    let spoke: usize = worlds.iter().map(|p| p.autic_spoke).sum();
    let learned: usize = worlds.iter().map(|p| p.autic_learned).sum();
    let max_basin: usize = worlds.iter().map(|p| p.max_basin()).max().unwrap_or(0);
    eprintln!("AUTIC END-TO-END: absorbed={absorbed} spoke={spoke} learned={learned} max_basin={max_basin} across {} civs", worlds.len());
    assert!(absorbed > 0, "AUTIC-1 absorb never fired despite a primed basin");
    assert!(spoke > 0, "AUTIC-2 COMPOSE move never fired");
}
