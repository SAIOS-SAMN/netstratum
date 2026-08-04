//! N-VOLUME, PROVER SIDE — the volumetric SEARCH and nothing else.
//!
//! **THE SPLIT LANDED 2026-08-04** (operator ruling lifting C8's
//! deferral, mirroring U1a's planar split). Everything a REPLAY needs
//! moved into `solver-verify::volume`: the `Volume` carrier, the `VOp`
//! vocabulary and its one apply path, the deterministic forge, the
//! degeneracy law, and a REAL parent-linked certificate. What stays
//! here is the search — expensive, free to improve, non-constitutional.
//!
//! **`seal_volume` IS GONE, not re-homed.** C8 ruled it decorative and
//! it was: a bare digest nobody could re-derive, tamper-evidence for no
//! one. Its replacement is `VolumeCertificate`, which a third party can
//! re-derive and whose divergence is NAMED. A decorative digest is not
//! a thing to keep alongside a real one — two seals over one solve is
//! exactly how a verifier gets called with the wrong one.
//!
//! Volumetric tasks are forge-only and re-derivable, so the verifier
//! needs no admissible set: `verify_volume_work` is v1's shape in three
//! dimensions, strictly simpler than corpus v2's Merkle inclusion.
//!
//! The carrier types are re-exported whole, so every existing caller
//! (`conductor`, the pins) keeps compiling against `solver::` names.

pub use solver_verify::volume::{
    forge_volume, run_volume, vocabulary, VOp, VProgram, VolDifficulty, Volume, VolumeCertificate,
    VolumeCertificateRead, VolumeTask,
};
pub use solver_verify::work::{
    decode_volume_work, encode_volume_work, verify_volume_work, VolumeProof,
};
pub use solver_verify::volume::verify_volume_certificate;

/// The volumetric solve reading.
#[derive(Debug, Clone)]
pub enum VolumeSolveRead {
    Solved {
        program: VProgram,
        test_output: Volume,
        matches_expected: Option<bool>,
        /// The REAL certificate (2026-08-04): parent-linked over
        /// observation -> word -> output, re-derivable by anyone. It
        /// replaces the decorative digest C8 struck.
        certificate: VolumeCertificate,
    },
    Unsolved,
}

/// Enumerative search over volumetric words: a program is a solution
/// ONLY if it closes every training pair exactly. The honest withhold
/// stands.
///
/// SUPERSEDED IN SPIRIT by the planar proposer's ruling (never
/// enumerate); the volumetric reach is not yet derived and this remains
/// a sweep. Recorded so the debt is visible rather than implied.
pub fn solve_volume(task: &VolumeTask, max_depth: usize) -> VolumeSolveRead {
    // THE VACUOUS SOLVE, REFUSED (operator ruling C8, U_DIRECTIVES §10
    // S1). `closes` is true over no pairs, so the empty program closed
    // an empty train at depth 0 and read Solved. Nothing was proved.
    // The verifier now refuses the same shape as FORM at decode, so
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
                let certificate = VolumeCertificate::seal(task, program, &test_output);
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
