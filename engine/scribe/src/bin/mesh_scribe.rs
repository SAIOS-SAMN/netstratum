//! THE MESH SCRIBE — netstratum attaches to the mycelium as a PEER.
//!
//! No exporter, no feed directory, no operator: the kernel's solve
//! records become offers over MY-K through the same attach loop every
//! kernel links (`mycelium::sdk::attach` — peel → type → offer). The
//! wire is the feed contract; verdicts, refusal semantics, and
//! once-per-work_id idempotency come from the mesh's own laws, so a
//! scribe that dies mid-beat re-attaches and continues — fault
//! tolerance inherited from the honesty machinery.
//!
//! Resident doctrine (#20): the seat outlives the socket (same
//! topological address = same kernel), liveness is beats, presence
//! buys a voice never a vote. A claim is spoken when NEW, or
//! re-spoken when it was withheld and the sun has since GROWN.
//!
//! STATUS: demoted to the MIGRATION VERB (Law II landed — mycelium
//! docs/23_RESIDENCY.md): live solves now offer themselves from the
//! conductor's solve-completion path (`scribe::membrane::offer`,
//! non-blocking, resident thread). This binary remains only to
//! backfill HISTORICAL solve records into the mesh — run once after
//! standing up a hypha, then delete it with a clear conscience.
//! Every re-utterance is a lawful no-op (installed orbits
//! short-circuit; once-per-work_id guards the court).
//!
//!   mesh-scribe --backfill        attach, speak every historical claim once, exit

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use mycelium::autic::letters::{LetterId, Word};
use mycelium::sdk::attach::{measurement, six_step, Arm};
use mycelium::sdk::{CrystalId, KernelRef, Session};

const WORLDS: &str = "/opt/netstratum/var/worlds";
const ADDR: &str = "hypha://127.0.0.1:7420";
const KERNEL: &str = "net";
const BEAT_SECS: u64 = 5;

type Grid = Vec<Vec<u8>>;

fn d4(name: &str, g: &Grid) -> Option<Grid> {
    let rows = g.len();
    let cols = g.first()?.len();
    let out = match name {
        "rotate90" => (0..cols)
            .map(|c| (0..rows).rev().map(|r| g[r][c]).collect())
            .collect(),
        "rotate180" => g.iter().rev().map(|r| r.iter().rev().copied().collect()).collect(),
        "rotate270" => (0..cols)
            .rev()
            .map(|c| (0..rows).map(|r| g[r][c]).collect())
            .collect(),
        "flip_h" => g.iter().map(|r| r.iter().rev().copied().collect()).collect(),
        "flip_v" => g.iter().rev().cloned().collect(),
        "transpose" => (0..cols)
            .map(|c| (0..rows).map(|r| g[r][c]).collect())
            .collect(),
        _ => return None,
    };
    Some(out)
}

const D4_NAMES: [&str; 6] = [
    "rotate90", "rotate180", "rotate270", "flip_h", "flip_v", "transpose",
];

fn solve_files() -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(worlds) = fs::read_dir(WORLDS) else {
        return out;
    };
    for w in worlds.filter_map(|e| e.ok()) {
        if let Ok(rd) = fs::read_dir(w.path().join("solves")) {
            out.extend(
                rd.filter_map(|e| e.ok().map(|e| e.path()))
                    .filter(|p| p.extension().is_some_and(|x| x == "sol")),
            );
        }
    }
    out.sort();
    out
}

fn printable(raw: &[u8]) -> String {
    raw.iter()
        .map(|&b| if (32..127).contains(&b) { b as char } else { ' ' })
        .collect()
}

/// A transform name counts only as a solve record: followed by '@'.
fn names_in(text: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    for name in D4_NAMES {
        for (at, _) in text.match_indices(name) {
            let tail = text[at + name.len()..].trim_start();
            if tail.starts_with('@') {
                found.insert(name.to_string());
                break;
            }
            if text[..at].ends_with("per_layer(") && tail.starts_with(')') {
                if tail[1..].trim_start().starts_with('@') {
                    found.insert(format!("per_layer_{name}_"));
                    break;
                }
            }
        }
    }
    found
}

/// Distinct standalone recolor[perm] records (compositions skipped —
/// named for a later pass, not silently dropped).
fn pure_recolors(files: &[PathBuf], limit: usize) -> Vec<[u8; 10]> {
    let mut seen: BTreeSet<[u8; 10]> = BTreeSet::new();
    let mut out = Vec::new();
    for f in files {
        let Ok(raw) = fs::read(f) else { continue };
        let text = printable(&raw);
        let mut rest = text.as_str();
        while let Some(at) = rest.find("recolor[") {
            rest = &rest[at + 8..];
            let Some(close) = rest.find(']') else { break };
            let inner = &rest[..close];
            let tail = &rest[close + 1..rest.len().min(close + 31)];
            rest = &rest[close + 1..];
            if ["flip", "rot", "transpose", "tile", "per_layer"]
                .iter()
                .any(|k| tail.contains(k))
            {
                continue;
            }
            let digits: Vec<u8> = inner
                .split(',')
                .filter_map(|d| d.trim().parse().ok())
                .collect();
            if digits.len() == 10 {
                let mut sorted = digits.clone();
                sorted.sort_unstable();
                if sorted == (0..10).collect::<Vec<u8>>() {
                    let mut perm = [0u8; 10];
                    perm.copy_from_slice(&digits);
                    if seen.insert(perm) {
                        out.push(perm);
                        if out.len() >= limit {
                            return out;
                        }
                    }
                }
            }
        }
    }
    out
}

/// The kernel's claims, read fresh from its own solve records:
/// (name, canonical pair). Deduped by name; content-addressed dedup
/// (same word, same orbit) happens mesh-side by law.
fn claims() -> Vec<(String, Grid, Grid)> {
    let files = solve_files();
    let base: Grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let hues: Grid = vec![vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9]];
    let mut names: BTreeSet<String> = BTreeSet::new();
    for f in &files {
        if let Ok(raw) = fs::read(f) {
            names.extend(names_in(&printable(&raw)));
        }
    }
    let mut out = Vec::new();
    for name in &names {
        let inner = name
            .strip_prefix("per_layer_")
            .and_then(|s| s.strip_suffix('_'))
            .unwrap_or(name);
        if let Some(img) = d4(inner, &base) {
            out.push((format!("net_{name}"), base.clone(), img));
        }
    }
    for perm in pure_recolors(&files, 400) {
        let img: Grid = hues
            .iter()
            .map(|row| row.iter().map(|&h| perm[h as usize]).collect())
            .collect();
        let tag: String = perm.iter().map(|d| d.to_string()).collect();
        out.push((format!("net_recolor_{tag}"), hues.clone(), img));
    }
    out
}

fn attach(occ: &mycelium::iae::peel::FullMeasurement) -> Option<Session> {
    let inv = Word::new(vec![LetterId::IDENTITY]);
    match Session::connect(ADDR, KernelRef::of(occ, None), occ, &inv, None) {
        Ok(s) => {
            eprintln!("mesh-scribe: kernel '{KERNEL}' seated at {ADDR}");
            Some(s)
        }
        Err(e) => {
            eprintln!("mesh-scribe: attach {ADDR} refused: {e:?}");
            None
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let resident = false; // Law II: residency lives in the membrane now
    if !args.iter().any(|a| a == "--backfill" || a == "--sync") {
        eprintln!("usage: mesh-scribe --backfill");
        std::process::exit(2);
    }
    // name → (arm code, sun words at attempt). 0 = installed: settled.
    let mut spoken: BTreeMap<String, (u8, usize)> = BTreeMap::new();
    let mut basis: Vec<CrystalId> = Vec::new();
    let mut session: Option<Session> = None;
    let mut words_seen = 0usize;
    loop {
        let work = claims();
        let Some(_) = work.first() else {
            if !resident {
                eprintln!("mesh-scribe: no solve records under {WORLDS}");
                return;
            }
            std::thread::sleep(std::time::Duration::from_secs(BEAT_SECS));
            continue;
        };
        if session.is_none() {
            // Stable position: the emblem, never the claim in hand.
            session = mycelium::sdk::attach::emblem(KERNEL, None).and_then(|occ| attach(&occ));
        }
        let Some(s) = session.as_mut() else {
            if !resident {
                std::process::exit(1);
            }
            std::thread::sleep(std::time::Duration::from_secs(BEAT_SECS));
            continue;
        };
        let words_now = s
            .nest_read()
            .map(|v| v.installed.len())
            .unwrap_or(words_seen);
        let mut sung = 0usize;
        let mut failed = 0usize;
        let mut attempted = 0usize;
        for (n, (name, input, output)) in work.iter().enumerate() {
            let due = match spoken.get(name) {
                None => true,
                Some((0, _)) => false,
                Some((_, w)) => *w != words_now,
            };
            if !due {
                continue;
            }
            attempted += 1;
            let pair = match (measurement(input), measurement(output)) {
                (Some(a), Some(b)) => vec![(a, b)],
                _ => continue,
            };
            let tag = u16::try_from(n).unwrap_or(u16::MAX);
            match six_step(s, KERNEL, &pair, tag, &mut basis) {
                Ok(Arm::ExactInstall) => {
                    spoken.insert(name.clone(), (0, words_now));
                    sung += 1;
                }
                Ok(Arm::Withhold { .. }) => {
                    spoken.insert(name.clone(), (1, words_now));
                }
                Ok(Arm::Unglueable) => {
                    spoken.insert(name.clone(), (2, words_now));
                }
                Err(_) => {
                    failed += 1; // retried next beat; socket loss heals below
                }
            }
        }
        if attempted > 0 {
            eprintln!(
                "mesh-scribe: {} claims · {attempted} spoken this beat · {sung} installed · {failed} errored · sun {words_now}",
                work.len()
            );
        }
        if !resident {
            return;
        }
        if failed > 0 && failed == attempted {
            session = None; // the seat outlives the socket: re-attach next beat
        }
        words_seen = words_now;
        std::thread::sleep(std::time::Duration::from_secs(BEAT_SECS));
    }
}
