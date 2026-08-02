//! REPLAY — the transition's SECOND definition.
//!
//! **This file is a known defect, isolated so it can be seen.** The
//! live proposer in `deeds` and this function both define what a step
//! does; they agree only while every rule they share stays still.
//! Kernel task #30 exists to collapse them into one. Until it lands,
//! any change to a value in `rules` must be verified through BOTH.
//!
//! THE VENT (GROUND §4b): the measured coupling is narrower than it
//! looks. This body recomputes exactly ONE shared rule --
//! `crate::dop::commensurate_weight`, in the `ResourceTransferred`
//! arm. Every other arm APPLIES the receipted amount rather than
//! re-deriving it. So #30's surface is one call site, not 113 arms.

use super::*;

pub fn replay(chronicle: &Chronicle) -> Committed {
    let mut orbs: Vec<ColonyOrb> = Vec::new();
    let mut entities: Vec<Entity> = Vec::new();
    let mut star: Option<crate::thermal::StarClass> = None;
    let mut debts: Vec<AbsorptionDebt> = Vec::new();
    let mut bonds: Vec<Bond> = Vec::new();
    let mut covenants: Vec<Covenant> = Vec::new();
    let mut councils: Vec<Council> = Vec::new();
    let mut techs: Vec<(u64, u8, [u8; 32])> = Vec::new();
    let mut offers: Vec<Offer> = Vec::new();
    let mut wants: Vec<(u64, u8)> = Vec::new();
    let mut prospects: Vec<Prospected> = Vec::new();
    let mut crossed = false;
    let mut gauge = vec![ratio(0, 1)];
    let mut genesis: Option<Genesis> = None;
    let mut terrain_seed: Option<[u8; 32]> = None;
    let mut practices: Vec<u8> = Vec::new();
    let mut instruments: Vec<(u64, [u8; 32], u8)> = Vec::new();
    let mut routes: Vec<(u64, Ratio)> = Vec::new();
    let mut warded: Option<bool> = None;
    let mut veiled = false;
    let mut taxon: Option<(u8, u8)> = None;
    let mut stations: Vec<(String, u8)> = Vec::new();
    for receipt in chronicle.receipts() {
        match &receipt.event {
            ClosureEvent::EntityRegistered { entity_id, principal, handle } => {
                entities.push(Entity {
                    entity_id: *entity_id,
                    handle: handle.clone(),
                    principal: match principal {
                        0 => None,
                        held => Some(*held),
                    },
                    grants: Vec::new(),
                });
            }
            ClosureEvent::ScopeGranted { agent, scope } => {
                if let Some(held) = entities.iter_mut().find(|held| held.entity_id == *agent) {
                    held.grants.push(*scope);
                    held.grants.sort();
                }
            }
            ClosureEvent::ScopeRevoked { agent, scope } => {
                if let Some(held) = entities.iter_mut().find(|held| held.entity_id == *agent) {
                    held.grants.retain(|granted| granted != scope);
                }
            }
            ClosureEvent::ColonyFounded { orb_id, name } => {
                let mut founded = ColonyOrb {
                    orb_id: *orb_id,
                    name: name.clone(),
                    ..ColonyOrb::default()
                };
                for (resource, amount) in founding_reserves() {
                    founded.bank(resource, amount);
                }
                orbs.push(founded);
            }
            ClosureEvent::ColonyCharted { orb_id, source_orb, name } => {
                if let Some(source) = peel(&mut orbs, *source_orb) {
                    for (resource, amount) in founding_reserves() {
                        source.bank(resource, -amount);
                    }
                }
                let mut daughter = ColonyOrb {
                    orb_id: *orb_id,
                    name: name.clone(),
                    ..ColonyOrb::default()
                };
                for (resource, amount) in founding_reserves() {
                    daughter.bank(resource, amount);
                }
                orbs.push(daughter);
            }
            ClosureEvent::ZoneBuilt { orb_id, tile, kind } => {
                if let Some(held) = peel(&mut orbs, *orb_id) {
                    for (resource, cost) in kind.cost() {
                        held.bank(resource, -cost);
                    }
                    held.zones.push(Zone { tile: *tile, kind: *kind });
                    held.zones.sort_by_key(|zone| zone.tile);
                }
            }
            ClosureEvent::YieldCollected { orb_id, resource, amount } => {
                if let Some(held) = peel(&mut orbs, *orb_id) {
                    held.bank(*resource, amount.clone());
                    held.last_touch = receipt.stamp;
                }
            }
            // U1: the discovery mint banks like a yield; the work
            // payload is judged at the door (kernel I4), not at
            // replay — the receipt carries it for re-verification.
            ClosureEvent::DiscoveryMinted { orb_id, resource, amount, .. } => {
                if let Some(held) = peel(&mut orbs, *orb_id) {
                    held.bank(*resource, amount.clone());
                    held.last_touch = receipt.stamp;
                }
            }
            ClosureEvent::UpkeepSettled { orb_id, resource, amount } => {
                if let Some(held) = peel(&mut orbs, *orb_id) {
                    held.bank(*resource, -amount.clone());
                    // TRADE-1 truth-update: every settle is a touch —
                    // the demurrage anchor replays for upkeep-only
                    // collects too, exactly as the live path set it.
                    held.last_touch = receipt.stamp;
                }
            }
            ClosureEvent::FamineStruck { orb_id, resource, drawn } => {
                if let Some(held) = peel(&mut orbs, *orb_id) {
                    held.bank(*resource, -drawn.clone());
                    held.famine.push(*resource);
                    held.famine.sort();
                }
            }
            ClosureEvent::FamineRelieved { orb_id, resource } => {
                if let Some(held) = peel(&mut orbs, *orb_id) {
                    held.famine.retain(|kept| kept != resource);
                }
            }
            ClosureEvent::ZoneRazed { orb_id, tile } => {
                if let Some(held) = peel(&mut orbs, *orb_id) {
                    if let Some(at) = held.zones.iter().position(|zone| zone.tile == *tile) {
                        let zone = held.zones.remove(at);
                        for (resource, refund) in zone.kind.raze_refund() {
                            held.bank(resource, refund);
                        }
                    }
                }
            }
            // CONVERGE-1 (07 §1): a universe-ledger event — it never
            // enters a world chronicle, and world replay ignores it.
            // D3: the terrain charter replays — the seed is forever.
            ClosureEvent::WorldCharted { seed } => terrain_seed = Some(*seed),
            // PRACTICE-1: the disciplines replay in adoption order.
            ClosureEvent::PracticeAdopted { practice } => practices.push(*practice),
            ClosureEvent::PracticeReleased { practice } => {
                if let Some(at) = practices.iter().position(|held| held == practice) {
                    practices.remove(at);
                }
            }
            // INCURSION-1a: a universe-ledger event — world replay
            // ignores it (the EraAnnexed precedent, explicit no-op).
            ClosureEvent::IncursionStruck { .. } => {}
            ClosureEvent::ElderWarded {} => warded = Some(false),
            ClosureEvent::WardLifted { .. } => warded = Some(true),
            ClosureEvent::VeilDrawn {} => veiled = true,
            ClosureEvent::VeilLifted { .. } => veiled = false,
            // KERNEL-TAXON: the seal replays — identity is forever.
            ClosureEvent::TaxonSealed { class, clade } => taxon = Some((*class, *clade)),
            // THE VERTICAL AXIS (tag 89): the seat replays from the
            // receipt, so the registry file stops being the authority
            // on who holds which office. One seat per line — re-seating
            // REPLACES, and the chain keeps the whole history of it.
            ClosureEvent::StationSeated { line, station } => {
                match stations.iter_mut().find(|(held, _)| held == line) {
                    Some(seat) => seat.1 = *station,
                    None => stations.push((line.clone(), *station)),
                }
            }
            ClosureEvent::EraAnnexed { .. } => {}
            // MESH-1b (NS-5 §7): authority is the mesh's walk of the
            // arbor — the world's committed projection is blind to it
            // (the game rules never consult the writing hand).
            ClosureEvent::HandoffSealed { .. } => {}
            // GALAXY-1 (13_GALAXY §4): a universe-ledger event — it
            // never enters a world chronicle, and world replay ignores
            // it (the EraAnnexed precedent, explicit no-op).
            ClosureEvent::GalaxyObserved { .. } => {}
            // THERMAL-1 replay arms.
            ClosureEvent::StarCommissioned { class } => {
                star = crate::thermal::StarClass::from_wire(*class);
            }
            ClosureEvent::PlanetSited { orb_id, albedo, orbit_r } => {
                if let Some(held) = peel(&mut orbs, *orb_id) {
                    held.thermal = Some(crate::dop::ThermalSite {
                        albedo: albedo.clone(),
                        orbit_r: orbit_r.clone(),
                    });
                }
            }
            ClosureEvent::OverheatStruck { orb_id, .. } => {
                if let Some(held) = peel(&mut orbs, *orb_id) {
                    held.overheat = true;
                }
            }
            ClosureEvent::OverheatCleared { orb_id } => {
                if let Some(held) = peel(&mut orbs, *orb_id) {
                    held.overheat = false;
                }
            }
            // VENT-1: the cumulative ledger replays from settlements.
            ClosureEvent::VentSettled { orb_id, amount } => {
                if let Some(held) = peel(&mut orbs, *orb_id) {
                    held.entropy_vented += amount.clone();
                    held.last_touch = receipt.stamp;
                }
            }
            // SINK-1: receipted demurrage replays as a bank draw.
            ClosureEvent::DemurrageSettled { orb_id, resource, amount } => {
                if let Some(held) = peel(&mut orbs, *orb_id) {
                    held.bank(*resource, -amount.clone());
                    held.last_touch = receipt.stamp;
                }
            }
            // TRADE-1: the move replays with the SAME crisis reads the
            // live path made — the compassion books are a replay fact.
            ClosureEvent::ResourceTransferred { from_orb, to_orb, resource, amount } => {
                let from_crisis = peek(&orbs, *from_orb)
                    .map(|held| in_crisis(held))
                    .unwrap_or(false);
                let to_crisis = peek(&orbs, *to_orb)
                    .map(|held| in_crisis(held))
                    .unwrap_or(false);
                let value = amount.clone() * crate::dop::commensurate_weight(*resource);
                if let Some(held) = peel(&mut orbs, *from_orb) {
                    held.bank(*resource, -amount.clone());
                    held.last_touch = receipt.stamp;
                    if to_crisis {
                        held.given_in_crisis += value.clone();
                    }
                }
                if let Some(held) = peel(&mut orbs, *to_orb) {
                    held.bank(*resource, amount.clone());
                    held.last_touch = receipt.stamp;
                    if from_crisis {
                        held.drawn_in_crisis += value.clone();
                    }
                }
                settle_debts(&mut debts, &mut orbs, *from_orb, *to_orb, value);
                credit_covenants(&mut covenants, *from_orb, *to_orb, *resource, amount);
                credit_councils(&mut councils, *from_orb, *to_orb, *resource, amount);
            }
            // NS-4 APERTURE: the source leg replays as a bank draw —
            // value left THIS world for another; last_touch anchors at
            // the deal stamp (the touch settled demurrage separately).
            ClosureEvent::ApertureSent { from_orb, resource, amount, .. } => {
                if let Some(held) = peel(&mut orbs, *from_orb) {
                    held.bank(*resource, -amount.clone());
                    held.last_touch = receipt.stamp;
                }
                crossed = true;
            }
            // NS-4 APERTURE: the destination leg replays as a bank
            // credit — value arrived from another world.
            ClosureEvent::ApertureReceived { to_orb, resource, amount, .. } => {
                if let Some(held) = peel(&mut orbs, *to_orb) {
                    held.bank(*resource, amount.clone());
                    held.last_touch = receipt.stamp;
                }
                crossed = true;
            }
            // N4 (NS-2 §1.5): the draw replays as a bank debit and a
            // touch anchor. The reading it paid for CANNOT replay —
            // the receipt is deliberately blind to it (the lens law).
            ClosureEvent::CrystalDrawn { orb_id, amount } => {
                if let Some(held) = peel(&mut orbs, *orb_id) {
                    held.bank(Resource::Crystal, -amount.clone());
                    held.last_touch = receipt.stamp;
                }
            }
            // NOVEL-1 (10 §3c): the succession boundary replays — the
            // living tree drops to the world's kernel; the record
            // stands in the receipts above and below this one.
            ClosureEvent::LineSucceeded { .. } => {
                techs.retain(|(_, condition, _)| *condition == 0);
                genesis = None;
            }
            // G1 (04 §1/§5): the binding copy replays as the sealed
            // line.
            ClosureEvent::GenesisSealed { civilization, disposition, metabolism, perception } => {
                genesis = Some(Genesis {
                    civilization: civilization.clone(),
                    disposition: *disposition,
                    metabolism: *metabolism,
                    perception: *perception,
                });
            }
            // ELEMENT-GRAMMAR-1: the receipt carries the drawn
            // properties — replay reads, never rolls.
            ClosureEvent::ProspectStruck { orb_id, stratum, element, richness, .. } => {
                prospects.push(Prospected {
                    orb: *orb_id,
                    stratum: *stratum,
                    element: *element,
                    richness: richness.clone(),
                    last_assay: receipt.stamp,
                });
            }
            ClosureEvent::AssayStruck { orb_id, element, amount } => {
                if let Some(vein) = prospects
                    .iter_mut()
                    .find(|held| held.orb == *orb_id && held.element == *element)
                {
                    vein.last_assay = receipt.stamp;
                }
                if let Some(held) = peel(&mut orbs, *orb_id) {
                    held.bank_element(*element, amount.clone());
                }
            }
            // ABSORB-1: the three-way move replays — the absorber's
            // ledger carries the shed; the window opens; crisis reads
            // match the live order exactly.
            ClosureEvent::OverflowAbsorbed { source_orb, absorber_orb, amount } => {
                let source_crisis = peek(&orbs, *source_orb)
                    .map(|held| in_crisis(held))
                    .unwrap_or(false);
                let absorber_crisis = peek(&orbs, *absorber_orb)
                    .map(|held| in_crisis(held))
                    .unwrap_or(false);
                let value =
                    amount.clone() * crate::dop::commensurate_weight(Resource::Sink);
                if let Some(held) = peel(&mut orbs, *absorber_orb)
                {
                    held.bank(Resource::Sink, -amount.clone());
                    held.entropy_vented += amount.clone();
                    held.last_touch = receipt.stamp;
                    if source_crisis {
                        held.given_in_crisis += value.clone();
                    }
                }
                if absorber_crisis {
                    if let Some(held) =
                        peel(&mut orbs, *source_orb)
                    {
                        held.drawn_in_crisis += value;
                    }
                }
                debts.push(AbsorptionDebt {
                    debtor: *source_orb,
                    creditor: *absorber_orb,
                    remainder: amount.clone(),
                    opened: receipt.stamp,
                });
            }
            // ABSORB-1: the lapse replays — extraction posts, the
            // oldest matching window drops (live emission order).
            ClosureEvent::AbsorptionLapsed { debtor_orb, creditor_orb, amount } => {
                if let Some(held) = peel(&mut orbs, *debtor_orb) {
                    held.extracted_across += amount.clone();
                }
                if let Some(at) = debts.iter().position(|debt| {
                    debt.debtor == *debtor_orb
                        && debt.creditor == *creditor_orb
                        && debt.remainder == *amount
                }) {
                    debts.remove(at);
                }
            }
            // BOND-1: the standing order replays whole.
            ClosureEvent::BondSealed { source_orb, target_orb, resource, rate, term } => {
                bonds.push(Bond {
                    source: *source_orb,
                    target: *target_orb,
                    resource: *resource,
                    rate: rate.clone(),
                    term: *term,
                    sealed: receipt.stamp,
                    anchored: receipt.stamp,
                });
            }
            // BOND-1: one settle receipt moves the span, posts the
            // love book, compensates open windows, and — severed —
            // ends the bond with its load. Non-ending settles re-
            // anchor at the stamp (live boundary == stamp, proven at
            // the landing).
            ClosureEvent::BondSettled { source_orb, target_orb, resource, amount, severed, load } => {
                let value = amount.clone() * crate::dop::commensurate_weight(*resource);
                if *amount > ratio(0, 1) {
                    if let Some(held) = peel(&mut orbs, *source_orb)
                    {
                        held.bank(*resource, -amount.clone());
                        held.last_touch = receipt.stamp;
                        held.bond_flow += value.clone();
                    }
                    if let Some(held) = peel(&mut orbs, *target_orb)
                    {
                        held.bank(*resource, amount.clone());
                        held.last_touch = receipt.stamp;
                    }
                    settle_debts(&mut debts, &mut orbs, *source_orb, *target_orb, value);
                    credit_covenants(&mut covenants, *source_orb, *target_orb, *resource, amount);
                    credit_councils(&mut councils, *source_orb, *target_orb, *resource, amount);
                }
                match severed {
                    true => {
                        if let Some(at) = bonds.iter().position(|bond| {
                            bond.source == *source_orb
                                && bond.target == *target_orb
                                && bond.resource == *resource
                        }) {
                            bonds.remove(at);
                        }
                        if *load > ratio(0, 1) {
                            if let Some(held) =
                                peel(&mut orbs, *source_orb)
                            {
                                held.severance_load += load.clone();
                            }
                        }
                    }
                    false => {
                        if let Some(bond) = bonds.iter_mut().find(|bond| {
                            bond.source == *source_orb
                                && bond.target == *target_orb
                                && bond.resource == *resource
                        }) {
                            bond.anchored = receipt.stamp;
                        }
                    }
                }
            }
            // COVENANT-1: the pact replays whole — the seal receipt
            // carries the priced collateral (the curve never re-runs).
            ClosureEvent::CovenantSealed {
                obligor_orb,
                beneficiary_orb,
                resource,
                quota,
                window,
                collateral,
            } => {
                if let Some(held) = peel(&mut orbs, *obligor_orb) {
                    held.bank(*resource, -collateral.clone());
                }
                covenants.push(Covenant {
                    obligor: *obligor_orb,
                    beneficiary: *beneficiary_orb,
                    resource: *resource,
                    quota: quota.clone(),
                    delivered: ratio(0, 1),
                    collateral: collateral.clone(),
                    sealed: receipt.stamp,
                    window: *window,
                });
            }
            ClosureEvent::CovenantReleased {
                obligor_orb, beneficiary_orb, resource, collateral,
            } => {
                let stake = collateral.clone() * crate::dop::commensurate_weight(*resource);
                if let Some(held) = peel(&mut orbs, *obligor_orb) {
                    held.bank(*resource, collateral.clone());
                    held.covenants_fulfilled += stake;
                }
                if let Some(at) = covenants.iter().position(|pact| {
                    pact.obligor == *obligor_orb
                        && pact.beneficiary == *beneficiary_orb
                        && pact.resource == *resource
                }) {
                    covenants.remove(at);
                }
            }
            ClosureEvent::CovenantSlashed {
                obligor_orb, beneficiary_orb, resource, collateral,
            } => {
                let stake = collateral.clone() * crate::dop::commensurate_weight(*resource);
                if let Some(held) =
                    peel(&mut orbs, *beneficiary_orb)
                {
                    held.bank(*resource, collateral.clone());
                }
                if let Some(held) = peel(&mut orbs, *obligor_orb) {
                    held.covenants_breached += stake;
                }
                if let Some(at) = covenants.iter().position(|pact| {
                    pact.obligor == *obligor_orb
                        && pact.beneficiary == *beneficiary_orb
                        && pact.resource == *resource
                }) {
                    covenants.remove(at);
                }
            }
            // IDEO-1: sealed friction replays into the vented ledger.
            ClosureEvent::FrictionSettled { orb_id, amount } => {
                if let Some(held) = peel(&mut orbs, *orb_id) {
                    held.entropy_vented += amount.clone();
                    held.last_touch = receipt.stamp;
                }
            }
            // TECH-1a: a discovered node replays into the tech list —
            // the solar leap (condition 0) among them.
            ClosureEvent::TechDiscovered { orb_id, node, condition, .. } => {
                techs.push((*orb_id, *condition, *node));
            }
            // COUNCIL-1: the seal replays whole — every member's lock
            // withdraws; deliveries re-credit through the transfer and
            // bond arms above.
            ClosureEvent::CouncilSealed {
                council, resource, quota, window, collateral, members,
            } => {
                for member in members {
                    if let Some(held) = peel(&mut orbs, *member) {
                        held.bank(*resource, -collateral.clone());
                    }
                }
                councils.push(Council {
                    council: *council,
                    members: members.clone(),
                    resource: *resource,
                    quota: quota.clone(),
                    delivered: vec![ratio(0, 1); members.len()],
                    collateral: collateral.clone(),
                    sealed: receipt.stamp,
                    window: *window,
                });
            }
            // COUNCIL-1: each member's reckoning banks exactly what
            // the receipt carries; the integrity stake reads from the
            // sealed council; the council leaves state when its last
            // member has closed.
            ClosureEvent::CouncilClosed { council, member, fulfilled, banked } => {
                if let Some(at) = councils.iter().position(|pact| pact.council == *council) {
                    let stake = councils[at].collateral.clone()
                        * crate::dop::commensurate_weight(councils[at].resource);
                    let resource = councils[at].resource;
                    if let Some(held) = peel(&mut orbs, *member) {
                        if *banked > ratio(0, 1) {
                            held.bank(resource, banked.clone());
                        }
                        match fulfilled {
                            true => held.covenants_fulfilled += stake,
                            false => held.covenants_breached += stake,
                        }
                    }
                    if let Some(seat) =
                        councils[at].members.iter().position(|held| held == member)
                    {
                        councils[at].members.remove(seat);
                        councils[at].delivered.remove(seat);
                    }
                    if councils[at].members.is_empty() {
                        councils.remove(at);
                    }
                }
            }
            // PLANNER-1: the issuing deed is a pure record src-side —
            // the far world's TechDiscovered leg carries the arrival.
            ClosureEvent::TechLicensed { .. } => {}
            // AEON-1a: every key move replays verbatim — the receipt
            // carries the wrapped value, the last one stands.
            ClosureEvent::GaugeShifted { gauge: key }
            | ClosureEvent::Diverged { gauge: key, .. }
            | ClosureEvent::Confluent { gauge: key, .. }
            | ClosureEvent::EraFolded { gauge: key, .. } => {
                gauge[0] = key.clone();
            }
            // WANT-1: the famine shape — strike lists, relief clears.
            ClosureEvent::WantStruck { orb_id, kind, .. } => {
                wants.push((*orb_id, *kind));
            }
            ClosureEvent::WantRelieved { orb_id, kind } => {
                wants.retain(|(o, k)| !(o == orb_id && k == kind));
            }
            // MARKET-1: the intent rests; the clear removes it.
            ClosureEvent::OfferPosted {
                offer, orb_id, give, give_amount, want, want_amount,
            } => {
                offers.push(Offer {
                    offer: *offer,
                    orb: *orb_id,
                    give: *give,
                    give_amount: give_amount.clone(),
                    want: *want,
                    want_amount: want_amount.clone(),
                    posted: receipt.stamp,
                });
            }
            ClosureEvent::OfferCleared { offer, .. } => {
                offers.retain(|held| held.offer != *offer);
            }
            // GAUGE-N: a graded shift replays verbatim, growing the
            // key when it opens a plane (zeros fill — ground rest).
            ClosureEvent::GaugeShiftedAt { grade, gauge: key } => {
                let at = *grade as usize;
                if gauge.len() <= at {
                    gauge.resize(at + 1, ratio(0, 1));
                }
                gauge[at] = key.clone();
            }
            // AEON-1a: the star moved in its region.
            ClosureEvent::StarAged { class } => {
                star = crate::thermal::StarClass::from_wire(*class);
            }
            // MANU-1: an instrument joined the held list.
            ClosureEvent::InstrumentForged { orb_id, tech_hash, grade } => {
                instruments.push((*orb_id, *tech_hash, *grade));
            }
            // MARKET-2 conform: the void's share moves no bank — the
            // receipt IS the fact; what landed was already credited.
            ClosureEvent::VoidShare { .. } => {}
            // ROUTE-1: the lane lives and dies by receipt.
            ClosureEvent::RouteChartered { deal, eta } => {
                routes.push((*deal, eta.clone()));
            }
            ClosureEvent::RouteLapsed { deal } => {
                routes.retain(|(held, _)| held != deal);
            }
        }
    }
    Committed {
        orbs, entities, star, debts, bonds, covenants, councils, offers, techs, wants, crossed,
        gauge, genesis, terrain_seed, practices, instruments, routes, prospects, warded,
        veiled, taxon, stations,
    }
}

