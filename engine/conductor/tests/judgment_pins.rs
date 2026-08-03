//! DIRECTIVE 19 — THE JUDGE IS HEARD.
//!
//! `chain_beat` runs `kernel::lawful` on every deed past the adoption
//! door. Before this pin the verdicts landed in a private `Vec` no
//! caller could reach — the judge speaking into a void, which is the
//! one thing the field's own comment said it existed to prevent.
//!
//! RED, recorded: with `refusals()` and `unreported_refusals()` absent,
//! this file does not compile — `error[E0616]: field `refused` of
//! struct `Polity` is private`. Unreachable is a stronger red than
//! failing.

use conductor::{founding_polities, Polity};
use ledger::registry::Registry;
use ledger::Ledger;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "conductor-judgment-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

#[test]
fn the_verdicts_are_readable_and_each_is_announced_once() {
    let root = scratch("heard");
    let var = root.to_string_lossy().into_owned();
    let registry = Registry::at(format!("{var}/registry"));
    let book = Ledger::at(format!("{var}/worlds"));

    let spec = founding_polities().into_iter().next().expect("a founding spec");
    let mut polity = Polity::genesis(spec, &registry, &book);
    for _ in 0..8 {
        polity.cycle(&book, &registry);
    }

    // MEASURED, not assumed: what the founding board's own play draws
    // out of the judge.
    let refusals = polity.refusals().len();
    println!("MEASURED: {refusals} refusals over 8 beats of founding play");
    for (sequence, unlawful) in polity.refusals() {
        println!("  deed {sequence}: {unlawful:?}");
    }

    // TRUTH-UPDATED 2026-08-04, exactly as the previous text instructed.
    //
    // As first measured (2026-08-03): eight beats of FOUNDING play drew
    // TWO refusals, both `MintWithoutStanding { verdict: Void }`. The
    // proposer minted into a world the judge reads as Void and `replay`
    // banked it, because replay is still the live transition.
    //
    // The proposer now asks the capacity law FIRST — the same two
    // functions `kernel::capacity_law` calls — so it no longer proposes
    // what the judge would refuse, and this count is ZERO. The old
    // assertion (`!refusals().is_empty()`) went red on the very run
    // that fixed it, which is what it was written to do.
    assert!(
        polity.refusals().is_empty(),
        "the board proposes only what the judge admits — a refusal here means \
         the proposer and the judge have drifted apart again: {:?}",
        polity.refusals()
    );

    // The announcement law: every verdict on the record is unreported
    // exactly once, then never again.
    let announced = polity.unreported_refusals().len();
    assert_eq!(
        announced, refusals,
        "the first read announces the whole standing record"
    );
    assert!(
        polity.unreported_refusals().is_empty(),
        "a verdict is announced ONCE — the cursor does not rewind"
    );

    // The record itself is permanent: announcing prunes nothing.
    assert_eq!(
        polity.refusals().len(),
        refusals,
        "the cursor moved, the record did not"
    );
}

/// THE WORK IS STILL DONE — it is simply not PAID.
///
/// The capacity law does not stop a line from solving; it stops the
/// solve from minting. That distinction has to be visible, because a
/// silently unpaid solve is indistinguishable from a line that never
/// solved anything — and the first version of this fix would have made
/// the whole PoUW economy look idle rather than barred.
#[test]
fn a_withheld_mint_is_recorded_not_swallowed() {
    let root = scratch("withheld");
    let var = root.to_string_lossy().into_owned();
    let registry = Registry::at(format!("{var}/registry"));
    let book = Ledger::at(format!("{var}/worlds"));

    let spec = founding_polities().into_iter().next().expect("a founding spec");
    let mut polity = Polity::genesis(spec, &registry, &book);
    for _ in 0..8 {
        polity.cycle(&book, &registry);
    }

    let withheld = polity.withheld_mints().len();
    println!("MEASURED: {withheld} withheld mints over 8 beats");
    for (stamp, verdict) in polity.withheld_mints() {
        println!("  t{stamp}: {verdict}");
    }

    // The same two deeds that used to be refused BY THE JUDGE are now
    // withheld BY THE PROPOSER, on the same reading.
    assert!(
        !polity.withheld_mints().is_empty(),
        "founding play proves work it cannot yet be paid for — if this is \
         empty, either the line stopped solving or standing became reachable \
         on the founding board; both are news, so measure before editing"
    );
    assert!(
        polity.withheld_mints().iter().all(|(_, why)| why == "Void"),
        "a founding world holds no pairwise obligation at all, so the reading \
         is Void — not Stasis, not Unframed: {:?}",
        polity.withheld_mints()
    );

    // The solve itself still landed: work proved, payment withheld.
    assert!(
        polity.solved > 0,
        "the line still solves — the law bars the mint, never the work"
    );

    // Announce-once, same law as the refusals.
    assert_eq!(polity.unreported_withheld().len(), withheld);
    assert!(polity.unreported_withheld().is_empty());
    assert_eq!(polity.withheld_mints().len(), withheld, "the record stands");
}
