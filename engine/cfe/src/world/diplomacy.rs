//! DIPLOMACY — what one world agrees with another (#42).
//!
//! Councils, bonds and forwards bind parties to QUANTITIES. A treaty
//! binds them to a RELATIONSHIP, and the relationship is what every
//! quantity between them is then priced through.
//!
//! **A pact is a PAIR object and neither world owns it.** Each book
//! carries its own leg, the aperture's deal-id pattern, so a treaty
//! STANDS between two worlds exactly when both have sealed it — and
//! either can renounce its own leg without needing the other's leave.
//! That is the whole reason it is co-signed rather than decreed.
//!
//! Everything shape-decidable died at the [`crate::treaty::TreatyTerms`]
//! door: a trade pact that raises the melt, a war that eases it, a
//! peace smuggling terms, a pact binding nothing. What is left here is
//! the CHAIN's business — who is already at war with whom, and what a
//! new pact does to the pacts already standing.

use super::*;
use crate::treaty::{Treaty, TreatyKind, TreatyTerms};

impl World {
    /// Seal a pact as the sovereign.
    pub fn treaty(
        &mut self,
        deal: u64,
        partner: &str,
        terms: TreatyTerms,
        term: u64,
    ) -> Result<(), ActionRefused> {
        self.treaty_as(SOVEREIGN, deal, partner, terms, term)
    }

    /// **SEAL THIS WORLD'S LEG OF A PACT.** Build scope: a treaty
    /// re-rules the ground others trade on, which is the founding
    /// class of act.
    ///
    /// THE DISPLACEMENT LAW is the political core. Peace and war cannot
    /// both stand on a pair, so sealing one STRIKES the other — and the
    /// two directions are not the same deed:
    ///
    ///   PEACE over a war   ends it. Nothing is owed for stopping.
    ///   WAR over a peace   BREAKS it, and the breach posts to this
    ///                      world's own entropic book, exactly as
    ///                      severing a bond without notice does.
    ///
    /// The asymmetry is the point: a world may always stop fighting for
    /// free, and never start for free.
    pub fn treaty_as(
        &mut self,
        actor: u64,
        deal: u64,
        partner: &str,
        terms: TreatyTerms,
        term: u64,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if self.orbs.is_empty() {
            return Err(ActionRefused::NoColony);
        }
        // A world does not treat with itself — the pair would be one
        // party, and every law here reads two.
        if self.orbs.iter().any(|held| held.name == partner) {
            return Err(ActionRefused::TreatyWithSelf);
        }
        let kind = terms.kind();
        let now = self.clock.now();
        // ONE LIVE PACT OF A KIND PER PARTNER. A second is not a
        // stronger pact, it is an ambiguous one — and `factor_on`
        // multiplies, so it would silently compound.
        if crate::treaty::stands(&self.treaties, partner, kind, now) {
            return Err(ActionRefused::TreatyAlreadyStands {
                partner: partner.to_string(),
                kind: kind.wire(),
            });
        }

        // **THE DEFENSE BAR** — what a defense pact COSTS to hold. You
        // cannot declare war on a world you swore to defend, so the
        // pact spends an option that a trade pact leaves in your hand.
        // Renounce the defense first if you mean it; the chronicle will
        // show you did, which is the point.
        if kind == TreatyKind::War
            && crate::treaty::stands(&self.treaties, partner, TreatyKind::Defense, now)
        {
            return Err(ActionRefused::SwornToDefend { partner: partner.to_string() });
        }

        // **AND THE SAME BAR, THE OTHER WAY.** You cannot swear to
        // defend a world you are at war with. Found by playing it: the
        // war bar alone let a world hold a defense pact and a war with
        // the same partner at once, which is not a position — it is two
        // contradictory ones. Peace first; the displacement law then
        // ends the war, and the oath can follow.
        if kind == TreatyKind::Defense && crate::treaty::walled(&self.treaties, partner, now) {
            return Err(ActionRefused::AtWar { partner: partner.to_string() });
        }

        // THE DISPLACEMENT — struck FIRST, and receipted in its own
        // right, so the chronicle shows the war ending rather than
        // merely showing a peace appearing.
        if let Some(displaced) = kind.displaces() {
            let standing: Vec<Treaty> = self
                .treaties
                .iter()
                .filter(|held| held.partner == partner && held.kind == displaced)
                .cloned()
                .collect();
            for held in standing {
                let broken = kind == TreatyKind::War;
                self.treaties
                    .retain(|other| !(other.deal == held.deal && other.partner == *partner));
                // E3.9: the annex dies with the pact it rides.
                self.element_allowlists
                    .retain(|(deal, other, _)| !(*deal == held.deal && other == partner));
                self.close(
                    ClosureEvent::TreatyStruck {
                        deal: held.deal,
                        partner: partner.to_string(),
                        broken,
                    },
                    actor,
                );
                // BREAKING A PEACE COSTS. The load posts to the
                // breaker's own severance book — the same book a bond
                // cut without notice posts to, because it is the same
                // act: an obligation ended by one side's choice.
                if broken {
                    let load = crate::dop::commensurate_weight(Resource::Iron);
                    if let Some(orb) = self.orbs.first_mut() {
                        orb.severance_load += load;
                    }
                }
            }
        }

        let kept = terms.into_terms();
        self.treaties.push(Treaty {
            deal,
            kind,
            partner: partner.to_string(),
            terms: kept.clone(),
            sealed: now,
            term,
        });
        self.close(
            ClosureEvent::TreatySealed {
                deal,
                kind: kind.wire(),
                partner: partner.to_string(),
                terms: kept,
                term,
            },
            actor,
        );
        Ok(())
    }

    /// **E3.9 — ANNEX AN ELEMENT ALLOW-LIST to a standing trade pact.**
    pub fn treaty_elements(
        &mut self,
        deal: u64,
        partner: &str,
        elements: Vec<[u8; 32]>,
    ) -> Result<(), ActionRefused> {
        self.treaty_elements_as(SOVEREIGN, deal, partner, elements)
    }

    /// The contained form. Build scope — the list re-rules what may
    /// cross the pair, the same founding class of act as the pact it
    /// rides.
    ///
    /// The list ANNEXES a live TRADE treaty (same deal, same partner):
    /// trade is the only kind that binds goods — defense and peace
    /// bind none, and war walls everything already. It dies with the
    /// pact (strike, renounce, or expiry — liveness is read from the
    /// treaty, never stored). Lists on one deal COMPOSE BY UNION,
    /// append-only, like every additive receipt here. An empty list
    /// refuses: barring all matter is war's job, not a trade annex's
    /// (the PeaceBindsNothing shape).
    pub fn treaty_elements_as(
        &mut self,
        actor: u64,
        deal: u64,
        partner: &str,
        elements: Vec<[u8; 32]>,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if elements.is_empty() {
            return Err(ActionRefused::AllowListEmpty);
        }
        let now = self.clock.now();
        let rides = self.treaties.iter().any(|held| {
            held.deal == deal
                && held.partner == partner
                && held.kind == TreatyKind::Trade
                && !held.expired(now)
        });
        if !rides {
            return Err(ActionRefused::AllowListNeedsTrade { deal });
        }
        self.element_allowlists
            .push((deal, partner.to_string(), elements.clone()));
        self.close(
            ClosureEvent::TreatyElementsAllowed {
                deal,
                partner: partner.to_string(),
                elements,
            },
            actor,
        );
        Ok(())
    }

    /// **E3.9 — IS THIS ELEMENT BARRED toward this partner?** The
    /// reading every element crossing consults, on BOTH books (the
    /// war wall's two-book shape). A list is LIVE only while the
    /// trade treaty it rides stands; while any live list stands on
    /// the pair, only the union of enumerated ids crosses. No live
    /// list ⇒ nothing is barred — the fence exists only where a pair
    /// co-signed one.
    pub fn element_barred(&self, partner: &str, element: &[u8; 32]) -> bool {
        let now = self.clock.now();
        let live: Vec<&Vec<[u8; 32]>> = self
            .element_allowlists
            .iter()
            .filter(|(deal, held, _)| {
                held == partner
                    && self.treaties.iter().any(|pact| {
                        pact.deal == *deal
                            && pact.partner == *held
                            && pact.kind == TreatyKind::Trade
                            && !pact.expired(now)
                    })
            })
            .map(|(_, _, elements)| elements)
            .collect();
        if live.is_empty() {
            return false;
        }
        !live.iter().any(|elements| elements.contains(element))
    }

    /// **RENOUNCE THIS WORLD'S OWN LEG.** Not a breach: a pact this
    /// world sealed, this world may end. What it cannot do is end the
    /// PARTNER's leg — that book is not ours to write, which is what
    /// makes the pair real rather than a convenience.
    pub fn renounce(&mut self, deal: u64, partner: &str) -> Result<(), ActionRefused> {
        self.renounce_as(SOVEREIGN, deal, partner)
    }

    pub fn renounce_as(
        &mut self,
        actor: u64,
        deal: u64,
        partner: &str,
    ) -> Result<(), ActionRefused> {
        self.permitted(actor, Scope::Build)?;
        if !self
            .treaties
            .iter()
            .any(|held| held.deal == deal && held.partner == partner)
        {
            return Err(ActionRefused::NoSuchTreaty { deal });
        }
        self.treaties
            .retain(|held| !(held.deal == deal && held.partner == partner));
        // E3.9: the annex dies with the pact it rides.
        self.element_allowlists
            .retain(|(held, other, _)| !(*held == deal && other == partner));
        self.close(
            ClosureEvent::TreatyStruck {
                deal,
                partner: partner.to_string(),
                broken: false,
            },
            actor,
        );
        Ok(())
    }

    /// The pacts this book holds that still STAND — expiry is read from
    /// the clock, never stored, so a lapsed pact needs no receipt to
    /// stop binding (the bond's own shape).
    pub fn treaties(&self) -> Vec<&Treaty> {
        let now = self.clock.now();
        self.treaties.iter().filter(|held| !held.expired(now)).collect()
    }

    /// **THE PAIR'S STANDING FACTOR ON ONE CHANNEL** — what this
    /// world's live pacts with `partner` make of it. One home; the
    /// melt and the reciprocity window both read it.
    pub fn pair_factor(&self, partner: &str, channel: u8) -> Ratio {
        crate::treaty::factor_on(&self.treaties, partner, channel, self.clock.now())
    }

    /// **IS THIS PAIR AT WAR?** The reading every crossing consults —
    /// a walled pair exchanges nothing, exactly as the elder's ward
    /// bars an aperture.
    pub fn at_war_with(&self, partner: &str) -> bool {
        crate::treaty::walled(&self.treaties, partner, self.clock.now())
    }
}
