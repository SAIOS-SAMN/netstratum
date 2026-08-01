//! A1 — THE PoUW LOOP, END TO END.
//!
//! Before this, an NPC that solved a task wrote a file and incremented
//! a counter: cognition proved out and the economy never heard of it.
//! This pin drives a real era and measures whether a `DiscoveryMinted`
//! receipt reaches a real chronicle.

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

#[test]
fn a_solving_line_mints_against_its_own_verified_work() {
    // Run the board long enough for a line to climb off the saturated
    // rungs. `f(0..=2) = 0` by the Emission Law — proof-of-position
    // pays nothing — so nothing should mint until a world reaches rung
    // 3, and then it earns exactly the ruled ladder.
    let root = scratch("loop");
    let var = root.to_string_lossy().into_owned();
    let worlds = run_era(&var, 60);

    let mut minted = 0usize;
    let mut earned_total = cfe::ratio::ratio(0, 1);
    let mut carried_proof = 0usize;

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

    assert!(
        minted > 0,
        "no line minted in 60 beats — the PoUW loop is still open: NPCs solve and the \
         economy never hears of it"
    );
    assert_eq!(
        carried_proof, minted,
        "every mint carries its proof WHOLE — the receipt is what re-verifies forever"
    );
    assert!(
        earned_total > cfe::ratio::ratio(0, 1),
        "the ladder paid: cognition became value"
    );
}

#[test]
fn the_saturated_rungs_pay_nothing_even_when_solved() {
    // The Emission Law's floor, measured on the real board rather than
    // asserted: a mint may never appear carrying zero. `mint_bound`
    // returns 0 on rungs 0-2 and the conductor does not emit at all
    // there, so free issuance cannot enter by a degenerate deed.
    let root = scratch("saturated");
    let var = root.to_string_lossy().into_owned();
    let worlds = run_era(&var, 60);

    for polity in &worlds {
        for receipt in polity.world.chronicle().receipts() {
            if let ClosureEvent::DiscoveryMinted { amount, .. } = &receipt.event {
                assert!(
                    *amount > cfe::ratio::ratio(0, 1),
                    "a zero-amount mint reached the chronicle — the saturated rungs must \
                     emit nothing at all, not a deed worth nothing"
                );
            }
        }
    }
}
