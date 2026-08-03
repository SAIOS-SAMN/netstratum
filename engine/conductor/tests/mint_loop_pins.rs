//! A1 — THE PoUW LOOP, END TO END, AND WHERE THE LAW STOPS IT.
//!
//! Before A1, an NPC that solved a task wrote a file and incremented a
//! counter: cognition proved out and the economy never heard of it.
//! A1 closed that loop and this file measured it on a real era.
//!
//! **TRUTH-UPDATED 2026-08-04, and the update is the finding.**
//! `a_solving_line_mints_against_its_own_verified_work` asserted that a
//! mint reaches a chronicle within 60 beats. It passed only because the
//! proposer never asked the capacity law and `replay` banked mints the
//! judge refused. With the proposer asking first (kernel VERSION 3,
//! `capacity_law`), NOTHING on the founding board mints, and the old
//! assertion went red.
//!
//! That is a COLLISION BETWEEN TWO LANDINGS OF THE SAME DAY, not a
//! regression:
//!
//! - **A1 (2026-08-01)** — verified work MINTS; the loop is closed.
//! - **VERSION 3 (2026-08-01)** — a mint requires `Vacuity::Entangled`:
//!   structure AND flow, both standing.
//!
//! Measured (`cargo run -p conductor --example vacuity_probe`): a world
//! reaches `Entangled` only with a THREE-orb obligation triangle that
//! does NOT compose. One bond reads `Unframed`. A triangle that composes
//! (1 + 2 = 3) reads `Unframed`. A founding NPC world — one orb, no
//! pairwise obligations at all — reads `Void`, and the conductor's play
//! seals no bonds or covenants between orbs, so no line it runs can ever
//! earn standing.
//!
//! So the PoUW economy is not open and not closed: it is BARRED, and
//! every solve is proved, certified, and unpaid. Which of the two
//! landings gives way is an operator ruling, and the tests below hold
//! the true state until it comes.

use std::path::PathBuf;

use cfe::dop::ClosureEvent;
use conductor::run_era;

fn scratch(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("netstratum-mint-{name}"));
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).expect("scratch");
    path
}

/// THE LOOP RUNS TO THE LAW'S EDGE AND STOPS THERE, VISIBLY.
///
/// Supersedes `a_solving_line_mints_against_its_own_verified_work`. The
/// work half of A1 still stands — lines solve and certify. The payment
/// half is withheld by the capacity law, and the point of this pin is
/// that the withholding is NAMED rather than silent: an unpaid solve
/// and a line that never solved look identical on a board that does not
/// say which it is.
#[test]
fn the_pouw_loop_runs_to_the_capacity_law_and_stops_there() {
    let root = scratch("loop");
    let var = root.to_string_lossy().into_owned();
    let worlds = run_era(&var, 60);

    let mut minted = 0usize;
    let mut carried_proof = 0usize;
    let mut earned_total = cfe::ratio::ratio(0, 1);
    for polity in &worlds {
        for receipt in polity.world.chronicle().receipts() {
            if let ClosureEvent::DiscoveryMinted { amount, work, .. } = &receipt.event {
                minted += 1;
                earned_total += amount.clone();
                if !work.is_empty() {
                    carried_proof += 1;
                }
            }
        }
    }
    let withheld: usize = worlds.iter().map(|p| p.withheld_mints().len()).sum();
    let solved: u32 = worlds.iter().map(|p| p.solved).sum();
    println!(
        "MEASURED over 60 beats: {solved} solves, {minted} mints, {withheld} withheld"
    );

    // THE WORK HALF OF A1 STILL STANDS.
    assert!(
        solved > 0,
        "no line solved in 60 beats — that is a cognition failure, not a law one"
    );

    // THE PAYMENT HALF IS BARRED, AND SAYS SO. Every mint the ladder
    // would have paid is on the withheld record with its verdict.
    assert!(
        withheld > 0,
        "the board neither minted nor withheld: work that earns nothing and \
         says nothing is the silent hole this pin exists to prevent"
    );

    // WHEN standing becomes reachable this flips, and the A1 invariant
    // must survive the flip: a mint carries its proof WHOLE or it is not
    // a mint. Held here so the guarantee is not lost while the economy
    // is barred.
    assert_eq!(
        carried_proof, minted,
        "every mint carries its proof whole — the receipt is what re-verifies forever"
    );
    assert!(
        earned_total >= cfe::ratio::ratio(0, 1),
        "the ladder never pays negative"
    );
}

/// The Emission Law's floor. **DELIBERATELY NOT VACUOUS:** this walks
/// the mints and asserts none carries zero, and with the economy barred
/// there are currently NO mints to walk — a check over an empty set
/// passes without deciding anything. So it states its own coverage: the
/// count is printed and the emptiness is asserted to match the withheld
/// record, which is what makes "zero mints" a measured fact rather than
/// a silence.
#[test]
fn the_saturated_rungs_pay_nothing_even_when_solved() {
    let root = scratch("saturated");
    let var = root.to_string_lossy().into_owned();
    let worlds = run_era(&var, 60);

    let mut examined = 0usize;
    for polity in &worlds {
        for receipt in polity.world.chronicle().receipts() {
            if let ClosureEvent::DiscoveryMinted { amount, .. } = &receipt.event {
                examined += 1;
                assert!(
                    *amount > cfe::ratio::ratio(0, 1),
                    "a zero-amount mint reached the chronicle — the saturated rungs \
                     must emit nothing at all, not a deed worth nothing"
                );
            }
        }
    }
    let withheld: usize = worlds.iter().map(|p| p.withheld_mints().len()).sum();
    println!("MEASURED: {examined} mints examined, {withheld} withheld by the capacity law");

    // The coverage statement. If mints exist, this pin decided something
    // about them; if none do, the reason must be the LAW withholding
    // them, never the board falling silent.
    assert!(
        examined > 0 || withheld > 0,
        "no mint was examined and none was withheld — this check decided nothing \
         and the board produced no work at all; that is a measurement failure, \
         not a passing test"
    );
}
