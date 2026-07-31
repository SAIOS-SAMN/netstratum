//! THE KERNEL — the one validated update function (CHARTER §11 rule 3).
//!
//! `lawful(prior, next, step)` judges whether ONE step lawfully carries
//! the universe from `prior` to `next`. It is:
//!
//! - **pure** — same inputs, same verdict, forever;
//! - **total** — every input returns `Ok` or a typed `Unlawful`, never
//!   panics, never hangs;
//! - **no I/O** — it touches no file, clock, socket, or thread;
//! - **encoding-blind** — it never produces or reads bytes.
//!   Serialization is the codec's monopoly (the spine), so the kernel
//!   and the chain can never disagree about bytes: only one component
//!   ever produces them. The kernel judges MEANING; the codec fixes
//!   FORM; the two never overlap.
//!
//! Conservation, containment, and exactness are INVARIANTS inside this
//! function — an unlawful transition is not caught after the fact, it
//! cannot be constructed. This is why CHARTER §11 dissolves every
//! reconciliation instrument: you do not reconcile what cannot
//! diverge.
//!
//! **Versioned like the charter.** [`VERSION`] is bumped only by
//! amendment. Every chain commitment binds the version that judged the
//! step, so old history verifies under the judge it was cut with — no
//! future change reinterprets the past. The kernel never reads the
//! commitment; the spine binds `VERSION` into it.
//!
//! **Coverage grows by amendment, never silently.** Each event's rules
//! land as a witnessed addition (one witness per invariant, red-first);
//! CHARTER §11 clause 2 guards that the growth is amendment-shaped and
//! that no state semantics ever live outside this function.

use cfe::dop::{ClosureEvent, Resource};
use cfe::ipsal::Scope;
use cfe::ratio::{ratio, Ratio};
use cfe::world::Committed;
use solver_verify::{verify_work, Difficulty, WorkBroken};

/// The kernel version — bumped ONLY by charter amendment. The spine
/// binds this into every commitment (`H(schema ‖ bytes ‖ parent ‖
/// coordinate)` carries the judging version alongside the schema), so
/// a chain segment is forever verified under the version that cut it.
pub const VERSION: u64 = 1;

/// One proposed transition: the deed that CLAIMS to carry `prior` to
/// `next`. `event` is what closed, `actor` is who under NS-6
/// containment, `stamp` is the world-clock value it is stamped with.
/// The kernel judges whether the claim is lawful; it never applies it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub event: ClosureEvent,
    pub actor: u64,
    pub stamp: u64,
}

/// A refused transition — typed, carrying its measurement whole (the
/// house law: a refusal names its cure; a break shows its residual,
/// never a scalar). An `Ok(())` from [`lawful`] is the transition
/// admitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unlawful {
    /// A value-conserving step changed the total of a resource across
    /// the world's banks — value was minted or burned where the deed
    /// promised only to MOVE it. Both totals carried (boxed — the
    /// carrier is a big-int rational); the residual is theirs to read
    /// (shown, never summarized away).
    ConservationBroken { resource: Resource, before: Box<Ratio>, after: Box<Ratio> },
    /// I2 — the actor is not an entity in `prior`. Identity precedes the
    /// world (NS-6 §1.2): a deed from no one refuses before any state read.
    NoSuchEntity { actor: u64 },
    /// I2 — the actor is an entity but lacks the scope the deed requires.
    /// Containment: authority is granted, never assumed.
    NotPermitted { actor: u64, scope: Scope },
    /// I4 — a mint carried a negative amount: a deed labelled a creation
    /// that actually DESTROYS value (a burn wearing a mint's name). Value
    /// enters only by a lawful, non-negative mint.
    MintNotPositive { resource: Resource, amount: Box<Ratio> },
    /// I5 — a decay carried a negative amount: a deed labelled a decay
    /// that actually CREATES value. Value leaves only by a lawful,
    /// non-negative decay.
    DecayNotPositive { resource: Resource, amount: Box<Ratio> },
    /// THE SIGN LAW (the value-law wave): a value-carrying deed
    /// declared a NEGATIVE amount — a credit wearing a debit's name,
    /// or the reverse. Every banked movement is non-negative by
    /// declaration; DIRECTION comes from the deed, never the sign
    /// (a negative transfer would invert the flow and authorize the
    /// wrong side; a negative settle would credit through a debit).
    /// No un-guarded debit/credit path exists.
    AmountNotPositive { resource: Resource, amount: Box<Ratio> },
    /// I5 — the decay exceeds the sealed demurrage rule's due:
    /// `min(rate · ease · elapsed, banked)`, flat, thermally gated.
    /// Both sides carried whole (what the rule allows, what the deed
    /// claimed). Value leaves only by the flat law — never faster.
    DecayExceedsBound { bound: Box<Ratio>, claimed: Box<Ratio> },
    /// I4 — the discovery mint's proof failed the replay (form,
    /// training, output, or chain — the sealed verifier names the
    /// break; the kernel carries it whole, never summarizes).
    WorkRefused { broken: WorkBroken },
    /// I4 — the mint exceeds what verified work earns. Both sides
    /// carried whole (the bound the law allows, the amount claimed).
    /// While Ruling Box 1 stands unruled the bound is ZERO — the
    /// mechanism is sealed shut; every positive discovery mint
    /// refuses HERE, by law, until the operator opens the rate.
    MintExceedsWork { bound: Box<Ratio>, claimed: Box<Ratio> },
    /// I3 — a value in `next` is not a canonical exact rational (coprime,
    /// positive denominator). Exactness is a TYPE-LEVEL guarantee
    /// (`BigRational` is canonical by construction, no float ever entered);
    /// I3 is its sealed declaration and defense-in-depth at the byte
    /// boundary. The kernel CALLS the sealed `ratio::is_canonical`, never
    /// defining canonicity itself (the equations are shared, never copied
    /// — KERNEL CHARTER §0b).
    NotCanonical,
    /// THE SEALED DEFAULT — the judge refuses what no invariant covers.
    /// This event has no landed rule; a deed the kernel cannot judge does
    /// NOT enter. Coverage grows by amendment: each landing moves an event
    /// from this default-refuse arm to an explicit judged arm, and the
    /// kernel wires as the live gate (Phase 3) only when `Unjudged` is
    /// unreachable — every event covered. **SEAL A is a READER's law,
    /// never the judge's:** readers skip unknown TAGS so old software
    /// survives new content; a judge that skips unknown DEEDS is a hole
    /// exactly the size of every future tag. Two laws, one word "unknown"
    /// — conflating them silently rebuilds default-admit.
    Unjudged,
}

/// THE JUDGE. Does `step` lawfully carry `prior` to `next`? Pure,
/// total, encoding-blind, AND network-blind (it reads no mesh, no peer,
/// no uplink — `engine/kernel` imports `cfe` only; all arrows point one
/// way). `Ok(())` admits; `Err(Unlawful)` refuses with the break whole.
pub fn lawful(prior: &Committed, next: &Committed, step: &Step) -> Result<(), Unlawful> {
    // The sealed default arm (mesh-operator ruling, airtight-critical):
    // refuse-unknown, not admit-unknown. What no invariant covers does
    // not enter.
    if !covered(&step.event) {
        return Err(Unlawful::Unjudged);
    }
    conservation(prior, next, step)?;
    containment(prior, step)?;
    exactness(next)?;
    creation(step)?;
    decay(prior, step)
}

/// The landed coverage of the invariant set. An event is judged (not
/// refused `Unjudged`) only when EVERY invariant that should govern it has
/// landed — the completeness rule, so partial coverage never admits. The
/// universal trio I1/I2/I3 is now landed, so events whose full set is
/// {containment, exactness} — the pure authority deeds, moving no value —
/// are fully judged. Value-moving deeds beyond the transfer wait on I1's
/// coverage extension; minting/decaying deeds wait on I4/I5. The set grows
/// only by witnessed amendment (§11 clause 2); the kernel wires as the live
/// gate (Phase 3) when this predicate is total (every event covered).
fn covered(event: &ClosureEvent) -> bool {
    use ClosureEvent as E;
    matches!(
        event,
        // I1 conservation covers the intra-world move AND the aperture
        // legs (the convergence seam): a leg debits/credits exactly its
        // declared amount, the local half of pair conservation.
        E::ResourceTransferred { .. } | E::ApertureSent { .. } | E::ApertureReceived { .. }
        // I4 creation / I5 decay (declaration==effect + sign) fully cover
        // the mint and the demurrage decay.
        | E::YieldCollected { .. } | E::DemurrageSettled { .. }
        // U1 — the discovery mint's FULL set is landed: I1
        // (declaration==effect), I2 (Collect), I3, and I4 whole (sign
        // + the replay-complete verification + the sealed-shut bound).
        | E::DiscoveryMinted { .. }
        // I2 containment + I3 exactness fully cover the authority deeds
        // (no value moves, no ratios): registering an agent and opening
        // or shutting its scopes.
        | E::EntityRegistered { .. } | E::ScopeGranted { .. } | E::ScopeRevoked { .. }
        // W-A slice-1 — the BANKED-NEUTRAL authority deeds. Each one's
        // `world.rs::replay` arm touches NO bank (it sets a flag, a
        // seal, a list: terrain seed, star, planet site, practice,
        // ward, veil, genesis, prospect, the succession boundary, a
        // licence record, an instrument, a route, an offer, a bond).
        // So {I2 containment, I3 exactness} complete them, and
        // conservation's non-constraint is CORRECT — ratified against
        // the replay reading in `authority_pins.rs`. The value-moving
        // "authority" deeds (ColonyFounded/Charted, ZoneBuilt/Razed,
        // Covenant/Council seals — all move banks) stay held out for
        // W-A slice-2's I1 arms. TaxonSealed stays coder-03's #42.
        | E::WorldCharted { .. } | E::StarCommissioned { .. } | E::PlanetSited { .. }
        | E::PracticeAdopted { .. } | E::PracticeReleased { .. } | E::ElderWarded { .. }
        | E::VeilDrawn { .. } | E::WardLifted { .. } | E::VeilLifted { .. }
        | E::GenesisSealed { .. } | E::ProspectStruck { .. } | E::LineSucceeded { .. }
        | E::TechLicensed { .. } | E::InstrumentForged { .. } | E::RouteChartered { .. }
        | E::OfferPosted { .. } | E::BondSealed { .. }
        // W-C / W-D slice — the BANKED-NEUTRAL world-driven and mesh
        // deeds. Their `world.rs::replay` arms set a flag, a list, or
        // are no-ops (no bank); `scope_of` is None (a tick's own
        // consequence carries no actor scope). Judged by I3 exactness,
        // conservation and containment vacuous — ratified against the
        // replay reading in `world_driven_pins.rs`. HELD OUT: the
        // value-movers of these families (FamineStruck −drawn,
        // CovenantSlashed +collateral, CouncilClosed +banked → the I1
        // value slice) and the FOLD TRIO (Confluent/EraAnnexed/
        // EraFolded → U2d's I6 gate; a fold is never admitted on
        // exactness alone).
        | E::FamineRelieved { .. } | E::GalaxyObserved { .. } | E::OverheatStruck { .. }
        | E::OverheatCleared { .. } | E::VentSettled { .. } | E::AbsorptionLapsed { .. }
        | E::WantStruck { .. } | E::WantRelieved { .. } | E::HandoffSealed { .. }
        | E::GaugeShifted { .. } | E::Diverged { .. } | E::StarAged { .. }
        | E::TechDiscovered { .. } | E::IncursionStruck { .. } | E::VoidShare { .. }
        | E::RouteLapsed { .. }
        // THE VALUE LAW WAVE (operator directive 2026-07-31): the
        // value-moving deeds under TOTAL conservation + THE SIGN LAW,
        // deltas declared per-resource in `lawful_deltas` (the sealed
        // rules called, stateful reads from prior). Witnessed in
        // `value_pins.rs`. Tier 1 (single-resource, sign-guarded),
        // Tier 2 (escrow returns, declaration==effect — the
        // EscrowTotal + BankedTotal macro-invariant strengthens in a
        // follow-up), Tier 3 (multi-resource vectors). STILL HELD OUT:
        // the fold trio (U2d's I6 gate), TaxonSealed (coder-03's #42),
        // AssayStruck (the element ledger's accounting, the almanac's
        // follow-up — an element mint is not admitted on
        // banked-neutrality).
        | E::UpkeepSettled { .. } | E::CrystalDrawn { .. } | E::OverflowAbsorbed { .. }
        | E::BondSettled { .. } | E::FamineStruck { .. } | E::CovenantSealed { .. }
        | E::CouncilSealed { .. } | E::FrictionSettled { .. } | E::OfferCleared { .. }
        | E::CovenantReleased { .. } | E::CovenantSlashed { .. } | E::CouncilClosed { .. }
        | E::ColonyFounded { .. } | E::ColonyCharted { .. } | E::ZoneBuilt { .. }
        | E::ZoneRazed { .. }
    )
}

/// INVARIANT — conservation, TOTAL (the value-law wave, operator
/// directive 2026-07-31). A deed changes the world's banked totals by
/// exactly its DECLARED per-resource deltas and nothing else:
///
/// ```text
/// ∀ r ∈ Resources:  total(next, r) == total(prior, r) + declared(r)
/// ```
///
/// Every resource present in either state or in the declaration is
/// checked — a covered deed with an EMPTY declaration must leave EVERY
/// bank untouched (a smuggled change under a neutral deed refuses).
/// Multi-resource deeds (founding reserves, zone costs and refunds)
/// declare vectors, never a scalar. The PAIR-level law (a far leg is
/// the near leg's exact inverse) is the cross-node opposition (KERNEL
/// CHARTER §5a); the kernel enforces the LOCAL half.
fn conservation(prior: &Committed, next: &Committed, step: &Step) -> Result<(), Unlawful> {
    let deltas = lawful_deltas(prior, &step.event)?;
    let mut resources: Vec<Resource> = Vec::new();
    for (resource, _) in &deltas {
        if !resources.contains(resource) {
            resources.push(*resource);
        }
    }
    for orb in prior.orbs.iter().chain(next.orbs.iter()) {
        for (resource, _) in &orb.banked {
            if !resources.contains(resource) {
                resources.push(*resource);
            }
        }
    }
    for resource in resources {
        let declared = deltas
            .iter()
            .filter(|(held, _)| *held == resource)
            .fold(ratio(0, 1), |sum, (_, delta)| sum + delta.clone());
        let before = total_banked(prior, resource);
        let after = total_banked(next, resource);
        let expected = before + declared;
        if after != expected {
            return Err(Unlawful::ConservationBroken {
                resource,
                before: Box::new(expected), // the lawful total this deed requires
                after: Box::new(after),     // the total the step actually produced
            });
        }
    }
    Ok(())
}

/// The declared per-resource banked deltas of a deed — the LAWFUL
/// movement, sign-guarded at the door (THE SIGN LAW: every amount is
/// non-negative by declaration; direction comes from the DEED). The
/// sealed rules are CALLED, never copied (founding reserves, zone
/// costs/refunds — CHARTER §0b). Stateful declarations (the council's
/// resource, the razed zone's kind) read PRIOR state, never the event.
/// An event with no arm declares nothing: every bank must hold still.
fn lawful_deltas(
    prior: &Committed,
    event: &ClosureEvent,
) -> Result<Vec<(Resource, Ratio)>, Unlawful> {
    use ClosureEvent as E;
    let non_negative = |resource: Resource, amount: &Ratio| match *amount < ratio(0, 1) {
        true => Err(Unlawful::AmountNotPositive {
            resource,
            amount: Box::new(amount.clone()),
        }),
        false => Ok(()),
    };
    Ok(match event {
        // — the moves (net zero), sign-guarded: a negative move would
        //   invert the flow and authorize the wrong side —
        E::ResourceTransferred { resource, amount, .. } => {
            non_negative(*resource, amount)?;
            Vec::new()
        }
        E::BondSettled { resource, amount, .. } => {
            non_negative(*resource, amount)?;
            Vec::new()
        }
        // ColonyCharted: the founding reserves MOVE source → daughter —
        // net zero per resource by the sealed rule.
        E::ColonyCharted { .. } => Vec::new(),
        // — the aperture legs, sign-guarded (a negative send is a
        //   receive wearing a send's name) —
        E::ApertureSent { resource, amount, .. } => {
            non_negative(*resource, amount)?;
            vec![(*resource, -amount.clone())]
        }
        E::ApertureReceived { resource, amount, .. } => {
            non_negative(*resource, amount)?;
            vec![(*resource, amount.clone())]
        }
        // — mints (the sign lives at I4, order preserved for the
        //   landed pins) —
        E::YieldCollected { resource, amount, .. } => vec![(*resource, amount.clone())],
        E::DiscoveryMinted { resource, amount, .. } => vec![(*resource, amount.clone())],
        // — decays (the sign lives at I5, order preserved) —
        E::DemurrageSettled { resource, amount, .. } => vec![(*resource, -amount.clone())],
        // — Tier 1: single-resource debits, sign-guarded —
        E::UpkeepSettled { resource, amount, .. } => {
            non_negative(*resource, amount)?;
            vec![(*resource, -amount.clone())]
        }
        E::CrystalDrawn { amount, .. } => {
            non_negative(Resource::Crystal, amount)?;
            vec![(Resource::Crystal, -amount.clone())]
        }
        E::OverflowAbsorbed { amount, .. } => {
            non_negative(Resource::Sink, amount)?;
            vec![(Resource::Sink, -amount.clone())]
        }
        E::FamineStruck { resource, drawn, .. } => {
            non_negative(*resource, drawn)?;
            vec![(*resource, -drawn.clone())]
        }
        E::CovenantSealed { resource, collateral, .. } => {
            non_negative(*resource, collateral)?;
            vec![(*resource, -collateral.clone())]
        }
        // The council locks collateral × EVERY member (multi-orb,
        // single-resource).
        E::CouncilSealed { resource, collateral, members, .. } => {
            non_negative(*resource, collateral)?;
            vec![(*resource, -(collateral.clone() * ratio(members.len() as i64, 1)))]
        }
        // — Tier 2: escrow returns —
        // INVARIANT SHAPE: EscrowTotal + BankedTotal = ConservedState —
        // declaration==effect this pass; the stateful escrow
        // cross-lookup strengthens in a follow-up (operator directive).
        E::CovenantReleased { resource, collateral, .. }
        | E::CovenantSlashed { resource, collateral, .. } => {
            non_negative(*resource, collateral)?;
            vec![(*resource, collateral.clone())]
        }
        // The close is STATEFUL: the resource reads from the PRIOR
        // sealed council, never the event. An unknown council declares
        // nothing — a banked change under it refuses.
        E::CouncilClosed { council, banked, .. } => {
            match prior.councils.iter().find(|held| held.council == *council) {
                Some(held) => {
                    non_negative(held.resource, banked)?;
                    vec![(held.resource, banked.clone())]
                }
                None => Vec::new(),
            }
        }
        // — Tier 3: multi-resource, the sealed rules called never
        //   copied —
        // The chartered genesis mint: EXACTLY the founding reserves.
        E::ColonyFounded { .. } => cfe::dop::founding_reserves(),
        E::ZoneBuilt { kind, .. } => {
            kind.cost().into_iter().map(|(resource, cost)| (resource, -cost)).collect()
        }
        // The raze refunds the PRIOR zone's kind at the tile (stateful
        // — the event names only the tile). No zone, no refund.
        E::ZoneRazed { orb_id, tile } => prior
            .orbs
            .iter()
            .find(|held| held.orb_id == *orb_id)
            .and_then(|orb| orb.zones.iter().find(|zone| zone.tile == *tile))
            .map(|zone| zone.kind.raze_refund())
            .unwrap_or_default(),
        // No arm: the deed declares NO banked movement — the total
        // check holds every bank still.
        _ => Vec::new(),
    })
}

/// The world's total banked holding of one resource, summed across
/// every orb. Pure; the canonical ledger drops zero entries, so the
/// sum is over what is actually held.
fn total_banked(state: &Committed, resource: Resource) -> Ratio {
    state
        .orbs
        .iter()
        .map(|orb| orb.banked_amount(resource))
        .fold(ratio(0, 1), |sum, held| sum + held)
}

/// INVARIANT — bounded creation (I4). Value is created ONLY by a lawful
/// mint, and the mint is non-negative (a mint that destroys value is a
/// burn mislabelled — refused). The declaration==effect half is
/// conservation (the world total rose by exactly the declared amount);
/// I4 adds the sign. NAMED SEAM: at convergence scale the mint is bounded
/// by verified work — `mint ≤ verify_certificate(work)` — the PoUW anchor
/// (KERNEL CHARTER §5a), which fills on Ruling Box 1. No fiat (05 §1.2).
fn creation(step: &Step) -> Result<(), Unlawful> {
    match &step.event {
        ClosureEvent::YieldCollected { resource, amount, .. } if *amount < ratio(0, 1) => {
            Err(Unlawful::MintNotPositive { resource: *resource, amount: Box::new(amount.clone()) })
        }
        // U1 — I4 completes on the discovery mint: sign, then the
        // REPLAY-COMPLETE verification (the kernel hands the opaque
        // payload to the sealed verifier whole, never parses it),
        // then the bound. No certificate → no mint; a mint past what
        // the work earns refuses with both sides carried.
        ClosureEvent::DiscoveryMinted { resource, amount, work, .. } => {
            if *amount < ratio(0, 1) {
                return Err(Unlawful::MintNotPositive {
                    resource: *resource,
                    amount: Box::new(amount.clone()),
                });
            }
            let difficulty =
                verify_work(work).map_err(|broken| Unlawful::WorkRefused { broken })?;
            let bound = mint_bound(&difficulty);
            match *amount <= bound {
                true => Ok(()),
                false => Err(Unlawful::MintExceedsWork {
                    bound: Box::new(bound),
                    claimed: Box::new(amount.clone()),
                }),
            }
        }
        _ => Ok(()),
    }
}

/// THE MINT BOUND — SEALED SHUT (U_DIRECTIVES §8, Ruling Box 1). The
/// rate `f`, the admissible work-shapes, and the reward split are
/// OPERATOR WORDS; until that amendment lands (a kernel VERSION
/// bump), verified work earns a bound of ZERO and every positive
/// discovery mint refuses at I4. The mechanism precedes the
/// economics by law — rule `f` against the observed chained economy,
/// never assumption.
fn mint_bound(_difficulty: &Difficulty) -> Ratio {
    ratio(0, 1)
}

/// INVARIANT — bounded decay (I5), COMPLETE (U1f). Value is destroyed
/// non-negative decay (a decay that creates value is refused). The
/// declaration==effect half is conservation (the world total fell by
/// exactly the declared amount); I5 adds the sign. NAMED SEAM: the decay
/// is FLAT and denominator-bounded, `decay ≤ rate · ease · elapsed`
/// (never proportional — the 2^1100 lesson), which fills when the sealed
/// demurrage rule + `last_touch` join the check.
fn decay(prior: &Committed, step: &Step) -> Result<(), Unlawful> {
    match &step.event {
        ClosureEvent::DemurrageSettled { orb_id, resource, amount } => {
            if *amount < ratio(0, 1) {
                return Err(Unlawful::DecayNotPositive {
                    resource: *resource,
                    amount: Box::new(amount.clone()),
                });
            }
            // I5's bound (U1f): THE SEALED DEMURRAGE RULE, called
            // never copied (CHARTER §0b) — one computation, two
            // judges. `elapsed` reads the prior orb's `last_touch`,
            // already committed on the orb wire (tag 28); an orb the
            // rule owes nothing (no leak, no bank, no time) bounds
            // at zero, so only the zero decay is lawful there.
            let due = prior
                .orbs
                .iter()
                .find(|held| held.orb_id == *orb_id)
                .and_then(|orb| {
                    cfe::world::demurrage_due(
                        orb,
                        prior.star,
                        step.stamp,
                        &cfe::world::ease_of(&prior.techs, *orb_id),
                    )
                    .into_iter()
                    .find(|(held, _)| held == resource)
                    .map(|(_, held)| held)
                })
                .unwrap_or_else(|| ratio(0, 1));
            match *amount <= due {
                true => Ok(()),
                false => Err(Unlawful::DecayExceedsBound {
                    bound: Box::new(due),
                    claimed: Box::new(amount.clone()),
                }),
            }
        }
        _ => Ok(()),
    }
}

/// INVARIANT — exactness (I3). Every value in `next` is a canonical exact
/// rational. Exactness is primarily a TYPE-LEVEL guarantee — `Ratio` is
/// `BigRational`, canonical by construction, and no float exists anywhere
/// in the codebase — so this passes for every well-typed state; I3 is the
/// sealed DECLARATION of that law plus defense-in-depth over the value
/// ledgers (the banked resources and elements most likely to carry a
/// decoded value). It CALLS the sealed `ratio::is_canonical` (KERNEL
/// CHARTER §0b: the equation is shared, never reimplemented in the judge).
fn exactness(next: &Committed) -> Result<(), Unlawful> {
    for orb in &next.orbs {
        let banked = orb.banked.iter().map(|(_, amount)| amount);
        let elements = orb.elements.iter().map(|(_, amount)| amount);
        for value in banked.chain(elements) {
            if !cfe::ratio::is_canonical(value) {
                return Err(Unlawful::NotCanonical);
            }
        }
    }
    Ok(())
}

/// INVARIANT — containment (I2). An actor-deed's actor must be an entity
/// in `prior` that holds the scope the deed requires. Identity precedes
/// the world: a deed from no one refuses before any state is read. A
/// sovereign holds every scope by construction; an agent holds exactly
/// its granted scopes. World-driven events (consequences the tick
/// computes, no actor) carry no scope requirement (`scope_of` = None).
fn containment(prior: &Committed, step: &Step) -> Result<(), Unlawful> {
    let Some(scope) = scope_of(&step.event) else {
        return Ok(());
    };
    let entity = prior
        .entities
        .iter()
        .find(|held| held.entity_id == step.actor)
        .ok_or(Unlawful::NoSuchEntity { actor: step.actor })?;
    match entity.permits(scope) {
        true => Ok(()),
        false => Err(Unlawful::NotPermitted { actor: step.actor, scope }),
    }
}

/// The sealed event → scope law (a CFE transition rule extracted into the
/// kernel). EXHAUSTIVE by design: adding a `ClosureEvent` variant forces a
/// scope decision here at compile time — the amendment discipline made
/// structural. `None` = a world-driven consequence with no actor scope, or
/// a mesh/universe-ledger deed governed by its own law (never a hole: an
/// uncovered event refuses `Unjudged` before this is consulted). Each arm
/// is RATIFIED by a red-first witness as its event enters `covered()`
/// against `world.rs`; today only `ResourceTransferred` (Collect) is
/// covered and witnessed. The rest are the drafted law, unenforced until
/// their coverage lands.
fn scope_of(event: &ClosureEvent) -> Option<Scope> {
    use ClosureEvent as E;
    match event {
        // Build — create or extend the world, or open authority.
        E::ColonyFounded { .. } | E::ZoneBuilt { .. } | E::WorldCharted { .. }
        | E::EntityRegistered { .. } | E::ScopeGranted { .. } | E::ScopeRevoked { .. }
        | E::ColonyCharted { .. } | E::StarCommissioned { .. } | E::PlanetSited { .. }
        | E::BondSealed { .. } | E::CovenantSealed { .. } | E::CouncilSealed { .. }
        | E::GaugeShiftedAt { .. } | E::TechLicensed { .. } | E::GenesisSealed { .. }
        | E::PracticeAdopted { .. } | E::PracticeReleased { .. } | E::ProspectStruck { .. }
        | E::ElderWarded { .. } | E::VeilDrawn { .. } | E::InstrumentForged { .. }
        | E::RouteChartered { .. } | E::OfferPosted { .. } | E::LineSucceeded { .. }
        // KERNEL-TAXON: the drafted Build-class arm coder-03 named for
        // their #42 — unenforced until their covered() entry + witness.
        | E::TaxonSealed { .. } => {
            Some(Scope::Build)
        }
        // Raze — demolish or lift what was raised.
        E::ZoneRazed { .. } | E::WardLifted { .. } | E::VeilLifted { .. } => Some(Scope::Raze),
        // Collect — settle or move banked value.
        E::YieldCollected { .. } | E::UpkeepSettled { .. } | E::ResourceTransferred { .. }
        | E::OverflowAbsorbed { .. } | E::BondSettled { .. } | E::CovenantReleased { .. }
        | E::FrictionSettled { .. } | E::ApertureSent { .. } | E::ApertureReceived { .. }
        | E::CrystalDrawn { .. } | E::AssayStruck { .. } | E::OfferCleared { .. }
        // U1: the discovery mint settles banked value against verified
        // work — Collect, like the yield it banks beside. Ratified by
        // the U1 witnesses as it enters covered().
        | E::DiscoveryMinted { .. } => {
            Some(Scope::Collect)
        }
        // None — world-driven consequences (empty provenance), mesh deeds,
        // and universe-ledger entries: no actor scope. Governed by other
        // invariants (conservation, decay) or their own protocol law.
        E::FamineStruck { .. } | E::FamineRelieved { .. } | E::EraAnnexed { .. }
        | E::GalaxyObserved { .. } | E::OverheatStruck { .. } | E::OverheatCleared { .. }
        | E::VentSettled { .. } | E::DemurrageSettled { .. } | E::AbsorptionLapsed { .. }
        | E::CovenantSlashed { .. } | E::WantStruck { .. } | E::WantRelieved { .. }
        | E::HandoffSealed { .. } | E::GaugeShifted { .. } | E::Diverged { .. }
        | E::Confluent { .. } | E::StarAged { .. } | E::EraFolded { .. }
        | E::CouncilClosed { .. } | E::TechDiscovered { .. } | E::IncursionStruck { .. }
        | E::VoidShare { .. } | E::RouteLapsed { .. } => None,
    }
}
