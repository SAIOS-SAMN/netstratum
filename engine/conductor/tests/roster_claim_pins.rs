//! THE CLAIM PATH'S SHAPE, pinned against the loader that must read it.
//!
//! `tools/claim_roster.py` merges the live civilization record into the
//! durable roster (`docs/ENDOWMENTS.md` E1/E4). Its whole value is that
//! the merged file still LOADS — a roster entry `load_polities` skips is
//! worse than an absent one, because it reads as claimed while the
//! engine silently drops it (the skip is a bare stderr note).
//!
//! These fixtures are the tool's real emitted output, copied verbatim
//! from a dry run against the live universe on 2026-08-03. They do NOT
//! read `var/` — the pin must hold in a clean checkout where no universe
//! exists.
//!
//! Landed on the record lane. `engine/conductor/` is unassigned on the
//! lane map; relayed rather than claimed.

use conductor::load_polities;
use std::fs;
use std::path::PathBuf;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "roster-claim-pin-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// One line of each kind the merge produces, verbatim from the tool:
/// a CLAIMED line (registry-only, sited from the live universe, carrying
/// the `founder` and `clade` keys the roster writer never emits), a
/// MERGED line that was renamed (slug held, name from the live record),
/// and a PRESERVED line the live universe does not hold.
const MERGED_ROSTER: &str = r#"[
  {"name":"Aerie","slug":"aerie","species":"peregrine falcon","clade":"avian","motto":"the thermal is the throne","disposition":"expansionist","metabolism":"biotic","perception":"volumetric","world":"windward-spire","planet":"gale","colony":"first-roost","policy":"steady","x":"18","y":"30","z":"55","star":"g","founder":"operator"},
  {"name":"Brine Cortex","slug":"eightfold","species":"octopus","clade":"cephalopod","motto":"eight arms, one argument","disposition":"epicurean","metabolism":"biotic","perception":"volumetric","world":"jovian-reach","planet":"europa","colony":"brine-garden","policy":"steady","x":"5","y":"58","z":"61","founder":"conductor"},
  {"name":"Draco","slug":"draco","species":"coherent of the warm dark","motto":"the union is the evidence; the seat above re-checks it","disposition":"epicurean","metabolism":"biotic","perception":"umbral","world":"eclipse-hold","planet":"umbra","colony":"totality","policy":"steady","x":"-11","y":"16","z":"4","station":"observer","star":"k","albedo":"1/4","orbit":"3/4"},
  {"name":"Bloomdeep","slug":"bloomdeep","species":"octopus","clade":"cephalopod","motto":"one line, two songs","disposition":"epicurean","metabolism":"biotic","perception":"volumetric","world":"bloom-deep","planet":"deepfall","colony":"bloomdeep-landing","policy":"steady","lineage":"ferndeep"}
]
"#;

/// EVERY merged entry loads. Nothing is silently skipped.
///
/// This is the pin the whole claim path rests on: the tool emits keys
/// `write_default_polities` never writes (`clade`, `founder`, `lineage`,
/// `slug` in place of `sigil`), and the loader is a brace-counting
/// substring reader rather than a JSON parser, so "it is valid JSON" is
/// NOT evidence that it loads.
#[test]
fn every_merged_entry_survives_the_loader() {
    let root = scratch("loads");
    fs::create_dir_all(&root).expect("scratch");
    let path = root.join("polities.json");
    fs::write(&path, MERGED_ROSTER).expect("write");

    let loaded = load_polities(&path).expect("the merged roster loads");
    assert_eq!(loaded.len(), 4, "all four entries load — none skipped");
}

/// THE IDENTITY SURVIVES THE RENAME. `slug` carries into `sigil` (the
/// loader's `field("sigil").or_else(|| field("slug"))`), so a renamed
/// line keeps the identity every other record keys on while its display
/// name moves.
#[test]
fn a_renamed_line_keeps_its_slug_as_the_identity() {
    let root = scratch("rename");
    fs::create_dir_all(&root).expect("scratch");
    let path = root.join("polities.json");
    fs::write(&path, MERGED_ROSTER).expect("write");

    let loaded = load_polities(&path).expect("loads");
    let renamed = loaded
        .iter()
        .find(|spec| spec.civilization.sigil == "eightfold")
        .expect("the slug is the key, and it is still eightfold");
    assert_eq!(renamed.civilization.name, "Brine Cortex", "the live name is the later authority");
    assert_eq!(renamed.world_name, "jovian-reach");
}

/// The registry-only fields ride through. `clade` and `lineage` exist
/// only in the live record and would be lost by any merge that wrote the
/// roster's shape alone.
#[test]
fn the_live_records_own_fields_survive_the_merge() {
    let root = scratch("fields");
    fs::create_dir_all(&root).expect("scratch");
    let path = root.join("polities.json");
    fs::write(&path, MERGED_ROSTER).expect("write");

    let loaded = load_polities(&path).expect("loads");
    let claimed = loaded
        .iter()
        .find(|spec| spec.civilization.sigil == "aerie")
        .expect("the claimed line loads");
    assert_eq!(claimed.civilization.clade, "avian", "clade is registry-only and survives");
    assert_eq!(claimed.world_name, "windward-spire", "sited from the live universe");

    let cleaved = loaded
        .iter()
        .find(|spec| spec.civilization.sigil == "bloomdeep")
        .expect("the cleaved line loads");
    assert_eq!(cleaved.civilization.lineage, vec!["ferndeep".to_string()], "the cleave is recorded");
}

/// A PRESERVED line — one the live universe does not hold — keeps its
/// deliberate siting and its station seat. This is the half of the merge
/// that `cp` destroyed: these three exist ONLY in the durable roster.
#[test]
fn a_preserved_line_keeps_its_seat_and_its_siting() {
    let root = scratch("preserved");
    fs::create_dir_all(&root).expect("scratch");
    let path = root.join("polities.json");
    fs::write(&path, MERGED_ROSTER).expect("write");

    let loaded = load_polities(&path).expect("loads");
    let draco = loaded
        .iter()
        .find(|spec| spec.civilization.sigil == "draco")
        .expect("the never-seated line is still in the record");
    assert_eq!(draco.civilization.station.as_deref(), Some("observer"), "the chartered seat holds");
    assert_eq!(draco.world_name, "eclipse-hold");
    assert!(draco.star.is_some(), "its pinned star survives");
    assert!(draco.albedo.is_some() && draco.orbit.is_some(), "and its pinned orbit");
}

/// THE REFUSAL IS LOAD-BEARING, MEASURED HERE RATHER THAN ASSERTED IN
/// PROSE. An entry missing `world`/`planet`/`colony` does not refuse the
/// file — it is dropped, and the other entries load around it. That is
/// exactly why `claim_roster.py` refuses such a line at merge time
/// instead of writing it: written, it would read as claimed and be
/// invisible to the engine.
#[test]
fn an_entry_missing_its_siting_is_silently_dropped_not_refused() {
    let root = scratch("dropped");
    fs::create_dir_all(&root).expect("scratch");
    let path = root.join("polities.json");
    let unsited = r#"[
  {"name":"Aerie","slug":"aerie","species":"peregrine falcon","motto":"the thermal is the throne","disposition":"expansionist","metabolism":"biotic","perception":"volumetric","world":"windward-spire","planet":"gale","colony":"first-roost","policy":"steady"},
  {"name":"Forgechoir","slug":"forgechoir","species":"tardigrade","motto":"one line, two songs","disposition":"expansionist","metabolism":"mineral","perception":"harmonic","policy":"steady"}
]
"#;
    fs::write(&path, unsited).expect("write");

    let loaded = load_polities(&path).expect("the file still loads");
    assert_eq!(loaded.len(), 1, "the unsited entry vanished — no refusal, no error");
    assert_eq!(loaded[0].civilization.sigil, "aerie");
    assert!(
        !loaded.iter().any(|spec| spec.civilization.sigil == "forgechoir"),
        "a claim written without siting is a claim the engine never sees"
    );
}

/// **THE FIFTH SEAT IS IN THE FALLBACK ROSTER** (`12_UNION` §2,
/// operator ruling 2026-07-30).
///
/// The seed file has carried Tunborn at `station: progenitor` since the
/// ruling. The BUILT-IN defaults did not — and they are what a board
/// boots with whenever `<var>/polities.json` is absent, which
/// `write_default_polities` then refuses to overwrite. That is not a
/// rare path; it is the path a rebuilt var takes.
///
/// Measured 2026-08-04 on the live var: five lines, every one
/// `station: None`, every one T3, and the entire progenitor band absent
/// from play — so a landed ruling had no expression on the board it
/// governs. This pins the allocation so the fallback cannot drift from
/// the law again.
#[test]
fn the_fallback_roster_seats_the_tardigrade_progenitor() {
    let founding = conductor::founding_polities();

    let tunborn = founding
        .iter()
        .find(|p| p.civilization.name == "Tunborn")
        .expect("the fifth seat must exist in the fallback roster");
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
        "a fallback board with no seat cannot exercise one tier law it carries"
    );
}
