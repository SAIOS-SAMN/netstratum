//! N-VOLUME — the volumetric carrier: puzzles in three dimensions.
//!
//! A `Volume` is a stack of congruent grids (layers). The vocabulary
//! stays GENERAL (no task knowledge): planar symmetries applied per
//! layer, plus two stack operators (reverse, cycle). The forge tier
//! "volumetric" named on the hardness ladder lands here — and the
//! PERCEPTION AXIS gains teeth: a volumetric civilization natively
//! attempts these tasks (the conductor routes by perception).
//!
//! ARC corpora are planar, so volumetric tasks are forge-only: the
//! external 27/960 scoreboard is untouched by anything here.

use solver_verify::certificate::hash_grid_into;
use solver_verify::grid::{Grid, Op};
use physics::Beacon;

/// A stack of congruent layers, bottom to top.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Volume {
    layers: Vec<Grid>,
}

impl Volume {
    /// All layers must share dimensions; at least one layer.
    pub fn new(layers: Vec<Grid>) -> Option<Volume> {
        let first = layers.first()?;
        let (rows, cols) = (first.rows, first.cols);
        layers
            .iter()
            .all(|l| l.rows == rows && l.cols == cols)
            .then_some(Volume { layers })
    }

    pub fn layers(&self) -> &[Grid] {
        &self.layers
    }
}

/// The volumetric vocabulary — general operators only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VOp {
    /// One planar op applied to EVERY layer identically.
    PerLayer(Op),
    /// Reverse the layer order (top↔bottom mirror).
    FlipStack,
    /// Cycle layers up by one (the top wraps to the bottom).
    CycleLayers,
}

impl VOp {
    pub fn describe(&self) -> String {
        match self {
            VOp::PerLayer(op) => format!("per_layer({})", op.describe()),
            VOp::FlipStack => "flip_stack".into(),
            VOp::CycleLayers => "cycle_layers".into(),
        }
    }

    pub fn apply(&self, v: &Volume) -> Volume {
        match self {
            VOp::PerLayer(op) => Volume {
                layers: v.layers.iter().map(|l| op.apply(l)).collect(),
            },
            VOp::FlipStack => Volume {
                layers: v.layers.iter().rev().cloned().collect(),
            },
            VOp::CycleLayers => {
                let mut layers = v.layers.clone();
                layers.rotate_right(1);
                Volume { layers }
            }
        }
    }
}

pub type VProgram = Vec<VOp>;

pub fn run_volume(program: &VProgram, input: &Volume) -> Volume {
    let mut v = input.clone();
    for op in program {
        v = op.apply(&v);
    }
    v
}

/// One volumetric task: training observations + the held-out test.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VolumeTask {
    pub name: String,
    pub train: Vec<(Volume, Volume)>,
    pub test_in: Volume,
    pub test_expected: Option<Volume>,
}

/// The volumetric hardness rung.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VolDifficulty {
    pub layers: usize,
    pub word_len: usize,
    pub rows: usize,
    pub cols: usize,
    pub train_pairs: usize,
}

impl VolDifficulty {
    pub fn level(level: u32) -> VolDifficulty {
        match level {
            0 | 1 => VolDifficulty { layers: 2, word_len: 1, rows: 3, cols: 3, train_pairs: 2 },
            2 => VolDifficulty { layers: 3, word_len: 2, rows: 3, cols: 3, train_pairs: 2 },
            3 => VolDifficulty { layers: 3, word_len: 2, rows: 4, cols: 4, train_pairs: 3 },
            4 => VolDifficulty { layers: 4, word_len: 3, rows: 4, cols: 4, train_pairs: 3 },
            _ => VolDifficulty { layers: 5, word_len: 3, rows: 4, cols: 5, train_pairs: 3 },
        }
    }
}

const SYMMETRIES: [Op; 6] = [
    Op::Rotate90,
    Op::Rotate180,
    Op::Rotate270,
    Op::FlipH,
    Op::FlipV,
    Op::Transpose,
];

fn sample_vop(beacon: &Beacon, slot: usize) -> VOp {
    match beacon.uniform_index(&format!("volforge.family.{slot}"), 3) {
        0 => VOp::PerLayer(
            SYMMETRIES[beacon.uniform_index(&format!("volforge.sym.{slot}"), 6) as usize].clone(),
        ),
        1 => VOp::FlipStack,
        _ => VOp::CycleLayers,
    }
}

fn sample_volume(beacon: &Beacon, d: &VolDifficulty, pair: usize) -> Volume {
    let layers = (0..d.layers)
        .map(|layer| {
            let cells = (0..d.rows * d.cols)
                .map(|i| {
                    beacon.uniform_index(&format!("volforge.cell.{pair}.{layer}.{i}"), 10) as u8
                })
                .collect();
            Grid::new(d.rows, d.cols, cells).expect("forged dimensions are consistent")
        })
        .collect();
    Volume::new(layers).expect("congruent layers")
}

/// Forge one volumetric task from a chronicle digest — same law as the
/// planar forge: the secret word renders the observations, then is
/// discarded; the verifier is the authority.
pub fn forge_volume(digest: [u8; 32], difficulty: VolDifficulty) -> VolumeTask {
    let beacon = Beacon::from_digest(digest);
    let word: VProgram = (0..difficulty.word_len)
        .map(|slot| sample_vop(&beacon, slot))
        .collect();
    let render = |v: &Volume| run_volume(&word, v);
    let train: Vec<(Volume, Volume)> = (0..difficulty.train_pairs)
        .map(|pair| {
            let input = sample_volume(&beacon, &difficulty, pair);
            let output = render(&input);
            (input, output)
        })
        .collect();
    let test_in = sample_volume(&beacon, &difficulty, difficulty.train_pairs);
    let test_expected = Some(render(&test_in));

    let mut name = String::from("volf_");
    for byte in &digest[..6] {
        name.push_str(&format!("{byte:02x}"));
    }
    VolumeTask {
        name: format!("{name}_d{}w{}", difficulty.layers, difficulty.word_len),
        train,
        test_in,
        test_expected,
    }
}

/// The volumetric solve reading.
#[derive(Debug, Clone)]
pub enum VolumeSolveRead {
    Solved {
        program: VProgram,
        test_output: Volume,
        matches_expected: Option<bool>,
        /// BLAKE3 chain digest over observation → word → output (the
        /// same certificate law as the planar solver).
        certificate: [u8; 32],
    },
    Unsolved,
}

fn hash_volume(hasher: &mut blake3::Hasher, v: &Volume) {
    hasher.update(&(v.layers.len() as u64).to_le_bytes());
    for layer in &v.layers {
        hash_grid_into(hasher, layer);
    }
}

// DECORATIVE (operator ruling C8, 2026-08-01). This seal is a bare
// digest no third party can re-derive — tamper-evidence for no one — and
// no volumetric deed can mint (tag 86 carries planar work only). It is
// NOT a verifier. Volumetric work is forge-only until higher-dimensional
// work becomes a real economic axis; on that day `verify_volume` lands
// mirroring `verify_work` (re-forge ∧ re-apply ∧ re-chain + the
// degenerate-difficulty guard) with its own deed. Until then this digest
// carries no economic weight and the empty-train solve is not a mint.
// See docs/U_DIRECTIVES.md §10 S1.
fn seal_volume(task: &VolumeTask, program: &VProgram, output: &Volume) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(task.name.as_bytes());
    for (input, expected) in &task.train {
        hash_volume(&mut hasher, input);
        hash_volume(&mut hasher, expected);
    }
    hash_volume(&mut hasher, &task.test_in);
    let mut parent = *hasher.finalize().as_bytes();
    for op in program {
        let mut link = blake3::Hasher::new();
        link.update(&parent);
        link.update(op.describe().as_bytes());
        parent = *link.finalize().as_bytes();
    }
    let mut last = blake3::Hasher::new();
    last.update(&parent);
    hash_volume(&mut last, output);
    *last.finalize().as_bytes()
}

/// Enumerative search over volumetric words: a program is a solution
/// ONLY if it closes every training pair exactly. The honest withhold
/// stands.
pub fn solve_volume(task: &VolumeTask, max_depth: usize) -> VolumeSolveRead {
    let mut vocabulary: Vec<VOp> = SYMMETRIES
        .iter()
        .map(|op| VOp::PerLayer(op.clone()))
        .collect();
    vocabulary.push(VOp::FlipStack);
    vocabulary.push(VOp::CycleLayers);

    let closes = |program: &VProgram| {
        task.train
            .iter()
            .all(|(input, expected)| &run_volume(program, input) == expected)
    };
    // FORGE-BOUND: the same deterministic budget as the planar path.
    let mut tested: usize = 0;
    let mut frontier: Vec<VProgram> = vec![Vec::new()];
    for _depth in 0..=max_depth {
        for program in &frontier {
            tested += 1;
            if tested > crate::search::SEARCH_BUDGET {
                return VolumeSolveRead::Unsolved;
            }
            if closes(program) {
                let test_output = run_volume(program, &task.test_in);
                let matches_expected =
                    task.test_expected.as_ref().map(|want| want == &test_output);
                let certificate = seal_volume(task, program, &test_output);
                return VolumeSolveRead::Solved {
                    program: program.clone(),
                    test_output,
                    matches_expected,
                    certificate,
                };
            }
        }
        let room = crate::search::SEARCH_BUDGET.saturating_sub(tested) + 1;
        let mut next = Vec::new();
        'grow: for program in &frontier {
            for op in &vocabulary {
                if next.len() >= room {
                    break 'grow;
                }
                let mut extended = program.clone();
                extended.push(op.clone());
                next.push(extended);
            }
        }
        frontier = next;
    }
    VolumeSolveRead::Unsolved
}
