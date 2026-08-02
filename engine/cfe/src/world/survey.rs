//! SURVEY AND TERRAIN — scanning, charting, prospecting,
//! terraforming, deposits.

use super::*;

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
        });
        self.close(
            ClosureEvent::PlanetSited { orb_id: orb, albedo, orbit_r },
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
    pub fn constellation(&self, orb: u64) -> Vec<(u8, bool, u64, u64)> {
        let now = self.clock.now();
        let deposits = self.crystal_deposits(now);
        let blank = CrystalDeposit::default();
        let sheet = deposits
            .iter()
            .find(|(held, _)| *held == orb)
            .map(|(_, sheet)| sheet)
            .unwrap_or(&blank);
        (0u8..=6)
            .map(|condition| {
                let held = self
                    .techs
                    .iter()
                    .any(|(o, c, _)| *o == orb && *c == condition);
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
                    _ => (
                        u64::from(self.condition_holds(orb, condition, now, sheet)),
                        1,
                    ),
                };
                (condition, held, progress, threshold)
            })
            .collect()
    }
    /// Chart a daughter colony as the sovereign.
    pub fn chart(&mut self, name: &str, source: u64) -> Result<u64, ActionRefused> {
        self.chart_as(SOVEREIGN, name, source)
    }
    /// P1 — chart a daughter colony on a new planet: the founding
    /// reserves MOVE from the source colony's bank to the daughter's.
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
        let mut daughter = ColonyOrb {
            orb_id,
            name: name.to_string(),
            ..ColonyOrb::default()
        };
        for (resource, amount) in founding_reserves() {
            daughter.bank(resource, amount);
        }
        self.orbs.push(daughter);
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
        self.permitted(actor, Scope::Collect)?;
        if self.orb_by_id(exemplar).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb: exemplar });
        }
        let Some(paying) = self.orb_by_id(prober) else {
            return Err(ActionRefused::NoSuchOrb { orb: prober });
        };
        // Affordability judged PURE before anything moves (crystals
        // are never demurraged, so the pre-touch read is the
        // post-decay read): a refusal draws nothing, reads nothing,
        // orphans nothing.
        let mut draw = crate::dop::scan_draw();
        // PRACTICE-1 the Vigil: the lens disciplined — the draw halves.
        if self.practices.contains(&3) {
            draw *= ratio(1, 2);
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
        let ease = ease_of(&self.techs, prober);
        let star = self.star;
        let held = peel(&mut self.orbs, prober).expect("judged above");
        touch(held, star, &mut self.chronicle, stamp, &ease);
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
}
