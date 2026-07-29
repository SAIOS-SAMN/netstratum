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
    // its own so stray siblings never leak into era discovery.
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
    let mirror = mirror::Mirror::at(var.join("worlds"));
    let eras = mirror::converge::default_era_roots(&var_str);
    assert!(
        eras.iter().any(|(name, _)| name == "var-era1"),
        "the archive sequence sees the folded era"
    );
    let converged = mirror::converge::converge(&mirror, &eras).expect("convergence");
    assert_eq!(
        converged.adopted.len(),
        count,
        "every folded world annexes into the new aeon"
    );

    // Convergence rebuilt the live universe (the annexed charts ARE
    // the new aeon's board) — so the NEXT fold is lawful: era 2, the
    // sheet advancing again. The aeon sequence is a walk, not a
    // one-shot.
    let (second, next_gauge) = fold(&var_str).expect("the second aeon folds");
    assert_eq!(second, 2);
    assert_eq!(next_gauge, ratio(1, 6), "2/12 wrapped — the sheets stack");
    assert!(var.parent().expect("holder").join("var-era2").is_dir());

    // With nothing live at all, the fold refuses honestly.
    assert!(fold(&var_str).is_err(), "nothing lives; nothing folds");
}
