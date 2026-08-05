//! The conductor pins: an era is deterministic, the board comes alive
//! unattended, policies diverge, the steward leash closes
//! end-to-end, research unlocks industry, and a thriving line
//! speciates — chronicle-chosen, lineage-recorded.

use conductor::{founding_polities, run_era, Polity};
use ledger::registry::Registry;
use ledger::Ledger;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "conductor-pin-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// The board lives: after an era every founding world has receipts, a
/// linked booked chain, and confirmed solves with depth rungs — no
/// operator involved.
#[test]
fn an_era_brings_the_board_alive() {
    let root = scratch("alive");
    let var = root.to_string_lossy().into_owned();
    let worlds = run_era(&var, 12);
    assert!(worlds.len() >= 3);

    for polity in worlds.iter().take(3) {
        assert!(polity.world.now() >= 12, "the clock advanced");
        assert!(
            polity.world.chronicle().receipts().len() > 5,
            "{}: receipts accumulated",
            polity.spec.civilization.sigil
        );
        assert!(polity.solved > 0, "{}: research confirmed solves", polity.spec.civilization.sigil);
        let world_dir = root.join("worlds").join(&polity.spec.world_name);
        assert!(world_dir.join("chronicle.jsonl").exists());
        let solves = fs::read_dir(world_dir.join("solves")).expect("solves dir").count();
        assert!(solves > 0, "solve certificates booked");
    }
    let registry = Registry::at(root.join("registry"));
    assert!(registry.list().len() >= 3, "the registry filled itself");
}


/// The policies actually diverge — different temperaments, different
/// cities by mid-era.
#[test]
fn policies_diverge() {
    let root = scratch("diverge");
    let worlds = run_era(&root.to_string_lossy(), 15);
    let zones: Vec<usize> = worlds
        .iter()
        .map(|b| b.world.orb().map(|o| o.zones.len()).unwrap_or(0))
        .collect();
    assert!(
        zones.windows(2).any(|w| w[0] != w[1]),
        "temperaments produce different cities: {zones:?}"
    );
}

/// RESEARCH UNLOCKS INDUSTRY: rung 3 opens smelters; by era's end a
/// rich civilization is smelting — the pyramid loop closed (surplus →
/// research → technique → industry).
#[test]
fn research_unlocks_industry() {
    let root = scratch("unlock");
    let worlds = run_era(&root.to_string_lossy(), 45);
    let smelting = worlds.iter().any(|polity| {
        polity.level >= 3
            && polity.world.orb().is_some_and(|orb| {
                orb.zones
                    .iter()
                    .any(|z| z.kind == cfe::dop::ZoneKind::Smelter)
            })
    });
    assert!(smelting, "an earned rung turned surplus into smelters");
}

/// DIVERGENCE (AEON-1b, 11 §3): a long era CLEAVES at least one line
/// into peer branches — line membership recorded in the registry, one
/// mutated axis, a new world beating alongside, and the KEYS move:
/// the origin receipts `Diverged` (+delta) and the branch founds at
/// the opposite phase — the branches are distinguishable on the G
/// fiber, never a parent and a child.
#[test]
fn a_thriving_line_diverges() {
    let root = scratch("speciate");
    let var = root.to_string_lossy().into_owned();
    let worlds = run_era(&var, 45);
    assert!(
        worlds.iter().any(|b| !b.spec.civilization.lineage.is_empty()),
        "a branch joined the board"
    );

    let registry = Registry::at(root.join("registry"));
    // **LINEAGE IS MEMBERSHIP, NOT PARENTHOOD** (LINEAGE-1). A SEEDED
    // line may carry its own head's slug and share its axes exactly —
    // Slagborn stands in the Tunborn line and is not a cleave of it.
    // This filtered on `!lineage.is_empty()` and so counted the seeded
    // line as a branch, then demanded it differ from its own head.
    //
    // Caught 2026-08-04 when the fifth seat was added to the fallback
    // roster and Slagborn gained the lineage the law always gave it.
    // The pin was reading the field, not the event: a CLEAVE is what
    // `divergence_step` produces at runtime, so the seeded roster is
    // what must be excluded to find one.
    let seeded: Vec<String> = conductor::founding_polities()
        .iter()
        .map(|p| p.civilization.sigil.clone())
        .collect();
    let branches: Vec<_> = registry
        .list()
        .into_iter()
        .filter(|c| !c.lineage.is_empty() && !seeded.contains(&c.sigil))
        .collect();
    assert!(!branches.is_empty(), "a cleave was recorded in the registry");

    for branch in &branches {
        let origin = registry
            .load(branch.lineage.first().expect("line"))
            .expect("the origin line exists");
        let mutations = [
            branch.disposition != origin.disposition,
            branch.metabolism != origin.metabolism,
            branch.perception != origin.perception,
        ]
        .iter()
        .filter(|m| **m)
        .count();
        // UNARY UNPINNED (operator ruling 2026-08-02). This asserted
        // `== 1` — "exactly one axis mutates per cleave" — which HELD
        // THE RETIRED MODEL IN PLACE BY TEST: `11_AEON` §3 retired
        // "Charted" precisely for terrestrial unary budding, one
        // parent MUTATING ONE AXIS, and the pin required exactly that
        // forever. A test can keep a struck model alive as surely as a
        // document can.
        //
        // The corrected law: a cleave draws a NONEMPTY SUBSET of the
        // three axes, so it always diverges (never a clone) and may
        // move on any combination. One axis is now one case among
        // seven, not the only case.
        assert!(
            (1..=3).contains(&mutations),
            "a cleave moves at least one axis and at most all three, got {mutations}"
        );
        // PEERS: the species is the line's, whole — the kinship
        // suffix is retired (11 §3: "Charted" imported terrestrial
        // unary budding, one parent mutating one axis, and it is
        // half a model with no second half).
        assert!(!branch.species.contains("descendant"));
        assert_eq!(branch.species, origin.species, "one line, one species");
    }

    // The keys moved: the origin receipts the cleave (+delta) and the
    // branch world founds at the OPPOSITE phase — never both at zero.
    // Same distinction as above: a SEEDED line carries its head's slug
    // and never cleaved. Only a runtime branch has a `Diverged` receipt
    // to find.
    let newborn = worlds
        .iter()
        .find(|b| {
            !b.spec.civilization.lineage.is_empty()
                && !seeded.contains(&b.spec.civilization.sigil)
        })
        .expect("newborn world");
    // **THE CLEAVING WORLD IS FOUND BY ITS RECEIPT, NOT BY THE LINEAGE
    // POINTER** (LINEAGE-1, corrected 2026-08-05).
    //
    // This read `lineage.first()` — the LINE HEAD — and called it the
    // origin. LINEAGE-1 is explicit that the pointer records LINE
    // MEMBERSHIP, never parenthood, so on a second-generation cleave the
    // head is a GRANDPARENT and the phase law says nothing about the
    // pair. It held only while the roster was small enough that every
    // cleave was first-generation; THE DEFINITION LAW compiled the full
    // 19-line roster in, a deeper cleave appeared, and the head and the
    // grandchild landed on the same phase — a true reading, failed
    // against the wrong world.
    //
    // The receipt names the parent unambiguously: exactly one world
    // receipted `Diverged { branch: newborn }`.
    let origin_world = worlds
        .iter()
        .find(|b| {
            b.world.chronicle().receipts().iter().any(|r| {
                matches!(
                    &r.event,
                    cfe::dop::ClosureEvent::Diverged { branch, .. }
                        if *branch == newborn.spec.civilization.sigil
                )
            })
        })
        .expect("the world that receipted this cleave beats on");
    assert!(
        !newborn.spec.civilization.lineage.is_empty(),
        "the branch stands in a line"
    );
    // **THE PHASES ARE READ AT THE CLEAVE, NOT AT ERA'S END**
    // (corrected 2026-08-05).
    //
    // The law is about the MOMENT of the cleave: the origin's key moves
    // to `g + δ` and the branch founds at `g − δ`, with δ drawn in
    // 1/12..=5/12. Those can never coincide — collision would need
    // 2δ ≡ 0 (mod 1), and 2δ lands in 2/12..=10/12.
    //
    // This read both worlds' gauges at the END of the era instead, by
    // which time keys have moved on for reasons that have nothing to do
    // with the cleave. It held only while the roster was small; with the
    // full 19-line roster two unrelated worlds sat at 1/6 apiece and the
    // pin reported a fiber collision that the arithmetic forbids.
    //
    // The receipt carries the origin's key AT the cleave, and the
    // branch's spec carries the phase it founded at.
    let at_cleave = origin_world
        .world
        .chronicle()
        .receipts()
        .iter()
        .find_map(|r| match &r.event {
            cfe::dop::ClosureEvent::Diverged { branch, gauge }
                if *branch == newborn.spec.civilization.sigil =>
            {
                Some(gauge.clone())
            }
            _ => None,
        })
        .expect("the cleave receipt carries the origin's key");
    let founded_at = newborn
        .spec
        .gauge
        .clone()
        .expect("the branch founds at an explicit phase");
    assert_ne!(at_cleave, cfe::ratio::ratio(0, 1), "the origin's key moved");
    assert_ne!(
        founded_at,
        cfe::ratio::ratio(0, 1),
        "the branch founds at its cleaved key"
    );
    assert_ne!(
        at_cleave, founded_at,
        "opposite phases — the branches never collide on the fiber"
    );
    assert!(newborn.world.now() > 0, "the branch's clock runs");
    assert!(root
        .join("worlds")
        .join(&newborn.spec.world_name)
        .join("chronicle.jsonl")
        .exists());

    // THE STAR-CHART: the branch settled ADJACENT to its origin —
    // charted, exact, displaced but within the half-unit-step reach
    // (dist² ≤ 8, ≠ 0).
    let book = Ledger::at(root.join("worlds"));
    let parent_world = &worlds
        .iter()
        .find(|b| {
            Some(&b.spec.civilization.sigil)
                == newborn.spec.civilization.lineage.first()
        })
        .expect("the parent beats")
        .spec;
    // THEMED NAMES, NEVER CHAINS: a branch's world is a fresh
    // two-part themed name — provenance lives in the lineage field and
    // the star-chart adjacency, not in ever-growing strings.
    assert!(
        !newborn.spec.world_name.contains(&parent_world.world_name),
        "no name concatenation: {}",
        newborn.spec.world_name
    );
    assert!(
        newborn.spec.world_name.matches('-').count() <= 1,
        "two segments max: {}",
        newborn.spec.world_name
    );

    // L1 toroidal distance (the L1 law: no squaring) — displaced but
    // within the half-step reach: 0 < L1 ≤ 6 (three axes × 2 max).
    let l1 = book
        .l1_distance(&parent_world.world_name, &newborn.spec.world_name)
        .expect("both charted");
    assert!(!num_traits::Zero::is_zero(&l1), "displaced, never on top");
    assert!(
        l1 <= cfe::ratio::ratio(6, 1),
        "adjacent on the torus: L1 = {l1} within reach"
    );
}

/// The leash works end-to-end: a steward directive queued in the
/// inbox applies through the contained steward entity on the next
/// heartbeat; an unknown verb refuses. Both archive.
#[test]
fn a_steward_directive_lands_through_the_leash() {
    let root = scratch("steward");
    let var = root.to_string_lossy().into_owned();
    let registry = Registry::at(format!("{var}/registry"));
    let book = Ledger::at(format!("{var}/worlds"));

    let mut worlds: Vec<Polity> = founding_polities()
        .into_iter()
        .map(|spec| Polity::genesis(spec, &registry, &book))
        .collect();
    for _ in 0..2 {
        for polity in &mut worlds {
            polity.cycle(&book, &registry);
        }
    }

    let inbox = book.world_dir("cinder-reach").join("inbox");
    fs::write(
        inbox.join("000001-zone.json"),
        r#"{"verb":"zone","args":["63","hab"],"as":"steward"}"#,
    )
    .expect("queues");
    fs::write(
        inbox.join("000002-smite.json"),
        r#"{"verb":"smite","args":[],"as":"steward"}"#,
    )
    .expect("queues");

    for polity in &mut worlds {
        polity.cycle(&book, &registry);
    }

    assert!(inbox.join("applied/000001-zone.json").exists(), "the zone applied");
    assert!(inbox.join("refused/000002-smite.json").exists(), "the smite refused");
    let acting = worlds
        .iter()
        .position(|p| p.spec.civilization.sigil == "slagborn")
        .expect("the acting line stands");
    assert!(
        worlds[acting]
            .world
            .orb()
            .expect("orb")
            .zones
            .iter()
            .any(|z| z.tile == 63),
        "tile 63 carries the steward's habitat"
    );
}

/// PERCEPTION ROUTES RESEARCH: the volumetric civilization's solves
/// are volumetric tasks (volf_*), the planar civilizations' are
/// planar (forge_*) — the genesis axis has mechanical teeth.
#[test]
fn perception_routes_the_research() {
    let root = scratch("perception");
    run_era(&root.to_string_lossy(), 8);
    let names = |world: &str| -> Vec<String> {
        fs::read_dir(root.join("worlds").join(world).join("solves"))
            .map(|dir| {
                dir.filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_default()
    };
    let hivelight = names("amber-veil"); // volumetric honeybees
    let slagborn = names("cinder-reach"); // planar tardigrades
    assert!(!hivelight.is_empty() && hivelight.iter().all(|n| n.starts_with("volf_")));
    assert!(!slagborn.is_empty() && slagborn.iter().all(|n| n.starts_with("forge_")));
}

/// **THE ROSTER IS THE COMPILED ROSTER** — the inverse of the pin this
/// replaces, which read *"lore lives in data: the default polities seed
/// a file the operator owns"*. THE DEFINITION LAW struck that creed:
/// a definition compiles, only a receipt persists.
///
/// The lines it named still stand, and they are checked BY IDENTITY,
/// never by position — the earlier version read `loaded[0]` and broke
/// the moment the fifth seat was added ahead of it.
#[test]
fn the_roster_carries_the_tardigrade_line_and_its_head() {
    let founding = founding_polities();
    assert!(
        founding.iter().any(|p| p.civilization.sigil == "slagborn"),
        "the tardigrade line stands in the roster"
    );
    assert!(
        founding.iter().any(|p| p.civilization.sigil == "tunborn"),
        "and so does its head — the fifth seat, 12_UNION §2"
    );
    // Two readings of a compiled roster are the same roster. The struck
    // path could not promise this: a second boot re-read a file the
    // first boot had written, and anything could have edited it between.
    assert_eq!(
        founding.len(),
        founding_polities().len(),
        "the roster does not change between readings"
    );
    assert!(
        founding.iter().all(|p| p.civilization.founder == "conductor"),
        "every compiled line is founded by the conductor, not by an editor"
    );
}
