//! IS THE ERA DETERMINISTIC? — SEAL B asked directly.
//!
//! The conductor's own header claims it: *"Everything is deterministic:
//! same era length → byte-identical receipts."* Three runs of
//! `mint_loop_pins` over the same 60 beats produced three different
//! economies (296/56/272, 297/57/276, 328/48/234), so the claim needs a
//! direct test rather than an inference.
//!
//! This runs the SAME era twice IN ONE PROCESS, each into its own fresh
//! process-unique root, and compares receipt-for-receipt. Controlling
//! the roots separates the two candidate causes:
//!
//!   differ  -> the ENGINE is non-deterministic; SEAL B is violated.
//!   same    -> the engine is fine and the variance is fixture state
//!              leaking between runs (`mint_loop_pins` is the one test
//!              in the tree without a process-unique temp dir, and
//!              `default_era_roots` scans the PARENT directory, where
//!              959 leftover fixture dirs are currently sitting).
//!
//! Usage: cargo run -p conductor --release --example determinism_probe -- [beats]

use conductor::run_era;

fn era(tag: &str, beats: u32) -> Vec<(String, Vec<String>)> {
    let mut root = std::env::temp_dir();
    root.push(format!(
        "netstratum-determinism-{tag}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("scratch");
    let worlds = run_era(&root.to_string_lossy(), beats);
    // Every world, and every receipt's event rendered — the comparable
    // shape. Sorted by world name so vector order cannot itself differ.
    let mut out: Vec<(String, Vec<String>)> = worlds
        .iter()
        .map(|p| {
            (
                p.spec.world_name.clone(),
                p.world
                    .chronicle()
                    .receipts()
                    .iter()
                    .map(|r| format!("{}:{:?}", r.sequence(), r.event()))
                    .collect(),
            )
        })
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn main() {
    let beats: u32 = std::env::args()
        .nth(1)
        .map(|b| b.parse().expect("beats"))
        .unwrap_or(30);

    println!("DETERMINISM PROBE — the same {beats}-beat era, twice, one process\n");
    let a = era("a", beats);
    let b = era("b", beats);

    println!("run A: {} worlds, {} receipts", a.len(), a.iter().map(|w| w.1.len()).sum::<usize>());
    println!("run B: {} worlds, {} receipts", b.len(), b.iter().map(|w| w.1.len()).sum::<usize>());

    if a == b {
        println!("\nIDENTICAL — receipt for receipt, across every world.");
        println!("The engine replays. The variance seen in `mint_loop_pins` is");
        println!("fixture state, not SEAL B.");
        return;
    }

    println!("\nDIFFERENT — SEAL B does not hold for a fresh era.\n");
    if a.len() != b.len() {
        println!("  world COUNT differs: {} vs {}", a.len(), b.len());
    }
    for (wa, wb) in a.iter().zip(b.iter()) {
        if wa.0 != wb.0 {
            println!("  world NAME differs: {} vs {}", wa.0, wb.0);
            break;
        }
        if wa.1 != wb.1 {
            println!("  first divergence in world {}", wa.0);
            println!("    receipts: {} vs {}", wa.1.len(), wb.1.len());
            for (i, (ra, rb)) in wa.1.iter().zip(wb.1.iter()).enumerate() {
                if ra != rb {
                    println!("    at receipt {i}:");
                    println!("      A  {}", &ra[..ra.len().min(140)]);
                    println!("      B  {}", &rb[..rb.len().min(140)]);
                    break;
                }
            }
            break;
        }
    }
}
