//! The work proof — the opaque payload's one form and its gate
//! (U1 KERNEL-VERIFY, U_DIRECTIVES §2).
//!
//! The OPAQUE-PAYLOAD law: the `DiscoveryMinted` deed carries its
//! proof as bytes that cfe never reads and the kernel never parses —
//! this module is the encoding's monopoly (the codec-seam pattern:
//! one component owns the form). Refuse never guess: truncation,
//! trailing bytes, unknown codes, out-of-range dimensions all refuse.
//!
//! The REPLAY-COMPLETE law: the certificate chain alone binds
//! (task, program, output) — it does not prove the program solves the
//! task; an intact chain can be sealed over a wrong output. So
//! [`verify_work`] is the full replay, every leg or refuse:
//!   1. re-forge — the task re-derives from its digest (pure);
//!   2. re-apply — the program maps every training pair exactly and
//!      the test input to the claimed output (the honest-withhold
//!      law: one miss = refuse, never a guess); the forge's own
//!      `test_expected` must agree;
//!   3. re-chain — the certificate re-derives Intact.
//!
//! Schema v4 (the carried derivation, Concordance 4.4 annex) replaces the
//! replay for forge work: the payload carries the forged task's samples and
//! both words' per-step state digests, and [`verify_carried_work`] walks them
//! locally — no re-forge, no re-seal. v1-v3 stay readable as records.
//!
//! The op-code vocabulary below is wire law — append-only, one code
//! one meaning forever: 1 Rotate90 · 2 Rotate180 · 3 Rotate270 ·
//! 4 FlipH · 5 FlipV · 6 Transpose · 7 Recolor+map\[10\] ·
//! 8 Object+objop (1 KeepLargest · 2 KeepSmallest · 3 DeleteLargest ·
//! 4 CropLargest · 5 CropContent · 6 PaintAllLikeLargest ·
//! 7 PaintAllMajority · 8 GravityDown) · 9 Tile+down,across,odd_row,
//! odd_col (TileMirror: 0 None · 1 Rows · 2 Cols · 3 Both).

use crate::certificate::{
    derive, verify_certificate, walk, CertificateRead, Derivation, SolveCertificate,
};
use crate::forge::{forge_input, forge_task, forge_word, Difficulty};
use crate::grid::{run, Grid, Op, Program, TileMirror};
use crate::loader::{Task, TaskPair};
use crate::objects::ObjOp;
use crate::volume::{
    forge_volume, run_volume, verify_volume_certificate, VOp, VProgram, VolDifficulty, Volume,
    VolumeCertificate, VolumeCertificateRead,
};

/// The work-proof wire schema. A bump is an amendment.
///
/// Schema bytes are append-only — one code, one meaning forever. v1 is
/// untouched by v2: the same v1 bytes decode to the same v1 proof, and
/// a v1 payload never grows a field.
pub const WORK_SCHEMA_V1: u8 = 1;

/// Work-shapes v2 — corpus work (operator ruling 2026-08-04). A corpus
/// task is observed content and cannot be re-derived from a digest, so
/// it rides whole. It carried an admissibility proof for one morning;
/// the Merkle mechanism was struck the same day (see `crate::corpus`)
/// and the payload no longer carries a path.
pub const WORK_SCHEMA_V2_CORPUS: u8 = 2;

/// Work-shapes v3 — volumetric work (operator ruling 2026-08-04,
/// lifting C8's deferral). Forge-only and re-derivable like v1, so the
/// payload carries only the digest, the rung, the word and the proof —
/// no task content at all.
pub const WORK_SCHEMA_V3_VOLUME: u8 = 3;

/// Work-shapes v4 — the carried derivation (Concordance 4.4 annex, ruled
/// 2026-10-08). Forge work whose proof is checked step by step, never by
/// re-running the forge or re-sealing a certificate. The payload, in order
/// (integers little-endian; grids as everywhere on this wire, `rows u16 ‖
/// cols u16 ‖ cells`; a program as `count u16 ‖ ops` in the op table above;
/// a derivation as `count u16 ‖ state digests[32]`, see
/// [`crate::certificate::STATE_DOMAIN`]):
///
/// ```text
/// 4 ‖ forge_digest[32] ‖ tier u8 ‖ word_len u8 ‖ rows u8 ‖ cols u8 ‖ train_pairs u8
///   ‖ inputs:  (train_pairs + 1) grids — every training input, then the test input
///   ‖ forge word: program
///   ‖ forge derivation: (word_len + 1) digests
///   ‖ outputs: (train_pairs + 1) grids — every training output, then the test answer
///   ‖ solve: program
///   ‖ solve derivation: (solve length + 1) digests
/// ```
///
/// STATE 0 is the inputs, each cell checked against its beacon sample under
/// the digest; the forge word is checked against its own samples slot by
/// slot. The forge derivation walked over state 0 must end at the outputs;
/// the solve derivation walked over state 0 must end at the same outputs —
/// so it maps every training pair and renders the test answer.
pub const WORK_SCHEMA_V4_CARRIED: u8 = 4;

/// The decoded proof: everything the replay needs, nothing else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkProof {
    pub forge_digest: [u8; 32],
    pub difficulty: Difficulty,
    pub program: Program,
    pub output: Grid,
    pub certificate: SolveCertificate,
}

/// A corpus proof (schema v2): the task rides whole, because observed
/// content cannot be re-derived from a digest.
///
/// The task's NAME is not carried. It is not content (see
/// [`crate::corpus::task_digest`]), and the certificate needs a name to
/// seal, so both halves derive the same canonical one from the task_id:
/// [`canonical_name`]. A miner cannot vary it, and nothing depends on
/// what a corpus file happened to be called.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusProof {
    /// The content digest — `corpus::task_digest(train, test_in)`.
    /// Binds the payload to the task it claims. It does not admit the
    /// task: identity is not admission, and the Merkle admission
    /// mechanism was struck 2026-08-04 (see `corpus.rs`).
    pub task_id: [u8; 32],
    /// The observed task, riding whole: it cannot be re-derived.
    pub train: Vec<TaskPair>,
    pub test_in: Grid,
    pub program: Program,
    pub output: Grid,
    pub certificate: SolveCertificate,
}

/// A volumetric proof (schema v3): forge-only, so the task is named by
/// its digest and rung and re-derived rather than carried.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VolumeProof {
    pub forge_digest: [u8; 32],
    pub difficulty: VolDifficulty,
    pub program: VProgram,
    pub output: Volume,
    pub certificate: VolumeCertificate,
}

/// A CARRIED-DERIVATION proof (schema v4): forge work, every step carried.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CarriedProof {
    pub forge_digest: [u8; 32],
    pub difficulty: Difficulty,
    /// State 0: every training input, then the test input.
    pub inputs: Vec<Grid>,
    /// The forge's word and its carried derivation over state 0.
    pub forge_word: Program,
    pub forge_derivation: Derivation,
    /// Every training output, then the test answer.
    pub outputs: Vec<Grid>,
    /// The solve and its carried derivation over state 0.
    pub program: Program,
    pub derivation: Derivation,
}

impl CarriedProof {
    /// Seal a solve of the task forged from `digest` at `difficulty`: the
    /// forge's samples, both words and both derivations. `None` when the
    /// program does not close the task or a dimension exceeds the wire.
    pub fn seal(
        forge_digest: [u8; 32],
        difficulty: Difficulty,
        program: &Program,
    ) -> Option<CarriedProof> {
        let inputs: Vec<Grid> = (0..=difficulty.train_pairs)
            .map(|pair| forge_input(forge_digest, difficulty, pair))
            .collect();
        let forge_word = forge_word(forge_digest, difficulty);
        let (forge_derivation, outputs) = derive(&forge_word, &inputs)?;
        let (derivation, closed) = derive(program, &inputs)?;
        (closed == outputs).then_some(CarriedProof {
            forge_digest,
            difficulty,
            inputs,
            forge_word,
            forge_derivation,
            outputs,
            program: program.clone(),
            derivation,
        })
    }

    /// The test answer the solve renders.
    pub fn output(&self) -> Option<&Grid> {
        self.outputs.last()
    }
}

/// What a verified payload turned out to be. v1 returns a ladder rung;
/// corpus work has no rung — its identity is its task_id — so the gate
/// reports the class and the kernel prices it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkClass {
    /// Forge work: re-derivable, priced on the ladder by `mint_bound`.
    Forge(Difficulty),
    /// Corpus work: replayed and off the ladder entirely.
    ///
    /// Renamed from `AdmissibleCorpus` when the Merkle admission
    /// mechanism was struck (2026-08-04) — the old name claimed an
    /// admission this gate no longer performs, and a name that
    /// overclaims is the vacuity the motion clause 6 forbids.
    Corpus { task_id: [u8; 32] },
    /// Volumetric work: re-derivable like forge work, but on its own
    /// ladder — a volumetric rung is not a planar rung and must never
    /// be priced as one (the two `level(n)` tables are different
    /// shapes, and confusing them would pay 3-D work at 2-D rates).
    Volumetric(VolDifficulty),
}

/// The canonical task name for corpus work — lowercase hex of the
/// task_id. Derived, never carried, so prover and verifier cannot
/// disagree about it.
pub fn canonical_name(task_id: &[u8; 32]) -> String {
    let mut name = String::with_capacity(64);
    for byte in task_id {
        name.push_str(&format!("{byte:02x}"));
    }
    name
}

/// A refused proof — typed, the break named whole (the house law).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkBroken {
    /// The payload's form refuses: truncation, trailing bytes, an
    /// unknown schema or op code, an out-of-range dimension.
    Malformed,
    /// The program does not map training pair `pair` exactly — the
    /// honest-withhold law: a solution maps every pair or it is none.
    TrainingMiss { pair: usize },
    /// The program's test output differs from the claimed output, or
    /// the claim differs from the forge's own expected grid.
    OutputMismatch,
    /// The certificate chain does not re-derive; the first diverged
    /// link named (0 = genesis, 1..=n = steps, n+1 = output).
    ChainDiverged { at_step: usize },
    /// v2 — the carried task does not hash to the claimed `task_id`.
    /// The payload is internally inconsistent: it names one task and
    /// carries another.
    TaskIdMismatch,
    /// The payload is a well-formed shape this gate cannot price. Not
    /// a malformation — a shape the caller has not been taught. Today:
    /// corpus work reaching `verify_work`, whose return type can only
    /// express a ladder rung. `verify_work_class` accepts it.
    UnsupportedShape,
    /// **P4, the carrier law (operator ruling 2026-08-06): the
    /// volumetric carrier is RETIRED at the mint.** Verified forever
    /// (`verify_volume_work` stands — history is never orphaned),
    /// minting never again. What was an accident of `verify_work`'s
    /// return type is now the ruling, named as itself.
    CarrierRetired,
    /// **P4: a verified corpus proof with no measured price yet.** The
    /// carrier law prices corpus work by W(τ) = complexity / measured
    /// solve rate, and the rate table fills from the blind benchmark
    /// (P3). Until it does, a corpus mint refuses as unpriced — never
    /// as broken; the proof itself verified whole.
    CarrierUnpriced,
    /// **The resolution path is absent** (operator directive
    /// 2026-08-07: *verify-time search is prohibited; require mint
    /// requests to provide the complete resolution path as proof,
    /// validate it in time linear in the path, and reject ungranted
    /// where the path is absent*).
    ///
    /// A claim with no program states no resolution — there is nothing
    /// to re-apply, and establishing one would mean searching at the
    /// gate, which is exactly what the directive forbids. The claimant
    /// carries the path or the claim does not stand.
    ///
    /// Refused rather than silently admitted: an empty program is the
    /// identity, so a task whose pairs happen to be identity would
    /// otherwise verify having proven nothing.
    PathAbsent,
    /// v4 — state 0 or the forge word is not what the digest's beacon
    /// samples draw: a carried input (`pair`, the test input at
    /// `train_pairs`) or the forge word (`pair: None`) diverges.
    SampleDiverged { pair: Option<usize> },
    /// v4 — the forge's carried derivation breaks at `at_step` (0 = state 0,
    /// k = the k-th op, word_len + 1 = it does not end at the outputs).
    ForgeDiverged { at_step: usize },
}

fn mirror_code(m: TileMirror) -> u8 {
    match m {
        TileMirror::None => 0,
        TileMirror::Rows => 1,
        TileMirror::Cols => 2,
        TileMirror::Both => 3,
    }
}

fn mirror_from(code: u8) -> Option<TileMirror> {
    match code {
        0 => Some(TileMirror::None),
        1 => Some(TileMirror::Rows),
        2 => Some(TileMirror::Cols),
        3 => Some(TileMirror::Both),
        _ => None,
    }
}

fn objop_code(op: &ObjOp) -> u8 {
    match op {
        ObjOp::KeepLargest => 1,
        ObjOp::KeepSmallest => 2,
        ObjOp::DeleteLargest => 3,
        ObjOp::CropLargest => 4,
        ObjOp::CropContent => 5,
        ObjOp::PaintAllLikeLargest => 6,
        ObjOp::PaintAllMajority => 7,
        ObjOp::GravityDown => 8,
    }
}

fn objop_from(code: u8) -> Option<ObjOp> {
    match code {
        1 => Some(ObjOp::KeepLargest),
        2 => Some(ObjOp::KeepSmallest),
        3 => Some(ObjOp::DeleteLargest),
        4 => Some(ObjOp::CropLargest),
        5 => Some(ObjOp::CropContent),
        6 => Some(ObjOp::PaintAllLikeLargest),
        7 => Some(ObjOp::PaintAllMajority),
        8 => Some(ObjOp::GravityDown),
        _ => None,
    }
}

fn put_u16(n: usize, out: &mut Vec<u8>) -> Option<()> {
    let v = u16::try_from(n).ok()?;
    out.extend_from_slice(&v.to_le_bytes());
    Some(())
}

fn put_grid(g: &Grid, out: &mut Vec<u8>) -> Option<()> {
    put_u16(g.rows, out)?;
    put_u16(g.cols, out)?;
    out.extend_from_slice(&g.cells);
    Some(())
}

/// The one program encoding, shared by every schema. Extracted when v2
/// landed: a second copy of the op table is a second definition of wire
/// law, and two definitions drift.
fn put_op(op: &Op, out: &mut Vec<u8>) -> Option<()> {
    match op {
        Op::Rotate90 => out.push(1),
        Op::Rotate180 => out.push(2),
        Op::Rotate270 => out.push(3),
        Op::FlipH => out.push(4),
        Op::FlipV => out.push(5),
        Op::Transpose => out.push(6),
        Op::Recolor(map) => {
            out.push(7);
            out.extend_from_slice(map);
        }
        Op::Object(obj) => {
            out.push(8);
            out.push(objop_code(obj));
        }
        Op::Tile {
            down,
            across,
            odd_row,
            odd_col,
        } => {
            out.push(9);
            out.push(u8::try_from(*down).ok()?);
            out.push(u8::try_from(*across).ok()?);
            out.push(mirror_code(*odd_row));
            out.push(mirror_code(*odd_col));
        }
        // docs/62 L1, appended: one code one meaning for ever.
        Op::Shift { dr, dc } => {
            out.push(10);
            out.push(dr.to_le_bytes()[0]);
            out.push(dc.to_le_bytes()[0]);
        }
        Op::Dilate { up, down } => {
            out.push(11);
            out.push(*up);
            out.push(*down);
        }
        Op::Project(rule) => {
            out.push(12);
            put_select(&rule.select, out)?;
            match rule.act {
                crate::project::Act::Recolor(h) => {
                    out.push(0);
                    out.push(h);
                }
                crate::project::Act::Remove => out.push(1),
            }
        }
    }
    Some(())
}

/// A projector's predicate on the wire (docs/62 L7), structural: a tag, then
/// its arguments; `Not` and `And` nest. Append-only like the op table.
fn put_select(select: &crate::project::Select, out: &mut Vec<u8>) -> Option<()> {
    use crate::project::Select;
    match select {
        Select::Hue(h) => out.extend([0, *h]),
        Select::Size(k) => {
            out.push(1);
            put_u16(*k, out)?;
        }
        Select::Largest => out.push(2),
        Select::Smallest => out.push(3),
        Select::Central => out.push(9),
        Select::Ordered { cut, extreme } => {
            use crate::project::{Cut, Extreme};
            out.push(10);
            match cut {
                Cut::All => out.push(0),
                Cut::Nearest(k) => {
                    out.push(1);
                    out.extend(k.to_le_bytes());
                }
                Cut::Farthest(k) => {
                    out.push(2);
                    out.extend(k.to_le_bytes());
                }
            }
            out.push(match extreme {
                Extreme::Least => 0,
                Extreme::Most => 1,
            });
        }
        Select::Border(on) => out.extend([4, u8::from(*on)]),
        Select::TwinOfOtherHue => out.push(5),
        Select::Shape(cells) => {
            out.push(6);
            put_u16(cells.len(), out)?;
            for (r, c) in cells {
                out.push(u8::try_from(*r).ok()?);
                out.push(u8::try_from(*c).ok()?);
            }
        }
        Select::Not(inner) => {
            out.push(7);
            put_select(inner, out)?;
        }
        Select::And(a, b) => {
            out.push(8);
            put_select(a, out)?;
            put_select(b, out)?;
        }
    }
    Some(())
}

fn put_program(program: &Program, out: &mut Vec<u8>) -> Option<()> {
    put_u16(program.len(), out)?;
    for op in program {
        put_op(op, out)?;
    }
    Some(())
}

/// **The one PROGRAM encoding, exported** (NOVELTY-GATE, task #13) —
/// a minted word's expansion rides receipts as opaque bytes, and those
/// bytes must be this wire form, never a second table that drifts.
/// Round-trips with [`program_from_bytes`]; trailing bytes refuse.
pub fn program_bytes(program: &Program) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    put_program(program, &mut out)?;
    Some(out)
}

pub fn program_from_bytes(bytes: &[u8]) -> Option<Program> {
    let mut reader = Reader { bytes, at: 0 };
    let program = reader.program()?;
    (reader.at == bytes.len()).then_some(program)
}

fn put_volume(v: &Volume, out: &mut Vec<u8>) -> Option<()> {
    put_u16(v.depth(), out)?;
    for layer in v.layers() {
        put_grid(layer, out)?;
    }
    Some(())
}

/// The volumetric op table — wire law, append-only, one code one
/// meaning forever: 1 PerLayer + a planar op · 2 FlipStack ·
/// 3 CycleLayers. `PerLayer` nests the planar table rather than
/// restating it, so the two can never drift apart.
fn put_vprogram(program: &VProgram, out: &mut Vec<u8>) -> Option<()> {
    put_u16(program.len(), out)?;
    for op in program {
        match op {
            VOp::PerLayer(planar) => {
                out.push(1);
                put_op(planar, out)?;
            }
            VOp::FlipStack => out.push(2),
            VOp::CycleLayers => out.push(3),
        }
    }
    Some(())
}

fn put_certificate(cert: &SolveCertificate, out: &mut Vec<u8>) -> Option<()> {
    out.extend_from_slice(&cert.genesis);
    put_u16(cert.steps.len(), out)?;
    for step in &cert.steps {
        out.extend_from_slice(step);
    }
    out.extend_from_slice(&cert.output);
    Some(())
}

/// Encode a proof. `None` if any dimension exceeds the wire's range —
/// the wire never lies by clamping.
pub fn encode_work(proof: &WorkProof) -> Option<Vec<u8>> {
    let mut out = vec![WORK_SCHEMA_V1];
    out.extend_from_slice(&proof.forge_digest);
    let d = proof.difficulty;
    out.push(d.tier);
    out.push(u8::try_from(d.word_len).ok()?);
    out.push(u8::try_from(d.rows).ok()?);
    out.push(u8::try_from(d.cols).ok()?);
    out.push(u8::try_from(d.train_pairs).ok()?);
    put_program(&proof.program, &mut out)?;
    put_grid(&proof.output, &mut out)?;
    put_certificate(&proof.certificate, &mut out)?;
    Some(out)
}

/// Encode a corpus proof (schema v2). The task rides whole — it is the
/// one work shape that cannot be re-derived.
pub fn encode_corpus_work(proof: &CorpusProof) -> Option<Vec<u8>> {
    let mut out = vec![WORK_SCHEMA_V2_CORPUS];
    out.extend_from_slice(&proof.task_id);
    // The inclusion path was struck 2026-08-04 with the Merkle root.
    // v2 payloads from the one morning that shape existed do not decode
    // under this schema and were never emitted by anything — no
    // conductor arm builds corpus work, and no chain carries one.
    put_u16(proof.train.len(), &mut out)?;
    for pair in &proof.train {
        put_grid(&pair.input, &mut out)?;
        put_grid(&pair.output, &mut out)?;
    }
    put_grid(&proof.test_in, &mut out)?;
    put_program(&proof.program, &mut out)?;
    put_grid(&proof.output, &mut out)?;
    put_certificate(&proof.certificate, &mut out)?;
    Some(out)
}

struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let held = self.bytes.get(self.at..self.at + n)?;
        self.at += n;
        Some(held)
    }
    fn byte(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }
    fn u16(&mut self) -> Option<usize> {
        Some(u16::from_le_bytes(self.take(2)?.try_into().ok()?) as usize)
    }
    fn digest(&mut self) -> Option<[u8; 32]> {
        self.take(32)?.try_into().ok()
    }
    fn grid(&mut self) -> Option<Grid> {
        let rows = self.u16()?;
        let cols = self.u16()?;
        let cells = self.take(rows.checked_mul(cols)?)?.to_vec();
        Grid::new(rows, cols, cells)
    }
    /// The one op decoding, shared by every schema and by the
    /// volumetric `PerLayer`. Unknown codes refuse.
    fn op(&mut self) -> Option<Op> {
        Some(match self.byte()? {
            1 => Op::Rotate90,
            2 => Op::Rotate180,
            3 => Op::Rotate270,
            4 => Op::FlipH,
            5 => Op::FlipV,
            6 => Op::Transpose,
            7 => Op::Recolor(self.take(10)?.try_into().ok()?),
            8 => Op::Object(objop_from(self.byte()?)?),
            9 => Op::Tile {
                down: self.byte()? as usize,
                across: self.byte()? as usize,
                odd_row: mirror_from(self.byte()?)?,
                odd_col: mirror_from(self.byte()?)?,
            },
            10 => Op::Shift {
                dr: i8::from_le_bytes([self.byte()?]),
                dc: i8::from_le_bytes([self.byte()?]),
            },
            11 => {
                let (up, down) = (self.byte()?, self.byte()?);
                // One of the two is 1, and neither is 0: a dilator is one scale.
                if up == 0 || down == 0 || (up > 1 && down > 1) {
                    return None;
                }
                Op::Dilate { up, down }
            }
            12 => {
                let select = self.select(0)?;
                let act = match self.byte()? {
                    0 => crate::project::Act::Recolor(self.byte()?),
                    1 => crate::project::Act::Remove,
                    _ => return None,
                };
                Op::Project(crate::project::ObjectRule { select, act })
            }
            _ => return None,
        })
    }
    /// A projector's predicate; nesting bounded so a hostile tape cannot
    /// recurse without end.
    fn select(&mut self, depth: usize) -> Option<crate::project::Select> {
        use crate::project::Select;
        if depth > 8 {
            return None;
        }
        Some(match self.byte()? {
            0 => Select::Hue(self.byte()?),
            1 => Select::Size(self.u16()?),
            2 => Select::Largest,
            3 => Select::Smallest,
            4 => Select::Border(self.byte()? != 0),
            5 => Select::TwinOfOtherHue,
            6 => {
                let n = self.u16()?;
                let mut cells = Vec::with_capacity(n.min(self.bytes.len()));
                for _ in 0..n {
                    cells.push((usize::from(self.byte()?), usize::from(self.byte()?)));
                }
                Select::Shape(cells)
            }
            7 => Select::Not(Box::new(self.select(depth + 1)?)),
            8 => Select::And(
                Box::new(self.select(depth + 1)?),
                Box::new(self.select(depth + 1)?),
            ),
            9 => Select::Central,
            10 => {
                use crate::project::{Cut, Extreme};
                let cut = match self.byte()? {
                    0 => Cut::All,
                    1 => Cut::Nearest(u16::try_from(self.u16()?).ok()?),
                    2 => Cut::Farthest(u16::try_from(self.u16()?).ok()?),
                    _ => return None,
                };
                let extreme = match self.byte()? {
                    0 => Extreme::Least,
                    1 => Extreme::Most,
                    _ => return None,
                };
                Select::Ordered { cut, extreme }
            }
            _ => return None,
        })
    }
    fn program(&mut self) -> Option<Program> {
        let op_count = self.u16()?;
        let mut program: Program = Vec::new();
        for _ in 0..op_count {
            program.push(self.op()?);
        }
        Some(program)
    }
    fn volume(&mut self) -> Option<Volume> {
        let depth = self.u16()?;
        let mut layers = Vec::new();
        for _ in 0..depth {
            layers.push(self.grid()?);
        }
        // `Volume::new` refuses an empty stack and incongruent layers —
        // the carrier's own law, not restated here.
        Volume::new(layers)
    }
    fn vprogram(&mut self) -> Option<VProgram> {
        let op_count = self.u16()?;
        let mut program: VProgram = Vec::new();
        for _ in 0..op_count {
            program.push(match self.byte()? {
                1 => VOp::PerLayer(self.op()?),
                2 => VOp::FlipStack,
                3 => VOp::CycleLayers,
                _ => return None,
            });
        }
        Some(program)
    }
    fn derivation(&mut self) -> Option<Derivation> {
        let count = self.u16()?;
        let mut states = Vec::new();
        for _ in 0..count {
            states.push(self.digest()?);
        }
        Some(Derivation { states })
    }
    fn certificate(&mut self) -> Option<SolveCertificate> {
        let genesis = self.digest()?;
        let step_count = self.u16()?;
        let mut steps = Vec::new();
        for _ in 0..step_count {
            steps.push(self.digest()?);
        }
        Some(SolveCertificate {
            genesis,
            steps,
            output: self.digest()?,
        })
    }
}

/// Decode a proof — the form only. Truncation, trailing bytes,
/// unknown codes, malformed grids all `None`.
pub fn decode_work(bytes: &[u8]) -> Option<WorkProof> {
    let mut r = Reader { bytes, at: 0 };
    if r.byte()? != WORK_SCHEMA_V1 {
        return None;
    }
    let forge_digest = r.digest()?;
    let difficulty = Difficulty {
        tier: r.byte()?,
        word_len: r.byte()? as usize,
        rows: r.byte()? as usize,
        cols: r.byte()? as usize,
        train_pairs: r.byte()? as usize,
    };
    // Refuse degenerate difficulty as form (the refuse-never-guess law
    // extended to degenerate fields). Two holes close here:
    //  · train_pairs 0 → the forge yields an empty training set and
    //    "maps every training pair" is vacuously true (a free mint);
    //  · rows/cols 0 → the forge's Grid::new(...).expect() panics in
    //    the kernel's call path — the totality law forbids it.
    // The gate never re-forges a degenerate task: it refuses first.
    // The condition now carries a NAME (`Difficulty::well_formed`) so the
    // v2 corpus decode path calls the same law instead of restating it.
    if !difficulty.well_formed() {
        return None;
    }
    let program = r.program()?;
    let output = r.grid()?;
    let certificate = r.certificate()?;
    if r.at != bytes.len() {
        return None; // trailing bytes refuse — the payload is exact
    }
    Some(WorkProof {
        forge_digest,
        difficulty,
        program,
        output,
        certificate,
    })
}

fn put_derivation(derivation: &Derivation, out: &mut Vec<u8>) -> Option<()> {
    put_u16(derivation.states.len(), out)?;
    for state in &derivation.states {
        out.extend_from_slice(state);
    }
    Some(())
}

/// Encode a CARRIED-DERIVATION proof (schema v4; the layout is documented at
/// [`WORK_SCHEMA_V4_CARRIED`]).
pub fn encode_carried_work(proof: &CarriedProof) -> Option<Vec<u8>> {
    let d = proof.difficulty;
    let pairs = d.train_pairs.checked_add(1)?;
    if proof.inputs.len() != pairs || proof.outputs.len() != pairs {
        return None;
    }
    let mut out = vec![WORK_SCHEMA_V4_CARRIED];
    out.extend_from_slice(&proof.forge_digest);
    out.push(d.tier);
    out.push(u8::try_from(d.word_len).ok()?);
    out.push(u8::try_from(d.rows).ok()?);
    out.push(u8::try_from(d.cols).ok()?);
    out.push(u8::try_from(d.train_pairs).ok()?);
    for g in &proof.inputs {
        put_grid(g, &mut out)?;
    }
    put_program(&proof.forge_word, &mut out)?;
    put_derivation(&proof.forge_derivation, &mut out)?;
    for g in &proof.outputs {
        put_grid(g, &mut out)?;
    }
    put_program(&proof.program, &mut out)?;
    put_derivation(&proof.derivation, &mut out)?;
    Some(out)
}

/// Decode a CARRIED-DERIVATION proof (schema v4) — the form only.
pub fn decode_carried_work(bytes: &[u8]) -> Option<CarriedProof> {
    let mut r = Reader { bytes, at: 0 };
    if r.byte()? != WORK_SCHEMA_V4_CARRIED {
        return None;
    }
    let forge_digest = r.digest()?;
    let difficulty = Difficulty {
        tier: r.byte()?,
        word_len: r.byte()? as usize,
        rows: r.byte()? as usize,
        cols: r.byte()? as usize,
        train_pairs: r.byte()? as usize,
    };
    // The degeneracy law, at this door too (no forge call can panic, and
    // "every training pair" is never vacuous).
    if !difficulty.well_formed() {
        return None;
    }
    let pairs = difficulty.train_pairs + 1;
    let inputs = (0..pairs).map(|_| r.grid()).collect::<Option<Vec<_>>>()?;
    let forge_word = r.program()?;
    let forge_derivation = r.derivation()?;
    let outputs = (0..pairs).map(|_| r.grid()).collect::<Option<Vec<_>>>()?;
    let program = r.program()?;
    let derivation = r.derivation()?;
    if r.at != bytes.len() {
        return None;
    }
    Some(CarriedProof {
        forge_digest,
        difficulty,
        inputs,
        forge_word,
        forge_derivation,
        outputs,
        program,
        derivation,
    })
}

/// **The CARRIED-DERIVATION judge** (schema v4, Concordance 4.4 annex). Every
/// leg is LOCAL — nothing re-runs the forge, nothing re-seals:
///
///   0. the path is carried (a claim states its resolution);
///   1. STATE 0 — every carried input is the beacon's own sample under the
///      digest, cell by cell, and the forge word is its own samples, slot
///      by slot;
///   2. The TASK — the forge word's carried derivation walks from state 0,
///      step by step, to the carried outputs;
///   3. The solve — the solve's carried derivation walks from the same
///      state 0, step by step, to the same outputs: every training pair
///      maps exactly and the test input renders the test answer.
pub fn verify_carried_work(bytes: &[u8]) -> Result<Difficulty, WorkBroken> {
    let proof = decode_carried_work(bytes).ok_or(WorkBroken::Malformed)?;
    if proof.program.is_empty() {
        return Err(WorkBroken::PathAbsent);
    }
    let (digest, difficulty) = (proof.forge_digest, proof.difficulty);
    // 1. State 0 and the forge word, checked against their samples.
    for (pair, input) in proof.inputs.iter().enumerate() {
        if *input != forge_input(digest, difficulty, pair) {
            return Err(WorkBroken::SampleDiverged { pair: Some(pair) });
        }
    }
    if proof.forge_word != forge_word(digest, difficulty) {
        return Err(WorkBroken::SampleDiverged { pair: None });
    }
    // 2. The task: the forge's derivation, walked to the carried outputs.
    let forged = walk(&proof.forge_derivation, &proof.forge_word, &proof.inputs)
        .map_err(|at_step| WorkBroken::ForgeDiverged { at_step })?;
    if forged != proof.outputs {
        return Err(WorkBroken::ForgeDiverged {
            at_step: proof.forge_word.len() + 1,
        });
    }
    // 3. The solve: its derivation, walked to the same outputs.
    let closed = walk(&proof.derivation, &proof.program, &proof.inputs)
        .map_err(|at_step| WorkBroken::ChainDiverged { at_step })?;
    let tested = difficulty.train_pairs;
    if let Some(pair) = (0..tested).find(|&pair| closed[pair] != proof.outputs[pair]) {
        return Err(WorkBroken::TrainingMiss { pair });
    }
    if closed[tested] != proof.outputs[tested] {
        return Err(WorkBroken::OutputMismatch);
    }
    Ok(difficulty)
}

/// Encode a volumetric proof (schema v3).
pub fn encode_volume_work(proof: &VolumeProof) -> Option<Vec<u8>> {
    let mut out = vec![WORK_SCHEMA_V3_VOLUME];
    out.extend_from_slice(&proof.forge_digest);
    let d = proof.difficulty;
    out.push(u8::try_from(d.layers).ok()?);
    out.push(u8::try_from(d.word_len).ok()?);
    out.push(u8::try_from(d.rows).ok()?);
    out.push(u8::try_from(d.cols).ok()?);
    out.push(u8::try_from(d.train_pairs).ok()?);
    put_vprogram(&proof.program, &mut out)?;
    put_volume(&proof.output, &mut out)?;
    out.extend_from_slice(&proof.certificate.genesis);
    put_u16(proof.certificate.steps.len(), &mut out)?;
    for step in &proof.certificate.steps {
        out.extend_from_slice(step);
    }
    out.extend_from_slice(&proof.certificate.output);
    Some(out)
}

/// Decode a volumetric proof (schema v3) — the form only.
pub fn decode_volume_work(bytes: &[u8]) -> Option<VolumeProof> {
    let mut r = Reader { bytes, at: 0 };
    if r.byte()? != WORK_SCHEMA_V3_VOLUME {
        return None;
    }
    let forge_digest = r.digest()?;
    let difficulty = VolDifficulty {
        layers: r.byte()? as usize,
        word_len: r.byte()? as usize,
        rows: r.byte()? as usize,
        cols: r.byte()? as usize,
        train_pairs: r.byte()? as usize,
    };
    // The vacuity, REFUSED as form — before the forge is ever called,
    // so the degenerate case is not merely guarded but unreachable.
    // `train_pairs: 0` is the exploitable one: "maps every training
    // pair" is vacuously true over none, which is a free mint. The
    // zero dimensions are the totality half — they panic the forge.
    if !difficulty.well_formed() {
        return None;
    }
    let program = r.vprogram()?;
    let output = r.volume()?;
    let genesis = r.digest()?;
    let step_count = r.u16()?;
    let mut steps = Vec::new();
    for _ in 0..step_count {
        steps.push(r.digest()?);
    }
    let cert_output = r.digest()?;
    if r.at != bytes.len() {
        return None;
    }
    Some(VolumeProof {
        forge_digest,
        difficulty,
        program,
        output,
        certificate: VolumeCertificate {
            genesis,
            steps,
            output: cert_output,
        },
    })
}

/// The volumetric judge (work-shapes v3). The REPLAY-COMPLETE law in
/// three dimensions — every leg or refuse:
///
///   1. **re-forge** — the task is its digest under this rung, pure and
///      deterministic, no external data ever. Volumetric work is
///      forge-only, so this single leg does what corpus v2 needed a
///      Merkle root and an inclusion path to do.
///   2. **re-apply** — the program maps every training pair exactly and
///      the test input to the claimed output; the forge's own
///      `test_expected` must agree. One miss refuses — the honest
///      withhold, never a guess.
///   3. **re-chain** — the certificate re-derives Intact, naming its
///      break otherwise. This is the leg C8 correctly said did not
///      exist: the old `seal_volume` was a bare digest no third party
///      could re-derive.
///
/// The degeneracy guard sits in `decode_volume_work`, so leg 1 never
/// receives a rung that could panic it or make leg 2 vacuous.
pub fn verify_volume_work(bytes: &[u8]) -> Result<WorkClass, WorkBroken> {
    let proof = decode_volume_work(bytes).ok_or(WorkBroken::Malformed)?;
    // 1. Re-forge.
    let task = forge_volume(proof.forge_digest, proof.difficulty);
    // 2. Re-apply.
    for (pair, (input, expected)) in task.train.iter().enumerate() {
        if &run_volume(&proof.program, input) != expected {
            return Err(WorkBroken::TrainingMiss { pair });
        }
    }
    let produced = run_volume(&proof.program, &task.test_in);
    if produced != proof.output {
        return Err(WorkBroken::OutputMismatch);
    }
    if let Some(expected) = &task.test_expected {
        if &produced != expected {
            return Err(WorkBroken::OutputMismatch);
        }
    }
    // 3. Re-chain.
    match verify_volume_certificate(&proof.certificate, &task, &proof.program, &proof.output) {
        VolumeCertificateRead::Intact => Ok(WorkClass::Volumetric(proof.difficulty)),
        VolumeCertificateRead::Diverged { at_step } => Err(WorkBroken::ChainDiverged { at_step }),
    }
}

/// Decode a corpus proof (schema v2) — the form only.
pub fn decode_corpus_work(bytes: &[u8]) -> Option<CorpusProof> {
    let mut r = Reader { bytes, at: 0 };
    if r.byte()? != WORK_SCHEMA_V2_CORPUS {
        return None;
    }
    let task_id = r.digest()?;
    let train_len = r.u16()?;
    // The vacuity, refused for the new shape too (98ccbbe's law): a
    // replay over no training pair proves nothing, and "maps every
    // pair" is vacuously true over none. v1 refuses this through
    // `Difficulty::well_formed`; v2 has no difficulty, so it refuses here.
    if train_len == 0 {
        return None;
    }
    let mut train = Vec::new();
    for _ in 0..train_len {
        let input = r.grid()?;
        let output = r.grid()?;
        train.push(TaskPair { input, output });
    }
    let test_in = r.grid()?;
    let program = r.program()?;
    let output = r.grid()?;
    let certificate = r.certificate()?;
    if r.at != bytes.len() {
        return None;
    }
    Some(CorpusProof {
        task_id,
        train,
        test_in,
        program,
        output,
        certificate,
    })
}

/// The judge's call, class-aware — verify any work payload and report
/// what it turned out to be. Pure, total, no I/O.
///
/// Dispatch is on the leading schema byte, so v1 and v2 can never be
/// confused for one another and a future v3 costs one arm.
///
/// **The kernel should call this**, not [`verify_work`]: only this
/// entry point can express corpus work, whose identity is a task_id and
/// not a ladder rung. Relayed to the kernel lane along with the
/// `mint_bound` arm that prices it.
pub fn verify_work_class(bytes: &[u8]) -> Result<WorkClass, WorkBroken> {
    match bytes.first() {
        Some(&WORK_SCHEMA_V1) => verify_work(bytes).map(WorkClass::Forge),
        Some(&WORK_SCHEMA_V4_CARRIED) => verify_carried_work(bytes).map(WorkClass::Forge),
        Some(&WORK_SCHEMA_V2_CORPUS) => verify_corpus_work(bytes),
        Some(&WORK_SCHEMA_V3_VOLUME) => verify_volume_work(bytes),
        _ => Err(WorkBroken::Malformed),
    }
}

/// The corpus judge (work-shapes v2). Three legs, all or refuse:
///
///   1. **re-hash** — the carried task hashes to the claimed `task_id`;
///      a payload that names one task and carries another refuses.
///   2. **re-apply** — the program maps every training pair exactly and
///      the test input to the claimed output (the honest-withhold law).
///   3. **re-chain** — the certificate re-derives Intact.
///
/// **The admit leg was struck 2026-08-04** (operator ruling: no Merkle
/// trees). It checked an inclusion path against a pinned root, which is
/// a static enumeration and an authority — see `crate::corpus` for the
/// reasoning kept whole. Its replacement is a predicate, not a list: a
/// claim is admissible when the base vocabulary provably fails the task
/// at depth `d` and the candidate closes it, both halves replayable
/// with the machinery already here.
///
/// **So this gate does not admit, and must not be read as if it did.**
/// It proves a payload is internally consistent and that the program
/// closes the carried task. Whether the task is one anybody authorized
/// is a question it no longer asks — lawfully, because corpus work has
/// no rate either: the motion clause 4 makes imported content a
/// benchmark and never a mint source. It carries no seed, it can be
/// authored, and its held-out answer is not checkable by the gate.
///
/// **What this cannot check, stated plainly and unchanged by the
/// strike.** A corpus task's held-out answer is not carried and not
/// derivable, so leg 2 proves the program closes the training pairs and
/// renders the claimed output — never that the output is correct.
/// Measured: of 34 programs that closed training, 33 matched the
/// held-out answer and one did not, so ~3% of closures are confidently
/// wrong. That was the fabrication surface a corpus rate would have
/// bought, and it is now one more reason there is no such rate.
pub fn verify_corpus_work(bytes: &[u8]) -> Result<WorkClass, WorkBroken> {
    let proof = decode_corpus_work(bytes).ok_or(WorkBroken::Malformed)?;
    // 0. The path is carried, or there is no claim (operator directive
    //    2026-08-07). Everything below RE-applies what the claimant
    //    supplied and is linear in the path's length; nothing here
    //    searches for a path, because a gate that searches prices
    //    admission at the cost of a search.
    if proof.program.is_empty() {
        return Err(WorkBroken::PathAbsent);
    }
    // 1. Re-hash: identity is content, so recompute it from content.
    if crate::corpus::task_digest(&proof.train, &proof.test_in) != proof.task_id {
        return Err(WorkBroken::TaskIdMismatch);
    }
    // The task, reconstructed exactly as the prover must have built it.
    let task = Task {
        name: canonical_name(&proof.task_id),
        train: proof.train.clone(),
        test_in: proof.test_in.clone(),
        test_expected: None,
    };
    // 2. Re-apply.
    for (pair, held) in task.train.iter().enumerate() {
        if run(&proof.program, &held.input) != held.output {
            return Err(WorkBroken::TrainingMiss { pair });
        }
    }
    if run(&proof.program, &task.test_in) != proof.output {
        return Err(WorkBroken::OutputMismatch);
    }
    // 3. Re-chain.
    match verify_certificate(&proof.certificate, &task, &proof.program, &proof.output) {
        CertificateRead::Intact => Ok(WorkClass::Corpus {
            task_id: proof.task_id,
        }),
        CertificateRead::Diverged { at_step } => Err(WorkBroken::ChainDiverged { at_step }),
    }
}

/// The judge's call — verify a forge payload whole. Pure, total,
/// no I/O; the kernel hands the bytes here and never parses them.
///
/// Unchanged by v2 and deliberately so: its return type is a ladder
/// rung, which corpus work does not have. A corpus payload reaching
/// here refuses `UnsupportedShape` rather than being priced as
/// something it is not — the caller has simply not been taught the
/// shape yet. [`verify_work_class`] is the taught entry point.
pub fn verify_work(bytes: &[u8]) -> Result<Difficulty, WorkBroken> {
    if matches!(
        bytes.first(),
        Some(&WORK_SCHEMA_V2_CORPUS) | Some(&WORK_SCHEMA_V3_VOLUME)
    ) {
        return Err(WorkBroken::UnsupportedShape);
    }
    // v4 is forge work too, proved by its carried derivation.
    if bytes.first() == Some(&WORK_SCHEMA_V4_CARRIED) {
        return verify_carried_work(bytes);
    }
    let proof = decode_work(bytes).ok_or(WorkBroken::Malformed)?;
    // 0. The path is carried, or there is no claim (operator directive
    //    2026-08-07). The forge carrier mints too, so the law is the
    //    same law: a claim states its resolution, and the gate
    //    re-applies it in time linear in the path. Nothing here
    //    searches.
    if proof.program.is_empty() {
        return Err(WorkBroken::PathAbsent);
    }
    // 1. Re-forge: the task is its digest under this difficulty.
    let task = forge_task(proof.forge_digest, proof.difficulty);
    // 2. Re-apply: every training pair exactly, or the solve is none.
    for (pair, held) in task.train.iter().enumerate() {
        if run(&proof.program, &held.input) != held.output {
            return Err(WorkBroken::TrainingMiss { pair });
        }
    }
    let produced = run(&proof.program, &task.test_in);
    if produced != proof.output {
        return Err(WorkBroken::OutputMismatch);
    }
    if let Some(expected) = &task.test_expected {
        if &produced != expected {
            return Err(WorkBroken::OutputMismatch);
        }
    }
    // 3. Re-chain: the certificate re-derives or names its break.
    match verify_certificate(&proof.certificate, &task, &proof.program, &proof.output) {
        CertificateRead::Intact => Ok(proof.difficulty),
        CertificateRead::Diverged { at_step } => Err(WorkBroken::ChainDiverged { at_step }),
    }
}
