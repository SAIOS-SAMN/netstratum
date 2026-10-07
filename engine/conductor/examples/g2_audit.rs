//! THE G2 AUDIT — a universe's chronicles read against docs/57_GENESIS_G2.md.
//!
//!   cargo run --release -p conductor --example g2_audit -- [var-g2]
//!
//! Reads every `<root>/worlds/*/chronicle.tlv` through the verifying load
//! and reports, per world, the checklist items a chronicle can show on its
//! own: the founding band (#2), the headcount path (#3), famine pairing
//! (#6), the discovery pulse (#9), STAMP minted (#10), cleaves (#11) and
//! collapse (#13). A reading, never a gate: it writes nothing.

use cfe::dop::ClosureEvent as E;
use std::collections::BTreeMap;

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| "var-g2".to_string());
    let mut dirs: Vec<_> = std::fs::read_dir(format!("{root}/worlds"))
        .expect("a worlds directory")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.join("chronicle.tlv").exists())
        .collect();
    dirs.sort();
    println!(
        "{:<18} {:>6} {:>5} {:>5} {:>5} {:>5} {:>7} {:>9} {:>9} {:>4} {:>4}",
        "world", "rcpts", "band", "pop", "peak", "dips", "famine", "unpaired", "discov", "div", "fall"
    );
    let mut flags = Vec::new();
    for dir in dirs {
        let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("?").to_string();
        let chronicle = match cfe::persist::load(&dir.join("chronicle.tlv")) {
            Ok(c) => c,
            Err(e) => {
                println!("{name:<18} the chronicle did not load: {e:?}");
                flags.push(format!("{name}: chronicle refused"));
                continue;
            }
        };
        let mut pop: BTreeMap<u64, i64> = BTreeMap::new();
        let (mut peak, mut dips, mut band_ok, mut bands) = (0i64, 0u32, None::<bool>, 0u32);
        let mut open: BTreeMap<(u64, String), u32> = BTreeMap::new();
        let (mut struck, mut discov, mut diverged, mut fell) = (0u32, Vec::new(), 0u32, false);
        for r in chronicle.receipts() {
            match r.event() {
                E::PopulationSettled { orb_id, delta } => {
                    if band_ok.is_none() {
                        band_ok = Some(*delta == 4);
                    }
                    if *delta == 4 {
                        bands += 1;
                    }
                    let held = pop.entry(*orb_id).or_default();
                    *held += delta;
                    if *delta < 0 {
                        dips += 1;
                    }
                    if *held < 0 {
                        flags.push(format!("{name}: orb {orb_id} headcount below zero ({held})"));
                    }
                    peak = peak.max(pop.values().sum());
                }
                E::FamineStruck { orb_id, resource, .. } => {
                    struck += 1;
                    *open.entry((*orb_id, format!("{resource:?}"))).or_default() += 1;
                }
                E::FamineRelieved { orb_id, resource } => {
                    if let Some(n) = open.get_mut(&(*orb_id, format!("{resource:?}"))) {
                        *n = n.saturating_sub(1);
                    }
                }
                E::DiscoveryMinted { amount, .. } => discov.push(amount.to_string()),
                E::Diverged { .. } => diverged += 1,
                E::CivilizationCollapsed { .. } => fell = true,
                _ => {}
            }
        }
        let now: i64 = pop.values().sum();
        let unpaired: u32 = open.values().sum();
        if band_ok == Some(false) {
            flags.push(format!("{name}: first PopulationSettled is not the +4 founding band"));
        }
        if diverged > 1 {
            flags.push(format!("{name}: cleaved {diverged} times (one per line)"));
        }
        if fell {
            flags.push(format!("{name}: COLLAPSED"));
        }
        let band = match band_ok {
            Some(true) => format!("+4x{bands}"),
            Some(false) => "BAD".into(),
            None => "-".into(),
        };
        let discov = if discov.is_empty() { "-".to_string() } else { discov.join(",") };
        println!(
            "{name:<18} {:>6} {band:>5} {now:>5} {peak:>5} {dips:>5} {struck:>7} {unpaired:>9} {discov:>9} {diverged:>4} {:>4}",
            chronicle.receipts().len(),
            if fell { "YES" } else { "-" }
        );
    }
    if flags.is_empty() {
        println!("\nno checklist breach readable from the chronicles");
    } else {
        println!("\nflags:");
        for f in flags {
            println!("  {f}");
        }
    }
}
