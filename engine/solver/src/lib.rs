//! The ARC solver harness (substrate contract §§1–3's first consumer;
//! the epistemological pyramid's apex made real).
//!
//! Three laws carried from the ancestry (v0.4 doc 17, the no-hardcoding
//! law) and the charter:
//!
//! - **No hardcoded solutions.** The engine holds a GENERAL vocabulary
//!   of exact transformations; task files are observed content. No
//!   branch on task identity exists on any solve path, and deleting
//!   any fixture changes no code path.
//! - **The honest withhold outranks a fabricated render.** A program
//!   is a solution ONLY if it maps every training pair exactly; the
//!   solver returns `Unsolved` rather than a guess.
//! - **Every solution is a proof, not an assertion.** A solve emits a
//!   BLAKE3 parent-linked certificate chain (task → each applied
//!   transformation → the output grid); `verify` replays it byte-exact.
//!   Finding a solution is search; checking one is O(program length).

pub mod search;
pub mod volume;

// The verifier half moved to `solver-verify` (the prover/verifier
// split, U_DIRECTIVES §2): grids + the FULL apply vocabulary (object
// ops included — a replay must apply every op a program may carry),
// the loader, the deterministic forge, the certificate chain. Only
// the SEARCH is prover-side. Re-exported whole so no caller breaks —
// `solver::grid::run` and the root names both stand.
pub use solver_verify::{certificate, forge, grid, loader, objects};
pub use solver_verify::{
    background, forge_task, load_task, segment, verify_certificate, CertificateRead,
    Difficulty, Grid, LoadError, ObjOp, Object, Op, Program, SolveCertificate, Task, TaskPair,
    TileMirror, OBJ_OPS,
};

pub use search::{solve, SolveRead};
pub use volume::{
    forge_volume, run_volume, solve_volume, VOp, VolDifficulty, Volume, VolumeSolveRead,
    VolumeTask,
};
