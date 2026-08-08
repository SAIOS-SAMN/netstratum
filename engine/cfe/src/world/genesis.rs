//! GENESIS AND IDENTITY — founding, resuming, entities, taxon,
//! stations, wards.

use super::*;

impl World {
    pub fn new() -> Self {
        Self {
            clock: WorldClock::new(),
            keepers: [Timekeeper { id: 0 }, Timekeeper { id: 1 }, Timekeeper { id: 2 }],
            chronicle: Chronicle::new(),
            orbs: Vec::new(),
            entities: Vec::new(),
            star: None,
            accrued: Vec::new(),
            vent_pending: Vec::new(),
            debts: Vec::new(),
            bonds: Vec::new(),
            forwards: Vec::new(),
            councils: Vec::new(),
            offers: Vec::new(),
            friction_pending: Vec::new(),
            techs: Vec::new(),
            wants: Vec::new(),
            shear_cursor: (0, Vec::new()),
            crossed: false,
            gauge: vec![ratio(0, 1)],
            genesis: None,
            terrain_seed: None,
            practices: Vec::new(),
            instruments: Vec::new(),
            routes: Vec::new(),
            element_routes: Vec::new(),
            treaties: Vec::new(),
            prospects: Vec::new(),
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
        }
    }
    /// P2-PERSIST — resume a world from its chronicle: the committed
    /// projection replays (orbs, entities), the clock reinstates at the
    /// highest stamped value the receipts prove, and the workspace
    /// starts EMPTY — accrued value and un-stamped ticks from the prior
    /// era had no receipts, so they honestly do not persist.
    pub fn resume(chronicle: Chronicle) -> World {
        let committed = replay(&chronicle);
        let stamp = chronicle
            .receipts()
            .iter()
            .map(|receipt| receipt.stamp)
            .max()
            .unwrap_or(0);
        World {
            clock: WorldClock::resume(stamp),
            keepers: [Timekeeper { id: 0 }, Timekeeper { id: 1 }, Timekeeper { id: 2 }],
            chronicle,
            orbs: committed.orbs,
            entities: committed.entities,
            star: committed.star,
            accrued: Vec::new(),
            vent_pending: Vec::new(),
            debts: committed.debts,
            bonds: committed.bonds,
            forwards: committed.forwards,
            councils: committed.councils,
            offers: committed.offers,
            friction_pending: Vec::new(),
            techs: committed.techs,
            wants: committed.wants,
            shear_cursor: (0, Vec::new()),
            crossed: committed.crossed,
            gauge: committed.gauge,
            genesis: committed.genesis,
            terrain_seed: committed.terrain_seed,
            practices: committed.practices,
            instruments: committed.instruments,
            routes: committed.routes,
            element_routes: committed.element_routes,
            treaties: committed.treaties,
            prospects: committed.prospects,
            warded: committed.warded,
            veiled: committed.veiled,
            taxon: committed.taxon,
            stations: committed.stations,
            hulls: committed.hulls,
            notes: committed.notes,
            futures: committed.futures,
            last_clears: committed.last_clears,
            shares: committed.shares,
            perps: committed.perps,
            clear_depths: committed.clear_depths,
            market_sealed_until: committed.market_sealed_until,
            cascade_pressures: committed.cascade_pressures,
            tamper_pressures: committed.tamper_pressures,
        }
    }
    /// ELEVATE-1 (12_UNION §6) — THE THREE-GATE GAUGE CLIMB, the
    /// discrete-gauge-morphodynamics ascent (Q3 resolved). An earned,
    /// receipted rise in G, executed as `GaugeShifted`, gated on three
    /// exact readings of this world's own chronicle:
    ///   S — thermodynamic settlement: no famine, no overheat, vent
    ///       workspace sealed, demurrage current on every orb;
    ///   T — loop coherence: the colony closed a loop within the last
    ///       year voice (60 ticks — 11 §1; placeholder constant);
    ///   G — the crossing: this world entered the union (THRESHOLD-1).
    /// The climb is monotone in G (the orientation law — time's arrow,
    /// not a queue): `delta` is a [`crate::dop::Positive`], so a flat
    /// or backward climb is unrepresentable and the old `BadClimb` arm
    /// is dead (29_CONSTRUCTION). A refusal names every gate's
    /// reading — the cure, not a verdict.
    pub fn elevate(&mut self, delta: crate::dop::Positive) -> Result<(), ActionRefused> {
        let delta = delta.value().clone();
        let stamp = self.clock.now();
        let settled = self.orbs.iter().all(|held| {
            held.famine.is_empty()
                && !held.overheat
                && demurrage_due(held, self.star, stamp, &ease_of(&self.techs, held.orb_id, stamp), &standing_supply(&self.orbs))
                    .iter()
                    .all(|(_, due)| *due == ratio(0, 1))
        }) && self
            .vent_pending
            .iter()
            .all(|(_, pending)| *pending == ratio(0, 1));
        let year_floor = stamp.saturating_sub(60);
        let coherent = self.chronicle.receipts().iter().any(|receipt| {
            receipt.stamp >= year_floor
                && matches!(receipt.event, ClosureEvent::YieldCollected { .. })
        });
        let crossed = self.crossed;
        // CHANNEL_ALIGN (operator-ruled): an adopted discipline must
        // stand ALIGNED — the syntropy ratio (09 §2.5, engine read)
        // holds the floor the disciplines THEMSELVES declare, or the
        // climb refuses. The alignment loop: resolve the friction to
        // rise. No adopted practice binds the gate → no floor → a line
        // with no practice, or no deeds posted, stands aligned.
        let aligned = match self.practice_gate(crate::vocabulary::CHANNEL_ALIGN) {
            None => true,
            Some(floor) => self
                .syntropy_ratio()
                .map(|ratio_read| ratio_read >= floor)
                .unwrap_or(true),
        };
        if !(settled && coherent && crossed && aligned) {
            return Err(ActionRefused::ElevationBarred { settled, coherent, crossed, aligned });
        }
        let gauge = wrap_gauge(self.gauge[0].clone() + delta);
        self.gauge[0] = gauge.clone();
        self.close(ClosureEvent::GaugeShifted { gauge }, SOVEREIGN);
        Ok(())
    }
    pub fn entity(&self, entity: u64) -> Option<&Entity> {
        self.entities.iter().find(|held| held.entity_id == entity)
    }
    /// Found the GENESIS colony — banks the bootstrap reserves (02
    /// §2.1); registers the sovereign first. Later colonies arrive by
    /// `chart` — founding is once per world.
    pub fn found(&mut self, name: &str) -> Result<(), ActionRefused> {
        match self.orbs.is_empty() {
            false => Err(ActionRefused::ColonyAlreadyFounded),
            true => {
                if self.entities.is_empty() {
                    self.entities.push(Entity {
                        entity_id: SOVEREIGN,
                        handle: "founder".to_string(),
                        principal: None,
                        grants: Vec::new(),
                    });
                    self.close(
                        ClosureEvent::EntityRegistered {
                            entity_id: SOVEREIGN,
                            principal: 0,
                            handle: "founder".to_string(),
                        },
                        SOVEREIGN,
                    );
                }
                let mut orb = ColonyOrb {
                    orb_id: FIRST_ORB,
                    name: name.to_string(),
                    ..ColonyOrb::default()
                };
                for (resource, amount) in founding_reserves() {
                    orb.bank(resource, amount);
                }
                self.orbs.push(orb);
                self.close(
                    ClosureEvent::ColonyFounded {
                        orb_id: FIRST_ORB,
                        name: name.to_string(),
                    },
                    SOVEREIGN,
                );
                Ok(())
            }
        }
    }
    /// G1 (04 §1/§5) — seal the line's three axes as THE binding
    /// receipt in this world's chronicle. Entry is in the bands
    /// (04 §6a — never a tier-0 seat); one line per world; the
    /// refusals are computed pure. From this receipt on, the registry
    /// entry is a cache and no mechanic reads it. Sovereign
    /// convenience.
    pub fn seal_genesis(
        &mut self,
        civilization: &str,
        disposition: u8,
        metabolism: u8,
        perception: u8,
    ) -> Result<(), ActionRefused> {
        self.seal_genesis_as(SOVEREIGN, civilization, disposition, metabolism, perception)
    }
    /// The contained form. Build scope: genesis is a founding-class
    /// act on a founded world.
    pub fn seal_genesis_as(
        &mut self,
        actor: u64,
        civilization: &str,
        disposition: u8,
        metabolism: u8,
        perception: u8,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        if self.genesis.is_some() {
            return Err(ActionRefused::GenesisAlreadySealed);
        }
        for (axis, code) in [(1u8, disposition), (2, metabolism), (3, perception)] {
            // 28_VOCABULARY V1: the axis vocabularies fold too, so a
            // minted disposition/metabolism/perception is seatable.
            let folded = crate::vocabulary::Axis::from_code(axis)
                .map(|on| code != 0 && code <= crate::vocabulary::ceiling(self.chronicle(), on))
                .unwrap_or(false);
            if !folded {
                return Err(ActionRefused::UnknownAxisValue { axis, code });
            }
        }
        self.genesis = Some(Genesis {
            civilization: civilization.to_string(),
            disposition,
            metabolism,
            perception,
        });
        self.close(
            ClosureEvent::GenesisSealed {
                civilization: civilization.to_string(),
                disposition,
                metabolism,
                perception,
            },
            actor,
        );
        Ok(())
    }
    /// G1: the sealed line, if any.
    pub fn genesis(&self) -> Option<&Genesis> {
        self.genesis.as_ref()
    }
    /// WARDEN-1a (10 §3e) — seat the reverse shield: this star is the
    /// elder. Refuses every aperture (both directions) until the ward
    /// lifts; petitions nothing; holds no seat. Once per star.
    pub fn ward_elder(&mut self) -> Result<(), ActionRefused> {
        if self.warded.is_some() {
            return Err(ActionRefused::WardOutOfOrder);
        }
        self.close(ClosureEvent::ElderWarded {}, SOVEREIGN);
        self.warded = Some(false);
        Ok(())
    }
    /// WARDEN-1a — lift the ward: the depth gate. The claimer's
    /// recorded depth (the ledger's solves reading, passed by the
    /// caller — depth lives ledger-side) must reach the ceiling
    /// (placeholder 5). The receipt names the claimer; the template
    /// grant is WARDEN-1b.
    pub fn lift_ward(&mut self, claimer: &str, depth: u32) -> Result<(), ActionRefused> {
        if self.warded != Some(false) {
            return Err(ActionRefused::WardOutOfOrder);
        }
        let ceiling = super::rules::ward_gate_depth();
        if depth < ceiling {
            return Err(ActionRefused::DepthBelowCeiling { depth, ceiling });
        }
        self.close(ClosureEvent::WardLifted { claimer: claimer.into() }, SOVEREIGN);
        self.warded = Some(true);
        Ok(())
    }
    /// WARDEN-1a: the reverse shield stands.
    pub fn ward_holds(&self) -> bool {
        self.warded == Some(false)
    }
    /// KERNEL-TAXON (15 §4) — seal class and clade as identity: once
    /// per star, in-vocabulary or refused, no value moved. Build
    /// scope (an identity-class act, the G1 posture). The coverage
    /// {containment, exactness} enters the constitution at the
    /// amendment; this deed is lawful now and sealed then.
    pub fn seal_taxon(&mut self, class: u8, clade: u8) -> Result<(), ActionRefused> {
        self.seal_taxon_as(SOVEREIGN, class, clade)
    }
    pub fn seal_taxon_as(
        &mut self,
        actor: u64,
        class: u8,
        clade: u8,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.taxon.is_some() {
            return Err(ActionRefused::TaxonAlreadySealed);
        }
        // 28_VOCABULARY V1: the ceilings are READINGS OF THE FOLD, not
        // constants. A class a progenitor minted this era is sealable in
        // it; a class nobody minted is not. Before this the ceilings were
        // compiled, which is why clades 19 and 20 stood ruled-but-
        // unsealable for four days.
        use crate::vocabulary::{ceiling, Axis};
        if class == 0
            || clade == 0
            || class > ceiling(self.chronicle(), Axis::Class)
            || clade > ceiling(self.chronicle(), Axis::Clade)
        {
            return Err(ActionRefused::TaxonUnknown { class, clade });
        }
        self.close(ClosureEvent::TaxonSealed { class, clade }, actor);
        self.taxon = Some((class, clade));
        Ok(())
    }
    /// KERNEL-TAXON: the sealed taxon, if the deed has been done.
    pub fn taxon(&self) -> Option<(u8, u8)> {
        self.taxon
    }
    /// **MINT A LINE** (tag 101, 2026-08-06): the engine mints its
    /// peoples — the standing ruling that retired the author. The seed
    /// is the chronicle's terminal digest (CHARTER §9); the whole line
    /// derives from it through `dop::line_synthesis`, so the receipt
    /// carries the seed alone and every reader derives the same people.
    /// Build scope, refusal-pure. Successive mints draw different
    /// lines, because each receipt moves the terminal digest.
    pub fn mint_line(&mut self) -> Result<crate::dop::LineDraw, ActionRefused> {
        self.mint_line_as(SOVEREIGN)
    }

    pub fn mint_line_as(
        &mut self,
        actor: u64,
    ) -> Result<crate::dop::LineDraw, ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        let Some(seed) = self.chronicle.receipts().last().map(|receipt| receipt.own.0)
        else {
            return Err(ActionRefused::NoColony);
        };
        // FOLD-AWARE (operator ruling 2026-08-06): the draw runs under
        // the FOLD's ceilings — a minted class or clade is drawable the
        // moment its receipt stands — and the receipt DECLARES the
        // ceilings it drew under, so every reader re-derives this exact
        // draw from the receipt alone (receipt authority; the ordering
        // law holds by declaration).
        let ceilings = (
            crate::vocabulary::ceiling(self.chronicle(), crate::vocabulary::Axis::Class),
            crate::vocabulary::ceiling(self.chronicle(), crate::vocabulary::Axis::Clade),
        );
        let draw = crate::dop::line_synthesis_at(&seed, ceilings.0, ceilings.1);
        self.close(ClosureEvent::LineMinted { seed, ceilings: Some(ceilings) }, actor);
        Ok(draw)
    }

    /// THE VERTICAL AXIS (tag 89) — seat a line in an office.
    ///
    /// A station GATES AUTHORITY (the tier-0 verbs check it), and until
    /// this deed existed it lived only in `var/registry/<sigil>.json` —
    /// a file anyone with a shell could edit. `KERNEL_RELAY` §1.5 named
    /// the hole exactly: *the game is constitutional; the universe is
    /// not*. The seat is now receipted and committed, and the registry
    /// is a CACHE from this receipt on — precisely what `GenesisSealed`
    /// did for the axes at G1.
    ///
    /// Build-scoped. `station` is a [`crate::dop::Station`] — the BAND-1
    /// vocabulary AS A TYPE, so an off-band index is unrepresentable and
    /// the old range check is dead (29_CONSTRUCTION C1).
    pub fn seat_station(
        &mut self,
        line: crate::dop::Name,
        station: crate::dop::Station,
    ) -> Result<(), ActionRefused> {
        self.seat_station_as(SOVEREIGN, line, station)
    }
    pub fn seat_station_as(
        &mut self,
        actor: u64,
        line: crate::dop::Name,
        station: crate::dop::Station,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        // Both shapes are unrepresentable now — a `Station` IS one of
        // the four offices, and a `Name` cannot be empty, so the dead
        // `LineUnnamed` arm has no case left to catch (29_CONSTRUCTION).
        let line = line.as_str();
        let station = station.as_wire();
        self.close(
            ClosureEvent::StationSeated { line: line.to_string(), station },
            actor,
        );
        match self.stations.iter_mut().find(|(held, _)| held == line) {
            Some(seat) => seat.1 = station,
            None => self.stations.push((line.to_string(), station)),
        }
        Ok(())
    }
    /// **THE VOCABULARY MINT** (tag 90, `28_VOCABULARY` §5–§7).
    ///
    /// A T0 progenitor adds an entry to a vocabulary, and the board
    /// grows a class of entity that no coder wrote. Two gates remain,
    /// both FACTS of the chain — standing and novelty; the shapes died
    /// at the doors (29_CONSTRUCTION): the axis is a
    /// [`crate::vocabulary::Axis`] (no unknown axis constructs, and no
    /// variant names a STATION — §10), and the binding is a
    /// [`crate::vocabulary::TradingBinding`] (one benefit, one
    /// friction, in bounds, on real channels — §4a — or it does not
    /// construct).
    ///
    /// **The code is DERIVED here, never accepted.** The caller does not
    /// pass one: the lawful value is `ceiling(axis) + 1` read from the
    /// fold, and the receipt carries it so a reader who has not folded
    /// the chain can still see which entry this is. A caller that could
    /// choose could collide two nodes deliberately.
    pub fn mint_vocabulary(
        &mut self,
        line: &str,
        axis: crate::vocabulary::Axis,
        sigil: &str,
        word: crate::dop::Name,
        binding: crate::vocabulary::TradingBinding,
    ) -> Result<u8, ActionRefused> {
        self.mint_vocabulary_as(SOVEREIGN, line, axis, sigil, word, binding)
    }
    pub fn mint_vocabulary_as(
        &mut self,
        actor: u64,
        line: &str,
        axis: crate::vocabulary::Axis,
        sigil: &str,
        word: crate::dop::Name,
        binding: crate::vocabulary::TradingBinding,
    ) -> Result<u8, ActionRefused> {
        self.mint_vocabulary_based_as(actor, line, axis, sigil, word, binding, None)
    }

    /// **THE ZONE MINT'S FULL FORM (#41)** — the same deed carrying a
    /// declared BASE, the first rung of the operator's resolution
    /// ladder (`vocabulary::zone_base`).
    ///
    /// `base` is meaningful only on [`crate::vocabulary::Axis::Zone`]:
    /// a minted building's binding is a set of scale factors, and the
    /// base is what they scale. Left `None`, the base is INFERRED from
    /// the channels the binding touches; where neither answers, the
    /// entry is lawful vocabulary that simply never builds.
    ///
    /// A declared base must name a COMPILED kind. Refusing here rather
    /// than degrading to inference is the P1 discipline: a caller that
    /// names a base at all has said something decidable, and a base
    /// that names nothing is a malformed declaration, not a silence.
    #[allow(clippy::too_many_arguments)]
    pub fn mint_vocabulary_based_as(
        &mut self,
        actor: u64,
        line: &str,
        axis: crate::vocabulary::Axis,
        sigil: &str,
        word: crate::dop::Name,
        binding: crate::vocabulary::TradingBinding,
        base: Option<u8>,
    ) -> Result<u8, ActionRefused> {
        use crate::vocabulary as vocab;
        self.permitted(actor, Scope::Build)?;

        if let Some(code) = base {
            if axis != vocab::Axis::Zone {
                return Err(ActionRefused::BaseOffAxis { axis: axis.code() });
            }
            if code == 0 || code > crate::dop::ZONE_KINDS {
                return Err(ActionRefused::BaseUnknown { code });
            }
        }

        // GATE 1 — STANDING. Tier 0 only. `station_of` reads the CHAIN,
        // so this cannot be granted by editing anything.
        if self.station_of(line) != Some(1) {
            return Err(ActionRefused::MintWithoutStanding { line: line.to_string() });
        }

        // The trade law (§4a) died at the `TradingBinding` door — a
        // binding that does not trade cannot be constructed.

        // GATE 2 — NOVELTY (§6), measured on the canonical mechanism.
        let standing = vocab::fold(self.chronicle(), axis);
        let shape = vocab::canonical(binding.entries());
        if let Some(twin) = standing
            .iter()
            .find(|entry| vocab::canonical(&entry.binding) == shape)
        {
            return Err(ActionRefused::MintNotNovel { like: twin.code });
        }
        // The empty word died at the `Name` door; only the collision
        // FACT remains — a mark that names two things defines neither.
        let word = word.as_str();
        if standing.iter().any(|entry| {
            entry.word == word || (!sigil.is_empty() && entry.sigil == sigil)
        }) {
            return Err(ActionRefused::MintNameTaken { word: word.to_string() });
        }

        let code = standing.iter().map(|entry| entry.code).max().unwrap_or(0) + 1;
        self.close(
            ClosureEvent::VocabularyMinted {
                // The wire keeps the u8 code and the raw entries — the
                // receipt is readable without the door types, and the
                // encoding is unchanged.
                axis: axis.code(),
                code,
                sigil: sigil.to_string(),
                word: word.to_string(),
                binding: binding.into_entries(),
                base,
            },
            actor,
        );
        Ok(code)
    }

    /// **CHARTER A PARAMETER** (tag 103, P2's mint path for the
    /// balance placeholders). The band died at the door — a
    /// [`crate::parameter::Chartered`] stands inside the fourfold band
    /// of its seed or it does not construct — so ONE gate remains, a
    /// fact of the chain: chartering is a TIER 0 power under the same
    /// standing law as the vocabulary mint (12_UNION ruling 4).
    pub fn charter_parameter(
        &mut self,
        line: &str,
        charter: crate::parameter::Chartered,
    ) -> Result<(), ActionRefused> {
        self.charter_parameter_as(SOVEREIGN, line, charter)
    }

    pub fn charter_parameter_as(
        &mut self,
        actor: u64,
        line: &str,
        charter: crate::parameter::Chartered,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.station_of(line) != Some(1) {
            return Err(ActionRefused::MintWithoutStanding { line: line.to_string() });
        }
        self.close(
            ClosureEvent::ParameterChartered {
                key: charter.key().code(),
                value: charter.value().clone(),
            },
            actor,
        );
        Ok(())
    }

    /// The chartered value of a parameter — the FOLD's reading (seed ∪
    /// charters). Public: the conductor's thermal mirror and the CLI
    /// read the same rule the tick runs.
    pub fn parameter(&self, key: crate::parameter::Parameter) -> crate::ratio::Ratio {
        crate::parameter::of(self.chronicle(), key)
    }

    /// THE VERTICAL AXIS: the seat a line holds, from the CHAIN.
    pub fn station_of(&self, line: &str) -> Option<u8> {
        self.stations.iter().find(|(held, _)| held == line).map(|(_, seat)| *seat)
    }
}
