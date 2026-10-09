//! **THE SEAT** — netstratum's ONE client kernel on the fleet (operator
//! priority 2026-10-08: traffic on the hyphae, minds in the games). It is the
//! membrane re-founded on mycelium-v0.1.0: one seated `Entity` under
//! netstratum's voice (held in trust), whose genome holds only words a court
//! admitted — the empty client kernel of COGNITION.md §1.5, filled by
//! consilience.
//!
//! Each stroke takes ONE task from the hypha's registry (`pull_task`): the
//! court decides what is useful work, never netstratum. netstratum's mind
//! thinks its train pairs with the same reduce organ and held conjectures the
//! civilizations use (`solver::think_task_held`), renders every test output,
//! and asks THE TRANSLATOR (mycelium 78.142 `sdk::rule`) for the rule. A
//! solve is filed only with its rule (`Entity::solve_task`, which crosses it
//! and replays it at home first): a raster is no proof of a motion. A
//! program with no form (object ops, tiles) is counted NotARule, never sent
//! and never dropped.

use mycelium::sdk::{Entity, Kernel};
use solver::{think_task_held, Conjecture, Crystal, SolveRead};
use std::io;
use std::path::Path;

/// What the seat has done, counted as it happens.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Tally {
    pub pulled: u64,
    /// A task whose grids are not netstratum grids (a hue past the ladder,
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

/// netstratum's seat on the fleet.
pub struct Seat {
    entity: Entity,
    home: Option<std::path::PathBuf>,
    grown: Vec<Crystal>,
    held: Vec<Conjecture>,
    /// Tasks the seat pulled, by citation, oldest first.
    known: std::collections::VecDeque<([u8; 32], Vec<mycelium::transducer::family::TrainPair>)>,
    /// Tasks not closed at home, waiting to be put before a seated mind.
    before: std::collections::VecDeque<[u8; 32]>,
    pub tally: Tally,
}

/// A fleet grid read as a netstratum grid, or `None` (ragged, empty, or a hue
/// past the ten-hue ladder).
fn grid(g: &[Vec<u8>]) -> Option<solver::Grid> {
    let rows = g.len();
    let cols = g.first()?.len();
    if g.iter().any(|r| r.len() != cols) {
        return None;
    }
    solver::Grid::new(rows, cols, g.concat())
}

fn back(g: &solver::Grid) -> Vec<Vec<u8>> {
    g.cells.chunks(g.cols.max(1)).map(|r| r.to_vec()).collect()
}

/// THE TRANSLATOR, shared with the conductor (docs/60 D10b): it lives in
/// the `tongue` crate.
pub use tongue::{translate, Untranslated};

/// One grid as plain text: rows of hue digits, one row per line.
fn grid_text(g: &[Vec<u8>]) -> String {
    g.iter().map(|r| r.iter().map(|h| char::from(b'0' + h.min(&35)) ).collect::<String>()).collect::<Vec<_>>().join("\n")
}

impl Seat {
    /// THE TASK ARCHIVE: every task the court hands this seat, kept once as
    /// plain text (`home/tasks/<id>.task`: `train` pairs then `test` inputs,
    /// grids as hue digits), so netstratum's minds can be measured on the
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

    /// Seat netstratum at `addr` under the voice at `home/voice.key`, its
    /// court-admitted words restored from `home/genome`.
    pub fn open(addr: &str, home: &Path) -> io::Result<Seat> {
        Self::open_as(addr, home, "voice.key")
    }

    /// Seat under the key file `key` in `home`. Concordance 6.2a (operator
    /// ruling 24fdc48): only the ANCIENTS are connected to the tubes, so a
    /// mu-earning solve is signed with the ancients' HOUSE key: netstratum's
    /// union tier, its reserve, held in trust for the ancient not yet arisen
    /// (`tier-union.key`).
    pub fn open_as(addr: &str, home: &Path, key: &str) -> io::Result<Seat> {
        std::fs::create_dir_all(home)?;
        let key = home.join(key);
        let key = key.to_str().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "the key path is not UTF-8"))?;
        // SIGNED (78.141): the seat signs its solves with netstratum's voice
        // key — held in trust, never minted here (a missing key is an error,
        // not a new identity).
        let key = mycelium::court::identity::HyphaKey::load_only(key)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "netstratum's voice key is not in the seat's home"))?;
        let genome = home.join("genome");
        let genome = genome.to_str().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "the genome path is not UTF-8"))?;
        let entity = Entity::open_persistent(addr, Kernel::from_key(key.sk.clone()), genome);
        Ok(Seat {
            entity,
            home: Some(home.to_path_buf()),
            grown: Vec::new(),
            held: Vec::new(),
            known: std::collections::VecDeque::new(),
            before: std::collections::VecDeque::new(),
            tally: Tally::default(),
        })
    }

    /// The session this seat speaks through: the house tends its mail on
    /// it (a house and its minds speak to the same hypha, 78.145).
    pub fn session(&self) -> &mycelium::sdk::Session {
        self.entity.relay()
    }

    /// ONE STROKE: pull a task, think it, file what can be filed. `Ok(None)`
    /// when the registry holds nothing for this voice.
    pub fn stroke(&mut self) -> io::Result<Option<String>> {
        let Some(task) = self.entity.pull_task()? else { return Ok(None) };
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
        // OPERATOR RULING 7 (2026-10-09): all puzzle solving is the seated
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

    /// The house's own think of a court task, RETIRED by ruling 7 (kept for
    /// the offline measures in `examples/`; never called by the live stroke).
    #[allow(dead_code)]
    fn think_retired(&mut self, task: mycelium::anastome::dialogue::TaskEnvelope, id: [u8; 32], name: String) -> io::Result<Option<String>> {
        let train: Option<Vec<solver::TaskPair>> =
            task.train.iter().map(|(a, b)| Some(solver::TaskPair { input: grid(a)?, output: grid(b)? })).collect();
        let tests: Option<Vec<solver::Grid>> = task.tests.iter().map(|t| grid(t)).collect();
        let (Some(train), Some(tests)) = (train, tests) else {
            self.tally.unperceived += 1;
            return Ok(Some(format!("task {name}: not perceived (grids off the ladder)")));
        };
        let Some(test_in) = tests.first().cloned() else {
            self.tally.unperceived += 1;
            return Ok(Some(format!("task {name}: no test input")));
        };
        self.tally.thought += 1;
        let local = solver::Task { name: name.clone(), train, test_in, test_expected: None };
        let SolveRead::Solved { program, .. } = think_task_held(&local, &mut self.grown, &mut self.held) else {
            // RESTORE 10: perceive bodies, counts and extrema, and read the
            // selection the pairs show. A closed object rule is counted; it is
            // stated to the court once 78.143's steps land in the translator.
            let pairs: Vec<(solver::Grid, solver::Grid)> = local.train.iter().map(|p| (p.input.clone(), p.output.clone())).collect();
            // THE PROPOSER (78.143): the mind speaks the court's own steps,
            // each checked by the judge's own steps on the train pairs, the
            // look bounded in time. A closed proposal is filed natively.
            if let Some(p) = crate::propose::propose_within(&task.train, &task.tests, 3000, std::time::Duration::from_secs(20)) {
                self.tally.solved_at_home += 1;
                self.tally.closed_by_objects += u64::from(solver::percept::learn(&pairs).is_some());
                let mut budget = self.entity.physics_mut().stroke();
                self.tally.submitted += 1;
                let verdict = self.entity.solve_task(&id, &task.train, &p.outputs, Some(&p.rule), &mut budget);
                return Ok(Some(self.verdict_line(&name, verdict, &format!("{:?}", p.steps))));
            }
            // THE MIND-LINK: a task this mind did not close goes before a
            // seated mind (the line's sibling vessel), once.
            if !self.before.contains(&id) {
                if self.before.len() >= KNOWN {
                    self.before.pop_front();
                }
                self.before.push_back(id);
            }
            return Ok(Some(format!("task {name}: {} pairs, not closed at home; before the seated minds", local.train.len())));
        };
        self.tally.solved_at_home += 1;
        let outputs: Vec<Vec<Vec<u8>>> = tests.iter().map(|t| back(&solver::grid::run(&program, t))).collect();
        let rule = match translate(&program, &task.train, &self.entity.epigenome) {
            Ok(rule) => rule,
            Err(Untranslated::NotARule) => {
                self.tally.not_a_rule += 1;
                return Ok(Some(format!("task {name}: closed at home ({} ops), NotARule (an object op or tile): not sent", program.len())));
            }
            Err(why) => {
                self.tally.unminted += 1;
                return Ok(Some(format!("task {name}: closed at home, the seam refused the rule: {why:?}")));
            }
        };
        let mut budget = self.entity.physics_mut().stroke();
        self.tally.submitted += 1;
        let verdict = self.entity.solve_task(&id, &task.train, &outputs, Some(&rule), &mut budget);
        Ok(Some(self.verdict_line(&name, verdict, &format!("{} ops", program.len()))))
    }

    /// The next task not closed at home, for the house to put before a
    /// seated mind.
    /// docs/67 N8 — the last FINAL TUniversal point this seat's session read
    /// off a door reply (78.152), or `None` before the first.
    pub fn tuniversal(&self) -> Option<mycelium::chronometry::tuniversal::TPoint> {
        self.entity.tuniversal()
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

    /// FILE A SEATED MIND'S ANSWER under the house's key (wire v1.2, fn 1):
    /// its rule and outputs for a task this seat pulled, crossed and
    /// replayed at home exactly as the seat's own (`Entity::solve_task`).
    /// The court's verdict, or `None` where the task is no longer known or
    /// the answer was withheld at home.
    pub fn file_answer(&mut self, id: &[u8; 32], rule: &mycelium::transducer::element::Element, outputs: &[Vec<Vec<u8>>]) -> Option<(u8, [u64; 4])> {
        let train = self.known.iter().find(|(k, _)| k == id)?.1.clone();
        let mut budget = self.entity.physics_mut().stroke();
        self.tally.answered += 1;
        let verdict = self.entity.solve_task(id, &train, outputs, Some(rule), &mut budget);
        if verdict.is_some_and(|(code, _)| code == mycelium::anastome::dialogue::SOLVE_BOUNTY || code == mycelium::anastome::dialogue::SOLVE_FLOOR) {
            self.tally.answered_closed += 1;
            self.save_genome();
        }
        verdict
    }

    /// THE HOUSE REMEMBERS (docs/60 D0): the genome — the words a court
    /// admitted — written to `home/genome` after a closure, which
    /// `open_persistent` restores at the next start.
    fn save_genome(&mut self) {
        let Some(path) = self.home.as_ref().map(|h| h.join("genome")) else { return };
        let Some(path) = path.to_str().map(str::to_string) else { return };
        if let Err(e) = self.entity.save_genome(&path) {
            eprintln!("seat: the genome was not saved: {e}");
        }
    }

    /// The court's verdict on a filed solve, counted and told.
    fn verdict_line(&mut self, name: &str, verdict: Option<(u8, [u64; 4])>, what: &str) -> String {
        match verdict {
            Some((code, mu)) if code == mycelium::anastome::dialogue::SOLVE_BOUNTY => {
                self.tally.closed_first += 1;
                self.save_genome();
                format!("task {name}: CLOSED FIRST ({what}), paid {mu:?}")
            }
            Some((code, _)) if code == mycelium::anastome::dialogue::SOLVE_FLOOR => {
                self.tally.closed_again += 1;
                self.save_genome();
                format!("task {name}: closed again ({what})")
            }
            Some(_) => {
                self.tally.refused += 1;
                format!("task {name}: refused by the court ({what})")
            }
            None => format!("task {name}: withheld at home ({what}: refuted, or refused before)"),
        }
    }
}
