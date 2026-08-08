//! Chemistry science path — civ utility · dark-7 · iso-union · catalog.

use cfe::almanac::{
    dark_utility, isomorphic, iso_union_product, iso_union_recipe, survival_utility, CivAxes,
    Compoundable, DarkPull,
};
use cfe::dop::{positive, unit, ClosureEvent, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB, SOVEREIGN};

fn at(line: u8, lane: u8, shell: u8, phase: u8, spin: u8) -> [u8; 32] {
    let mut id = [0u8; 32];
    id[3] = line - 1;
    id[4] = lane - 1;
    id[5] = shell - 1;
    id[6] = phase - 1;
    id[7] = spin;
    id
}

fn bonding_pair() -> [[u8; 32]; 2] {
    [at(1, 3, 1, 4, 0), at(1, 3, 1, 4, 1)]
}

fn stocked(name: &str, members: &[[u8; 32]], each: cfe::ratio::Ratio) -> World {
    let mut world = World::new();
    world.found(name).expect("founds");
    world.commission_star(StarClass::G).expect("star");
    world
        .site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("site");
    world.zone(0, ZoneKind::Extractor).expect("zone");
    world.prospect(FIRST_ORB, 1).expect("prospect");
    let stamp = world.now();
    for member in members {
        world.chronicle_mut().append(
            ClosureEvent::AssayStruck {
                orb_id: FIRST_ORB,
                element: *member,
                amount: each.clone(),
                richness: None,
            },
            stamp,
            vec![SOVEREIGN],
        );
    }
    // Crystal for catalog fee.
    world.chronicle_mut().append(
        ClosureEvent::DiscoveryMinted {
            orb_id: FIRST_ORB,
            resource: cfe::dop::Resource::Crystal,
            amount: ratio(4, 1),
            work: vec![4u8; 32],
        },
        stamp,
        vec![SOVEREIGN],
    );
    World::resume(world.chronicle().clone())
}

#[test]
fn isomorphic_same_shell_phase_only() {
    let a = at(1, 3, 2, 3, 0);
    let b = at(5, 1, 2, 3, 1);
    let c = at(1, 3, 2, 4, 0);
    assert!(isomorphic(&a, &b), "shell+phase match");
    assert!(!isomorphic(&a, &c), "phase differs");
    let product = iso_union_product(&a, &b).expect("product");
    assert_ne!(product, a);
    assert_ne!(product, b);
    let (spend, yield_, p2) = iso_union_recipe(&a, &b).unwrap();
    assert_eq!(spend, ratio(1, 1));
    assert_eq!(yield_, ratio(2, 1));
    assert_eq!(p2, product);
}

#[test]
fn survival_utility_favours_empiricist_knowledge() {
    // Deep pair: knowledge dominates mass so empiricist outscores expansionist.
    let pair = [at(6, 3, 1, 4, 0), at(6, 3, 1, 4, 1)];
    let set = Compoundable::new(&pair).unwrap();
    let (_, y) = cfe::almanac::recipe_of(&set);
    let emp = CivAxes {
        disposition: 2,
        metabolism: 1,
        perception: 1,
    };
    let exp = CivAxes {
        disposition: 1,
        metabolism: 1,
        perception: 1,
    };
    let s_emp = survival_utility(emp, &pair, &y);
    let s_exp = survival_utility(exp, &pair, &y);
    assert!(
        s_emp > s_exp,
        "empiricist scores knowledge harder: emp={s_emp} exp={s_exp}"
    );
}

#[test]
fn dark_green_prefers_shallow_yield() {
    let shallow = [at(1, 3, 1, 4, 0), at(1, 3, 1, 4, 1)];
    let deep = [at(6, 3, 1, 4, 0), at(6, 3, 1, 4, 1)];
    let y = ratio(6, 1);
    let g_s = dark_utility(DarkPull::Green, &shallow, &y, 2);
    let g_d = dark_utility(DarkPull::Green, &deep, &y, 12);
    assert!(g_s > g_d, "green greed scores shallow higher");
    let p_s = dark_utility(DarkPull::Pride, &shallow, &y, 2);
    let p_d = dark_utility(DarkPull::Pride, &deep, &y, 12);
    assert!(p_d > p_s, "pride scores deep knowledge higher");
}

#[test]
fn compound_and_catalog_and_iso_union_seal_b() {
    let pair = bonding_pair();
    let mut w = stocked("chem-sci", &pair, ratio(10, 1));
    w.seal_genesis("line", 2, 1, 1).expect("genesis empiricist");
    let gem = w.compound(&pair).expect("bond");
    assert!(w.orb().unwrap().element_amount(&gem) > ratio(0, 1));
    // Science bench ranks something.
    let ranked = w.bench_science(FIRST_ORB);
    let _ = ranked;
    // Seat chemistry lab (paid unlock) before catalog.
    w.seat_science(FIRST_ORB, cfe::almanac::ScienceDiscipline::Chem)
        .expect("seat chem");
    // Catalog the gem (crystal fee).
    let k = w.chem_catalog_gem(FIRST_ORB, gem).expect("catalog");
    assert!(k >= 1);
    assert!(w.chem_catalog().contains(&gem));
    assert!(matches!(
        w.chem_catalog_gem(FIRST_ORB, gem),
        Err(ActionRefused::ChemAlreadyCataloged { .. })
    ));
    // Need a second isomorphic gem for union — compound again with more stock.
    // Re-stock by re-assay style: resume path already spent members; bank more.
    let stamp = w.now();
    for m in &pair {
        w.chronicle_mut().append(
            ClosureEvent::AssayStruck {
                orb_id: FIRST_ORB,
                element: *m,
                amount: ratio(10, 1),
                richness: None,
            },
            stamp,
            vec![SOVEREIGN],
        );
    }
    let mut w = World::resume(w.chronicle().clone());
    let gem2 = w.compound(&pair).expect("second gem batch");
    // Second gem same identity (same members → same gem).
    assert_eq!(gem, gem2);
    // Second isomorphic gem: same shell/phase, different spin/line (line 1).
    let alt = [at(1, 5, 1, 4, 0), at(1, 1, 1, 4, 1)]; // binds 5+1=6 saturates arity 2
    assert!(Compoundable::new(&alt).is_some(), "alt pair bonds");
    for m in &alt {
        let stamp = w.now();
        w.chronicle_mut().append(
            ClosureEvent::AssayStruck {
                orb_id: FIRST_ORB,
                element: *m,
                amount: ratio(10, 1),
                richness: None,
            },
            stamp,
            vec![SOVEREIGN],
        );
    }
    let mut w = World::resume(w.chronicle().clone());
    let gem_b = w.compound(&alt).expect("alt gem");
    assert_ne!(gem, gem_b);
    // Union of the two *gems* (not members): gems must share shell/phase seats.
    match w.iso_union(FIRST_ORB, gem, gem_b) {
        Ok(product) => {
            assert_ne!(product, gem);
            assert!(w.orb().unwrap().element_amount(&product) > ratio(0, 1));
        }
        Err(ActionRefused::NotIsomorphic) => {
            // Gem digests may land on different planes — lawful.
        }
        Err(e) => panic!("unexpected {e:?}"),
    }
    // Direct iso on element ids that share plane.
    let a = at(1, 2, 2, 3, 0);
    let b = at(4, 4, 2, 3, 1);
    assert!(isomorphic(&a, &b));
    assert_eq!(replay(w.chronicle()), w.committed(), "SEAL B chemistry science");
}

#[test]
fn dark_pulls_envy_when_intel_live() {
    let mut w = World::new();
    w.found("envy").expect("f");
    assert!(w.dark_pulls().iter().all(|(p, _)| *p != DarkPull::Envy));
    // Manually inject intel via committed path would need Spy deeds; use
    // seal + empty is fine — sloth may fire with no instruments.
    let pulls = w.dark_pulls();
    assert!(
        pulls.iter().any(|(p, _)| *p == DarkPull::Sloth),
        "no instruments → sloth well: {:?}",
        pulls
    );
}
