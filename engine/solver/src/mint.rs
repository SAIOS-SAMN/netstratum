//! **THE NOVELTY GATE** (38_COGNITION_CONVERGENCE §4, task #13) — the
//! knocker for the vocabulary's mint door.
//!
//! A candidate WORD is a composition of existing total primitives — a
//! macro letter. Minting one changes REACHABILITY, never capability
//! class (the nursery's closed-algebra law): the word expands to its
//! constituent ops in every certificate, so the judge never learns a
//! new symbol and the tag-86 wire never moves.
//!
//! **THE ORBIT IS AN ACTION FINGERPRINT**, not an RCF class. The RCF
//! quotients out frames — and a word IS a frame change, so the RCF
//! identifies away exactly what a word is (charter §4 amended at this
//! landing). Two words are the same letter iff they act identically on
//! the canonical probe family: `rot90∘rot90` hashes with `rot180`,
//! and a re-spelling can never re-mint.
//!
//! The gates, in order (none is a tunable):
//!
//! ```text
//! 1. UNIQUE      orbit ∉ library ∪ orbits(standing letters)
//! 2. PRODUCTIVE  the word CLOSED a task at depth ≥ 2 — the standing
//!                letters, as letters, could not (a depth-1 sweep is
//!                a strict subset of every deeper sweep)
//! 3. CONSERVED   the verifier re-derives the expansion against every
//!                training pair (existing law; nothing here waives it)
//! 4. CONFIRMED   three DISTINCT tasks closed by the same orbit — the
//!                rung ladder's own promote count, applied to words.
//!                (Cross-LINE convergence is #14's crystallization.)
//! ```

use solver_verify::grid::{Grid, Op, TileMirror};
use solver_verify::loader::Task;

/// One minted word: its action orbit, its expansion (the letters it
/// spells to — what every certificate carries), and the tasks that
/// confirmed it (digests, in confirmation order).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MintedWord {
    pub orbit: [u8; 32],
    pub expansion: Vec<Op>,
    pub cited: Vec<[u8; 32]>,
}

/// A line's own minted library — the fold of its `SolverWordMinted`
/// receipts. Never persisted here: the chronicle is the register and
/// the conductor re-folds it (the vocabulary fold's own law).
///
/// WORDS COMPETE (docs/60 D6): each word carries the beat it last closed a
/// task (or was adopted); [`Library::forget_unused`] lets a word unused for
/// a span give way. Only the working library forgets: the chronicle keeps
/// every word, and a process re-fold restores them all.
#[derive(Debug, Clone, Default)]
pub struct Library {
    words: Vec<MintedWord>,
    /// The beat each word last closed a task, parallel to `words`.
    used: Vec<u64>,
    /// The owner's clock: what an adoption or a use is stamped with.
    clock: u64,
    /// Bumped on every adoption and every forgetting: the fingerprint a
    /// cache keyed on this library reads.
    generation: u64,
}

impl Library {
    pub fn new() -> Library {
        Library::default()
    }
    pub fn holds(&self, orbit: &[u8; 32]) -> bool {
        self.words.iter().any(|w| &w.orbit == orbit)
    }
    pub fn adopt(&mut self, word: MintedWord) {
        if !self.holds(&word.orbit) {
            self.words.push(word);
            self.used.push(self.clock);
            self.generation += 1;
        }
    }
    pub fn words(&self) -> &[MintedWord] {
        &self.words
    }
    /// Set the owner's clock (the beat).
    pub fn set_clock(&mut self, beat: u64) {
        self.clock = beat;
    }
    /// The word spelling `expansion` closed a task now.
    pub fn touch(&mut self, expansion: &[Op]) {
        if let Some(at) = self.words.iter().position(|w| w.expansion == expansion) {
            self.used[at] = self.clock;
        }
    }
    /// Let every word unused for `span` beats give way; how many did.
    pub fn forget_unused(&mut self, span: u64) -> usize {
        let clock = self.clock;
        let before = self.words.len();
        let mut used = std::mem::take(&mut self.used).into_iter();
        let mut kept_used = Vec::with_capacity(before);
        self.words.retain(|_| {
            let at = used.next().unwrap_or(clock);
            let keep = clock.saturating_sub(at) < span;
            if keep {
                kept_used.push(at);
            }
            keep
        });
        self.used = kept_used;
        let gone = before - self.words.len();
        if gone > 0 {
            self.generation += 1;
        }
        gone
    }
    /// The fingerprint of the library's contents.
    pub fn generation(&self) -> u64 {
        self.generation
    }
}

/// **THE CANONICAL PROBE FAMILY** — the fixed basis every word's
/// action is read against. Asymmetric shapes (so every symmetry acts
/// distinguishably), the whole hue ladder (so every recolor acts), and
/// separable objects on a background (so every object op acts). These
/// are canonical basis DATA, the polities.rs class — definition, not
/// tuning; changing them re-keys every orbit and is a reviewed act.
fn probes() -> Vec<Grid> {
    let ramp = |rows: usize, cols: usize| -> Grid {
        let cells = (0..rows * cols).map(|k| (k % 10) as u8).collect();
        Grid::new(rows, cols, cells).expect("canonical ramp probe")
    };
    // Two objects (hue 3 and hue 5 blocks) on a zero background —
    // different sizes so Keep/Delete/Crop-largest all distinguish.
    let objects = Grid::new(
        4,
        5,
        vec![
            3, 3, 0, 0, 0, //
            3, 3, 0, 5, 0, //
            0, 0, 0, 5, 0, //
            0, 0, 0, 0, 0,
        ],
    )
    .expect("canonical object probe");
    vec![ramp(2, 3), ramp(3, 3), ramp(3, 4), objects]
}

/// The word's ACTION ORBIT: blake3 over its outputs on the canonical
/// probe family. Total — an op that cannot act on a probe (dimension
/// or content mismatch) contributes the untouched probe, so partial
/// domains still fingerprint deterministically.
pub fn word_orbit(word: &[Op]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"netstratum.solver.word-orbit.v1");
    for probe in probes() {
        let mut grid = probe;
        for op in word {
            grid = op.apply(&grid);
        }
        hasher.update(&(grid.rows as u32).to_le_bytes());
        hasher.update(&(grid.cols as u32).to_le_bytes());
        hasher.update(&grid.cells);
    }
    *hasher.finalize().as_bytes()
}

/// The standing letters' orbits — the single-op fingerprints of a
/// task's own sweep vocabulary. A macro whose action equals ANY single
/// letter is a re-spelling, not a word.
pub fn standing_orbits(vocabulary: &[Op]) -> Vec<[u8; 32]> {
    vocabulary
        .iter()
        .map(|op| word_orbit(std::slice::from_ref(op)))
        .collect()
}

/// GATES 1 + 2 on one candidate: a program that closed `cited` at
/// depth ≥ 2, against this line's library and the standing letters.
/// Returns the candidate's orbit if it may seek confirmation; `None`
/// names no fault — most compositions are re-spellings, lawfully.
pub fn consider(
    word: &[Op],
    library: &Library,
    standing: &[[u8; 32]],
) -> Option<[u8; 32]> {
    if word.len() < 2 {
        return None; // a single letter is already in the alphabet
    }
    let orbit = word_orbit(word);
    if library.holds(&orbit) || standing.contains(&orbit) {
        return None; // gate 1 — a re-spelling never re-mints
    }
    Some(orbit)
}

/// GATE 4's count: the rung ladder's own promote number, derived from
/// the same law (three confirms promote), never a fresh dial.
pub const CONFIRMS_TO_MINT: usize = 3;

/// **THE FAMILY CENSUS** (38 §6, task #15) — the line's own families
/// ordered THINNEST-FIRST by how many minted words touch each: the
/// measured frontier a novelty-starved mind reaches toward. A word
/// counts toward every family its expansion spells (a composite is a
/// citizen of each). Ties keep the declaration order — deterministic,
/// no dial.
pub fn family_census(library: &Library) -> Vec<crate::need::Family> {
    use crate::need::{family_of, Family};
    let families = [Family::Symmetry, Family::Recolor, Family::Tile, Family::Object];
    let mut counts: Vec<(Family, usize)> = families
        .iter()
        .map(|family| {
            let count = library
                .words()
                .iter()
                .filter(|word| word.expansion.iter().any(|op| family_of(op) == *family))
                .count();
            (*family, count)
        })
        .collect();
    counts.sort_by_key(|(_, count)| *count);
    counts.into_iter().map(|(family, _)| family).collect()
}

/// **THE PRE-REACH** — the library read before the sweep: each minted
/// word tried whole, as a depth-1 letter. A hit returns the EXPANSION
/// (the certificate's form); the judge sees only letters it already
/// knows. The sweep runs unchanged behind it, so nothing previously
/// reachable is lost and the enumeration order of the old search is
/// untouched (same first match for everything the library misses).
pub fn pre_reach(
    task: &solver_verify::loader::Task,
    library: &Library,
) -> Option<Vec<Op>> {
    if task.train.is_empty() {
        return None; // the vacuous-solve refusal, upheld here too
    }
    for word in library.words() {
        let closes = task.train.iter().all(|pair| {
            let mut grid = pair.input.clone();
            for op in &word.expansion {
                grid = op.apply(&grid);
            }
            grid == pair.output
        });
        if closes {
            return Some(word.expansion.clone());
        }
    }
    None
}

// ─────────────────────────────────────────────────────────────────────
// THE STANDING VOCABULARY — op-DERIVATION off the observation, the input to
// the novelty gate above. Moved here from the deleted `search` module
// (2026-08-19): these are bounded derivations (peel the task's own ops), never
// the retired enumerate-and-test search. `sweep_vocabulary` lists the ops a
// task's observation NAMES; `standing_orbits` fingerprints them so a minted
// word that is merely a standing op is refused as non-novel.
// ─────────────────────────────────────────────────────────────────────

/// Derive candidate recolor maps from the training pairs: under each geometric
/// normalisation prefix (identity + the six symmetries), if every pair reads
/// cell-wise consistent between `prefix(input)` and `output`, the observed hue
/// mapping is a candidate op. Observed content, never a rule table.
fn derived_recolors(task: &Task) -> Vec<Op> {
    let prefixes: [Option<Op>; 7] = [
        None,
        Some(Op::Rotate90),
        Some(Op::Rotate180),
        Some(Op::Rotate270),
        Some(Op::FlipH),
        Some(Op::FlipV),
        Some(Op::Transpose),
    ];
    let mut found: Vec<Op> = Vec::new();
    'prefix: for prefix in &prefixes {
        let mut map = [255u8; 10];
        for pair in &task.train {
            let seen = match prefix {
                None => pair.input.clone(),
                Some(op) => op.apply(&pair.input),
            };
            if seen.rows != pair.output.rows || seen.cols != pair.output.cols {
                continue 'prefix;
            }
            for (i, &from) in seen.cells.iter().enumerate() {
                let to = pair.output.cells[i];
                match map[from as usize] {
                    255 => map[from as usize] = to,
                    already if already == to => {}
                    _ => continue 'prefix,
                }
            }
        }
        for (h, slot) in map.iter_mut().enumerate() {
            if *slot == 255 {
                *slot = h as u8;
            }
        }
        let op = Op::Recolor(map);
        if !found.contains(&op) {
            found.push(op);
        }
    }
    found
}

/// Derive candidate tilings from the shape ratio, under the same seven prefixes.
/// A ratio must hold on EVERY pair (one pair agreeing is a coincidence). The
/// 4×4 `TileMirror` grid is enumerated to list the STANDING tile vocabulary
/// (bounded ≤112 ops, grid-independent — vocabulary construction, not a
/// recognition sweep; the tile SEAT reads the specific mirror off the carrier).
fn derived_tiles(task: &Task) -> Vec<Op> {
    let prefixes: [Option<Op>; 7] = [
        None,
        Some(Op::Rotate90),
        Some(Op::Rotate180),
        Some(Op::Rotate270),
        Some(Op::FlipH),
        Some(Op::FlipV),
        Some(Op::Transpose),
    ];
    let mut ops: Vec<Op> = Vec::new();
    for prefix in &prefixes {
        let mut ratio: Option<(usize, usize)> = None;
        let mut holds = true;
        for pair in &task.train {
            let seen = match prefix {
                None => pair.input.clone(),
                Some(op) => op.apply(&pair.input),
            };
            let (ir, ic) = (seen.rows, seen.cols);
            let (or, oc) = (pair.output.rows, pair.output.cols);
            if ir == 0 || ic == 0 || or % ir != 0 || oc % ic != 0 {
                holds = false;
                break;
            }
            let here = (or / ir, oc / ic);
            match ratio {
                None => ratio = Some(here),
                Some(held) if held == here => {}
                Some(_) => {
                    holds = false;
                    break;
                }
            }
        }
        let Some((down, across)) = ratio.filter(|_| holds) else { continue };
        if down * across <= 1 || down > 6 || across > 6 {
            continue;
        }
        for odd_row in TileMirror::ALL {
            for odd_col in TileMirror::ALL {
                let op = Op::Tile { down, across, odd_row, odd_col };
                if !ops.contains(&op) {
                    ops.push(op);
                }
            }
        }
    }
    ops
}

/// **THE STANDING VOCABULARY** — the ops a task's observation names: derived
/// recolors and tilings first (what the observation itself spells), then the
/// fixed symmetry alphabet and object ops. Feeds `standing_orbits` (the novelty
/// gate's "is this merely a standing op?" check). Bounded derivation, no search.
pub fn sweep_vocabulary(task: &Task) -> Vec<Op> {
    let mut vocabulary: Vec<Op> = Vec::new();
    vocabulary.extend(derived_recolors(task));
    vocabulary.extend(derived_tiles(task));
    vocabulary.extend([
        Op::Rotate90,
        Op::Rotate180,
        Op::Rotate270,
        Op::FlipH,
        Op::FlipV,
        Op::Transpose,
    ]);
    vocabulary.extend(solver_verify::objects::OBJ_OPS.iter().cloned().map(Op::Object));
    vocabulary
}
