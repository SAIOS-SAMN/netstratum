//! THE VOLUMETRIC VERIFIER — work-shapes v3 (operator ruling
//! 2026-08-04, lifting C8's deferral).
//!
//! A `Volume` is a stack of congruent grids. The vocabulary stays
//! GENERAL — planar symmetries applied per layer, plus two stack
//! operators — so no task knowledge exists on any path here.
//!
//! **WHAT CHANGED.** C8 (2026-08-01) ruled the volumetric tier
//! DECORATIVE: `seal_volume` was a bare digest nobody could re-derive,
//! tamper-evidence for no one, and no volumetric deed could mint. That
//! deferral is now LIFTED — volumetric work is an economic axis, so it
//! needs a real verifier, and this is it.
//!
//! **THE SPLIT, mirroring U1a.** This module holds exactly what a
//! REPLAY needs: the volume carrier, the one apply path, the
//! deterministic forge, and a parent-linked certificate. The SEARCH
//! (`solver::volume::solve_volume`) stays prover-side, free and
//! non-constitutional. Finding is expensive; checking is O(word).
//!
//! **SIMPLER THAN CORPUS v2, and for a structural reason.** A
//! volumetric task is FORGE-ONLY: it re-derives from its digest, so the
//! judge needs nothing but the digest to know the task is real. There
//! is no admissible-set problem, no Merkle root, no inclusion path —
//! all of which corpus work needs precisely because observed content
//! cannot be re-derived. Volumetric work is v1's shape in three
//! dimensions.
//!
//! **THE VACUITY IS STRUCTURALLY IMPOSSIBLE HERE, not merely guarded.**
//! `VolDifficulty::lawful` refuses a degenerate rung at DECODE, before
//! any forge or replay runs, so a payload claiming zero training pairs
//! (whose "maps every pair" would be vacuously true — a free mint) is
//! not a payload at all. The same law as `Difficulty::lawful`, and the
//! reason both exist as named predicates rather than inline conditions:
//! a guard restated at each door is a guard that a new door forgets.

use crate::certificate::hash_grid_into;
use crate::grid::{Grid, Op};
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

    pub fn depth(&self) -> usize {
        self.layers.len()
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

/// THE ONE APPLY PATH. A replay must apply every operator a program may
/// carry — the same law that kept the object ops verifier-side.
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

    /// THE DEGENERACY LAW, volumetric — the counterpart of
    /// [`crate::forge::Difficulty::lawful`], named for the same reason.
    ///
    /// A rung with no layer, no row, no column, or NO TRAINING PAIR is
    /// refused at decode, before the forge is ever called. Each field
    /// closes a distinct hole:
    /// - `train_pairs: 0` — the vacuity, and the exploitable one: "maps
    ///   EVERY training pair" is true over no pairs, so an empty program
    ///   would verify and mint for nothing;
    /// - `rows`/`cols`/`layers` of 0 — the forge's `Grid::new(..)
    ///   .expect(..)` and `Volume::new(..).expect(..)` PANIC, which is a
    ///   totality violation inside the judge (audit HOLE B's class).
    ///
    /// `word_len: 0` is LAWFUL and deliberately admitted: the empty word
    /// is the identity task, and a copy task is a task.
    pub fn lawful(&self) -> bool {
        self.layers > 0 && self.rows > 0 && self.cols > 0 && self.train_pairs > 0
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

/// The vocabulary a volumetric search draws from — exported so the
/// prover and the verifier cannot disagree about what exists.
pub fn vocabulary() -> Vec<VOp> {
    let mut vocabulary: Vec<VOp> = SYMMETRIES.iter().map(|op| VOp::PerLayer(op.clone())).collect();
    vocabulary.push(VOp::FlipStack);
    vocabulary.push(VOp::CycleLayers);
    vocabulary
}

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
                .map(|i| beacon.uniform_index(&format!("volforge.cell.{pair}.{layer}.{i}"), 10) as u8)
                .collect();
            Grid::new(d.rows, d.cols, cells).expect("forged dimensions are consistent")
        })
        .collect();
    Volume::new(layers).expect("congruent layers")
}

/// Forge one volumetric task from a chronicle digest — the same law as
/// the planar forge: the secret word renders the observations, then is
/// discarded; the verifier is the authority.
///
/// PRECONDITION: `difficulty.lawful()`. Every decode door checks it
/// first, so the `expect`s above are unreachable from a payload.
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

pub(crate) fn hash_volume(hasher: &mut blake3::Hasher, v: &Volume) {
    hasher.update(&(v.layers.len() as u64).to_le_bytes());
    for layer in &v.layers {
        hash_grid_into(hasher, layer);
    }
}

/// THE VOLUMETRIC CERTIFICATE — a real parent-linked chain, replacing
/// the bare digest C8 correctly called decorative.
///
/// Same shape as [`crate::certificate::SolveCertificate`]: the genesis
/// binds the whole observation, each step binds one transformation, the
/// final digest binds the rendered output. Tampering with any link
/// diverges every digest after it, and the divergence is NAMED.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VolumeCertificate {
    pub genesis: [u8; 32],
    pub steps: Vec<[u8; 32]>,
    pub output: [u8; 32],
}

/// The typed verification reading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VolumeCertificateRead {
    Intact,
    Diverged { at_step: usize },
}

fn link(parent: &[u8; 32], payload: impl FnOnce(&mut blake3::Hasher)) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(parent);
    payload(&mut hasher);
    *hasher.finalize().as_bytes()
}

impl VolumeCertificate {
    /// Seal a volumetric solution: observation → word → output.
    pub fn seal(task: &VolumeTask, program: &VProgram, output: &Volume) -> VolumeCertificate {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"netstratum.volume.genesis.v1");
        hasher.update(task.name.as_bytes());
        for (input, expected) in &task.train {
            hash_volume(&mut hasher, input);
            hash_volume(&mut hasher, expected);
        }
        hash_volume(&mut hasher, &task.test_in);
        let genesis = *hasher.finalize().as_bytes();

        let mut parent = genesis;
        let steps: Vec<[u8; 32]> = program
            .iter()
            .map(|op| {
                parent = link(&parent, |h| {
                    h.update(op.describe().as_bytes());
                });
                parent
            })
            .collect();
        let output_digest = link(&parent, |h| hash_volume(h, output));
        VolumeCertificate { genesis, steps, output: output_digest }
    }
}

/// Re-derive the chain and name the first diverged link.
///
/// `0` = genesis, `1..=n` = steps, `n+1` = the output link.
pub fn verify_volume_certificate(
    certificate: &VolumeCertificate,
    task: &VolumeTask,
    program: &VProgram,
    output: &Volume,
) -> VolumeCertificateRead {
    let honest = VolumeCertificate::seal(task, program, output);
    if honest.genesis != certificate.genesis {
        return VolumeCertificateRead::Diverged { at_step: 0 };
    }
    if honest.steps.len() != certificate.steps.len() {
        return VolumeCertificateRead::Diverged {
            at_step: certificate.steps.len().min(honest.steps.len()) + 1,
        };
    }
    for (n, (a, b)) in honest.steps.iter().zip(&certificate.steps).enumerate() {
        if a != b {
            return VolumeCertificateRead::Diverged { at_step: n + 1 };
        }
    }
    if honest.output != certificate.output {
        return VolumeCertificateRead::Diverged { at_step: honest.steps.len() + 1 };
    }
    VolumeCertificateRead::Intact
}
