//! CONVERGENCE, EXTRAPOLATED FROM THE TWO POLES — D and V across an
//! era fold.
//!
//! Operator ruling 2026-08-04: extrapolate convergence from the two
//! poles, and read it at the era fold (CONVERGE-1 — annexation onto
//! `TUniversal{S, G, T}`, the torn-tail law, four-C placement).
//!
//! The fold is the one place this engine already has a before and an
//! after for the same civilizations: `conductor::fold` seals the live
//! era into `<base>-era<N>` and the next boot ANNEXES it. So the
//! question "do the poles converge?" is answerable by measuring them on
//! either side of that seam.
//!
//! **BOTH POLES, NEVER THEIR SUM.** D is the extent (floored at GROUND,
//! never zero) and V the flow (zero is REST, lawfully). Each has a case
//! the other cannot see — measured in both directions in
//! `rfa-core/examples/capacity_climb.rs` and
//! `toroid_encapsulation.rs` — so a single convergence number would
//! hide which pole moved. Every row below carries the pair.
//!
//! Usage: cargo run -p conductor --release --example fold_poles -- [beats]

use cfe::dop::ColonyOrb;
use cfe::ratio::ratio;
use cfe::world::{Bond, Committed, World};
use conductor::{fold, run_era};
use kernel::bridge::read_entangled;

struct Reading {
    world: String,
    d: usize,
    v: usize,
    orbs: usize,
    receipts: usize,
}

fn poles(var: &str, beats: u32) -> Vec<Reading> {
    let worlds = run_era(var, beats);
    let mut out: Vec<Reading> = worlds
        .iter()
        .map(|p| {
            let committed = p.world.committed();
            let read = read_entangled(&committed);
            Reading {
                world: p.spec.world_name.clone(),
                d: read.degree().get(),
                v: read.flows.len(),
                orbs: committed.orbs.len(),
                receipts: p.world.chronicle().receipts().len(),
            }
        })
        .collect();
    out.sort_by(|a, b| a.world.cmp(&b.world));
    out
}

fn table(label: &str, rows: &[Reading]) {
    println!("{label}");
    println!("   {:<26} {:>5} {:>5} {:>6} {:>10}", "world", "D", "V", "orbs", "receipts");
    for r in rows {
        println!(
            "   {:<26} {:>5} {:>5} {:>6} {:>10}",
            &r.world[..r.world.len().min(26)],
            r.d,
            r.v,
            r.orbs,
            r.receipts
        );
    }
    let (d, v): (usize, usize) = rows.iter().fold((0, 0), |(d, v), r| (d + r.d, v + r.v));
    println!("   {:<26} {:>5} {:>5}", "— totals —", d, v);
}

/// THE INSTRUMENT'S OWN CONTROL — does the reading MOVE when structure
/// exists? Without this, "both poles converged" and "the probe is
/// blind" are indistinguishable.
fn instrument_control() {
    let mut state: Committed = World::new().committed();
    state.orbs = (1..=3u64)
        .map(|orb_id| ColonyOrb { orb_id, name: format!("orb{orb_id}"), ..Default::default() })
        .collect();
    let bond = |source, target, rate: i64| Bond {
        source,
        target,
        resource: cfe::dop::Resource::Iron,
        rate: ratio(rate, 1),
        term: 0,
        sealed: 0,
        anchored: 0,
    };
    let ground = read_entangled(&state);
    // A triangle that does not compose: 1 + 2 != 7, so it carries
    // holonomy no frame explains.
    state.bonds = vec![bond(1, 2, 1), bond(2, 3, 2), bond(1, 3, 7)];
    let standing = read_entangled(&state);
    println!("\n   INSTRUMENT CONTROL — the same reading on a 3-orb world:");
    println!(
        "      no obligations      D {}  V {}",
        ground.degree().get(),
        ground.flows.len()
    );
    println!(
        "      non-composing triad D {}  V {}",
        standing.degree().get(),
        standing.flows.len()
    );
    println!("   The probe RESPONDS to structure. The flat reading above is the");
    println!("   board's, not the instrument's.");
}

fn main() {
    let beats: u32 = std::env::args()
        .nth(1)
        .map(|b| b.parse().expect("beats"))
        .unwrap_or(30);

    let mut root = std::env::temp_dir();
    root.push(format!("netstratum-fold-poles-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("scratch");
    let var = root.to_string_lossy().into_owned();

    println!("CONVERGENCE FROM THE TWO POLES — {beats} beats an era\n");

    let before = poles(&var, beats);
    table("ERA 1 — before the fold", &before);

    match fold(&var) {
        Ok((era, gauge)) => println!("\nFOLDED — era {era} sealed; the new sheet seats at G = {gauge}"),
        Err(refusal) => {
            println!("\nFOLD REFUSED — {refusal}. Nothing to compare; stopping.");
            return;
        }
    }

    // The next era CONVERGES the sealed one: `run_era` calls
    // `ledger::converge` before it founds anything.
    let after = poles(&var, beats);
    println!();
    table("ERA 2 — after the fold, annexing era 1", &after);

    // ── the extrapolation ────────────────────────────────────────────
    let (d0, v0) = before.iter().fold((0, 0), |(d, v), r| (d + r.d, v + r.v));
    let (d1, v1) = after.iter().fold((0, 0), |(d, v), r| (d + r.d, v + r.v));
    println!("\nACROSS THE FOLD");
    println!("   D  {d0} → {d1}");
    println!("   V  {v0} → {v1}");

    // Convergence is read PER POLE. Collapsing them would answer with
    // one number and hide which one moved — the failure `dual_pole`
    // and the V ∪ D benchmark both demonstrated.
    // COVERAGE FIRST. If every world sits at the FLOOR (D = GROUND = 1)
    // and at REST (V = 0), then both poles "converged" only because
    // neither had anywhere to be. A check over an empty set decides
    // nothing, and reporting it as convergence would be exactly the
    // vacuity this lane has spent the session removing.
    let all_ground = before.iter().chain(after.iter()).all(|r| r.d == 1 && r.v == 0);
    if all_ground {
        println!("\n   UNMEASURED — every world reads D = GROUND and V = REST, on");
        println!("   BOTH sides of the fold. There is no structure to carry across,");
        println!("   so this comparison cannot distinguish a capacity-preserving");
        println!("   annexation from a destructive one. It is not a convergence");
        println!("   result; it is the absence of anything to converge.");
        println!("\n   WHY: every world holds ONE orb (see the table), and the");
        println!("   bridge projects PAIRWISE obligations only. A one-orb world has");
        println!("   no pair, so D sits at the floor and V at rest by construction.");
        println!("   The conductor's play seals no inter-orb bond, covenant or");
        println!("   debt, so the live board never leaves the ground.");
        instrument_control();
        return;
    }

    let d_converged = d1 == d0;
    let v_converged = v1 == v0;
    println!(
        "\n   D converged: {d_converged}      V converged: {v_converged}"
    );
    match (d_converged, v_converged) {
        (true, true) => println!(
            "   BOTH POLES CONVERGE — the fold carried the state across without\n   \
             moving either reading. Annexation is capacity-neutral."
        ),
        (true, false) => println!(
            "   ONLY D CONVERGES — the extent survives the fold and the FLOW does\n   \
             not. The seam costs motion, never room."
        ),
        (false, true) => println!(
            "   ONLY V CONVERGES — the flow survives and the EXTENT does not. The\n   \
             fold is losing structure, which is the reading that would matter\n   \
             most: annexation should not cost capacity."
        ),
        (false, false) => println!(
            "   NEITHER CONVERGES — the fold moves both poles. That is not a\n   \
             failure on its own (era 2 plays its own beats), but it means the\n   \
             fold cannot be read as capacity-preserving without isolating the\n   \
             annexation from the play that follows it."
        ),
    }
}
