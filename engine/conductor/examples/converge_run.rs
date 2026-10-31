//! Converge run — perform the same era-convergence pass `main.rs` runs at
//! boot, standalone, without paying the ~9-minute live-roster genesis cost.
//! Real effect (not a dry run): verifies each archived world's chain,
//! quarantines broken ones, places valid ones on the shared star chart, and
//! appends `EraAnnexed` receipts to `var/worlds/convergence.tlv` — exactly
//! what boot does, idempotently (already-annexed worlds are skipped).
//!
//! ```text
//! cargo run --release -p conductor --example converge_run -- [var_root]
//! ```

fn main() {
    let var_root = std::env::args().nth(1).unwrap_or_else(|| "var".to_string());
    std::fs::create_dir_all(&var_root).expect("var root");
    let book = ledger::Ledger::at(format!("{var_root}/worlds"));
    let eras = ledger::converge::default_era_roots(&var_root);
    println!(
        "era roots found: {:?}",
        eras.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );
    let converged = ledger::converge::converge(&book, &eras).expect("convergence");
    println!(
        "converged {} worlds ({} already annexed, {} quarantined) — epoch base {}",
        converged.adopted.len(),
        converged.already,
        converged.quarantined.len(),
        converged.epoch_base,
    );
    for (era, world) in &converged.adopted {
        println!("  adopted  {era}/{world}");
    }
    for q in &converged.quarantined {
        println!("  QUARANTINED {}/{} (break at {})", q.era, q.world, q.at);
    }
}
