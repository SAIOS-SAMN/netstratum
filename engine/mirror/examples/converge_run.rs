//! Run era convergence against a var root (CONVERGE-1, 07):
//!   cargo run -p mirror --example converge_run -- <var-root>
//! Idempotent — a second run adopts nothing.

use mirror::converge::{converge, default_era_roots};
use mirror::Mirror;

fn main() {
    let var_root = std::env::args()
        .nth(1)
        .expect("usage: converge_run <var-root>");
    let mirror = Mirror::at(format!("{var_root}/worlds"));
    let eras = default_era_roots(&var_root);
    println!(
        "eras discovered: {}",
        eras.iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>()
            .join(" → ")
    );
    let read = converge(&mirror, &eras).expect("convergence");
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
        mirror::universal_clock(read.epoch_base),
    );
}
