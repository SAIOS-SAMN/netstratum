//! AEON-1b — THE FOLD (11 §5). The clean start is not deletion: the
//! live era SEALS (`EraFolded` onto the universe ledger, the new base
//! gauge advancing the sheet) and ROTATES whole into the archive
//! sequence; the next boot CONVERGES the sealed era — the four C's
//! read which trajectories continue. Nothing is destroyed.

use cfe::dop::ClosureEvent;
use cfe::ratio::ratio;
use conductor::{fold, run_era};
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aeon-fold-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    // The archive rotates BESIDE the var root — give it a parent of
    // its own so stray counterparts never leak into era discovery.
    let _ = fs::remove_dir_all(std::env::temp_dir().join(format!(
        "aeon-fold-{name}-{}-holder",
        std::process::id()
    )));
    let holder = std::env::temp_dir().join(format!(
        "aeon-fold-{name}-{}-holder",
        std::process::id()
    ));
    fs::create_dir_all(&holder).expect("holder");
    holder.join("var")
}

/// A short era folds: the archive gains `var-era1` with the worlds and
/// registry whole, the ledger carries `EraFolded { era: 1 }` with the
/// advanced base gauge (1/12, wrapped), the live root stands empty of
/// worlds — and the NEXT era converges the sealed one (the fold is the
/// annexation law run forward; nothing destroyed).
#[test]
fn the_fold_seals_rotates_and_the_next_aeon_converges_it() {
    let var = scratch("whole");
    let var_str = var.to_string_lossy().into_owned();
    let worlds = run_era(&var_str, 3);
    let count = worlds.len();
    assert!(count > 0, "an era beat");
    drop(worlds);

    let (era, gauge) = fold(&var_str).expect("the aeon folds");
    assert_eq!(era, 1);
    assert_eq!(gauge, ratio(1, 12), "the new sheet seats at the advanced base");

    let archive = var.parent().expect("holder").join("var-era1");
    assert!(archive.join("worlds").is_dir(), "the worlds rotated whole");
    assert!(archive.join("registry").is_dir(), "the registry rotated whole");
    assert!(!var.join("worlds").exists(), "the live root stands folded");

    // The seal is a receipt on the universe ledger, moved with the era.
    let ledger = cfe::persist::load(&archive.join("worlds/convergence.tlv"))
        .expect("the sealed ledger reads");
    let folded = ledger.receipts().iter().any(|receipt| {
        matches!(
            &receipt.event,
            ClosureEvent::EraFolded { era: 1, gauge } if *gauge == ratio(1, 12)
        )
    });
    assert!(folded, "EraFolded receipted to the universe ledger");

    // The next aeon boots and CONVERGES the sealed era: the archived
    // worlds annex onto the new chart — some things continue.
    let book = ledger::Ledger::at(var.join("worlds"));
    let eras = ledger::converge::default_era_roots(&var_str);
    assert!(
        eras.iter().any(|(name, _)| name == "var-era1"),
        "the archive sequence sees the folded era"
    );
    let converged = ledger::converge::converge(&book, &eras).expect("convergence");
    assert_eq!(
        converged.adopted.len(),
        count,
        "every folded world annexes into the new aeon"
    );

    // Convergence rebuilt the live universe (the annexed charts ARE
    // the new aeon's board) — so the NEXT fold is lawful: era 2, the
    // sheet advancing again. The aeon sequence is a walk, not a
    // one-shot.
    // LIVE the second aeon before folding it (an empty aeon folds
    // charts alone; the sheet test needs real worlds on sheet 1/12).
    let second_aeon = run_era(&var_str, 3);
    assert!(!second_aeon.is_empty(), "the second aeon beats");
    drop(second_aeon);
    let (second, next_gauge) = fold(&var_str).expect("the second aeon folds");
    assert_eq!(second, 2);
    assert_eq!(next_gauge, ratio(1, 6), "2/12 wrapped — the sheets stack");
    assert!(var.parent().expect("holder").join("var-era2").is_dir());

    // THE SHEET SEATING (11 §5.2, the second-look audit): converge
    // the two folded aeons onto a third boot and read the chart — the
    // same world name from era 1 and era 2 must sit at DIFFERENT
    // G-heights (era 2 carries the advanced sheet base). Without the
    // seating, every aeon stacks at one height — the recurrence
    // collision 11 §2.0 exists to prevent.
    let book3 = ledger::Ledger::at(var.join("worlds"));
    let eras3 = ledger::converge::default_era_roots(&var_str);
    ledger::converge::converge(&book3, &eras3).expect("third-aeon convergence");
    let chart = fs::read_to_string(var.join("worlds/starchart.json")).expect("chart");
    let gauge_of = |world: &str| -> String {
        chart
            .lines()
            .find(|line| line.contains(&format!("\"world\":\"{world}\"")))
            .and_then(|line| {
                line.split("\"g\":\"").nth(1).and_then(|rest| rest.split('\"').next())
            })
            .map(str::to_string)
            .unwrap_or_default()
    };
    let first = gauge_of("var-era1--cinder-reach");
    let second = gauge_of("var-era2--cinder-reach");
    assert!(!first.is_empty() && !second.is_empty(), "both aeons chart");
    assert_ne!(
        first, second,
        "two aeons never stack at one G-height — the sheets separate"
    );

    // The third convergence re-created a live board (annexed charts),
    // so a third fold is LAWFUL — the walk continues; and only when
    // truly nothing lives does the fold refuse honestly.
    let (third, _) = fold(&var_str).expect("the third aeon folds");
    assert_eq!(third, 3);
    assert!(fold(&var_str).is_err(), "nothing lives; nothing folds");
}
