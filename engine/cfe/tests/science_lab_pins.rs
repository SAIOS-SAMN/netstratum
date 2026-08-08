//! Seated science labs — seat fee · dual ± · explosion · SEAL B.

use cfe::almanac::{
    explodes, explosion_risk, explosion_threshold, lab_pole_card, science_seat_fee, LabPath,
    ScienceDiscipline,
};
use cfe::dop::{positive, unit, ZoneKind};
use cfe::ratio::ratio;
use cfe::thermal::StarClass;
use cfe::world::{replay, ActionRefused, World, FIRST_ORB};

fn founded() -> World {
    let mut w = World::new();
    w.found("lab-home").expect("f");
    w.commission_star(StarClass::G).expect("star");
    w.site_planet(FIRST_ORB, unit(0, 1), positive(1, 1), None, None)
        .expect("site");
    w.zone(0, ZoneKind::Extractor).expect("z");
    // Crystal for seats + create fees.
    let stamp = w.now();
    w.chronicle_mut().append(
        cfe::dop::ClosureEvent::DiscoveryMinted {
            orb_id: FIRST_ORB,
            resource: cfe::dop::Resource::Crystal,
            amount: ratio(50, 1),
            work: vec![7u8; 32],
        },
        stamp,
        vec![cfe::world::SOVEREIGN],
    );
    w.chronicle_mut().append(
        cfe::dop::ClosureEvent::DiscoveryMinted {
            orb_id: FIRST_ORB,
            resource: cfe::dop::Resource::Biomass,
            amount: ratio(20, 1),
            work: vec![8u8; 32],
        },
        stamp,
        vec![cfe::world::SOVEREIGN],
    );
    World::resume(w.chronicle().clone())
}

#[test]
fn pure_explosion_threshold_is_half() {
    assert_eq!(explosion_threshold(), ratio(1, 2));
    let r = explosion_risk(LabPath::ChemWork, &ratio(0, 1), &ratio(1, 1), 0, false);
    assert!(!explodes(&r), "base chem risk alone under half");
    let r2 = explosion_risk(LabPath::SynthMod, &ratio(1, 1), &ratio(0, 1), 2, true);
    assert!(explodes(&r2), "synth + dark + unseated + thin must explode");
}

#[test]
fn seat_fee_scales_by_discipline_and_rank() {
    let f1 = science_seat_fee(ScienceDiscipline::Chem, 1);
    let f2 = science_seat_fee(ScienceDiscipline::Chem, 2);
    assert!(f2 > f1);
    let synth = science_seat_fee(ScienceDiscipline::Synthetic, 1);
    assert!(synth > f1, "higher wire code → heavier seat");
}

#[test]
fn plasmic_biology_is_alien_fee_vs_primary_physics() {
    use cfe::almanac::{science_seat_affinity, science_seat_fee_for, SeatAffinity};
    let bio = science_seat_affinity(3, ScienceDiscipline::Biology, Some(12));
    let phys = science_seat_affinity(3, ScienceDiscipline::Physics, Some(12));
    assert_eq!(bio, SeatAffinity::Alien);
    assert_eq!(phys, SeatAffinity::Primary);
    let f_bio = science_seat_fee_for(ScienceDiscipline::Biology, 1, 3, Some(12));
    let f_phys = science_seat_fee_for(ScienceDiscipline::Physics, 1, 3, Some(12));
    assert!(f_bio > f_phys, "alien body labs cost more for plasmiform");
}

#[test]
fn polybrachia_math_is_primary() {
    use cfe::almanac::{science_seat_affinity, SeatAffinity};
    let m = science_seat_affinity(2, ScienceDiscipline::Math, Some(7));
    assert_eq!(m, SeatAffinity::Primary);
}

#[test]
fn chem_catalog_requires_seat() {
    let mut w = founded();
    let gem = [1u8; 32];
    // Bank a fake gem via assay-style not needed — refusal is seat first.
    match w.chem_catalog_gem(FIRST_ORB, gem) {
        Err(ActionRefused::ScienceSeatRequired { discipline, .. }) => {
            assert_eq!(discipline, ScienceDiscipline::Chem as u8);
        }
        other => panic!("expected seat required, got {other:?}"),
    }
    w.seat_science(FIRST_ORB, ScienceDiscipline::Chem)
        .expect("seat");
    assert_eq!(w.science_rank(FIRST_ORB, ScienceDiscipline::Chem), 1);
}

#[test]
fn dual_poles_known_always_hidden_conditional() {
    let card = lab_pole_card(
        LabPath::CloneWork,
        &ratio(0, 1),
        &ratio(1, 1),
        0,
        false,
        1, // not empiricist
        0, // no math
        false,
    );
    assert!(card.hidden_pos.is_none(), "hidden + sealed without math/empiricist");
    assert!(card.hidden_neg.is_none() || !card.will_explode);
    let card2 = lab_pole_card(
        LabPath::CloneWork,
        &ratio(0, 1),
        &ratio(1, 1),
        0,
        false,
        2, // empiricist
        0,
        false,
    );
    assert!(card2.hidden_pos.is_some());
}

#[test]
fn seat_and_create_clone_seal_b() {
    let mut w = founded();
    w.seat_science(FIRST_ORB, ScienceDiscipline::Biology)
        .expect("bio r1");
    w.seat_science(FIRST_ORB, ScienceDiscipline::Biology)
        .expect("bio r2");
    w.seat_science(FIRST_ORB, ScienceDiscipline::Clone)
        .expect("clone r1");
    w.seat_science(FIRST_ORB, ScienceDiscipline::Clone)
        .expect("clone r2");
    assert_eq!(w.science_rank(FIRST_ORB, ScienceDiscipline::Clone), 2);
    // Create may explode under thin crystal after seat fees — either path is lawful.
    let r = w.lab_create(FIRST_ORB, LabPath::CloneWork);
    match r {
        Ok(res) => {
            if res.exploded {
                assert!(res.hidden_neg_landed);
            } else {
                assert!(w.clone_count_of(FIRST_ORB) >= 1);
            }
        }
        Err(e) => panic!("create refused: {e:?}"),
    }
    assert_eq!(replay(w.chronicle()), w.committed(), "SEAL B science lab");
}

#[test]
fn lab_create_refuses_without_rank2() {
    let mut w = founded();
    w.seat_science(FIRST_ORB, ScienceDiscipline::Synthetic)
        .expect("r1");
    w.seat_science(FIRST_ORB, ScienceDiscipline::Math)
        .expect("math");
    // Only R1 synth — create needs R2 on primary.
    match w.lab_create(FIRST_ORB, LabPath::SynthMod) {
        Err(ActionRefused::ScienceSeatRequired { need_rank, .. }) => {
            assert_eq!(need_rank, 2);
        }
        other => panic!("expected rank2 gate, got {other:?}"),
    }
}
