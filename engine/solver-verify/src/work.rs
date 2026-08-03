//! THE WORK PROOF — the opaque payload's one form and its judge
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
use crate::corpus::CORPUS_ROOT;
use crate::forge::{forge_task, Difficulty};
use crate::grid::{run, Grid, Op, Program, TileMirror};
use crate::loader::{Task, TaskPair};
use crate::objects::ObjOp;

/// The work-proof wire schema. A bump is an amendment.
///
/// Schema bytes are APPEND-ONLY — one code, one meaning forever. v1 is
/// untouched by v2: the same v1 bytes decode to the same v1 proof, and
/// a v1 payload never grows a field.
pub const WORK_SCHEMA_V1: u8 = 1;

/// Work-shapes v2 — CORPUS work (operator ruling 2026-08-04). A corpus
/// task is observed content and cannot be re-derived from a digest, so
/// it rides WHOLE, carrying its admissibility proof with it.
pub const WORK_SCHEMA_V2_CORPUS: u8 = 2;

/// The decoded proof: everything the replay needs, nothing else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkProof {
    pub forge_digest: [u8; 32],
    pub difficulty: Difficulty,
    pub program: Program,
    pub output: Grid,
    pub certificate: SolveCertificate,
}

/// A CORPUS proof (schema v2): the task rides whole, with the proof
/// that it belongs to the admissible set.
///
/// The task's NAME is not carried. It is not content (see
/// [`crate::corpus::task_digest`]), and the certificate needs a name to
/// seal, so both halves derive the same canonical one from the task_id:
/// [`canonical_name`]. A miner cannot vary it, and nothing depends on
/// what a corpus file happened to be called.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusProof {
    /// The content digest — `corpus::task_digest(train, test_in)`.
    pub task_id: [u8; 32],
    /// The Merkle path from this leaf to the pinned root.
    pub inclusion: Vec<crate::corpus::PathStep>,
    /// The observed task, riding whole: it cannot be re-derived.
    pub train: Vec<TaskPair>,
    pub test_in: Grid,
    pub program: Program,
    pub output: Grid,
    pub certificate: SolveCertificate,
}

/// What a verified payload turned out to BE. v1 returns a ladder rung;
/// corpus work has no rung — its identity is its task_id — so the judge
/// reports the class and the kernel prices it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkClass {
    /// Forge work: re-derivable, priced on the ladder by `mint_bound`.
    Forge(Difficulty),
    /// Corpus work: admissible, replayed, and OFF the ladder entirely.
    AdmissibleCorpus { task_id: [u8; 32] },
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
    /// v2 — the task_id is not in the admissible corpus: its inclusion
    /// path does not resolve to the pinned root. THE FORGED-TASK
    /// REFUSAL — an honestly-hashed, honestly-solved task that nobody
    /// authorized still earns nothing.
    NotAdmissible,
    /// The payload is a well-formed shape this judge cannot price. Not
    /// a malformation — a SHAPE the caller has not been taught. Today:
    /// corpus work reaching `verify_work`, whose return type can only
    /// express a ladder rung. `verify_work_class` accepts it.
    UnsupportedShape,
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
fn put_program(program: &Program, out: &mut Vec<u8>) -> Option<()> {
    put_u16(program.len(), out)?;
    for op in program {
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
    put_u16(proof.inclusion.len(), &mut out)?;
    for step in &proof.inclusion {
        out.push(u8::from(step.counterpart_is_left));
        out.extend_from_slice(&step.counterpart);
    }
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
    /// The ONE program decoding, shared by every schema — the mirror of
    /// `put_program`. Unknown op codes refuse.
    fn program(&mut self) -> Option<Program> {
        let op_count = self.u16()?;
        let mut program: Program = Vec::with_capacity(op_count);
        for _ in 0..op_count {
            program.push(match self.byte()? {
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
    // The judge never re-forges a degenerate task: it refuses first.
    // The condition now carries a NAME (`Difficulty::lawful`) so the
    // v2 corpus decode path calls the same law instead of restating it.
    if !difficulty.lawful() {
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

/// Decode a CORPUS proof (schema v2) — the FORM only.
pub fn decode_corpus_work(bytes: &[u8]) -> Option<CorpusProof> {
    let mut r = Reader { bytes, at: 0 };
    if r.byte()? != WORK_SCHEMA_V2_CORPUS {
        return None;
    }
    let task_id = r.digest()?;
    let path_len = r.u16()?;
    let mut inclusion = Vec::with_capacity(path_len);
    for _ in 0..path_len {
        let counterpart_is_left = match r.byte()? {
            0 => false,
            1 => true,
            _ => return None, // the direction bit is exact, never truthy
        };
        inclusion.push(crate::corpus::PathStep {
            counterpart: r.digest()?,
            counterpart_is_left,
        });
    }
    let train_len = r.u16()?;
    // THE VACUITY, refused for the new shape too (98ccbbe's law): a
    // replay over no training pair proves nothing, and "maps EVERY
    // pair" is vacuously true over none. v1 refuses this through
    // `Difficulty::lawful`; v2 has no difficulty, so it refuses here.
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
        inclusion,
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
        _ => Err(WorkBroken::Malformed),
    }
}

/// THE CORPUS JUDGE (work-shapes v2). Four legs, all or refuse:
///
///   1. **re-hash** — the carried task hashes to the claimed `task_id`;
///      a payload that names one task and carries another refuses.
///   2. **admit** — the inclusion path resolves to [`CORPUS_ROOT`].
///      THIS IS THE NEW LEG, and the one v1 never needed: it is what
///      stops a miner authoring their own trivial task, hashing it
///      honestly, solving it honestly, and minting.
///   3. **re-apply** — the program maps EVERY training pair exactly and
///      the test input to the claimed output (the honest-withhold law).
///   4. **re-chain** — the certificate re-derives Intact.
///
/// **What this CANNOT check, stated plainly.** A corpus task's held-out
/// answer is not carried and not derivable, so leg 3 proves the program
/// closes the TRAINING pairs and renders the claimed output — never
/// that the output is CORRECT. That gap is not a defect in the judge;
/// it is the actual ARC problem, and the reason a corpus rate is an
/// economics question rather than a mechanical one. Measured on this
/// corpus: of 34 programs that closed training, 33 matched the held-out
/// answer and one did not — so roughly 3% of closures are confidently
/// wrong, and that is the fabrication surface a corpus rate buys.
pub fn verify_corpus_work(bytes: &[u8]) -> Result<WorkClass, WorkBroken> {
    verify_corpus_work_against(bytes, &CORPUS_ROOT)
}

/// The corpus judge against a NAMED root.
///
/// [`verify_corpus_work`] is this against [`CORPUS_ROOT`]. The root is a
/// parameter for two reasons, both structural rather than cosmetic:
/// the operator's ruling places the constant in the kernel, and this is
/// the seam that lets it be hoisted there without the kernel ever
/// parsing a payload (THE OPAQUE-PAYLOAD LAW holds — the kernel would
/// pass a 32-byte constant down, not read bytes); and a corpus
/// amendment can be verified against the incoming root before it is
/// pinned. Tests use it to exercise the legs BEYOND admissibility,
/// which the pinned root would otherwise short-circuit.
pub fn verify_corpus_work_against(
    bytes: &[u8],
    root: &[u8; 32],
) -> Result<WorkClass, WorkBroken> {
    let proof = decode_corpus_work(bytes).ok_or(WorkBroken::Malformed)?;
    // 1. Re-hash: identity is content, so recompute it from content.
    if crate::corpus::task_digest(&proof.train, &proof.test_in) != proof.task_id {
        return Err(WorkBroken::TaskIdMismatch);
    }
    // 2. Admit: pure arithmetic against the pinned root — no corpus,
    //    no filesystem, nothing the kernel could not do in a vacuum.
    if !crate::corpus::verify_inclusion(&proof.task_id, &proof.inclusion, root) {
        return Err(WorkBroken::NotAdmissible);
    }
    // The task, reconstructed exactly as the prover must have built it.
    let task = Task {
        name: canonical_name(&proof.task_id),
        train: proof.train.clone(),
        test_in: proof.test_in.clone(),
        test_expected: None,
    };
    // 3. Re-apply.
    for (pair, held) in task.train.iter().enumerate() {
        if run(&proof.program, &held.input) != held.output {
            return Err(WorkBroken::TrainingMiss { pair });
        }
    }
    if run(&proof.program, &task.test_in) != proof.output {
        return Err(WorkBroken::OutputMismatch);
    }
    // 4. Re-chain.
    match verify_certificate(&proof.certificate, &task, &proof.program, &proof.output) {
        CertificateRead::Intact => Ok(WorkClass::AdmissibleCorpus { task_id: proof.task_id }),
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
    if bytes.first() == Some(&WORK_SCHEMA_V2_CORPUS) {
        return Err(WorkBroken::UnsupportedShape);
    }
    let proof = decode_work(bytes).ok_or(WorkBroken::Malformed)?;
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
