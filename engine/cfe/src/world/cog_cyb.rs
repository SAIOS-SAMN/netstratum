//! P5-E COG + CYB — cognitive load reading, instrument guard, mesh guard (36 §2).
//!
//! Compose WANT-1 / PRACTICE-1 / MANU-1 / aperture. Zero new tags.

use super::*;
use crate::dop::Resource;

impl World {
    /// COG integrity: open wants + adopted practices + posture COG rank.
    pub fn cog_integrity(&self) -> CogIntegrity {
        let p = self.defense_posture();
        let want_load = self.wants.len() as u64;
        let practice_load = self.practices.len() as u64;
        // Load stress when wants exceed practices + 1 (1+1 identity).
        let stressed = want_load > practice_load.saturating_add(1);
        CogIntegrity {
            want_load,
            practice_load,
            stressed,
            cog_rank: p.ranks[2],
            voice: if stressed {
                "want load exceeds practice — doctrine under strain"
            } else if want_load == 0 {
                "quiet mind — no open wants"
            } else {
                "wants held under practice"
            },
        }
    }

    /// CYB integrity: instrument floor + mesh surface.
    pub fn cyb_integrity(&self) -> CybIntegrity {
        let p = self.defense_posture();
        let grade = self.detection_grade();
        let routes = self.routes.len() as u64;
        let stamp = self
            .orb()
            .map(|o| o.stamp_amount())
            .unwrap_or_else(|| ratio(0, 1));
        CybIntegrity {
            detection_grade: grade,
            instrument_count: self.instrument_count(),
            routes_open: routes,
            stamp_banked: stamp > ratio(0, 1),
            veiled: self.is_veiled(),
            ward_holds: self.ward_holds(),
            cyb_rank: p.ranks[3],
            mesh_ready: routes > 0 && !self.ward_holds(),
            voice: if grade == 0 {
                "no instrument floor — mesh reads thin"
            } else if self.ward_holds() {
                "ward holds — mesh aperture sealed"
            } else if routes == 0 {
                "instruments ready — no lanes chartered"
            } else {
                "instrument + mesh surface green"
            },
        }
    }

    /// P5.21 instrument-guard: refuse re-forge of identical grade/node on orb
    /// (anti-spoof — one grade per tech node per orb).
    pub fn instrument_guard_ok(&self, orb: u64, tech_hash: [u8; 32], grade: u8) -> Result<(), ActionRefused> {
        if self
            .instruments
            .iter()
            .any(|(o, h, g)| *o == orb && *h == tech_hash && *g == grade)
        {
            return Err(ActionRefused::InstrumentSpoof {
                orb,
                grade,
            });
        }
        Ok(())
    }

    /// P5.22 mesh-guard: veiled positive-melt aperture needs detection
    /// grade ≥ 1 (instrument witnesses the corridor under OPSEC).
    /// Unveiled trade stays open (R1–R2 fun); local zero-melt free.
    pub fn mesh_guard_haul(&self, melt: &Ratio) -> Result<(), ActionRefused> {
        if *melt > ratio(0, 1) && self.is_veiled() {
            self.require_detection(1)?;
        }
        Ok(())
    }

    /// STAMP clear witness hygiene: last-clear must meet seal OR Exchange
    /// seated before STAMP aperture is "honest" under mesh guard reading.
    /// Pure reading for suite; hard refuse optional on stamp haul path.
    pub fn stamp_mesh_hygiene(&self) -> bool {
        self.exchange_seated()
            || self.clear_depth_of(Resource::Crystal) >= CLEAR_SEAL_DEPTH / 2
            || self.detection_grade() >= 1
    }
}

/// COG integrity snapshot.
#[derive(Debug, Clone)]
pub struct CogIntegrity {
    pub want_load: u64,
    pub practice_load: u64,
    pub stressed: bool,
    pub cog_rank: u8,
    pub voice: &'static str,
}

/// CYB integrity snapshot.
#[derive(Debug, Clone)]
pub struct CybIntegrity {
    pub detection_grade: u8,
    pub instrument_count: usize,
    pub routes_open: u64,
    pub stamp_banked: bool,
    pub veiled: bool,
    pub ward_holds: bool,
    pub cyb_rank: u8,
    pub mesh_ready: bool,
    pub voice: &'static str,
}
