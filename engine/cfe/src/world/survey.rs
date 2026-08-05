//! SURVEY AND TERRAIN — scanning, charting, prospecting,
//! terraforming, deposits.

use super::*;
use crate::literature::{work_of, WORKS};

/// **THE SKY IS BASE-11** (operator ruling 2026-08-02, *uplift to
/// base-11*). Eleven nodes, in two tiers, and `7` is not one of them:
///
/// ```text
///     0        the solar leap
///     1..=5    the five crystallized conditions
///     6        the gifted crown — completes the BASE
///     7        NOT A SKY NODE — the re-emergence lesson (PRACTICE-1),
///              which "rides separately as TechDiscovered condition 7"
///              (`dop.rs`) and eases `deposit_window` 240 -> 180
///     8..=11   THE UPLIFT TIER — one node per chartered work
/// ```
///
/// Seven base nodes plus four works is ELEVEN. The condition FIELD is a
/// shared namespace with two populations — sky nodes and separately
/// riding ones — and `7` was already spoken for by landed law before
/// the uplift. A first cut of this ruling put the works at `7..=10` and
/// so gave one number two meanings: `deposit_window` reads `techs` for
/// the lesson while the sky claimed the same slot for VALOR, which
/// would have rendered a held lesson as an unreached work and hidden
/// the real node entirely.
///
/// **The crown's gate does not move.** It still gates on `0..=5` and
/// still completes base-7; the uplift sits ABOVE it rather than
/// widening what it waits for.
/// **DERIVED, NOT TYPED** (operator ruling 2026-08-05: *bring these down
/// to the theorem level*). Eleven is a SUM and the sum is the sky's own
/// structure — the base tier plus one node per chartered work. Written
/// as `11`, the day a sixth condition lands someone edits one place and
/// not the others.
/// **WHAT KIND OF NODE A CONDITION IS** — the classification every
/// count below is derived from (operator ruling 2026-08-05: *all of
/// these constants need to be derived dynamically*).
///
/// A first cut typed `1`, `5`, `1` and `4`. Those are not numbers, they
/// are the SIZES OF POPULATIONS in the condition namespace, and the
/// namespace already knows its own members: `dop::tech_node_of` answers
/// for the crystallized tier, `dop::tech_node_name` names the crown and
/// the lesson, and `literature::WORKS` lists the chartered works. Count
/// what is there instead of restating how much of it there was.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    /// Condition 0 — the solar leap. One, because a world has one star.
    Leap,
    /// The crystallized tier — every condition carrying a tech node.
    Crystallized,
    /// The gifted crown, granted rather than earned.
    Crown,
    /// PRACTICE-1's re-emergence lesson — rides the condition field but
    /// is NOT a sky node.
    Lesson,
    /// The uplift tier — one node per chartered work.
    Work,
    /// Outside the namespace.
    None,
}

/// Classify a condition by asking the vocabularies, never a table of
/// numbers.
pub fn node_kind(condition: u8) -> NodeKind {
    if condition == 0 {
        return NodeKind::Leap;
    }
    if crate::dop::tech_node_of(condition).is_some() {
        return NodeKind::Crystallized;
    }
    // Classified by MEANING, asked of the vocabulary — never by parsing
    // a composed name (`tech_node_name` returns "{voice} {concept}", so
    // a `starts_with` reads the voice) and never by a number.
    match crate::dop::condition_concept(condition) {
        // PRACTICE-1's lesson rides the condition field and is not a sky
        // node.
        Some("re-emergence") => NodeKind::Lesson,
        // The gifted crown completes the base.
        Some("terraforming") => NodeKind::Crown,
        Some(_) => NodeKind::Crystallized,
        None => {
            // Past the named vocabulary lie the uplift nodes, one per
            // chartered work.
            let named = 0u8..first_work();
            match !named.contains(&condition) && condition < first_work() + works() {
                true => NodeKind::Work,
                false => NodeKind::None,
            }
        }
    }
}

/// How many conditions are of a kind — counted, never asserted.
pub fn count_of(kind: NodeKind) -> u8 {
    (0u8..=u8::MAX)
        .take(64)
        .filter(|c| match kind {
            NodeKind::Leap => *c == 0,
            NodeKind::Crystallized => crate::dop::tech_node_of(*c).is_some(),
            NodeKind::Crown => {
                crate::dop::condition_concept(*c) == Some("terraforming")
            }
            NodeKind::Lesson => {
                crate::dop::condition_concept(*c) == Some("re-emergence")
            }
            _ => false,
        })
        .count() as u8
}

/// The chartered works — the UPLIFT tier's own size, read from the
/// literature vocabulary rather than restated here.
pub fn works() -> u8 {
    crate::literature::WORKS.len() as u8
}

/// One, because a world has ONE star to make the leap to.
pub const fn solar_leap() -> u8 {
    1
}
/// The BASE tier — what the crown completes and `base_seven_complete`
/// waits for. Its gate does not move.
/// The BASE tier, COUNTED: the leap, every crystallized condition, and
/// the gifted crown.
pub fn base_nodes() -> u8 {
    count_of(NodeKind::Leap) + count_of(NodeKind::Crystallized) + count_of(NodeKind::Crown)
}
/// The whole stack, COUNTED: from the first work through the last one.
/// There is no `11` anywhere — ask the vocabularies and they add to it.
pub fn last_condition() -> u8 {
    first_work() + works() - 1
}
/// The first node of the uplift tier, where the works begin.
/// `7` is skipped: it belongs to PRACTICE-1 and is not a sky node.
/// The first node of the uplift tier.
///
/// The base occupies indices `0..=6` — seven nodes — and PRACTICE-1's
/// lesson rides at `7`, so the works begin at `base + lesson = 8`. The
/// counts ARE the indices consumed; there is no extra `+1` to add, and a
/// first cut that added one put VALOR at 9 and left 8 unnamed.
pub fn first_work() -> u8 {
    base_nodes() + count_of(NodeKind::Lesson)
}
/// PRACTICE-1's re-emergence lesson. Held in `techs` like a sky node,
/// rendered in the sky like none — it rides separately by design.
pub const RE_EMERGENCE: u8 = 7;

impl World {
    /// TERRA-1: does this world hold the gifted terraforming node?
    pub fn can_terraform(&self) -> bool {
        self.techs.iter().any(|(_, condition, _)| *condition == 6)
    }
    /// TERRA-1 — TERRAFORM: move a SITED planet between substrate
    /// bands by re-seating its albedo and orbit — the highest
    /// technology a civilization can hold, and only ever GIFTED
    /// (licensed down from tier 0 once the base-7 stack stands).
    /// Receipted as a fresh PlanetSited — replay takes the last
    /// seat; the orbit that "does not move" moves only under the
    /// crown tech.
    pub fn terraform(
        &mut self,
        orb: u64,
        albedo: Ratio,
        orbit_r: Ratio,
    ) -> Result<(), ActionRefused> {
        use crate::dop::ThermalSite;
        if !self.can_terraform() {
            return Err(ActionRefused::TerraformUnearned);
        }
        if self.star.is_none() {
            return Err(ActionRefused::NoStarCommissioned);
        }
        let zero = ratio(0, 1);
        let one = ratio(1, 1);
        if albedo < zero || albedo >= one || orbit_r <= zero {
            return Err(ActionRefused::BadSiting);
        }
        let held = peel(&mut self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        if held.thermal.is_none() {
            // A first siting is the solar leap's door, not terraform's.
            return Err(ActionRefused::NoStarCommissioned);
        }
        held.thermal = Some(ThermalSite {
            albedo: albedo.clone(),
            orbit_r: orbit_r.clone(),
            // Historical siting carries no angle — an orb has a radius
            // and no place until one is receipted.
            theta: None, psi: None,
        });
        self.close(
            ClosureEvent::PlanetSited { orb_id: orb, albedo, orbit_r, theta: None , psi: None },
            SOVEREIGN,
        );
        Ok(())
    }
    /// TECH-1b (10 §3) — THE CRYSTAL SHEAR: one pass over the
    /// chronicle deposits every orb's condition readings (famine
    /// spans, overheat spans, the vent-window sums) — the converge
    /// deposit idiom, never a rescan per condition per orb. State
    /// readings (sink, surplus) evaluate directly at the boundary.
    /// #41 — the INCREMENTAL shear: only receipts past the cursor
    /// deposit; windowed entries prune at each boundary, so the sweep
    /// stays bounded however old the chronicle grows. The carried
    /// sheet is a pure cache — same receipts, same sheet as the
    /// full scan, by construction (both run deposit_receipt).
    pub(super) fn crystal_deposits_at(&mut self, now: u64) -> Vec<(u64, CrystalDeposit)> {
        let (cursor, mut sheet) = std::mem::take(&mut self.shear_cursor);
        let receipts = self.chronicle.receipts();
        for receipt in &receipts[cursor.min(receipts.len())..] {
            deposit_receipt(&mut sheet, receipt);
        }
        let floor = now.saturating_sub(self.deposit_window());
        for (_, held) in sheet.iter_mut() {
            held.vented_entries.retain(|(stamp, _)| *stamp >= floor);
            held.encased_entries.retain(|(stamp, _)| *stamp >= floor);
        }
        self.shear_cursor = (receipts.len(), sheet.clone());
        sheet
    }
    fn crystal_deposits(&self, now: u64) -> Vec<(u64, CrystalDeposit)> {
        let _ = now;
        let mut sheet: Vec<(u64, CrystalDeposit)> = Vec::new();
        for receipt in self.chronicle.receipts() {
            deposit_receipt(&mut sheet, receipt);
        }
        sheet
    }
    /// THE CONSTELLATION (operator ruling 2026-07-30): the base-7
    /// stack as a per-orb reading for every surface — each condition's
    /// held state and its PROGRESS toward crystallizing, derived from
    /// the same deposit sheet the crystallizer reads (never a second
    /// bookkeeping). Returns (condition, held, progress, threshold):
    /// the timer conditions report ticks (insulation 480, discipline
    /// 240); the state conditions and the kernel report 1-of-1 when
    /// their reading holds; the crown reports base-7 readiness (the
    /// gift needs the stack, 6 of 6 earned nodes).
    /// The chartered work a node of the uplift tier stands for, if it
    /// is one. `7..=10` map to the charter's order.
    pub fn work_at(condition: u8) -> Option<&'static str> {
        match (first_work()..=last_condition()).contains(&condition) {
            true => WORKS.get((condition - first_work()) as usize).copied(),
            false => None,
        }
    }

    /// Has this line CLAIMED the work this node stands for? Derived
    /// from the aspiration's witnesses — the evidence IS the node.
    pub fn work_claimed(&self, orb: u64, condition: u8) -> bool {
        let Some(name) = World::work_at(condition) else {
            return false;
        };
        let work = work_of(name);
        self.orb_by_id(orb).is_some_and(|held| {
            held.aspirations
                .iter()
                .any(|held| held.work == work && held.claimed())
        })
    }

    pub fn constellation(&self, orb: u64) -> Vec<(u8, bool, u64, u64)> {
        let now = self.clock.now();
        let deposits = self.crystal_deposits(now);
        let blank = CrystalDeposit::default();
        let sheet = deposits
            .iter()
            .find(|(held, _)| *held == orb)
            .map(|(_, sheet)| sheet)
            .unwrap_or(&blank);
        (0u8..=last_condition())
            .filter(|condition| *condition != RE_EMERGENCE)
            .map(|condition| {
                // The BASE tier is earned into `techs`. The UPLIFT tier
                // (7..=10) is not granted at all — a work is held when
                // the line CLAIMED it, so the node IS the deed.
                let held = match condition >= first_work() {
                    true => self.work_claimed(orb, condition),
                    false => self
                        .techs
                        .iter()
                        .any(|(o, c, _)| *o == orb && *c == condition),
                };
                let (progress, threshold) = match condition {
                    0 => (u64::from(self.knows_its_sun()), 1),
                    1 => {
                        let open = sheet
                            .famine_open
                            .map(|struck| now.saturating_sub(struck))
                            .unwrap_or(0);
                        ((sheet.famine_total + open).min(480), 480)
                    }
                    2 => (
                        sheet
                            .overheat_open
                            .map(|struck| now.saturating_sub(struck))
                            .unwrap_or(0)
                            .min(240),
                        240,
                    ),
                    6 => {
                        let earned = (0u8..=5)
                            .filter(|c| {
                                self.techs.iter().any(|(o, k, _)| *o == orb && k == c)
                            })
                            .count() as u64;
                        (earned, 6)
                    }
                    // THE UPLIFT TIER — one node per chartered work,
                    // held on a WITNESSED claim. A deed is the upgrade.
                    held_work if held_work >= first_work() => {
                        (u64::from(self.work_claimed(orb, condition)), 1)
                    }
                    _ => (
                        u64::from(self.condition_holds(orb, condition, now, sheet)),
                        1,
                    ),
                };
                (condition, held, progress, threshold)
            })
            .collect()
    }
    /// Chart a charted colony as the sovereign.
    pub fn chart(&mut self, name: &str, source: u64) -> Result<u64, ActionRefused> {
        self.chart_as(SOVEREIGN, name, source)
    }
    /// P1 — chart a charted colony on a new planet: the founding
    /// reserves MOVE from the source colony's bank to the charted's.
    /// Conservation holds — chartering mints nothing. Build scope.
    pub fn chart_as(&mut self, actor: u64, name: &str, source: u64) -> Result<u64, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        if self.orbs.iter().any(|held| held.name == name) {
            return Err(ActionRefused::PlanetNameTaken { name: name.to_string() });
        }
        let source_orb = peel(&mut self.orbs, source)
            .ok_or(ActionRefused::NoSuchOrb { orb: source })?;
        for (resource, amount) in founding_reserves() {
            let have = source_orb.banked_amount(resource);
            if have < amount {
                return Err(ActionRefused::CannotAfford {
                    resource,
                    need: Box::new(amount),
                    have: Box::new(have),
                });
            }
        }
        for (resource, amount) in founding_reserves() {
            source_orb.bank(resource, -amount);
        }
        let orb_id = self.orbs.iter().map(|held| held.orb_id).max().unwrap_or(0) + 1;
        let mut charted = ColonyOrb {
            orb_id,
            name: name.to_string(),
            ..ColonyOrb::default()
        };
        for (resource, amount) in founding_reserves() {
            charted.bank(resource, amount);
        }
        self.orbs.push(charted);
        self.close(
            ClosureEvent::ColonyCharted {
                orb_id,
                source_orb: source,
                name: name.to_string(),
            },
            actor,
        );
        Ok(orb_id)
    }
    /// N4 (NS-2 RQP) — the scanner: a resonance query over this
    /// world's orb population through the {S, G} lens. The reading is
    /// PURE and receipts NOTHING; the one mutation is the crystal draw
    /// (NS-2 §1.5), which closes FIRST — the lens reads the committed
    /// world as it stands once open. Sovereign convenience.
    pub fn scan(&mut self, prober: u64, exemplar: u64) -> Result<crate::query::ScanRead, ActionRefused> {
        self.scan_as(SOVEREIGN, prober, exemplar)
    }
    /// The contained form. Collect scope in v1: the draw moves banked
    /// value, and collect is the scope that moves value into and out of
    /// a bank — a dedicated scan scope awaits an operator ruling.
    pub fn scan_as(
        &mut self,
        actor: u64,
        prober: u64,
        exemplar: u64,
    ) -> Result<crate::query::ScanRead, ActionRefused> {
        // P2: the standing supply the sink's EPOCH 1 rate reads,
        // snapshotted BEFORE any orb is peeled — an owned Vec, so no
        // borrow of `self.orbs` is held across the mutable touch below.
        let supply = standing_supply(&self.orbs);
        self.permitted(actor, Scope::Collect)?;
        if self.orb_by_id(exemplar).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb: exemplar });
        }
        let Some(paying) = self.orb_by_id(prober) else {
            return Err(ActionRefused::NoSuchOrb { orb: prober });
        };
        // Affordability gated PURE before anything moves (crystals
        // are never demurraged, so the pre-touch read is the
        // post-decay read): a refusal draws nothing, reads nothing,
        // orphans nothing.
        let mut draw = crate::dop::scan_draw();
        // PRACTICE-1 the Restraint: the lens disciplined — the draw halves.
        if self.practices.contains(&3) {
            draw *= ratio(1, 2);
        }
        // 28_VOCABULARY V0, CHANNEL_SURVEY: the sealed class's own
        // mechanism. A metashifter sees for half the draw; a
        // pelagic-aerial line, having no fixed vantage, pays more.
        if let Some((class, _)) = self.taxon {
            let binding = crate::vocabulary::archetype_binding(class);
            draw *= crate::vocabulary::factor_on(&binding, crate::vocabulary::CHANNEL_SURVEY);
        }
        let have = paying.banked_amount(Resource::Crystal);
        if have < draw {
            return Err(ActionRefused::CannotAfford {
                resource: Resource::Crystal,
                need: Box::new(draw),
                have: Box::new(have),
            });
        }
        // The draw closes first, at the touch law (SINK-1): due
        // demurrage settles, the anchor moves, the crystal leaves.
        let stamp = self.clock.now();
        let ease = ease_of(&self.techs, prober, stamp);
        let star = self.star;
        let held = peel(&mut self.orbs, prober).expect("judged above");
        touch(held, star, &mut self.chronicle, stamp, &ease, &supply);
        held.bank(Resource::Crystal, -draw.clone());
        self.close(ClosureEvent::CrystalDrawn { orb_id: prober, amount: draw }, actor);
        let stamp = self.chronicle.receipts().last().expect("just closed").stamp;
        let exemplar_orb = self.orb_by_id(exemplar).expect("judged above");
        let (probed, hits, refusals) = crate::query::resonance_read(exemplar_orb, &self.orbs, star);
        // G3 (NS-2 §4): the translation annotation — every candidate
        // in ONE world shares the line's basis, so the local read
        // survives whole; the cross-frame pairing composes at N5/N6
        // through the same table.
        let basis = self.genesis.as_ref().map(|held| held.perception).unwrap_or(0);
        let translations = hits
            .iter()
            .map(|orb| (*orb, crate::dop::translation_survival(basis, basis)))
            .collect();
        Ok(crate::query::ScanRead { stamp, probed, hits, refusals, translations })
    }
    /// ELEMENT-GRAMMAR-1 (14 §3) — the mint deed: read one stratum of
    /// what the genesis seed always held. Identity = BLAKE3(receipt
    /// 0's digest ‖ orb ‖ stratum) — fixed at founding, so a redraw
    /// cannot re-roll; properties derive from the hash under the
    /// star's abundance arbor; the receipt carries them whole so
    /// replay reads, never rolls. The mint creates identity, never
    /// matter (the assay moves matter). Sovereign convenience.
    pub fn prospect(&mut self, orb: u64, stratum: u8) -> Result<crate::dop::Prospect, ActionRefused> {
        self.prospect_as(SOVEREIGN, orb, stratum)
    }
    /// The contained form. Build scope: prospecting opens the world.
    pub fn prospect_as(
        &mut self,
        actor: u64,
        orb: u64,
        stratum: u8,
    ) -> Result<crate::dop::Prospect, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.orb_by_id(orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        let Some(class) = self.star else {
            return Err(ActionRefused::NoStarCommissioned);
        };
        if self.prospects.iter().any(|held| held.orb == orb && held.stratum == stratum) {
            return Err(ActionRefused::StratumProspected { orb, stratum });
        }
        // The genesis root is the FOUNDING receipt's digest — receipt
        // 0 is the sovereign's registration and identical across all
        // worlds (measured); the founding carries the name, so every
        // world's seed is its own.
        let root = self
            .chronicle
            .receipts()
            .iter()
            .find(|receipt| matches!(receipt.event, ClosureEvent::ColonyFounded { .. }))
            .expect("a founded world holds its founding")
            .own
            .0;
        let mut seed = root.to_vec();
        seed.extend_from_slice(&orb.to_le_bytes());
        seed.push(stratum);
        let element: [u8; 32] = blake3::hash(&seed).into();
        let (family, tier, richness) = crate::dop::element_properties(&element, class);
        let name = crate::dop::element_name(family, &element);
        let stamp = self.clock.now();
        self.prospects.push(Prospected {
            orb,
            stratum,
            element,
            richness: richness.clone(),
            last_assay: stamp,
        });
        self.close(
            ClosureEvent::ProspectStruck {
                orb_id: orb,
                stratum,
                element,
                family,
                tier,
                richness: richness.clone(),
                name: name.clone(),
            },
            actor,
        );
        Ok(crate::dop::Prospect { element, family, tier, richness, name })
    }
    /// Seal a closure with its actor's generation tag (NS-6).
    /// D3 (02 §2.5) — chart the star's terrain: the OPT-IN deed. The
    /// seed is the FOUNDING receipt's digest (chronicle-seeded,
    /// CHARTER §9), receipted once as `WorldCharted`; the tile classes
    /// derive pure from it forever. Build scope — reading the map is
    /// the surveyor's office. Uncharted stars read all Standard.
    pub fn chart_terrain(&mut self) -> Result<[u8; 32], ActionRefused> {
        self.chart_terrain_as(SOVEREIGN)
    }
    pub fn chart_terrain_as(&mut self, actor: u64) -> Result<[u8; 32], ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.terrain_seed.is_some() {
            return Err(ActionRefused::TerrainAlreadyCharted);
        }
        let Some(founding) = self.chronicle.receipts().first() else {
            return Err(ActionRefused::NoColony);
        };
        let seed = founding.own.0;
        self.close(ClosureEvent::WorldCharted { seed }, actor);
        self.terrain_seed = Some(seed);
        Ok(seed)
    }
    /// D3: the charted seed, if the deed has been done.
    pub fn terrain_seed(&self) -> Option<[u8; 32]> {
        self.terrain_seed
    }
    /// PRACTICE-1: the condition window — 240, eased ×3/4 to 180
    /// while a re-emergence lesson (condition 7) is held AND alignment
    /// holds (the operator's speed boost to understanding; placeholder
    /// constants, the syntropy floor at 1/2).
    pub(super) fn deposit_window(&self) -> u64 {
        let relearned = self.techs.iter().any(|(_, condition, _)| *condition == 7);
        let aligned = self
            .syntropy_ratio()
            .map(|ratio_read| ratio_read >= ratio(1, 2))
            .unwrap_or(true);
        if relearned && aligned {
            180
        } else {
            240
        }
    }

    /// **THE ESTATE SURVEY** (tag 97, E5b — operator rulings 2026-08-05:
    /// *survey is a derivation of observation; observations must be
    /// witnessed; surveys must be metered*). Build scope — reading the
    /// map is the surveyor's office, the `chart_terrain` precedent.
    ///
    /// The three theorems, enforced here in order, refusal-pure (every
    /// check before any mutation — a refusal orphans nothing):
    ///
    /// 1. WITNESSED: one witness per orb, the digest of that orb's own
    ///    founding receipt (`ColonyFounded` / `ColonyCharted`), gathered
    ///    from the chronicle. An orb the chronicle cannot witness
    ///    refuses `ClaimWithoutWitness` — the LIT-1 refusal, reused
    ///    because it is the same law: a claim stands on receipts or it
    ///    does not stand.
    /// 2. METERED: `cost = survey_meter().1 × volume`, drawn from
    ///    `payer` before the receipt closes. Unpayable refuses
    ///    `CannotAfford` with nothing written.
    /// 3. DERIVED: the receipt seeds from the chronicle's terminal
    ///    digest (CHARTER §9); parcels derive pure from it forever via
    ///    [`crate::dop::parcels_of`] — see [`Self::parcels`].
    pub fn survey_estate(&mut self, payer: u64) -> Result<[u8; 32], ActionRefused> {
        self.survey_estate_as(SOVEREIGN, payer)
    }

    pub fn survey_estate_as(
        &mut self,
        actor: u64,
        payer: u64,
    ) -> Result<[u8; 32], ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        // THEOREM 1 — the witnesses, gathered before anything moves.
        let mut covered: Vec<(u64, [u8; 32])> = Vec::with_capacity(self.orbs.len());
        for held in &self.orbs {
            let witness = self
                .chronicle
                .receipts()
                .iter()
                .find(|receipt| match &receipt.event {
                    ClosureEvent::ColonyFounded { orb_id, .. }
                    | ClosureEvent::ColonyCharted { orb_id, .. } => *orb_id == held.orb_id,
                    _ => false,
                })
                .map(|receipt| receipt.own.0)
                .ok_or(ActionRefused::ClaimWithoutWitness { name: held.name.clone() })?;
            covered.push((held.orb_id, witness));
        }
        // THEOREM 2 — the meter, verified post-check and drawn pre-close.
        let (resource, rate) = crate::dop::survey_meter();
        let cost = rate * ratio(covered.len() as i64, 1);
        let seed = self
            .chronicle
            .receipts()
            .last()
            .map(|receipt| receipt.own.0)
            .expect("a world with orbs has receipts");
        let held = peel(&mut self.orbs, payer).ok_or(ActionRefused::NoSuchOrb { orb: payer })?;
        let have = held.banked_amount(resource);
        if have < cost {
            return Err(ActionRefused::CannotAfford {
                resource,
                need: Box::new(cost),
                have: Box::new(have),
            });
        }
        held.bank(resource, -cost.clone());
        // THEOREM 3 — the receipt the parcels derive from.
        self.close(
            ClosureEvent::EstateSurveyed { seed, payer, covered, cost },
            actor,
        );
        Ok(seed)
    }

    /// **THE PARCELS — derived, never stored.** The latest survey's
    /// receipt is the sole source: its seed and its covering, through
    /// [`crate::dop::parcels_of`]. `None` before any survey — an
    /// unsurveyed estate has no parcels, because deriving one would be
    /// an unmetered survey.
    pub fn parcels(&self) -> Option<Vec<crate::dop::Parcel>> {
        self.chronicle.receipts().iter().rev().find_map(|receipt| {
            match &receipt.event {
                ClosureEvent::EstateSurveyed { seed, covered, .. } => {
                    let orbs: Vec<u64> = covered.iter().map(|(orb, _)| *orb).collect();
                    Some(crate::dop::parcels_of(seed, &orbs))
                }
                _ => None,
            }
        })
    }
}

