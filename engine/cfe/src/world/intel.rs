//! P5-C INF — veil/ward surface + MANU-1 detection grade (36 §13).
//!
//! Compose MASK-1 / WARDEN-1a / instruments. No free stealth points.
//! No new tags: pure readings + refuse paths on existing deeds.

use super::*;

impl World {
    /// MANU-1 detection grade: best forged instrument grade held (0 = none).
    pub fn detection_grade(&self) -> u8 {
        self.instruments
            .iter()
            .map(|(_, _, grade)| *grade)
            .max()
            .unwrap_or(0)
    }

    /// Count of forged instruments (INF / CYB surface).
    pub fn instrument_count(&self) -> usize {
        self.instruments.len()
    }

    /// Pure INF integrity for defend / suite.
    pub fn inf_integrity(&self) -> InfIntegrity {
        let grade = self.detection_grade();
        InfIntegrity {
            veiled: self.is_veiled(),
            ward_holds: self.ward_holds(),
            ward_state: match self.warded {
                None => WardState::Unseated,
                Some(false) => WardState::Active,
                Some(true) => WardState::Lifted,
            },
            detection_grade: grade,
            instrument_count: self.instrument_count(),
            veil_toll: crate::parameter::of(self.chronicle(), crate::parameter::Parameter::VeilToll),
            inf_rank: self.defense_posture().ranks[4], // INF domain index
        }
    }

    /// Grade required to open a prospect stratum (compose depth = grade).
    /// Stratum 0 is free surface; deeper strata demand instrument grade ≥ stratum.
    pub fn detection_need_for_stratum(stratum: u8) -> u8 {
        stratum
    }

    /// Grade required to scan while veiled: at least grade 1 instrument.
    pub fn detection_need_veiled_scan() -> u8 {
        1
    }

    /// Refuse path: detection too thin for the asked work.
    pub fn require_detection(&self, need: u8) -> Result<(), ActionRefused> {
        let have = self.detection_grade();
        if have < need {
            return Err(ActionRefused::DetectionThin { need, have });
        }
        Ok(())
    }
}

/// Ward seat state (WARDEN-1a).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WardState {
    /// No ElderWarded deed yet.
    Unseated,
    /// Ward stands (aperture locked).
    Active,
    /// Ward was lifted after solve.
    Lifted,
}

impl WardState {
    pub fn word(self) -> &'static str {
        match self {
            WardState::Unseated => "—",
            WardState::Active => "WARD",
            WardState::Lifted => "lifted",
        }
    }
}

/// INF integrity snapshot (pure).
#[derive(Debug, Clone)]
pub struct InfIntegrity {
    pub veiled: bool,
    pub ward_holds: bool,
    pub ward_state: WardState,
    pub detection_grade: u8,
    pub instrument_count: usize,
    pub veil_toll: Ratio,
    /// Posture INF rank 1..=3.
    pub inf_rank: u8,
}
