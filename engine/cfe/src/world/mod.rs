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
    founding_reserves, ClosureEvent, ColonyOrb, Resource, Station, Zone, ZoneKind,
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
pub mod chemistry;
pub mod diplomacy;
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
pub mod fleet;
pub mod balance;

pub use replay::replay;
pub use fleet::hull_voice;
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
    amount: crate::dop::Positive,
    deal: u64,
) -> Result<(), ActionRefused> {
    haul(src, from, dst, to, resource, amount.value().clone(), deal, &ratio(0, 1))
}

/// E-APT-1 — cross-world ELEMENT haul with melt. Zero melt is the
/// local door; positive melt is MARKET-2 iceberg on open matter.
/// Demurrage does not touch the element ledger (31 §1).
#[allow(clippy::too_many_arguments)] // mirrors haul / aperture_transfer arity
pub fn element_aperture_transfer(
    src: &mut World,
    from: u64,
    dst: &mut World,
    to: u64,
    element: [u8; 32],
    amount: crate::dop::Positive,
    deal: u64,
    melt: &Ratio,
) -> Result<(), ActionRefused> {
    haul_element(
        src,
        from,
        dst,
        to,
        element,
        amount.value().clone(),
        deal,
        melt,
    )
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
    if *melt >= ratio(1, 1) {
        return Err(ActionRefused::EconomicallyWalled { melt: Box::new(melt.clone()) });
    }
    // FLEET-1: positive melt hauls need hull capacity (Phase 3).
    if *melt > ratio(0, 1) {
        fleet::require_hull_capacity(src, deal, &amount)?;
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
    // **#42 THE WALL** — a pair at WAR exchanges nothing, in either
    // direction, exactly as the elder's ward bars a crossing. Either
    // book's war is enough: it takes two to trade and one to refuse.
    // This is what makes war expensive — it costs the trade, not a
    // number chosen to make it costly.
    if src.at_war_with(dst.name()) || dst.at_war_with(src.name()) {
        return Err(ActionRefused::AtWar { partner: dst.name().to_string() });
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

/// E-APT-1 — the element twin of [`haul`]. Same shield/war/melt doors;
/// banks the open-matter ledger (no demurrage touch).
#[allow(clippy::too_many_arguments)]
fn haul_element(
    src: &mut World,
    from: u64,
    dst: &mut World,
    to: u64,
    element: [u8; 32],
    amount: Ratio,
    deal: u64,
    melt: &Ratio,
) -> Result<(), ActionRefused> {
    if *melt >= ratio(1, 1) {
        return Err(ActionRefused::EconomicallyWalled {
            melt: Box::new(melt.clone()),
        });
    }
    if *melt > ratio(0, 1) {
        fleet::require_hull_capacity(src, deal, &amount)?;
    }
    if src.ward_holds() || dst.ward_holds() {
        return Err(ActionRefused::TheWardHolds);
    }
    if src.at_war_with(dst.name()) || dst.at_war_with(src.name()) {
        return Err(ActionRefused::AtWar {
            partner: dst.name().to_string(),
        });
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
    {
        let source = src
            .orbs
            .iter()
            .find(|held| held.orb_id == from)
            .expect("checked");
        let have = source.element_amount(&element);
        if have < amount {
            return Err(ActionRefused::CannotAffordElement {
                element,
                need: Box::new(amount),
                have: Box::new(have),
            });
        }
    }
    let src_stamp = src.clock.now();
    let dst_stamp = dst.clock.now();
    {
        let source = src
            .orbs
            .iter_mut()
            .find(|held| held.orb_id == from)
            .expect("checked");
        source.bank_element(element, -amount.clone());
    }
    src.chronicle.append(
        ClosureEvent::ElementApertureSent {
            deal,
            from_orb: from,
            element,
            amount: amount.clone(),
        },
        src_stamp,
        vec![SOVEREIGN],
    );
    let delivered = amount.clone() * (ratio(1, 1) - melt.clone());
    {
        let target = dst
            .orbs
            .iter_mut()
            .find(|held| held.orb_id == to)
            .expect("checked");
        target.bank_element(element, delivered.clone());
    }
    dst.chronicle.append(
        ClosureEvent::ElementApertureReceived {
            deal,
            to_orb: to,
            element,
            amount: delivered,
        },
        dst_stamp,
        vec![SOVEREIGN],
    );
    // THRESHOLD-1: element trade is also a crossing.
    src.crossed = true;
    dst.crossed = true;
    Ok(())
}

/// E-ROUTE-1 — charter a ROUTE-1 lane specialized to one element and
/// rail. Same alloy cost and dual-book pattern as [`charter_route`].
/// Rail must be 1..=4 (`Rail::from_wire`).
#[allow(clippy::too_many_arguments)]
pub fn charter_element_route(
    a: &mut World,
    orb_a: u64,
    b: &mut World,
    orb_b: u64,
    element: [u8; 32],
    eta: crate::dop::Unit,
    rail: u8,
    deal: u64,
) -> Result<(), ActionRefused> {
    if crate::dop::Rail::from_wire(rail).is_none() {
        return Err(ActionRefused::UnknownAxisValue { axis: 0, code: rail });
    }
    if !a.knows_its_sun() || !b.knows_its_sun() {
        return Err(ActionRefused::Shielded {
            source_shielded: !a.knows_its_sun(),
            dest_shielded: !b.knows_its_sun(),
        });
    }
    if a.routes.iter().any(|(held, _)| *held == deal)
        || b.routes.iter().any(|(held, _)| *held == deal)
        || a.element_routes.iter().any(|(held, _, _, _)| *held == deal)
        || b.element_routes.iter().any(|(held, _, _, _)| *held == deal)
    {
        return Err(ActionRefused::RouteKnown { deal });
    }
    // Same alloy charter bill as ROUTE-1 — one function, one literal.
    let cost = route_charter_cost();
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
            ClosureEvent::UpkeepSettled {
                orb_id: orb,
                resource: Resource::Alloy,
                amount: cost.clone(),
            },
            stamp,
            vec![SOVEREIGN],
        );
        world.routes.push((deal, eta.clone()));
        world.element_routes.push((deal, eta.clone(), element, rail));
        world.chronicle.append(
            ClosureEvent::ElementRouteChartered {
                deal,
                element,
                eta: eta.clone(),
                rail,
            },
            stamp,
            vec![SOVEREIGN],
        );
    }
    Ok(())
}

/// STAMP-1 — convert crystal into STAMP at identity rate (amount ==
/// crystal_spent). No free mint; no undervied curve dial.
pub fn mint_stamp_from_crystal(
    world: &mut World,
    orb: u64,
    amount: crate::dop::Positive,
) -> Result<(), ActionRefused> {
    let spent = amount.value().clone();
    let held = peel(&mut world.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
    let have = held.banked_amount(Resource::Crystal);
    if have < spent {
        return Err(ActionRefused::CannotAfford {
            resource: Resource::Crystal,
            need: Box::new(spent),
            have: Box::new(have),
        });
    }
    held.bank(Resource::Crystal, -spent.clone());
    held.bank_stamp(spent.clone());
    let stamp = world.clock.now();
    world.chronicle.append(
        ClosureEvent::StampMinted {
            orb_id: orb,
            amount: spent.clone(),
            crystal_spent: spent,
        },
        stamp,
        vec![SOVEREIGN],
    );
    Ok(())
}

/// STAMP aperture — cross-world STAMP move with melt (same doors as
/// resource haul; STAMP has no demurrage).
#[allow(clippy::too_many_arguments)]
pub fn stamp_aperture_transfer(
    src: &mut World,
    from: u64,
    dst: &mut World,
    to: u64,
    amount: crate::dop::Positive,
    deal: u64,
    melt: &Ratio,
) -> Result<(), ActionRefused> {
    if *melt >= ratio(1, 1) {
        return Err(ActionRefused::EconomicallyWalled {
            melt: Box::new(melt.clone()),
        });
    }
    let amount_check = amount.value().clone();
    if *melt > ratio(0, 1) {
        fleet::require_hull_capacity(src, deal, &amount_check)?;
    }
    if src.ward_holds() || dst.ward_holds() {
        return Err(ActionRefused::TheWardHolds);
    }
    if src.at_war_with(dst.name()) || dst.at_war_with(src.name()) {
        return Err(ActionRefused::AtWar {
            partner: dst.name().to_string(),
        });
    }
    if !src.knows_its_sun() || !dst.knows_its_sun() {
        return Err(ActionRefused::Shielded {
            source_shielded: !src.knows_its_sun(),
            dest_shielded: !dst.knows_its_sun(),
        });
    }
    let amount = amount.value().clone();
    {
        let source = src
            .orbs
            .iter()
            .find(|h| h.orb_id == from)
            .ok_or(ActionRefused::NoSuchOrb { orb: from })?;
        if source.stamp_amount() < amount {
            return Err(ActionRefused::CannotAfford {
                resource: Resource::Crystal, // typed refuse; stamp is not Resource
                need: Box::new(amount.clone()),
                have: Box::new(source.stamp_amount()),
            });
        }
    }
    if !dst.orbs.iter().any(|h| h.orb_id == to) {
        return Err(ActionRefused::NoSuchOrb { orb: to });
    }
    let src_stamp = src.clock.now();
    let dst_stamp = dst.clock.now();
    {
        let source = src.orbs.iter_mut().find(|h| h.orb_id == from).expect("ok");
        source.bank_stamp(-amount.clone());
    }
    src.chronicle.append(
        ClosureEvent::StampApertureSent {
            deal,
            from_orb: from,
            amount: amount.clone(),
        },
        src_stamp,
        vec![SOVEREIGN],
    );
    let delivered = amount * (ratio(1, 1) - melt.clone());
    {
        let target = dst.orbs.iter_mut().find(|h| h.orb_id == to).expect("ok");
        target.bank_stamp(delivered.clone());
    }
    dst.chronicle.append(
        ClosureEvent::StampApertureReceived {
            deal,
            to_orb: to,
            amount: delivered,
        },
        dst_stamp,
        vec![SOVEREIGN],
    );
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
    let cost = route_charter_cost();
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

/// ROUTE-1 / E-ROUTE-1 alloy charter bill — one literal for both
/// charter paths (call-not-copy; ratchet).
fn route_charter_cost() -> Ratio {
    ratio(4, 1)
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
                self.element_routes.retain(|(held_deal, _, _, _)| *held_deal != deal);
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

/// Phase-2 (33 / TASKS_TRADE_FLEET_DESK): base ticks per orbital slot
/// step — the year-voice length (`YEAR_TICKS` / journal ring). Not a
/// free dial: colony loop already segments years at 60. Individual
/// orbits scale this by core magnetism, star dynamo, and polity
/// disposition ([`orbit_phase_period`]) so not every ring advances
/// in lockstep.
pub const ORBIT_PHASE_TICKS: u64 = 60;

/// Alignment band for desk and route windows (derived from `D_orbit`,
/// not a second melt formula).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// Orbital separation in the lower quarter of a turn-scale.
    Cheap,
    /// Mid band.
    Fair,
    /// Wide orbital separation (dearest hauls).
    Dear,
    /// One or both ends unsited — no orbital term.
    Unplaced,
}

impl Alignment {
    pub fn word(self) -> &'static str {
        match self {
            Alignment::Cheap => "CHEAP",
            Alignment::Fair => "FAIR",
            Alignment::Dear => "DEAR",
            Alignment::Unplaced => "—",
        }
    }
}

/// **Core magnetism grade** — ring depth of the sited orbit on the
/// orbit grain (`ORBIT_GRAIN` = 64). Farther rings couple more weakly
/// to the stellar dynamo and take longer per θ step. Floor one grain
/// step so an orbit always has a period (never zero).
pub fn core_magnetism(site: &crate::dop::ThermalSite) -> u64 {
    use num_bigint::BigInt;
    use num_traits::ToPrimitive;
    let grid = BigInt::from(crate::almanac::ORBIT_GRAIN);
    let r = site.orbit_r.value();
    let stepped = r.numer() * &grid / r.denom();
    let at_least = if stepped < BigInt::from(1) {
        BigInt::from(1)
    } else {
        stepped
    };
    // Lawful grain steps fit u64; overflow clamps to u64::MAX (still ≥ 1).
    at_least.to_u64().unwrap_or(u64::MAX).max(1)
}

/// **Polity offset of the star's world** — sealed genesis disposition
/// (1 expansionist · 2 empiricist · 3 epicurean). Stretches the period
/// so different civil characters under different suns do not share a
/// single clockwork. Unsealed worlds read as 1 (no extra stretch).
pub fn star_polity_offset(world: &World) -> u64 {
    world
        .genesis
        .as_ref()
        .map(|g| u64::from(g.disposition.max(1)))
        .unwrap_or(1)
}

/// **Ticks per θ-slot step** for one world's sited genesis orb.
///
/// ```text
/// period = max(1, ORBIT_PHASE_TICKS × core_magnetism × polity
///                  / star.magnetism())
/// ```
///
/// - **core** — ring depth (grain steps of `orbit_r`)
/// - **polity** — sealed disposition under this star
/// - **star.magnetism** — dynamo grade (`StarClass::magnetism`);
///   stronger main-sequence fields (M > G > O) couple rings tighter
///   → shorter periods
///
/// Pure reading; no receipt. Different (class, ring, polity) triples
/// advance at different rates → relative `D_orbit` moves over time.
pub fn orbit_phase_period(world: &World, site: &crate::dop::ThermalSite) -> u64 {
    let core = core_magnetism(site);
    let polity = star_polity_offset(world);
    let star_m = world
        .star
        .map(|c| u64::from(c.magnetism().max(1)))
        .unwrap_or(1);
    let num = ORBIT_PHASE_TICKS
        .saturating_mul(core)
        .saturating_mul(polity);
    (num / star_m).max(1)
}

/// Phase advance on a sited θ under this world's star + polity + core.
/// `index + floor(now / period)` wrapped on the turn lattice. Pure
/// reading — no receipt (Q-F1 / design: derived from clock + site +
/// dynamo law).
fn phased_theta(
    world: &World,
    site: &crate::dop::ThermalSite,
    now: u64,
) -> Option<crate::dop::Slot> {
    let base = site.theta.as_ref()?;
    let period = orbit_phase_period(world, site);
    let steps = (now / period) as i64;
    Some(crate::dop::Slot::from_index(base.index() + steps))
}

/// Thermal site as the orbit law sees it at `now` (θ advanced; ψ fixed).
fn phased_site(
    world: &World,
    site: &crate::dop::ThermalSite,
    now: u64,
) -> crate::dop::ThermalSite {
    let mut out = site.clone();
    out.theta = phased_theta(world, site, now);
    out
}

/// **D_orbit** — wrapped L1 separation of the two worlds' genesis orbs
/// at their phased places. Zero when either is unsited / unplaced
/// (33 §3.1: not "far", unplaced). Each end advances on its own
/// period (core × polity ÷ star magnetism).
pub fn d_orbit(a: &World, b: &World) -> Ratio {
    let (Some(site_a), Some(site_b)) = (
        a.orb().and_then(|o| o.thermal.as_ref()),
        b.orb().and_then(|o| o.thermal.as_ref()),
    ) else {
        return ratio(0, 1);
    };
    let sa = phased_site(a, site_a, a.now());
    let sb = phased_site(b, site_b, b.now());
    crate::dop::separation(&sa, &sb).unwrap_or_else(|| ratio(0, 1))
}

/// **D_eff** — the one effective transit distance (33 §3.2):
/// `D_chart + D_orbit`. Call-not-copy for every melt path.
pub fn effective_distance(a: &World, b: &World, d_chart: &Ratio) -> Ratio {
    d_chart.clone() + d_orbit(a, b)
}

/// Alignment band from current `D_orbit` (exact thresholds: 1/4, 1/2).
pub fn alignment_of(a: &World, b: &World) -> Alignment {
    let (Some(_), Some(_)) = (
        a.orb().and_then(|o| o.thermal.as_ref()).and_then(|s| s.theta.as_ref()),
        b.orb().and_then(|o| o.thermal.as_ref()).and_then(|s| s.theta.as_ref()),
    ) else {
        return Alignment::Unplaced;
    };
    let d = d_orbit(a, b);
    // Thresholds: quarter and half of a turn — built from 1+1, no dials.
    let two = ratio(1, 1) + ratio(1, 1);
    let four = two.clone() + two.clone();
    let quarter = ratio(1, 1) / four;
    let half = ratio(1, 1) / two;
    if d < quarter {
        Alignment::Cheap
    } else if d < half {
        Alignment::Fair
    } else {
        Alignment::Dear
    }
}

/// Projected melt at a future home clock (peer clock held). Pure.
pub fn route_melt_at(
    a: &World,
    b: &World,
    d_chart: &Ratio,
    home_now: u64,
) -> Ratio {
    // Temporarily phase home against an explicit now by constructing
    // D_orbit with a synthetic read: recompute separation with home
    // phased to home_now and peer to its own now.
    let (Some(site_a), Some(site_b)) = (
        a.orb().and_then(|o| o.thermal.as_ref()),
        b.orb().and_then(|o| o.thermal.as_ref()),
    ) else {
        return pair_melt(a, b, d_chart);
    };
    let sa = phased_site(a, site_a, home_now);
    let sb = phased_site(b, site_b, b.now());
    let orbit = crate::dop::separation(&sa, &sb).unwrap_or_else(|| ratio(0, 1));
    let d_eff = d_chart.clone() + orbit;
    pair_melt(a, b, &d_eff)
}

/// **Route melt** — the desk and market path: `pair_melt` over
/// [`effective_distance`]. One producer of the iceberg number for
/// interplanetary economics.
pub fn route_melt(a: &World, b: &World, d_chart: &Ratio) -> Ratio {
    pair_melt(a, b, &effective_distance(a, b, d_chart))
}

/// MARKET-2: THE pair's melt fraction — the one producer of the
/// formula. The transit reads the whole manifold gap (spatial L1 +
/// orbital phase + the wrapped key gap: 11 §2.1's clash consumes
/// payload, never a tertiary sink) at φ = 1/64 per unit; the best
/// lane BOTH books hold (ROUTE-1) suppresses it. At ≥ 1 the pair is
/// economically WALLED by decay physics.
///
/// **Callers that have a chart distance should prefer [`route_melt`],**
/// which folds Phase-2 `D_orbit` into the distance argument. Passing
/// raw `D_chart` alone understates cost when both ends are sited.
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
    // O6.1 T4 Caravan — office advantage (not a gate): either end
    // holding caravan further suppresses melt by one instrument grade
    // of η. Grade is the Resonator's crystal yield (landed table), not
    // a new dial.
    let caravan = a.stations.iter().any(|(_, s)| *s == Station::Caravan.as_wire())
        || b.stations.iter().any(|(_, s)| *s == Station::Caravan.as_wire());
    if caravan {
        let grade = ZoneKind::Resonator.yield_per_tick().1;
        melt *= ratio(1, 1) - grade;
    }
    // FLEET-1 / Q-F2: hull η stacks as product of complements with
    // route η — best hull on either book (idle or assigned).
    let hull_eta = a
        .hulls
        .iter()
        .chain(b.hulls.iter())
        .map(|h| crate::dop::hull_eta(h.role, h.grade))
        .max()
        .unwrap_or_else(|| ratio(0, 1));
    if hull_eta > ratio(0, 1) {
        melt *= ratio(1, 1) - hull_eta;
    }
    // **#42 THE PACTS** — what two worlds have AGREED the crossing
    // costs. A trade or defense pact eases the melt; a war burdens it
    // (for whatever still crosses before the wall refuses it).
    //
    // BOTH BOOKS MUST HOLD IT. A treaty is a pair object and neither
    // world owns it, so one side's signature moves nothing — which is
    // exactly why a pact is worth having and a decree is not.
    let ours = a.pair_factor(b.name(), crate::treaty::PAIR_MELT);
    let theirs = b.pair_factor(a.name(), crate::treaty::PAIR_MELT);
    if ours == theirs {
        melt *= ours;
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
        .ok_or(ActionRefused::NoSuchOffer { offer: offer_a })?;
    let held_b = b
        .offers
        .iter()
        .find(|held| held.offer == offer_b)
        .cloned()
        .ok_or(ActionRefused::NoSuchOffer { offer: offer_b })?;
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
    // Phase-2: distance arg is D_chart; melt sees D_eff.
    let melt = route_melt(a, b, &distance);
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
    // A FACT, now honestly named: the destination already holds this
    // node. It wore the transfer refusal's word for want of its own.
    if dst.techs.iter().any(|(_, _, held, _)| *held == node) {
        return Err(ActionRefused::NodeAlreadyHeld { condition });
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
    // A licensed node carries the SAME mechanism a crystallized one
    // does — the receipt asserts the table's magnitude, and the tick
    // obeys the receipt (receipt authority, operator ruling
    // 2026-08-05). Pre-ruling licensed receipts asserted 1, and 1 they
    // remain: history stands as written.
    let (modifier, magnitude) =
        crate::dop::tech_node_of(condition).unwrap_or((0, ratio(1, 1)));
    dst.techs.push((to, condition, node, magnitude.clone()));
    dst.chronicle.append(
        ClosureEvent::TechDiscovered {
            orb_id: to,
            node,
            condition,
            modifier,
            magnitude,
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
    // ONE TOUCH, ONE RECEIPT (the demurrage seam, closed 2026-08-06):
    // every settled resource rides one DemurrageTouched, because the
    // anchor moves once per touch. The per-row form let a judge see a
    // prior the first row had already anchored, so a dual-resource
    // touch's second row always read elapsed 0 and refused lawful
    // decay. A zero-due touch still anchors silently — the carrier
    // deed's own replay arm carries the anchor.
    let decays = demurrage_due(orb, star, stamp, ease, supply);
    if !decays.is_empty() {
        for (resource, decay) in &decays {
            orb.bank(*resource, -decay.clone());
        }
        chronicle.append(
            ClosureEvent::DemurrageTouched { orb_id: orb.orb_id, decays },
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
