//! NS-1 DOP — dimensional orbs. The colony orb is the first consumer:
//! a self-contained game object whose committed state round-trips its
//! wire codec byte-exact.
//!
//! THE NETSTRATUM WIRE TAG REGISTRY (permanent — one tag, one meaning,
//! forever; retired codes are never reused):
//!   1  ClosureEvent::ColonyFounded
//!   2  ClosureEvent::ZoneBuilt
//!   3  ClosureEvent::YieldCollected
//!   16 Orb header (orb_id ‖ name)
//!   17 Orb zone entry (tile ‖ kind)
//!   18 Orb banked entry (resource ‖ ratio)

use crate::ratio::{self, Ratio};
use crate::tlv;

/// The resources of the first slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Resource {
    Iron = 1,
    Biomass = 2,
}

impl Resource {
    pub fn from_wire(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Iron),
            2 => Some(Self::Biomass),
            _ => None,
        }
    }
}

/// The zone kinds of the first slice, with their exact per-tick yields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ZoneKind {
    Extractor = 1,
    Habitat = 2,
}

impl ZoneKind {
    pub fn from_wire(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Extractor),
            2 => Some(Self::Habitat),
            _ => None,
        }
    }

    /// The exact yield rate — a ratio, never a float (RFA).
    pub fn yield_per_tick(&self) -> (Resource, Ratio) {
        match self {
            Self::Extractor => (Resource::Iron, ratio::ratio(3, 2)),
            Self::Habitat => (Resource::Biomass, ratio::ratio(1, 4)),
        }
    }
}

/// A zoned tile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zone {
    pub tile: u16,
    pub kind: ZoneKind,
}

/// The closures of the first slice — completed actions, the only
/// things that chronicle (NS-3 §2.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClosureEvent {
    ColonyFounded { orb_id: u64, name: String },
    ZoneBuilt { orb_id: u64, tile: u16, kind: ZoneKind },
    YieldCollected { orb_id: u64, resource: Resource, amount: Ratio },
}

impl ClosureEvent {
    /// The normative TLV event encoding (NS-3 §2.2, landed here at N2):
    /// one record, tag = the event's permanent code, value = the
    /// event's field layout.
    pub fn wire_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        match self {
            Self::ColonyFounded { orb_id, name } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(name.as_bytes());
                tlv::put(1, &value, &mut out);
            }
            Self::ZoneBuilt { orb_id, tile, kind } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.extend_from_slice(&tile.to_le_bytes());
                value.push(*kind as u8);
                tlv::put(2, &value, &mut out);
            }
            Self::YieldCollected {
                orb_id,
                resource,
                amount,
            } => {
                let mut value = orb_id.to_le_bytes().to_vec();
                value.push(*resource as u8);
                ratio::encode(amount, &mut value);
                tlv::put(3, &value, &mut out);
            }
        }
        out
    }

    /// Decode one event from its wire record; unknown tags are not an
    /// event (the caller skips them — SEAL A).
    pub fn from_wire(tag: u8, value: &[u8]) -> Option<Self> {
        match tag {
            1 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let name = String::from_utf8(value.get(8..)?.to_vec()).ok()?;
                Some(Self::ColonyFounded { orb_id, name })
            }
            2 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let tile = u16::from_le_bytes(value.get(8..10)?.try_into().ok()?);
                let kind = ZoneKind::from_wire(*value.get(10)?)?;
                Some(Self::ZoneBuilt { orb_id, tile, kind })
            }
            3 => {
                let orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                let resource = Resource::from_wire(*value.get(8)?)?;
                let (amount, _) = ratio::decode(value.get(9..)?)?;
                Some(Self::YieldCollected {
                    orb_id,
                    resource,
                    amount,
                })
            }
            _ => None,
        }
    }
}

/// The colony orb — COMMITTED state only. Unreceipted accrual is
/// workspace (it lives on the world, not the orb) and never encodes:
/// what has no receipt does not persist.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ColonyOrb {
    pub orb_id: u64,
    pub name: String,
    /// Zones, tile-ascending (canonical order).
    pub zones: Vec<Zone>,
    /// Banked yields, resource-ascending (canonical order).
    pub banked: Vec<(Resource, Ratio)>,
}

impl ColonyOrb {
    /// The canonical orb codec: header, then zones tile-ascending, then
    /// banked entries resource-ascending. Canonical order + canonical
    /// ratio carriers = byte-exact round trips.
    pub fn wire_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let mut header = self.orb_id.to_le_bytes().to_vec();
        header.extend_from_slice(self.name.as_bytes());
        tlv::put(16, &header, &mut out);
        for zone in &self.zones {
            let mut value = zone.tile.to_le_bytes().to_vec();
            value.push(zone.kind as u8);
            tlv::put(17, &value, &mut out);
        }
        for (resource, amount) in &self.banked {
            let mut value = vec![*resource as u8];
            ratio::encode(amount, &mut value);
            tlv::put(18, &value, &mut out);
        }
        out
    }

    /// Decode an orb; unknown tags are SKIPPED (SEAL A) — bytes from a
    /// future engine version decode to the fields this version knows.
    pub fn from_wire(bytes: &[u8]) -> Option<Self> {
        let mut orb = ColonyOrb::default();
        let mut seen_header = false;
        for (tag, value) in tlv::records(bytes) {
            match tag {
                16 => {
                    orb.orb_id = u64::from_le_bytes(value.get(..8)?.try_into().ok()?);
                    orb.name = String::from_utf8(value.get(8..)?.to_vec()).ok()?;
                    seen_header = true;
                }
                17 => {
                    let tile = u16::from_le_bytes(value.get(..2)?.try_into().ok()?);
                    let kind = ZoneKind::from_wire(*value.get(2)?)?;
                    orb.zones.push(Zone { tile, kind });
                }
                18 => {
                    let resource = Resource::from_wire(*value.first()?)?;
                    let (amount, _) = ratio::decode(value.get(1..)?)?;
                    orb.banked.push((resource, amount));
                }
                _ => {}
            }
        }
        seen_header.then_some(orb)
    }

    /// Bank an amount into the canonical (resource-ascending) ledger.
    pub fn bank(&mut self, resource: Resource, amount: Ratio) {
        match self.banked.iter_mut().find(|(held, _)| *held == resource) {
            Some((_, held)) => *held += amount,
            None => {
                self.banked.push((resource, amount));
                self.banked.sort_by_key(|(resource, _)| *resource);
            }
        }
    }
}
