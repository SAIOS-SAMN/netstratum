//! THE MEMBRANE — Law II of the packaged shape (mycelium
//! docs/23_RESIDENCY.md): the solve offers itself.
//!
//! The conductor's solve-completion path calls [`offer`] with the
//! task name and the program string it already holds — at the moment
//! of solve, no paper round-trip, no watcher process. The call is
//! non-blocking: it sends down a channel and returns, so the world
//! beat never waits on the wire. A resident thread owns the mesh
//! session and speaks on its own schedule: attach lazily, re-attach
//! on socket loss (the seat outlives the socket — same topological
//! address, same kernel), keep unspoken offers pending until the
//! mesh hears them. Idempotency is the mesh's law (installed orbits
//! short-circuit; once-per-work_id guards the court), so nothing
//! here is durable state — Law I: the mesh is the memory.
//!
//! Programs this membrane can peel today: the D4 family, single-layer
//! `per_layer(d4)`, and standalone `recolor[perm]`. Compositions and
//! tiles return None — named for the C6 pass, never silently dropped.

use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::OnceLock;

use mycelium::autic::letters::{LetterId, Word};
use mycelium::sdk::attach::{measurement, six_step, Arm};
use mycelium::sdk::{CrystalId, KernelRef, Session};

pub type Grid = Vec<Vec<u8>>;

const ADDR_ENV: &str = "MYCELIUM_ADDR";
const ADDR_DEFAULT: &str = "hypha://127.0.0.1:7420";
const KERNEL: &str = "net";
const BEAT_SECS: u64 = 5;

pub const D4_NAMES: [&str; 6] = [
    "rotate90", "rotate180", "rotate270", "flip_h", "flip_v", "transpose",
];

pub fn d4(name: &str, g: &Grid) -> Option<Grid> {
    let rows = g.len();
    let cols = g.first()?.len();
    let out = match name {
        "rotate90" => (0..cols)
            .map(|c| (0..rows).rev().map(|r| g[r][c]).collect())
            .collect(),
        "rotate180" => g
            .iter()
            .rev()
            .map(|r| r.iter().rev().copied().collect())
            .collect(),
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

/// A canonical claim pair for a solve program, or None when this
/// membrane cannot peel it yet (compositions, tiles — the C6 pass).
pub fn claim_for(program: &str) -> Option<(String, Grid, Grid)> {
    let program = program.trim();
    let base: Grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let hues: Grid = vec![vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9]];
    let inner = program
        .strip_prefix("per_layer(")
        .and_then(|s| s.strip_suffix(')'))
        .unwrap_or(program);
    if D4_NAMES.contains(&inner) {
        let img = d4(inner, &base)?;
        let safe: String = program
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect();
        return Some((format!("net_{safe}"), base, img));
    }
    let perm_digits = program
        .strip_prefix("recolor[")
        .and_then(|s| s.strip_suffix(']'))?;
    let digits: Vec<u8> = perm_digits
        .split(',')
        .filter_map(|d| d.trim().parse().ok())
        .collect();
    let mut sorted = digits.clone();
    sorted.sort_unstable();
    if digits.len() != 10 || sorted != (0..10).collect::<Vec<u8>>() {
        return None;
    }
    let img: Grid = hues
        .iter()
        .map(|row| row.iter().map(|&h| digits[h as usize]).collect())
        .collect();
    let tag: String = digits.iter().map(|d| d.to_string()).collect();
    Some((format!("net_recolor_{tag}"), hues, img))
}

/// The resident mouth: send-only from the engine's side.
pub struct Membrane {
    tx: mpsc::Sender<(String, String)>,
}

impl Membrane {
    pub fn spawn() -> Membrane {
        let (tx, rx) = mpsc::channel::<(String, String)>();
        std::thread::Builder::new()
            .name("mycelium-membrane".into())
            .spawn(move || run(rx))
            .expect("membrane thread spawns");
        Membrane { tx }
    }

    /// Non-blocking: the world beat never waits on the wire. A dead
    /// membrane thread means the mesh is unreachable — the solve
    /// stands in the kernel's own record either way, and backfill
    /// remains the lawful migration verb.
    pub fn offer(&self, task: &str, program: &str) {
        let _ = self.tx.send((task.to_string(), program.to_string()));
    }
}

fn attach(addr: &str, occ: &mycelium::iae::peel::FullMeasurement) -> Option<Session> {
    let inv = Word::new(vec![LetterId::IDENTITY]);
    match Session::connect(addr, KernelRef::of(occ, None), occ, &inv, None) {
        Ok(s) => {
            eprintln!("membrane: kernel '{KERNEL}' seated at {addr}");
            Some(s)
        }
        Err(_) => None,
    }
}

fn run(rx: mpsc::Receiver<(String, String)>) {
    let addr = std::env::var(ADDR_ENV).unwrap_or_else(|_| ADDR_DEFAULT.to_string());
    let mut session: Option<Session> = None;
    let mut basis: Vec<CrystalId> = Vec::new();
    let mut pending: VecDeque<(String, String)> = VecDeque::new();
    let mut tag: u16 = 0;
    loop {
        // Drain what arrived; block only when there is nothing to do.
        let disconnected = if pending.is_empty() {
            match rx.recv() {
                Ok(item) => {
                    pending.push_back(item);
                    false
                }
                Err(_) => true,
            }
        } else {
            match rx.recv_timeout(std::time::Duration::from_secs(BEAT_SECS)) {
                Ok(item) => {
                    pending.push_back(item);
                    false
                }
                Err(mpsc::RecvTimeoutError::Timeout) => false,
                Err(mpsc::RecvTimeoutError::Disconnected) => true,
            }
        };
        if disconnected {
            if pending.is_empty() {
                return; // sender gone, nothing owed
            }
            // Sender gone but offers still owed: pace the retries.
            std::thread::sleep(std::time::Duration::from_secs(BEAT_SECS));
        }
        while let Ok(more) = rx.try_recv() {
            pending.push_back(more);
        }
        let mut still: VecDeque<(String, String)> = VecDeque::new();
        while let Some((task, program)) = pending.pop_front() {
            let Some((_claim, input, output)) = claim_for(&program) else {
                // Not peelable yet (composition/tile) — named, dropped.
                eprintln!("membrane: {task} program '{program}' waits on the C6 pass");
                continue;
            };
            let pair = match (measurement(&input), measurement(&output)) {
                (Some(a), Some(b)) => vec![(a, b)],
                _ => continue,
            };
            if session.is_none() {
                session = attach(&addr, &pair[0].0);
            }
            let Some(s) = session.as_mut() else {
                still.push_back((task, program)); // mesh unreachable: keep owing
                continue;
            };
            tag = tag.wrapping_add(1);
            match six_step(s, KERNEL, &pair, tag, &mut basis) {
                Ok(Arm::ExactInstall) => {}
                Ok(_) => {} // withheld/unglueable: the mesh's honest verdict
                Err(_) => {
                    // Socket likely gone: the seat outlives the socket.
                    session = None;
                    still.push_back((task, program));
                }
            }
        }
        pending = still;
    }
}

static MOUTH: OnceLock<Membrane> = OnceLock::new();

/// The one-line hook for the solve path: lazily spawns the resident
/// membrane on first use. `record_solve` writes the kernel's own
/// paper; this speaks the same solve to the mesh — two pens, one
/// moment, neither waiting on the other.
pub fn offer(task: &str, program: &str) {
    MOUTH.get_or_init(Membrane::spawn).offer(task, program);
}
