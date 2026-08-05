//! THE VERIFIER HALF — the prover/verifier split made constitutional
//! (U_DIRECTIVES §2, 16_UNIT §3).
//!
//! This crate holds exactly what a REPLAY needs and nothing a SEARCH
//! needs: the grid vocabulary and the one apply path (`grid::run`),
//! the task loader, the deterministic forge (a task re-derives from
//! its digest — no external data ever), and the certificate chain.
//! Finding a solution is search — expensive, free to improve,
//! NON-constitutional; it lives in `solver`, which depends on this
//! crate. Verifying one is a replay — cheap, pure, total; the kernel
//! calls it and never solves.
//!
//! The arrow law: `solver-verify` sits in the sealed math tier
//! (imports `physics` for the beacon, nothing above); `solver`,
//! `kernel`, and `conductor` point down at it; nothing here points up.

pub mod certificate;
pub mod corpus;
pub mod field;
pub mod forge;
pub mod grid;
pub mod loader;
pub mod objects;
pub mod rung;
pub mod shell;
pub mod volume;
pub mod work;

pub use certificate::{verify_certificate, CertificateRead, SolveCertificate};
pub use corpus::task_digest;
pub use field::{quarter_turn, Axis, Field, Word};
pub use rung::{forge_field, FieldTask, Rung};
pub use shell::{Ring, Shell, Turn};
pub use forge::{forge_task, Difficulty};
pub use grid::{run, Grid, Op, Program, TileMirror};
pub use loader::{load_task, LoadError, Task, TaskPair};
pub use objects::{background, segment, ObjOp, Object, OBJ_OPS};
pub use volume::{
    forge_volume, run_volume, verify_volume_certificate, VOp, VProgram, VolDifficulty, Volume,
    VolumeCertificate, VolumeCertificateRead, VolumeTask,
};
pub use work::{decode_work, encode_work, verify_work, WorkBroken, WorkProof, WORK_SCHEMA_V1};
pub use work::{
    canonical_name, decode_corpus_work, encode_corpus_work, verify_corpus_work,
    verify_work_class,
    CorpusProof, WorkClass, WORK_SCHEMA_V2_CORPUS,
};
pub use work::{
    decode_volume_work, encode_volume_work, verify_volume_work, VolumeProof,
    WORK_SCHEMA_V3_VOLUME,
};
