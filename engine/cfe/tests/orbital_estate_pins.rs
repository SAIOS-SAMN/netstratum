//! 35_ORBIT_DYNAMO_AND_ESTATE — orbital place + stratum appraisal.

use cfe::dop::{Holding, Positive, Slot, Unit, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{core_magnetism, orbit_phase_period, replay, World, FIRST_ORB};

fn sited_surveyed() -> World {
    let mut w = World::new();
    w.found("estate-orb").expect("found");
    w.commission_star(StarClass::G).expect("star");
    let a = Unit::new(ratio(0, 1)).expect("A");
    let r = Positive::new(ratio(1, 1)).expect("r");
    w.site_planet(FIRST_ORB, a, r, Some(Slot::from_index(0)), None)
        .expect("site");
    w.zone(0, ZoneKind::Extractor).expect("zone");
    for _ in 0..8 {
        w.tick();
    }
    let _ = w.collect();
    w.survey_estate(FIRST_ORB).expect("survey");
    w
}

#[test]
fn orbital_appraisal_exceeds_planetary_by_site_premium() {
    let mut w = sited_surveyed();
    let planet = w
        .appraise(Holding::Planetary { orb: FIRST_ORB })
        .expect("planet");
    // Re-survey not needed; second appraisal after first still works if we
    // re-survey after motion... first appraise ages the world (receipt).
    // Appraise orbital on a fresh survey path: re-survey after bank growth.
    for _ in 0..4 {
        w.tick();
    }
    let _ = w.collect();
    w.survey_estate(FIRST_ORB).expect("re-survey");
    let planet2 = w
        .appraise_value(&Holding::Planetary { orb: FIRST_ORB })
        .expect("planet val");
    let orbital = w
        .appraise_value(&Holding::Orbital { orb: FIRST_ORB })
        .expect("orbital val");
    assert!(
        orbital > planet2,
        "sited place premium: orbital {orbital} > planetary {planet2} (earlier deed {planet})"
    );
    let site = w.orb().and_then(|o| o.thermal.clone()).expect("site");
    assert!(core_magnetism(&site) >= 1);
    assert!(orbit_phase_period(&w, &site) >= 1);
    assert_eq!(replay(w.chronicle()), w.committed());
}

#[test]
fn unsited_orbital_appraises_none() {
    let mut w = World::new();
    w.found("no-site").expect("found");
    w.survey_estate(FIRST_ORB).expect("survey");
    assert!(
        w.appraise_value(&Holding::Orbital { orb: FIRST_ORB })
            .is_none(),
        "absence is never zero"
    );
}

#[test]
fn stratum_appraises_prospect_richness() {
    let mut w = World::new();
    w.found("vein-world").expect("found");
    w.commission_star(StarClass::G).expect("star");
    let _ = w.prospect(FIRST_ORB, 0);
    w.survey_estate(FIRST_ORB).expect("survey");
    let val = w.appraise_value(&Holding::Stratum {
        orb: FIRST_ORB,
        stratum: 0,
    });
    // Prospect may or may not yield richness depending on draw; if present, > 0.
    if let Some(v) = val {
        assert!(v > ratio(0, 1));
        let struck = w
            .appraise(Holding::Stratum {
                orb: FIRST_ORB,
                stratum: 0,
            })
            .expect("deed");
        assert_eq!(struck, v);
        assert_eq!(replay(w.chronicle()), w.committed());
    }
}
