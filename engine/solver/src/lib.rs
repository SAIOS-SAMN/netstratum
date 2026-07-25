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

pub mod certificate;
pub mod forge;
pub mod grid;
pub mod loader;
pub mod search;

pub use certificate::{verify_certificate, CertificateRead, SolveCertificate};
pub use forge::{forge_task, Difficulty};
pub use grid::{Grid, Op, Program, TileMirror};
pub use loader::{load_task, LoadError, Task, TaskPair};
pub use search::{solve, SolveRead};
