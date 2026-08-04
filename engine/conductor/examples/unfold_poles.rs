//! THE UNFOLD — does the capacity meter see the engine OPEN a plane?
//!
//! Operator correction 2026-08-04: *you are folding and should be
//! unfolding.* The previous probe measured the era FOLD — the seam
//! where the engine SEALS an era and archives it. That is the
//! contraction. The expansion is `shift_gauge_at`.
//!
//! **THE UNFOLD, as the engine defines it.** `world::shift_gauge_at`
//! (GAUGE-N, `11` §2.1) grows the graded gauge key. Opening a plane at
//! a grade the key does not yet hold is EARNED — one receipted mint per
//! new dimension, counted over `TechDiscovered { condition: 1.. }`,
//! `Diverged` and `Confluent`. Moving an already-open plane is free.
//! Genesis is `[0]`: one grade, and the key LENGTHENS as discoveries
//! land. That is the polynomial fiber gaining dimensions — the thing
//! `17_CAPACITY` means by a universe that grows.
//!
//! **THE QUESTION.** `D` is supposed to be the extent — how much room
//! stands. If the fiber unfolds and `D` does not move, then `D` is not
//! measuring the engine's dimensional growth; it is measuring something
//! else that happens to share the name.
//!
//! Held fixed: the obligations. Varied: only the number of open grades.
//! If `D` reads the unfold, it must move here and nowhere else.
//!
//! Usage: cargo run -p conductor --release --example unfold_poles

use cfe::dop::{ColonyOrb, Resource};
use cfe::ratio::ratio;
use cfe::world::{Bond, Committed, World};
use kernel::bridge::read_entangled;

fn triad() -> Committed {
    let mut state: Committed = World::new().committed();
    state.orbs = (1..=3u64)
        .map(|orb_id| ColonyOrb { orb_id, name: format!("orb{orb_id}"), ..Default::default() })
        .collect();
    let bond = |source, target, rate: i64| Bond {
        source,
        target,
        resource: Resource::Iron,
        rate: ratio(rate, 1),
        term: 0,
        sealed: 0,
        anchored: 0,
    };
    // A triangle that does not compose — 1 + 2 != 7 — so it carries
    // holonomy no frame explains. This is the ONLY structure present,
    // and it never changes below.
    state.bonds = vec![bond(1, 2, 1), bond(2, 3, 2), bond(1, 3, 7)];
    state
}

/// DOES THE LIVE BOARD UNFOLD NOW? The mechanism was wired into the
/// beat; this asks whether autonomous lines actually open planes.
fn live_board(beats: u32) {
    let mut root = std::env::temp_dir();
    root.push(format!("netstratum-unfold-live-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("scratch");
    let mut worlds = conductor::run_era(&root.to_string_lossy(), beats);
    // Driven explicitly: the unfold is not in the beat (see
    // `Polity::unfold` — wiring it there broke a replay pin).
    for p in worlds.iter_mut() {
        while p.unfold() {}
    }
    println!("\nTHE LIVE BOARD — {beats} beats, after wiring the unfold into the beat");
    println!("   {:<26} {:>7} {:>9} {:>5} {:>5}", "world", "grades", "unfolded", "D", "V");
    let (mut grades, mut opened) = (0usize, 0u64);
    for p in &worlds {
        let committed = p.world.committed();
        let read = read_entangled(&committed);
        let g = p.world.key().len();
        grades += g;
        opened += p.unfolded;
        println!(
            "   {:<26} {:>7} {:>9} {:>5} {:>5}",
            &p.spec.world_name[..p.spec.world_name.len().min(26)],
            g,
            p.unfolded,
            read.degree().get(),
            read.flows.len()
        );
    }
    println!("   {:<26} {:>7} {:>9}", "— totals —", grades, opened);
    match opened > 0 {
        true => println!(
            "\n   THE BOARD UNFOLDS. {opened} planes opened across {} lines, where\n   \
             before this the count was zero for every era ever run.",
            worlds.len()
        ),
        false => println!(
            "\n   STILL FLAT — no line opened a plane. Either no mint was earned in\n   \
             {beats} beats, or the wiring does not reach."
        ),
    }
}

fn main() {
    println!("THE UNFOLD — the fiber opens; does D follow?\n");
    println!("   obligations held FIXED (one non-composing triad on 3 orbs)");
    println!("   varied: the number of OPEN GRADES in the gauge key\n");

    println!("   {:>7} {:>26} {:>5} {:>5}", "grades", "gauge key", "D", "V");
    for grades in 1..=8usize {
        let mut state = triad();
        // The unfold: a key of `grades` open planes. Genesis is [0];
        // each further grade is one earned dimension.
        state.gauge = (0..grades).map(|k| ratio(k as i64, 12)).collect();
        let read = read_entangled(&state);
        let rendered: Vec<String> = state.gauge.iter().map(|g| format!("{g}")).collect();
        let shown = rendered.join(",");
        println!(
            "   {:>7} {:>26} {:>5} {:>5}",
            grades,
            &shown[..shown.len().min(26)],
            read.degree().get(),
            read.flows.len()
        );
    }

    // The control, in the other direction: hold the fiber at ONE grade
    // and add a relation instead. If D moves here and not above, the
    // meter reads obligations and not dimensions.
    println!("\n   CONTROL — fiber held at 1 grade, obligations varied:");
    println!("   {:>10} {:>5} {:>5}", "bonds", "D", "V");
    for extra in 0..=3usize {
        let mut state = triad();
        state.gauge = vec![ratio(0, 1)];
        for k in 0..extra {
            state.bonds.push(Bond {
                source: 1,
                target: 2 + k as u64 % 2,
                resource: Resource::Biomass,
                rate: ratio(3 + k as i64, 1),
                term: 0,
                sealed: 0,
                anchored: 0,
            });
        }
        let read = read_entangled(&state);
        println!(
            "   {:>10} {:>5} {:>5}",
            state.bonds.len(),
            read.degree().get(),
            read.flows.len()
        );
    }

    // ── the reading ──────────────────────────────────────────────────
    let mut one = triad();
    one.gauge = vec![ratio(0, 1)];
    let mut many = triad();
    many.gauge = (0..8).map(|k| ratio(k, 12)).collect();
    let d_one = read_entangled(&one).degree().get();
    let d_many = read_entangled(&many).degree().get();

    println!("\nREADING");
    println!("   1 open grade  → D {d_one}");
    println!("   8 open grades → D {d_many}");
    live_board(30);

    match d_many > d_one {
        true => println!(
            "\n   D FOLLOWS THE UNFOLD. Opening planes raises the extent, and the\n   \
             meter is measuring the engine's dimensional growth."
        ),
        false => println!(
            "\n   D DOES NOT FOLLOW THE UNFOLD. The fiber went from 1 open grade to\n   \
             8 — eight earned dimensions, one receipted mint apiece — and the\n   \
             extent did not move.\n\n   \
             The bridge builds Δ from PAIRWISE OBLIGATIONS ONLY: bonds,\n   \
             forwards, debts, lanes. It never reads `Committed::gauge`. So D\n   \
             measures the obligation graph and not the polynomial fiber, and\n   \
             the engine's own expansion mechanism is invisible to the reading\n   \
             named for extent."
        ),
    }
}
