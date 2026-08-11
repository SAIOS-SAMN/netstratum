//! **THE SHEAR AGAINST THE SWEEP** — a differential witness for
//! `World::bench` (operator ruling 2026-07-30, carried in `peel`'s own
//! doc: *no cartesian find-scan repetition; peels and shears where
//! there were iterations*).
//!
//! The bench used to walk EVERY subset of a ledger up to five bodies
//! and ask the door about each. The bonding law partitions that space —
//! matter bonds only within one `(shell, phase)` — so the sweep spent
//! nearly all of its work on candidates the law could never admit:
//! measured, some 55,000 of them for a world holding 24 elements, and
//! `chemistry_pins` fell from 7.32s to 0.04s when the plane shear
//! replaced it.
//!
//! **An optimisation is only lawful if the reading does not move.** The
//! naive sweep is kept HERE, deliberately, as a reference oracle: it is
//! the law read the slow obvious way, and the pin asserts the fast form
//! agrees with it set for set over worlds whose matter nobody chose.
//! That is what makes the shear a change of WHERE TO LOOK rather than a
//! change of what bonds.

use cfe::almanac::Compoundable;
use cfe::dop::{positive, unit, ZoneKind};
use cfe::thermal::StarClass;
use cfe::world::{World, FIRST_ORB, SOVEREIGN};

/// CIV-REACH-1: the differential wants a RICH stock, so the fixture
/// digs at full industry — techs 1..=5 open every stratum the G forge
/// carries (reach 5); the shear/sweep agreement is what's under test,
/// not the growth ladder.
fn industrial_dig(world: &mut World) -> Vec<[u8; 32]> {
    for c in 1u8..=5 {
        world.grant_tech_as(SOVEREIGN, FIRST_ORB, c).expect("tech");
    }
    let mut drawn = Vec::new();
    for stratum in 1..=world.civ_reach() {
        if let Ok(struck) = world.prospect(FIRST_ORB, stratum) {
            drawn.push(struck.element);
        }
    }
    drawn
}

/// THE REFERENCE ORACLE — the original sweep, verbatim: every subset up
/// to five bodies, each put to the door. Slow and obviously correct,
/// which is exactly its job here.
fn swept(stock: &[[u8; 32]]) -> Vec<Vec<[u8; 32]>> {
    let widest = usize::from(cfe::almanac::SHELLS + 1);
    let mut found: Vec<Vec<[u8; 32]>> = Vec::new();
    let mut frontier: Vec<Vec<usize>> = stock.iter().enumerate().map(|(at, _)| vec![at]).collect();
    while let Some(picked) = frontier.pop() {
        let members: Vec<[u8; 32]> = picked.iter().map(|at| stock[*at]).collect();
        if Compoundable::new(&members).is_some() {
            found.push(members.clone());
        }
        if picked.len() < widest {
            let last = *picked.last().expect("non-empty");
            for next in (last + 1)..stock.len() {
                let mut wider = picked.clone();
                wider.push(next);
                frontier.push(wider);
            }
        }
    }
    found
}

#[test]
fn the_shear_finds_exactly_what_the_sweep_found() {
    let mut worlds = 0;
    let mut sets = 0;
    for trial in 0..30 {
        let mut world = World::new();
        world.found(&format!("equiv-{trial}")).expect("founds");
        world.commission_star(StarClass::G).expect("commissions");
        world
            .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
            .expect("sites");
        world.zone(0, ZoneKind::Extractor).expect("zones");
        let drawn = industrial_dig(&mut world);
        for _ in 0..4 {
            world.tick();
        }
        for element in &drawn {
            let _ = world.assay(FIRST_ORB, *element);
        }
        let stock: Vec<[u8; 32]> = world
            .committed()
            .orbs
            .iter()
            .find(|orb| orb.orb_id == FIRST_ORB)
            .expect("stands")
            .elements
            .iter()
            .map(|(id, _)| *id)
            .collect();

        let mut old = swept(&stock);
        let mut new: Vec<Vec<[u8; 32]>> =
            world.bench(FIRST_ORB).into_iter().map(|(members, _, _)| members).collect();
        // Compare as SETS of sets — the shear's order is its own.
        old.sort();
        new.sort();
        assert_eq!(new, old, "trial {trial}: the shear and the sweep disagree");
        sets += new.len();
        if !new.is_empty() {
            worlds += 1;
        }
    }
    // The witness has to BITE: thirty worlds that all bonded nothing
    // would agree trivially and prove nothing about the shear.
    assert!(
        sets > 0 && worlds > 0,
        "the differential found no bonding set at all — it agreed about nothing"
    );
    println!("RESULT: {worlds}/30 worlds bond; {sets} sets; shear == sweep on every one");
}

#[test]
fn the_shear_never_leaves_its_plane() {
    // WHY the shear is allowed to skip what it skips, stated directly:
    // every set the bench returns sits inside ONE (shell, phase), and
    // holds exactly that shell's arity. Those are the two conditions
    // that make the space between planes unsearchable rather than
    // merely unsearched.
    let mut world = World::new();
    world.found("plane-read").expect("founds");
    world.commission_star(StarClass::G).expect("commissions");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("sites");
    world.zone(0, ZoneKind::Extractor).expect("zones");
    let drawn = industrial_dig(&mut world);
    for _ in 0..4 {
        world.tick();
    }
    for element in &drawn {
        let _ = world.assay(FIRST_ORB, *element);
    }
    for (members, _, _) in world.bench(FIRST_ORB) {
        let seats: Vec<_> = members.iter().map(cfe::almanac::periodic_of).collect();
        let first = &seats[0];
        assert!(
            seats.iter().all(|seat| seat.shell == first.shell && seat.phase == first.phase),
            "a returned set straddles two planes"
        );
        assert_eq!(
            members.len(),
            usize::from(first.arity),
            "a returned set is not its plane's arity"
        );
    }
}
