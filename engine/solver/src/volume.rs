//! N-VOLUME, prover side — the volumetric search and nothing else.
//!
//! **The split landed 2026-08-04** (operator ruling lifting C8's
//! deferral, mirroring U1a's planar split). Everything a replay needs
//! moved into `solver-verify::volume`: the `Volume` carrier, the `VOp`
//! vocabulary and its one apply path, the deterministic forge, the
//! degeneracy law, and a real parent-linked certificate. What stays
//! here is the search — expensive, free to improve, non-constitutional.
//!
//! **`seal_volume` is gone, not re-homed.** C8 ruled it decorative and
//! it was: a bare digest nobody could re-derive, tamper-evidence for no
//! one. Its replacement is `VolumeCertificate`, which a third party can
//! re-derive and whose divergence is named. A decorative digest is not
//! a thing to keep alongside a real one — two seals over one solve is
//! exactly how a verifier gets called with the wrong one.
//!
//! Volumetric tasks are forge-only and re-derivable, so the verifier
//! needs no admissible set: `verify_volume_work` is v1's shape in three
//! dimensions, strictly simpler than corpus v2's Merkle inclusion.
//!
//! The carrier types are re-exported whole, so every existing caller
//! (`conductor`, the pins) keeps compiling against `solver::` names.

pub use solver_verify::volume::verify_volume_certificate;
pub use solver_verify::volume::{
    forge_volume, run_volume, vocabulary, VOp, VProgram, VolDifficulty, Volume, VolumeCertificate,
    VolumeCertificateRead, VolumeTask,
};
pub use solver_verify::work::{
    decode_volume_work, encode_volume_work, verify_volume_work, VolumeProof,
};

/// The volumetric solve reading.
#[derive(Debug, Clone)]
pub enum VolumeSolveRead {
    Solved {
        program: VProgram,
        test_output: Volume,
        matches_expected: Option<bool>,
        /// The real certificate (2026-08-04): parent-linked over
        /// observation -> word -> output, re-derivable by anyone. It
        /// replaces the decorative digest C8 struck.
        certificate: VolumeCertificate,
    },
    Unsolved,
}

/// Enumerative search over volumetric words: a program is a solution
/// only if it closes every training pair exactly. The honest withhold
/// stands.
///
/// Superseded in spirit by the planar proposer's ruling (never
/// enumerate); the volumetric reach is not yet derived and this remains
/// a sweep. Recorded so the debt is visible rather than implied.
pub fn solve_volume(task: &VolumeTask, max_depth: usize) -> VolumeSolveRead {
    // The vacuous solve, REFUSED (operator ruling C8, U_DIRECTIVES §10
    // S1). `closes` is true over no pairs, so the empty program closed
    // an empty train at depth 0 and read Solved. Nothing was proved.
    // The verifier now refuses the same shape as form at decode, so
    // this guard and that one agree.
    if task.train.is_empty() {
        return VolumeSolveRead::Unsolved;
    }
    let vocabulary = vocabulary();

    let closes = |program: &VProgram| {
        task.train
            .iter()
            .all(|(input, expected)| &run_volume(program, input) == expected)
    };
    // Forge-bound: the same derived frontier as the planar path
    // (`search::frontier`) — depth is the bound, the candidate count is
    // its consequence. The flat `SEARCH_BUDGET` is decommissioned.
    let mut frontier: Vec<VProgram> = vec![Vec::new()];
    for depth in 0..=max_depth {
        for program in &frontier {
            if closes(program) {
                let test_output = run_volume(program, &task.test_in);
                let matches_expected = task.test_expected.as_ref().map(|want| want == &test_output);
                let certificate = VolumeCertificate::seal(task, program, &test_output);
                return VolumeSolveRead::Solved {
                    program: program.clone(),
                    test_output,
                    matches_expected,
                    certificate,
                };
            }
        }
        // The last depth grows nothing — see `search::solve`.
        if depth == max_depth {
            break;
        }
        let mut next = Vec::with_capacity(frontier.len() * vocabulary.len());
        for program in &frontier {
            for op in &vocabulary {
                let mut extended = program.clone();
                extended.push(op.clone());
                next.push(extended);
            }
        }
        frontier = next;
    }
    VolumeSolveRead::Unsolved
}
