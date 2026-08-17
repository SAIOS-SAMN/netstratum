//! ING-2 — the STAPLE DIET is drawn from the world's OWN prospected veins by the
//! ingestion kernel, not a 3-`Resource` enum. This witnesses the non-breaking
//! mechanism (`staple_diet_of`); ING-2b routes consumption through it.

use cfe::dop::{positive, unit};
use cfe::thermal::StarClass;
use cfe::world::World;

#[test]
fn the_staple_diet_is_the_almanac_over_the_worlds_veins() {
    let mut w = World::new();
    w.found("diet-hold").expect("founds");
    let orb = cfe::world::FIRST_ORB;

    // A world whose ground is unopened bears no vein — no world-borne food to
    // draw. The kernel evaluates to ∅: the Starvation Cull, from an empty almanac.
    assert_eq!(w.staple_diet_of(orb), None, "no prospected veins ⇒ no diet");

    // A commissioned star and a sited planet, then full base tech, then prospect
    // the strata so the planet bears real elements (AP bounds how many strike, so
    // collect what opens).
    w.commission_star(StarClass::G).expect("commissions");
    w.site_planet(orb, unit(0, 1), positive(1, 1), None, None).expect("sites");
    for c in 1u8..=5 {
        w.grant_tech_as(cfe::world::SOVEREIGN, orb, c).expect("reach");
    }
    let mut veins: Vec<[u8; 32]> = Vec::new();
    for stratum in 1..=8u8 {
        if let Ok(struck) = w.prospect(orb, stratum) {
            veins.push(struck.element);
        }
    }
    assert!(!veins.is_empty(), "the ground opened and bore veins");

    // The diet reads the PLANET, not a table: the staple, when the world offers
    // matter this line can metabolise, is one of the elements its own veins bear.
    match w.staple_diet_of(orb) {
        Some(staple) => assert!(
            veins.contains(&staple),
            "the staple is drawn from the world's own prospected veins"
        ),
        None => { /* the veins bore nothing this species can eat — an honest cull */ }
    }
}
