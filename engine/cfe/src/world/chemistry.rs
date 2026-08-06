//! CHEMISTRY — what a world's matter makes when it is put together.
//!
//! **ELEMENT-CHEMISTRY (#37): the almanac's first consumer.** Until
//! this deed the periodic table was a complete, pinned derivation that
//! nothing in the world ever read — five axes deciding nothing, which
//! is exactly the shape LAW 8 warns about. The compounding deed is
//! what makes the table load-bearing: banked elements bond into gems
//! by the lane law, so where an element SITS finally decides what a
//! civilization can make.
//!
//! THE DOOR / FACT SPLIT (29_CONSTRUCTION):
//!
//!   DOOR   whether the matter bonds at all — the shells agree, the
//!          phases agree, the binds saturate. A property of the SET,
//!          decidable without the chain, so it dies at
//!          [`crate::almanac::Compoundable`] and cannot be re-asked.
//!   FACTS  locality, pressure and workability. Every one reads the
//!          world — the star's depth, the strata opened here, this
//!          orb's temperature — so every one stays at the verb
//!          forever: a set that cannot bond on this rock bonds on the
//!          next one.

use super::*;

/// One reading of the bench: the members that bond, the gem they name,
/// the knowledge won, and what (if anything) stands in the way here.
pub type BenchRead = (Vec<[u8; 32]>, [u8; 32], u16, Option<ActionRefused>);

impl World {
    /// Compound on the genesis orb as the sovereign.
    pub fn compound(&mut self, members: &[[u8; 32]]) -> Result<[u8; 32], ActionRefused> {
        self.compound_as(SOVEREIGN, FIRST_ORB, members)
    }

    /// The contained form. Collect scope: compounding moves banked
    /// element value, exactly as the assay does.
    ///
    /// REFUSAL-PURE — every check completes before anything moves, so
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
        // THE DOOR: does this matter bond? Past here the set is proven,
        // and its gem and knowledge are total.
        let set = crate::almanac::Compoundable::new(members)
            .ok_or(ActionRefused::MatterDoesNotBond)?;

        // THE FACTS — the place, then the ledger. One home, so the
        // BENCH cannot promise what the deed would refuse.
        if let Some(refusal) = self.bond_refusal(orb, set.members()) {
            return Err(refusal);
        }

        // THE RECIPE — one home, the same the gate re-derives from.
        let (spends, yielded) = crate::almanac::recipe_of(&set);

        // Nothing above moved anything. The deed:
        let gem = set.gem();
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
        Ok(gem)
    }

    /// **WHETHER THIS BOND STANDS RIGHT NOW** — everything past the
    /// door, in one home: the PLACE (does the sky carry it, has the
    /// ground the pressure, is the orb at a workable heat) and then
    /// the LEDGER (is there enough to spend). `None` means the deed
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

    /// **WHETHER THIS PLACE CAN WORK THIS MATTER** — the three chain
    /// facts, in one home so the deed and the bench cannot disagree.
    /// `None` means the place stands; otherwise the first refusal, on
    /// the first member that earns it.
    ///
    /// PRESSURE reads the DEEPEST stratum this orb has opened:
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
    /// GRAINED reading (`almanac::grained_t_eq4`), because workability
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

    /// **THE BENCH, WITH ITS STANDING** — every set this orb's matter
    /// would bond, each carrying what (if anything) this PLACE has to
    /// say about it.
    ///
    /// A bench that listed only what bonds is a trap: measured in live
    /// play, a world's own drawn matter routinely offers recipes its
    /// orb cannot work, because a compound shares ONE phase and an orb
    /// sits in ONE thermal region. Reading the refusal here — from the
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

    /// **THE BENCH** — every set of this orb's banked elements that
    /// would bond, as (members, gem, knowledge). Derived at call and
    /// never stored: it is a reading of the ledger, not state.
    ///
    /// Bounded by construction: it walks combinations of the elements
    /// actually HELD, and the widest compound the table admits is five
    /// bodies ([`crate::almanac::periodic::SHELLS`] + 1).
    pub fn bench(&self, orb: u64) -> Vec<(Vec<[u8; 32]>, [u8; 32], u16)> {
        let Some(held) = self.orb_by_id(orb) else {
            return Vec::new();
        };
        let stock: Vec<[u8; 32]> = held.elements.iter().map(|(id, _)| *id).collect();
        let mut found = Vec::new();
        let widest = usize::from(crate::almanac::periodic::SHELLS + 1);
        // Every subset up to the widest arity, in index order so the
        // reading is deterministic.
        let mut frontier: Vec<Vec<usize>> = stock.iter().enumerate().map(|(at, _)| vec![at]).collect();
        while let Some(picked) = frontier.pop() {
            let members: Vec<[u8; 32]> = picked.iter().map(|at| stock[*at]).collect();
            if let Some(set) = crate::almanac::Compoundable::new(&members) {
                found.push((members.clone(), set.gem(), set.knowledge()));
            }
            if picked.len() < widest {
                let last = *picked.last().expect("non-empty");
                for next in (last + 1)..stock.len() {
                    let mut wider = picked.clone();
                    wider.push(next);
                    frontier.push(wider);
                }
            }
        }
        found.sort_by_key(|(members, _, _)| members.clone());
        found
    }
}
