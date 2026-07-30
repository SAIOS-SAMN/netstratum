//! NS-2 RQP — the resonance query (N4). The lens beside the mirror:
//! a PURE structural read of the committed orb population, addressed
//! by sigil (frame-invariant canonical form), O(bucket), never a scan
//! of the population. Nothing here mutates and nothing here receipts —
//! the one chronicle event of RQP (the crystal draw, NS-2 §1.5) lives
//! in `world::scan`, which pays before it calls in.
//!
//! Composed FROM the v0.4 sigil index (`iae::reduction`, the SIGIL1
//! block — CHARTER §8): the address is projections of ONE joint
//! canonical shadow, never independent marginals (the
//! dimensional-collapse warning); T is excluded from the key — you
//! query an orb by WHAT it is (S, G), never by WHEN it was found.

use crate::dop::{ColonyOrb, Resource};
use crate::ratio::{ratio, Ratio};
use crate::thermal::{band_of, t_eq4, StarClass, SubstrateBand};
use std::collections::BTreeMap;

/// The chart's edge (the 8×8 grid — `clients/cli` renders the same).
const GRID: u8 = 8;

/// The sigil address — the bucket key: two projections of the joint
/// shadow (§1.1). Coarse on purpose: presence, not amounts — exactness
/// lives in the joint comparison.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address {
    /// S-canonical: the (row, col) list of the joint winner — zone
    /// geometry under the grid symmetry, kinds and amounts shed.
    pub s_canonical: Vec<(u8, u8)>,
    /// G-canonical: the ascending resource codes of the nonzero
    /// banked ledger.
    pub g_resources: Vec<u8>,
    /// The thermal class: 0 unbanded (unsited or starless), else the
    /// substrate band (1 cryogenic ‖ 2 mesic ‖ 3 hyperthermal).
    pub thermal: u8,
}

/// The joint canonical shadow — the resonance check. Two orbs resonate
/// iff these are EQUAL: no threshold, no score, no similarity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Joint {
    /// Lex-min over the eight grid symmetries of the sorted
    /// (row, col, kind) tuples. Kind is the LAST component, so the
    /// position sequence settles first — the S projection of this
    /// winner IS the position-only lex-min.
    pub zones: Vec<(u8, u8, u8)>,
    /// The banked ledger, resource-ascending, zeros absent (the NS-1
    /// canonical order, carried as (code, amount)).
    pub banked: Vec<(u8, Ratio)>,
    /// The thermal class, same coding as the address.
    pub thermal: u8,
}

/// An orb's sigil: its address and its joint shadow, computed pure at
/// read time — never stored, never receipted (the derived-reading law).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sigil {
    pub address: Address,
    pub joint: Joint,
}

/// A near-miss: same bucket, joint mismatch — refused WITH ITS DELTA
/// WHOLE (NS-2 §1.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NearMiss {
    pub orb: u64,
    /// Probe minus candidate, one entry per differing resource — the
    /// full list, never a truncation.
    pub banked_residuals: Vec<(Resource, Ratio)>,
    /// Canonical tiles where the joint winners disagree:
    /// (tile, probe kind, candidate kind) — kind 0 for an unzoned side.
    pub zone_deltas: Vec<(u16, u8, u8)>,
    /// (probe class, candidate class) — equal in v1 (the band keys the
    /// bucket); carried so the surface holds if a ruling widens it.
    pub thermal: (u8, u8),
    /// C(Δ) = Σ R² over the banked residuals — the coherence-functional
    /// shape (exact rational, zero iff the ledgers agree), computed
    /// local: cfe carries no rfa-core dependency. THE SUMMARY, never
    /// the payload.
    pub coherence: Ratio,
}

/// One scan's reading (NS-2 §1.3–§1.4). Both lists empty = the empty
/// bucket, its own typed verdict: NO delta against the population —
/// nearest-candidate search would be the very scan RQP refuses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanRead {
    /// The CrystalDrawn receipt's stamp: WHEN this lens was paid for —
    /// ledger-side annotation, never a key.
    pub stamp: u64,
    pub probed: Address,
    /// Every resonant orb, bucket order — nothing truncates silently.
    pub hits: Vec<u64>,
    pub refusals: Vec<NearMiss>,
    /// G3 (NS-2 §4): per-hit translation survival, parallel to `hits`
    /// — the prober's basis against the candidate's, the loss shown
    /// never silent. Additive: pre-G3 readers ignore it.
    pub translations: Vec<(u64, Ratio)>,
}

/// The eight grid symmetries — the plane group of the square chart,
/// a structural constant bounded by group order (the v0.4 law: the
/// only sweep is over the OBSERVER's frame group, never content).
fn transforms(row: u8, col: u8) -> [(u8, u8); 8] {
    let last = GRID - 1;
    [
        (row, col),
        (col, last - row),
        (last - row, last - col),
        (last - col, row),
        (row, last - col),
        (last - row, col),
        (col, row),
        (last - col, last - row),
    ]
}

/// The orb's sigil with no star context (unsited semantics): the pure
/// form the witnesses measure. Inside a world, prefer `sigil_of`.
pub fn orb_sigil(orb: &ColonyOrb) -> Sigil {
    sigil_of(orb, None)
}

/// The orb's sigil under a world's star: the joint shadow and its two
/// address projections (NS-2 §1.1).
pub fn sigil_of(orb: &ColonyOrb, star: Option<StarClass>) -> Sigil {
    let thermal = thermal_class(orb, star);
    let zones = joint_zones(orb);
    let s_canonical = zones.iter().map(|(row, col, _)| (*row, *col)).collect();
    let banked: Vec<(u8, Ratio)> = orb
        .banked
        .iter()
        .map(|(resource, amount)| (*resource as u8, amount.clone()))
        .collect();
    let g_resources = banked.iter().map(|(code, _)| *code).collect();
    Sigil {
        address: Address { s_canonical, g_resources, thermal },
        joint: Joint { zones, banked, thermal },
    }
}

/// Lex-min over the eight symmetries of the sorted (row, col, kind)
/// tuples. Minimizing the full tuple list restricts to the positional
/// lex-min coset first (kind is last), then settles kinds within it —
/// one pass, coset-respecting, no two-phase search.
fn joint_zones(orb: &ColonyOrb) -> Vec<(u8, u8, u8)> {
    (0..8)
        .map(|way| {
            let mut mapped: Vec<(u8, u8, u8)> = orb
                .zones
                .iter()
                .map(|zone| {
                    let (row, col) = (zone.tile as u8 / GRID, zone.tile as u8 % GRID);
                    let (to_row, to_col) = transforms(row, col)[way];
                    (to_row, to_col, zone.kind as u8)
                })
                .collect();
            mapped.sort_unstable();
            mapped
        })
        .min()
        .unwrap_or_default()
}

/// The thermal class facet: 0 unbanded (unsited, starless, or off the
/// banded range), else the substrate band code.
fn thermal_class(orb: &ColonyOrb, star: Option<StarClass>) -> u8 {
    let (Some(site), Some(class)) = (&orb.thermal, star) else {
        return 0;
    };
    match band_of(&t_eq4(&class.luminosity(), &site.albedo, &site.orbit_r)) {
        Some(SubstrateBand::Cryogenic) => 1,
        Some(SubstrateBand::Mesic) => 2,
        Some(SubstrateBand::HyperThermal) => 3,
        None => 0,
    }
}

/// The sigil index: every orb bucketed by address, insertion order
/// held. Built pure over the population at read time — a store of
/// orbs is self-indexing by geometry (SIGIL1); nothing persists.
pub struct SigilIndex {
    buckets: BTreeMap<Address, Vec<(u64, Joint)>>,
}

impl SigilIndex {
    pub fn build(orbs: &[ColonyOrb], star: Option<StarClass>) -> Self {
        let mut buckets: BTreeMap<Address, Vec<(u64, Joint)>> = BTreeMap::new();
        for orb in orbs {
            let sigil = sigil_of(orb, star);
            buckets.entry(sigil.address).or_default().push((orb.orb_id, sigil.joint));
        }
        SigilIndex { buckets }
    }

    /// One bucket, O(bucket): the candidates at an address.
    fn bucket(&self, address: &Address) -> &[(u64, Joint)] {
        self.buckets.get(address).map(Vec::as_slice).unwrap_or(&[])
    }
}

/// The resonance read (NS-2 §1.2–§1.4), pure: the exemplar's structure
/// is the query; the exemplar never matches itself. Returns the probed
/// address, the hits, and the near-miss refusals, bucket order.
pub fn resonance_read(
    exemplar: &ColonyOrb,
    orbs: &[ColonyOrb],
    star: Option<StarClass>,
) -> (Address, Vec<u64>, Vec<NearMiss>) {
    let probe = sigil_of(exemplar, star);
    let index = SigilIndex::build(orbs, star);
    let mut hits = Vec::new();
    let mut refusals = Vec::new();
    for (orb_id, joint) in index.bucket(&probe.address) {
        if *orb_id == exemplar.orb_id {
            continue;
        }
        match *joint == probe.joint {
            true => hits.push(*orb_id),
            false => refusals.push(near_miss(*orb_id, &probe.joint, joint)),
        }
    }
    (probe.address, hits, refusals)
}

/// The delta, whole (NS-2 §1.4): every differing banked entry as an
/// exact rational residual, every disagreeing canonical tile, the
/// thermal pair, and the summary scalar last.
fn near_miss(orb: u64, probe: &Joint, candidate: &Joint) -> NearMiss {
    let mut banked_residuals: Vec<(Resource, Ratio)> = Vec::new();
    let mut at_probe = probe.banked.iter().peekable();
    let mut at_candidate = candidate.banked.iter().peekable();
    loop {
        let (code, residual) = match (at_probe.peek(), at_candidate.peek()) {
            (Some((held, ours)), Some((theirs_code, theirs))) if held == theirs_code => {
                let entry = (*held, ours.clone() - theirs.clone());
                at_probe.next();
                at_candidate.next();
                entry
            }
            (Some((held, ours)), Some((theirs_code, _))) if held < theirs_code => {
                let entry = (*held, ours.clone());
                at_probe.next();
                entry
            }
            (Some(_), Some((theirs_code, theirs))) => {
                let entry = (*theirs_code, -theirs.clone());
                at_candidate.next();
                entry
            }
            (Some((held, ours)), None) => {
                let entry = (*held, ours.clone());
                at_probe.next();
                entry
            }
            (None, Some((theirs_code, theirs))) => {
                let entry = (*theirs_code, -theirs.clone());
                at_candidate.next();
                entry
            }
            (None, None) => break,
        };
        if residual != ratio(0, 1) {
            let resource = Resource::from_wire(code).expect("canonical ledgers carry known codes");
            banked_residuals.push((resource, residual));
        }
    }
    let mut zone_deltas: Vec<(u16, u8, u8)> = Vec::new();
    let mut ours = probe.zones.iter().peekable();
    let mut theirs = candidate.zones.iter().peekable();
    loop {
        let delta = match (ours.peek(), theirs.peek()) {
            (Some((row, col, held)), Some((to_row, to_col, other)))
                if (row, col) == (to_row, to_col) =>
            {
                let entry = match held == other {
                    true => None,
                    false => Some((tile_of(*row, *col), *held, *other)),
                };
                ours.next();
                theirs.next();
                entry
            }
            (Some((row, col, held)), Some((to_row, to_col, _)))
                if (row, col) < (to_row, to_col) =>
            {
                let entry = Some((tile_of(*row, *col), *held, 0));
                ours.next();
                entry
            }
            (Some(_), Some((to_row, to_col, other))) => {
                let entry = Some((tile_of(*to_row, *to_col), 0, *other));
                theirs.next();
                entry
            }
            (Some((row, col, held)), None) => {
                let entry = Some((tile_of(*row, *col), *held, 0));
                ours.next();
                entry
            }
            (None, Some((to_row, to_col, other))) => {
                let entry = Some((tile_of(*to_row, *to_col), 0, *other));
                theirs.next();
                entry
            }
            (None, None) => break,
        };
        if let Some(entry) = delta {
            zone_deltas.push(entry);
        }
    }
    let coherence = banked_residuals
        .iter()
        .fold(ratio(0, 1), |sum, (_, residual)| sum + residual.clone() * residual.clone());
    NearMiss {
        orb,
        banked_residuals,
        zone_deltas,
        thermal: (probe.thermal, candidate.thermal),
        coherence,
    }
}

fn tile_of(row: u8, col: u8) -> u16 {
    row as u16 * GRID as u16 + col as u16
}

/// NS-2 §2.2 — the query record (tag 56): the address only. A query
/// carries no amounts, no kinds, no T.
pub fn query_record_bytes(address: &Address) -> Vec<u8> {
    let mut value = (address.s_canonical.len() as u16).to_le_bytes().to_vec();
    for (row, col) in &address.s_canonical {
        value.push(*row);
        value.push(*col);
    }
    value.push(address.g_resources.len() as u8);
    value.extend_from_slice(&address.g_resources);
    value.push(address.thermal);
    let mut out = Vec::new();
    crate::tlv::put(56, &value, &mut out);
    out
}

/// NS-2 §2.3 — the hit record (tag 57): the resonant orb and the
/// reading's own stamp, ledger-side; the translation survival rides
/// BEHIND the pre-G3 prefix (additive — SEAL A on the wire: an old
/// reader parses orb ‖ stamp and never needs the tail).
pub fn hit_record_bytes(orb: u64, stamp: u64, survival: &Ratio) -> Vec<u8> {
    let mut value = orb.to_le_bytes().to_vec();
    value.extend_from_slice(&stamp.to_le_bytes());
    crate::ratio::encode(survival, &mut value);
    let mut out = Vec::new();
    crate::tlv::put(57, &value, &mut out);
    out
}

/// G3 (04 §1.3; NS-2 §4) — the cross-frame read, the N5-ready seam:
/// do two orbs resonate (joint shadows equal — the class survives
/// translation BY DEFINITION), and how much structure survives OUR
/// read of THEIR sky? Pure; the loss is the pair's distance, never a
/// property of either structure alone.
pub fn translated_read(
    ours: &ColonyOrb,
    our_basis: u8,
    theirs: &ColonyOrb,
    their_basis: u8,
) -> (bool, Ratio) {
    let resonant = orb_sigil(ours).joint == orb_sigil(theirs).joint;
    (resonant, crate::dop::translation_survival(our_basis, their_basis))
}

/// NS-2 §2.4 — the refusal record (tag 58): the residual list whole,
/// the scalar last, as summary.
pub fn refusal_record_bytes(miss: &NearMiss) -> Vec<u8> {
    let mut value = miss.orb.to_le_bytes().to_vec();
    value.push(miss.banked_residuals.len() as u8);
    for (resource, residual) in &miss.banked_residuals {
        value.push(*resource as u8);
        crate::ratio::encode(residual, &mut value);
    }
    value.push(miss.zone_deltas.len() as u8);
    for (tile, held, other) in &miss.zone_deltas {
        value.extend_from_slice(&tile.to_le_bytes());
        value.push(*held);
        value.push(*other);
    }
    value.push(miss.thermal.0);
    value.push(miss.thermal.1);
    crate::ratio::encode(&miss.coherence, &mut value);
    let mut out = Vec::new();
    crate::tlv::put(58, &value, &mut out);
    out
}
