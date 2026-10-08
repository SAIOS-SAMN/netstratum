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
//! and asks THE TRANSLATOR for the rule. A solve is filed only with its rule
//! (`Entity::solve_task`, which also replays it at home first): a raster is
//! no proof of a motion. Until mycelium 78.142's `sdk::rule` lands the
//! translator states no rule, so a solve is counted UNMINTED and nothing is
//! sent — the honest count, never a fabricated rule.

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
    pub submitted: u64,
    pub closed_first: u64,
    pub closed_again: u64,
    pub refused: u64,
}

/// netstratum's seat on the fleet.
pub struct Seat {
    entity: Entity,
    grown: Vec<Crystal>,
    held: Vec<Conjecture>,
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

/// THE TRANSLATOR: a netstratum program as a court rule. Built only through
/// mycelium's own `sdk::rule` constructors (coordinator ruling) — which land
/// in 78.142. Until then it states nothing.
fn rule_for(_program: &solver::Program, _train: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)]) -> Option<mycelium::transducer::element::Element> {
    None
}

impl Seat {
    /// Seat netstratum at `addr` under the voice at `home/voice.key`, its
    /// court-admitted words restored from `home/genome`.
    pub fn open(addr: &str, home: &Path) -> io::Result<Seat> {
        std::fs::create_dir_all(home)?;
        let key = home.join("voice.key");
        let key = key.to_str().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "the key path is not UTF-8"))?;
        let voice = mycelium::court::identity::HyphaKey::load_or_mint(key).vk_bytes();
        let genome = home.join("genome");
        let genome = genome.to_str().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "the genome path is not UTF-8"))?;
        let entity = Entity::open_persistent(addr, Kernel::from_voice(voice), genome);
        Ok(Seat { entity, grown: Vec::new(), held: Vec::new(), tally: Tally::default() })
    }

    /// ONE STROKE: pull a task, think it, file what can be filed. `Ok(None)`
    /// when the registry holds nothing for this voice.
    pub fn stroke(&mut self) -> io::Result<Option<String>> {
        let Some(task) = self.entity.pull_task()? else { return Ok(None) };
        self.tally.pulled += 1;
        let id = task.id.as_bytes().to_vec();
        let name: String = id.iter().take(6).map(|b| format!("{b:02x}")).collect();
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
            return Ok(Some(format!("task {name}: {} pairs, not closed at home", local.train.len())));
        };
        self.tally.solved_at_home += 1;
        let outputs: Vec<Vec<Vec<u8>>> = tests.iter().map(|t| back(&solver::grid::run(&program, t))).collect();
        let Some(rule) = rule_for(&program, &task.train) else {
            self.tally.unminted += 1;
            return Ok(Some(format!("task {name}: closed at home ({} ops), no rule stated yet: not sent", program.len())));
        };
        let mut budget = self.entity.physics().stroke();
        self.tally.submitted += 1;
        let verdict = self.entity.solve_task(&id, &task.train, &outputs, Some(&rule), &mut budget);
        Ok(Some(match verdict {
            Some((code, mu)) if code == mycelium::anastome::dialogue::SOLVE_BOUNTY => {
                self.tally.closed_first += 1;
                format!("task {name}: CLOSED FIRST, paid {mu:?}")
            }
            Some((code, _)) if code == mycelium::anastome::dialogue::SOLVE_FLOOR => {
                self.tally.closed_again += 1;
                format!("task {name}: closed again")
            }
            Some(_) => {
                self.tally.refused += 1;
                format!("task {name}: refused by the court")
            }
            None => format!("task {name}: withheld at home (refuted, or refused before)"),
        }))
    }
}
