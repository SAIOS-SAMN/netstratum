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
    exactness(next)
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
        // I2 containment + I3 exactness fully cover the authority deeds
        // (no value moves, no ratios): registering an agent and opening
        // or shutting its scopes.
        | E::EntityRegistered { .. } | E::ScopeGranted { .. } | E::ScopeRevoked { .. }
    )
}

/// INVARIANT — conservation. A deed changes the world's total of a
/// resource by exactly its LAWFUL DELTA and no more: a pure intra-world
/// move conserves (delta 0 — `from_orb` loses what `to_orb` gains); an
/// aperture leg debits or credits exactly its declared amount (value that
/// crossed to or from ANOTHER world). Value minted or burned beyond the
/// lawful delta refuses with the expected and actual totals whole. The
/// PAIR-level law (a far leg is the near leg's exact inverse) is the
/// cross-node opposition (KERNEL CHARTER §5a); the kernel enforces the
/// LOCAL half — each leg moved exactly what it claimed.
fn conservation(prior: &Committed, next: &Committed, step: &Step) -> Result<(), Unlawful> {
    use ClosureEvent as E;
    // The lawful change to the world's total of `resource` for this deed.
    let (resource, lawful_delta) = match &step.event {
        E::ResourceTransferred { resource, .. } => (*resource, ratio(0, 1)),
        // The source leg: value LEFT this world for another (debit).
        E::ApertureSent { resource, amount, .. } => (*resource, -amount.clone()),
        // The destination leg: value ARRIVED from another world (credit).
        E::ApertureReceived { resource, amount, .. } => (*resource, amount.clone()),
        // A covered event conservation does not constrain (uncovered
        // events already refused Unjudged; other invariants govern this).
        // Coverage grows by witnessed amendment — clause 2.
        _ => return Ok(()),
    };
    let before = total_banked(prior, resource);
    let after = total_banked(next, resource);
    let expected = before + lawful_delta;
    match after == expected {
        true => Ok(()),
        false => Err(Unlawful::ConservationBroken {
            resource,
            before: Box::new(expected), // the lawful total this deed requires
            after: Box::new(after),     // the total the step actually produced
        }),
    }
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
        | E::RouteChartered { .. } | E::OfferPosted { .. } | E::LineSucceeded { .. } => {
            Some(Scope::Build)
        }
        // Raze — demolish or lift what was raised.
        E::ZoneRazed { .. } | E::WardLifted { .. } | E::VeilLifted { .. } => Some(Scope::Raze),
        // Collect — settle or move banked value.
        E::YieldCollected { .. } | E::UpkeepSettled { .. } | E::ResourceTransferred { .. }
        | E::OverflowAbsorbed { .. } | E::BondSettled { .. } | E::CovenantReleased { .. }
        | E::FrictionSettled { .. } | E::ApertureSent { .. } | E::ApertureReceived { .. }
        | E::CrystalDrawn { .. } | E::AssayStruck { .. } | E::OfferCleared { .. } => {
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
