//! **DO TWO CHARACTERS ON ONE BOARD THINK DIFFERENTLY?**
//!
//! The question the wants-move exists to answer. Before it, `Entity`
//! carried `entity_id`, `handle`, `principal`, `grants` — and two
//! characters with the same grants were identical BY CONSTRUCTION.
//! There was no field in which they could differ.
//!
//! Hunger is now read from each line's OWN history, so it is the first
//! thing about a character that is its own. This measures whether that
//! actually separates them, or whether they all starve together and the
//! difference is cosmetic.
//!
//! Usage: cargo run -p conductor --release --example two_characters

use solver::{propose_measured, Difficulty, Hunger, Need, SolveRead};
use solver_verify::forge_task;

fn describe(need: &Need) -> String {
    match need.hungers.as_slice() {
        [] => "sated".into(),
        many => many
            .iter()
            .map(|h| match h {
                Hunger::Novelty => "novelty",
                Hunger::Resonance => "resonance",
            })
            .collect::<Vec<_>>()
            .join("+"),
    }
}

fn main() {
    // Four characters, differing ONLY in what they are starved for —
    // same board, same forge, same rung. Anything that separates them
    // is the hunger and nothing else.
    let characters: [(&str, Need); 4] = [
        ("a fed line", Need::sated()),
        ("starved of links", Need::struck(Hunger::Resonance, 240)),
        ("starved of signal", Need::struck(Hunger::Novelty, 480)),
        (
            "starved of both",
            Need { hungers: vec![Hunger::Novelty, Hunger::Resonance], starved_for: 900 },
        ),
    ];

    println!("Four characters, one board. Same tasks, same rung — only the hunger differs.\n");
    println!(
        "{:<20} {:>8} {:>8} {:>9} {:>8}",
        "character", "reach", "tested", "solved", "depth"
    );
    println!("{}", "-".repeat(58));

    let mut totals = Vec::new();
    for (name, need) in &characters {
        let (mut reach, mut tested, mut solved) = (0usize, 0usize, 0usize);
        for seed in 0..40u8 {
            let digest = [seed.wrapping_mul(7).wrapping_add(3); 32];
            let task = forge_task(digest, Difficulty::level(3));
            let read = propose_measured(&task, need);
            reach += read.reach;
            tested += read.tested;
            if matches!(read.read, SolveRead::Solved { .. }) {
                solved += 1;
            }
        }
        println!(
            "{:<20} {:>8} {:>8} {:>9} {:>8}",
            describe(need),
            reach,
            tested,
            solved,
            need.compose_limit()
        );
        totals.push((name.to_string(), describe(need), tested, solved));
    }

    // THE QUESTION, ANSWERED IN ONE LINE.
    let distinct_effort: std::collections::BTreeSet<usize> =
        totals.iter().map(|(_, _, tested, _)| *tested).collect();
    let distinct_yield: std::collections::BTreeSet<usize> =
        totals.iter().map(|(_, _, _, solved)| *solved).collect();

    println!("\ndistinct search efforts: {}/4", distinct_effort.len());
    println!("distinct solve counts:   {}/4", distinct_yield.len());
    println!(
        "\n{}",
        match (distinct_effort.len() > 1, distinct_yield.len() > 1) {
            (true, true) => "THEY DIFFER IN BOTH — hunger changes how hard a line looks AND what it finds.",
            (true, false) =>
                "THEY DIFFER IN EFFORT ONLY — hunger changes how hard a line looks, not what it finds. \
                 The reach is the same set walked in a different order and to a different depth.",
            (false, _) => "THEY DO NOT DIFFER — the hunger is cosmetic and the move did not land.",
        }
    );
}
