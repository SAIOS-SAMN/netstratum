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
//! kernel is encoding-blind, the arbor consumes these bytes, and no
//! other component in the workspace may produce a second encoding of
//! committed state.

use cfe::dop::{Cargo, ColonyOrb, Resource};
use cfe::ipsal::{Entity, Scope};
use cfe::ratio::{self, Ratio};
use cfe::thermal::StarClass;
use cfe::world::{AbsorptionDebt, Bond, Committed, Council, Forward, Genesis, Offer, Prospected};

/// Schema version 1: the freeze-window snapshot of the committed
/// field set.
pub const SCHEMA_V1: u8 = 1;
/// KERNEL-TAXON (15 §4): v2 appends the sealed taxon as a trailing
/// presence-byte + (class, clade). Emit v2; ACCEPT v1 (taxon absent →
/// None) so every v1 commitment replays whole. CODER-04 TO RATIFY —
/// the schema versions are the codec seat's byte monopoly; this is
/// the mechanical trailing addition the freeze window exists for.
pub const SCHEMA_V2: u8 = 2;
/// THE VERTICAL AXIS (2026-08-01): v3 appends the SEATS held as a
/// trailing count + (station byte, length-prefixed line). Emit v3;
/// ACCEPT v2 and v1 (seats absent → empty, which is the truthful
/// reading of a world that never seated one). The additive-trailing
/// shape is the KERNEL-TAXON precedent, unchanged.
pub const SCHEMA_V3: u8 = 3;
/// RECEIPT AUTHORITY (operator ruling 2026-08-05): v4 appends the
/// techs' RECEIPTED MAGNITUDES as a trailing run of ratios, one per
/// tech entry in the techs family's own order. Emit v4; ACCEPT v3 and
/// below by RECONSTRUCTING each magnitude from the closed writer rule
/// of those eras (every pre-v4 writer was deterministic: the
/// crystallized table for conditions it names, 3/4 for the
/// re-emergence lesson, 1 otherwise). Honest caveat, recorded: a
/// pre-v4 LICENSED node of a crystallized condition carried 1 in its
/// receipt while the table says otherwise — the reconstruction reads
/// the table, so such a node decodes with the table's value; the
/// kernel reads no magnitude today, so no admission changes.
pub const SCHEMA_V4: u8 = 4;

/// **SCHEMA v5 (#42) — THE PACTS RIDE THE SNAPSHOT.** A world's
/// standing treaties are committed state, so a snapshot that dropped
/// them would reopen a world at peace with everyone it was at war
/// with. Appended after the routes, version-gated: a v4 snapshot
/// decodes with NO pacts, which is exactly what a world that predates
/// treaties had.
pub const SCHEMA_V5: u8 = 5;

/// **SCHEMA v6 (Wave-3/4) — ELEMENT ROUTES.** Specialized element
/// lanes ride the snapshot trail after treaties. A v5 blob ends before
/// them and decodes with an empty element-route table (SEAL A).
pub const SCHEMA_V6: u8 = 6;

/// **SCHEMA v7 (Phase-3 fleet) — hulls trail after element routes.**
pub const SCHEMA_V7: u8 = 7;

/// **SCHEMA v8 (Phase-4 instruments) — notes, futures, last clears.**
pub const SCHEMA_V8: u8 = 8;

/// **SCHEMA v9 — shares + perps trail.**
pub const SCHEMA_V9: u8 = 9;

/// **SCHEMA v10 — P5-M clear depths + circuit breaker.**
pub const SCHEMA_V10: u8 = 10;

/// **SCHEMA v11 — P5-M2 cascade + P5-D tamper pressures.**
pub const SCHEMA_V11: u8 = 11;

/// **SCHEMA v12 — P5-B posture doctrine overlay.**
pub const SCHEMA_V12: u8 = 12;

/// **SCHEMA v13 — P5-F intel stash + spy marks.**
pub const SCHEMA_V13: u8 = 13;

/// **SCHEMA v14 — O6.5 vault custody STAMP per orb.**
pub const SCHEMA_V14: u8 = 14;

/// **SCHEMA v15 — CHEM science catalog (known gems).**
pub const SCHEMA_V15: u8 = 15;

/// **SCHEMA v16 — seated science labs (seats · clones · synth · skin · rogue).**
pub const SCHEMA_V16: u8 = 16;

/// **SCHEMA v17 — open clade (u16 LE in taxon field; no u8 tax cap).**
pub const SCHEMA_V17: u8 = 17;

/// **SCHEMA v18 — element_wants (E3.7 material WANT-1 targets).**
pub const SCHEMA_V18: u8 = 18;

/// **SCHEMA v19 — element_allowlists (E3.9 treaty element annexes).**
pub const SCHEMA_V19: u8 = 19;

/// **SCHEMA v20 — evolution_stage (44_LINEAGES §2c, one trailing byte).**
pub const SCHEMA_V20: u8 = 20;

/// **SCHEMA v21 — treasury_balances (49_TAXATION §5, trailing run).**
pub const SCHEMA_V21: u8 = 21;

/// **SCHEMA v22 — collapsed (50_WIN_CONDITIONS, the failure state, one
/// optional trailing stamp).**
pub const SCHEMA_V22: u8 = 22;

/// **SCHEMA v23 — pop_accrued (41_PROVISION, COG Slice 2, trailing run).**
/// The fractional population carry per orb — a v22 reader stops before it
/// and decodes a whole state with every band at a whole headcount.
pub const SCHEMA_V23: u8 = 23;

/// **SCHEMA v24 — leisure_banks (COG-1, doc 52 §7, trailing run).** The
/// cognition budget per orb — a v23 reader stops before it and decodes a
/// whole state with every mind's leisure bank empty.
pub const SCHEMA_V24: u8 = 24;

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
    let mut out = vec![SCHEMA_V24];

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

    put_u32(&mut out, committed.forwards.len() as u32);
    for forward in &committed.forwards {
        put_u64(&mut out, forward.obligor);
        put_u64(&mut out, forward.beneficiary);
        out.push(forward.resource as u8);
        ratio::encode(&forward.quota, &mut out);
        ratio::encode(&forward.delivered, &mut out);
        ratio::encode(&forward.collateral, &mut out);
        put_u64(&mut out, forward.sealed);
        put_u64(&mut out, forward.window);
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
        // E-MKT-1: Cargo wire (Resource 1..=7 or 0‖ElementId).
        offer.give.encode(&mut out);
        ratio::encode(&offer.give_amount, &mut out);
        offer.want.encode(&mut out);
        ratio::encode(&offer.want_amount, &mut out);
        put_u64(&mut out, offer.posted);
    }

    // The in-place family keeps its v1 3-field layout; the receipted
    // magnitudes ride the v4 TRAILING run below, so every older
    // decoder still reads this family whole.
    put_u32(&mut out, committed.techs.len() as u32);
    for (orb, condition, node, _) in &committed.techs {
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
        ratio::encode(eta.value(), &mut out);
    }

    put_u32(&mut out, committed.practices.len() as u32);
    out.extend_from_slice(&committed.practices);

    put_u32(&mut out, committed.prospects.len() as u32);
    for prospect in &committed.prospects {
        put_u64(&mut out, prospect.orb);
        out.push(prospect.stratum);
        out.extend_from_slice(&prospect.element);
        // ONT-2: the vein carries its extraction integral, not a richness
        // scalar (richness derives as natural − extracted).
        ratio::encode(&prospect.extracted, &mut out);
        put_u64(&mut out, prospect.last_assay);
    }

    match committed.warded {
        None => out.push(0),
        Some(false) => out.push(1),
        Some(true) => out.push(2),
    }

    out.push(committed.veiled as u8);
    // KERNEL-TAXON (v2 trailing field): presence, then class, clade.
    match committed.taxon {
        Some((class, clade)) => {
            out.push(1);
            out.push(class);
            // v17: clade as u16 LE (open expansion). Pre-v17: single u8.
            put_u16(&mut out, clade);
        }
        None => out.push(0),
    }
    // THE VERTICAL AXIS (v3 trailing field): seat count, then each as
    // station byte + length-prefixed line.
    out.push(u8::try_from(committed.stations.len()).unwrap_or(u8::MAX));
    for (line, station) in committed.stations.iter().take(usize::from(u8::MAX)) {
        out.push(*station);
        let name = line.as_bytes();
        out.push(u8::try_from(name.len()).unwrap_or(u8::MAX));
        out.extend_from_slice(&name[..name.len().min(usize::from(u8::MAX))]);
    }
    // RECEIPT AUTHORITY (v4 trailing run): each tech entry's receipted
    // magnitude, in the techs family's own order — no count, because
    // the count IS the techs count already written above.
    for (_, _, _, magnitude) in &committed.techs {
        ratio::encode(magnitude, &mut out);
    }
    // #42 (v5 TRAILING RUN): the standing pacts, LAST — the
    // additive-trailing law this codec has followed since v2. An
    // older reader stops before it and decodes a whole state; a block
    // inserted mid-stream would misalign every field after it, which
    // is exactly what `world_taxon`'s v1 pin caught when this was
    // first written between the routes and the practices.
    put_u32(&mut out, committed.treaties.len() as u32);
    for treaty in &committed.treaties {
        put_u64(&mut out, treaty.deal);
        out.push(treaty.kind.wire());
        put_u64(&mut out, treaty.sealed);
        put_u64(&mut out, treaty.term);
        let partner = treaty.partner.as_bytes();
        put_u32(&mut out, partner.len() as u32);
        out.extend_from_slice(partner);
        put_u32(&mut out, treaty.terms.len() as u32);
        for (channel, factor) in &treaty.terms {
            out.push(*channel);
            ratio::encode(factor, &mut out);
        }
    }
    // Wave-3 E-ROUTE (v6 TRAILING RUN): element-specialized lanes.
    put_u32(&mut out, committed.element_routes.len() as u32);
    for (deal, eta, element, rail) in &committed.element_routes {
        put_u64(&mut out, *deal);
        ratio::encode(eta.value(), &mut out);
        out.extend_from_slice(element);
        out.push(*rail);
    }
    // Phase-3 fleet hulls (v7 trail).
    put_u32(&mut out, committed.hulls.len() as u32);
    for h in &committed.hulls {
        put_u64(&mut out, h.hull_id);
        put_u64(&mut out, h.home_orb);
        put_u64(&mut out, h.assigned_route);
        out.push(h.role);
        out.push(h.grade);
    }
    // Phase-4 instruments (v8 trail).
    put_u32(&mut out, committed.notes.len() as u32);
    for n in &committed.notes {
        put_u64(&mut out, n.note_id);
        put_u64(&mut out, n.orb_id);
        put_u64(&mut out, n.issuer);
        put_u64(&mut out, n.holder);
        ratio::encode(&n.notional, &mut out);
        ratio::encode(&n.collateral, &mut out);
        put_u64(&mut out, n.maturity);
        out.push(u8::from(n.open));
    }
    put_u32(&mut out, committed.futures.len() as u32);
    for f in &committed.futures {
        put_u64(&mut out, f.fut_id);
        put_u64(&mut out, f.orb_id);
        put_u64(&mut out, f.long);
        put_u64(&mut out, f.short);
        out.push(f.resource as u8);
        ratio::encode(&f.notional, &mut out);
        ratio::encode(&f.price, &mut out);
        ratio::encode(&f.margin, &mut out);
        put_u64(&mut out, f.expiry);
        out.push(u8::from(f.open));
    }
    put_u32(&mut out, committed.last_clears.len() as u32);
    for (r, p) in &committed.last_clears {
        out.push(*r as u8);
        ratio::encode(p, &mut out);
    }
    put_u32(&mut out, committed.shares.len() as u32);
    for s in &committed.shares {
        put_u64(&mut out, s.share_id);
        put_u64(&mut out, s.orb_id);
        put_u64(&mut out, s.holder);
        out.push(s.stream as u8);
        ratio::encode(&s.units, &mut out);
        ratio::encode(&s.par, &mut out);
        out.push(u8::from(s.open));
    }
    put_u32(&mut out, committed.perps.len() as u32);
    for p in &committed.perps {
        put_u64(&mut out, p.perp_id);
        put_u64(&mut out, p.orb_id);
        put_u64(&mut out, p.long);
        put_u64(&mut out, p.short);
        out.push(p.resource as u8);
        ratio::encode(&p.notional, &mut out);
        ratio::encode(&p.entry, &mut out);
        ratio::encode(&p.margin, &mut out);
        out.push(u8::from(p.open));
    }
    // SCHEMA v10 — clear seal depths + circuit breaker clock.
    put_u32(&mut out, committed.clear_depths.len() as u32);
    for (r, d) in &committed.clear_depths {
        out.push(*r as u8);
        put_u64(&mut out, *d);
    }
    match committed.market_sealed_until {
        None => out.push(0),
        Some(t) => {
            out.push(1);
            put_u64(&mut out, t);
        }
    }
    // SCHEMA v11 — cascade + tamper pressures.
    put_u32(&mut out, committed.cascade_pressures.len() as u32);
    for (orb, p) in &committed.cascade_pressures {
        put_u64(&mut out, *orb);
        put_u64(&mut out, *p);
    }
    put_u32(&mut out, committed.tamper_pressures.len() as u32);
    for (orb, p) in &committed.tamper_pressures {
        put_u64(&mut out, *orb);
        ratio::encode(p, &mut out);
    }
    // SCHEMA v12 — posture doctrine overlay (0 = none).
    match committed.posture_disposition {
        None => out.push(0),
        Some(d) => {
            out.push(1);
            out.push(d);
        }
    }
    match committed.posture_perception {
        None => out.push(0),
        Some(p) => {
            out.push(1);
            out.push(p);
        }
    }
    match committed.posture_primary {
        None => out.push(0),
        Some(p) => {
            out.push(1);
            out.push(p);
        }
    }
    // SCHEMA v13 — intel hashes + spy marks.
    put_u32(&mut out, committed.intel_hashes.len() as u32);
    for h in &committed.intel_hashes {
        out.extend_from_slice(h);
    }
    put_u32(&mut out, committed.spy_marks.len() as u32);
    for (orb, d) in &committed.spy_marks {
        put_u64(&mut out, *orb);
        out.push(*d);
    }
    // SCHEMA v14 — vault custody STAMP.
    put_u32(&mut out, committed.vault_stamp.len() as u32);
    for (orb, amt) in &committed.vault_stamp {
        put_u64(&mut out, *orb);
        ratio::encode(amt, &mut out);
    }
    // SCHEMA v15 — chemistry catalog.
    put_u32(&mut out, committed.chem_catalog.len() as u32);
    for h in &committed.chem_catalog {
        out.extend_from_slice(h);
    }
    // SCHEMA v16 — science lab state.
    put_u32(&mut out, committed.science_seats.len() as u32);
    for (d, orb, rank) in &committed.science_seats {
        out.push(*d);
        put_u64(&mut out, *orb);
        out.push(*rank);
    }
    put_u32(&mut out, committed.clone_counts.len() as u32);
    for (orb, c) in &committed.clone_counts {
        put_u64(&mut out, *orb);
        put_u64(&mut out, *c);
    }
    put_u32(&mut out, committed.synth_mods.len() as u32);
    for (orb, g) in &committed.synth_mods {
        put_u64(&mut out, *orb);
        out.push(*g);
    }
    put_u32(&mut out, committed.skin_mods.len() as u32);
    for (orb, g) in &committed.skin_mods {
        put_u64(&mut out, *orb);
        out.push(*g);
    }
    put_u32(&mut out, committed.rogue_pressure.len() as u32);
    for (orb, p) in &committed.rogue_pressure {
        put_u64(&mut out, *orb);
        out.push(*p);
    }
    // SCHEMA v18 trailing: element material wants.
    put_u32(&mut out, committed.element_wants.len() as u32);
    for (orb, element) in &committed.element_wants {
        put_u64(&mut out, *orb);
        out.extend_from_slice(element);
    }
    // SCHEMA v19 trailing: treaty element allow-lists (E3.9) —
    // (deal, partner, ids), append order. The additive-trailing law:
    // a v18 reader stops before this run and decodes a whole state.
    put_u32(&mut out, committed.element_allowlists.len() as u32);
    for (deal, partner, elements) in &committed.element_allowlists {
        put_u64(&mut out, *deal);
        put_text(&mut out, partner);
        put_u32(&mut out, elements.len() as u32);
        for element in elements {
            out.extend_from_slice(element);
        }
    }
    // SCHEMA v20 trailing: the evolutionary stage (44_LINEAGES §2c) — one
    // byte. A v19 reader stops before it and decodes a whole state at
    // stage 0.
    out.push(committed.evolution_stage);
    // SCHEMA v21 trailing: sovereign treasury balances (49_TAXATION §5) —
    // (orb, balance) run, LAST. A v20 reader stops before it and decodes a
    // whole state with an empty treasury.
    put_u32(&mut out, committed.treasury_balances.len() as u32);
    for (orb, bal) in &committed.treasury_balances {
        put_u64(&mut out, *orb);
        ratio::encode(bal, &mut out);
    }
    // SCHEMA v22 trailing: the failure state (50_WIN_CONDITIONS) — one
    // flag byte, then the stamp if set. A v21 reader stops before it and
    // decodes a whole state as not-collapsed.
    match committed.collapsed {
        Some(stamp) => {
            out.push(1);
            put_u64(&mut out, stamp);
        }
        None => out.push(0),
    }
    // SCHEMA v23 trailing: the fractional population carry (41_PROVISION,
    // COG Slice 2) — (orb, remainder) run, LAST. A v22 reader stops before
    // it and decodes a whole state with every band at a whole headcount.
    put_u32(&mut out, committed.pop_accrued.len() as u32);
    for (orb, remainder) in &committed.pop_accrued {
        put_u64(&mut out, *orb);
        ratio::encode(remainder, &mut out);
    }
    // SCHEMA v24 trailing: the leisure banks (COG-1, doc 52 §7) — (orb,
    // balance) run, LAST. A v23 reader stops before it and decodes a whole
    // state with every mind's cognition budget empty.
    put_u32(&mut out, committed.leisure_banks.len() as u32);
    for (orb, bal) in &committed.leisure_banks {
        put_u64(&mut out, *orb);
        ratio::encode(bal, &mut out);
    }
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

    /// E-MKT-1: Cargo wire (Resource 1..=7 or 0 ‖ ElementId).
    fn cargo(&mut self) -> Option<Cargo> {
        let (cargo, used) = Cargo::decode(self.bytes.get(self.at..)?)?;
        self.at += used;
        Some(cargo)
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
    let schema = take.u8()?;
    if schema != SCHEMA_V1 && schema != SCHEMA_V2 && schema != SCHEMA_V3 && schema != SCHEMA_V4
        && schema != SCHEMA_V5
        && schema != SCHEMA_V6
        && schema != SCHEMA_V7
        && schema != SCHEMA_V8
        && schema != SCHEMA_V9
        && schema != SCHEMA_V10
        && schema != SCHEMA_V11
        && schema != SCHEMA_V12
        && schema != SCHEMA_V13
        && schema != SCHEMA_V14
        && schema != SCHEMA_V15
        && schema != SCHEMA_V16
        && schema != SCHEMA_V17
        && schema != SCHEMA_V18
        && schema != SCHEMA_V19
        && schema != SCHEMA_V20
        && schema != SCHEMA_V21
        && schema != SCHEMA_V22
        && schema != SCHEMA_V23
        && schema != SCHEMA_V24
    {
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

    let mut forwards = Vec::new();
    for _ in 0..take.u32()? {
        forwards.push(Forward {
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
            give: take.cargo()?,
            give_amount: take.ratio()?,
            want: take.cargo()?,
            want_amount: take.ratio()?,
            posted: take.u64()?,
        });
    }

    // The 3-field family decodes as it always did; the receipted
    // magnitudes arrive in the v4 trailing run (or reconstruct, for
    // older schemas) once the tail is reached.
    let mut bare_techs = Vec::new();
    for _ in 0..take.u32()? {
        bare_techs.push((take.u64()?, take.u8()?, take.array32()?));
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
        // FORM AT THE DOOR: the snapshot's eta re-proves [0,1) at decode,
        // same law as the chronicle door — a Unit past here is proven.
        routes.push((take.u64()?, cfe::dop::Unit::new(take.ratio()?)?));
    }

    let practices_len = take.u32()? as usize;
    let practices = take.slice(practices_len)?.to_vec();

    let mut prospects = Vec::new();
    for _ in 0..take.u32()? {
        prospects.push(Prospected {
            orb: take.u64()?,
            stratum: take.u8()?,
            element: take.array32()?,
            extracted: take.ratio()?,
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

    // KERNEL-TAXON: v1 carries no taxon (None); v2–v16: class + clade u8;
    // v17+: class + clade u16 LE (open expansion, SEAL B both ways).
    let taxon = if schema == SCHEMA_V1 {
        None
    } else {
        match take.u8()? {
            0 => None,
            1 => {
                let class = take.u8()?;
                let clade = if schema >= SCHEMA_V17 {
                    take.u16()?
                } else {
                    u16::from(take.u8()?)
                };
                Some((class, clade))
            }
            _ => return None,
        }
    };

    // THE VERTICAL AXIS: v1/v2 carry no seats (empty — a world that
    // never seated one); v3 reads the trailing table. Refuse never
    // guess: a short or unlawful entry refuses whole.
    let mut stations: Vec<(String, u8)> = Vec::new();
    if schema >= SCHEMA_V3 {
        let held = take.u8()?;
        for _ in 0..held {
            let station = take.u8()?;
            // Stations 1..=16 (T0–T15); 0 unlawful.
            if station == 0 || station > 16 {
                return None;
            }
            let len = usize::from(take.u8()?);
            let mut name = Vec::with_capacity(len);
            for _ in 0..len {
                name.push(take.u8()?);
            }
            stations.push((String::from_utf8(name).ok()?, station));
        }
    }

    // RECEIPT AUTHORITY: v4 reads the trailing magnitude run, one per
    // tech entry in order. Older schemas never carried it —
    // reconstruct by the closed writer rule of those eras (see
    // `SCHEMA_V4`): the crystallized table where it answers, 3/4 for
    // the re-emergence lesson, 1 otherwise.
    let techs: Vec<(u64, u8, [u8; 32], Ratio)> = if schema >= SCHEMA_V4 {
        let mut whole = Vec::with_capacity(bare_techs.len());
        for (orb, condition, node) in bare_techs {
            whole.push((orb, condition, node, take.ratio()?));
        }
        whole
    } else {
        bare_techs
            .into_iter()
            .map(|(orb, condition, node)| {
                let magnitude = cfe::dop::tech_node_of(condition)
                    .map(|(_, magnitude)| magnitude)
                    .unwrap_or_else(|| match condition {
                        7 => ratio::ratio(3, 4), // SEALED-RATE: pre-v4 writer rule
                        _ => ratio::ratio(1, 1),
                    });
                (orb, condition, node, magnitude)
            })
            .collect()
    };

    // #42 (v5 TRAILING RUN): the pacts after older fields.
    let mut treaties: Vec<cfe::treaty::Treaty> = Vec::new();
    if schema >= SCHEMA_V5 {
        for _ in 0..take.u32()? {
            let deal = take.u64()?;
            // The kind must NAME a pact — refuse, never guess.
            let kind = cfe::treaty::TreatyKind::from_wire(take.u8()?)?;
            let sealed = take.u64()?;
            let term = take.u64()?;
            let partner_len = take.u32()? as usize;
            let partner = String::from_utf8(take.slice(partner_len)?.to_vec()).ok()?;
            let mut terms = Vec::new();
            for _ in 0..take.u32()? {
                terms.push((take.u8()?, take.ratio()?));
            }
            treaties.push(cfe::treaty::Treaty { deal, kind, partner, terms, sealed, term });
        }
    }

    // Wave-3 (v6 TRAILING RUN): element-specialized lanes.
    let mut element_routes = Vec::new();
    if schema >= SCHEMA_V6 {
        for _ in 0..take.u32()? {
            let deal = take.u64()?;
            let eta = cfe::dop::Unit::new(take.ratio()?)?;
            let element = take.array32()?;
            let rail = take.u8()?;
            cfe::dop::Rail::from_wire(rail)?;
            element_routes.push((deal, eta, element, rail));
        }
    }

    let mut hulls = Vec::new();
    if schema >= SCHEMA_V7 {
        for _ in 0..take.u32()? {
            let hull_id = take.u64()?;
            let home_orb = take.u64()?;
            let assigned_route = take.u64()?;
            let role = take.u8()?;
            let grade = take.u8()?;
            hulls.push(cfe::dop::Hull {
                hull_id,
                role,
                grade,
                home_orb,
                assigned_route,
            });
        }
    }

    let mut notes = Vec::new();
    let mut futures = Vec::new();
    let mut last_clears = Vec::new();
    if schema >= SCHEMA_V8 {
        for _ in 0..take.u32()? {
            let note_id = take.u64()?;
            let orb_id = take.u64()?;
            let issuer = take.u64()?;
            let holder = take.u64()?;
            let notional = take.ratio()?;
            let collateral = take.ratio()?;
            let maturity = take.u64()?;
            let open = take.u8()? != 0;
            notes.push(cfe::dop::MarketNote {
                note_id,
                orb_id,
                issuer,
                holder,
                notional,
                collateral,
                maturity,
                open,
            });
        }
        for _ in 0..take.u32()? {
            let fut_id = take.u64()?;
            let orb_id = take.u64()?;
            let long = take.u64()?;
            let short = take.u64()?;
            let resource = cfe::dop::Resource::from_wire(take.u8()?)?;
            let notional = take.ratio()?;
            let price = take.ratio()?;
            let margin = take.ratio()?;
            let expiry = take.u64()?;
            let open = take.u8()? != 0;
            futures.push(cfe::dop::MarketFuture {
                fut_id,
                orb_id,
                long,
                short,
                resource,
                notional,
                price,
                margin,
                expiry,
                open,
            });
        }
        for _ in 0..take.u32()? {
            let resource = cfe::dop::Resource::from_wire(take.u8()?)?;
            let price = take.ratio()?;
            last_clears.push((resource, price));
        }
    }

    let mut shares = Vec::new();
    let mut perps = Vec::new();
    if schema >= SCHEMA_V9 {
        for _ in 0..take.u32()? {
            let share_id = take.u64()?;
            let orb_id = take.u64()?;
            let holder = take.u64()?;
            let stream = cfe::dop::Resource::from_wire(take.u8()?)?;
            let units = take.ratio()?;
            let par = take.ratio()?;
            let open = take.u8()? != 0;
            shares.push(cfe::dop::MarketShare {
                share_id, orb_id, holder, stream, units, par, open,
            });
        }
        for _ in 0..take.u32()? {
            let perp_id = take.u64()?;
            let orb_id = take.u64()?;
            let long = take.u64()?;
            let short = take.u64()?;
            let resource = cfe::dop::Resource::from_wire(take.u8()?)?;
            let notional = take.ratio()?;
            let entry = take.ratio()?;
            let margin = take.ratio()?;
            let open = take.u8()? != 0;
            perps.push(cfe::dop::MarketPerp {
                perp_id, orb_id, long, short, resource, notional, entry, margin, open,
            });
        }
    }

    let mut clear_depths = Vec::new();
    let mut market_sealed_until = None;
    if schema >= SCHEMA_V10 {
        for _ in 0..take.u32()? {
            let resource = cfe::dop::Resource::from_wire(take.u8()?)?;
            let depth = take.u64()?;
            clear_depths.push((resource, depth));
        }
        match take.u8()? {
            0 => market_sealed_until = None,
            1 => market_sealed_until = Some(take.u64()?),
            _ => return None,
        }
    }

    let mut cascade_pressures = Vec::new();
    let mut tamper_pressures = Vec::new();
    if schema >= SCHEMA_V11 {
        for _ in 0..take.u32()? {
            let orb = take.u64()?;
            let p = take.u64()?;
            cascade_pressures.push((orb, p));
        }
        for _ in 0..take.u32()? {
            let orb = take.u64()?;
            let p = take.ratio()?;
            tamper_pressures.push((orb, p));
        }
    }

    let mut posture_disposition = None;
    let mut posture_perception = None;
    let mut posture_primary = None;
    if schema >= SCHEMA_V12 {
        match take.u8()? {
            0 => posture_disposition = None,
            1 => posture_disposition = Some(take.u8()?),
            _ => return None,
        }
        match take.u8()? {
            0 => posture_perception = None,
            1 => posture_perception = Some(take.u8()?),
            _ => return None,
        }
        match take.u8()? {
            0 => posture_primary = None,
            1 => posture_primary = Some(take.u8()?),
            _ => return None,
        }
    }

    let mut intel_hashes = Vec::new();
    let mut spy_marks = Vec::new();
    if schema >= SCHEMA_V13 {
        for _ in 0..take.u32()? {
            let h = take.array32()?;
            intel_hashes.push(h);
        }
        for _ in 0..take.u32()? {
            let orb = take.u64()?;
            let d = take.u8()?;
            spy_marks.push((orb, d));
        }
    }

    let mut vault_stamp = Vec::new();
    if schema >= SCHEMA_V14 {
        for _ in 0..take.u32()? {
            let orb = take.u64()?;
            let amt = take.ratio()?;
            vault_stamp.push((orb, amt));
        }
    }

    let mut chem_catalog = Vec::new();
    if schema >= SCHEMA_V15 {
        for _ in 0..take.u32()? {
            chem_catalog.push(take.array32()?);
        }
    }

    let mut science_seats = Vec::new();
    let mut clone_counts = Vec::new();
    let mut synth_mods = Vec::new();
    let mut skin_mods = Vec::new();
    let mut rogue_pressure = Vec::new();
    if schema >= SCHEMA_V16 {
        for _ in 0..take.u32()? {
            let d = take.u8()?;
            let orb = take.u64()?;
            let rank = take.u8()?;
            science_seats.push((d, orb, rank));
        }
        for _ in 0..take.u32()? {
            let orb = take.u64()?;
            let c = take.u64()?;
            clone_counts.push((orb, c));
        }
        for _ in 0..take.u32()? {
            let orb = take.u64()?;
            let g = take.u8()?;
            synth_mods.push((orb, g));
        }
        for _ in 0..take.u32()? {
            let orb = take.u64()?;
            let g = take.u8()?;
            skin_mods.push((orb, g));
        }
        for _ in 0..take.u32()? {
            let orb = take.u64()?;
            let p = take.u8()?;
            rogue_pressure.push((orb, p));
        }
    }

    let mut element_wants = Vec::new();
    if schema >= SCHEMA_V18 {
        for _ in 0..take.u32()? {
            let orb = take.u64()?;
            let element = take.array32()?;
            element_wants.push((orb, element));
        }
    }

    let mut element_allowlists = Vec::new();
    if schema >= SCHEMA_V19 {
        for _ in 0..take.u32()? {
            let deal = take.u64()?;
            let partner = take.text()?;
            let mut elements = Vec::new();
            for _ in 0..take.u32()? {
                elements.push(take.array32()?);
            }
            element_allowlists.push((deal, partner, elements));
        }
    }

    // SCHEMA v20 trailing: the evolutionary stage — one byte, or 0 for a
    // pre-v20 snapshot (a line that had not yet advanced).
    let evolution_stage = if schema >= SCHEMA_V20 { take.u8()? } else { 0 };

    // SCHEMA v21 trailing: sovereign treasury balances, or empty for a
    // pre-v21 snapshot (no line had yet collected a levy).
    let mut treasury_balances = Vec::new();
    if schema >= SCHEMA_V21 {
        for _ in 0..take.u32()? {
            let orb = take.u64()?;
            let bal = take.ratio()?;
            treasury_balances.push((orb, bal));
        }
    }

    // SCHEMA v22 trailing: the failure state, or None for a pre-v22
    // snapshot (no line had yet collapsed under this schema).
    let collapsed = if schema >= SCHEMA_V22 {
        match take.u8()? {
            1 => Some(take.u64()?),
            _ => None,
        }
    } else {
        None
    };

    // SCHEMA v23 trailing: the fractional population carry, or empty for a
    // pre-v23 snapshot (every band read at a whole headcount).
    let mut pop_accrued = Vec::new();
    if schema >= SCHEMA_V23 {
        for _ in 0..take.u32()? {
            let orb = take.u64()?;
            let remainder = take.ratio()?;
            pop_accrued.push((orb, remainder));
        }
    }

    // SCHEMA v24 trailing: the leisure banks, or empty for a pre-v24 snapshot
    // (every mind's cognition budget read as empty).
    let mut leisure_banks = Vec::new();
    if schema >= SCHEMA_V24 {
        for _ in 0..take.u32()? {
            let orb = take.u64()?;
            let bal = take.ratio()?;
            leisure_banks.push((orb, bal));
        }
    }

    if !take.done() {
        return None;
    }

    Some(Committed {
        orbs,
        entities,
        star,
        debts,
        bonds,
        forwards,
        councils,
        offers,
        techs,
        wants,
        element_wants,
        crossed,
        gauge,
        genesis,
        terrain_seed,
        instruments,
        routes,
        element_routes,
        treaties,
        practices,
        prospects,
        warded,
        veiled,
        taxon,
        evolution_stage,
        collapsed,
        stations,
        hulls,
        notes,
        futures,
        last_clears,
        shares,
        perps,
        clear_depths,
        market_sealed_until,
        cascade_pressures,
        tamper_pressures,
        posture_disposition,
        posture_perception,
        posture_primary,
        intel_hashes,
        spy_marks,
        vault_stamp,
        chem_catalog,
        science_seats,
        clone_counts,
        pop_accrued,
        treasury_balances,
        leisure_banks,
        synth_mods,
        skin_mods,
        rogue_pressure,
        element_allowlists,
    })
}
