//! **THE DEFINITION LAW, pinned** (`conductor::polities`, 2026-08-05).
//!
//! > A definition COMPILES. Only a receipt PERSISTS.
//!
//! This file previously pinned the shape of `polities.json` against the
//! brace-counting loader that read it — five witnesses proving a merged
//! roster file still parsed. The file, the loader and the merge tool are
//! all struck, so those witnesses are replaced rather than deleted: what
//! they were really protecting was *no line silently vanishes between the
//! record and the board*, and that property still needs pinning. It is
//! simply guarded by the compiler now instead of by a substring reader.
//!
//! The one witness kept verbatim is the fifth seat.

use conductor::{founding_polities, roster};
use ledger::registry::{Civilization, Registry};
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "roster-definition-pin-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn card(name: &str, sigil: &str, station: Option<&str>) -> Civilization {
    Civilization {
        name: name.into(),
        sigil: sigil.into(),
        species: "test organism".into(),
        clade: String::new(),
        motto: "measured, not asserted".into(),
        disposition: "empiricist".into(),
        metabolism: "biotic".into(),
        perception: "planar".into(),
        founder: "operator".into(),
        lineage: Vec::new(),
        station: station.map(|s| s.into()),
    }
}

/// **NO ROSTER FILE IS READ, ANYWHERE.**
///
/// The direct expression of the law. `conductor/src` must not name a
/// roster artifact at all — not to seed it, not to adopt a legacy name,
/// not to archive a copy. This is the pin that makes the law structural:
/// re-introducing a file roster means deleting a witness, which is the
/// amendment path (the recurrence guard's own discipline).
#[test]
fn the_roster_is_compiled_and_no_file_is_read() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut named = Vec::new();
    for entry in fs::read_dir(&src).expect("conductor src") {
        let path = entry.expect("entry").path();
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        let text = fs::read_to_string(&path).expect("source reads");
        // The law's own prose names the struck file to explain itself;
        // only CODE lines count as a read-back.
        for line in text.lines().filter(|l| !l.trim_start().starts_with("//")) {
            if line.contains("polities.json") {
                named.push(format!("{}: {}", path.display(), line.trim()));
            }
        }
    }
    assert!(
        named.is_empty(),
        "a roster file came back into conductor/src — the definition law bars it:\n{}",
        named.join("\n")
    );
}

/// **EVERY REGISTERED LINE REACHES THE BOARD.**
///
/// The property the struck pins actually protected. `civ new` writes a
/// registry card and nothing else; if the roster did not pick that card
/// up, the line would read as registered and never play — which is the
/// old silent-skip failure wearing new clothes.
#[test]
fn every_registered_line_reaches_the_board() {
    let root = scratch("registered");
    fs::create_dir_all(&root).expect("scratch");
    let registry = Registry::at(&root);
    registry.create(&card("Aerie", "aerie", None)).expect("registers");
    registry.create(&card("Windward", "windward", None)).expect("registers");

    let all = roster(&registry);
    let founding = founding_polities();
    assert_eq!(
        all.len(),
        founding.len() + 2,
        "the compiled lines plus every registered one — nothing dropped"
    );
    for sigil in ["aerie", "windward"] {
        let seated = all
            .iter()
            .find(|spec| spec.civilization.sigil == sigil)
            .unwrap_or_else(|| panic!("{sigil} registered but never reached the board"));
        assert_eq!(seated.world_name, format!("{sigil}-hearth"), "sited by derivation");
        assert!(
            seated.star.is_none() && seated.albedo.is_none() && seated.orbit.is_none(),
            "a card-borne line is UNSITED — its orbit derives at genesis"
        );
    }
}

/// **A REGISTERED LINE NEVER DOUBLES A COMPILED ONE.**
///
/// The founding lines have registry cards too (genesis writes them), so
/// a naive union would seat every founder twice on the second boot.
#[test]
fn a_registered_card_never_doubles_a_compiled_line() {
    let root = scratch("double");
    fs::create_dir_all(&root).expect("scratch");
    let registry = Registry::at(&root);
    registry.create(&card("Tunborn", "tunborn", None)).expect("registers");

    let all = roster(&registry);
    let tunborn: Vec<_> = all.iter().filter(|s| s.civilization.sigil == "tunborn").collect();
    assert_eq!(tunborn.len(), 1, "one line, one seat");
    assert_eq!(
        tunborn[0].civilization.station.as_deref(),
        Some("progenitor"),
        "and the COMPILED seat wins — the card cannot demote a chartered line"
    );
}

/// **NO TWO LINES SHARE A PLACEMENT.**
///
/// The struck loader's worst silent failure was geometric: a missing
/// coordinate key defaulted to the origin, so two worlds landed on the
/// same point and nothing said so. The derived placement resolves
/// collisions deterministically instead.
#[test]
fn no_two_lines_share_a_placement() {
    let root = scratch("placement");
    fs::create_dir_all(&root).expect("scratch");
    let registry = Registry::at(&root);
    for n in 0..24 {
        registry
            .create(&card(&format!("Line {n}"), &format!("line-{n}"), None))
            .expect("registers");
    }

    let all = roster(&registry);
    let mut seen = Vec::new();
    for spec in &all {
        let at = (
            spec.position.0.to_integer(),
            spec.position.1.to_integer(),
            spec.position.2.to_integer(),
        );
        assert!(
            !seen.contains(&at),
            "{} landed on an occupied point {at:?}",
            spec.civilization.sigil
        );
        seen.push(at);
    }
}

/// **THE PLACEMENT IS STABLE ACROSS MACHINES.**
///
/// Derived, so it must be a pure function of the sigil and the roster —
/// not of directory order, which `read_dir` gives no guarantee about.
#[test]
fn the_placement_is_the_same_on_every_reading() {
    let root = scratch("stable");
    fs::create_dir_all(&root).expect("scratch");
    let registry = Registry::at(&root);
    for sigil in ["zeta", "alpha", "mu"] {
        registry.create(&card(sigil, sigil, None)).expect("registers");
    }
    let first = roster(&registry);
    let second = roster(&registry);
    let read = |all: &[conductor::PolitySpec]| -> Vec<String> {
        all.iter()
            .map(|s| format!("{} @ {} {} {}", s.civilization.sigil, s.position.0, s.position.1, s.position.2))
            .collect()
    };
    assert_eq!(read(&first), read(&second), "two readings, one board");
}

/// **EVERY ARCHETYPE CLASS KEEPS ITS SEATED LINES** (`15_TAXON` §2).
///
/// This pin is the inverse of the one it replaces, which asserted that
/// six mythologically-named lines stayed OUT of the roster. That was a
/// category error the operator corrected: occult terminology in the LAW
/// (mythology invoked as an authority, as the retired `18_TORSION` once did) is
/// struck; ARCHETYPES for civilization classes are game content and
/// belong here.
///
/// Left standing, the error had already cost something measurable —
/// class 7 (apex-terrestrial) is seated by Lyran and Anunnaki ALONE, so
/// holding them out emptied a class, and clade 19 `draconic` was
/// orphaned by dropping Draco, the one line that wears it.
///
/// So the guard runs the other way: the taxonomy's named occupants must
/// be IN the roster. A class the partition says is seated, and which no
/// compiled line occupies, is a class that exists only on paper.
#[test]
fn every_archetype_class_keeps_its_seated_lines() {
    // `15_TAXON` §2's own table, by class code — only the lines the
    // partition names as FOUNDING occupants (grown lines join at play).
    const SEATED: [(u8, &str, &[&str]); 6] = [
        (1, "biotic-swarm", &["hivelight", "klethaxu", "mycelial-weft", "coralline", "deepmat"]),
        (2, "geodic-lithovore", &["slagborn", "tunborn", "agarthan"]),
        (3, "metashifter", &["darlok"]),
        (4, "telepathic-resonant", &["syzygy", "archon", "draco"]),
        (6, "plasma-field", &["coronal-choir"]),
        (7, "apex-terrestrial", &["lyran", "anunnaki"]),
    ];
    let founding = founding_polities();
    let mut empty = Vec::new();
    for (code, class, occupants) in SEATED {
        let present: Vec<&str> = occupants
            .iter()
            .copied()
            .filter(|sigil| founding.iter().any(|s| s.civilization.sigil == *sigil))
            .collect();
        assert!(
            !present.is_empty(),
            "class {code} ({class}) has NO occupant in the roster — \
             the partition names {occupants:?} and the board seats none"
        );
        if present.len() < occupants.len() {
            empty.push(format!("class {code} ({class}): {present:?} of {occupants:?}"));
        }
    }
    assert!(
        empty.is_empty(),
        "a class lost a seated line — the archetype partition and the \
         roster must agree:\n  {}",
        empty.join("\n  ")
    );

    // AND THE RULED SUB-ARCHETYPE IS WORN. Clade 19 `draconic` seats
    // INSIDE class 4 beside Archon (operator ruling 2026-08-01: one
    // archetype, two expressions). A clade with no line wearing it is a
    // ruling with no expression on the board it governs.
    let draco = founding
        .iter()
        .find(|s| s.civilization.sigil == "draco")
        .expect("clade 19 draconic needs the line that wears it");
    assert_eq!(draco.civilization.clade, "draconic");
    let archon = founding
        .iter()
        .find(|s| s.civilization.sigil == "archon")
        .expect("the class-4 archetype Draco expresses");
    assert_ne!(
        draco.civilization.clade, archon.civilization.clade,
        "two expressions, or it is not a sub-archetype"
    );
}

/// **THE FIFTH SEAT** (`12_UNION` §2, operator ruling 2026-07-30).
///
/// Kept verbatim from the struck file, with one word changed: this is no
/// longer the *fallback* roster, it is the roster. The failure it was
/// written for — a board booting the built-in defaults and losing the
/// progenitor band — is now unreachable, because there is no second
/// roster to fall back to.
#[test]
fn the_roster_seats_the_tardigrade_progenitor() {
    let founding = founding_polities();

    let tunborn = founding
        .iter()
        .find(|p| p.civilization.name == "Tunborn")
        .expect("the fifth seat must exist in the roster");
    assert_eq!(
        tunborn.civilization.station.as_deref(),
        Some("progenitor"),
        "the tardigrade line's head sits in the progenitor band"
    );
    assert_eq!(
        ledger::registry::tier_of(tunborn.civilization.station.as_deref()),
        Some(ledger::registry::Tier::T0),
        "a progenitor reads T0 — the seat is the tier, derived not asserted"
    );

    let slagborn = founding
        .iter()
        .find(|p| p.civilization.name == "Slagborn")
        .expect("Slagborn stands");
    assert_eq!(
        slagborn.civilization.lineage,
        vec!["tunborn".to_string()],
        "the line resolves to its own head, or the lineage law reads nothing"
    );

    // AND THE BAND IS NOT EMPTY. The failure this pin exists for was
    // not a wrong station on one line — it was a board with no seated
    // office at all, where every tier gate was unexercisable.
    let seated = founding
        .iter()
        .filter(|p| p.civilization.station.is_some())
        .count();
    assert!(
        seated > 0,
        "a board with no seat cannot exercise one tier law it carries"
    );
}
