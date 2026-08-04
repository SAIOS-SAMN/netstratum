//! CAN `D` CLIMB TO n THROUGH ANY COMBINATION, WITHOUT REACHING INFINITY?
//!
//! Operator requirement 2026-08-04. Three claims, each decidable:
//!
//!   1. UNBOUNDED ABOVE — `D` reaches any n given enough structure.
//!   2. NEVER INFINITE — every reading is a finite, exact count. The
//!      T∞ purge already barred the other pole; this asks whether the
//!      capacity meter honours it by construction.
//!   3. ANY VARIATION OF COMBINATIONS — the climb is not down one
//!      privileged axis. Different ARRANGEMENTS of the same amount of
//!      structure reach different capacity.
//!
//! Measured on the algebra directly (`ext_x ∘ circulation_on`, the
//! support-aware reading — `capacity()` splits over the complete graph
//! and cannot see which relations exist).
//!
//! Run: cargo run -p rfa-core --example capacity_climb

use cfe::ratio::{ratio, Ratio};
use rfa_core::capacity::circulation_on;
use rfa_core::{ext_x, holonomies, Delta, Support};

fn edge(d: &mut Delta, s: &mut Support, i: usize, j: usize, l: usize, v: Ratio) {
    let fiber = d.fiber();
    let mut coords = vec![ratio(0, 1); fiber];
    coords[l] = v;
    d.set_antisym(i, j, coords);
    s.connect(i, j, l);
}

fn degree(d: &Delta, s: &Support) -> usize {
    ext_x(&circulation_on(d, s)).degree().get()
}

/// A complete relation on `n` entities over `f` channels, valued so no
/// triangle composes — every cycle carries charge no frame explains.
fn saturated(n: usize, f: usize) -> (Delta, Support) {
    let mut d = Delta::zero(n, f);
    let mut s = Support::new(n, f);
    let mut seed = 1i64;
    for i in 0..n {
        for j in (i + 1)..n {
            let mut coords = vec![ratio(0, 1); f];
            for (l, slot) in coords.iter_mut().enumerate() {
                seed += 1;
                *slot = ratio(seed * (l as i64 + 1) * (l as i64 + 1), 1);
                s.connect(i, j, l);
            }
            d.set_antisym(i, j, coords);
        }
    }
    (d, s)
}

fn main() {
    println!("CAN D CLIMB TO n, THROUGH ANY COMBINATION, WITHOUT INFINITY?\n");

    // ── 1. unbounded above, along the ENTITY axis ────────────────────
    println!("1. THE ENTITY AXIS — one channel, more entities");
    println!("   {:>8} {:>10} {:>8} {:>14}", "entities", "relations", "D", "ceiling 1+C(n,2)");
    for n in 3..=9usize {
        let (d, s) = saturated(n, 1);
        let ceiling = 1 + n * (n - 1) / 2;
        println!("   {:>8} {:>10} {:>8} {:>14}", n, n * (n - 1) / 2, degree(&d, &s), ceiling);
    }

    // ── 2. the CHANNEL axis ──────────────────────────────────────────
    println!("\n2. THE CHANNEL AXIS — four entities, more channels");
    println!("   {:>8} {:>8} {:>18}", "channels", "D", "ceiling 1+C(4,2)*f");
    for f in 1..=5usize {
        let (d, s) = saturated(4, f);
        println!("   {:>8} {:>8} {:>18}", f, degree(&d, &s), 1 + 6 * f);
    }

    // ── 3. ANY VARIATION OF COMBINATIONS ─────────────────────────────
    // The same COUNT of relations, arranged differently. If D varied
    // only with the amount of structure, these would be equal.
    println!("\n3. THE SAME FOUR RELATIONS, ARRANGED THREE WAYS (4 entities, 1 channel)");
    let shapes: [(&str, &[(usize, usize)]); 3] = [
        ("star + chord  (0-1,0-2,0-3,1-2)", &[(0, 1), (0, 2), (0, 3), (1, 2)]),
        ("4-cycle       (0-1,1-2,2-3,3-0)", &[(0, 1), (1, 2), (2, 3), (3, 0)]),
        ("triangle+tail (0-1,1-2,2-0,2-3)", &[(0, 1), (1, 2), (2, 0), (2, 3)]),
    ];
    println!("   {:<34} {:>6} {:>6}", "arrangement", "rank", "D");
    for (label, edges) in shapes {
        let mut d = Delta::zero(4, 1);
        let mut s = Support::new(4, 1);
        for (k, (i, j)) in edges.iter().enumerate() {
            edge(&mut d, &mut s, *i, *j, 0, ratio(2 + k as i64 * 3, 1));
        }
        println!(
            "   {:<34} {:>6} {:>6}",
            label,
            holonomies(&d, &s).len(),
            degree(&d, &s)
        );
    }

    // ── 4. never infinite ────────────────────────────────────────────
    println!("\n4. NEVER INFINITE");
    let (d, s) = saturated(9, 3);
    let reached = degree(&d, &s);
    println!("   9 entities x 3 channels reaches D = {reached}");
    println!("   the reading is a COUNT of surviving coordinates — bounded by");
    println!("   C(n,2)*f, which is finite for any finite state. There is no");
    println!("   arithmetic path to an unbounded value: `Degree` wraps a usize");
    println!("   whose only constructor adds circulation to GROUND, and the");
    println!("   circulation is a count of entries that exist.");

    println!("\nVERDICT");
    let (a, sa) = saturated(3, 1);
    let (b, sb) = saturated(9, 1);
    let climbs_entities = degree(&b, &sb) > degree(&a, &sa);
    let (c, sc) = saturated(4, 1);
    let (e, se) = saturated(4, 5);
    let climbs_channels = degree(&e, &se) > degree(&c, &sc);
    println!("   climbs on entities : {climbs_entities}");
    println!("   climbs on channels : {climbs_channels}");
    println!("   finite always      : true by construction (a count, not a limit)");
}
