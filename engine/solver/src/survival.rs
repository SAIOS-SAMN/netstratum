//! **THE SURVIVAL SEAT** (N-I5, THE INTELLIGENCE PORT) — books become a
//! `FullMeasurement`, mouths eat. The SECOND seat of the ONE reduce organ: where
//! the ARC seat (`crate::think`) peels a grid pair, this peels the RATION BOOK —
//! HAVE and OWE as two occupancies of the SAME fiber (resource indices as
//! `GaugeDirection`), the working mouths as sites. `reduce_pair` reads whether
//! the larder covers the mouths (`Fed`) or a mouth is short (`Short`). There is
//! no `BookThought`, no `choose_zone`, no second solver — the same
//! `signed_remainder` / `reduce_pair` the grid seat runs.
//!
//! **The species is the COLUMN.** The owed direction IS the species's ration —
//! a plasmoid owes joule, a biotic owes biomass, an element-eater owes its
//! element — read from the caller as `ration_dir`, never a hardcoded Biomass.
//! Identity lives in the measurement (the same law as the sister trees' roster
//! peel), not in a branch.
//!
//! This landing is the OBSERVATION only: `Fed` lets the beat think ARC, `Short`
//! withholds it (Pin G). Naming a survival DEED from the residual is N-I6.

use iae::peel::{CellFrame, FullMeasurement, GaugeDirection, Site};
use iae::reduction::{reduce_pair, ReduceVerdict};

/// The unfed direction — a mouth the larder does not cover carries this, not the
/// ration. Reserved `0` so every ration direction (a `Resource` discriminant,
/// `1..`, or the element slot) is distinct from it.
pub const UNFED: u8 = 0;

/// The survival reading — the ration book either closes (every working mouth
/// fed) or a mouth is short. The ONLY verdict the seat renders; the beat thinks
/// ARC only on `Fed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RationRead {
    /// The larder covers every mouth — the book closes; research may run.
    Fed,
    /// A mouth is short — the book does not close; the beat withholds (Pin G).
    Short,
}

/// Peel the ration book of one line into HAVE and OWE over a mouths-as-sites
/// frame, and reduce the pair with the ONE organ.
///
/// - `mouths` — the working force (`labor_force`); one site per mouth.
/// - `fed` — how many of them the larder covers (⌊available / per-mouth⌋),
///   capped here at `mouths` (surplus feeds no phantom mouth).
/// - `ration_dir` — the species's owed direction (≥ 1; `UNFED` is reserved).
///
/// OWE seats EVERY mouth at the ration direction (each mouth owes its ration).
/// HAVE seats the fed mouths at the ration direction and the rest at `UNFED`.
/// `reduce_pair` reads `Congruent` when have == owe (every mouth fed → `Fed`)
/// and a `Remainder` when a mouth is short (an owed ration `arrived` where have
/// carries only `UNFED` → `Short`). Zero mouths is a whole empty pair — nothing
/// to feed closes trivially (`Fed`).
pub fn ration_reads(mouths: u64, fed: u64, ration_dir: u8) -> RationRead {
    debug_assert!(ration_dir != UNFED, "the ration direction must be distinct from UNFED");
    let fed = fed.min(mouths);
    let frame = CellFrame { rows: 1, cols: mouths as u32 };
    let fiber = ration_dir.saturating_add(1);
    let owe_occ: Vec<(Site, GaugeDirection)> = (0..mouths as i64)
        .map(|c| (Site { row: 0, col: c }, GaugeDirection { index: ration_dir }))
        .collect();
    let have_occ: Vec<(Site, GaugeDirection)> = (0..mouths as i64)
        .map(|c| {
            let index = if (c as u64) < fed { ration_dir } else { UNFED };
            (Site { row: 0, col: c }, GaugeDirection { index })
        })
        .collect();
    let owe = FullMeasurement::try_new(fiber, frame, owe_occ).expect("owe is whole (dense mouths)");
    let have =
        FullMeasurement::try_new(fiber, frame, have_occ).expect("have is whole (dense mouths)");
    match reduce_pair(&have, &owe, &[]) {
        ReduceVerdict::Congruent { .. } => RationRead::Fed,
        _ => RationRead::Short,
    }
}
