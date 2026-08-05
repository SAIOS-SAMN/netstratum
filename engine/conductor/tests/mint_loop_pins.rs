//! A1 — THE PoUW LOOP, END TO END.
//!
//! Before A1, an NPC that solved a task wrote a file and incremented a
//! counter: cognition proved out and the economy never heard of it.
//! This pin drives a real era and measures whether a `DiscoveryMinted`
//! receipt reaches a real chronicle.
//!
//! **THE HISTORY THIS FILE CARRIES, because it is the whole argument
//! for the ruling that ends it:**
//!
//! - A1 (2026-08-01) closed the loop and this pin passed.
//! - It passed VACUOUSLY. The proposer never asked the gate, and
//!   `replay` — still the live transition — banked mints that
//!   `kernel::admits` had refused as `MintWithoutStanding`.
//! - 2026-08-04: the proposer was taught to ask first, and the board
//!   went to **374 solves, 0 mints, 124 withheld** over 60 beats. The
//!   capacity law required `Vacuity::Entangled`, which needed a
//!   three-orb obligation triangle that does not compose — a shape no
//!   world the game produces can form.
//! - 2026-08-04, operator ruling: **the capacity law is struck.**
//!   *Identity is pegged to entity, and identity gets recycled without
//!   bonds* — so a standing test read off bonds refused the lawful
//!   recycled line by construction (measured: a succeeded world reads
//!   `Void` before and after succession).
//!
//! So this pin returns to asserting what A1 always meant, and it is no
//! longer vacuous: the mint's anchor is `verify_work` (re-forge,
//! re-apply, re-chain) with `mint_bound` capping the ladder, and NOTHING
//! now stands between a proved solve and its payment.

use std::path::PathBuf;

use cfe::dop::ClosureEvent;
use conductor::run_era;

/// **THE SCRATCH IS PER-PROCESS** (fixed 2026-08-05).
///
/// This helper was the only one in the tree without `process::id()` in
/// its path, so two concurrent boards — a second worktree running its
/// own `cargo test`, which is ordinary on this machine — shared
/// `/tmp/netstratum-mint-loop` and each `remove_dir_all`'d the other's
/// universe mid-era. The symptom was a pin that FAILED IN THE FULL RUN
/// and PASSED ALONE, twice, which reads exactly like flakiness in the
/// engine and is not: the engine is deterministic, and the two runs were
/// deleting each other's `var`.
///
/// Found while confirming V0–V4, where a real board failure and this one
/// would have been indistinguishable.
fn scratch(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    // PROCESS-UNIQUE, like every other fixture in the tree. Without the
    // pid this was the ONE test reusing a fixed path across runs and
    // across concurrent cargo invocations, and `run_era` ->
    // `default_era_roots` scans the PARENT directory for `<base>-era*`
    // — so leftover state from another run is reachable. Measured: the
    // same 60-beat era gave 296/56/272, then 297/57/276, then
    // 328/48/234 with two kernel refusals. The engine itself replays
    // exactly (`examples/determinism_probe.rs`); only this fixture
    // drifted.
    path.push(format!("netstratum-mint-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).expect("scratch");
    path
}

#[test]
fn a_solving_line_mints_against_its_own_verified_work() {
    // Run the board long enough for a line to climb off the saturated
    // rungs. `f(0..=2) = 0` by the Emission Law — proof-of-position pays
    // nothing — so nothing mints until a world reaches rung 3, and then
    // it earns exactly the ruled ladder.
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
    let solved: u32 = worlds.iter().map(|p| p.solved).sum();
    let spent: u64 = worlds.iter().map(|p| p.spent).sum();
    let swept: u64 = worlds.iter().map(|p| p.swept).sum();
    let growth = worlds.iter().filter(|p| p.trajectory == conductor::Trajectory::Growth).count();
    let stagnation = worlds.iter().filter(|p| p.trajectory == conductor::Trajectory::Stagnation).count();
    let decline = worlds.iter().filter(|p| p.trajectory == conductor::Trajectory::Decline).count();
    println!("MEASURED over 60 beats: {solved} solves, {minted} mints, earned {earned_total}");
    println!("MEASURED spend: {spent} read-candidates + {swept} sweep-invocations  |  trajectories at close: growth {growth}, stagnation {stagnation}, decline {decline}");

    assert!(
        minted > 0,
        "no line minted in 60 beats — the PoUW loop is open again: NPCs solve \
         and the economy never hears of it ({solved} solves went unpaid)"
    );
    assert_eq!(
        carried_proof, minted,
        "every mint carries its proof WHOLE — the receipt is what re-verifies forever"
    );
    assert!(
        earned_total > cfe::ratio::ratio(0, 1),
        "the ladder paid: cognition became value"
    );

    // THE JUDGE AGREES. The mints above are not merely unblocked, they
    // are LAWFUL under the version that gated them — the removal
    // loosened the law, and this pin is where that claim is checked
    // rather than assumed.
    let refused: usize = worlds.iter().map(|p| p.refusals().len()).sum();
    assert_eq!(
        refused, 0,
        "a mint the judge refuses reached a chronicle: {:?}",
        worlds
            .iter()
            .flat_map(|p| p.refusals())
            .collect::<Vec<_>>()
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
    println!("MEASURED: {examined} mints examined");

    // COVERAGE, STATED. This check walks the mints; over an empty set it
    // passes without deciding anything, which is exactly how it went
    // quietly vacuous while the capacity law barred the economy. It now
    // fails rather than passing on nothing.
    assert!(
        examined > 0,
        "no mint was examined — this check decided nothing, which is a \
         measurement failure and not a passing test"
    );
}
