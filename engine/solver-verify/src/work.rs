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
use crate::forge::{forge_task, Difficulty};
use crate::grid::{run, Grid, Op, Program, TileMirror};
use crate::objects::ObjOp;

/// The work-proof wire schema. A bump is an amendment.
pub const WORK_SCHEMA_V1: u8 = 1;

/// The decoded proof: everything the replay needs, nothing else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkProof {
    pub forge_digest: [u8; 32],
    pub difficulty: Difficulty,
    pub program: Program,
    pub output: Grid,
    pub certificate: SolveCertificate,
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
    put_u16(proof.program.len(), &mut out)?;
    for op in &proof.program {
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
    put_grid(&proof.output, &mut out)?;
    out.extend_from_slice(&proof.certificate.genesis);
    put_u16(proof.certificate.steps.len(), &mut out)?;
    for step in &proof.certificate.steps {
        out.extend_from_slice(step);
    }
    out.extend_from_slice(&proof.certificate.output);
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
    if difficulty.rows == 0 || difficulty.cols == 0 || difficulty.train_pairs == 0 {
        return None;
    }
    let op_count = r.u16()?;
    let mut program: Program = Vec::with_capacity(op_count);
    for _ in 0..op_count {
        program.push(match r.byte()? {
            1 => Op::Rotate90,
            2 => Op::Rotate180,
            3 => Op::Rotate270,
            4 => Op::FlipH,
            5 => Op::FlipV,
            6 => Op::Transpose,
            7 => Op::Recolor(r.take(10)?.try_into().ok()?),
            8 => Op::Object(objop_from(r.byte()?)?),
            9 => Op::Tile {
                down: r.byte()? as usize,
                across: r.byte()? as usize,
                odd_row: mirror_from(r.byte()?)?,
                odd_col: mirror_from(r.byte()?)?,
            },
            _ => return None,
        });
    }
    let output = r.grid()?;
    let genesis = r.digest()?;
    let step_count = r.u16()?;
    let mut steps = Vec::with_capacity(step_count);
    for _ in 0..step_count {
        steps.push(r.digest()?);
    }
    let cert_output = r.digest()?;
    if r.at != bytes.len() {
        return None; // trailing bytes refuse — the payload is exact
    }
    Some(WorkProof {
        forge_digest,
        difficulty,
        program,
        output,
        certificate: SolveCertificate { genesis, steps, output: cert_output },
    })
}

/// THE JUDGE'S CALL — verify a work payload whole. Pure, total,
/// no I/O; the kernel hands the bytes here and never parses them.
pub fn verify_work(bytes: &[u8]) -> Result<Difficulty, WorkBroken> {
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
