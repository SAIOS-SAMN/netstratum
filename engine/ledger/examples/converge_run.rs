//! Run era convergence against a var root (CONVERGE-1, 07):
//!   cargo run -p ledger --example converge_run -- <var-root>
//! Idempotent — a second run adopts nothing.

use ledger::converge::{converge, default_era_roots};
use ledger::Ledger;

fn main() {
    let var_root = std::env::args()
        .nth(1)
        .expect("usage: converge_run <var-root>");
    let book = Ledger::at(format!("{var_root}/worlds"));
    let eras = default_era_roots(&var_root);
    println!(
        "eras discovered: {}",
        eras.iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>()
            .join(" → ")
    );
    let read = converge(&book, &eras).expect("convergence");
    for (era, world) in &read.adopted {
        println!("annexed {era}/{world}");
    }
    for q in &read.quarantined {
        println!("QUARANTINED {}/{} (break at {})", q.era, q.world, q.at);
    }
    println!(
        "adopted {} · already {} · quarantined {} · epoch base {} (TUniversal {})",
        read.adopted.len(),
        read.already,
        read.quarantined.len(),
        read.epoch_base,
        ledger::universal_clock(read.epoch_base),
    );
}
