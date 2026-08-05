//! TERRA-1 — THE GIFTED CROWN (12_UNION §2, operator ruling
//! 2026-07-30). Terraforming is the highest technology a civilization
//! can hold and is only ever GIFTED: it licenses down from tier 0
//! ONLY onto a complete base-7 stack (the solar leap + all five
//! crystallized conditions), and under it — and only under it — the
//! orbit that "does not move" moves: albedo and orbit re-seat, the
//! world crosses substrate bands.

use cfe::dop::{positive, unit};

use cfe::dop::Resource;
use cfe::ratio::ratio;
use cfe::thermal::{t_eq4, StarClass, SubstrateBand};
use cfe::world::{
    aperture_transfer, license_tech, ActionRefused, Leap, World, FIRST_ORB,
};

fn sunlit(name: &str) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world.site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None).expect("sites");
    world
}

#[test]
fn the_crown_gifts_only_onto_a_finished_stack_and_moves_the_orbit() {
    // A tier-0 grantor with an open council, and a crossed receiver.
    let mut grantor = sunlit("lyra-hearth");
    let b = grantor.chart("midhold", FIRST_ORB).expect("charts");
    let c = grantor.chart("farhold", b).expect("charts");
    grantor.transfer(cfe::dop::between(c, FIRST_ORB), Resource::Iron, cfe::dop::positive(6, 1)).expect("spreads");
    grantor.transfer(cfe::dop::between(c, b), Resource::Iron, cfe::dop::positive(6, 1)).expect("spreads");
    grantor
        .seal_council(1, vec![FIRST_ORB, b, c], Resource::Iron, ratio(2, 1), 600, ratio(1, 1))
        .expect("the council stands");
    let mut receiver = sunlit("kepler-passage");
    aperture_transfer(
        &mut receiver, FIRST_ORB, &mut grantor, FIRST_ORB,
        Resource::Iron, cfe::dop::positive(1, 1), 1,
    )
    .expect("both cross");

    // The orbit does not move without the crown.
    assert_eq!(
        receiver.terraform(FIRST_ORB, unit(1, 2), positive(1, 2)),
        Err(ActionRefused::TerraformUnearned)
    );
    // The crown refuses an unfinished stack (only the solar leap held).
    let crown = |deal| Leap { condition: 6, name: "terraforming".into(), deal, council: 1 };
    assert_eq!(
        license_tech(&mut grantor, FIRST_ORB, &mut receiver, FIRST_ORB, crown(50)),
        Err(ActionRefused::BaseSevenUnfinished)
    );

    // Build the stack by licensed leaps (conditions 1..=5), then gift.
    for condition in 1u8..=5 {
        license_tech(
            &mut grantor, FIRST_ORB, &mut receiver, FIRST_ORB,
            Leap { condition, name: "granted working".into(), deal: 10 + u64::from(condition), council: 1 },
        )
        .expect("the stack grows");
    }
    assert!(receiver.base_seven_complete(), "six earned-or-granted nodes stand");
    license_tech(&mut grantor, FIRST_ORB, &mut receiver, FIRST_ORB, crown(50))
        .expect("the crown gifts onto the finished stack");
    assert!(receiver.can_terraform());

    // Under the crown, the world moves BETWEEN BANDS: mesic (load 1)
    // re-seats hot (load 2) — measured in T_eq4, exact.
    receiver.terraform(FIRST_ORB, unit(1, 2), positive(1, 2)).expect("the orbit moves");
    let site = receiver.orb().expect("orb").thermal.clone().expect("sited");
    let t4 = t_eq4(&StarClass::G.luminosity(), site.albedo.value(), site.orbit_r.value());
    assert!(t4 > SubstrateBand::Mesic.bounds4().1, "the world crossed into the hot band");
}
