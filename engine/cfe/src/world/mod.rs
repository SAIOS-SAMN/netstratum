//! The world — THE CONTAINER of the topology (04 §5): multiple colony
//! orbs (planets' settlements) under ONE clock, ONE chronicle, ONE
//! receipt chain (binding #1 — the world is the replay unit, the mesh
//! unit at N6, the anchor root at N7). The P1 landing paid the
//! registered naming debt by making the struct MEAN the container.
//!
//! The flow model (02 §2.3) runs PER ORB: each tick, every active zone
//! nets yield − upkeep into its orb's accrued workspace; famine strikes
//! per orb; `collect` settles one orb. Chartering (P1) founds a
//! charted colony by MOVING the founding reserves from a source
//! colony's bank — conservation holds; no colony after the first gets
//! free matter.

use crate::dop::{
    founding_reserves, ClosureEvent, ColonyOrb, Resource, Zone, ZoneKind,
};
use crate::ipsal::{Entity, Scope};
use crate::ratio::{ratio, Ratio};
use crate::vrp::{Attestation, Chronicle, Timekeeper, WorldClock};

/// The sovereign's entity id — the operator at the terminal, registered
/// at founding.
pub const SOVEREIGN: u64 = 1;

/// The genesis orb id — the first colony.
pub const FIRST_ORB: u64 = 1;


// GROUND §4c — the world is ONE ORGAN presenting ONE SURFACE. These
// modules are internal structure; every name is re-exported below so
// no caller's import changes (GROUND §3 invariant 1).
pub mod refusal;
pub mod rules;

pub use refusal::ActionRefused;
pub use rules::{
    demurrage_due, ease_of, in_crisis, standing_supply, ward_gate_depth, ward_progress,
    MintWarrant, NoWarrant,
};




pub mod accord;
pub mod books;
pub mod growth;
pub mod literature;
pub mod core;
pub mod economy;
pub mod gauge;
pub mod genesis;
pub mod survey;
pub mod tech;
pub mod thermal;
pub mod tick;
pub mod zones;
pub mod replay;
pub mod state;
pub mod stock;

pub use replay::replay;
pub use state::{
    AbsorptionDebt, Bond, Committed, Council, Forward, Genesis, Leap, Offer, Prospected,
    TickOutcome, World,
};
// Internal to the organ — never re-exported (GROUND §3 invariant 1:
// the parts are structure, not a new seam for other crates).
use state::CrystalDeposit;



















































































































/// NS-4 APERTURE (BUILD_ORDER §3a) — THE CROSS-WORLD VALUE CARRIER.
/// Banked value crosses from an orb in ONE world to an orb in ANOTHER,
/// conservation-exact across the pair, receipted in BOTH chronicles as
/// a two-legged deed sharing one `deal` id: `ApertureSent` debits the
/// source, `ApertureReceived` credits the destination. Unlike TRADE-1
/// (one chronicle proves conservation), no single chronicle proves it
/// here — the deal id links the legs (double entry across two books).
/// Both orbs settle due demurrage at the touch (the SINK-1 law) before
/// value moves; affordability is gated on the source's post-decay
/// bank, computed pure, so a refusal orphans nothing in either world.
/// The cross-empire cohesion books (compassion/morality) DO NOT post
/// here: cross-world reading composes at N5/N6 (09 board law) — an
/// aperture carries value, never judgment.
pub fn aperture_transfer(
    src: &mut World,
    from: u64,
    dst: &mut World,
    to: u64,
    resource: Resource,
    amount: Ratio,
    deal: u64,
) -> Result<(), ActionRefused> {
    haul(src, from, dst, to, resource, amount, deal, &ratio(0, 1))
}

/// MARKET-2 (ruled 2026-07-31): the melt coefficient φ — the fraction
/// of cargo the void takes per unit of L1 distance (spatial + key).
/// Space transport consumes its own payload (Samuelson's iceberg);
/// at melt ≥ 1 trade walls off by decay physics alone.
pub const MELT_PER_UNIT: (i64, i64) = (1, 64);

/// The one transport body. `melt` is the transit's iceberg fraction:
/// the sender debits the DISPATCHED amount, the receiver banks and
/// receipts the DELIVERED amount — the difference is the void's,
/// derivable from the pair (sent − received = melt·sent), so the
/// cross-book audit stays exact. Zero melt is the local door.
#[allow(clippy::too_many_arguments)]
fn haul(
    src: &mut World,
    from: u64,
    dst: &mut World,
    to: u64,
    resource: Resource,
    amount: Ratio,
    deal: u64,
    melt: &Ratio,
) -> Result<(), ActionRefused> {
    // P2: each world's standing supply, snapshotted before either side
    // is peeled. A haul crosses two worlds and each sink reads its OWN
    // supply — a busy world's rate must not be charged to a quiet one.
    let src_supply = standing_supply(&src.orbs);
    let dst_supply = standing_supply(&dst.orbs);
    if amount <= ratio(0, 1) {
        return Err(ActionRefused::BadTransfer);
    }
    if *melt >= ratio(1, 1) {
        return Err(ActionRefused::EconomicallyWalled { melt: Box::new(melt.clone()) });
    }
    // THRESHOLD-1 (12_UNION §7): the shield — an aperture touches only
    // worlds whose peoples have made the solar leap (TECH-1a). A world
    // is a star system, so every aperture IS interstellar; the first
    // lawful one is a world's CROSSING into the union.
    // WARDEN-1a: the elder's reverse shield — no aperture reaches a
    // warded star, either direction, until the gate is solved.
    if src.ward_holds() || dst.ward_holds() {
        return Err(ActionRefused::TheWardHolds);
    }
    if !src.knows_its_sun() || !dst.knows_its_sun() {
        return Err(ActionRefused::Shielded {
            source_shielded: !src.knows_its_sun(),
            dest_shielded: !dst.knows_its_sun(),
        });
    }
    if !src.orbs.iter().any(|held| held.orb_id == from) {
        return Err(ActionRefused::NoSuchOrb { orb: from });
    }
    if !dst.orbs.iter().any(|held| held.orb_id == to) {
        return Err(ActionRefused::NoSuchOrb { orb: to });
    }
    let src_stamp = src.clock.now();
    let dst_stamp = dst.clock.now();
    // Affordability on the source's post-decay bank — honest at the
    // stamp, computed pure, so a refusal moves nothing in either world.
    {
        let source = src.orbs.iter().find(|held| held.orb_id == from).expect("checked");
        let decay = demurrage_due(source, src.star, src_stamp, &ease_of(&src.techs, from, src_stamp), &standing_supply(&src.orbs))
            .into_iter()
            .find(|(due, _)| *due == resource)
            .map(|(_, decay)| decay)
            .unwrap_or_else(|| ratio(0, 1));
        let have = source.banked_amount(resource) - decay;
        if have < amount {
            return Err(ActionRefused::CannotAfford {
                resource,
                need: Box::new(amount),
                have: Box::new(have),
            });
        }
    }
    // The touch (SINK-1) then the debit, in the source world.
    {
        let source = src.orbs.iter_mut().find(|held| held.orb_id == from).expect("checked");
        let ease = ease_of(&src.techs, from, src_stamp);
        touch(source, src.star, &mut src.chronicle, src_stamp, &ease, &src_supply);
        source.bank(resource, -amount.clone());
    }
    src.chronicle.append(
        ClosureEvent::ApertureSent { deal, from_orb: from, resource, amount: amount.clone() },
        src_stamp,
        vec![SOVEREIGN],
    );
    // The touch then the credit, in the destination world — the
    // DELIVERED amount: what the void melted never arrives, and the
    // receipt speaks only what landed (MARKET-2).
    let delivered = amount.clone() * (ratio(1, 1) - melt.clone());
    {
        let target = dst.orbs.iter_mut().find(|held| held.orb_id == to).expect("checked");
        let ease = ease_of(&dst.techs, to, dst_stamp);
        touch(target, dst.star, &mut dst.chronicle, dst_stamp, &ease, &dst_supply);
        target.bank(resource, delivered.clone());
    }
    let void_share = amount - delivered.clone();
    dst.chronicle.append(
        ClosureEvent::ApertureReceived { deal, to_orb: to, resource, amount: delivered },
        dst_stamp,
        vec![SOVEREIGN],
    );
    // The conform ruling (2026-07-31): the melt is a FIRST-CLASS
    // receipt on the receiving book — sent = received + void share,
    // so every deal sums to zero under one rule and the gate only
    // sums, never recomputes φ·D. Losses are receipted, never
    // silent. Banks never move on it.
    if void_share > ratio(0, 1) {
        dst.chronicle.append(
            ClosureEvent::VoidShare { orb_id: to, deal, resource, amount: void_share },
            dst_stamp,
            vec![SOVEREIGN],
        );
    }
    dst.relieve_recovered(to);
    // THRESHOLD-1: the deed IS the crossing, for both worlds.
    src.crossed = true;
    dst.crossed = true;
    Ok(())
}

/// ROUTE-1 (the MARKET-2 sequence): CHARTER A LANE — a two-legged
/// deed pairing both books on one deal id. The lane suppresses the
/// iceberg (melt × (1 − η)); η comes from the partners' technology
/// (the callers derive it from instrument grades — built, never
/// granted) and must sit inside [0, 1). The charter draws alloy 4
/// from each side (v1 placeholder — infrastructure is paid for);
/// maintenance is drawn at `maintain_routes` (the year voice), and
/// an unpaid lane LAPSES on its own book — a pair needs both legs.
pub fn charter_route(
    a: &mut World,
    orb_a: u64,
    b: &mut World,
    orb_b: u64,
    eta: crate::dop::Unit,
    deal: u64,
) -> Result<(), ActionRefused> {
    // BadEta is DEAD (29_CONSTRUCTION C1): efficiency is a `Unit` —
    // [0, 1) by construction — so the range judge has nothing to catch.
    if !a.knows_its_sun() || !b.knows_its_sun() {
        return Err(ActionRefused::Shielded {
            source_shielded: !a.knows_its_sun(),
            dest_shielded: !b.knows_its_sun(),
        });
    }
    if a.routes.iter().any(|(held, _)| *held == deal)
        || b.routes.iter().any(|(held, _)| *held == deal)
    {
        return Err(ActionRefused::RouteKnown { deal });
    }
    let cost = ratio(4, 1);
    for (world, orb) in [(&mut *a, orb_a), (&mut *b, orb_b)] {
        let held = peek(&world.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        let have = held.banked_amount(Resource::Alloy);
        if have < cost {
            return Err(ActionRefused::CannotAfford {
                resource: Resource::Alloy,
                need: Box::new(cost),
                have: Box::new(have),
            });
        }
    }
    for (world, orb) in [(&mut *a, orb_a), (&mut *b, orb_b)] {
        let held = peel(&mut world.orbs, orb).expect("checked");
        held.bank(Resource::Alloy, -cost.clone());
        let stamp = world.clock.now();
        world.chronicle.append(
            ClosureEvent::UpkeepSettled { orb_id: orb, resource: Resource::Alloy, amount: cost.clone() },
            stamp,
            vec![SOVEREIGN],
        );
        world.routes.push((deal, eta.clone()));
        world.chronicle.append(
            ClosureEvent::RouteChartered { deal, eta: eta.clone() },
            stamp,
            vec![SOVEREIGN],
        );
    }
    Ok(())
}

impl World {
    /// ROUTE-1: the year-voice maintenance draw — 1/4 joule per live
    /// lane (v1 placeholder). An unaffordable lane LAPSES by receipt
    /// on this book; the pair dies with either leg. Returns the
    /// lapsed deal ids.
    pub fn maintain_routes(&mut self, orb: u64) -> Vec<u64> {
        let toll = ratio(1, 4);
        let mut lapsed = Vec::new();
        let deals: Vec<u64> = self.routes.iter().map(|(deal, _)| *deal).collect();
        for deal in deals {
            let stamp = self.clock.now();
            let Some(held) = peel(&mut self.orbs, orb) else { break };
            if held.banked_amount(Resource::Joule) >= toll {
                held.bank(Resource::Joule, -toll.clone());
                self.chronicle.append(
                    ClosureEvent::UpkeepSettled {
                        orb_id: orb,
                        resource: Resource::Joule,
                        amount: toll.clone(),
                    },
                    stamp,
                    vec![SOVEREIGN],
                );
            } else {
                self.routes.retain(|(held_deal, _)| *held_deal != deal);
                self.chronicle.append(ClosureEvent::RouteLapsed { deal }, stamp, vec![SOVEREIGN]);
                lapsed.push(deal);
            }
        }
        lapsed
    }

    /// ROUTE-1: the live lanes — (deal, eta).
    pub fn routes(&self) -> &[(u64, crate::dop::Unit)] {
        &self.routes
    }
}

/// MARKET-2: THE pair's melt fraction — the one producer of the
/// formula. The transit reads the whole manifold gap (spatial L1 +
/// the wrapped key gap: 11 §2.1's clash consumes payload, never a
/// tertiary sink) at φ = 1/64 per unit; the best lane BOTH books
/// hold (ROUTE-1) suppresses it. At ≥ 1 the pair is economically
/// WALLED by decay physics.
pub fn pair_melt(a: &World, b: &World, distance: &Ratio) -> Ratio {
    let gap = key_distance(a.key(), b.key());
    let mut melt = (distance.clone() + gap) * ratio(MELT_PER_UNIT.0, MELT_PER_UNIT.1);
    let best_eta = a
        .routes
        .iter()
        .filter(|(deal, _)| b.routes.iter().any(|(held, _)| held == deal))
        .map(|(_, eta)| eta.clone())
        .max();
    if let Some(eta) = best_eta {
        melt *= ratio(1, 1) - eta.value().clone();
    }
    melt
}

/// THE SMELT (ruled 2026-07-31): every FILL lands on the 1/64
/// lattice the melt law speaks — floor(amount × 64)/64. GCD
/// reduction alone cannot bound growth: lowest terms themselves grow
/// when arbitrary rationals multiply through fills and premiums
/// (measured 2026-07-31: the open no-singles book stalled the beat
/// by ~6 crossings deep on compounding denominators — the demurrage
/// precedent at `demurrage_due`, now made a market law). Bounded
/// lattices keep replay and consilience verification affordable
/// forever — the POW++ requirement. The shaved dust never leaves the
/// giver's bank: nothing moves un-receipted, conservation holds by
/// construction, and a fill smelted to zero refuses whole (dust
/// never trades).
pub fn smelt(amount: &Ratio) -> Ratio {
    let grid = num_bigint::BigInt::from(64);
    Ratio::new(amount.numer() * &grid / amount.denom(), grid)
}

/// MARKET-1 (BUILD_ORDER §3a) — CLEAR A CROSSING: two resting offers
/// on two worlds meet; the fill happens at the RESTING offer's exact
/// price (price-time priority: the earlier posted rules; ties break
/// by lower offer id), moves as TWO aperture pairs (deal and deal+1 —
/// the two directions, each conservation-exact, shield-checked), and
/// closes both intents with OfferCleared. Fill = the largest exchange
/// both intents admit, SMELTED to the lattice; remainders are not
/// carried — an unfilled remainder reposts (offers are intents, not
/// escrows).
pub fn clear_trade(
    a: &mut World,
    offer_a: u64,
    b: &mut World,
    offer_b: u64,
    deal: u64,
    distance: Ratio,
) -> Result<(), ActionRefused> {
    let held_a = a
        .offers
        .iter()
        .find(|held| held.offer == offer_a)
        .cloned()
        .ok_or(ActionRefused::BadOffer)?;
    let held_b = b
        .offers
        .iter()
        .find(|held| held.offer == offer_b)
        .cloned()
        .ok_or(ActionRefused::BadOffer)?;
    // Complementary resources, or no crossing exists.
    if held_a.give != held_b.want || held_a.want != held_b.give {
        return Err(ActionRefused::NoCrossing);
    }
    // The prices must CROSS: B offers at least what A asks, per unit.
    // A asks want_a/give_a of Y per X; B gives give_b/want_b of Y per
    // X. Exact rational comparison, no division by zero (amounts are
    // positive by posting law).
    let asks = held_a.want_amount.clone() * held_b.want_amount.clone();
    let gives = held_b.give_amount.clone() * held_a.give_amount.clone();
    if gives < asks {
        return Err(ActionRefused::NoCrossing);
    }
    // The resting offer's price rules (price-time priority).
    let a_rests = (held_a.posted, held_a.offer) <= (held_b.posted, held_b.offer);
    // Fill x of A's give (X): bounded by both intents, SMELTED.
    let x = smelt(&held_a.give_amount.clone().min(held_b.want_amount.clone()));
    // y of B's give (Y) at the resting price, SMELTED after the
    // price multiply (the price is exact; the lattice bounds what
    // MOVES — the dust stays with the giver).
    let y = match a_rests {
        true => x.clone() * held_a.want_amount.clone() / held_a.give_amount.clone(),
        false => x.clone() * held_b.give_amount.clone() / held_b.want_amount.clone(),
    };
    let y = smelt(&y.min(held_b.give_amount.clone()));
    if x <= ratio(0, 1) || y <= ratio(0, 1) {
        return Err(ActionRefused::NoCrossing);
    }
    // MARKET-2 (ruled 2026-07-31) — THE ICEBERG: delivered = sent ×
    // (1 − φ·D). ONE producer of the melt number (the conform
    // ruling's spirit): clear_trade and every profit test read
    // pair_melt, nothing recomputes the formula.
    let melt = pair_melt(a, b, &distance);
    // The two directions — each an aperture pair, shield-checked,
    // demurrage-touched; sent − received = the void's exact share.
    haul(a, held_a.orb, b, held_b.orb, held_a.give, x, deal, &melt)?;
    haul(b, held_b.orb, a, held_a.orb, held_b.give, y, deal + 1, &melt)?;
    // Both intents close; replay removes them.
    a.offers.retain(|held| held.offer != offer_a);
    b.offers.retain(|held| held.offer != offer_b);
    let stamp_a = a.clock.now();
    a.chronicle.append(
        ClosureEvent::OfferCleared { offer: offer_a, deal },
        stamp_a,
        vec![SOVEREIGN],
    );
    let stamp_b = b.clock.now();
    b.chronicle.append(
        ClosureEvent::OfferCleared { offer: offer_b, deal },
        stamp_b,
        vec![SOVEREIGN],
    );
    Ok(())
}


/// PLANNER-1 (12_UNION §2) — LICENSE A TECHNOLOGY LEAP into another
/// world: the planner's issuing power, receipted in BOTH chronicles
/// under one deal (the aperture shape). The deed is council-gated —
/// it must reference an OPEN council in the granting world (delegation
/// settles inside diplomacy) — and shield-bounded: the receiving world
/// must know its sun AND have crossed (a licensed leap lands only on a
/// union member; below the threshold the young are left to their sky).
/// The solar leap (condition 0) is NEVER licensed. The leap may be
/// known or UNKNOWN to the receiver — the name and node arrive with
/// the deed; a node the receiver already holds refuses idempotent.
pub fn license_tech(
    src: &mut World,
    from: u64,
    dst: &mut World,
    to: u64,
    leap: Leap,
) -> Result<(), ActionRefused> {
    let Leap { condition, name, deal, council } = leap;
    if condition == 0 {
        return Err(ActionRefused::SolarLeapNeverLicensed);
    }
    // TERRA-1: the crown licenses only onto a finished base-7 stack —
    // the gift completes what the line earned; it never substitutes.
    if condition == 6 && !dst.base_seven_complete() {
        return Err(ActionRefused::BaseSevenUnfinished);
    }
    if !src.councils.iter().any(|pact| pact.council == council) {
        return Err(ActionRefused::NoSuchCouncil { council });
    }
    if dst.is_shielded() {
        return Err(ActionRefused::Shielded {
            source_shielded: src.is_shielded(),
            dest_shielded: true,
        });
    }
    if !dst.crossed {
        return Err(ActionRefused::BelowTheThreshold);
    }
    if !src.orbs.iter().any(|held| held.orb_id == from) {
        return Err(ActionRefused::NoSuchOrb { orb: from });
    }
    if !dst.orbs.iter().any(|held| held.orb_id == to) {
        return Err(ActionRefused::NoSuchOrb { orb: to });
    }
    // The node id is seeded from the licensing deed (SEAL B): the
    // granting chronicle's own digest never enters — the deal and
    // condition suffice, so both legs derive the SAME node.
    let mut seed = b"tech.licensed".to_vec();
    seed.extend_from_slice(&deal.to_le_bytes());
    seed.push(condition);
    let node: [u8; 32] = blake3::hash(&seed).into();
    if dst.techs.iter().any(|(_, _, held)| *held == node) {
        return Err(ActionRefused::BadTransfer);
    }
    let src_stamp = src.clock.now();
    let dst_stamp = dst.clock.now();
    src.chronicle.append(
        ClosureEvent::TechLicensed {
            deal,
            from_orb: from,
            council,
            condition,
            node,
            name: name.clone(),
        },
        src_stamp,
        vec![SOVEREIGN],
    );
    dst.techs.push((to, condition, node));
    dst.chronicle.append(
        ClosureEvent::TechDiscovered {
            orb_id: to,
            node,
            condition,
            modifier: 0,
            magnitude: ratio(1, 1),
            name,
        },
        dst_stamp,
        vec![SOVEREIGN],
    );
    Ok(())
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// Replay a chronicle into the committed projection it grounds
/// (CHARTER SEAL B) — every orb, every entity, from receipts alone.
/// AEON-1a: wrap a gauge value onto the torus fiber [0, 1) — exact
/// rational modulo 1, no floats, no drift.
pub fn wrap_gauge(gauge: Ratio) -> Ratio {
    gauge.clone() - gauge.floor()
}

/// GAUGE-N: wrapped L1 between two graded keys of ANY length — an
/// unopened plane reads ground (0); the short way around each circle,
/// summed whole (the L1 law on the fiber). Resonance and clash read
/// here (11 §2.1).
pub fn key_distance(a: &[Ratio], b: &[Ratio]) -> Ratio {
    let zero = ratio(0, 1);
    let one = ratio(1, 1);
    let mut total = ratio(0, 1);
    for at in 0..a.len().max(b.len()) {
        let held = a.get(at).unwrap_or(&zero);
        let far = b.get(at).unwrap_or(&zero);
        let d = wrap_gauge(held.clone() - far.clone());
        total += d.clone().min(one.clone() - d);
    }
    total
}

/// D3 (02 §2.5–2.6): the terrain-and-adjacency boost on a yield —
/// tile class first, then the 4-neighborhood: an extractor beside a
/// rich TILE runs ×5/4 (the seam runs under the fence); a habitat
/// beside an ACTIVE extractor runs ×3/4 (noise and dust). Upkeep never
/// scales; converters and heat-law zones are untouched (v1: terrain
/// speaks to extraction and agriculture — seams and basins). An
/// uncharted star reads 1 everywhere — the opt-in law.
fn terrain_boost(
    seed: &Option<[u8; 32]>,
    orb_id: u64,
    zone: &crate::dop::Zone,
    active: &[crate::dop::Zone],
) -> Ratio {
    use crate::dop::{tile_class, TileClass};
    let Some(seed) = seed else {
        return ratio(1, 1);
    };
    if !matches!(zone.kind, ZoneKind::Extractor | ZoneKind::Habitat) {
        return ratio(1, 1);
    }
    let mut boost = tile_class(seed, orb_id, zone.tile).multiplier();
    let beside = |tile: u16| -> Vec<u16> {
        let (row, col) = (tile / 8, tile % 8);
        let mut out = Vec::new();
        if row > 0 { out.push(tile - 8); }
        if row < 7 { out.push(tile + 8); }
        if col > 0 { out.push(tile - 1); }
        if col < 7 { out.push(tile + 1); }
        out
    };
    match zone.kind {
        ZoneKind::Extractor
            if beside(zone.tile)
                .into_iter()
                .any(|t| tile_class(seed, orb_id, t) == TileClass::Rich) =>
        {
            boost *= ratio(5, 4);
        }
        ZoneKind::Habitat
            if beside(zone.tile)
                .into_iter()
                .any(|t| active.iter().any(|z| z.tile == t && z.kind == ZoneKind::Extractor)) =>
        {
            boost *= ratio(3, 4);
        }
        _ => {}
    }
    boost
}

/// The ONE deposit rule — shared verbatim by the incremental cursor
/// and the pure full scan (same receipts, same sheet, by
/// construction). Window filtering happens at READ (condition 4) and
/// by pruning; the rule itself deposits everything typed.
fn deposit_receipt(sheet: &mut Vec<(u64, CrystalDeposit)>, receipt: &crate::vrp::Receipt) {
    fn deposit(sheet: &mut Vec<(u64, CrystalDeposit)>, orb: u64) -> &mut CrystalDeposit {
        if let Some(at) = sheet.iter().position(|(held, _)| *held == orb) {
            return &mut sheet[at].1;
        }
        sheet.push((orb, CrystalDeposit::default()));
        &mut sheet.last_mut().expect("just pushed").1
    }
    match &receipt.event {
        ClosureEvent::FamineStruck { orb_id, resource: Resource::Joule, .. } => {
            let held = deposit(sheet, *orb_id);
            held.famine_open = held.famine_open.or(Some(receipt.stamp));
        }
        ClosureEvent::FamineRelieved { orb_id, resource: Resource::Joule } => {
            let held = deposit(sheet, *orb_id);
            if let Some(struck) = held.famine_open.take() {
                held.famine_total += receipt.stamp.saturating_sub(struck);
            }
        }
        ClosureEvent::OverheatStruck { orb_id, .. } => {
            let held = deposit(sheet, *orb_id);
            held.overheat_open = held.overheat_open.or(Some(receipt.stamp));
        }
        ClosureEvent::OverheatCleared { orb_id } => {
            deposit(sheet, *orb_id).overheat_open = None;
        }
        ClosureEvent::VentSettled { orb_id, amount } => {
            deposit(sheet, *orb_id)
                .vented_entries
                .push((receipt.stamp, amount.clone()));
        }
        ClosureEvent::YieldCollected { orb_id, resource: Resource::Joule, amount } => {
            deposit(sheet, *orb_id)
                .encased_entries
                .push((receipt.stamp, amount.clone()));
        }
        ClosureEvent::ApertureSent { from_orb: orb_id, .. }
        | ClosureEvent::ApertureReceived { to_orb: orb_id, .. }
        | ClosureEvent::BondSealed { source_orb: orb_id, .. }
        | ClosureEvent::BondSettled { source_orb: orb_id, .. } => {
            deposit(sheet, *orb_id).last_link = Some(receipt.stamp);
        }
        ClosureEvent::TechDiscovered { orb_id, .. } => {
            deposit(sheet, *orb_id).last_node = Some(receipt.stamp);
        }
        _ => {}
    }
}




/// The touch law (SINK-1, extended by TRADE-1): settle an orb's due
/// demurrage by receipt and anchor `last_touch` — decay or not, a
/// touch anchors.
fn touch(
    orb: &mut ColonyOrb,
    star: Option<crate::thermal::StarClass>,
    chronicle: &mut Chronicle,
    stamp: u64,
    ease: &Ratio,
    supply: &[(Resource, Ratio)],
) {
    for (resource, decay) in demurrage_due(orb, star, stamp, ease, supply) {
        orb.bank(resource, -decay.clone());
        chronicle.append(
            ClosureEvent::DemurrageSettled { orb_id: orb.orb_id, resource, amount: decay },
            stamp,
            vec![],
        );
    }
    orb.last_touch = stamp;
}

/// ABSORB-1 (09 §2.4): a payment from debtor to creditor compensates
/// open reciprocity windows OLDEST FIRST; each paid portion posts the
/// debtor's returned book. Shared verbatim by the live path and replay.
fn settle_debts(
    debts: &mut Vec<AbsorptionDebt>,
    orbs: &mut [ColonyOrb],
    from: u64,
    to: u64,
    value: Ratio,
) {
    let zero = ratio(0, 1);
    let mut left = value;
    for debt in debts.iter_mut() {
        if left == zero {
            break;
        }
        if debt.debtor != from || debt.creditor != to {
            continue;
        }
        let pay = left.clone().min(debt.remainder.clone());
        if pay > zero {
            debt.remainder -= pay.clone();
            left -= pay.clone();
            if let Some(held) = peel(orbs, from) {
                held.returned_across += pay;
            }
        }
    }
    debts.retain(|debt| debt.remainder != ratio(0, 1));
}

/// IDEO-1 (09 §3.2, placeholder ruling): the measured boundary policy
/// for the friction law — returned/(returned+extracted); EMPTY books
/// read as 1 (a fresh orb trades frictionless until measured).
fn reciprocity_reading(orb: &ColonyOrb) -> Ratio {
    let denominator = orb.returned_across.clone() + orb.extracted_across.clone();
    match denominator > ratio(0, 1) {
        true => orb.returned_across.clone() / denominator,
        false => ratio(1, 1),
    }
}


/// PEEL — lift ONE orb out of the field by id: the single hand into
/// the collection (operator ruling 2026-07-30: no cartesian find-scan
/// repetition; peels and shears where there were iterations).
fn peel(orbs: &mut [ColonyOrb], orb: u64) -> Option<&mut ColonyOrb> {
    orbs.iter_mut().find(|held| held.orb_id == orb)
}

/// PEEK — the read-only peel.
fn peek(orbs: &[ColonyOrb], orb: u64) -> Option<&ColonyOrb> {
    orbs.iter().find(|held| held.orb_id == orb)
}

fn credit_forwards(
    forwards: &mut [Forward],
    from: u64,
    to: u64,
    resource: Resource,
    amount: &Ratio,
) {
    for pact in forwards.iter_mut() {
        if pact.obligor == from && pact.beneficiary == to && pact.resource == resource {
            pact.delivered += amount.clone();
        }
    }
}

/// COUNCIL-1: a delivery from one member to a FELLOW member credits
/// the giver's council quota (transfers and bond flow alike — the
/// forward precedent, widened to the group).
fn credit_councils(
    councils: &mut [Council],
    from: u64,
    to: u64,
    resource: Resource,
    amount: &Ratio,
) {
    for pact in councils.iter_mut() {
        if pact.resource == resource && pact.members.contains(&to) {
            if let Some(at) = pact.members.iter().position(|member| *member == from) {
                pact.delivered[at] += amount.clone();
            }
        }
    }
}

fn triple_amount(ledger: &[(u64, Resource, Ratio)], orb: u64, resource: Resource) -> Ratio {
    ledger
        .iter()
        .find(|(held, res, _)| *held == orb && *res == resource)
        .map(|(_, _, amount)| amount.clone())
        .unwrap_or_else(|| ratio(0, 1))
}

fn accrue(ledger: &mut Vec<(u64, Resource, Ratio)>, orb: u64, resource: Resource, amount: Ratio) {
    match ledger
        .iter_mut()
        .find(|(held, res, _)| *held == orb && *res == resource)
    {
        Some((_, _, held)) => *held += amount,
        None => {
            ledger.push((orb, resource, amount));
            ledger.sort_by_key(|(orb, resource, _)| (*orb, *resource));
        }
    }
    ledger.retain(|(_, _, amount)| amount != &ratio(0, 1));
}
