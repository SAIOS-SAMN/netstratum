//! The proof-of-solution chain: a BLAKE3 parent-linked certificate —
//! task observation → each applied transformation → the rendered
//! output. Finding a solution is search (hard); verifying one is a
//! replay of the chain (cheap). The NS-3 shape, at the solver's
//! altitude; wiring into the world chronicle lands with the first
//! game consumer (LAW XVII).

use crate::grid::{Grid, Program};
use crate::loader::Task;

fn hash_grid(hasher: &mut blake3::Hasher, g: &Grid) {
    hasher.update(&(g.rows as u64).to_le_bytes());
    hasher.update(&(g.cols as u64).to_le_bytes());
    hasher.update(&g.cells);
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
    if expected.steps.len() != certificate.steps.len()
        || expected.output != certificate.output
    {
        return CertificateRead::Diverged {
            at_step: expected.steps.len().min(certificate.steps.len()) + 1,
        };
    }
    CertificateRead::Intact
}
