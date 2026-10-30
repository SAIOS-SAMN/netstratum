//! Chemistry — what a world's matter makes when it is put together.
//!
//! **ELEMENT-CHEMISTRY (#37): the almanac's first consumer.** Until
//! this deed the periodic table was a complete, pinned derivation that
//! nothing in the world ever read — five axes deciding nothing, which
//! is exactly the shape law 8 warns about. The compounding deed is
//! what makes the table load-bearing: banked elements bond into gems
//! by the lane law, so where an element sits finally decides what a
//! civilization can make.
//!
//! **Science path** — the bench is not only a list: it scores bonds for
//! *this* sealed civilization (survival utility), names the dark-7
//! gravity wells that pull toward immediate need, and supports
//! isomorphic gem union + chemistry catalog (chemical creation as
//! known recipe, never free matter).
//!
//! The door / fact split (29_CONSTRUCTION):
//!
//!   Door   whether the matter bonds at all — the shells agree, the
//!          phases agree, the binds saturate. A property of the set,
//!          decidable without the chain, so it dies at
//!          [`crate::almanac::Compoundable`] and cannot be re-asked.
//!   Facts  locality, pressure and workability. Every one reads the
//!          world — the star's depth, the strata opened here, this
//!          orb's temperature — so every one stays at the verb
//!          forever: a set that cannot bond on this rock bonds on the
//!          next one.

use super::*;
use crate::almanac::{CivAxes, DarkPull};

/// One reading of the bench: the members that bond, the gem they name,
/// the knowledge won, and what (if anything) stands in the way here.
pub type BenchRead = (Vec<[u8; 32]>, [u8; 32], u16, Option<ActionRefused>);

/// Science-ranked bench row: bond set + gem + knowledge + standing
/// refusal + (survival, dark, net) utilities.
pub type ScienceRead = (
    Vec<[u8; 32]>,
    [u8; 32],
    u16,
    Option<ActionRefused>,
    Ratio,
    Ratio,
    Ratio,
);

/// A plane of the table — the `(shell, phase)` matter must share before
/// it can bond at all. Bonding never crosses one, so a plane is the
/// unit the bench looks at.
type Plane = (u8, u8);

/// One element as a plane holds it: its identity, and the lane valence
/// the saturation walk needs — read once at the shear rather than
/// re-read for every set the element might appear in.
type Seated = ([u8; 32], u8);

impl World {
    /// **REFINE-1** — convert banked raw element into its refined form
    /// through an ElementRefinery on the orb. Collect scope: moves the
    /// element ledger. Rates and identity re-derive from
    /// [`crate::dop::refine_recipe`] (call-not-copy).
    pub fn refine(&mut self, orb: u64, raw: [u8; 32]) -> Result<[u8; 32], ActionRefused> {
        self.refine_as(SOVEREIGN, orb, raw)
    }

    /// The contained form of refine.
    pub fn refine_as(
        &mut self,
        actor: u64,
        orb: u64,
        raw: [u8; 32],
    ) -> Result<[u8; 32], ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        let Some(held) = self.orb_by_id(orb) else {
            return Err(ActionRefused::NoSuchOrb { orb });
        };
        let stands = held
            .zones
            .iter()
            .any(|zone| zone.kind == ZoneKind::ElementRefinery);
        if !stands {
            return Err(ActionRefused::NoRefineryStanding { orb });
        }
        let Some(class) = self.star else {
            return Err(ActionRefused::NoStarCommissioned);
        };
        // CIV-REACH-1: refining deep lines needs the same ceiling as assay.
        let line = crate::almanac::periodic_of(&raw).line;
        let reach = self.civ_reach();
        if line > reach {
            return Err(ActionRefused::CivReachShort {
                need: line,
                have: reach,
            });
        }
        if !crate::almanac::local_to(&raw, class) {
            return Err(ActionRefused::ElementNotLocal { element: raw });
        }
        let (mut spend, refined, yielded) = crate::dop::refine_recipe(&raw, class);
        let refined_line = crate::almanac::periodic_of(&refined).line;
        if refined_line > reach {
            return Err(ActionRefused::CivReachShort {
                need: refined_line,
                have: reach,
            });
        }
        // O6.2 T5 Crucible — office advantage (not a gate): half the
        // spend. Derive half as (1+1); no new ratio dial (ratchet 112).
        // Q-T2 coded default: T3 may still refine; Crucible only eases.
        let crucible = self
            .stations
            .iter()
            .any(|(_, s)| *s == Station::Crucible.as_wire());
        if crucible {
            spend /= ratio(1, 1) + ratio(1, 1);
        }
        // #136 D3 — refine declares its grain (1/1024): the Crucible halving can push
        // the spend off the grain, and the yield is a recipe rational; both smelt to
        // the element grain before the affordability check, order arms, receipt, and
        // banking read them, so the stock stays on-grain and replay (which decodes
        // these fields) is byte-identical.
        let spend = smelt_element(&spend);
        let yielded = smelt_element(&yielded);
        let have = held.element_amount(&raw);
        if have < spend {
            return Err(ActionRefused::CannotAffordElement {
                element: raw,
                need: Box::new(spend),
                have: Box::new(have),
            });
        }
        // 34_MATTER_BALANCE: order-work gate (entropy ∥ syntropy).
        self.require_order_balance(orb)?;
        let (ent, syn) = self.order_arms_refine(orb, &spend, &yielded);
        let held = peel(&mut self.orbs, orb).expect("judged above");
        held.bank_element(raw, -spend.clone());
        held.bank_element(refined, yielded.clone());
        self.close(
            ClosureEvent::ElementRefined {
                orb_id: orb,
                raw,
                refined,
                spent: spend,
                yielded,
            },
            actor,
        );
        self.post_order_worked(actor, orb, balance::ORDER_REFINE, ent, syn);
        Ok(refined)
    }

    /// Compound on the genesis orb as the sovereign.
    pub fn compound(&mut self, members: &[[u8; 32]]) -> Result<[u8; 32], ActionRefused> {
        self.compound_as(SOVEREIGN, FIRST_ORB, members)
    }

    /// The contained form. Collect scope: compounding moves banked
    /// element value, exactly as the assay does.
    ///
    /// Refusal-pure — every check completes before anything moves, so
    /// a refused compounding leaves no receipt and spends no matter.
    pub fn compound_as(
        &mut self,
        actor: u64,
        orb: u64,
        members: &[[u8; 32]],
    ) -> Result<[u8; 32], ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        if self.orb_by_id(orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        if self.star.is_none() {
            return Err(ActionRefused::NoStarCommissioned);
        }
        // The door: does this matter bond? Past here the set is proven,
        // and its gem and knowledge are total.
        let set = crate::almanac::Compoundable::new(members)
            .ok_or(ActionRefused::MatterDoesNotBond)?;

        // The facts — the place, then the ledger. One home, so the
        // bench cannot promise what the deed would refuse.
        if let Some(refusal) = self.bond_refusal(orb, set.members()) {
            return Err(refusal);
        }

        // The recipe — one home, the same the gate re-derives from.
        let (spends, yielded) = crate::almanac::recipe_of(&set);
        // #136 D3 — gem-compound declares its grain (1/1024): every spend and the
        // yield smelt to the element grain before the order arms, receipt, and banking
        // read them, so the element stocks stay on-grain and replay (which decodes
        // these fields) is byte-identical.
        let spends: Vec<([u8; 32], Ratio)> =
            spends.iter().map(|(element, spend)| (*element, smelt_element(spend))).collect();
        let yielded = smelt_element(&yielded);

        // 34_MATTER_BALANCE: alchemy is order-work.
        self.require_order_balance(orb)?;
        let (ent, syn) = self.order_arms_compound(&yielded);

        // Nothing above moved anything. The deed:
        let gem = set.gem();
        let knowledge = set.knowledge();
        // Science residual: if dark gravity outranks survival, receipt it.
        let (survival, dark, _) =
            self.score_bond(set.members(), &yielded, knowledge);
        let dominant = self.dominant_dark_pull();
        let held = peel(&mut self.orbs, orb).expect("judged above");
        for (element, spend) in &spends {
            held.bank_element(*element, -spend.clone());
        }
        held.bank_element(gem, yielded);
        self.close(
            ClosureEvent::GemCompounded {
                orb_id: orb,
                members: set.members().to_vec(),
                gem,
            },
            actor,
        );
        if dark > survival {
            if let Some((pull, weight)) = dominant {
                self.close(
                    ClosureEvent::DarkPullStruck {
                        orb_id: orb,
                        pull: pull as u8,
                        weight,
                        dark_score: dark,
                    },
                    actor,
                );
            }
        }
        self.post_order_worked(actor, orb, balance::ORDER_COMPOUND, ent, syn);
        Ok(gem)
    }

    /// **Whether this bond stands right NOW** — everything past the
    /// door, in one home: the place (does the sky carry it, has the
    /// ground the pressure, is the orb at a workable heat) and then
    /// the ledger (is there enough to spend). `None` means the deed
    /// would go through.
    ///
    /// The bench reads this too, so a set it marks ready is a set the
    /// verb accepts — measured in live play, the two drifting apart is
    /// what turns a bench into a trap.
    pub fn bond_refusal(&self, orb: u64, members: &[[u8; 32]]) -> Option<ActionRefused> {
        if let Some(refusal) = self.place_refusal(orb, members) {
            return Some(refusal);
        }
        let set = crate::almanac::Compoundable::new(members)?;
        let (spends, _) = crate::almanac::recipe_of(&set);
        let held = peek(&self.orbs, orb)?;
        for (element, spend) in &spends {
            let have = held.element_amount(element);
            if have < *spend {
                return Some(ActionRefused::CannotAffordElement {
                    element: *element,
                    need: Box::new(spend.clone()),
                    have: Box::new(have),
                });
            }
        }
        None
    }

    /// **Whether this place can work this MATTER** — the three chain
    /// facts, in one home so the deed and the bench cannot disagree.
    /// `None` means the place stands; otherwise the first refusal, on
    /// the first member that earns it.
    ///
    /// Pressure reads the deepest stratum this orb has opened:
    /// prospecting is what brings a system's pressure to bear, so a
    /// world that has only scratched the surface cannot release deep
    /// matter however much of it it holds.
    pub fn place_refusal(&self, orb: u64, members: &[[u8; 32]]) -> Option<ActionRefused> {
        // A starless world is not a place that works matter — said
        // here rather than left to `?`, which would have read as "the
        // place stands".
        let Some(class) = self.star else {
            return Some(ActionRefused::NoStarCommissioned);
        };
        let deepest = self
            .prospects
            .iter()
            .filter(|vein| vein.orb == orb)
            .map(|vein| vein.stratum)
            .max()
            .unwrap_or(0);
        let heat = self.orb_heat(orb);
        for member in members {
            if !crate::almanac::local_to(member, class) {
                return Some(ActionRefused::ElementNotLocal { element: *member });
            }
            if !crate::almanac::refines(member, deepest, class) {
                return Some(ActionRefused::PressureUnreached { element: *member });
            }
            match &heat {
                Some(t_eq4) if crate::almanac::workable_at(member, t_eq4) => {}
                // An unsited orb has no temperature, so nothing is
                // workable on it — matter needs a place.
                _ => return Some(ActionRefused::NotWorkableHere { element: *member }),
            }
        }
        None
    }

    /// This orb's equilibrium heat, if it has a place at all. The
    /// grained reading (`almanac::grained_t_eq4`), because workability
    /// is an almanac question and the almanac puts orbits on the grain
    /// so exact arithmetic stays cheap.
    pub fn orb_heat(&self, orb: u64) -> Option<Ratio> {
        let class = self.star?;
        let site = self.orb_by_id(orb)?.thermal.as_ref()?;
        Some(crate::almanac::grained_t_eq4(
            &class.luminosity(),
            site.albedo.value(),
            site.orbit_r.value(),
        ))
    }

    /// **The bench, with its standing** — every set this orb's matter
    /// would bond, each carrying what (if anything) this place has to
    /// say about it.
    ///
    /// A bench that listed only what bonds is a trap: measured in live
    /// play, a world's own drawn matter routinely offers recipes its
    /// orb cannot work, because a compound shares one phase and an orb
    /// sits in one thermal region. Reading the refusal here — from the
    /// same [`Self::place_refusal`] the deed calls, so the two cannot
    /// drift — turns that from a dead end into a map: it says which
    /// rock to build the next colony on.
    pub fn bench_standing(&self, orb: u64) -> Vec<BenchRead> {
        self.bench(orb)
            .into_iter()
            .map(|(members, gem, knowledge)| {
                let standing = self.bond_refusal(orb, &members);
                (members, gem, knowledge, standing)
            })
            .collect()
    }

    /// **The bench** — every set of this orb's banked elements that
    /// would bond, as (members, gem, knowledge). Derived at call and
    /// never stored: it is a reading of the ledger, not state.
    ///
    /// **Sheared onto the PLANES, never swept** — the standing ruling
    /// that [`peel`] carries (operator, 2026-07-30): *no cartesian
    /// find-scan repetition; peels and shears where there were
    /// iterations.*
    ///
    /// A first cut of this walked every subset of the ledger up to five
    /// bodies and asked the door about each — some 55,000 candidate
    /// sets for a world holding 24 elements, every one of them
    /// re-reading each member's seat in the table. It also had to sort
    /// its output afterwards, which is the tell: a frontier walk has no
    /// order of its own, so determinism had to be bolted on.
    ///
    /// The bonding law already partitions the search and the sweep was
    /// ignoring it. A set bonds only where every member shares the
    /// SHELL and the phase, so `(shell, phase)` are the planes matter
    /// can bond on, and nothing lying between two planes was ever a
    /// candidate. So: one pass deposits the ledger onto its planes (the
    /// crystal-shear idiom), and each plane is then read on its own
    /// terms — terms which are constants there rather than things to
    /// search for, because the shell fixes how many bond at once and
    /// the saturation identity fixes what their binds must sum to.
    ///
    /// Order is deterministic by construction — planes in table order,
    /// members in ledger order — so nothing is sorted afterwards.
    ///
    /// The reading is unchanged: `compounds` demands one shell, one
    /// phase, and a count equal to the arity, so every set the sweep
    /// could find lies inside exactly one plane at exactly that size.
    /// The door remains the one authority on what bonds; the shear only
    /// decides where to look.
    pub fn bench(&self, orb: u64) -> Vec<(Vec<[u8; 32]>, [u8; 32], u16)> {
        let Some(held) = self.orb_by_id(orb) else {
            return Vec::new();
        };
        // The shear — one pass over the ledger. Each element is
        // deposited on its plane carrying the single number the walk
        // needs (its lane's valence), read once here instead of
        // re-read for every candidate it might appear in.
        let mut sheet: Vec<(Plane, Vec<Seated>)> = Vec::new();
        for (element, _) in &held.elements {
            let seat = crate::almanac::periodic_of(element);
            let plane = (seat.shell, seat.phase);
            match sheet.iter_mut().find(|(held, _)| *held == plane) {
                Some((_, members)) => members.push((*element, seat.binds)),
                None => sheet.push((plane, vec![(*element, seat.binds)])),
            }
        }
        // The sheet is bounded by the table, not by the ledger: there
        // are only `SHELLS × PHASES` planes however much matter a world
        // banks.
        sheet.sort_by_key(|(plane, _)| *plane);

        let mut found = Vec::new();
        for (plane, members) in &sheet {
            // The plane's own constant: its shell fixes the arity, so
            // a plane that cannot seat that many holds no set at all
            // and is never walked.
            let arity = usize::from(plane.0) + 1;
            if members.len() < arity {
                continue;
            }
            for picked in saturating_sets(members, arity) {
                if let Some(set) = crate::almanac::Compoundable::new(&picked) {
                    found.push((set.members().to_vec(), set.gem(), set.knowledge()));
                }
            }
        }
        found
    }

    // ── science path ──────────────────────────────────────────────

    /// Sealed civ axes for utility (posture doctrine overlays disposition
    /// / perception when reoriented; metabolism stays anatomy).
    pub fn civ_axes(&self) -> Option<CivAxes> {
        let g = self.genesis.as_ref()?;
        Some(CivAxes {
            disposition: self.posture_disposition.unwrap_or(g.disposition),
            metabolism: g.metabolism,
            perception: self.posture_perception.unwrap_or(g.perception),
        })
    }

    /// Active dark-7 pulls with weights (0 = silent). Derived from
    /// wants, famine, war residual, intel, surplus, ease — never stored.
    pub fn dark_pulls(&self) -> Vec<(DarkPull, Ratio)> {
        let mut out = Vec::new();
        let one = ratio(1, 1);
        let half = one.clone() / (one.clone() + one.clone());
        // Green — growth greed: crystal famine pressure or cascade.
        let crystal_thin = self.orb().is_some_and(|o| {
            o.banked_amount(Resource::Crystal) < ratio(1, 1)
        });
        let cascade = self.cascade_pressures.iter().any(|(_, p)| *p > 0);
        if crystal_thin || cascade {
            out.push((DarkPull::Green, if cascade { one.clone() } else { half.clone() }));
        }
        // Vanity — surplus display, the epicurean trait read generically
        // (a line minted from an epicurean displays too; `44_LINEAGES`).
        if self.disposition_carries(crate::dop::DispositionTrait::VanityDisplay) {
            let rich = self.orb().is_some_and(|o| o.stamp_amount() > ratio(1, 1));
            if rich {
                out.push((DarkPull::Vanity, half.clone()));
            }
        }
        // Envy — intel stash or spy marks live.
        if !self.intel_hashes.is_empty() || !self.spy_marks.is_empty() {
            out.push((DarkPull::Envy, one.clone()));
        }
        // Wrath — any war treaty residual (at_war books).
        if self.treaties.iter().any(|t| t.kind == crate::treaty::TreatyKind::War) {
            out.push((DarkPull::Wrath, one.clone()));
        }
        // Sloth — no instruments forged, no practices.
        if self.instruments.is_empty() && self.practices.is_empty() {
            out.push((DarkPull::Sloth, half.clone()));
        }
        // Gluttony — large free banks (iron+biomass+crystal).
        if let Some(o) = self.orb() {
            let mass = o.banked_amount(Resource::Iron)
                + o.banked_amount(Resource::Biomass)
                + o.banked_amount(Resource::Crystal);
            if mass > ratio(1, 1) + ratio(1, 1) + ratio(1, 1) + ratio(1, 1) {
                out.push((DarkPull::Gluttony, half.clone()));
            }
        }
        // Pride — novelty want struck (starved mind chases depth).
        if self.wants.iter().any(|(_, k)| *k == 3) {
            out.push((DarkPull::Pride, one));
        }
        out
    }

    fn dominant_dark_pull(&self) -> Option<(DarkPull, Ratio)> {
        self.dark_pulls()
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1))
    }

    /// Score a candidate bond under this world's civ + dark gravity.
    pub fn score_bond(
        &self,
        members: &[[u8; 32]],
        yielded: &Ratio,
        knowledge: u16,
    ) -> (Ratio, Ratio, Ratio) {
        let axes = self.civ_axes().unwrap_or(CivAxes {
            disposition: 1,
            metabolism: 1,
            perception: 1,
        });
        let pulls = self.dark_pulls();
        crate::almanac::net_utility(axes, members, yielded, knowledge, &pulls)
    }

    /// Bench ranked by net science utility (highest first). Pure read.
    pub fn bench_science(&self, orb: u64) -> Vec<ScienceRead> {
        let mut rows: Vec<ScienceRead> = self
            .bench_standing(orb)
            .into_iter()
            .map(|(members, gem, knowledge, standing)| {
                let set = crate::almanac::Compoundable::new(&members);
                let yielded = set
                    .as_ref()
                    .map(|s| crate::almanac::recipe_of(s).1)
                    .unwrap_or_else(|| ratio(0, 1));
                let (survival, dark, net) = self.score_bond(&members, &yielded, knowledge);
                (members, gem, knowledge, standing, survival, dark, net)
            })
            .collect();
        rows.sort_by(|a, b| b.6.cmp(&a.6));
        rows
    }

    /// Chemistry catalog (known gems).
    pub fn chem_catalog(&self) -> &[[u8; 32]] {
        &self.chem_catalog
    }

    /// **Chemical creation (catalog)** — seal a banked gem as known
    /// chemistry. No free matter; spends resonator-eighth crystal.
    /// Empiricist novelty want may relieve on success (mind fed).
    pub fn chem_catalog_gem(
        &mut self,
        orb: u64,
        gem: [u8; 32],
    ) -> Result<u16, ActionRefused> {
        self.chem_catalog_gem_as(SOVEREIGN, orb, gem)
    }

    pub fn chem_catalog_gem_as(
        &mut self,
        actor: u64,
        orb: u64,
        gem: [u8; 32],
    ) -> Result<u16, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if peek(&self.orbs, orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        // Lab seat gate — chemistry catalog is a paid capability.
        self.require_science_seat(orb, crate::almanac::ScienceDiscipline::Chem, 1)?;
        if self.chem_catalog.contains(&gem) {
            return Err(ActionRefused::ChemAlreadyCataloged { gem });
        }
        let held = peek(&self.orbs, orb).expect("checked");
        if held.element_amount(&gem) <= ratio(0, 1) {
            return Err(ActionRefused::NoSuchGem { gem });
        }
        let fee = ZoneKind::Resonator.yield_per_tick().1;
        let have = held.banked_amount(Resource::Crystal);
        if have < fee {
            return Err(ActionRefused::CannotAfford {
                resource: Resource::Crystal,
                need: Box::new(fee),
                have: Box::new(have),
            });
        }
        let knowledge = u16::from(crate::almanac::periodic_of(&gem).line);
        let axes = self.civ_axes().unwrap_or(CivAxes {
            disposition: 1,
            metabolism: 1,
            perception: 1,
        });
        let survival = crate::almanac::survival_utility(axes, &[gem], &ratio(1, 1));
        let held = peel(&mut self.orbs, orb).expect("checked");
        held.bank(Resource::Crystal, -fee);
        self.chem_catalog.push(gem);
        self.close(
            ClosureEvent::ChemCataloged {
                orb_id: orb,
                gem,
                knowledge,
                survival,
            },
            actor,
        );
        // Novelty want relief when empiricist mind was starving.
        if self.wants.iter().any(|(o, k)| *o == orb && *k == 3) {
            self.wants.retain(|(o, k)| !(*o == orb && *k == 3));
            self.close(ClosureEvent::WantRelieved { orb_id: orb, kind: 3 }, actor);
        }
        Ok(knowledge)
    }

    /// **Isomorphic union** — fuse two banked gems of the same shell/phase
    /// family into a product (chemical creation of a higher body).
    /// Spends 1 of each, yields 2 of product (mass-exact).
    pub fn iso_union(
        &mut self,
        orb: u64,
        a: [u8; 32],
        b: [u8; 32],
    ) -> Result<[u8; 32], ActionRefused> {
        self.iso_union_as(SOVEREIGN, orb, a, b)
    }

    pub fn iso_union_as(
        &mut self,
        actor: u64,
        orb: u64,
        a: [u8; 32],
        b: [u8; 32],
    ) -> Result<[u8; 32], ActionRefused> {
        self.permitted(actor, Scope::Collect)?;
        if peek(&self.orbs, orb).is_none() {
            return Err(ActionRefused::NoSuchOrb { orb });
        }
        if self.star.is_none() {
            return Err(ActionRefused::NoStarCommissioned);
        }
        // Lab seat gate — iso-union is chemistry creation.
        self.require_science_seat(orb, crate::almanac::ScienceDiscipline::Chem, 1)?;
        if !crate::almanac::isomorphic(&a, &b) || a == b {
            return Err(ActionRefused::NotIsomorphic);
        }
        let (spend_each, yielded, product) = crate::almanac::iso_union_recipe(&a, &b)
            .ok_or(ActionRefused::NotIsomorphic)?;
        // #136 D3 — iso-union declares its grain (1/1024): the per-side spend and the
        // yield smelt to the element grain before the affordability check, order arms,
        // receipt, and banking read them, so the element stocks stay on-grain and
        // replay (which decodes these fields) is byte-identical.
        let spend_each = smelt_element(&spend_each);
        let yielded = smelt_element(&yielded);
        let knowledge = crate::almanac::iso_union_knowledge(&a, &b).unwrap_or(0);
        let held = peek(&self.orbs, orb).expect("checked");
        for id in [&a, &b] {
            let have = held.element_amount(id);
            if have < spend_each {
                return Err(ActionRefused::CannotAffordElement {
                    element: *id,
                    need: Box::new(spend_each.clone()),
                    have: Box::new(have),
                });
            }
        }
        self.require_order_balance(orb)?;
        let (ent, syn) = self.order_arms_compound(&yielded);
        let (survival, dark, _) = self.score_bond(&[a, b], &yielded, knowledge);
        let dominant = self.dominant_dark_pull();
        let held = peel(&mut self.orbs, orb).expect("checked");
        held.bank_element(a, -spend_each.clone());
        held.bank_element(b, -spend_each);
        held.bank_element(product, yielded);
        self.close(
            ClosureEvent::IsoUnionStruck {
                orb_id: orb,
                a,
                b,
                product,
                knowledge,
            },
            actor,
        );
        if dark > survival {
            if let Some((pull, weight)) = dominant {
                self.close(
                    ClosureEvent::DarkPullStruck {
                        orb_id: orb,
                        pull: pull as u8,
                        weight,
                        dark_score: dark,
                    },
                    actor,
                );
            }
        }
        self.post_order_worked(actor, orb, balance::ORDER_COMPOUND, ent, syn);
        Ok(product)
    }
}

/// **One plane's saturating sets** — the deterministic walk inside a
/// plane, where the arity and the binds target are the plane's own
/// constants rather than things to search for.
///
/// Members are taken in ledger order and never revisited, so the
/// reading needs no sort. The comparison is the compounding law's own
/// doubled form (`Σ binds · 2 == arity · (LANES/2 + 1)`), so no
/// division enters and the halving that law relies on is never
/// restated here.
fn saturating_sets(plane: &[Seated], arity: usize) -> Vec<Vec<[u8; 32]>> {
    let doubled = arity as u32 * u32::from(crate::almanac::LANES / 2 + 1);
    let mut found = Vec::new();
    let mut picked: Vec<usize> = Vec::with_capacity(arity);
    walk_plane(plane, arity, doubled, 0, 0, &mut picked, &mut found);
    found
}

fn walk_plane(
    plane: &[Seated],
    arity: usize,
    doubled: u32,
    from: usize,
    sum: u32,
    picked: &mut Vec<usize>,
    found: &mut Vec<Vec<[u8; 32]>>,
) {
    if picked.len() == arity {
        if sum * 2 == doubled {
            found.push(picked.iter().map(|at| plane[*at].0).collect());
        }
        return;
    }
    // What the rest could still add, at the lane fold's peak.
    let peak = u32::from(crate::almanac::LANES / 2);
    let rest = (arity - picked.len() - 1) as u32;
    for next in from..plane.len() {
        let sum = sum + u32::from(plane[next].1);
        // Binds are strictly positive, so an overshoot never recovers;
        // and a partial sum that cannot reach the target even if every
        // remaining member sat at the peak never will. Neither prune
        // may break the loop — a plane is in ledger order, not binds
        // order, so a lighter member can always follow a heavier one.
        if sum * 2 > doubled || (sum + rest * peak) * 2 < doubled {
            continue;
        }
        picked.push(next);
        walk_plane(plane, arity, doubled, next + 1, sum, picked, found);
        picked.pop();
    }
}
