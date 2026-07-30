//! AEON-1a — THE MANIFOLD CORE (11_AEON, tags 42–46). The gauge key G
//! lives on the torus fiber: exact rational, wrapped [0, 1), zero at
//! genesis, moved ONLY by receipt. Divergence and confluence are key
//! changes, not births; the star moves in its region, never on rails;
//! the fold seals an era and seats the next aeon on its own G-sheet.
//! Every receipt carries the WRAPPED value — replay never re-derives
//! the wrap (SEAL B).

use cfe::dop::ClosureEvent;
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, wrap_gauge, World, FIRST_ORB};

/// The wrap law: the fiber is a circle — 3/2 wraps to 1/2, −1/4 wraps
/// to 3/4, exactly; a shift replays and resumes byte-honest.
#[test]
fn the_key_wraps_and_replays() {
    assert_eq!(wrap_gauge(ratio(3, 2)), ratio(1, 2));
    assert_eq!(wrap_gauge(ratio(-1, 4)), ratio(3, 4));
    assert_eq!(wrap_gauge(ratio(0, 1)), ratio(0, 1));
    assert_eq!(wrap_gauge(ratio(7, 7)), ratio(0, 1));

    let mut world = World::new();
    world.found("keystone").expect("founds");
    assert_eq!(world.gauge(), &ratio(0, 1), "G is zero at genesis");

    world.shift_gauge(ratio(3, 2));
    assert_eq!(world.gauge(), &ratio(1, 2), "the receipt carries the wrap");

    let committed = replay(world.chronicle());
    assert_eq!(committed.gauge, ratio(1, 2));
    let resumed = World::resume(world.chronicle().clone());
    assert_eq!(resumed.gauge(), &ratio(1, 2), "a resumed world holds its key");
}

/// Divergence moves the key by the cleave's delta; confluence settles
/// it at the passed interference; replay reproduces the whole walk —
/// the LAST key stands.
#[test]
fn divergence_and_confluence_are_key_changes() {
    let mut world = World::new();
    world.found("meridian").expect("founds");

    world.diverge("the-severed-chord", ratio(1, 3));
    assert_eq!(world.gauge(), &ratio(1, 3), "the cleave IS a key change");

    world.conflue("haven", ratio(5, 6));
    assert_eq!(world.gauge(), &ratio(5, 6), "the key settles at the interference");

    let branch_named = world.chronicle().receipts().iter().any(|receipt| {
        matches!(
            &receipt.event,
            ClosureEvent::Diverged { branch, .. } if branch == "the-severed-chord"
        )
    });
    assert!(branch_named, "the departing peer is named in the receipt");
    assert_eq!(replay(world.chronicle()).gauge, ratio(5, 6));
}

/// The star moves in its region: aging a G sun to a red giant
/// recomputes luminosity a thousandfold — the same orbit's T_eq⁴
/// crosses bands with no re-siting; replay reproduces the sky.
#[test]
fn the_star_moves_in_its_region() {
    use cfe::thermal::t_eq4;
    let mut world = World::new();
    world.found("emberfall").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world.site_planet(FIRST_ORB, ratio(0, 1), ratio(1, 1)).expect("sites");

    let before = t_eq4(&StarClass::G.luminosity(), &ratio(0, 1), &ratio(1, 1));
    world.age_star(StarClass::RedGiant).expect("the star reddens");
    assert_eq!(world.star(), Some(StarClass::RedGiant));
    let after = t_eq4(&StarClass::RedGiant.luminosity(), &ratio(0, 1), &ratio(1, 1));
    assert_eq!(after.clone() / before, ratio(1000, 1), "L moved 1000-fold, exactly");

    assert_eq!(replay(world.chronicle()).star, Some(StarClass::RedGiant));

    // A world with no sky cannot age one.
    let mut dark = World::new();
    dark.found("darkmoor").expect("founds");
    assert!(dark.age_star(StarClass::M).is_err());
}

/// The fold: the era seals, the base G advances, and the new sheet's
/// key replays — nothing destroyed, everything receipted.
#[test]
fn the_fold_seats_a_new_sheet() {
    let mut world = World::new();
    world.found("lastlight").expect("founds");
    world.shift_gauge(ratio(1, 4));

    world.seal_fold(1, ratio(5, 4));
    assert_eq!(world.gauge(), &ratio(1, 4), "the base advanced and wrapped");
    let folded = world.chronicle().receipts().iter().any(|receipt| {
        matches!(&receipt.event, ClosureEvent::EraFolded { era: 1, .. })
    });
    assert!(folded, "the fold is a receipt, not a deletion");
    assert_eq!(replay(world.chronicle()).gauge, ratio(1, 4));
}

