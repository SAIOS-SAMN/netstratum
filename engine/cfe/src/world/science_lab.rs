//! Seated science labs — paid discipline unlocks, dual ± outcomes, explosion.
//!
//! Law: no free capability. Seat with crystal (resonator-eighth × weight × rank).
//! Creation always lands known positive + known negative; hidden poles
//! conditional; explosion when derived risk ≥ 1/2.

use super::*;
use crate::almanac::{
    bill_crystal_portion, lab_create_bill, lab_pole_card, lab_requires_off_planet,
    science_seat_affinity, science_seat_bill, seat_requires_off_planet, ExplosionKind, LabPath,
    LabPoleCard, ScienceDiscipline, SeatAffinity,
};
use crate::dop::{Resource, ZoneKind};

impl World {
    /// Taxon economy reading (mode · planet fit · flatlander flag).
    pub fn taxon_economy_of(&self, orb: u64) -> crate::almanac::TaxonEconomy {
        let (class, clade) = self.taxon.unwrap_or((0, u16::from(crate::almanac::FLATLANDER_CLADE)));
        // Unsealed taxon defaults flatlander baseline for play readability.
        let clade = if self.taxon.is_none() {
            u16::from(crate::almanac::FLATLANDER_CLADE)
        } else {
            clade
        };
        let class = if self.taxon.is_none() { 7 } else { class }; // apex-solitary baseline class
        let region = self.thermal_region_of(orb);
        crate::almanac::taxon_economy(class, clade, region)
    }

    pub fn regenerator_tech_need(&self) -> u8 {
        crate::almanac::regenerator_tech_need(self.taxon_economy_of(FIRST_ORB).mode)
    }

    pub fn extractor_soft_cap(&self) -> u64 {
        crate::almanac::extractor_soft_cap(self.taxon_economy_of(FIRST_ORB).mode)
    }

    fn thermal_region_of(&self, orb: u64) -> Option<crate::almanac::ThermalRegion> {
        let class = self.star?;
        let site = self.orb_by_id(orb)?.thermal.as_ref()?;
        let t = crate::almanac::grained_t_eq4(
            &class.luminosity(),
            site.albedo.value(),
            site.orbit_r.value(),
        );
        Some(crate::almanac::region_of(&t))
    }

    /// Off-planet resourcing evidence for advanced bench tech.
    ///
    /// True if the line has crossed the void, holds an element route,
    /// or holds any ROUTE-1 deal (aperture-capable resourcing path).
    pub fn off_planet_ready(&self) -> bool {
        self.has_crossed()
            || !self.element_routes.is_empty()
            || !self.routes.is_empty()
    }

    fn require_off_planet(&self, kind: u8) -> Result<(), ActionRefused> {
        if self.off_planet_ready() {
            Ok(())
        } else {
            Err(ActionRefused::OffPlanetResourceRequired { kind })
        }
    }

    /// Debit a multi-resource bill; pure refuse if any line short.
    fn afford_bill(
        &self,
        orb: u64,
        bill: &[(Resource, crate::ratio::Ratio)],
    ) -> Result<(), ActionRefused> {
        let held = peek(&self.orbs, orb).ok_or(ActionRefused::NoSuchOrb { orb })?;
        for (resource, need) in bill {
            let have = held.banked_amount(*resource);
            if have < *need {
                return Err(ActionRefused::CannotAfford {
                    resource: *resource,
                    need: Box::new(need.clone()),
                    have: Box::new(have),
                });
            }
        }
        Ok(())
    }

    fn pay_bill(&mut self, orb: u64, bill: &[(Resource, crate::ratio::Ratio)]) {
        if let Some(held) = peel(&mut self.orbs, orb) {
            for (resource, amount) in bill {
                held.bank(*resource, -amount.clone());
            }
        }
    }

    /// Rank of a science seat on an orb (0 = unseated).
    pub fn science_rank(&self, orb: u64, discipline: ScienceDiscipline) -> u8 {
        self.science_seats
            .iter()
            .find(|(d, o, _)| *d == discipline.as_wire() && *o == orb)
            .map(|(_, _, r)| *r)
            .unwrap_or(0)
    }

    pub fn science_seated(&self, orb: u64, discipline: ScienceDiscipline) -> bool {
        self.science_rank(orb, discipline) >= 1
    }

    /// **Cognitive capacity** (COG-6) — the deepest thought this colony's built
    /// brain can contain, in compose-depth. Leisure buys the time to think; this
    /// bounds the size — an 11-dimensional bridge cannot be contained in a mud
    /// hut. The apparatus is the seated hard-cognitive sciences (Math, Physics,
    /// Research — the Academy/Observatory in all but name), each a paid, built
    /// research institution. Basic invention is universal — every line can
    /// compose a depth-2 tool (knap flint), the minimum a mint requires — but
    /// the depth-3 FRONTIER (the particle accelerator, the 11-dimensional
    /// bridge) is contained only by a built brain: it opens once a hard-
    /// cognitive science (Math, Physics, Research) is seated. A mud hut invents
    /// the simple; the Academy invents the deep. To think a bigger thought,
    /// build a bigger brain. Read into `Need::capacity`; the search composes no
    /// deeper. (COG-8 gives the conductor the drive to build this apparatus.)
    pub fn cognitive_capacity(&self, orb: u64) -> usize {
        use crate::almanac::science::ScienceDiscipline::{Math, Physics, Research};
        let brain = [Math, Physics, Research]
            .into_iter()
            .map(|d| self.science_rank(orb, d))
            .max()
            .unwrap_or(0) as usize;
        // **Basin → capacity** (operator directive 2026-08-20). The seated
        // sciences are one path to a bigger brain; the mind's own GROWTH is the
        // other. Every proven word — minted here or licensed from the roster —
        // condenses the basis, a letter earned by thinking, so a line that has
        // thought enough reaches the depth frontier by COGNITION, not only by
        // building an academy. Grained one rung per `WORDS_PER_RUNG` words. At
        // the frontier a mind's Deepen-γ headroom vanishes (`cognitive_deficit`),
        // so the abundance fold spends its surplus outward (Expand/Synthesise) —
        // the deeper mind builds the richer city, emergent from the fold, no
        // authored tilt. (Bounded at the no-overfit frontier: condensation is the
        // legitimate path past it, but the live consumer — the γ — saturates at
        // 3, so a second, capacity-scaled consumer is what a beyond-3 gradient
        // would need; wiring cognition to the frontier is this slice.)
        const WORDS_PER_RUNG: usize = 8;
        let vocabulary = self.chronicle.word_receipts() as usize;
        let condensed = vocabulary / WORDS_PER_RUNG;
        (2 + brain.max(condensed)).min(3)
    }

    pub fn require_science_seat(
        &self,
        orb: u64,
        discipline: ScienceDiscipline,
        need_rank: u8,
    ) -> Result<(), ActionRefused> {
        let have = self.science_rank(orb, discipline);
        if have < need_rank {
            return Err(ActionRefused::ScienceSeatRequired {
                discipline: discipline.as_wire(),
                need_rank,
                have_rank: have,
            });
        }
        Ok(())
    }

    pub fn clone_count_of(&self, orb: u64) -> u64 {
        self.clone_counts
            .iter()
            .find(|(o, _)| *o == orb)
            .map(|(_, c)| *c)
            .unwrap_or(0)
    }

    /// **The elastic population** (41_PROVISION, ruling R5) — the clone
    /// count is the population that births and dies; habitats are the
    /// fixed housing floor. Population dynamics grow and cull this.
    pub fn population_of(&self, orb: u64) -> u64 {
        self.clone_count_of(orb)
    }

    /// **The SPECIES lifespan `L`** (42_LIFESPAN §1, ruling R7) — derived
    /// from the recovery grain, a 4× spread: the mineral / energy /
    /// lattice classes (geodic-lithovore 2, plasma-field 6, crystal-mind
    /// 10, thermal-banker 12 — beings of stone, energy, and crystal)
    /// live four grains; every other class, biotic and collective, one
    /// grain. No new dial — `REGROWTH_GRAIN` and a factor of four.
    pub fn lifespan_of(&self) -> u64 {
        let class = self.taxon.map(|(class, _)| class).unwrap_or(0);
        let long_lived = matches!(class, 2 | 6 | 10 | 12);
        let grain = crate::almanac::REGROWTH_GRAIN as u64;
        if long_lived {
            grain * 4
        } else {
            grain
        }
    }

    /// **The population ceiling `K_pop`** — the living footprint × clade
    /// habitability that bounds the ration stock (#28), lifted by the
    /// provisioning footprint (`43_DISTRIBUTION` §2): each active granary
    /// sustains more people. With no granary this is exactly the
    /// ecological `ration_capacity_of`, so every #40–42 world is
    /// byte-identical.
    pub fn population_capacity(&self, orb: u64) -> Ratio {
        self.population_ceiling(orb)
    }

    /// The fractional population carry on an orb — the `[0, 1)` remainder
    /// the logistic has grown but not yet promoted to a whole colonist
    /// (COG Slice 2). Zero where a line stands at a whole headcount.
    pub fn pop_accrued_of(&self, orb: u64) -> Ratio {
        self.pop_accrued
            .iter()
            .find(|(o, _)| *o == orb)
            .map(|(_, r)| r.clone())
            .unwrap_or_else(|| ratio(0, 1))
    }

    fn set_pop_accrued(&mut self, orb: u64, remainder: Ratio) {
        self.pop_accrued.retain(|(o, _)| *o != orb);
        if remainder > ratio(0, 1) {
            self.pop_accrued.push((orb, remainder));
        }
    }

    /// **Settle the population** (41_PROVISION §3, tag 167) — the whole-head
    /// entry point: apply an integer net change. A convenience over
    /// [`Self::settle_population_exact`] for the seed, the tests, and any
    /// caller with a whole delta; a whole delta leaves the fractional carry
    /// untouched, so this path is byte-identical to the pre-Slice-2 settle.
    pub fn settle_population(&mut self, orb: u64, delta: i64) {
        self.settle_population_exact(orb, ratio(delta, 1));
    }

    /// **Settle the population, exactly** (41_PROVISION §3, COG Slice 2) —
    /// apply a rational net change (births − deaths) to the exact-rational
    /// population. The current mass is the integer headcount plus its
    /// fractional carry; the net delta folds into that mass, whole colonists
    /// promote to the headcount (receipt tag 167), and the surviving `[0, 1)`
    /// carry is kept (receipt tag 179). Population is a logistic, not a
    /// ledger — the receipts carry the promotion and the remainder, so replay
    /// reconstructs both. A sub-one delta that does not cross a whole colonist
    /// still moves the carry, so a small founding band grows instead of
    /// freezing. The tag-179 receipt is emitted only when the carry actually
    /// changes, so a whole-integer settle stays byte-identical to the old
    /// path (which had no carry).
    pub fn settle_population_exact(&mut self, orb: u64, delta: Ratio) {
        if delta == ratio(0, 1) {
            return;
        }
        let count = self.clone_count_of(orb) as i64;
        let prev_carry = self.pop_accrued_of(orb);
        // The true mass as an exact rational, floored at extinction — a cull
        // can empty a line but never take it negative.
        let total = (ratio(count, 1) + prev_carry.clone() + delta).max(ratio(0, 1));
        let whole: i64 = total.floor().to_integer().try_into().unwrap_or(i64::MAX);
        // The #28 LATTICE guard, on the population carry. The logistic feeds
        // the carry back as the next touch's `p0`, so its denominator
        // compounds every settle (the same recurrence `stock.rs` lattice-
        // floors for the ration — a denominator that squares each tick).
        // Bound it: quantize the `[0, 1)` carry onto the same lattice the
        // ration stock uses (`stock::lattice`), so there is one lattice in
        // the engine, not a new dial. The shave is sub-one-lattice-step of a
        // colonist and bounded; population is a logistic, not a ledger, so
        // this loses no conserved matter.
        let lattice = crate::world::stock::lattice();
        let remainder = ((total - ratio(whole, 1)) * lattice.clone()).floor() / lattice;
        let next = whole.max(0) as u64;
        let promotion = next as i64 - count;
        let stamp = self.clock.now();
        self.set_clone_count(orb, next);
        self.set_pop_accrued(orb, remainder.clone());
        // The whole-colonist promotion receipts on tag 167 (integer
        // headcount); a touch that only moves the carry emits no promotion.
        if promotion != 0 {
            self.chronicle.append(
                crate::dop::ClosureEvent::PopulationSettled { orb_id: orb, delta: promotion },
                stamp,
                vec![],
            );
        }
        // The surviving fractional carry receipts on tag 179 (absolute) —
        // only when it changed, so a whole-integer settle carries no new deed.
        if remainder != prev_carry {
            self.chronicle.append(
                crate::dop::ClosureEvent::PopulationAccrued { orb_id: orb, remainder },
                stamp,
                vec![],
            );
        }
        // 50_WIN_CONDITIONS, operator ruling 2026-08-13: the failure STATE.
        // A line has collapsed only when it is truly extinct — every orb at
        // zero headcount and no fractional carry anywhere (a `0.4`-colonist
        // band is still alive and still growing, so it is not a collapse).
        // Checked here, fires exactly once, idempotent (None -> Some only).
        let extinct = self.clone_counts.iter().all(|(_, count)| *count == 0)
            && self.pop_accrued.is_empty();
        if self.collapsed.is_none() && extinct {
            self.collapsed = Some(stamp);
            self.chronicle.append(
                crate::dop::ClosureEvent::CivilizationCollapsed { orb_id: FIRST_ORB },
                stamp,
                vec![],
            );
        }
    }

    pub fn synth_grade_of(&self, orb: u64) -> u8 {
        self.synth_mods
            .iter()
            .find(|(o, _)| *o == orb)
            .map(|(_, g)| *g)
            .unwrap_or(0)
    }

    pub fn skin_grade_of(&self, orb: u64) -> u8 {
        self.skin_mods
            .iter()
            .find(|(o, _)| *o == orb)
            .map(|(_, g)| *g)
            .unwrap_or(0)
    }

    pub fn rogue_pressure_of(&self, orb: u64) -> u8 {
        self.rogue_pressure
            .iter()
            .find(|(o, _)| *o == orb)
            .map(|(_, p)| *p)
            .unwrap_or(0)
    }

    /// **The quarantine threshold (task #25, ruled 2026-08-12)** — the
    /// bio-pressure at which a world's modification has run past what it
    /// can safely hold. It is the SYNTH-grade ceiling (grades 1..=7),
    /// the same bound `rogue_pressure` and the mod grades already cap at
    /// (`apply_*` clamp with `.min(7)`) — not a chosen dial but the law's
    /// own ceiling read as a threshold.
    pub const QUARANTINE_PRESSURE: u64 = 7;

    /// G2/BIO: has this orb's bio-pressure crossed into quarantine? Both
    /// cascade (BioChemCross) and rogue (SynthRogue) pressure count, each
    /// against the shared ceiling — either maxing out seals the world.
    /// Pure over standing state, so replay reads it identically.
    pub fn is_quarantined(&self, orb: u64) -> bool {
        self.cascade_pressure_of(orb) >= Self::QUARANTINE_PRESSURE
            || u64::from(self.rogue_pressure_of(orb)) >= Self::QUARANTINE_PRESSURE
    }

    /// BIO: does any orb of this world stand quarantined? A quarantine
    /// severs the WORLD's external apertures (the Agent Door to inbound
    /// trade, and melt/trade treaties), so the reading is world-wide.
    pub fn world_quarantined(&self) -> bool {
        self.orbs.iter().any(|held| self.is_quarantined(held.orb_id))
    }

    fn set_science_seat(&mut self, orb: u64, discipline: ScienceDiscipline, rank: u8) {
        let d = discipline.as_wire();
        if let Some(slot) = self
            .science_seats
            .iter_mut()
            .find(|(dd, o, _)| *dd == d && *o == orb)
        {
            slot.2 = rank;
        } else {
            self.science_seats.push((d, orb, rank));
        }
    }

    /// **Seat A science discipline** — pay crystal fee, unlock rank 1 or elevate.
    pub fn seat_science(
        &mut self,
        orb: u64,
        discipline: ScienceDiscipline,
    ) -> Result<u8, ActionRefused> {
        self.seat_science_as(SOVEREIGN, orb, discipline)
    }

    pub fn seat_science_as(
        &mut self,
        actor: u64,
        orb: u64,
        discipline: ScienceDiscipline,
    ) -> Result<u8, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if peek(&self.orbs, orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        let have = self.science_rank(orb, discipline);
        if have >= discipline.max_rank() {
            return Err(ActionRefused::ScienceAlreadySeated {
                discipline: discipline.as_wire(),
                rank: have,
            });
        }
        let next_rank = have.saturating_add(1);
        // Advanced R2: off-planet resourcing required (not payable in local STAMP).
        if seat_requires_off_planet(discipline, next_rank) {
            self.require_off_planet(1)?;
        }
        let met = self.civ_axes().map(|a| a.metabolism).unwrap_or(2);
        let clade = self.taxon.map(|(_, c)| c);
        let mut aff = science_seat_affinity(met, discipline, clade).fee_mult();
        if matches!(discipline, ScienceDiscipline::Research) {
            aff *= crate::almanac::research_affinity_mult(self.taxon_economy_of(orb).mode);
        }
        // Resource-specific bill (local + resonance) — not unit-of-account STAMP.
        let bill = science_seat_bill(discipline, next_rank, met, aff);
        self.afford_bill(orb, &bill)?;
        self.pay_bill(orb, &bill);
        let fee = bill_crystal_portion(&bill); // receipt displays crystal portion (may be 0)
        // If no crystal, still carry a non-zero ratio for kernel: first line amount.
        let fee = if fee > ratio(0, 1) {
            fee
        } else {
            bill.first()
                .map(|(_, a)| a.clone())
                .unwrap_or_else(|| ratio(0, 1))
        };
        self.set_science_seat(orb, discipline, next_rank);
        self.close(
            ClosureEvent::ScienceSeated {
                orb_id: orb,
                discipline: discipline.as_wire(),
                rank: next_rank,
                fee,
            },
            actor,
        );
        Ok(next_rank)
    }

    /// Count required seats missing for a path.
    fn unseated_cross(&self, orb: u64, path: LabPath) -> u8 {
        path.required_seats()
            .iter()
            .filter(|d| !self.science_seated(orb, **d))
            .count() as u8
    }

    /// Pure reading of poles + explosion risk for a path.
    pub fn lab_preview(&self, orb: u64, path: LabPath) -> LabPoleCard {
        let axes = self.civ_axes().unwrap_or(crate::almanac::CivAxes {
            disposition: 1,
            metabolism: 1,
            perception: 1,
        });
        let pulls = self.dark_pulls();
        let dark = pulls
            .iter()
            .fold(ratio(0, 1), |acc, (_, w)| acc + w.clone());
        // Survival proxy: biomass+crystal standing as "health" of the lab.
        let survival = self
            .orb_by_id(orb)
            .map(|o| {
                o.banked_amount(Resource::Biomass) + o.banked_amount(Resource::Crystal)
            })
            .unwrap_or_else(|| ratio(0, 1));
        let unseated = self.unseated_cross(orb, path);
        let order_thin = self.order_balance_thin(orb);
        let envy_or_wrath = pulls.iter().any(|(p, _)| {
            matches!(
                p,
                crate::almanac::DarkPull::Envy | crate::almanac::DarkPull::Wrath
            )
        });
        let math_rank = self.science_rank(orb, ScienceDiscipline::Math);
        lab_pole_card(
            path,
            &dark,
            &survival,
            unseated,
            order_thin,
            axes.perception,
            math_rank,
            envy_or_wrath,
        )
    }

    fn order_balance_thin(&self, orb: u64) -> bool {
        // Prefer thermal / matter balance if present; else crystal famine.
        self.orb_by_id(orb)
            .map(|o| o.banked_amount(Resource::Crystal) < ratio(1, 1))
            .unwrap_or(true)
    }

    /// **Lab create** — dual ± outcomes; may explode.
    ///
    /// Always: known positive + known negative (fee).
    /// If not explode: optional hidden positive (when preview reveals it).
    /// If explode: hidden negative lands; partial reverse of positive.
    pub fn lab_create(
        &mut self,
        orb: u64,
        path: LabPath,
    ) -> Result<LabCreateResult, ActionRefused> {
        self.lab_create_as(SOVEREIGN, orb, path)
    }

    pub fn lab_create_as(
        &mut self,
        actor: u64,
        orb: u64,
        path: LabPath,
    ) -> Result<LabCreateResult, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if peek(&self.orbs, orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        // All required seats must sit (rank ≥ 1).
        for d in path.required_seats() {
            self.require_science_seat(orb, *d, 1)?;
        }
        // Creation paths need rank ≥ 2 on the primary discipline.
        let primary = path.required_seats()[0];
        self.require_science_seat(orb, primary, 2)?;
        // Advanced create paths need off-planet resourcing (aperture/routes).
        if lab_requires_off_planet(path) {
            self.require_off_planet(2)?;
        }

        let card = self.lab_preview(orb, path);
        let met = self.civ_axes().map(|a| a.metabolism).unwrap_or(2);
        let bill = lab_create_bill(path, met);
        self.afford_bill(orb, &bill)?;
        // Known negative: pay resource bill (local + resonance; not STAMP).
        self.pay_bill(orb, &bill);
        let fee = bill_crystal_portion(&bill);
        let fee = if fee > ratio(0, 1) {
            fee
        } else {
            bill.first()
                .map(|(_, a)| a.clone())
                .unwrap_or_else(|| ratio(0, 1))
        };

        let exploded = card.will_explode;
        let kind = path.explosion_kind();
        let mut hidden_pos_landed = false;
        let mut hidden_neg_landed = false;

        if exploded {
            hidden_neg_landed = true;
            self.apply_explosion(actor, orb, path, kind)?;
        } else {
            // Known positive lands.
            self.apply_known_positive(orb, path);
            // Hidden positive when math/empiricist can see it (and thus earn it).
            if card.hidden_pos.is_some() {
                hidden_pos_landed = true;
                self.apply_hidden_positive(orb, path);
            }
        }

        let risk = card.risk.clone();
        self.close(
            ClosureEvent::LabCreated {
                orb_id: orb,
                path: path.as_wire(),
                risk: risk.clone(),
                exploded,
                explosion_kind: if exploded { kind.as_wire() } else { 0 },
                hidden_pos: hidden_pos_landed,
                hidden_neg: hidden_neg_landed,
                fee: fee.clone(),
            },
            actor,
        );

        Ok(LabCreateResult {
            path,
            risk,
            exploded,
            explosion_kind: if exploded { Some(kind) } else { None },
            known_pos: card.known_pos,
            known_neg: card.known_neg,
            hidden_pos_landed,
            hidden_neg_landed,
            hidden_pos: card.hidden_pos,
            hidden_neg: card.hidden_neg,
        })
    }

    fn apply_known_positive(&mut self, orb: u64, path: LabPath) {
        match path {
            LabPath::ChemWork => {
                // Catalog already has its own deed; here a thin research
                // practice signal is minted: bump detection-adjacent via
                // nothing free — only record path success is the receipt.
            }
            LabPath::BioChem => {
                // Small biomass gift is free mint — refuse that. Instead
                // convert crystal already paid into nothing extra; the
                // positive is "workable hybrid flag" via receipt only.
            }
            LabPath::SynthMod => {
                let g = self.synth_grade_of(orb).saturating_add(1).min(7);
                self.set_synth_grade(orb, g);
            }
            LabPath::CloneWork => {
                let c = self.clone_count_of(orb).saturating_add(1);
                self.set_clone_count(orb, c);
            }
            LabPath::SkinWork => {
                let g = self.skin_grade_of(orb).saturating_add(1).min(7);
                self.set_skin_grade(orb, g);
            }
        }
    }

    fn apply_hidden_positive(&mut self, orb: u64, path: LabPath) {
        match path {
            LabPath::SynthMod => {
                let g = self.synth_grade_of(orb).saturating_add(1).min(7);
                self.set_synth_grade(orb, g);
            }
            LabPath::CloneWork => {
                // Escort pool: second clone (still paid by fee — no free matter).
                let c = self.clone_count_of(orb).saturating_add(1);
                self.set_clone_count(orb, c);
            }
            LabPath::SkinWork => {
                let g = self.skin_grade_of(orb).saturating_add(1).min(7);
                self.set_skin_grade(orb, g);
            }
            LabPath::ChemWork | LabPath::BioChem => {
                // Hidden map is informational (receipt flag only).
            }
        }
    }

    fn apply_explosion(
        &mut self,
        actor: u64,
        orb: u64,
        path: LabPath,
        kind: ExplosionKind,
    ) -> Result<(), ActionRefused> {
        let slash = ZoneKind::Resonator.yield_per_tick().1; // another eighth
        match kind {
            ExplosionKind::ChemRunaway => {
                if let Some(held) = peel(&mut self.orbs, orb) {
                    let have = held.banked_amount(Resource::Crystal);
                    let take = if have > slash { slash.clone() } else { have };
                    held.bank(Resource::Crystal, -take.clone());
                    self.close(
                        ClosureEvent::ScienceExploded {
                            orb_id: orb,
                            path: path.as_wire(),
                            kind: kind.as_wire(),
                            resource: Resource::Crystal as u8,
                            slash: take,
                        },
                        actor,
                    );
                }
            }
            ExplosionKind::BioChemCross => {
                if let Some(held) = peel(&mut self.orbs, orb) {
                    let have = held.banked_amount(Resource::Biomass);
                    let take = if have > slash { slash.clone() } else { have };
                    held.bank(Resource::Biomass, -take.clone());
                    self.close(
                        ClosureEvent::ScienceExploded {
                            orb_id: orb,
                            path: path.as_wire(),
                            kind: kind.as_wire(),
                            resource: Resource::Biomass as u8,
                            slash: take,
                        },
                        actor,
                    );
                }
                // Cascade seed: bump cascade pressure.
                let p = self
                    .cascade_pressures
                    .iter()
                    .find(|(o, _)| *o == orb)
                    .map(|(_, p)| *p)
                    .unwrap_or(0)
                    .saturating_add(1);
                self.cascade_pressures.retain(|(o, _)| *o != orb);
                self.cascade_pressures.push((orb, p));
            }
            ExplosionKind::SynthRogue => {
                let p = self.rogue_pressure_of(orb).saturating_add(1).min(7);
                self.set_rogue_pressure(orb, p);
                // Downgrade synth if any.
                let g = self.synth_grade_of(orb).saturating_sub(1);
                self.set_synth_grade(orb, g);
                self.close(
                    ClosureEvent::ScienceExploded {
                        orb_id: orb,
                        path: path.as_wire(),
                        kind: kind.as_wire(),
                        resource: Resource::Crystal as u8,
                        slash: ratio(0, 1),
                    },
                    actor,
                );
                self.close(
                    ClosureEvent::RogueFactionStruck {
                        orb_id: orb,
                        pressure: p,
                    },
                    actor,
                );
            }
            ExplosionKind::CloneRevolt => {
                let c = self.clone_count_of(orb).saturating_sub(1);
                self.set_clone_count(orb, c);
                // Strike novelty want (identity crisis).
                if !self.wants.iter().any(|(o, k)| *o == orb && *k == 3) {
                    self.wants.push((orb, 3));
                    self.close(
                        ClosureEvent::WantStruck {
                            orb_id: orb,
                            kind: 3,
                            reading: ratio(1, 1),
                        },
                        actor,
                    );
                }
                self.close(
                    ClosureEvent::ScienceExploded {
                        orb_id: orb,
                        path: path.as_wire(),
                        kind: kind.as_wire(),
                        resource: Resource::Biomass as u8,
                        slash: slash.clone(),
                    },
                    actor,
                );
            }
            ExplosionKind::SkinUnmask => {
                let g = self.skin_grade_of(orb).saturating_sub(1);
                self.set_skin_grade(orb, g);
                // Reverse spy advantage: mark detection exposure.
                if let Some(slot) = self.spy_marks.iter_mut().find(|(o, _)| *o == orb) {
                    slot.1 = slot.1.saturating_add(1);
                } else {
                    self.spy_marks.push((orb, 1));
                }
                self.close(
                    ClosureEvent::ScienceExploded {
                        orb_id: orb,
                        path: path.as_wire(),
                        kind: kind.as_wire(),
                        resource: Resource::Crystal as u8,
                        slash: slash.clone(),
                    },
                    actor,
                );
            }
        }
        Ok(())
    }

    fn set_clone_count(&mut self, orb: u64, count: u64) {
        self.clone_counts.retain(|(o, _)| *o != orb);
        if count > 0 {
            self.clone_counts.push((orb, count));
        }
    }

    fn set_synth_grade(&mut self, orb: u64, grade: u8) {
        self.synth_mods.retain(|(o, _)| *o != orb);
        if grade > 0 {
            self.synth_mods.push((orb, grade));
        }
    }

    fn set_skin_grade(&mut self, orb: u64, grade: u8) {
        self.skin_mods.retain(|(o, _)| *o != orb);
        if grade > 0 {
            self.skin_mods.push((orb, grade));
        }
    }

    fn set_rogue_pressure(&mut self, orb: u64, pressure: u8) {
        self.rogue_pressure.retain(|(o, _)| *o != orb);
        if pressure > 0 {
            self.rogue_pressure.push((orb, pressure));
        }
    }

    fn set_cascade_pressure(&mut self, orb: u64, pressure: u64) {
        self.cascade_pressures.retain(|(o, _)| *o != orb);
        if pressure > 0 {
            self.cascade_pressures.push((orb, pressure));
        }
    }

    /// **BIO quarantine relief (task #30, ruling R3, tag 168)** — a
    /// quarantined orb recovers one step at the touch: both bio-pressure
    /// axes (cascade and rogue) ebb by one, the mirror of the `+1` each
    /// explosion accrued, and the recovery is receipted. When the maxed
    /// axis falls below `QUARANTINE_PRESSURE` the world's apertures
    /// reopen. Not quarantined → no ebb → no deed → byte-identical. (The
    /// step is the accrual's own unit, not a dial; a slower recovery arc
    /// is a rate the operator can rule later.)
    pub fn ebb_pressure(&mut self, orb: u64) {
        if !self.is_quarantined(orb) {
            return;
        }
        let cascade = self.cascade_pressure_of(orb).saturating_sub(1);
        let rogue = self.rogue_pressure_of(orb).saturating_sub(1);
        self.set_cascade_pressure(orb, cascade);
        self.set_rogue_pressure(orb, rogue);
        let stamp = self.clock.now();
        self.chronicle.append(
            crate::dop::ClosureEvent::PressureEbbed { orb_id: orb },
            stamp,
            vec![],
        );
    }

    /// Board reading for CLI science index.
    pub fn science_lab_board(&self, orb: u64) -> ScienceLabBoard {
        let met = self.civ_axes().map(|a| a.metabolism).unwrap_or(2);
        let clade = self.taxon.map(|(_, c)| c);
        let seats: Vec<(ScienceDiscipline, u8, crate::ratio::Ratio, SeatAffinity)> = [
            ScienceDiscipline::Chem,
            ScienceDiscipline::Biology,
            ScienceDiscipline::Physics,
            ScienceDiscipline::Math,
            ScienceDiscipline::Research,
            ScienceDiscipline::Synthetic,
            ScienceDiscipline::Clone,
            ScienceDiscipline::Skin,
        ]
        .into_iter()
        .map(|d| {
            let rank = self.science_rank(orb, d);
            let aff = science_seat_affinity(met, d, clade);
            let next = if rank >= d.max_rank() {
                ratio(0, 1)
            } else {
                let eighth = ZoneKind::Resonator.yield_per_tick().1;
                eighth
                    * ratio(d.seat_weight(), 1)
                    * ratio(i64::from(rank + 1), 1)
                    * ratio(aff.fee_mult(), 1)
            };
            (d, rank, next, aff)
        })
        .collect();
        ScienceLabBoard {
            seats,
            clones: self.clone_count_of(orb),
            synth_grade: self.synth_grade_of(orb),
            skin_grade: self.skin_grade_of(orb),
            rogue: self.rogue_pressure_of(orb),
            metabolism: met,
            clade,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LabCreateResult {
    pub path: LabPath,
    pub risk: crate::ratio::Ratio,
    pub exploded: bool,
    pub explosion_kind: Option<ExplosionKind>,
    pub known_pos: (&'static str, &'static str),
    pub known_neg: (&'static str, &'static str),
    pub hidden_pos_landed: bool,
    pub hidden_neg_landed: bool,
    pub hidden_pos: Option<(&'static str, &'static str)>,
    pub hidden_neg: Option<(&'static str, &'static str)>,
}

#[derive(Debug, Clone)]
pub struct ScienceLabBoard {
    /// (discipline, rank, next_fee, affinity) — affinity is metabolism/clade coloured.
    pub seats: Vec<(ScienceDiscipline, u8, crate::ratio::Ratio, SeatAffinity)>,
    pub clones: u64,
    pub synth_grade: u8,
    pub skin_grade: u8,
    pub rogue: u8,
    pub metabolism: u8,
    pub clade: Option<u16>,
}
