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


/// **THE STEPPER** (task #9, operator order 2026-08-12 — the
/// structural cure). `replay` was one monolithic loop that could not
/// advance by a single receipt, so every caller needing per-receipt
/// states paid a full prefix replay per receipt — O(history) each,
/// O(history²) per pass; the conductor's per-deed judgment and its
/// boot both wedged on exactly that (measured: 402s of a 430s beat).
/// The Replayer holds the committed state and applies ONE receipt at
/// a time; a checkpoint is a clone of state — O(state), never
/// O(history). `replay` is now the fold over this stepper, so the
/// transition keeps ONE body and every SEAL B pin in the workspace
/// witnesses the equivalence.
#[derive(Clone)]
pub struct Replayer {
    state: Committed,
}

impl Default for Replayer {
    fn default() -> Self {
        Self::new()
    }
}

impl Replayer {
    /// The empty book — the state before the first receipt.
    pub fn new() -> Replayer {
        Replayer {
            state: Committed {
                orbs: Vec::new(),
                entities: Vec::new(),
                star: None,
                debts: Vec::new(),
                bonds: Vec::new(),
                forwards: Vec::new(),
                councils: Vec::new(),
                techs: Vec::new(),
                offers: Vec::new(),
                wants: Vec::new(),
                element_wants: Vec::new(),
                prospects: Vec::new(),
                crossed: false,
                gauge: vec![ratio(0, 1)],
                genesis: None,
                terrain_seed: None,
                practices: Vec::new(),
                instruments: Vec::new(),
                routes: Vec::new(),
                element_routes: Vec::new(),
                treaties: Vec::new(),
                element_allowlists: Vec::new(),
                warded: None,
                veiled: false,
                taxon: None,
                stations: Vec::new(),
                hulls: Vec::new(),
                notes: Vec::new(),
                futures: Vec::new(),
                last_clears: Vec::new(),
                shares: Vec::new(),
                perps: Vec::new(),
                clear_depths: Vec::new(),
                market_sealed_until: None,
                cascade_pressures: Vec::new(),
                tamper_pressures: Vec::new(),
                posture_disposition: None,
                posture_perception: None,
                posture_primary: None,
                intel_hashes: Vec::new(),
                spy_marks: Vec::new(),
                vault_stamp: Vec::new(),
                chem_catalog: Vec::new(),
                science_seats: Vec::new(),
                clone_counts: Vec::new(),
                synth_mods: Vec::new(),
                skin_mods: Vec::new(),
                rogue_pressure: Vec::new(),
            },
        }
    }

    /// The state after every receipt applied so far.
    pub fn state(&self) -> &Committed {
        &self.state
    }

    pub fn into_committed(self) -> Committed {
        self.state
    }

    /// Apply ONE receipt — the transition's single body.
    pub fn apply(&mut self, receipt: &crate::vrp::Receipt) {
        let Committed {
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
            practices,
            instruments,
            routes,
            element_routes,
            treaties,
            prospects,
            warded,
            veiled,
            taxon,
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
            synth_mods,
            skin_mods,
            rogue_pressure,
            element_allowlists,
        } = &mut self.state;
    match &receipt.event {
        // BOOK-1 (19_LENS §16) — the open registry rebuilds from
        // receipts alone, like every other committed ledger here.
        // LIT-1 — the literature rebuilds from receipts alone.
        ClosureEvent::WorkAspired { orb_id, work, name } => {
            if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *orb_id) {
                held.aspirations.push(crate::literature::Aspiration {
                    work: *work,
                    name: name.clone(),
                    witnesses: Vec::new(),
                });
                held.aspirations.sort_by_key(|held| held.work);
            }
        }
        ClosureEvent::WorkClaimed { orb_id, work, witnesses } => {
            if let Some(held) = orbs
                .iter_mut()
                .find(|held| held.orb_id == *orb_id)
                .and_then(|held| held.aspirations.iter_mut().find(|held| held.work == *work))
            {
                held.witnesses = witnesses.clone();
            }
        }
        ClosureEvent::BookMinted { orb_id, kind, name } => {
            if let Some(held) = orbs.iter_mut().find(|held| held.orb_id == *orb_id) {
                held.books.push(crate::books::BookAccount {
                    kind: *kind,
                    name: name.clone(),
                    minted_by: *orb_id,
                    kept: ratio(0, 1),
                    lost: ratio(0, 1),
                    archived: false,
                });
                held.books.sort_by_key(|held| held.kind);
            }
        }
        ClosureEvent::BookPosted { orb_id, kind, kept, lost } => {
            if let Some(account) = orbs
                .iter_mut()
                .find(|held| held.orb_id == *orb_id)
                .and_then(|held| held.books.iter_mut().find(|held| held.kind == *kind))
            {
                account.kept += kept.clone();
                account.lost += lost.clone();
            }
        }
        ClosureEvent::BookArchived { orb_id, kind } => {
            if let Some(account) = orbs
                .iter_mut()
                .find(|held| held.orb_id == *orb_id)
                .and_then(|held| held.books.iter_mut().find(|held| held.kind == *kind))
            {
                // Closed to WRITING. Every entry it took stays.
                account.archived = true;
            }
        }
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
            if let Some(source) = peel(&mut *orbs, *source_orb) {
                for (resource, amount) in founding_reserves() {
                    source.bank(resource, -amount);
                }
            }
            let mut charted = ColonyOrb {
                orb_id: *orb_id,
                name: name.clone(),
                ..ColonyOrb::default()
            };
            for (resource, amount) in founding_reserves() {
                charted.bank(resource, amount);
            }
            orbs.push(charted);
        }
        ClosureEvent::ZoneBuilt { orb_id, tile, kind, base } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                // #41: the build was PRICED on its base — replay
                // reads the receipt's own declaration rather than
                // re-resolving the fold, so a minted kind replays
                // at exactly what it was raised for.
                for (resource, cost) in base.unwrap_or(*kind).cost() {
                    held.bank(resource, -cost);
                }
                held.zones.push(Zone { tile: *tile, kind: *kind, base: *base });
                held.zones.sort_by_key(|zone| zone.tile);
            }
        }
        ClosureEvent::YieldCollected { orb_id, resource, amount } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(*resource, amount.clone());
                // THE SYNTROPIC BOOK — the inverse of entropy_vented.
                // A joule yield IS an encased joule (the crystal
                // sheet already reads it that way), so containing
                // one books it, whether the excess came from a star
                // or from friction. Replay must mirror the live path
                // EXACTLY or the two states diverge on a field.
                if *resource == Resource::Joule {
                    held.syntropy_contained += amount.clone();
                }
                held.last_touch = receipt.stamp;
            }
        }
        // U1: the discovery mint banks like a yield; the work
        // payload is gated at the door (kernel I4), not at
        // replay — the receipt carries it for re-verification.
        ClosureEvent::DiscoveryMinted { orb_id, resource, amount, .. } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(*resource, amount.clone());
                held.last_touch = receipt.stamp;
            }
        }
        ClosureEvent::UpkeepSettled { orb_id, resource, amount } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(*resource, -amount.clone());
                // TRADE-1 truth-update: every settle is a touch —
                // the demurrage anchor replays for upkeep-only
                // collects too, exactly as the live path set it.
                held.last_touch = receipt.stamp;
            }
        }
        // THE ESTATE SURVEY (tag 97): the meter's draw replays
        // exactly — the resource comes from the SAME sealed rule the
        // live path drew from (call-not-copy), and every settle is a
        // touch. The parcels are derived, so nothing else moves.
        ClosureEvent::EstateSurveyed { payer, cost, .. } => {
            if let Some(held) = peel(&mut *orbs, *payer) {
                let (resource, _) = crate::dop::survey_meter();
                held.bank(resource, -cost.clone());
                held.last_touch = receipt.stamp;
            }
        }
        // THE APPRAISAL (tag 98): a valuation reading, receipted.
        // Moves no bank, sites nothing — the receipt IS the state.
        ClosureEvent::AppraisalStruck { .. } => {}
        // THE COUNTER (tag 99): the resting offer's ask amends —
        // the SAME mutation the live path made, so fills read one
        // truth on both sides of a restart.
        ClosureEvent::OfferCountered { offer, want_amount, .. } => {
            if let Some(held) = offers.iter_mut().find(|held| held.offer == *offer) {
                held.want_amount = want_amount.clone();
            }
        }
        // CONVEY (tag 100): the postal move replays EXACTLY as the
        // live path made it — sender pays amount + postage (the
        // stamp from the SAME sealed rule), receiver banks the
        // amount whole, both touched.
        ClosureEvent::ParcelConveyed { from_orb, to_orb, resource, amount, postage, .. } => {
            let (stamp, _) = crate::dop::postal_meter();
            if let Some(held) = peel(&mut *orbs, *from_orb) {
                held.bank(*resource, -amount.clone());
                held.bank(stamp, -postage.clone());
                held.last_touch = receipt.stamp;
            }
            if let Some(dest) = peel(&mut *orbs, *to_orb) {
                dest.bank(*resource, amount.clone());
                dest.last_touch = receipt.stamp;
            }
        }
        // THE MINTED LINE (tag 101): identity, never matter — the
        // line derives from the receipt's seed wherever it is read;
        // nothing in committed state moves.
        ClosureEvent::LineMinted { .. } => {}
        ClosureEvent::FamineStruck { orb_id, resource, drawn } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(*resource, -drawn.clone());
                held.famine.push(*resource);
                held.famine.sort();
            }
        }
        ClosureEvent::FamineRelieved { orb_id, resource } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.famine.retain(|kept| kept != resource);
            }
        }
        ClosureEvent::ZoneRazed { orb_id, tile } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                if let Some(at) = held.zones.iter().position(|zone| zone.tile == *tile) {
                    let zone = held.zones.remove(at);
                    // #41: the base it was raised on prices the
                    // salvage — see `Zone::priced_as`.
                    for (resource, refund) in zone.priced_as().raze_refund() {
                        held.bank(resource, refund);
                    }
                }
            }
        }
        // CONVERGE-1 (07 §1): a universe-ledger event — it never
        // enters a world chronicle, and world replay ignores it.
        // D3: the terrain charter replays — the seed is forever.
        ClosureEvent::WorldCharted { seed } => *terrain_seed = Some(*seed),
        // PRACTICE-1: the disciplines replay in adoption order.
        ClosureEvent::PracticeAdopted { practice } => practices.push(*practice),
        ClosureEvent::PracticeReleased { practice } => {
            if let Some(at) = practices.iter().position(|held| held == practice) {
                practices.remove(at);
            }
        }
        // INCURSION-1a: a universe-ledger event — world replay
        // ignores it (the EraAnnexed precedent, explicit no-op).
        // #42: a pact enters this book. Sealing a kind that
        // DISPLACES another strikes that other down first — peace
        // ends a war, war breaks a peace — so the register can
        // never hold both at once and replay needs no second rule
        // to say so.
        ClosureEvent::TreatySealed { deal, kind, partner, terms, term } => {
            if let Some(kind) = crate::treaty::TreatyKind::from_wire(*kind) {
                if let Some(displaced) = kind.displaces() {
                    treaties.retain(|held| {
                        !(held.partner == *partner && held.kind == displaced)
                    });
                }
                treaties.push(crate::treaty::Treaty {
                    deal: *deal,
                    kind,
                    partner: partner.clone(),
                    terms: terms.clone(),
                    sealed: receipt.stamp,
                    term: *term,
                });
            }
        }
        // #42: the strike carries WHETHER it was a breach, so the
        // load posts HERE and not at the seal that caused it.
        //
        // Found by the replay pin: the verb strikes the displaced
        // pact in its OWN receipt, BEFORE sealing the new one, so
        // by the time replay saw the seal the peace was already
        // gone and the breach was invisible. Live charged it,
        // replay did not — a SEAL B split. The receipt already
        // knew: `broken` is the flag, and reading it here makes
        // the two definitions agree by construction.
        ClosureEvent::TreatyStruck { deal, partner, broken } => {
            treaties.retain(|held| !(held.deal == *deal && held.partner == *partner));
            // E3.9: the annex dies with the pact it rides — the
            // verb prunes at renounce AND at displacement, and
            // both paths receipt this strike, so one arm agrees
            // with both.
            element_allowlists
                .retain(|(held, other, _)| !(*held == *deal && other == partner));
            if *broken {
                let load = crate::dop::commensurate_weight(Resource::Iron);
                if let Some(orb) = orbs.first_mut() {
                    orb.severance_load += load;
                }
            }
        }
        // E3.9: an element allow-list annexes its pact. The verb
        // proved the trade treaty stood; replay reads, never
        // re-judges (liveness is a reading over `treaties`).
        ClosureEvent::TreatyElementsAllowed { deal, partner, elements } => {
            element_allowlists.push((*deal, partner.clone(), elements.clone()));
        }
        ClosureEvent::IncursionStruck { .. } => {}
        ClosureEvent::ElderWarded {} => *warded = Some(false),
        ClosureEvent::WardLifted { .. } => *warded = Some(true),
        ClosureEvent::VeilDrawn {} => *veiled = true,
        ClosureEvent::VeilLifted { .. } => *veiled = false,
        // KERNEL-TAXON: the seal replays — identity is forever.
        ClosureEvent::TaxonSealed { class, clade } => *taxon = Some((*class, *clade)),
        // 28_VOCABULARY V2: the mint carries no per-world state —
        // the vocabulary is READ from the chronicle by
        // `vocabulary::fold`, never mirrored into `Committed`. A
        // second copy in the committed state would be the two-truths
        // defect the definition law struck, one layer down.
        ClosureEvent::VocabularyMinted { .. } => {}
        // P2 (tag 103): the chartered value is FOLD-derived
        // (`parameter::of`) — the receipt is the state.
        ClosureEvent::ParameterChartered { .. } => {}
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
            *star = crate::thermal::StarClass::from_wire(*class);
            // STAR-ENDOW-1: top-up genesis kit (pure of class).
            if let (Some(c), Some(held)) = (*star, peel(&mut *orbs, FIRST_ORB)) {
                crate::dop::apply_endowment_top_up(held, &crate::dop::star_endowment(c));
            }
        }
        ClosureEvent::PlanetSited { orb_id, albedo, orbit_r, theta, psi } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.thermal = Some(crate::dop::ThermalSite {
                    albedo: albedo.clone(),
                    orbit_r: orbit_r.clone(),
                    // BOTH TURNS FROM THE RECEIPT — never None.
                    // Binding one and discarding it loses an orb's
                    // PLACE on replay while the live path keeps it,
                    // which is the live/replay divergence class.
                    // Clippy caught this twice here, once per turn.
                    theta: theta.clone(),
                    psi: psi.clone(),
                });
                // ORBIT-ENDOW-1: band gift (pure of star + site).
                if let Some(class) = star {
                    let teq = crate::thermal::t_eq4(
                        &class.luminosity(),
                        albedo.value(),
                        orbit_r.value(),
                    );
                    let band = crate::thermal::band_of(&teq);
                    crate::dop::apply_endowment_add(
                        held,
                        &crate::dop::orbit_endowment(band),
                    );
                }
            }
        }
        ClosureEvent::OverheatStruck { orb_id, .. } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.overheat = true;
            }
        }
        ClosureEvent::OverheatCleared { orb_id } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.overheat = false;
            }
        }
        // VENT-1: the cumulative ledger replays from settlements.
        ClosureEvent::VentSettled { orb_id, amount } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.entropy_vented += amount.clone();
                held.last_touch = receipt.stamp;
            }
        }
        // SINK-1: receipted demurrage replays as a bank draw.
        // (Sealed per-row form — old chronicles hold it forever.)
        ClosureEvent::DemurrageSettled { orb_id, resource, amount } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(*resource, -amount.clone());
                held.last_touch = receipt.stamp;
            }
        }
        // SINK-1: the ATOMIC touch (tag 102) — every settled
        // resource applies together and the anchor moves ONCE.
        ClosureEvent::DemurrageTouched { orb_id, decays } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                for (resource, amount) in decays {
                    held.bank(*resource, -amount.clone());
                }
                held.last_touch = receipt.stamp;
            }
        }
        // TRADE-1: the move replays with the SAME crisis reads the
        // live path made — the compassion books are a replay fact.
        ClosureEvent::ResourceTransferred { from_orb, to_orb, resource, amount } => {
            let from_crisis = peek(&orbs, *from_orb)
                .map(in_crisis)
                .unwrap_or(false);
            let to_crisis = peek(&orbs, *to_orb)
                .map(in_crisis)
                .unwrap_or(false);
            let value = amount.clone() * crate::dop::commensurate_weight(*resource);
            if let Some(held) = peel(&mut *orbs, *from_orb) {
                held.bank(*resource, -amount.clone());
                held.last_touch = receipt.stamp;
                if to_crisis {
                    held.given_in_crisis += value.clone();
                }
            }
            if let Some(held) = peel(&mut *orbs, *to_orb) {
                held.bank(*resource, amount.clone());
                held.last_touch = receipt.stamp;
                if from_crisis {
                    held.drawn_in_crisis += value.clone();
                }
            }
            settle_debts(&mut *debts, &mut *orbs, *from_orb, *to_orb, value);
            credit_forwards(&mut *forwards, *from_orb, *to_orb, *resource, amount);
            credit_councils(&mut *councils, *from_orb, *to_orb, *resource, amount);
        }
        // NS-4 APERTURE: the source leg replays as a bank draw —
        // value left THIS world for another; last_touch anchors at
        // the deal stamp (the touch settled demurrage separately).
        ClosureEvent::ApertureSent { from_orb, resource, amount, .. } => {
            if let Some(held) = peel(&mut *orbs, *from_orb) {
                held.bank(*resource, -amount.clone());
                held.last_touch = receipt.stamp;
            }
            *crossed = true;
        }
        // NS-4 APERTURE: the destination leg replays as a bank
        // credit — value arrived from another world.
        ClosureEvent::ApertureReceived { to_orb, resource, amount, .. } => {
            if let Some(held) = peel(&mut *orbs, *to_orb) {
                held.bank(*resource, amount.clone());
                held.last_touch = receipt.stamp;
            }
            *crossed = true;
        }
        // E-APT-1: element legs — same doors, open-matter ledger.
        ClosureEvent::ElementApertureSent {
            from_orb,
            element,
            amount,
            ..
        } => {
            if let Some(held) = peel(&mut *orbs, *from_orb) {
                held.bank_element(*element, -amount.clone());
            }
            *crossed = true;
        }
        ClosureEvent::ElementApertureReceived {
            to_orb,
            element,
            amount,
            ..
        } => {
            if let Some(held) = peel(&mut *orbs, *to_orb) {
                held.bank_element(*element, amount.clone());
            }
            *crossed = true;
        }
        // N4 (NS-2 §1.5): the draw replays as a bank debit and a
        // touch anchor. The reading it paid for CANNOT replay —
        // the receipt is deliberately blind to it (the lens law).
        ClosureEvent::CrystalDrawn { orb_id, amount } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(Resource::Crystal, -amount.clone());
                held.last_touch = receipt.stamp;
            }
        }
        // NOVEL-1 (10 §3c): the succession boundary replays — the
        // living tree drops to the world's kernel; the record
        // stands in the receipts above and below this one.
        ClosureEvent::LineSucceeded { .. } => {
            techs.retain(|(_, condition, _, _)| *condition == 0);
            *genesis = None;
        }
        // G1 (04 §1/§5): the binding copy replays as the sealed
        // line.
        ClosureEvent::GenesisSealed { civilization, disposition, metabolism, perception } => {
            *genesis = Some(Genesis {
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
        // ELEMENT-CHEMISTRY (#37): the bond replays by re-deriving
        // its own recipe from the members the receipt names — the
        // same one home the verb settled and the gate judged. A
        // receipt whose members no longer bond spends nothing,
        // which cannot arise past the gate.
        ClosureEvent::GemCompounded { orb_id, members, gem } => {
            if let Some(set) = crate::almanac::Compoundable::new(members) {
                let (spends, yielded) = crate::almanac::recipe_of(&set);
                if let Some(held) = peel(&mut *orbs, *orb_id) {
                    for (element, spend) in &spends {
                        held.bank_element(*element, -spend.clone());
                    }
                    held.bank_element(*gem, yielded);
                }
            }
        }
        ClosureEvent::ElementRefined {
            orb_id,
            raw,
            refined,
            spent,
            yielded,
        } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank_element(*raw, -spent.clone());
                held.bank_element(*refined, yielded.clone());
            }
        }
        ClosureEvent::AssayStruck { orb_id, element, amount, richness } => {
            if let Some(vein) = prospects
                .iter_mut()
                .find(|held| held.orb == *orb_id && held.element == *element)
            {
                vein.last_assay = receipt.stamp;
                // ECO-1 (#40): the receipt carries the vein's flow
                // after the deed — replay reads, never re-derives.
                // The sealed 3-field form carries none: richness
                // holds still, exactly as the pre-ecology law did.
                if let Some(after) = richness {
                    vein.richness = after.clone();
                }
            }
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank_element(*element, amount.clone());
            }
        }
        // ABSORB-1: the three-way move replays — the absorber's
        // ledger carries the shed; the window opens; crisis reads
        // match the live order exactly.
        ClosureEvent::OverflowAbsorbed { source_orb, absorber_orb, amount } => {
            let source_crisis = peek(&orbs, *source_orb)
                .map(in_crisis)
                .unwrap_or(false);
            let absorber_crisis = peek(&orbs, *absorber_orb)
                .map(in_crisis)
                .unwrap_or(false);
            let value =
                amount.clone() * crate::dop::commensurate_weight(Resource::Sink);
            if let Some(held) = peel(&mut *orbs, *absorber_orb)
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
                    peel(&mut *orbs, *source_orb)
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
            if let Some(held) = peel(&mut *orbs, *debtor_orb) {
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
                if let Some(held) = peel(&mut *orbs, *source_orb)
                {
                    held.bank(*resource, -amount.clone());
                    held.last_touch = receipt.stamp;
                    held.bond_flow += value.clone();
                }
                if let Some(held) = peel(&mut *orbs, *target_orb)
                {
                    held.bank(*resource, amount.clone());
                    held.last_touch = receipt.stamp;
                }
                settle_debts(&mut *debts, &mut *orbs, *source_orb, *target_orb, value);
                credit_forwards(&mut *forwards, *source_orb, *target_orb, *resource, amount);
                credit_councils(&mut *councils, *source_orb, *target_orb, *resource, amount);
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
                            peel(&mut *orbs, *source_orb)
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
        // FORWARD-1: the pact replays whole — the seal receipt
        // carries the priced collateral (the curve never re-runs).
        ClosureEvent::ForwardSealed {
            obligor_orb,
            beneficiary_orb,
            resource,
            quota,
            window,
            collateral,
        } => {
            if let Some(held) = peel(&mut *orbs, *obligor_orb) {
                held.bank(*resource, -collateral.clone());
            }
            forwards.push(Forward {
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
        ClosureEvent::ForwardReleased {
            obligor_orb, beneficiary_orb, resource, collateral,
        } => {
            let stake = collateral.clone() * crate::dop::commensurate_weight(*resource);
            if let Some(held) = peel(&mut *orbs, *obligor_orb) {
                held.bank(*resource, collateral.clone());
                held.forwards_fulfilled += stake;
            }
            if let Some(at) = forwards.iter().position(|pact| {
                pact.obligor == *obligor_orb
                    && pact.beneficiary == *beneficiary_orb
                    && pact.resource == *resource
            }) {
                forwards.remove(at);
            }
        }
        ClosureEvent::ForwardSlashed {
            obligor_orb, beneficiary_orb, resource, collateral,
        } => {
            let stake = collateral.clone() * crate::dop::commensurate_weight(*resource);
            if let Some(held) =
                peel(&mut *orbs, *beneficiary_orb)
            {
                held.bank(*resource, collateral.clone());
            }
            if let Some(held) = peel(&mut *orbs, *obligor_orb) {
                held.forwards_breached += stake;
            }
            if let Some(at) = forwards.iter().position(|pact| {
                pact.obligor == *obligor_orb
                    && pact.beneficiary == *beneficiary_orb
                    && pact.resource == *resource
            }) {
                forwards.remove(at);
            }
        }
        // IDEO-1: sealed friction replays into the vented ledger.
        ClosureEvent::FrictionSettled { orb_id, amount } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.entropy_vented += amount.clone();
                held.last_touch = receipt.stamp;
            }
        }
        // TECH-1a: a discovered node replays into the tech list —
        // the solar leap (condition 0) among them. The RECEIPT's
        // magnitude rides along: it is the number the tick obeys
        // (receipt authority, operator ruling 2026-08-05).
        ClosureEvent::TechDiscovered { orb_id, node, condition, magnitude, .. } => {
            techs.push((*orb_id, *condition, *node, magnitude.clone()));
        }
        // COUNCIL-1: the seal replays whole — every member's lock
        // withdraws; deliveries re-credit through the transfer and
        // bond arms above.
        ClosureEvent::CouncilSealed {
            council, resource, quota, window, collateral, members,
        } => {
            for member in members {
                if let Some(held) = peel(&mut *orbs, *member) {
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
                if let Some(held) = peel(&mut *orbs, *member) {
                    if *banked > ratio(0, 1) {
                        held.bank(resource, banked.clone());
                    }
                    match fulfilled {
                        true => held.forwards_fulfilled += stake,
                        false => held.forwards_breached += stake,
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
        ClosureEvent::ElementWantStruck { orb_id, element, .. } => {
            if !element_wants
                .iter()
                .any(|(o, e)| *o == *orb_id && e == element)
            {
                element_wants.push((*orb_id, *element));
            }
        }
        ClosureEvent::ElementWantRelieved { orb_id, element } => {
            element_wants.retain(|(o, e)| !(*o == *orb_id && e == element));
        }
        // MARKET-1 / E-MKT-1: the intent rests; the clear removes it.
        ClosureEvent::OfferPosted {
            offer, orb_id, give, give_amount, want, want_amount,
        } => {
            offers.push(Offer {
                offer: *offer,
                orb: *orb_id,
                give: give.clone(),
                give_amount: give_amount.clone(),
                want: want.clone(),
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
            *star = crate::thermal::StarClass::from_wire(*class);
        }
        // MANU-1: an instrument joined the held list.
        ClosureEvent::InstrumentForged { orb_id, tech_hash, grade } => {
            instruments.push((*orb_id, *tech_hash, *grade));
        }
        // MARKET-2 conform: the void's share moves no bank — the
        // receipt IS the fact; what landed was already credited.
        ClosureEvent::VoidShare { .. } => {}
        // D5 — THE SOVEREIGN UNION. A CONSTITUTIONAL deed: it
        // records that the offices stand in union and moves no
        // value, no bank, no zone and no key. Replay therefore has
        // nothing to rebuild from it — the receipt IS the state.
        // The review horizon is READ from the chronicle on demand
        // (`World::union_review`), never accumulated here, so
        // there is no counter to drift.
        ClosureEvent::SovereignUnionSealed { .. } => {}
        // ROUTE-1: the lane lives and dies by receipt.
        ClosureEvent::RouteChartered { deal, eta } => {
            routes.push((*deal, eta.clone()));
        }
        ClosureEvent::RouteLapsed { deal } => {
            routes.retain(|(held, _)| held != deal);
            element_routes.retain(|(held, _, _, _)| held != deal);
        }
        ClosureEvent::ElementRouteChartered {
            deal,
            element,
            eta,
            rail,
        } => {
            element_routes.push((*deal, eta.clone(), *element, *rail));
            // Also a ROUTE-1 leg for melt suppression (same deal id).
            if !routes.iter().any(|(held, _)| held == deal) {
                routes.push((*deal, eta.clone()));
            }
        }
        ClosureEvent::StampMinted {
            orb_id,
            amount,
            crystal_spent,
        } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(Resource::Crystal, -crystal_spent.clone());
                held.bank_stamp(amount.clone());
            }
        }
        ClosureEvent::StampApertureSent { from_orb, amount, .. } => {
            if let Some(held) = peel(&mut *orbs, *from_orb) {
                held.bank_stamp(-amount.clone());
            }
            *crossed = true;
        }
        ClosureEvent::StampApertureReceived { to_orb, amount, .. } => {
            if let Some(held) = peel(&mut *orbs, *to_orb) {
                held.bank_stamp(amount.clone());
            }
            *crossed = true;
        }
        ClosureEvent::ScrapSalvaged { orb_id, amount, iron_spent, .. } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(Resource::Iron, -iron_spent.clone());
                held.bank_scrap(amount.clone());
            }
        }
        ClosureEvent::HullMinted {
            hull_id,
            orb_id,
            role,
            grade,
            scrap_spent,
        } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank_scrap(-scrap_spent.clone());
            }
            hulls.push(crate::dop::Hull {
                hull_id: *hull_id,
                role: *role,
                grade: *grade,
                home_orb: *orb_id,
                assigned_route: 0,
            });
        }
        ClosureEvent::HullAssigned { hull_id, deal } => {
            if let Some(h) = hulls.iter_mut().find(|h| h.hull_id == *hull_id) {
                h.assigned_route = *deal;
            }
        }
        ClosureEvent::HullScrapped {
            hull_id,
            orb_id,
            scrap_returned,
        } => {
            hulls.retain(|h| h.hull_id != *hull_id);
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank_scrap(scrap_returned.clone());
            }
        }
        ClosureEvent::OrderWorked {
            orb_id,
            entropy,
            syntropy,
            ..
        } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.entropy_vented += entropy.clone();
                held.syntropy_contained += syntropy.clone();
            }
        }
        ClosureEvent::ElementSalvaged {
            orb_id,
            element,
            amount,
        } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank_element(*element, -amount.clone());
                held.bank_scrap(amount.clone());
            }
        }
        ClosureEvent::NoteIssued {
            note_id,
            orb_id,
            issuer,
            holder,
            notional,
            collateral,
            maturity,
        } => {
            // O6.5: Vault-seated coll is earmarked in vault_stamp, not free bank.
            let vaulted = stations
                .iter()
                .any(|(_, s)| *s == crate::dop::Station::Vault as u8);
            if !vaulted {
                if let Some(held) = peel(&mut *orbs, *orb_id) {
                    held.bank_stamp(-collateral.clone());
                }
            }
            notes.push(crate::dop::MarketNote {
                note_id: *note_id,
                orb_id: *orb_id,
                issuer: *issuer,
                holder: *holder,
                notional: notional.clone(),
                collateral: collateral.clone(),
                maturity: *maturity,
                open: true,
            });
        }
        ClosureEvent::NoteRedeemed {
            note_id,
            orb_id,
            paid,
            seized,
        } => {
            if let Some(n) = notes.iter_mut().find(|n| n.note_id == *note_id) {
                let coll = n.collateral.clone();
                n.open = false;
                let vaulted = stations
                    .iter()
                    .any(|(_, s)| *s == crate::dop::Station::Vault as u8);
                if vaulted {
                    if *seized > ratio(0, 1) {
                        let cur = vault_stamp
                            .iter()
                            .find(|(o, _)| *o == *orb_id)
                            .map(|(_, a)| a.clone())
                            .unwrap_or_else(|| ratio(0, 1));
                        vault_stamp.retain(|(o, _)| *o != *orb_id);
                        let next = if cur > *seized {
                            cur - seized.clone()
                        } else {
                            ratio(0, 1)
                        };
                        if next > ratio(0, 1) {
                            vault_stamp.push((*orb_id, next));
                        }
                        if let Some(held) = peel(&mut *orbs, *orb_id) {
                            held.bank_stamp(seized.clone());
                            held.bank_stamp(-paid.clone());
                        }
                    } else if let Some(held) = peel(&mut *orbs, *orb_id) {
                        held.bank_stamp(-paid.clone());
                    }
                } else if let Some(held) = peel(&mut *orbs, *orb_id) {
                    // Mirror live path: full pay releases collateral; seize path credits seize then pays.
                    if *seized == ratio(0, 1) {
                        held.bank_stamp(-paid.clone());
                        held.bank_stamp(coll);
                    } else {
                        held.bank_stamp(seized.clone());
                        held.bank_stamp(-paid.clone());
                    }
                }
                // I5.6: the unpaid remainder posts to the severance
                // book — derived from the receipt's paid against the
                // register's notional, the one home the verb read.
                let unpaid = n.notional.clone() - paid.clone();
                if unpaid > ratio(0, 1) {
                    if let Some(held) = peel(&mut *orbs, *orb_id) {
                        held.severance_load += unpaid;
                    }
                }
            }
        }
        ClosureEvent::FutureOpened {
            fut_id,
            orb_id,
            long,
            short,
            resource,
            notional,
            price,
            margin,
            expiry,
        } => {
            let lock = margin.clone() + margin.clone();
            let vaulted = stations
                .iter()
                .any(|(_, s)| *s == crate::dop::Station::Vault as u8);
            if !vaulted {
                if let Some(held) = peel(&mut *orbs, *orb_id) {
                    held.bank_stamp(-lock);
                }
            }
            futures.push(crate::dop::MarketFuture {
                fut_id: *fut_id,
                orb_id: *orb_id,
                long: *long,
                short: *short,
                resource: *resource,
                notional: notional.clone(),
                price: price.clone(),
                margin: margin.clone(),
                expiry: *expiry,
                open: true,
            });
        }
        ClosureEvent::FutureSettled {
            fut_id,
            orb_id,
            mark: _,
            long_pnl,
            short_pnl: _,
        } => {
            if let Some(f) = futures.iter_mut().find(|f| f.fut_id == *fut_id) {
                f.open = false;
                let vaulted = stations
                    .iter()
                    .any(|(_, s)| *s == crate::dop::Station::Vault as u8);
                if let Some(held) = peel(&mut *orbs, *orb_id) {
                    if !vaulted {
                        let unlock = f.margin.clone() + f.margin.clone();
                        held.bank_stamp(unlock);
                    }
                    if *long_pnl > ratio(0, 1) {
                        held.bank_stamp(long_pnl.clone());
                    } else if *long_pnl < ratio(0, 1) {
                        let loss = ratio(0, 1) - long_pnl.clone();
                        let have = held.stamp_amount();
                        let take = if have < loss { have } else { loss };
                        held.bank_stamp(-take);
                    }
                }
            }
        }
        ClosureEvent::LastClearMarked {
            orb_id: _,
            resource,
            price,
        } => {
            last_clears.retain(|(r, _)| *r != *resource);
            last_clears.push((*resource, price.clone()));
        }
        ClosureEvent::ShareIssued {
            share_id,
            orb_id,
            holder,
            stream,
            units,
            par,
        } => {
            shares.push(crate::dop::MarketShare {
                share_id: *share_id,
                orb_id: *orb_id,
                holder: *holder,
                stream: *stream,
                units: units.clone(),
                par: par.clone(),
                open: true,
            });
        }
        ClosureEvent::ShareTransferred {
            share_id,
            from: _,
            to,
            units,
        } => {
            if let Some(s) = shares.iter_mut().find(|s| s.share_id == *share_id) {
                s.units = s.units.clone() - units.clone();
                if s.units == ratio(0, 1) {
                    s.open = false;
                }
                let stream = s.stream;
                let orb = s.orb_id;
                let par = s.par.clone();
                if let Some(dest) = shares
                    .iter_mut()
                    .find(|s| s.open && s.holder == *to && s.stream == stream && s.orb_id == orb)
                {
                    dest.units += units.clone();
                } else {
                    let nid = shares.iter().map(|s| s.share_id).max().unwrap_or(0) + 1;
                    shares.push(crate::dop::MarketShare {
                        share_id: nid,
                        orb_id: orb,
                        holder: *to,
                        stream,
                        units: units.clone(),
                        par,
                        open: true,
                    });
                }
            }
        }
        ClosureEvent::ShareDividend {
            share_id: _,
            orb_id,
            stream,
            resource_cut,
            amount,
        } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(*stream, -resource_cut.clone());
                held.bank_stamp(amount.clone());
            }
        }
        ClosureEvent::PerpOpened {
            perp_id,
            orb_id,
            long,
            short,
            resource,
            notional,
            entry,
            margin,
        } => {
            let lock = margin.clone() + margin.clone();
            let vaulted = stations
                .iter()
                .any(|(_, s)| *s == crate::dop::Station::Vault as u8);
            if !vaulted {
                if let Some(held) = peel(&mut *orbs, *orb_id) {
                    held.bank_stamp(-lock);
                }
            }
            perps.push(crate::dop::MarketPerp {
                perp_id: *perp_id,
                orb_id: *orb_id,
                long: *long,
                short: *short,
                resource: *resource,
                notional: notional.clone(),
                entry: entry.clone(),
                margin: margin.clone(),
                open: true,
            });
        }
        ClosureEvent::PerpFunded { .. } => {}
        ClosureEvent::PerpClosed {
            perp_id,
            orb_id,
            mark: _,
            long_pnl,
            liquidated: _,
        } => {
            if let Some(p) = perps.iter_mut().find(|p| p.perp_id == *perp_id) {
                p.open = false;
                let vaulted = stations
                    .iter()
                    .any(|(_, s)| *s == crate::dop::Station::Vault as u8);
                if let Some(held) = peel(&mut *orbs, *orb_id) {
                    if !vaulted {
                        let unlock = p.margin.clone() + p.margin.clone();
                        held.bank_stamp(unlock);
                    }
                    if *long_pnl > ratio(0, 1) {
                        held.bank_stamp(long_pnl.clone());
                    } else if *long_pnl < ratio(0, 1) {
                        let loss = ratio(0, 1) - long_pnl.clone();
                        let have = held.stamp_amount();
                        let take = if have < loss { have } else { loss };
                        held.bank_stamp(-take);
                    }
                }
            }
        }
        ClosureEvent::HullFinanced { .. } => {
            // NoteIssued + HullMinted already applied on their own receipts.
        }
        ClosureEvent::EscortOnDuty { .. } => {
            // HullAssigned already set assigned_route; this is KIN duty mark.
        }
        ClosureEvent::InterdictionStruck { .. } => {
            // Residual is in ApertureReceived delivered amount; typed audit only.
        }
        ClosureEvent::ConvoyRiskMarked { .. } => {}
        ClosureEvent::MarketSealed { resume_at } => {
            *market_sealed_until = Some(*resume_at);
        }
        ClosureEvent::MarketResumed {} => {
            *market_sealed_until = None;
        }
        ClosureEvent::SpotClearRecorded {
            resource,
            price,
            depth,
            ..
        } => {
            clear_depths.retain(|(r, _)| *r != *resource);
            clear_depths.push((*resource, *depth));
            if *depth >= 8 {
                last_clears.retain(|(r, _)| *r != *resource);
                last_clears.push((*resource, price.clone()));
            }
        }
        ClosureEvent::LiquidityRaidStruck {
            orb_id,
            fee,
            offers_voided,
        } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank_stamp(-fee.clone());
            }
            if *offers_voided > 0 {
                if let Some(i) = offers.iter().position(|o| o.orb == *orb_id) {
                    offers.remove(i);
                }
            }
        }
        ClosureEvent::ClearPoisonRefused { .. } => {}
        ClosureEvent::NoteCascadeStruck {
            orb_id,
            fee,
            pressure,
        } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank_stamp(-fee.clone());
            }
            cascade_pressures.retain(|(o, _)| *o != *orb_id);
            cascade_pressures.push((*orb_id, *pressure));
        }
        ClosureEvent::CovertTamperStruck {
            orb_id,
            cost,
            pressure,
        } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(crate::dop::Resource::Crystal, -cost.clone());
            }
            tamper_pressures.retain(|(o, _)| *o != *orb_id);
            if *pressure > ratio(0, 1) {
                tamper_pressures.push((*orb_id, pressure.clone()));
            }
        }
        ClosureEvent::MediumReinforced {
            orb_id,
            cost,
            pressure,
            resource,
        } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(*resource, -cost.clone());
            }
            tamper_pressures.retain(|(o, _)| *o != *orb_id);
            if *pressure > ratio(0, 1) {
                tamper_pressures.push((*orb_id, pressure.clone()));
            }
        }
        ClosureEvent::TamperDetected { .. } => {
            // Residual audit only; pressure already set by CovertTamper
            // or cleared/unmasked by overt terraform.
        }
        ClosureEvent::PostureReoriented {
            disposition,
            perception,
            primary,
        } => {
            // Fee is STAMP on FIRST_ORB; replay of stamp deeds elsewhere
            // is imperfect for instruments — fee applied here for fidelity.
            if let Some(held) = peel(&mut *orbs, 1) {
                held.bank_stamp(-ratio(1, 1));
            }
            *posture_disposition = Some(*disposition);
            *posture_perception = Some(*perception);
            *posture_primary = if *primary == 0 {
                None
            } else {
                Some(*primary)
            };
        }
        ClosureEvent::SpyProbed {
            orb_id,
            depth,
            fee,
        } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(crate::dop::Resource::Crystal, -fee.clone());
            }
            spy_marks.retain(|(o, _)| *o != *orb_id);
            spy_marks.push((*orb_id, *depth));
        }
        ClosureEvent::TechExfiltrated {
            tech_hash,
            fee,
            ..
        } => {
            if let Some(held) = peel(&mut *orbs, 1) {
                held.bank(crate::dop::Resource::Crystal, -fee.clone());
            }
            if !intel_hashes.contains(tech_hash) {
                intel_hashes.push(*tech_hash);
            }
        }
        ClosureEvent::InstrumentSabotaged {
            orb_id,
            grade,
            tech_hash,
        } => {
            if let Some(i) = instruments
                .iter()
                .position(|(o, h, g)| o == orb_id && h == tech_hash && g == grade)
            {
                instruments.remove(i);
            }
        }
        ClosureEvent::RailPromoted {
            deal,
            to_rail,
            ..
        } => {
            if let Some(r) = element_routes.iter_mut().find(|(d, _, _, _)| d == deal) {
                r.3 = *to_rail;
            }
        }
        ClosureEvent::StrandExported {
            orb_id,
            amount,
            ..
        } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank_stamp(-amount.clone());
            }
        }
        ClosureEvent::AnchorImported {
            orb_id,
            amount,
            ..
        } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank_stamp(amount.clone());
            }
        }
        ClosureEvent::AnchorImportRefused { .. } => {}
        ClosureEvent::VaultDeposited { orb_id, amount } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank_stamp(-amount.clone());
            }
            let cur = vault_stamp
                .iter()
                .find(|(o, _)| *o == *orb_id)
                .map(|(_, a)| a.clone())
                .unwrap_or_else(|| ratio(0, 1));
            vault_stamp.retain(|(o, _)| *o != *orb_id);
            let next = cur + amount.clone();
            if next > ratio(0, 1) {
                vault_stamp.push((*orb_id, next));
            }
        }
        ClosureEvent::VaultWithdrawn { orb_id, amount } => {
            let cur = vault_stamp
                .iter()
                .find(|(o, _)| *o == *orb_id)
                .map(|(_, a)| a.clone())
                .unwrap_or_else(|| ratio(0, 1));
            vault_stamp.retain(|(o, _)| *o != *orb_id);
            let next = if cur > *amount {
                cur - amount.clone()
            } else {
                ratio(0, 1)
            };
            if next > ratio(0, 1) {
                vault_stamp.push((*orb_id, next));
            }
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank_stamp(amount.clone());
            }
        }
        ClosureEvent::AssayConsilience { .. } => {}
        ClosureEvent::AssaySlashed {
            orb_id,
            element,
            slash,
            richness_after,
        } => {
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(crate::dop::Resource::Crystal, -slash.clone());
            }
            if let Some(v) = prospects
                .iter_mut()
                .find(|p| p.orb == *orb_id && p.element == *element)
            {
                v.richness = richness_after.clone();
            }
        }
        ClosureEvent::IsoUnionStruck {
            orb_id,
            a,
            b,
            product,
            ..
        } => {
            if let Some((spend, yielded, _)) = crate::almanac::iso_union_recipe(a, b) {
                if let Some(held) = peel(&mut *orbs, *orb_id) {
                    held.bank_element(*a, -spend.clone());
                    held.bank_element(*b, -spend);
                    held.bank_element(*product, yielded);
                }
            }
        }
        ClosureEvent::ChemCataloged { orb_id, gem, .. } => {
            let fee = crate::dop::ZoneKind::Resonator.yield_per_tick().1;
            if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(crate::dop::Resource::Crystal, -fee);
            }
            if !chem_catalog.contains(gem) {
                chem_catalog.push(*gem);
            }
        }
        ClosureEvent::DarkPullStruck { .. } => {}
        ClosureEvent::ScienceSeated {
            orb_id,
            discipline,
            rank,
            fee,
        } => {
            // Re-derive multi-resource bill (local + resonance) — not STAMP.
            // Fee field is crystal portion / primary for kernel display.
            if let Some(d) = crate::almanac::ScienceDiscipline::from_wire(*discipline) {
                let met = genesis
                    .as_ref()
                    .map(|g| g.metabolism)
                    .unwrap_or(2);
                let bill = crate::almanac::science_seat_bill(d, *rank, met, 1);
                if let Some(held) = peel(&mut *orbs, *orb_id) {
                    for (resource, amount) in &bill {
                        held.bank(*resource, -amount.clone());
                    }
                }
            } else if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(crate::dop::Resource::Crystal, -fee.clone());
            }
            if let Some(slot) = science_seats
                .iter_mut()
                .find(|(d, o, _)| *d == *discipline && *o == *orb_id)
            {
                slot.2 = *rank;
            } else {
                science_seats.push((*discipline, *orb_id, *rank));
            }
        }
        ClosureEvent::LabCreated {
            orb_id,
            path,
            fee,
            exploded,
            hidden_pos,
            ..
        } => {
            if let Some(p) = crate::almanac::LabPath::from_wire(*path) {
                let met = genesis
                    .as_ref()
                    .map(|g| g.metabolism)
                    .unwrap_or(2);
                let bill = crate::almanac::lab_create_bill(p, met);
                if let Some(held) = peel(&mut *orbs, *orb_id) {
                    for (resource, amount) in &bill {
                        held.bank(*resource, -amount.clone());
                    }
                }
            } else if let Some(held) = peel(&mut *orbs, *orb_id) {
                held.bank(crate::dop::Resource::Crystal, -fee.clone());
            }
            if !*exploded {
                // Apply known positives by path (call-not-copy of live path).
                match crate::almanac::LabPath::from_wire(*path) {
                    Some(crate::almanac::LabPath::SynthMod) => {
                        let g = synth_mods
                            .iter()
                            .find(|(o, _)| *o == *orb_id)
                            .map(|(_, g)| *g)
                            .unwrap_or(0)
                            .saturating_add(1)
                            .min(7);
                        synth_mods.retain(|(o, _)| *o != *orb_id);
                        if g > 0 {
                            synth_mods.push((*orb_id, g));
                        }
                        if *hidden_pos {
                            let g2 = g.saturating_add(1).min(7);
                            synth_mods.retain(|(o, _)| *o != *orb_id);
                            if g2 > 0 {
                                synth_mods.push((*orb_id, g2));
                            }
                        }
                    }
                    Some(crate::almanac::LabPath::CloneWork) => {
                        let mut c = clone_counts
                            .iter()
                            .find(|(o, _)| *o == *orb_id)
                            .map(|(_, c)| *c)
                            .unwrap_or(0)
                            .saturating_add(1);
                        if *hidden_pos {
                            c = c.saturating_add(1);
                        }
                        clone_counts.retain(|(o, _)| *o != *orb_id);
                        if c > 0 {
                            clone_counts.push((*orb_id, c));
                        }
                    }
                    Some(crate::almanac::LabPath::SkinWork) => {
                        let mut g = skin_mods
                            .iter()
                            .find(|(o, _)| *o == *orb_id)
                            .map(|(_, g)| *g)
                            .unwrap_or(0)
                            .saturating_add(1)
                            .min(7);
                        if *hidden_pos {
                            g = g.saturating_add(1).min(7);
                        }
                        skin_mods.retain(|(o, _)| *o != *orb_id);
                        if g > 0 {
                            skin_mods.push((*orb_id, g));
                        }
                    }
                    _ => {}
                }
            }
        }
        ClosureEvent::ScienceExploded {
            orb_id,
            path,
            kind,
            resource,
            slash,
        } => {
            if let Some(res) = crate::dop::Resource::from_wire(*resource) {
                if *slash > ratio(0, 1) {
                    if let Some(held) = peel(&mut *orbs, *orb_id) {
                        let have = held.banked_amount(res);
                        let take = if have > *slash {
                            slash.clone()
                        } else {
                            have
                        };
                        held.bank(res, -take);
                    }
                }
            }
            match crate::almanac::ExplosionKind::from_wire(*kind) {
                Some(crate::almanac::ExplosionKind::SynthRogue) => {
                    let g = synth_mods
                        .iter()
                        .find(|(o, _)| *o == *orb_id)
                        .map(|(_, g)| *g)
                        .unwrap_or(0)
                        .saturating_sub(1);
                    synth_mods.retain(|(o, _)| *o != *orb_id);
                    if g > 0 {
                        synth_mods.push((*orb_id, g));
                    }
                }
                Some(crate::almanac::ExplosionKind::CloneRevolt) => {
                    let c = clone_counts
                        .iter()
                        .find(|(o, _)| *o == *orb_id)
                        .map(|(_, c)| *c)
                        .unwrap_or(0)
                        .saturating_sub(1);
                    clone_counts.retain(|(o, _)| *o != *orb_id);
                    if c > 0 {
                        clone_counts.push((*orb_id, c));
                    }
                }
                Some(crate::almanac::ExplosionKind::SkinUnmask) => {
                    let g = skin_mods
                        .iter()
                        .find(|(o, _)| *o == *orb_id)
                        .map(|(_, g)| *g)
                        .unwrap_or(0)
                        .saturating_sub(1);
                    skin_mods.retain(|(o, _)| *o != *orb_id);
                    if g > 0 {
                        skin_mods.push((*orb_id, g));
                    }
                    if let Some(slot) = spy_marks.iter_mut().find(|(o, _)| *o == *orb_id) {
                        slot.1 = slot.1.saturating_add(1);
                    } else {
                        spy_marks.push((*orb_id, 1));
                    }
                }
                Some(crate::almanac::ExplosionKind::BioChemCross) => {
                    let p = cascade_pressures
                        .iter()
                        .find(|(o, _)| *o == *orb_id)
                        .map(|(_, p)| *p)
                        .unwrap_or(0)
                        .saturating_add(1);
                    cascade_pressures.retain(|(o, _)| *o != *orb_id);
                    cascade_pressures.push((*orb_id, p));
                }
                _ => {
                    let _ = path;
                }
            }
        }
        ClosureEvent::RogueFactionStruck { orb_id, pressure } => {
            rogue_pressure.retain(|(o, _)| *o != *orb_id);
            if *pressure > 0 {
                rogue_pressure.push((*orb_id, *pressure));
            }
        }
    }
    }
}

pub fn replay(chronicle: &Chronicle) -> Committed {
    let mut stepper = Replayer::new();
    for receipt in chronicle.receipts() {
        stepper.apply(receipt);
    }
    stepper.into_committed()
}
