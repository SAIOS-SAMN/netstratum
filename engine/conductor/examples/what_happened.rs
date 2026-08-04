//! **WHERE DID THE SOLVES GO?**
//!
//! The board once churned out roughly a solve per line per beat. It no
//! longer does, and "brute force was removed" is not an answer — it is
//! a guess about a number nobody re-measured.
//!
//! So this runs the SAME era under each search mode and reports what
//! each one costs and yields, per line and in total. `Uncoupled` is the
//! sweep the board used to run; it is kept as a live CONTROL rather
//! than a remembered figure, which is the only reason this comparison
//! is possible at all.
//!
//! Usage: cargo run -p conductor --release --example what_happened -- [beats]

use conductor::{run_era_with, SearchMode};

fn main() {
    let beats: u32 = std::env::args()
        .nth(1)
        .and_then(|n| n.parse().ok())
        .unwrap_or(60);

    println!("THE SAME ERA, {beats} beats, under each mode\n");
    println!(
        "{:<12} {:>7} {:>7} {:>8} {:>9} {:>9} {:>7}",
        "mode", "lines", "solves", "mints", "earned", "read-cand", "sweeps"
    );
    println!("{}", "-".repeat(66));

    let mut rows = Vec::new();
    for (name, mode) in [
        ("uncoupled", SearchMode::Uncoupled),
        ("coupled", SearchMode::Coupled),
        ("union", SearchMode::Union),
    ] {
        let board = run_era_with("var", beats, mode);
        let lines = board.len();
        let solves: u64 = board.iter().map(|p| p.solved as u64).sum();
        let spent: u64 = board.iter().map(|p| p.spent).sum();
        let swept: u64 = board.iter().map(|p| p.swept).sum();
        let mints: u64 = board
            .iter()
            .map(|p| {
                p.world
                    .chronicle()
                    .receipts()
                    .iter()
                    .filter(|r| {
                        matches!(r.event, cfe::dop::ClosureEvent::DiscoveryMinted { .. })
                    })
                    .count() as u64
            })
            .sum();
        println!(
            "{name:<12} {lines:>7} {solves:>7} {mints:>8} {:>9} {spent:>9} {swept:>7}",
            "-"
        );
        rows.push((name, lines, solves, spent, swept));
    }

    // THE CEILING: one solve per line per beat is what "churning"
    // looked like. Everything below it is either a harder rung or a
    // reach that saw nothing.
    if let Some((_, lines, solves, _, _)) = rows.first() {
        let ceiling = (*lines as u64) * beats as u64;
        println!("\nthe churn ceiling ({lines} lines x {beats} beats) is {ceiling} solve-opportunities");
        for (name, _, solves_n, spent, swept) in &rows {
            let pct = solves_n * 100 / ceiling.max(1);
            println!(
                "  {name:<11} took {pct:>3}% of them   ({solves_n} solves, {spent} read-candidates, {swept} sweeps)"
            );
        }
        let _ = solves;
    }
    println!(
        "\nA solve here is a FORGE solve and self-checking: the task re-derives\n\
         from its digest and the render must match the forge's own answer. So a\n\
         difference between modes is a difference in what was FOUND, never in\n\
         what was counted."
    );
}
