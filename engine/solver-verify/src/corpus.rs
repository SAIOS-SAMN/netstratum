//! Corpus TASK IDENTITY — content addressing, and nothing more.
//!
//! **The merkle admission mechanism is struck** (operator ruling
//! 2026-08-04). It stood for one morning. Recorded rather than deleted
//! silently, because the failure is instructive and the reasoning that
//! produced it was half right.
//!
//! ## What was here, and why it went
//!
//! Corpus work raised a question v1 never had: a corpus task is
//! observed content, cannot be re-derived from a digest, and so must
//! ride in the payload — at which point the gate must ask *is this a
//! real task, or one the miner wrote?* The finding that content
//! addressing does not answer that question stands and is not what was
//! struck: **`task_id = H(content)` proves which task was solved, never
//! that the task was admissible.** Identity is not admission.
//!
//! What was struck is the answer: a static Merkle tree over the
//! admissible corpus with its root pinned as a constant. Two rulings
//! removed it, and the second follows from the first:
//!
//! 1. **The engine must be able to forge new corpus** — continuous
//!    novel work is what PoW++ runs on.
//! 2. **No Merkle trees.**
//!
//! In this tree's own terms: a pinned root is a static ENUMERATION — a
//! finite well, when ruling 1 asks for an open one. It is also an
//! authority, a constant deciding what counts, which is a second
//! machine-readable truth of exactly the kind `00_CHARTER` §11 excised
//! the mirror for. And it cannot move at mesh speed: every node must
//! carry the same root, and changing it is a kernel VERSION amendment.
//!
//! ## What replaces it: the carried path
//!
//! **Operator rulings 2026-08-07, and they settle both halves.**
//!
//! **1. The Carrier Switch supersedes the motion clause 4.** Imported
//! content is a mint source. Clause 4 held that the ARC corpus was a
//! benchmark and never a mint source, and this module justified having
//! no admission test on the grounds that corpus work *"also has no
//! rate"*. That premise expired the day it gained one: kernel 0.14.0
//! prices corpus work from the measured carrier table, and a solve
//! mints at `W(τ) = scarcity × complexity`. The clause and the
//! justification are struck; what stood on them is gone with them.
//!
//! **2. Verify-time search is prohibited.** The predicate drafted here
//! — *the base vocabulary provably fails at depth `d` and the candidate
//! closes it* — is struck for its first half. Re-running a base search
//! at the gate prices admission at the cost of a search, and
//! `verify_work` is called on every judged mint.
//!
//! So admission is neither a list nor a search. **The claimant carries
//! the complete resolution path, and the gate re-applies it.** Every
//! check in `verify_corpus_work` is linear in the path's own length:
//! re-hash the task, re-apply the program to each pair, re-run it on
//! the test input, re-chain the certificate. Nothing is looked up and
//! nothing is discovered. A claim carrying no path is refused
//! `PathAbsent` — refused rather than admitted, because the empty
//! program is the identity and would otherwise verify a task whose
//! pairs happen to be identity while proving nothing.
//!
//! What this costs, stated plainly: the gate confirms that a claimed
//! resolution holds, never that it was hard to find. Difficulty is
//! priced by the measured carrier table instead — which is what
//! measurement is for, and is why the rate had to land before the
//! admission question could be answered this way.

use crate::grid::Grid;
use crate::loader::TaskPair;

/// Tag: the observable content of one corpus task.
const TAG_TASK: &[u8] = b"netstratum.corpus.task.v1";

/// The task's IDENTITY is its content — and content means exactly the
/// observable task: every training pair in order, then the test input.
///
/// This survives the strike unchanged. It is what binds a payload to
/// the task it claims, so a proof that names one task and carries
/// another refuses. It says nothing about whether the task is one of
/// Netstratum's — that was always a separate question, and it is now answered
/// by a predicate rather than by a list.
///
/// Deliberately excluded:
/// - **the name.** A filename is provenance, not content; renaming a
///   corpus file must not change what the task is.
/// - **`test_expected`.** The held-out answer never rides in a proof.
///   For real corpus work it may not even be known — which is the whole
///   point of the benchmark — and a gate that needed it could only
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
