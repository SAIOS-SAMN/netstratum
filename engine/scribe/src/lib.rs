//! THE SCRIBE (#37's inversion) — the display WRITERS, moved ABOVE
//! the lens. The layering, top to bottom:
//!
//! ```text
//! scribe  (this crate: journals, the wall, state/chronicle exports,
//!          solve records — the pens that WRITE the displays)
//!   ↓ consumes
//! lens    (projections/renderers — reads the chain, never a ledger
//!          file; owns the display JSON emitter)
//!   ↓ consumes
//! ledger  (LAW ONLY: the steward seam, converge, registry, galaxy,
//!          the flat-JSON extractors — the pre-chain Tier-A debts)
//! ```
//!
//! The cycle the inversion dissolved: the ledger once held both the
//! law and the display writers, so it leaned on the lens for its
//! renderers while the lens was meant to stand alone. The writers now
//! live HERE, the ledger imports no lens, and the lens imports no
//! ledger — the arrows point one way.
//!
//! **THE WRITE-ONLY LAW (§11 rule 2):** exports are deletable and
//! regenerable. Colony authority is `chronicle.tlv` + `state.tlv`.
//! No JSON is written for colony or ledger cards (`docs/JSON_DISPLAY_ONLY.md`).

use cfe::world::World;
use ledger::{card, sanitize, Ledger};
use std::fs;

pub mod journal;

/// Reflect one colony: write `state.tlv` + `world.card`, rotate on
/// chronicle divergence. Call after every command.
pub fn record(
    book: &Ledger,
    world_name: &str,
    planet: &str,
    civilization: &str,
    world: &World,
) -> std::io::Result<()> {
    let dir = book.world_dir(world_name);
    rotate_divergent(&dir, world)?;
    fs::create_dir_all(dir.join("inbox"))?;
    fs::create_dir_all(dir.join("solves"))?;
    let planet_dir = dir.join("planets").join(sanitize(planet));
    fs::create_dir_all(&planet_dir)?;

    // state.tlv — committed orb wire (machine mirror).
    if let Some(orb) = world.orb() {
        let tmp = planet_dir.join("state.tlv.tmp");
        fs::write(&tmp, orb.wire_bytes())?;
        fs::rename(&tmp, planet_dir.join("state.tlv"))?;
    }

    // world.card — TLV labels (not law).
    let mut planets: Vec<String> = fs::read_dir(dir.join("planets"))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    planets.sort();
    {
        let mut out = Vec::new();
        card::put_str(card::world::NAME, world_name, &mut out);
        for p in &planets {
            card::put_str(card::world::PLANET, p, &mut out);
        }
        card::put_str(30, civilization, &mut out);
        card::put_str(31, planet, &mut out);
        card::write_atomic(&dir.join("world.card"), &out)?;
    }

    // Drop legacy JSON display files if present.
    let _ = fs::remove_file(planet_dir.join("state.json"));
    let _ = fs::remove_file(dir.join("world.json"));
    let _ = fs::remove_file(dir.join("chronicle.jsonl"));
    Ok(())
}

/// Divergence: compare resident **chronicle.tlv** digests to the live world.
fn rotate_divergent(dir: &std::path::Path, world: &World) -> std::io::Result<()> {
    let receipts = world.chronicle().receipts();
    if receipts.is_empty() {
        return Ok(());
    }
    let tlv_path = dir.join("chronicle.tlv");
    if !tlv_path.exists() {
        return Ok(());
    }
    let Ok(resident) = cfe::persist::load(&tlv_path) else {
        return Ok(());
    };
    let res = resident.receipts();
    if res.is_empty() {
        return Ok(());
    }
    let check = res.len().min(receipts.len()) - 1;
    if res[check].own.0 == receipts[check].own.0 {
        return Ok(());
    }
    let terminal = hex_digest(&res.last().expect("nonempty").own.0);
    rotate_world_dir(dir, &terminal)
}

fn hex_digest(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Rotate a world directory to `--closed-<digest8>` archive.
fn rotate_world_dir(dir: &std::path::Path, terminal_digest: &str) -> std::io::Result<()> {
    let tag: String = terminal_digest.chars().take(8).collect();
    let base = dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let mut closed = dir.with_file_name(format!("{base}--closed-{tag}"));
    let mut bump = 1u32;
    while closed.exists() {
        bump += 1;
        closed = dir.with_file_name(format!("{base}--closed-{tag}-{bump}"));
    }
    fs::rename(dir, closed)
}

/// Record a proof-of-solution as TLV (`.sol`), not JSON.
/// `depth` is the difficulty rung the solve was earned at.
pub fn record_solve(
    book: &Ledger,
    world: &str,
    task: &str,
    program: &str,
    certificate_hex: &str,
    depth: u32,
) -> std::io::Result<()> {
    let dir = book.world_dir(world).join("solves");
    fs::create_dir_all(&dir)?;
    let mut out = Vec::new();
    card::put_str(card::sol::TASK, task, &mut out);
    card::put_str(card::sol::PROGRAM, program, &mut out);
    card::put_str(card::sol::CERT, certificate_hex, &mut out);
    card::put_u64(card::sol::DEPTH, u64::from(depth), &mut out);
    card::write_atomic(&dir.join(format!("{}.sol", sanitize(task))), &out)
}

pub use journal::{record_journal, render_wall};
