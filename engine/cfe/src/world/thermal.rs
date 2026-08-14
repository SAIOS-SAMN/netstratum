//! THERMAL DEEDS — the star, the siting, venting and encasement.
//!
//! THE VENT (20_FOUNDING §4): a mineral or plasmic line founds
//! BANDLESS -- above the mesic ceiling, an order of magnitude below
//! the hyperthermal floor. Coverage is defined by a bucket lookup that
//! returns nothing in that gap. Task #3 opens it.

use super::*;

impl World {
    /// THERMAL-1: the commissioned sun.
    pub fn star(&self) -> Option<crate::thermal::StarClass> {
        self.star
    }
    /// THERMAL-1 — commission the container's sun (once per world): the
    /// StarClass fixes L for every orbit (NS-6 §7).
    pub fn commission_star(
        &mut self,
        class: crate::thermal::StarClass,
    ) -> Result<(), ActionRefused> {
        if self.star.is_some() {
            return Err(ActionRefused::StarAlreadyCommissioned);
        }
        self.star = Some(class);
        // STAR-ENDOW-1: top-up genesis orb kit from the sky (pure of class).
        if let Some(orb) = peel(&mut self.orbs, FIRST_ORB) {
            crate::dop::apply_endowment_top_up(orb, &crate::dop::star_endowment(class));
        }
        self.close(ClosureEvent::StarCommissioned { class: class as u8 }, SOVEREIGN);
        Ok(())
    }
    /// THERMAL-1 — site an orb on its orbit (05 §1.1): albedo A and
    /// radius r enter the equilibrium baseline; thermal wakes for this orb.
    pub fn site_planet(
        &mut self,
        orb: u64,
        albedo: crate::dop::Unit,
        orbit_r: crate::dop::Positive,
        theta: Option<crate::dop::Slot>,
        psi: Option<crate::dop::Slot>,
    ) -> Result<(), ActionRefused> {
        use crate::dop::ThermalSite;
        if self.star.is_none() {
            return Err(ActionRefused::NoStarCommissioned);
        }
        // THE PLACEMENT DRAW (2026-08-06), rewritten NATIVE to the slot
        // type the C1 kill landed: the draw produces `Slot::from_index`
        // directly — total, wrapping — so an off-lattice draw is not a
        // case that exists. `None` still means *the world places me*:
        // the longitude draws from the chronicle's terminal digest
        // (CHARTER §9) and walks forward past taken places
        // deterministically; the RECEIPT CARRIES THE RESOLVED SLOT, so
        // replay applies and never re-draws.
        let theta = match theta {
            Some(turn) => Some(turn),
            None => {
                let seed = self
                    .chronicle
                    .receipts()
                    .last()
                    .map(|receipt| receipt.own.0)
                    .unwrap_or([0u8; 32]);
                let mut hasher = blake3::Hasher::new();
                hasher.update(b"netstratum.place.theta.v1");
                hasher.update(&seed);
                hasher.update(&orb.to_le_bytes());
                let slots = crate::ratio::TURN_SLOTS as u64;
                let start = u64::from_le_bytes(
                    hasher.finalize().as_bytes()[..8].try_into().expect("eight bytes"),
                ) % slots;
                let taken = |candidate: &crate::dop::Slot| {
                    self.orbs.iter().any(|held| {
                        held.orb_id != orb
                            && held.thermal.as_ref().is_some_and(|site| {
                                site.orbit_r == orbit_r
                                    && site.theta.as_ref() == Some(candidate)
                                    && site.psi == psi
                            })
                    })
                };
                let drawn = (0..slots)
                    .map(|offset| crate::dop::Slot::from_index((start + offset) as i64))
                    .find(|candidate| !taken(candidate));
                match drawn {
                    Some(turn) => Some(turn),
                    None => {
                        let first = crate::dop::Slot::from_index(start as i64);
                        let holder = self
                            .orbs
                            .iter()
                            .find(|held| {
                                held.thermal.as_ref().is_some_and(|site| {
                                    site.orbit_r == orbit_r
                                        && site.theta.as_ref() == Some(&first)
                                        && site.psi == psi
                                })
                            })
                            .map(|held| held.orb_id)
                            .unwrap_or(orb);
                        return Err(ActionRefused::OrbitOccupied {
                            orb: holder,
                            slot: first.index(),
                        });
                    }
                }
            }
        };
        // THE PLACEMENT LAW, half retired by construction
        // (29_CONSTRUCTION C1): an angle IS a `Slot` — off-slot cannot
        // reach this function, so the `OffSlot` judge arms are deleted
        // and only the FACT half remains: no two orbs share a place.
        // Collision stays decidable because the type is discrete.
        if let Some(turn) = &theta {
            let slot = turn.index();
            if let Some(taken) = self.orbs.iter().find(|held| {
                held.orb_id != orb
                    && held.thermal.as_ref().is_some_and(|site| {
                        site.orbit_r == orbit_r
                            && site.theta.as_ref() == Some(turn)
                            // ψ SEPARATES: two orbs may share a ring
                            // and a longitude if they are inclined
                            // differently. Without this the collision
                            // law would forbid a volume it should allow.
                            && site.psi == psi
                    })
            }) {
                return Err(ActionRefused::OrbitOccupied { orb: taken.orb_id, slot });
            }
        }
        // BadSiting is DEAD (29_CONSTRUCTION C1): albedo is a `Unit`
        // and the radius a `Positive` — out-of-range cannot reach this
        // verb, so the range judge is deleted and only facts remain.
        //
        // THE RING REACH (operator ruling 2026-08-06) is exactly such a
        // FACT: the star's MAGNETISM bounds how many RINGS it anchors —
        // a ring is a distinct orbit radius, and the reach is the field
        // grade (`StarClass::ring_reach`, the dynamo derivation).
        // Filling an anchored ring stays admitted; THREADING A NEW ONE
        // beyond the field refuses. A count against a derived bound is
        // arithmetic, not a shape — it stays at the door forever.
        let mut rings: Vec<&crate::dop::Positive> = Vec::new();
        for held in &self.orbs {
            if let Some(site) = &held.thermal {
                if !rings.contains(&&site.orbit_r) {
                    rings.push(&site.orbit_r);
                }
            }
        }
        if !rings.contains(&&orbit_r) {
            let reach = self.star.expect("checked above").ring_reach();
            if rings.len() >= usize::from(reach) {
                return Err(ActionRefused::RingReachExceeded { held: rings.len(), reach });
            }
        }
        let held = peel(&mut self.orbs, orb)
            .ok_or(ActionRefused::NoSuchOrb { orb })?;
        if held.thermal.is_some() {
            return Err(ActionRefused::AlreadySited { orb });
        }
        held.thermal = Some(ThermalSite {
            albedo: albedo.clone(),
            orbit_r: orbit_r.clone(),
            theta: theta.clone(),
            // THE LIVE PATH KEEPS THE INCLINATION. Writing None here
            // while the RECEIPT carried it made live and replay
            // disagree in the opposite direction to the earlier drops —
            // and clippy could not see it, because the binding IS used
            // (by the receipt). Only replay_fidelity catches this.
            psi: psi.clone(),
        });
        // ORBIT-ENDOW-1: band gift from place (pure of star + site).
        let class = self.star.expect("checked above");
        let teq = crate::thermal::t_eq4(
            &class.luminosity(),
            albedo.value(),
            orbit_r.value(),
        );
        let band = crate::thermal::band_of(&teq);
        crate::dop::apply_endowment_add(held, &crate::dop::orbit_endowment(band));
        // TECH-1a (12_UNION §5a): a world's FIRST thermal opt-in
        // receipts the kernel SOLAR LEAP — its people now know their
        // sun. Once per world, never per orb; the node id is seeded
        // from the kernel name + the discovering orb (SEAL B).
        if !self.techs.iter().any(|(_, condition, _, _)| *condition == 0) {
            let mut seed = b"tech.kernel.solar-leap".to_vec();
            seed.extend_from_slice(&orb.to_le_bytes());
            let node: [u8; 32] = blake3::hash(&seed).into();
            self.techs.push((orb, 0, node, ratio(1, 1)));
            self.close(
                ClosureEvent::TechDiscovered {
                    orb_id: orb,
                    node,
                    condition: 0,
                    modifier: 0,
                    magnitude: ratio(1, 1),
                    name: "the solar leap".into(),
                },
                SOVEREIGN,
            );
        }
        self.close(
            ClosureEvent::PlanetSited { orb_id: orb, albedo, orbit_r, theta, psi },
            SOVEREIGN,
        );
        Ok(())
    }
    /// AEON-1a (11 §6): the star MOVES in its region — the class
    /// steps, luminosity recomputes, and every sited orbit's T_eq⁴
    /// shifts at the next tick. Movement, never a rung.
    pub fn age_star(
        &mut self,
        class: crate::thermal::StarClass,
    ) -> Result<(), ActionRefused> {
        if self.star.is_none() {
            return Err(ActionRefused::NoStarCommissioned);
        }
        self.star = Some(class);
        self.close(ClosureEvent::StarAged { class: class as u8 }, SOVEREIGN);
        // #27 STELLAR NUCLEOSYNTHESIS: a star aging into a REMNANT (the
        // heavy end of the class spine — a white dwarf and beyond) dies
        // and scatters heavy elements; every sited orb gains an enrichment
        // vein. A main-sequence move forges nothing new.
        if class as u8 >= crate::thermal::StarClass::WhiteDwarf as u8 {
            self.scatter_enrichment(class);
        }
        Ok(())
    }

    /// #27: seed each sited orb with an enrichment vein of a heavy element
    /// the dying `class` forged — element drawn from the remnant's own
    /// periodic spine (a heavier remnant seeds heavier matter), at a
    /// reserved stellar stratum above the diggable reach so it never
    /// collides with a prospect. Idempotent per (orb, remnant): a star
    /// that dies twice into the same class enriches once.
    fn scatter_enrichment(&mut self, class: crate::thermal::StarClass) {
        let stratum = class as u8;
        let Some(root) = self
            .chronicle
            .receipts()
            .iter()
            .find(|receipt| matches!(receipt.event, ClosureEvent::ColonyFounded { .. }))
            .map(|receipt| receipt.own.0)
        else {
            return;
        };
        let sited: Vec<u64> = self
            .orbs
            .iter()
            .filter(|held| held.thermal.is_some())
            .map(|held| held.orb_id)
            .collect();
        let stamp = self.clock.now();
        for orb in sited {
            if self.prospects.iter().any(|vein| vein.orb == orb && vein.stratum == stratum) {
                continue;
            }
            let mut seed = root.to_vec();
            seed.extend_from_slice(&orb.to_le_bytes());
            seed.push(stratum);
            let mut element: [u8; 32] = blake3::hash(&seed).into();
            element[3] %= crate::almanac::strata_lines(u8::MAX, class).max(1);
            let (_, _, richness) = crate::dop::element_properties(&element, class);
            self.prospects.push(crate::world::Prospected {
                orb,
                stratum,
                element,
                richness: richness.clone(),
                last_assay: stamp,
            });
            self.chronicle.append(
                ClosureEvent::EnrichmentStruck { orb_id: orb, stratum, element, richness },
                stamp,
                vec![SOVEREIGN],
            );
        }
    }
    /// ABSORB-1: this orb's un-sealed vented overflow (workspace) —
    /// what stands absorbable before the void claims it at collect.
    pub fn vent_pending_of(&self, orb: u64) -> Ratio {
        self.vent_pending
            .iter()
            .find(|(held, _)| *held == orb)
            .map(|(_, pending)| pending.clone())
            .unwrap_or_else(|| ratio(0, 1))
    }
}
