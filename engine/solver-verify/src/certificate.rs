//! The proof-of-solution chain: a BLAKE3 parent-linked certificate —
//! task observation → each applied transformation → the rendered
//! output. Finding a solution is search (hard); verifying one is a
//! replay of the chain (cheap). The NS-3 shape, at the solver's
//! altitude; wiring into the world chronicle lands with the first
//! game consumer (law xvii).

use crate::grid::{Grid, Program};
use crate::loader::Task;

fn hash_grid(hasher: &mut blake3::Hasher, g: &Grid) {
    hasher.update(&(g.rows as u64).to_le_bytes());
    hasher.update(&(g.cols as u64).to_le_bytes());
    hasher.update(&g.cells);
}

/// The one grid-hashing law, shared with the volumetric sealer (which
/// lives prover-side in `solver` — hence pub across the split).
pub fn hash_grid_into(hasher: &mut blake3::Hasher, g: &Grid) {
    hash_grid(hasher, g);
}

fn link(parent: &[u8; 32], payload: impl FnOnce(&mut blake3::Hasher)) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(parent);
    payload(&mut hasher);
    *hasher.finalize().as_bytes()
}

/// The sealed certificate: the genesis digest binds the full
/// observation (name, every training pair, the test input); each step
/// digest binds one transformation; the final digest binds the
/// rendered output. Tampering with any link diverges every digest
/// after it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolveCertificate {
    pub genesis: [u8; 32],
    pub steps: Vec<[u8; 32]>,
    pub output: [u8; 32],
}

/// The typed verification reading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificateRead {
    Intact,
    Diverged { at_step: usize },
}

impl SolveCertificate {
    /// Seal a solution: observation → program word → output.
    pub fn seal(task: &Task, program: &Program, output: &Grid) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(task.name.as_bytes());
        for pair in &task.train {
            hash_grid(&mut hasher, &pair.input);
            hash_grid(&mut hasher, &pair.output);
        }
        hash_grid(&mut hasher, &task.test_in);
        let genesis = *hasher.finalize().as_bytes();

        let mut parent = genesis;
        let steps = program
            .iter()
            .map(|op| {
                parent = link(&parent, |h| {
                    h.update(op.describe().as_bytes());
                });
                parent
            })
            .collect();

        let output = link(&parent, |h| hash_grid(h, output));
        Self {
            genesis,
            steps,
            output,
        }
    }
}

/// Re-derive the chain from the same observation, program, and output;
/// any mismatch names the first diverged link (0 = genesis,
/// 1..=n = program steps, n+1 = output).
pub fn verify_certificate(
    certificate: &SolveCertificate,
    task: &Task,
    program: &Program,
    output: &Grid,
) -> CertificateRead {
    let expected = SolveCertificate::seal(task, program, output);
    if expected.genesis != certificate.genesis {
        return CertificateRead::Diverged { at_step: 0 };
    }
    for (i, (a, b)) in expected.steps.iter().zip(&certificate.steps).enumerate() {
        if a != b {
            return CertificateRead::Diverged { at_step: i + 1 };
        }
    }
    if expected.steps.len() != certificate.steps.len() || expected.output != certificate.output {
        return CertificateRead::Diverged {
            at_step: expected.steps.len().min(certificate.steps.len()) + 1,
        };
    }
    CertificateRead::Intact
}

/// **The carried derivation** (Concordance 4.4 annex, ruled 2026-10-08;
/// shared by every kernel and the court): a word's proof is a sequence of
/// steps `(constructor_k, digest of the state after step k)`. Step k is
/// valid when constructor_k applied to state k-1 yields a state whose digest
/// is the carried one. A checker walks the steps locally — it never re-seals
/// a certificate to compare, and a divergence names its step.
///
/// The STATE, plainly: the ordered grids a word acts on (for a task: every
/// training input, then the test input). Its digest is
///
/// ```text
/// BLAKE3( "netstratum.state.v1" ‖ count u16 ‖ (rows u16 ‖ cols u16 ‖ cells)·count )
/// ```
///
/// integers little-endian, one byte a cell (hue 0-9) in row-major order — the
/// same grid bytes as the work wire. A constructor is one op in the work
/// wire's op table, applied to every grid of the state.
pub const STATE_DOMAIN: &[u8] = b"netstratum.state.v1";

/// The digest of a state (see [`STATE_DOMAIN`]). `None` if a count or a
/// dimension exceeds u16 — the encoding never clamps.
pub fn state_digest(grids: &[Grid]) -> Option<[u8; 32]> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(STATE_DOMAIN);
    hasher.update(&u16::try_from(grids.len()).ok()?.to_le_bytes());
    for g in grids {
        hasher.update(&u16::try_from(g.rows).ok()?.to_le_bytes());
        hasher.update(&u16::try_from(g.cols).ok()?.to_le_bytes());
        hasher.update(&g.cells);
    }
    Some(*hasher.finalize().as_bytes())
}

/// A word's carried derivation: `states[0]` is the digest of the state the
/// word starts from, `states[k]` the digest after its k-th op.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Derivation {
    pub states: Vec<[u8; 32]>,
}

/// Derive a word over `start`: the carried derivation and the final state.
pub fn derive(program: &Program, start: &[Grid]) -> Option<(Derivation, Vec<Grid>)> {
    let mut state = start.to_vec();
    let mut states = vec![state_digest(&state)?];
    for op in program {
        state = state.iter().map(|g| op.apply(g)).collect();
        states.push(state_digest(&state)?);
    }
    Some((Derivation { states }, state))
}

/// Walk a carried derivation from `start`, one step at a time: state 0 must
/// hash to `states[0]`, and each op applied to the state before it must hash
/// to the carried digest after it. The final state, or the first diverged
/// step (`0` = the start state, `k` = the k-th op; a derivation whose length
/// does not match the word diverges at the first step it lacks or exceeds).
pub fn walk(
    derivation: &Derivation,
    program: &Program,
    start: &[Grid],
) -> Result<Vec<Grid>, usize> {
    match (derivation.states.first(), state_digest(start)) {
        (Some(carried), Some(start)) if *carried == start => {}
        _ => return Err(0),
    }
    let mut state = start.to_vec();
    for (k, op) in program.iter().enumerate() {
        let carried = derivation.states.get(k + 1).ok_or(k + 1)?;
        state = state.iter().map(|g| op.apply(g)).collect();
        if state_digest(&state).as_ref() != Some(carried) {
            return Err(k + 1);
        }
    }
    if derivation.states.len() != program.len() + 1 {
        return Err(program.len() + 1);
    }
    Ok(state)
}
