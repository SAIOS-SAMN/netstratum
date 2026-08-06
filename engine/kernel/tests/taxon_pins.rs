//! THE LAST DEED — TaxonSealed enters coverage, and with it the
//! kernel is TOTAL: every one of the 64 deeds is gated, `Unjudged`
//! unreachable via any current deed (the Phase 3 precondition).
//!
//! The completeness witness enumerates ALL 64 variants and asserts
//! none is refused `Unjudged` — the derived guard the founding lesson
//! demands (EQUATIONS §0.3 chapter three: a count that balances
//! against an uncounted universe is the failure mode; this counts the
//! universe). It is the standing test that would have caught the
//! missed sixth on sight.

use cfe::dop::{ClosureEvent, ColonyOrb, Resource, ZoneKind};
use cfe::ipsal::{Entity, Scope};
use cfe::ratio::ratio;
use cfe::world::{Committed, World, SOVEREIGN};
use kernel::{admits, Step, Refused};

fn seated() -> Committed {
    let mut committed = World::new().committed();
    let mut orb = ColonyOrb { orb_id: 1, name: "site".into(), ..Default::default() };
    orb.bank(Resource::Iron, ratio(6, 1));
    committed.orbs = vec![orb];
    committed.entities = vec![
        Entity { entity_id: SOVEREIGN, handle: "sovereign".into(), principal: None, grants: vec![] },
        Entity { entity_id: 100, handle: "hand".into(), principal: Some(SOVEREIGN), grants: vec![] },
    ];
    committed
}

fn step(event: ClosureEvent, actor: u64) -> Step {
    Step { event, actor, stamp: 1 }
}

#[test]
fn taxon_sealed_gates_by_the_build_scope() {
    // The seal moves no bank, no element, no key — {I2, I3} complete
    // it. The sovereign holds Build: admitted (prior == next, neutral).
    let state = seated();
    let seal = ClosureEvent::TaxonSealed { class: 1, clade: 1 };
    assert_eq!(admits(&state, &state, &step(seal.clone(), SOVEREIGN)), Ok(()));
    // A bare agent lacks Build: refused by containment.
    assert_eq!(
        admits(&state, &state, &step(seal.clone(), 100)),
        Err(Refused::NotPermitted { actor: 100, scope: Scope::Build }),
    );
    // A deed from no one: refused before any state read.
    assert_eq!(
        admits(&state, &state, &step(seal, 999)),
        Err(Refused::NoSuchEntity { actor: 999 }),
    );
}

/// One instance of EVERY `ClosureEvent` variant. When the enum grows,
/// its `from_wire`/`scope_of` arms force the author to the new variant
/// elsewhere; this list is the coverage side of that same discipline.
fn every_deed() -> Vec<ClosureEvent> {
    use ClosureEvent as E;
    let r = |n: i64| ratio(n, 1);
    let bytes = [0u8; 32];
    vec![
        E::ColonyFounded { orb_id: 1, name: "c".into() },
        E::ZoneBuilt { orb_id: 1, tile: 0, kind: ZoneKind::Extractor },
        E::YieldCollected { orb_id: 1, resource: Resource::Iron, amount: r(1) },
        E::UpkeepSettled { orb_id: 1, resource: Resource::Iron, amount: r(1) },
        E::FamineStruck { orb_id: 1, resource: Resource::Iron, drawn: r(1) },
        E::FamineRelieved { orb_id: 1, resource: Resource::Iron },
        E::ZoneRazed { orb_id: 1, tile: 0 },
        E::WorldCharted { seed: bytes },
        E::EntityRegistered { entity_id: 2, principal: 0, handle: "e".into() },
        E::ScopeGranted { agent: 2, scope: Scope::Collect },
        E::ScopeRevoked { agent: 2, scope: Scope::Collect },
        E::ColonyCharted { orb_id: 2, source_orb: 1, name: "d".into() },
        E::EraAnnexed { era: "e1".into(), world: "w".into(), terminal: bytes, offset: 1 },
        E::GalaxyObserved {
            observer: "o".into(),
            creator: "c".into(),
            name: "g".into(),
            center: (r(0), r(0), r(0)),
            reach: r(1),
        },
        E::OverheatStruck { orb_id: 1, overflow: r(1) },
        E::OverheatCleared { orb_id: 1 },
        E::StarCommissioned { class: 5 },
        E::PlanetSited { orb_id: 1, albedo: cfe::dop::Unit::new(r(0)).expect("unit"), orbit_r: cfe::dop::Positive::new(r(1)).expect("positive") , theta: None, psi: None },
        E::VentSettled { orb_id: 1, amount: r(1) },
        E::DemurrageSettled { orb_id: 1, resource: Resource::Iron, amount: r(1) },
        E::ResourceTransferred { from_orb: 1, to_orb: 2, resource: Resource::Iron, amount: r(1) },
        E::OverflowAbsorbed { source_orb: 1, absorber_orb: 2, amount: r(1) },
        E::AbsorptionLapsed { debtor_orb: 1, creditor_orb: 2, amount: r(1) },
        E::BondSealed { source_orb: 1, target_orb: 2, resource: Resource::Iron, rate: r(1), term: 0 },
        E::BondSettled { source_orb: 1, target_orb: 2, resource: Resource::Iron, amount: r(1), severed: false, load: r(0) },
        E::ForwardSealed { obligor_orb: 1, beneficiary_orb: 2, resource: Resource::Iron, quota: r(4), window: 8, collateral: r(1) },
        E::ForwardReleased { obligor_orb: 1, beneficiary_orb: 2, resource: Resource::Iron, collateral: r(1) },
        E::ForwardSlashed { obligor_orb: 1, beneficiary_orb: 2, resource: Resource::Iron, collateral: r(1) },
        E::FrictionSettled { orb_id: 1, amount: r(1) },
        E::WantStruck { orb_id: 1, kind: 2, reading: r(1) },
        E::WantRelieved { orb_id: 1, kind: 2 },
        E::HandoffSealed { from: 1, to: 2 },
        E::OfferPosted { offer: 1, orb_id: 1, give: Resource::Iron, give_amount: r(1), want: Resource::Biomass, want_amount: r(1) },
        E::OfferCleared { offer: 1, deal: 1 },
        E::GaugeShiftedAt { grade: 1, gauge: r(0) },
        E::GaugeShifted { gauge: r(0) },
        E::Diverged { branch: "b".into(), gauge: r(0) },
        E::Confluent { partner: "p".into(), gauge: r(0) },
        E::StarAged { class: 5 },
        E::EraFolded { era: 1, gauge: r(0) },
        E::TechLicensed { deal: 1, from_orb: 1, council: 1, condition: 1, node: bytes, name: "t".into() },
        E::CouncilSealed { council: 1, resource: Resource::Iron, quota: r(4), window: 8, collateral: r(1), members: vec![1, 2] },
        E::CouncilClosed { council: 1, member: 1, fulfilled: true, banked: r(0) },
        E::TechDiscovered { orb_id: 1, node: bytes, condition: 1, modifier: 0, magnitude: r(1), name: "t".into() },
        E::ApertureSent { deal: 1, from_orb: 1, resource: Resource::Iron, amount: r(1) },
        E::ApertureReceived { deal: 1, to_orb: 1, resource: Resource::Iron, amount: r(1) },
        E::CrystalDrawn { orb_id: 1, amount: r(1) },
        E::LineSucceeded { successor: "s".into() },
        E::GenesisSealed { civilization: "c".into(), disposition: 1, metabolism: 1, perception: 1 },
        E::PracticeAdopted { practice: 1 },
        E::PracticeReleased { practice: 1 },
        E::ProspectStruck { orb_id: 1, stratum: 0, element: bytes, family: 1, tier: 1, richness: r(1), name: "p".into() },
        E::AssayStruck { orb_id: 1, element: bytes, amount: r(1), richness: None },
        E::ElderWarded {},
        E::WardLifted { claimer: "w".into() },
        E::VeilDrawn {},
        E::VeilLifted { starved: false },
        E::IncursionStruck { era: "e".into(), world: "w".into(), line: "l".into() },
        E::TaxonSealed { class: 1, clade: 1 },
        E::DiscoveryMinted { orb_id: 1, resource: Resource::Iron, amount: r(1), work: Vec::new() },
        E::InstrumentForged { orb_id: 1, tech_hash: bytes, grade: 1 },
        E::VoidShare { orb_id: 1, deal: 1, resource: Resource::Iron, amount: r(1) },
        E::RouteChartered { deal: 1, eta: cfe::dop::unit(0, 1) },
        E::RouteLapsed { deal: 1 },
    ]
}

#[test]
fn coverage_is_total_the_sealed_default_is_unreachable() {
    // THE PHASE 3 PRECONDITION as a standing test (supersedes the old
    // sealed-default pin now that the last deed is covered): every one
    // of the 64 deeds is gated — none refuses Unjudged. A deed may
    // refuse for its OWN law (a value-mover over prior==next breaks
    // conservation; an authority deed from a bare hand refuses
    // NotPermitted) — that is a JUDGED verdict. Only Unjudged means a
    // coverage hole.
    let deeds = every_deed();
    assert_eq!(deeds.len(), 64, "one instance of every ClosureEvent variant");
    let state = seated();
    for event in deeds {
        let verdict = admits(&state, &state, &step(event.clone(), SOVEREIGN));
        assert_ne!(
            verdict,
            Err(Refused::Unjudged),
            "coverage hole — {event:?} refuses Unjudged; every deed must be judged"
        );
    }
}
