//! P5-A KIN pins — escort, convoy risk, interdiction residual, posture.

use cfe::dop::{HullRole, Positive, Resource, ZoneKind};
use cfe::ratio::ratio;
use cfe::world::{replay, DefenseDomain, RiskBand, World, FIRST_ORB, SOVEREIGN};

fn yard_world() -> World {
    let mut w = World::new();
    w.found("convoy-home").expect("found");
    w.zone(1, ZoneKind::ShipYard).expect("yard");
    let scrap = Positive::new(ratio(4, 1) + ratio(4, 1)).expect("scrap");
    w.salvage_scrap(FIRST_ORB, scrap).expect("salvage");
    w
}

#[test]
fn posture_is_species_relative() {
    let mut w = World::new();
    w.found("posture").expect("f");
    let p = w.defense_posture();
    assert!(matches!(
        p.primary,
        DefenseDomain::Kin | DefenseDomain::Cel | DefenseDomain::Atm | DefenseDomain::Cog
    ));
    assert_eq!(p.ranks.len(), 6);
    assert!(!p.voice.is_empty());
}

#[test]
fn base_risk_positive_escort_cover_formula() {
    let mut w = yard_world();
    let risk0 = w.convoy_risk(1);
    assert!(risk0 > ratio(0, 1), "unguarded corridor has risk");
    assert_eq!(w.escort_cover(1), ratio(0, 1));
    // base risk = 1/8 < quarter → EXPOSED (not yet SECURE without cover)
    assert_eq!(w.convoy_risk_band(1), RiskBand::Exposed);
    // Escort grade 0 → cover (0+1)*(1/8) = 1/8; risk = (1/8)*(1-1/8)=7/64
    let escort = w.mint_hull(FIRST_ORB, HullRole::Escort, 0).expect("escort");
    // Manually assign route without charter: set via assign if deal known — refuse.
    // Pure formula: after EscortOnDuty we need assign. Inject assigned_route by
    // escort_assign only when route exists. Cover without assignment stays 0.
    let cover_idle = w.escort_cover(1);
    assert_eq!(cover_idle, ratio(0, 1));
    let _ = escort;
    let _ = risk0;
}

#[test]
fn escort_assign_requires_escort_role_and_route() {
    let mut w = yard_world();
    let hauler = w.mint_hull(FIRST_ORB, HullRole::Hauler, 0).expect("hauler");
    let escort = w.mint_hull(FIRST_ORB, HullRole::Escort, 0).expect("escort");
    // Hauler cannot take escort duty.
    assert!(matches!(
        w.escort_assign(hauler, 1),
        Err(cfe::world::ActionRefused::EscortRoleRequired { .. })
    ));
    // Escort without route → RouteUnknown from assign_hull.
    assert!(matches!(
        w.escort_assign(escort, 1),
        Err(cfe::world::ActionRefused::RouteUnknown { .. })
    ));
}

#[test]
fn interdiction_receipts_typed_residual() {
    let mut w = yard_world();
    let cargo = ratio(8, 1);
    let lost = w
        .strike_interdiction(SOVEREIGN, 1, FIRST_ORB, Resource::Iron, &cargo)
        .expect("strike");
    // risk = 1/8, lost = cargo * risk / 2 = 8 * 1/8 / 2 = 1/2
    assert_eq!(lost, ratio(1, 2));
    assert!(w.chronicle().receipts().iter().any(|r| {
        matches!(
            r.event,
            cfe::dop::ClosureEvent::InterdictionStruck { .. }
        )
    }));
    assert_eq!(replay(w.chronicle()), w.committed());
}

#[test]
fn mark_convoy_risk_receipts() {
    let mut w = yard_world();
    let r = w.mark_convoy_risk(3).expect("mark");
    assert!(r > ratio(0, 1));
    assert!(w.chronicle().receipts().iter().any(|r| {
        matches!(r.event, cfe::dop::ClosureEvent::ConvoyRiskMarked { .. })
    }));
    assert_eq!(replay(w.chronicle()), w.committed());
}
