//! THE WORK PROOF — the opaque payload's one form and its gate
//! (U1 KERNEL-VERIFY, U_DIRECTIVES §2).
//!
//! THE OPAQUE-PAYLOAD LAW: the `DiscoveryMinted` deed carries its
//! proof as bytes that cfe never reads and the kernel never parses —
//! this module is the encoding's MONOPOLY (the codec-seam pattern:
//! one component owns the form). Refuse never guess: truncation,
//! trailing bytes, unknown codes, out-of-range dimensions all refuse.
//!
//! THE REPLAY-COMPLETE LAW: the certificate chain alone binds
//! (task, program, output) — it does NOT prove the program SOLVES the
//! task; an intact chain can be sealed over a wrong output. So
//! [`verify_work`] is the full replay, every leg or refuse:
//!   1. re-forge — the task re-derives from its digest (pure);
//!   2. re-apply — the program maps EVERY training pair exactly and
//!      the test input to the claimed output (the honest-withhold
//!      law: one miss = refuse, never a guess); the forge's own
//!      `test_expected` must agree;
//!   3. re-chain — the certificate re-derives Intact.
//!
//! The op-code vocabulary below is wire law — append-only, one code
//! one meaning forever: 1 Rotate90 · 2 Rotate180 · 3 Rotate270 ·
//! 4 FlipH · 5 FlipV · 6 Transpose · 7 Recolor+map[10] ·
//! 8 Object+objop (1 KeepLargest · 2 KeepSmallest · 3 DeleteLargest ·
//! 4 CropLargest · 5 CropContent · 6 PaintAllLikeLargest ·
//! 7 PaintAllMajority · 8 GravityDown) · 9 Tile+down,across,odd_row,
//! odd_col (TileMirror: 0 None · 1 Rows · 2 Cols · 3 Both).

use crate::certificate::{verify_certificate, CertificateRead, SolveCertificate};
use crate::forge::{forge_task, Difficulty};
use crate::grid::{run, Grid, Op, Program, TileMirror};
use crate::loader::{Task, TaskPair};
use crate::objects::ObjOp;
use crate::volume::{
    forge_volume, run_volume, verify_volume_certificate, VOp, VProgram, VolDifficulty, Volume,
    VolumeCertificate, VolumeCertificateRead,
};

/// The work-proof wire schema. A bump is an amendment.
///
/// Schema bytes are APPEND-ONLY — one code, one meaning forever. v1 is
/// untouched by v2: the same v1 bytes decode to the same v1 proof, and
/// a v1 payload never grows a field.
pub const WORK_SCHEMA_V1: u8 = 1;

/// Work-shapes v2 — CORPUS work (operator ruling 2026-08-04). A corpus
/// task is observed content and cannot be re-derived from a digest, so
/// it rides WHOLE. It carried an admissibility proof for one morning;
/// the Merkle mechanism was struck the same day (see `crate::corpus`)
/// and the payload no longer carries a path.
pub const WORK_SCHEMA_V2_CORPUS: u8 = 2;

/// Work-shapes v3 — VOLUMETRIC work (operator ruling 2026-08-04,
/// lifting C8's deferral). Forge-only and re-derivable like v1, so the
/// payload carries only the digest, the rung, the word and the proof —
/// no task content at all.
pub const WORK_SCHEMA_V3_VOLUME: u8 = 3;

/// The decoded proof: everything the replay needs, nothing else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkProof {
    pub forge_digest: [u8; 32],
    pub difficulty: Difficulty,
    pub program: Program,
    pub output: Grid,
    pub certificate: SolveCertificate,
}

/// A CORPUS proof (schema v2): the task rides whole, because observed
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
    /// Binds the payload to the task it claims. It does NOT admit the
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

/// A VOLUMETRIC proof (schema v3): forge-only, so the task is named by
/// its digest and rung and re-derived rather than carried.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VolumeProof {
    pub forge_digest: [u8; 32],
    pub difficulty: VolDifficulty,
    pub program: VProgram,
    pub output: Volume,
    pub certificate: VolumeCertificate,
}

/// What a verified payload turned out to BE. v1 returns a ladder rung;
/// corpus work has no rung — its identity is its task_id — so the gate
/// reports the class and the kernel prices it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkClass {
    /// Forge work: re-derivable, priced on the ladder by `mint_bound`.
    Forge(Difficulty),
    /// Corpus work: replayed and OFF the ladder entirely.
    ///
    /// Renamed from `AdmissibleCorpus` when the Merkle admission
    /// mechanism was struck (2026-08-04) — the old name CLAIMED an
    /// admission this gate no longer performs, and a name that
    /// overclaims is the vacuity THE MOTION clause 6 forbids.
    Corpus { task_id: [u8; 32] },
    /// Volumetric work: re-derivable like forge work, but on its OWN
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
    /// The payload's FORM refuses: truncation, trailing bytes, an
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
    /// a malformation — a SHAPE the caller has not been taught. Today:
    /// corpus work reaching `verify_work`, whose return type can only
    /// express a ladder rung. `verify_work_class` accepts it.
    UnsupportedShape,
    /// **P4, THE CARRIER LAW (operator ruling 2026-08-06): the
    /// volumetric carrier is RETIRED at the mint.** Verified forever
    /// (`verify_volume_work` stands — history is never orphaned),
    /// minting never again. What was an accident of `verify_work`'s
    /// return type is now the ruling, named as itself.
    CarrierRetired,
    /// **P4: a VERIFIED corpus proof with no measured price yet.** The
    /// carrier law prices corpus work by W(τ) = complexity / measured
    /// solve rate, and the rate table fills from the blind benchmark
    /// (P3). Until it does, a corpus mint refuses as UNPRICED — never
    /// as broken; the proof itself verified whole.
    CarrierUnpriced,
    /// **THE RESOLUTION PATH IS ABSENT** (operator directive
    /// 2026-08-07: *verify-time search is PROHIBITED; require mint
    /// requests to provide the complete resolution path as proof,
    /// validate it in time linear in the path, and reject ungranted
    /// where the path is absent*).
    ///
    /// A claim with no program states no resolution — there is nothing
    /// to re-apply, and establishing one would mean SEARCHING at the
    /// gate, which is exactly what the directive forbids. The claimant
    /// carries the path or the claim does not stand.
    ///
    /// Refused rather than silently admitted: an empty program is the
    /// identity, so a task whose pairs happen to be identity would
    /// otherwise verify having proven nothing.
    PathAbsent,
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

/// The ONE program encoding, shared by every schema. Extracted when v2
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
        Op::Tile { down, across, odd_row, odd_col } => {
            out.push(9);
            out.push(u8::try_from(*down).ok()?);
            out.push(u8::try_from(*across).ok()?);
            out.push(mirror_code(*odd_row));
            out.push(mirror_code(*odd_col));
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

fn put_volume(v: &Volume, out: &mut Vec<u8>) -> Option<()> {
    put_u16(v.depth(), out)?;
    for layer in v.layers() {
        put_grid(layer, out)?;
    }
    Some(())
}

/// The volumetric op table — wire law, append-only, one code one
/// meaning forever: 1 PerLayer + a planar op · 2 FlipStack ·
/// 3 CycleLayers. `PerLayer` nests the PLANAR table rather than
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

/// Encode a CORPUS proof (schema v2). The task rides whole — it is the
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
    /// The ONE op decoding, shared by every schema and by the
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
            _ => return None,
        })
    }
    fn program(&mut self) -> Option<Program> {
        let op_count = self.u16()?;
        let mut program: Program = Vec::with_capacity(op_count);
        for _ in 0..op_count {
            program.push(self.op()?);
        }
        Some(program)
    }
    fn volume(&mut self) -> Option<Volume> {
        let depth = self.u16()?;
        let mut layers = Vec::with_capacity(depth);
        for _ in 0..depth {
            layers.push(self.grid()?);
        }
        // `Volume::new` refuses an empty stack and incongruent layers —
        // the carrier's own law, not restated here.
        Volume::new(layers)
    }
    fn vprogram(&mut self) -> Option<VProgram> {
        let op_count = self.u16()?;
        let mut program: VProgram = Vec::with_capacity(op_count);
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
    fn certificate(&mut self) -> Option<SolveCertificate> {
        let genesis = self.digest()?;
        let step_count = self.u16()?;
        let mut steps = Vec::with_capacity(step_count);
        for _ in 0..step_count {
            steps.push(self.digest()?);
        }
        Some(SolveCertificate { genesis, steps, output: self.digest()? })
    }
}

/// Decode a proof — the FORM only. Truncation, trailing bytes,
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
    // Refuse DEGENERATE difficulty as form (the refuse-never-guess law
    // extended to degenerate fields). Two holes close here:
    //  · train_pairs 0 → the forge yields an empty training set and
    //    "maps EVERY training pair" is vacuously true (a free mint);
    //  · rows/cols 0 → the forge's Grid::new(...).expect() PANICS in
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

/// Encode a VOLUMETRIC proof (schema v3).
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

/// Decode a VOLUMETRIC proof (schema v3) — the FORM only.
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
    // THE VACUITY, REFUSED AS FORM — before the forge is ever called,
    // so the degenerate case is not merely guarded but unreachable.
    // `train_pairs: 0` is the exploitable one: "maps EVERY training
    // pair" is vacuously true over none, which is a free mint. The
    // zero dimensions are the totality half — they PANIC the forge.
    if !difficulty.well_formed() {
        return None;
    }
    let program = r.vprogram()?;
    let output = r.volume()?;
    let genesis = r.digest()?;
    let step_count = r.u16()?;
    let mut steps = Vec::with_capacity(step_count);
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
        certificate: VolumeCertificate { genesis, steps, output: cert_output },
    })
}

/// THE VOLUMETRIC JUDGE (work-shapes v3). The REPLAY-COMPLETE LAW in
/// three dimensions — every leg or refuse:
///
///   1. **re-forge** — the task IS its digest under this rung, pure and
///      deterministic, no external data ever. Volumetric work is
///      forge-only, so this single leg does what corpus v2 needed a
///      Merkle root and an inclusion path to do.
///   2. **re-apply** — the program maps EVERY training pair exactly and
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
        VolumeCertificateRead::Diverged { at_step } => {
            Err(WorkBroken::ChainDiverged { at_step })
        }
    }
}

/// Decode a CORPUS proof (schema v2) — the FORM only.
pub fn decode_corpus_work(bytes: &[u8]) -> Option<CorpusProof> {
    let mut r = Reader { bytes, at: 0 };
    if r.byte()? != WORK_SCHEMA_V2_CORPUS {
        return None;
    }
    let task_id = r.digest()?;
    let train_len = r.u16()?;
    // THE VACUITY, refused for the new shape too (98ccbbe's law): a
    // replay over no training pair proves nothing, and "maps EVERY
    // pair" is vacuously true over none. v1 refuses this through
    // `Difficulty::well_formed`; v2 has no difficulty, so it refuses here.
    if train_len == 0 {
        return None;
    }
    let mut train = Vec::with_capacity(train_len);
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

/// THE JUDGE'S CALL, class-aware — verify ANY work payload and report
/// what it turned out to be. Pure, total, no I/O.
///
/// Dispatch is on the leading schema byte, so v1 and v2 can never be
/// confused for one another and a future v3 costs one arm.
///
/// **The kernel should call THIS**, not [`verify_work`]: only this
/// entry point can express corpus work, whose identity is a task_id and
/// not a ladder rung. Relayed to the kernel lane along with the
/// `mint_bound` arm that prices it.
pub fn verify_work_class(bytes: &[u8]) -> Result<WorkClass, WorkBroken> {
    match bytes.first() {
        Some(&WORK_SCHEMA_V1) => verify_work(bytes).map(WorkClass::Forge),
        Some(&WORK_SCHEMA_V2_CORPUS) => verify_corpus_work(bytes),
        Some(&WORK_SCHEMA_V3_VOLUME) => verify_volume_work(bytes),
        _ => Err(WorkBroken::Malformed),
    }
}

/// THE CORPUS JUDGE (work-shapes v2). Three legs, all or refuse:
///
///   1. **re-hash** — the carried task hashes to the claimed `task_id`;
///      a payload that names one task and carries another refuses.
///   2. **re-apply** — the program maps EVERY training pair exactly and
///      the test input to the claimed output (the honest-withhold law).
///   3. **re-chain** — the certificate re-derives Intact.
///
/// **THE ADMIT LEG WAS STRUCK 2026-08-04** (operator ruling: no Merkle
/// trees). It checked an inclusion path against a pinned root, which is
/// a static enumeration AND an authority — see `crate::corpus` for the
/// reasoning kept whole. Its replacement is a PREDICATE, not a list: a
/// claim is admissible when the BASE vocabulary provably fails the task
/// at depth `d` and the candidate closes it, both halves replayable
/// with the machinery already here.
///
/// **So this gate does NOT admit, and must not be read as if it did.**
/// It proves a payload is internally consistent and that the program
/// closes the carried task. Whether the task is one anybody authorized
/// is a question it no longer asks — lawfully, because corpus work has
/// no rate either: THE MOTION clause 4 makes imported content a
/// BENCHMARK and never a mint source. It carries no seed, it can be
/// authored, and its held-out answer is not checkable by the gate.
///
/// **What this CANNOT check, stated plainly and unchanged by the
/// strike.** A corpus task's held-out answer is not carried and not
/// derivable, so leg 2 proves the program closes the TRAINING pairs and
/// renders the claimed output — never that the output is CORRECT.
/// Measured: of 34 programs that closed training, 33 matched the
/// held-out answer and one did not, so ~3% of closures are confidently
/// wrong. That was the fabrication surface a corpus rate would have
/// bought, and it is now one more reason there is no such rate.
pub fn verify_corpus_work(bytes: &[u8]) -> Result<WorkClass, WorkBroken> {
    let proof = decode_corpus_work(bytes).ok_or(WorkBroken::Malformed)?;
    // 0. THE PATH IS CARRIED, OR THERE IS NO CLAIM (operator directive
    //    2026-08-07). Everything below RE-APPLIES what the claimant
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
        CertificateRead::Intact => Ok(WorkClass::Corpus { task_id: proof.task_id }),
        CertificateRead::Diverged { at_step } => Err(WorkBroken::ChainDiverged { at_step }),
    }
}

/// THE JUDGE'S CALL — verify a FORGE payload whole. Pure, total,
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
    let proof = decode_work(bytes).ok_or(WorkBroken::Malformed)?;
    // 0. THE PATH IS CARRIED, OR THERE IS NO CLAIM (operator directive
    //    2026-08-07). The forge carrier mints too, so the law is the
    //    same law: a claim states its resolution, and the gate
    //    re-applies it in time linear in the path. Nothing here
    //    searches.
    if proof.program.is_empty() {
        return Err(WorkBroken::PathAbsent);
    }
    // 1. Re-forge: the task IS its digest under this difficulty.
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
