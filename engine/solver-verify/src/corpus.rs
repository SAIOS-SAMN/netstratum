//! CORPUS TASK IDENTITY — content addressing, and nothing more.
//!
//! **THE MERKLE ADMISSION MECHANISM IS STRUCK** (operator ruling
//! 2026-08-04). It stood for one morning. Recorded rather than deleted
//! silently, because the failure is instructive and the reasoning that
//! produced it was half right.
//!
//! ## What was here, and why it went
//!
//! Corpus work raised a question v1 never had: a corpus task is
//! observed content, cannot be re-derived from a digest, and so must
//! ride in the payload — at which point the judge must ask *is this a
//! real task, or one the miner wrote?* The finding that content
//! addressing does not answer that question stands and is not what was
//! struck: **`task_id = H(content)` proves WHICH task was solved, never
//! that the task was ADMISSIBLE.** Identity is not admission.
//!
//! What was struck is the ANSWER: a static Merkle tree over the
//! admissible corpus with its root pinned as a constant. Two rulings
//! killed it, and the second follows from the first:
//!
//! 1. **The engine must be able to FORGE NEW CORPUS** — continuous
//!    novel work is what PoW++ runs on.
//! 2. **No Merkle trees.**
//!
//! In this tree's own terms: a pinned root is a static ENUMERATION — a
//! finite well, when ruling 1 asks for an open one. It is also an
//! AUTHORITY, a constant deciding what counts, which is a second
//! machine-readable truth of exactly the kind `00_CHARTER` §11 excised
//! the mirror for. And it cannot move at mesh speed: every node must
//! carry the same root, and changing it is a kernel VERSION amendment.
//!
//! ## What replaces it: a PREDICATE, not a list
//!
//! Admission becomes re-checkable rather than looked up — a claim is
//! admissible when the BASE vocabulary provably FAILS the task at depth
//! `d` and the candidate closes it. Both halves replay under machinery
//! `verify_work` already has: re-run the base search, re-run the
//! candidate. No root, no constant, no authority, and it moves as fast
//! as the mesh does. The shape is the nursery vocabulary-minting gate.
//!
//! Until that lands, corpus work has NO admission test — which is
//! lawful, because it also has no rate, and THE MOTION's clause 4 says
//! imported content is a BENCHMARK and never a mint source: the ARC
//! corpus carries no seed, can be authored, and its held-out answer is
//! not checkable by the judge. Measuring against it stays useful;
//! minting from it was never the plan this ruling took away.

use crate::grid::Grid;
use crate::loader::TaskPair;

/// Tag: the observable content of one corpus task.
const TAG_TASK: &[u8] = b"netstratum.corpus.task.v1";

/// THE TASK'S IDENTITY IS ITS CONTENT — and content means exactly the
/// OBSERVABLE task: every training pair in order, then the test input.
///
/// This survives the strike unchanged. It is what binds a payload to
/// the task it claims, so a proof that names one task and carries
/// another refuses. It says nothing about whether the task is one of
/// ours — that was always a separate question, and it is now answered
/// by a predicate rather than by a list.
///
/// Deliberately excluded:
/// - **the name.** A filename is provenance, not content; renaming a
///   corpus file must not change what the task IS.
/// - **`test_expected`.** The held-out answer never rides in a proof.
///   For real corpus work it may not even be known — which is the whole
///   point of the benchmark — and a judge that needed it could only
///   ever verify tasks whose answers were already public.
pub fn task_digest(train: &[TaskPair], test_in: &Grid) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(TAG_TASK);
    hasher.update(&(train.len() as u64).to_le_bytes());
    for pair in train {
        hash_grid(&mut hasher, &pair.input);
        hash_grid(&mut hasher, &pair.output);
    }
    hash_grid(&mut hasher, test_in);
    *hasher.finalize().as_bytes()
}

fn hash_grid(hasher: &mut blake3::Hasher, g: &Grid) {
    hasher.update(&(g.rows as u64).to_le_bytes());
    hasher.update(&(g.cols as u64).to_le_bytes());
    hasher.update(&g.cells);
}
