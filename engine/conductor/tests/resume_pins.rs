//! BLOCKER A1 — THE G0 PERSISTENCE SEAM, CLOSED. The conductor boot
//! RESUMES before it founds: a verifying carrier continues the
//! world's receipts, techs, and keys across a process restart; a
//! missing carrier founds fresh. No operator fold required for the
//! board to remember itself.

use conductor::{run_era, Polity};
use ledger::registry::Registry;
use ledger::Ledger;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("resume-pin-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

#[test]
fn the_board_remembers_across_restarts() {
    let root = scratch("remember");
    let var = root.to_string_lossy().into_owned();

    // First boot: a short era beats, then the process "dies" (drop).
    let first = run_era(&var, 5);
    // The truth that persists is the RECEIPTED truth: the terminal
    // digest. (Un-receipted ticks honestly do not survive a restart —
    // the workspace invariant applies to time itself; the clock
    // reinstates at the last receipt's stamp.)
    let before: Vec<(String, [u8; 32], usize, bool)> = first
        .iter()
        .map(|e| {
            let last = e.world.chronicle().receipts().last().expect("receipts");
            (
                e.spec.world_name.clone(),
                last.own().0,
                e.world.chronicle().receipts().len(),
                // THE SOLAR LEAP IS CAPTURED, NOT ASSUMED (2026-08-05).
                // This pin used to assert every resumed world knows its
                // sun outright. That is a property of the ROSTER — of
                // whether five beats happen to be enough for each line —
                // not of resumption, and it went red the moment THE
                // DEFINITION LAW compiled the fuller roster in: a line
                // that had not made the leap by beat 5 was read as a
                // world that FORGOT its sky.
                //
                // The law here is PRESERVATION: whatever a world knew,
                // it still knows after a restart. So the reading is
                // taken before and compared after, and a line still
                // climbing is no longer mistaken for a line that lost
                // something.
                e.world.knows_its_sun(),
            )
        })
        .collect();
    drop(first);

    // Second boot, same roster: the worlds RESUME — clocks continue,
    // nothing re-founds, no divergent resident rotates to a ruin.
    let registry = Registry::at(root.join("registry"));
    let book = Ledger::at(root.join("worlds"));
    let second: Vec<Polity> = conductor::founding_polities()
        .into_iter()
        .map(|spec| Polity::genesis(spec, &registry, &book))
        .collect();
    for polity in &second {
        let (_, terminal, count, knew_its_sun) = before
            .iter()
            .find(|(name, _, _, _)| name == &polity.spec.world_name)
            .expect("same roster");
        let resumed = polity.world.chronicle().receipts();
        assert_eq!(
            resumed.len(),
            *count,
            "{} carries its whole receipted history",
            polity.spec.world_name
        );
        assert_eq!(
            resumed.last().expect("receipts").own().0,
            *terminal,
            "{} continues the SAME chain — resumption, never a refound",
            polity.spec.world_name
        );
        assert_eq!(
            polity.world.knows_its_sun(),
            *knew_its_sun,
            "{} did not remember its sky across the restart",
            polity.spec.world_name
        );
    }
    // AND THE CLAIM IS NOT VACUOUS. Preservation of "knew nothing" is
    // no evidence of anything, so at least one line must actually have
    // made the solar leap for the comparison above to have content.
    assert!(
        before.iter().any(|(_, _, _, knew)| *knew),
        "no line made the solar leap in this era — the preservation \
         check above would be comparing false against false"
    );
    // No ruins: resumption never rotated a divergent resident.
    let ruins = std::fs::read_dir(root.join("worlds"))
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| e.file_name().to_string_lossy().contains("--closed-"))
                .count()
        })
        .unwrap_or(0);
    assert_eq!(ruins, 0, "no divergent residents — the same timeline continued");
}
