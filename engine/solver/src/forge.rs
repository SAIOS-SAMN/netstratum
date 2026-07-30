//! N-FORGE — the puzzle forge (the Infinite ARC).
//!
//! Beacon-seeded task generation: the previous receipt digest
//! (CHARTER §9 — randomness is a reading of history) deterministically
//! forges a fresh transformation task. Same digest, same difficulty →
//! byte-identical task on every machine; a different chronicle forges
//! a different task — the stream is endless and non-precomputable.
//!
//! THE BOUNDARY LAW (ruled 2026-07-25): the forge allocates REWARDS
//! and paces difficulty; it is NEVER a security primitive. Security
//! stays on real primitives (BLAKE3 chains, replayable validity,
//! NS-7 anchoring). No confidentiality claim is made or implied by a
//! transformation word — our own `coboundary_reduce` strips frame
//! distortion in polynomial time, and any "spatial cipher" collapses
//! the same way.

use crate::grid::{run, Grid, Op, Program, TileMirror};
use crate::loader::{Task, TaskPair};
use physics::Beacon;

/// One rung of the hardness ladder. Tiers name the vocabulary that may
/// appear in the secret word; higher levels lengthen the word and
/// widen the grid. (Volumetric/hypergraph tiers are named on the board
/// and land with their carriers — the ladder today is planar, honest.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Difficulty {
    /// 1 = symmetries · 2 = +recolor · 3 = +symmetry-quilt tiles.
    pub tier: u8,
    /// The secret word's length (also the fair search depth).
    pub word_len: usize,
    /// Base grid rows/cols (tiles multiply beyond this).
    pub rows: usize,
    pub cols: usize,
    /// Training pairs forged per task.
    pub train_pairs: usize,
}

impl Difficulty {
    /// The static ladder — network-rate adjustment lands with the mesh
    /// (N6); until then levels are explicit and measured.
    pub fn level(level: u32) -> Difficulty {
        match level {
            0 | 1 => Difficulty { tier: 1, word_len: 1, rows: 3, cols: 3, train_pairs: 2 },
            2 => Difficulty { tier: 1, word_len: 2, rows: 4, cols: 4, train_pairs: 2 },
            3 => Difficulty { tier: 2, word_len: 2, rows: 4, cols: 5, train_pairs: 3 },
            4 => Difficulty { tier: 3, word_len: 2, rows: 3, cols: 3, train_pairs: 3 },
            _ => Difficulty { tier: 3, word_len: 3, rows: 4, cols: 4, train_pairs: 3 },
        }
    }
}

impl Difficulty {
    /// FORGE-2a (14_CREEDS §4): the difficulty VECTOR derived from the
    /// line's discrete moduli — open gauge planes, crystallized
    /// doctrine, the crossing. Exact integers in, one deterministic
    /// rung out; no floats, no curvature, no stored moduli (the graded
    /// key IS the moduli, receipted at every shift). A READ only: the
    /// static ladder still drives the live forge until the wiring
    /// lands (network-rate adjustment at N6, the ladder's own law).
    /// Every coefficient is a placeholder parameter.
    pub fn from_key(planes: usize, doctrine: usize, crossed: bool) -> Difficulty {
        let tier = 1 + (planes.saturating_sub(1)).min(2) as u8;
        let word_len = (1 + doctrine / 2 + usize::from(crossed)).min(4);
        let span = (3 + planes.saturating_sub(1)).min(6);
        Difficulty {
            tier,
            word_len,
            rows: span,
            cols: span,
            train_pairs: 2 + usize::from(doctrine >= 3),
        }
    }
}

/// Sample one op for position `slot` of the secret word.
fn sample_op(beacon: &Beacon, difficulty: &Difficulty, slot: usize) -> Op {
    let symmetries = [
        Op::Rotate90,
        Op::Rotate180,
        Op::Rotate270,
        Op::FlipH,
        Op::FlipV,
        Op::Transpose,
    ];
    // Family choice widens with the tier.
    let families = match difficulty.tier {
        1 => 1,
        2 => 2,
        _ => 3,
    };
    let family = beacon.uniform_index(&format!("forge.family.{slot}"), families);
    match family {
        0 => symmetries
            [beacon.uniform_index(&format!("forge.sym.{slot}"), 6) as usize]
            .clone(),
        1 => {
            // A beacon-seeded hue permutation (Fisher–Yates, exact).
            let mut map = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            for i in (1..10usize).rev() {
                let j =
                    beacon.uniform_index(&format!("forge.recolor.{slot}.{i}"), (i + 1) as u64);
                map.swap(i, j as usize);
            }
            Op::Recolor(map)
        }
        _ => {
            let down = 1 + beacon.uniform_index(&format!("forge.tile.d.{slot}"), 3) as usize;
            let across = if down == 1 {
                2 + beacon.uniform_index(&format!("forge.tile.a.{slot}"), 2) as usize
            } else {
                1 + beacon.uniform_index(&format!("forge.tile.a.{slot}"), 3) as usize
            };
            let mirrors = TileMirror::ALL;
            Op::Tile {
                down,
                across,
                odd_row: mirrors
                    [beacon.uniform_index(&format!("forge.tile.r.{slot}"), 4) as usize],
                odd_col: mirrors
                    [beacon.uniform_index(&format!("forge.tile.c.{slot}"), 4) as usize],
            }
        }
    }
}

/// Sample one input grid (hues 0–9, dimensions from the difficulty).
fn sample_grid(beacon: &Beacon, difficulty: &Difficulty, pair: usize) -> Grid {
    let rows = difficulty.rows;
    let cols = difficulty.cols;
    let mut cells = Vec::with_capacity(rows * cols);
    for i in 0..rows * cols {
        cells.push(beacon.uniform_index(&format!("forge.cell.{pair}.{i}"), 10) as u8);
    }
    Grid::new(rows, cols, cells).expect("forged dimensions are consistent")
}

/// Forge one task from a chronicle digest. The secret word is sampled,
/// applied to freshly sampled inputs, then DISCARDED — the task
/// carries only observations; finding A word that closes them is the
/// solver's work (it need not be the forged one; the verifier is the
/// authority either way).
pub fn forge_task(digest: [u8; 32], difficulty: Difficulty) -> Task {
    let beacon = Beacon::from_digest(digest);
    let word: Program = (0..difficulty.word_len)
        .map(|slot| sample_op(&beacon, &difficulty, slot))
        .collect();

    let render = |g: &Grid| run(&word, g);
    let train: Vec<TaskPair> = (0..difficulty.train_pairs)
        .map(|pair| {
            let input = sample_grid(&beacon, &difficulty, pair);
            let output = render(&input);
            TaskPair { input, output }
        })
        .collect();
    let test_in = sample_grid(&beacon, &difficulty, difficulty.train_pairs);
    let test_expected = Some(render(&test_in));

    let mut name = String::from("forge_");
    for byte in &digest[..6] {
        name.push_str(&format!("{byte:02x}"));
    }
    Task {
        name: format!("{name}_t{}w{}", difficulty.tier, difficulty.word_len),
        train,
        test_in,
        test_expected,
    }
}
