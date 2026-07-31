//! CODEC-1 — the committed state's ONE canonical encoding (Charter
//! §11 ladder #35; registry tag 83 names the container when these
//! bytes ride a TLV stream).
//!
//! This is the artifact digests bind. The rules, in full:
//!
//! - **One state, one encoding.** Every committed family is written
//!   positionally, in `Committed`'s declaration order, using the
//!   engine's own canonical orders (orbs by the landed NS-1 orb
//!   codec; vectors in their maintained ascending orders). Equal
//!   states produce equal bytes; distinct states cannot collide on
//!   encoding because nothing about the encoding is chosen.
//! - **The schema leads.** Byte 0 is the schema version. Version 1
//!   is the freeze-window snapshot of the field set (all twenty
//!   families). A schema this kernel does not know REFUSES — a
//!   schema bump is a kernel amendment, never a silent widening.
//! - **Refuse, never guess.** Truncation, trailing bytes, an unknown
//!   resource or star or scope code — every malformation decodes to
//!   None. There is no partial state.
//!
//! Serialization is this crate's monopoly (the Phase-1 seam): the
//! kernel is encoding-blind, the spine consumes these bytes, and no
//! other component in the workspace may produce a second encoding of
//! committed state.

use cfe::dop::{ColonyOrb, Resource};
use cfe::ipsal::{Entity, Scope};
use cfe::ratio::{self, Ratio};
use cfe::thermal::StarClass;
use cfe::world::{AbsorptionDebt, Bond, Committed, Council, Covenant, Genesis, Offer, Prospected};

/// Schema version 1: the freeze-window snapshot of the committed
/// field set.
pub const SCHEMA_V1: u8 = 1;

fn put_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn put_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn put_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn put_text(out: &mut Vec<u8>, text: &str) {
    put_u16(out, text.len() as u16);
    out.extend_from_slice(text.as_bytes());
}

/// The one encoding: schema byte, then every committed family in
/// declaration order, positionally, canonically.
pub fn state_bytes(committed: &Committed) -> Vec<u8> {
    let mut out = vec![SCHEMA_V1];

    put_u32(&mut out, committed.orbs.len() as u32);
    for orb in &committed.orbs {
        let bytes = orb.wire_bytes();
        put_u32(&mut out, bytes.len() as u32);
        out.extend_from_slice(&bytes);
    }

    put_u32(&mut out, committed.entities.len() as u32);
    for entity in &committed.entities {
        put_u64(&mut out, entity.entity_id);
        match entity.principal {
            None => out.push(0),
            Some(principal) => {
                out.push(1);
                put_u64(&mut out, principal);
            }
        }
        put_text(&mut out, &entity.handle);
        put_u32(&mut out, entity.grants.len() as u32);
        for scope in &entity.grants {
            out.push(*scope as u8);
        }
    }

    match committed.star {
        None => out.push(0),
        Some(class) => out.push(class as u8),
    }

    put_u32(&mut out, committed.debts.len() as u32);
    for debt in &committed.debts {
        put_u64(&mut out, debt.debtor);
        put_u64(&mut out, debt.creditor);
        ratio::encode(&debt.remainder, &mut out);
        put_u64(&mut out, debt.opened);
    }

    put_u32(&mut out, committed.bonds.len() as u32);
    for bond in &committed.bonds {
        put_u64(&mut out, bond.source);
        put_u64(&mut out, bond.target);
        out.push(bond.resource as u8);
        ratio::encode(&bond.rate, &mut out);
        put_u64(&mut out, bond.term);
        put_u64(&mut out, bond.sealed);
        put_u64(&mut out, bond.anchored);
    }

    put_u32(&mut out, committed.covenants.len() as u32);
    for covenant in &committed.covenants {
        put_u64(&mut out, covenant.obligor);
        put_u64(&mut out, covenant.beneficiary);
        out.push(covenant.resource as u8);
        ratio::encode(&covenant.quota, &mut out);
        ratio::encode(&covenant.delivered, &mut out);
        ratio::encode(&covenant.collateral, &mut out);
        put_u64(&mut out, covenant.sealed);
        put_u64(&mut out, covenant.window);
    }

    put_u32(&mut out, committed.councils.len() as u32);
    for council in &committed.councils {
        put_u64(&mut out, council.council);
        put_u32(&mut out, council.members.len() as u32);
        for member in &council.members {
            put_u64(&mut out, *member);
        }
        out.push(council.resource as u8);
        ratio::encode(&council.quota, &mut out);
        put_u32(&mut out, council.delivered.len() as u32);
        for delivered in &council.delivered {
            ratio::encode(delivered, &mut out);
        }
        ratio::encode(&council.collateral, &mut out);
        put_u64(&mut out, council.sealed);
        put_u64(&mut out, council.window);
    }

    put_u32(&mut out, committed.offers.len() as u32);
    for offer in &committed.offers {
        put_u64(&mut out, offer.offer);
        put_u64(&mut out, offer.orb);
        out.push(offer.give as u8);
        ratio::encode(&offer.give_amount, &mut out);
        out.push(offer.want as u8);
        ratio::encode(&offer.want_amount, &mut out);
        put_u64(&mut out, offer.posted);
    }

    put_u32(&mut out, committed.techs.len() as u32);
    for (orb, condition, node) in &committed.techs {
        put_u64(&mut out, *orb);
        out.push(*condition);
        out.extend_from_slice(node);
    }

    put_u32(&mut out, committed.wants.len() as u32);
    for (orb, kind) in &committed.wants {
        put_u64(&mut out, *orb);
        out.push(*kind);
    }

    out.push(committed.crossed as u8);

    put_u32(&mut out, committed.gauge.len() as u32);
    for grade in &committed.gauge {
        ratio::encode(grade, &mut out);
    }

    match &committed.genesis {
        None => out.push(0),
        Some(genesis) => {
            out.push(1);
            put_text(&mut out, &genesis.civilization);
            out.push(genesis.disposition);
            out.push(genesis.metabolism);
            out.push(genesis.perception);
        }
    }

    match &committed.terrain_seed {
        None => out.push(0),
        Some(seed) => {
            out.push(1);
            out.extend_from_slice(seed);
        }
    }

    put_u32(&mut out, committed.instruments.len() as u32);
    for (orb, tech_hash, grade) in &committed.instruments {
        put_u64(&mut out, *orb);
        out.extend_from_slice(tech_hash);
        out.push(*grade);
    }

    put_u32(&mut out, committed.routes.len() as u32);
    for (deal, eta) in &committed.routes {
        put_u64(&mut out, *deal);
        ratio::encode(eta, &mut out);
    }

    put_u32(&mut out, committed.practices.len() as u32);
    out.extend_from_slice(&committed.practices);

    put_u32(&mut out, committed.prospects.len() as u32);
    for prospect in &committed.prospects {
        put_u64(&mut out, prospect.orb);
        out.push(prospect.stratum);
        out.extend_from_slice(&prospect.element);
        ratio::encode(&prospect.richness, &mut out);
        put_u64(&mut out, prospect.last_assay);
    }

    match committed.warded {
        None => out.push(0),
        Some(false) => out.push(1),
        Some(true) => out.push(2),
    }

    out.push(committed.veiled as u8);
    out
}

/// The decode cursor: every take refuses past the end.
struct Take<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Take<'a> {
    fn u8(&mut self) -> Option<u8> {
        let value = *self.bytes.get(self.at)?;
        self.at += 1;
        Some(value)
    }

    fn u16(&mut self) -> Option<u16> {
        let value = u16::from_le_bytes(self.bytes.get(self.at..self.at + 2)?.try_into().ok()?);
        self.at += 2;
        Some(value)
    }

    fn u32(&mut self) -> Option<u32> {
        let value = u32::from_le_bytes(self.bytes.get(self.at..self.at + 4)?.try_into().ok()?);
        self.at += 4;
        Some(value)
    }

    fn u64(&mut self) -> Option<u64> {
        let value = u64::from_le_bytes(self.bytes.get(self.at..self.at + 8)?.try_into().ok()?);
        self.at += 8;
        Some(value)
    }

    fn array32(&mut self) -> Option<[u8; 32]> {
        let value: [u8; 32] = self.bytes.get(self.at..self.at + 32)?.try_into().ok()?;
        self.at += 32;
        Some(value)
    }

    fn slice(&mut self, len: usize) -> Option<&'a [u8]> {
        let value = self.bytes.get(self.at..self.at + len)?;
        self.at += len;
        Some(value)
    }

    fn ratio(&mut self) -> Option<Ratio> {
        let (value, used) = ratio::decode(self.bytes.get(self.at..)?)?;
        self.at += used;
        Some(value)
    }

    fn text(&mut self) -> Option<String> {
        let len = self.u16()? as usize;
        String::from_utf8(self.slice(len)?.to_vec()).ok()
    }

    fn resource(&mut self) -> Option<Resource> {
        Resource::from_wire(self.u8()?)
    }

    fn done(&self) -> bool {
        self.at == self.bytes.len()
    }
}

/// The one decoding: an unknown schema, a truncated stream, trailing
/// bytes, or an unlawful code refuses — never a guess, never a
/// partial state.
pub fn from_state_bytes(bytes: &[u8]) -> Option<Committed> {
    let mut take = Take { bytes, at: 0 };
    if take.u8()? != SCHEMA_V1 {
        return None;
    }

    let mut orbs = Vec::new();
    for _ in 0..take.u32()? {
        let len = take.u32()? as usize;
        orbs.push(ColonyOrb::from_wire(take.slice(len)?)?);
    }

    let mut entities = Vec::new();
    for _ in 0..take.u32()? {
        let entity_id = take.u64()?;
        let principal = match take.u8()? {
            0 => None,
            1 => Some(take.u64()?),
            _ => return None,
        };
        let handle = take.text()?;
        let mut grants = Vec::new();
        for _ in 0..take.u32()? {
            grants.push(Scope::from_wire(take.u8()?)?);
        }
        entities.push(Entity { entity_id, handle, principal, grants });
    }

    let star = match take.u8()? {
        0 => None,
        code => Some(StarClass::from_wire(code)?),
    };

    let mut debts = Vec::new();
    for _ in 0..take.u32()? {
        debts.push(AbsorptionDebt {
            debtor: take.u64()?,
            creditor: take.u64()?,
            remainder: take.ratio()?,
            opened: take.u64()?,
        });
    }

    let mut bonds = Vec::new();
    for _ in 0..take.u32()? {
        bonds.push(Bond {
            source: take.u64()?,
            target: take.u64()?,
            resource: take.resource()?,
            rate: take.ratio()?,
            term: take.u64()?,
            sealed: take.u64()?,
            anchored: take.u64()?,
        });
    }

    let mut covenants = Vec::new();
    for _ in 0..take.u32()? {
        covenants.push(Covenant {
            obligor: take.u64()?,
            beneficiary: take.u64()?,
            resource: take.resource()?,
            quota: take.ratio()?,
            delivered: take.ratio()?,
            collateral: take.ratio()?,
            sealed: take.u64()?,
            window: take.u64()?,
        });
    }

    let mut councils = Vec::new();
    for _ in 0..take.u32()? {
        let council = take.u64()?;
        let mut members = Vec::new();
        for _ in 0..take.u32()? {
            members.push(take.u64()?);
        }
        let resource = take.resource()?;
        let quota = take.ratio()?;
        let mut delivered = Vec::new();
        for _ in 0..take.u32()? {
            delivered.push(take.ratio()?);
        }
        councils.push(Council {
            council,
            members,
            resource,
            quota,
            delivered,
            collateral: take.ratio()?,
            sealed: take.u64()?,
            window: take.u64()?,
        });
    }

    let mut offers = Vec::new();
    for _ in 0..take.u32()? {
        offers.push(Offer {
            offer: take.u64()?,
            orb: take.u64()?,
            give: take.resource()?,
            give_amount: take.ratio()?,
            want: take.resource()?,
            want_amount: take.ratio()?,
            posted: take.u64()?,
        });
    }

    let mut techs = Vec::new();
    for _ in 0..take.u32()? {
        techs.push((take.u64()?, take.u8()?, take.array32()?));
    }

    let mut wants = Vec::new();
    for _ in 0..take.u32()? {
        wants.push((take.u64()?, take.u8()?));
    }

    let crossed = match take.u8()? {
        0 => false,
        1 => true,
        _ => return None,
    };

    let mut gauge = Vec::new();
    for _ in 0..take.u32()? {
        gauge.push(take.ratio()?);
    }

    let genesis = match take.u8()? {
        0 => None,
        1 => Some(Genesis {
            civilization: take.text()?,
            disposition: take.u8()?,
            metabolism: take.u8()?,
            perception: take.u8()?,
        }),
        _ => return None,
    };

    let terrain_seed = match take.u8()? {
        0 => None,
        1 => Some(take.array32()?),
        _ => return None,
    };

    let mut instruments = Vec::new();
    for _ in 0..take.u32()? {
        instruments.push((take.u64()?, take.array32()?, take.u8()?));
    }

    let mut routes = Vec::new();
    for _ in 0..take.u32()? {
        routes.push((take.u64()?, take.ratio()?));
    }

    let practices_len = take.u32()? as usize;
    let practices = take.slice(practices_len)?.to_vec();

    let mut prospects = Vec::new();
    for _ in 0..take.u32()? {
        prospects.push(Prospected {
            orb: take.u64()?,
            stratum: take.u8()?,
            element: take.array32()?,
            richness: take.ratio()?,
            last_assay: take.u64()?,
        });
    }

    let warded = match take.u8()? {
        0 => None,
        1 => Some(false),
        2 => Some(true),
        _ => return None,
    };

    let veiled = match take.u8()? {
        0 => false,
        1 => true,
        _ => return None,
    };

    if !take.done() {
        return None;
    }

    Some(Committed {
        orbs,
        entities,
        star,
        debts,
        bonds,
        covenants,
        councils,
        offers,
        techs,
        wants,
        crossed,
        gauge,
        genesis,
        terrain_seed,
        instruments,
        routes,
        practices,
        prospects,
        warded,
        veiled,
    })
}
