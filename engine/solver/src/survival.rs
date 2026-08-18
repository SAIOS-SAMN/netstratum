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

// ─────────────────────────────────────────────────────────────────────
// N-I6 — THE EXPRESS ARM: the residual NAMES one deed.
// ─────────────────────────────────────────────────────────────────────

/// The reserved housing fiber — habitat SUPPORT vs mouths and extractor DRAIN vs
/// habitats are peeled on their own direction, distinct from the ration column.
const HOUSING: u8 = 62;
/// The reserved matter fiber — alloy vs iron, the industry leftover (Pin F).
const MATTER: u8 = 63;

/// The closed survival-deed vocabulary — the survival twin of the ARC seat's
/// `Program`. Each is an INVERSE-CARRYING deed (raze inverts build; collect
/// inverts the draw just priced) the conductor expresses through a door that
/// already exists (`zone_as` / `raze_as` / the cycle `collect_as`). NOT a score,
/// NOT a `ZoneKind` (the conductor resolves those) — a leftover class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurvivalDeed {
    /// Pin H — the accrued larder already covers the shortfall; the cycle
    /// `collect` IS the deed. Express nothing extra.
    Collect,
    /// Founding (charter §3): a settled orb yields the ration nowhere — seat the
    /// unique zone whose `zone_yield` is the ration. The conductor resolves the
    /// `ZoneKind` from the ration column; the solver only names "the source."
    SeatRationSource,
    /// Pin E — housing support is short of the mouths; seat a Habitat.
    Habitat,
    /// Pin E — extractors outweigh the habitats and drain the food column; raze
    /// one. Never a THIRD extractor (that is the wrong column).
    RazeExtractor,
    /// Pin F — the ration is Fed and the matter leftover is alloy over iron.
    Synthesizer,
    /// Pin F — the ration is Fed and the matter leftover is iron over alloy.
    Smelter,
    /// No inverse-priced deed — rest (a withhold, never a sweep).
    Rest,
}

/// The peeled survival book — the exact readings the conductor hands the seat
/// (all integer mouth-/unit-equivalents; NO cfe types, NO ZoneKind). The solver
/// thinks the measurements; the conductor peeled the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurvivalBook {
    /// Zones whose `zone_yield` is the ration. `0` on a settled orb ⇒ founding.
    pub ration_source_sites: u64,
    /// Working mouths (`labor_force`) and how many the larder feeds this beat.
    pub mouths: u64,
    pub fed: u64,
    /// The species column — the owed ration direction (≥ 1).
    pub ration_dir: u8,
    /// The accrued larder, in mouth-equivalents (Pin H: covers the shortfall?).
    pub accrued: u64,
    /// Housing support (habitat capacity) and the standing habitat/extractor
    /// counts (Pin E: support vs mouths, then extractors vs habitats).
    pub habitat_support: u64,
    pub habitats: u64,
    pub extractors: u64,
    /// Whether Need opened the matter book this beat (only a Fed line may), and
    /// the matter fiber's two sites (Pin F).
    pub matter_opened: bool,
    pub alloy: u64,
    pub iron: u64,
}

/// Does `have` cover `owe` on `dir` — the ONE organ (`reduce_pair`) read as a
/// coverage: `have` fed mouths of an `owe`-mouth demand close iff `have ≥ owe`.
fn covers(have: u64, owe: u64, dir: u8) -> bool {
    matches!(ration_reads(owe, have, dir), RationRead::Fed)
}

/// **THINK THE SURVIVAL BOOK** — the residual names ONE deed, by a SEQUENCE of
/// `reduce_pair` peels (ration first, then — only when Short — accrued, housing,
/// drain; or — only when Fed — the matter book). Ration and matter are
/// SEQUENTIAL seats, never two train pairs of one `think`. A closed table, the
/// survival twin of `render_frame`: no disposition, no `Policy`, no score. Any
/// leg with no inverse-priced deed falls through to `Rest`.
pub fn think_survival(book: &SurvivalBook) -> SurvivalDeed {
    // Founding — a settled orb yields the ration nowhere. Seat the source first
    // (owe one production site on the ration fiber, have none). Charter §3.
    if book.ration_source_sites == 0 {
        return SurvivalDeed::SeatRationSource;
    }
    // Ration seat FIRST. Fed → the ration book names nothing; a Fed line may
    // open matter (Pin F); else rest and let ARC run.
    if covers(book.fed, book.mouths, book.ration_dir) {
        if book.matter_opened {
            let alloy_over = covers(book.alloy, book.iron, MATTER);
            let iron_over = covers(book.iron, book.alloy, MATTER);
            return match (alloy_over, iron_over) {
                (true, false) => SurvivalDeed::Synthesizer,
                (false, true) => SurvivalDeed::Smelter,
                _ => SurvivalDeed::Rest, // balanced — no leftover direction
            };
        }
        return SurvivalDeed::Rest;
    }
    // Short — the sequential ration-fiber peels: accrued (H), then housing (E
    // build), then drain (E raze). The FIRST that fires is the deed.
    let shortfall = book.mouths.saturating_sub(book.fed);
    if covers(book.accrued, shortfall, book.ration_dir) {
        return SurvivalDeed::Collect; // Pin H — the accrued larder settles at collect
    }
    if !covers(book.habitat_support, book.mouths, HOUSING) {
        return SurvivalDeed::Habitat; // Pin E — housing short of the mouths
    }
    if !covers(book.habitats, book.extractors, HOUSING) {
        return SurvivalDeed::RazeExtractor; // Pin E — extractors outweigh habitats
    }
    SurvivalDeed::Rest
}
