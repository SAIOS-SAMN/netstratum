//! Binary TLV cards for ledger surfaces — **not JSON**.
//!
//! Same shape as NS-1: `tag u8 ‖ LE32(len) ‖ value`. Unknown tags skip
//! (SEAL A). Casual hand-editing is impractical; the engine rewrites
//! cards whole via atomic rename. Colony law remains `chronicle.tlv`.
//!
//! Used by: registry · universe · starchart · galaxies · world card ·
//! steward directives · solve certificates.

use std::fs;
use std::path::Path;

/// Put one TLV record.
pub fn put(tag: u8, value: &[u8], out: &mut Vec<u8>) {
    out.push(tag);
    out.extend_from_slice(&(value.len() as u32).to_le_bytes());
    out.extend_from_slice(value);
}

pub fn put_str(tag: u8, s: &str, out: &mut Vec<u8>) {
    put(tag, s.as_bytes(), out);
}

pub fn put_u64(tag: u8, v: u64, out: &mut Vec<u8>) {
    put(tag, &v.to_le_bytes(), out);
}

/// Iterate records; stop on torn tail (never guess).
pub fn records(data: &[u8]) -> impl Iterator<Item = (u8, &[u8])> + '_ {
    RecordIter { data, at: 0 }
}

struct RecordIter<'a> {
    data: &'a [u8],
    at: usize,
}

impl<'a> Iterator for RecordIter<'a> {
    type Item = (u8, &'a [u8]);
    fn next(&mut self) -> Option<Self::Item> {
        if self.at + 5 > self.data.len() {
            return None;
        }
        let tag = self.data[self.at];
        let len = u32::from_le_bytes(
            self.data[self.at + 1..self.at + 5]
                .try_into()
                .expect("4 bytes"),
        ) as usize;
        let start = self.at + 5;
        let end = start.checked_add(len)?;
        if end > self.data.len() {
            return None;
        }
        let value = &self.data[start..end];
        self.at = end;
        Some((tag, value))
    }
}

pub fn str_of(value: &[u8]) -> Option<String> {
    std::str::from_utf8(value).ok().map(str::to_string)
}

pub fn u64_of(value: &[u8]) -> Option<u64> {
    if value.len() != 8 {
        return None;
    }
    Some(u64::from_le_bytes(value.try_into().ok()?))
}

/// Atomic write: tmp then rename.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, bytes)?;
    fs::rename(&tmp, path)
}

pub fn read(path: &Path) -> Option<Vec<u8>> {
    fs::read(path).ok()
}

// ── Tag tables (append-only; never reuse) ──────────────────────────

/// Civilization registry card (`var/registry/<sigil>.civ`).
pub mod civ {
    pub const NAME: u8 = 1;
    pub const SIGIL: u8 = 2;
    pub const SPECIES: u8 = 3;
    pub const CLADE: u8 = 4;
    pub const MOTTO: u8 = 5;
    pub const DISPOSITION: u8 = 6;
    pub const METABOLISM: u8 = 7;
    pub const PERCEPTION: u8 = 8;
    pub const FOUNDER: u8 = 9;
    pub const LINEAGE: u8 = 10; // space-joined or repeated
    pub const STATION: u8 = 11;
}

/// Universe card (`var/worlds/universe.card`).
pub mod uni {
    pub const TOPOLOGY: u8 = 1;
    pub const SPAN: u8 = 2;
    pub const TUNIVERSAL: u8 = 3;
    pub const TICKS: u8 = 4;
    pub const FIBER: u8 = 5;
}

/// One starchart entry (concatenated in `starchart.card`).
pub mod chart {
    pub const WORLD: u8 = 1;
    pub const X: u8 = 2;
    pub const Y: u8 = 3;
    pub const Z: u8 = 4;
    pub const G: u8 = 5;
    /// Separator between entries (empty value).
    pub const END: u8 = 0;
}

/// Galaxy card entry (concatenated in `galaxies.card`).
pub mod galaxy {
    pub const NAME: u8 = 1;
    pub const CX: u8 = 2;
    pub const CY: u8 = 3;
    pub const CZ: u8 = 4;
    pub const REACH: u8 = 5;
    pub const END: u8 = 0;
}

/// World directory card (`world.card`).
pub mod world {
    pub const NAME: u8 = 1;
    pub const PLANET: u8 = 2; // repeated
}

/// Steward directive (`inbox/*.dir`).
pub mod dir {
    pub const VERB: u8 = 1;
    pub const ARG: u8 = 2; // repeated
    pub const ACTING_AS: u8 = 3;
}

/// Solve certificate (`solves/<task>.sol`).
pub mod sol {
    pub const TASK: u8 = 1;
    pub const PROGRAM: u8 = 2;
    pub const CERT: u8 = 3;
    pub const DEPTH: u8 = 4;
}
