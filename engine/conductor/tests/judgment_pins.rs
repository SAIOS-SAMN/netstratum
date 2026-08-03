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

    // THE MEASUREMENT, RECORDED (2026-08-03, first run of this pin):
    // eight beats of FOUNDING play drew TWO refusals, both
    // `MintWithoutStanding { verdict: Void }` — the VERSION 3 capacity
    // law. The proposer mints discovery on a world the judge reads as
    // Void, and `world.rs::replay` banks it regardless, because replay
    // is still the live transition. This assertion exists to keep that
    // fact from going quiet: when the proposer learns to ask the
    // capacity law before it mints, this line goes red and whoever
    // lands the fix TRUTH-UPDATES it — it is not deleted, it is
    // rewritten to say the board now proposes only what the judge
    // admits.
    assert!(
        !polity.refusals().is_empty(),
        "founding play still proposes deeds the judge refuses — if this is \
         green, the proposer was fixed: truth-update this pin, do not delete it"
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
