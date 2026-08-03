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

    // TRUTH-UPDATED TWICE, and the history is the point.
    //
    // 2026-08-03, first measured: eight beats of founding play drew TWO
    // refusals, both `MintWithoutStanding { verdict: Void }` — the
    // proposer minted into worlds the judge barred and `replay` banked
    // them anyway.
    //
    // 2026-08-04, the proposer learned to ask first: refusals fell to
    // zero and the old assertion went red, as written.
    //
    // 2026-08-04, THE CAPACITY LAW WAS STRUCK (operator ruling —
    // identity is pegged to entity and is recycled without bonds). The
    // count stays zero, but for a different reason: there is no longer a
    // standing test to fail. What this pin still guards is the thing
    // that outlived the law — the judge's verdicts REACH somebody.
    assert!(
        polity.refusals().is_empty(),
        "the board proposes only what the judge admits — a refusal here means \
         the proposer and the judge have drifted apart: {:?}",
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
