//! The world — the tick loop binding the organs. Actions that complete
//! CLOSE as receipts; per-tick accrual is workspace (unreceipted, never
//! persisted); the chronicle replays the committed universe from
//! sequence zero (CHARTER SEAL B).

use crate::dop::{ClosureEvent, ColonyOrb, Resource, Zone, ZoneKind};
use crate::ratio::Ratio;
use crate::vrp::{Attestation, Chronicle, Timekeeper, TickRead, WorldClock};

/// A typed action refusal — the world refuses instead of guessing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionRefused {
    NoColony,
    ColonyAlreadyFounded,
    TileAlreadyZoned { tile: u16 },
    NothingAccrued,
}

/// The committed projection — exactly what the chronicle grounds.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Committed {
    pub orb: Option<ColonyOrb>,
}

pub struct World {
    clock: WorldClock,
    keepers: [Timekeeper; 3],
    chronicle: Chronicle,
    orb: Option<ColonyOrb>,
    /// WORKSPACE: yield accrued since the last collection — no receipt,
    /// no persistence, not part of the committed projection.
    accrued: Vec<(Resource, Ratio)>,
}

impl World {
    pub fn new() -> Self {
        Self {
            clock: WorldClock::new(),
            keepers: [Timekeeper { id: 0 }, Timekeeper { id: 1 }, Timekeeper { id: 2 }],
            chronicle: Chronicle::new(),
            orb: None,
            accrued: Vec::new(),
        }
    }

    pub fn now(&self) -> u64 {
        self.clock.now()
    }

    pub fn chronicle(&self) -> &Chronicle {
        &self.chronicle
    }

    pub fn orb(&self) -> Option<&ColonyOrb> {
        self.orb.as_ref()
    }

    pub fn accrued(&self) -> &[(Resource, Ratio)] {
        &self.accrued
    }

    /// Found the colony — a closure; the founding receipt is sequence 0
    /// of the world's story.
    pub fn found(&mut self, name: &str) -> Result<(), ActionRefused> {
        match self.orb {
            Some(_) => Err(ActionRefused::ColonyAlreadyFounded),
            None => {
                let orb_id = 1;
                self.orb = Some(ColonyOrb {
                    orb_id,
                    name: name.to_string(),
                    zones: Vec::new(),
                    banked: Vec::new(),
                });
                self.close(ClosureEvent::ColonyFounded {
                    orb_id,
                    name: name.to_string(),
                });
                Ok(())
            }
        }
    }

    /// Zone a tile — a closure (construction completes instantly in the
    /// first slice).
    pub fn zone(&mut self, tile: u16, kind: ZoneKind) -> Result<(), ActionRefused> {
        let orb = self.orb.as_mut().ok_or(ActionRefused::NoColony)?;
        match orb.zones.iter().any(|zone| zone.tile == tile) {
            true => Err(ActionRefused::TileAlreadyZoned { tile }),
            false => {
                orb.zones.push(Zone { tile, kind });
                orb.zones.sort_by_key(|zone| zone.tile);
                let orb_id = orb.orb_id;
                self.close(ClosureEvent::ZoneBuilt { orb_id, tile, kind });
                Ok(())
            }
        }
    }

    /// One simulation tick: every zone accrues its exact yield into the
    /// workspace, and the clock walks by quorum. Nothing chronicles —
    /// per-tick state never does (NS-3 §2.3).
    pub fn tick(&mut self) -> TickRead {
        if let Some(orb) = &self.orb {
            for zone in &orb.zones {
                let (resource, amount) = zone.kind.yield_per_tick();
                accrue(&mut self.accrued, resource, amount);
            }
        }
        let attestations: [Attestation; 3] = [
            self.keepers[0].attest(&self.clock),
            self.keepers[1].attest(&self.clock),
            self.keepers[2].attest(&self.clock),
        ];
        self.clock.tick(&attestations)
    }

    /// Collect — the accrued workspace closes into banked, receipted
    /// state, one receipt per resource, stamped with the clock value
    /// the world reads NOW.
    pub fn collect(&mut self) -> Result<usize, ActionRefused> {
        let orb = self.orb.as_mut().ok_or(ActionRefused::NoColony)?;
        match self.accrued.is_empty() {
            true => Err(ActionRefused::NothingAccrued),
            false => {
                let accrued = std::mem::take(&mut self.accrued);
                let orb_id = orb.orb_id;
                let count = accrued.len();
                let stamp = self.clock.now();
                for (resource, amount) in accrued {
                    orb.bank(resource, amount.clone());
                    self.chronicle.append(
                        ClosureEvent::YieldCollected {
                            orb_id,
                            resource,
                            amount,
                        },
                        stamp,
                        vec![],
                    );
                }
                Ok(count)
            }
        }
    }

    /// The committed projection of the live world.
    pub fn committed(&self) -> Committed {
        Committed {
            orb: self.orb.clone(),
        }
    }

    fn close(&mut self, event: ClosureEvent) {
        let stamp = self.clock.now();
        self.chronicle.append(event, stamp, vec![]);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// Replay a chronicle into the committed projection it grounds
/// (CHARTER SEAL B: the chronicle IS the world; everything else is a
/// view). Unknown future events would be skipped by the event decoder;
/// receipts carry typed events here, so the fold is total.
pub fn replay(chronicle: &Chronicle) -> Committed {
    let mut orb: Option<ColonyOrb> = None;
    for receipt in chronicle.receipts() {
        match &receipt.event {
            ClosureEvent::ColonyFounded { orb_id, name } => {
                orb = Some(ColonyOrb {
                    orb_id: *orb_id,
                    name: name.clone(),
                    zones: Vec::new(),
                    banked: Vec::new(),
                });
            }
            ClosureEvent::ZoneBuilt { tile, kind, .. } => {
                if let Some(orb) = orb.as_mut() {
                    orb.zones.push(Zone {
                        tile: *tile,
                        kind: *kind,
                    });
                    orb.zones.sort_by_key(|zone| zone.tile);
                }
            }
            ClosureEvent::YieldCollected {
                resource, amount, ..
            } => {
                if let Some(orb) = orb.as_mut() {
                    orb.bank(*resource, amount.clone());
                }
            }
        }
    }
    Committed { orb }
}

fn accrue(ledger: &mut Vec<(Resource, Ratio)>, resource: Resource, amount: Ratio) {
    match ledger.iter_mut().find(|(held, _)| *held == resource) {
        Some((_, held)) => *held += amount,
        None => {
            ledger.push((resource, amount));
            ledger.sort_by_key(|(resource, _)| *resource);
        }
    }
}
