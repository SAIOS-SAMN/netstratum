//! VENT 1 — THE YIELD OF DATA-DEFINED OPERATORS.
//!
//! Vent 1 is the seam where new operator DATA enters: a new
//! parameterisation inside a shape the interpreter already knows.
//! `Op::Recolor([u8; 10])` is the existence proof — behaviour carried
//! as data, executed by sealed code — and `Op::Tile(down, across,
//! mirror)` is the second instance.
//!
//! **What this does NOT measure, deliberately.** The relayed directive
//! proposed iterating a `data_space` of parameterisations and counting
//! closures. That measures brute force, not the vent: the recolor space
//! is `10! = 3_628_800`, which is 18× the entire `SEARCH_BUDGET`, so an
//! enumerator never reaches a recolor at all. `search::solve` never
//! enumerated it either — it has derived recolors from the pairs since
//! before this measurement existed. There is no enumeration baseline to
//! race against, because enumeration was never on the table.
//!
//! **What the vent's energy actually is:** a data-defined operator lets
//! the engine NAME one point in a space it could never search, from the
//! observation, in one candidate. So the measurement is threefold:
//!
//!   1. ATTRIBUTION — what share of confirmed closures carry a
//!      data-defined op? That is the yield vent 1 is holding up.
//!   2. COLLAPSE — how large is the space each such op names? That is
//!      the energy drawn per candidate spent.
//!   3. THE ASSUMPTION — `derive_recolors` fills every hue it never
//!      observed with IDENTITY. That is a guess, not a derivation, and
//!      the honest-withhold law elsewhere in this engine says one miss
//!      refuses rather than guesses. This counts how often the guess is
//!      LOAD-BEARING: the test input carrying a hue training never
//!      showed.
//!
//! Usage: cargo run -p solver --release --example vent1_data -- <dir>

use solver::{derive, load_task, propose_measured, Family, Grid, Need, Op, SolveRead};

/// The size of the space a data-defined operator names.
fn space_of(op: &Op) -> u64 {
    match op {
        // A hue permutation: 10! total maps.
        Op::Recolor(_) => (1..=10u64).product(),
        // down ∈ 1..=3, across ∈ 1..=3, times the mirror combinations.
        Op::Tile { .. } => 9 * 16,
        _ => 1,
    }
}

fn is_data_defined(op: &Op) -> bool {
    matches!(op, Op::Recolor(_) | Op::Tile { .. })
}

/// Every hue that appears anywhere in a grid.
fn hues(grid: &Grid, seen: &mut [bool; 10]) {
    for &cell in &grid.cells {
        if (cell as usize) < 10 {
            seen[cell as usize] = true;
        }
    }
}

fn main() {
    let dir = std::env::args()
        .nth(1)
        .expect("usage: vent1_data <corpus_dir>");
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("corpus dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "txt"))
        .collect();
    paths.sort();

    let (mut attempted, mut closed, mut confirmed) = (0u64, 0u64, 0u64);
    let (mut with_data, mut without_data) = (0u64, 0u64);
    let (mut conf_with_data, mut conf_without_data) = (0u64, 0u64);
    let mut tested_total = 0u64;
    // Derivation census over every task, closed or not.
    let (mut cand_total, mut cand_data) = (0u64, 0u64);
    // The identity assumption.
    let (mut recolor_confirmed, mut recolor_assumed) = (0u64, 0u64);
    let mut collapse_total = 0u64;

    for path in &paths {
        let Ok(task) = load_task(path) else { continue };
        attempted += 1;

        for candidate in derive(&task) {
            cand_total += 1;
            if matches!(candidate.family, Family::Recolor | Family::Tile) {
                cand_data += 1;
            }
        }

        // The most efficient operating point measured so far (sated:
        // 24 confirmed at 4 candidates/task, 36_364x the sweep).
        let proposal = propose_measured(&task, &Need::sated());
        tested_total += proposal.tested as u64;
        let SolveRead::Solved { program, matches_expected, .. } = &proposal.read else {
            continue;
        };
        closed += 1;
        let carries_data = program.iter().any(is_data_defined);
        match carries_data {
            true => with_data += 1,
            false => without_data += 1,
        }
        for op in program.iter().filter(|op| is_data_defined(op)) {
            collapse_total += space_of(op);
        }
        if *matches_expected == Some(true) {
            confirmed += 1;
            match carries_data {
                true => conf_with_data += 1,
                false => conf_without_data += 1,
            }
        }

        // THE ASSUMPTION, counted only where a recolor actually carried
        // a confirmed closure: does the test input use a hue that no
        // training input ever showed? If so, `derive_recolors` guessed
        // identity for it and nothing in the training data checks that.
        if *matches_expected == Some(true)
            && program.iter().any(|op| matches!(op, Op::Recolor(_)))
        {
            recolor_confirmed += 1;
            let mut trained = [false; 10];
            for pair in &task.train {
                hues(&pair.input, &mut trained);
            }
            let mut tested_hues = [false; 10];
            hues(&task.test_in, &mut tested_hues);
            if (0..10).any(|h| tested_hues[h] && !trained[h]) {
                recolor_assumed += 1;
            }
        }
    }

    println!("VENT 1 — data-defined operators, over {attempted} tasks in {dir}\n");

    println!("DERIVATION CENSUS");
    println!("  candidates derived           {cand_total}");
    println!("  of which data-defined        {cand_data}");
    println!("  candidates tested (sated)    {tested_total}");
    println!();

    println!("ATTRIBUTION — which closures does vent 1 hold up?");
    println!("  closed                       {closed}   (with data-op {with_data}, without {without_data})");
    println!("  confirmed                    {confirmed}   (with data-op {conf_with_data}, without {conf_without_data})");
    if confirmed > 0 {
        println!(
            "  vent 1 share of confirmed    {conf_with_data}/{confirmed}"
        );
    }
    println!();

    println!("COLLAPSE — the space named, per candidate spent");
    println!("  10! (one recolor)            {}", (1..=10u64).product::<u64>());
    println!("  SEARCH_BUDGET                200000");
    println!("  recolor space / budget       {}x", (1..=10u64).product::<u64>() / 200_000);
    println!("  total space named            {collapse_total}");
    println!();

    println!("THE IDENTITY ASSUMPTION — is the guess load-bearing?");
    println!("  confirmed closures using a recolor      {recolor_confirmed}");
    println!("  ...whose TEST carries an untrained hue  {recolor_assumed}");
    match recolor_confirmed {
        0 => println!("  (no recolor carried a confirmed closure — nothing to assume)"),
        _ => println!(
            "  the identity fill was load-bearing in {recolor_assumed} of {recolor_confirmed}"
        ),
    }
}
