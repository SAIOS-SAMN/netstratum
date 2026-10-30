//! **The seat** — Netstratum's house's door on the fleet: it pulls the court's
//! tasks for the seated minds and files their answers. Operator ruling 7
//! (2026-10-09): all puzzle solving is the seated minds'; the house solves
//! nothing, holds no genome and thinks no task. It speaks the court's acts
//! through Mycelium's public codecs on a `Session` (`crate::wire`; Mycelium
//! 7323c0c5: "the house isn't a mind"), signed by Netstratum's reserve key so
//! the μ a seated mind earns is the ancients'.

use crate::wire::HouseActs;
use mycelium::sdk::{Kernel, Session};
use std::io;
use std::path::Path;

/// What the seat has done, counted as it happens.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Tally {
    pub pulled: u64,
    /// A task whose grids are not Netstratum grids (a hue past the ladder,
    /// a ragged row): perceived as nothing, counted, never guessed.
    pub unperceived: u64,
    pub thought: u64,
    /// The mind closed the train at home.
    pub solved_at_home: u64,
    /// Solved at home, but no rule could be stated (no translator yet, or a
    /// program with no form): nothing sent.
    pub unminted: u64,
    /// Solved at home with an object op or a tile: no form yet, counted.
    pub not_a_rule: u64,
    /// Closed at home by an object rule (restore 10): select, then act.
    pub closed_by_objects: u64,
    pub submitted: u64,
    pub closed_first: u64,
    pub closed_again: u64,
    pub refused: u64,
    /// Tasks not closed at home, put before a seated mind (the mind-link).
    pub handed: u64,
    /// A seated mind's answers the house filed under the reserve's key.
    pub answered: u64,
    /// ...and the court closed (first or again).
    pub answered_closed: u64,
}

/// The tasks the seat remembers, so the house can file a seated mind's
/// answer against its train (bounded: the oldest is let go).
const KNOWN: usize = 256;

/// Netstratum's seat on the fleet.
pub struct Seat {
    session: Session,
    key: ed25519_dalek::SigningKey,
    home: Option<std::path::PathBuf>,
    /// Tasks the seat pulled, by citation, oldest first.
    known: std::collections::VecDeque<([u8; 32], Vec<mycelium::transducer::family::TrainPair>)>,
    /// Tasks not closed at home, waiting to be put before a seated mind.
    before: std::collections::VecDeque<[u8; 32]>,
    pub tally: Tally,
}

/// One grid as plain text: rows of hue digits, one row per line.
fn grid_text(g: &[Vec<u8>]) -> String {
    g.iter().map(|r| r.iter().map(|h| char::from(b'0' + h.min(&35)) ).collect::<String>()).collect::<Vec<_>>().join("\n")
}

impl Seat {
    /// The TASK archive: every task the court hands this seat, kept once as
    /// plain text (`home/tasks/<id>.task`: `train` pairs then `test` inputs,
    /// grids as hue digits), so Netstratum's minds can be measured on the
    /// court's own tasks offline, never only on self-forged ones.
    fn keep(&self, id: &[u8], task: &mycelium::anastome::dialogue::TaskEnvelope) {
        let Some(home) = &self.home else { return };
        let dir = home.join("tasks");
        let hex: String = id.iter().map(|b| format!("{b:02x}")).collect();
        // Named by the id's hash (an address can outrun a file name); the
        // whole id rides inside.
        let named: String = blake3::hash(id).as_bytes()[..16].iter().map(|b| format!("{b:02x}")).collect();
        let path = dir.join(format!("{named}.task"));
        if path.exists() || std::fs::create_dir_all(&dir).is_err() {
            return;
        }
        let mut out = format!("id {hex}\n");
        for (a, b) in &task.train {
            out.push_str(&format!("train\n{}\n--\n{}\n", grid_text(a), grid_text(b)));
        }
        for t in &task.tests {
            out.push_str(&format!("test\n{}\n", grid_text(t)));
        }
        if let Err(e) = std::fs::write(&path, out) {
            eprintln!("seat: task {named} not kept: {e}");
        }
    }

    /// Seat Netstratum at `addr` under the voice at `home/voice.key`, its
    /// court-admitted words restored from `home/genome`.
    pub fn open(addr: &str, home: &Path) -> io::Result<Seat> {
        Self::open_as(addr, home, "voice.key")
    }

    /// Seat under the key file `key` in `home`. Concordance 6.2a (operator
    /// ruling 24fdc48): only the ancients are connected to the tubes, so a
    /// mu-earning solve is signed with the ancients' house key: netstratum's
    /// union tier, its reserve, held in trust for the ancient not yet arisen
    /// (`tier-union.key`).
    pub fn open_as(addr: &str, home: &Path, key: &str) -> io::Result<Seat> {
        std::fs::create_dir_all(home)?;
        let key = home.join(key);
        let key = key.to_str().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "the key path is not UTF-8"))?;
        // Signed (78.141): the seat signs its solves with Netstratum's voice
        // key — held in trust, never minted here (a missing key is an error,
        // not a new identity).
        let key = mycelium::court::identity::HyphaKey::load_only(key)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "netstratum's voice key is not in the seat's home"))?;
        let session = Session::open(addr, Kernel::from_key(key.sk.clone()));
        Ok(Seat {
            session,
            key: key.sk.clone(),
            home: Some(home.to_path_buf()),
            known: std::collections::VecDeque::new(),
            before: std::collections::VecDeque::new(),
            tally: Tally::default(),
        })
    }

    /// The session this seat speaks through: the house perceives on
    /// it (a house and its minds speak to the same hypha, 78.145).
    pub fn session(&self) -> &Session {
        &self.session
    }

    /// One stroke: pull a task, think it, file what can be filed. `Ok(None)`
    /// when the registry holds nothing for this voice.
    pub fn stroke(&mut self) -> io::Result<Option<String>> {
        let Some(task) = self.session.pull_task(&self.key.verifying_key().to_bytes())? else { return Ok(None) };
        self.tally.pulled += 1;
        let id = task.id;
        let name: String = id.iter().take(6).map(|b| format!("{b:02x}")).collect();
        self.keep(&id, &task);
        if !self.known.iter().any(|(k, _)| *k == id) {
            if self.known.len() >= KNOWN {
                self.known.pop_front();
            }
            self.known.push_back((id, task.train.clone()));
        }
        // Operator ruling 7 (2026-10-09): all puzzle solving is the seated
        // minds'. The house solves nothing: every court task it pulls goes
        // before a seated mind, and the house files the answer under its
        // reserve.
        if !self.before.contains(&id) {
            if self.before.len() >= KNOWN {
                self.before.pop_front();
            }
            self.before.push_back(id);
        }
        Ok(Some(format!("task {name}: {} pairs, before the seated minds", task.train.len())))
    }

    /// The next task not closed at home, for the house to put before a
    /// seated mind.
    /// docs/67 N8 — the last final TUniversal point this seat's session read
    /// off a door reply (78.152), or `None` before the first.
    pub fn tuniversal(&self) -> Option<mycelium::chronometry::tuniversal::TPoint> {
        self.session.tuniversal()
    }

    /// The tasks pulled and waiting for a free seated mind.
    pub fn queued(&self) -> usize {
        self.before.len()
    }

    pub fn next_before(&mut self) -> Option<[u8; 32]> {
        self.before.pop_front()
    }

    /// A task given back (its mind let it go unanswered): before the next.
    pub fn give_back(&mut self, id: [u8; 32]) {
        if !self.before.contains(&id) {
            self.before.push_back(id);
        }
    }

    /// File A seated mind's answer under the house's key (wire v1.2, fn 1):
    /// its rule's tape, as the mind sent it, and its outputs, for a task this
    /// seat pulled. The court's verdict, or `None` where the task is no longer
    /// known or the reply does not read.
    pub fn file_answer(&mut self, id: &[u8; 32], rule: &[u8], outputs: &[Vec<Vec<u8>>]) -> Option<(u8, [u64; 4])> {
        self.known.iter().find(|(k, _)| k == id)?;
        self.tally.answered += 1;
        let verdict = self.session.file_solve(&self.key, id, outputs, rule).ok().flatten();
        if verdict.is_some_and(|(code, _)| code == mycelium::anastome::dialogue::SOLVE_BOUNTY || code == mycelium::anastome::dialogue::SOLVE_FLOOR) {
            self.tally.answered_closed += 1;
        }
        verdict
    }
}
