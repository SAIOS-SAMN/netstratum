//! ENCAPSULATE THE DIVERGENT POLES IN TOROIDS, THEN MEASURE AGAIN.
//!
//! Operator ruling 2026-08-04. The claim being tested is about the
//! HODGE SPLIT, so it is measured rather than argued.
//!
//! `Δ = grad(p) ⊕ circulation`. `capacity` counts only the circulation
//! — frame contributes nothing to `D` — and `circulation_on` obtains it
//! by solving the SUPPORT graph's Laplacian, `L·p = −div`. That solve
//! treats the support as a plain graph. The universe it describes is
//! not one: `TUniversal{S, G, T}` is a compact 3-TORUS, every direction
//! comes back around, and `ledger::l1_distance` already measures the
//! short way ROUND rather than across.
//!
//! So a vertex reading `div ≠ 0` — an EGRESS or an INGRESS, a divergent
//! pole — may be an artefact of measuring an open chain in a closed
//! space. What leaves may return, and if it does, it was never
//! gradient: it was circulation the split could not see because the
//! path it closes through was not in the support.
//!
//! THE MEASUREMENT. The same relation, twice:
//!
//!   OPEN          the chain as the support carries it today
//!   ENCAPSULATED  the same chain with its poles CLOSED through the
//!                 torus — the return leg that makes the walk a loop
//!
//! If `D` rises on encapsulation, then capacity currently discarded as
//! frame is structure, and the discard is an artefact of an open
//! support rather than a property of the state.
//!
//! Run: cargo run -p rfa-core --example toroid_encapsulation

use cfe::ratio::{ratio, Ratio};
use rfa_core::capacity::{capacity, circulation_on, flux};
use rfa_core::ext_x;
use rfa_core::{boundary_form, holonomies, Delta, Support};

fn edge(d: &mut Delta, s: &mut Support, i: usize, j: usize, v: Ratio) {
    d.set_antisym(i, j, vec![v]);
    s.connect(i, j, 0);
}

fn report(label: &str, d: &Delta, s: &Support) {
    let census = flux(d);
    let form = boundary_form(d, s);
    let rank = holonomies(d, s).len();
    // THE SUPPORT-AWARE READING. `capacity()` splits over the COMPLETE
    // graph — it treats every absent relation as present-carrying-zero,
    // so it reports the same D for an open chain and a closed loop and
    // cannot see this experiment at all (measured: D = 4 for all three
    // shapes). `circulation_on` is the reading that respects which
    // relations EXIST, which is the only one that can answer whether
    // closing a pole changed anything.
    let extent = ext_x(&circulation_on(d, s));
    let complete = capacity(d);
    println!("  {label}");
    println!(
        "     poles: {} source(s), {} sink(s), {} balanced",
        census.sources, census.sinks, census.balanced
    );
    println!("     ∂Δ carries {} flow(s)", form.rank());
    println!(
        "     rank {rank}   D(support) = {}   D(complete graph) = {}",
        extent.degree(),
        complete.degree()
    );
}

fn main() {
    println!("ENCAPSULATING DIVERGENT POLES — the same relation, twice\n");

    // ── the open chain ────────────────────────────────────────────────
    // 0 → 1 → 2. Vertex 0 emits and vertex 2 absorbs: two divergent
    // poles, and a support that is a PATH, so nothing closes.
    let mut open = Delta::zero(3, 1);
    let mut open_s = Support::new(3, 1);
    edge(&mut open, &mut open_s, 0, 1, ratio(2, 1));
    edge(&mut open, &mut open_s, 1, 2, ratio(2, 1));
    report("OPEN — the chain, measured as a graph", &open, &open_s);

    // ── encapsulated ─────────────────────────────────────────────────
    // The SAME two relations, plus the return leg that closes the walk
    // through the torus: 2 → 0 carrying what left. Every vertex now
    // balances, and the walk is a loop rather than a line.
    let mut closed = Delta::zero(3, 1);
    let mut closed_s = Support::new(3, 1);
    edge(&mut closed, &mut closed_s, 0, 1, ratio(2, 1));
    edge(&mut closed, &mut closed_s, 1, 2, ratio(2, 1));
    edge(&mut closed, &mut closed_s, 2, 0, ratio(2, 1));
    println!();
    report("ENCAPSULATED — the poles closed through the torus", &closed, &closed_s);

    // ── the reading ──────────────────────────────────────────────────
    let open_rank = holonomies(&open, &open_s).len();
    let closed_rank = holonomies(&closed, &closed_s).len();
    let open_d = ext_x(&circulation_on(&open, &open_s)).degree();
    let closed_d = ext_x(&circulation_on(&closed, &closed_s)).degree();
    println!("\nΔD on encapsulation: {open_d} → {closed_d}   (rank {open_rank} → {closed_rank})");

    match closed_d > open_d {
        true => {
            println!("\nCAPACITY ROSE. What the open support read as GRADIENT — pure");
            println!("frame, contributing nothing to D — is CIRCULATION once the poles");
            println!("close. The discard was an artefact of the support being open, not");
            println!("a property of the state.");
        }
        false => {
            println!("\nCAPACITY DID NOT RISE. Closing the poles did not convert frame");
            println!("into structure here, so the split is not hiding capacity in this");
            println!("shape — the divergence was real, not an artefact of an open walk.");
        }
    }

    // ── and the honest control ───────────────────────────────────────
    // A closing leg that does NOT balance the poles. If D rises here
    // too, then the rise measures the EDGE added and not the closure,
    // and the whole reading above would be an artefact of the probe.
    let mut skew = Delta::zero(3, 1);
    let mut skew_s = Support::new(3, 1);
    edge(&mut skew, &mut skew_s, 0, 1, ratio(2, 1));
    edge(&mut skew, &mut skew_s, 1, 2, ratio(2, 1));
    edge(&mut skew, &mut skew_s, 2, 0, ratio(7, 1));
    println!();
    report("CONTROL — closed, but the return does not balance", &skew, &skew_s);
    println!(
        "\n  control D = {}   (if this equals the encapsulated D, the rise is the",
        ext_x(&circulation_on(&skew, &skew_s)).degree()
    );
    println!("  EDGE and not the closure, and the reading above proves nothing)");
}
