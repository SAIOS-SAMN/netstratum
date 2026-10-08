//! The ARC solver harness (substrate contract §§1–3's first consumer;
//! the epistemological pyramid's apex made real).
//!
//! Three laws carried from the provenance (v0.4 doc 17, the no-hardcoding
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
//!   transformation → the output grid). The certificate is
//!   TAMPER-evidence (it re-derives the chain over the SAME triple);
//!   the constitutional work-verifier is `solver_verify::verify_work`,
//!   which additionally RE-FORGES the task and RE-APPLIES the program
//!   (THE REPLAY-COMPLETE LAW — an intact chain over a non-solution is
//!   not a solution). Finding is search; checking is O(program length).

pub mod derive;
pub mod instinct;
pub mod mint;
pub mod need;
pub mod perspective;
pub mod selfmap;
// N-I3 (THE INTELLIGENCE PORT) — the ARC seat: peel train grids into
// `reduct::FullMeasurement`, read them through the ONE reduce organ (`reduct::think`),
// render a glued frame to a program, seal the proof. NOT a search path.
pub mod think;
pub mod percept;
// docs/60 D1 — the channels a word acts in and a mind perceives.
pub mod senses;
// N-I5 (THE INTELLIGENCE PORT) — the survival seat: peel the ration book
// (HAVE|OWE, mouths as sites) through the SAME reduce organ. The observation
// that gates the beat (Pin G); naming a survival deed is N-I6.
pub mod survival;
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

pub use derive::{derive, Candidate};
pub use need::{Family, Hunger, Need};
pub use selfmap::{consilience, consilience_sensed, knowledge_map, knowledge_order, knowledge_tension, Consilience, SelfMap};
pub use think::{
    hear, hear_word, hold_conjectures, measure_grid, measure_volume, think_task, think_task_held,
    think_task_sensing, think_volume_task, Conjecture, Heard, SolveRead, HELD_CONJECTURES,
};
pub use survival::{ration_reads, think_survival, RationRead, SurvivalBook, SurvivalDeed};
// N-I4 — the cognitive basin's element, re-exported for the vessel layer
// (`conductor` grows a `Vec<Crystal>` per line and passes it to `think_task`).
pub use reduct::Crystal;
pub use volume::{
    forge_volume, run_volume, solve_volume, VOp, VolDifficulty, Volume, VolumeSolveRead,
    VolumeTask,
};
