//! LAW XV witnesses — THE ONE TRANSPORT PAIR (IAE-1, netstratum vendor).
//!
//! Every transport carries its exact inverse at construction, verified
//! `Φ·Φ⁻¹ = 1` exact-rational; a pair that does not close is refused with
//! the residual carried whole; the up stroke read backwards off the
//! carried inverse IS the down stroke; and a movement is DATA on the one
//! pair (arity is a composition count, never a counterpart mechanism).
//!
//! The letter-alphabet witnesses (the 17 Cayley letters, word composition)
//! land with the `letters` module in a later IAE slice — they bridge to the
//! chain carrier. These four stand on `substrate` alone, which is exactly
//! what IAE-1 establishes: the geometry, before the chain.

use iae::transport::{Axis, Movement, Stroke, Transport, TransportError};
use substrate::horizon::HorizonContext;
use substrate::multivector::{Multivector, PlaneIndex};
use substrate::rational::{BigInt, Rational, Space};
use substrate::vector_module::VectorModule;

const RANK: u8 = 4;

fn h() -> HorizonContext {
    HorizonContext::default()
}

fn rs(num: i64, denom: i64) -> Rational<Space> {
    Rational::<Space>::try_new(BigInt::from(num), BigInt::from(denom)).expect("in profile")
}

#[test]
fn the_transport_pair_carries_its_exact_inverse_at_construction() {
    let one = Multivector::<Space>::identity(RANK, h());
    let t = Transport::<Space>::try_new(one.clone(), one.clone()).expect("1·1 = 1");
    assert_eq!(t.phi(), &one);
    assert_eq!(t.phi_inverse(), &one);
}

#[test]
fn a_transport_missing_its_exact_inverse_is_refused_with_the_residual_carried() {
    let one = Multivector::<Space>::identity(RANK, h());
    // A wrong inverse: 2·1 (so Φ·"Φ⁻¹" = 2 ≠ 1; the residual is 1, whole).
    let two =
        Multivector::<Space>::grade0(VectorModule::<Space>::from_vec(vec![rs(2, 1)]), RANK, h());
    match Transport::<Space>::try_new(one, two) {
        Err(TransportError::NotInverse { residual }) => {
            assert!(
                !residual.is_zero(),
                "the refusal carries the non-zero residual whole"
            );
        }
        other => panic!("expected NotInverse with carried residual, got {other:?}"),
    }
}

#[test]
fn the_up_stroke_read_backwards_by_the_carried_inverse_is_the_down_stroke() {
    // The unipotent pair from real plane content: N = (1−P)·X·P, Φ = 1+N.
    let v = Multivector::<Space>::from_planes(
        &[(
            PlaneIndex::single(1).expect("e1"),
            VectorModule::<Space>::from_vec(vec![rs(1, 2), rs(1, 3)]),
        )],
        RANK,
        h(),
    )
    .expect("in rank");
    let x = v.e0_wedge().expect("even-grade carrier");
    let t = Transport::<Space>::unipotent_from(&x).expect("(1+N)(1−N)=1");
    let content = Multivector::<Space>::from_planes(
        &[(
            PlaneIndex::pair(1, 2).expect("e1∧e2"),
            VectorModule::<Space>::from_vec(vec![rs(2, 5)]),
        )],
        RANK,
        h(),
    )
    .expect("in rank");
    let up = Movement {
        stroke: Stroke::Up,
        arity: 1,
        altitude: 2,
        axis: Axis::Spatial,
    };
    let down = Movement {
        stroke: Stroke::Down,
        arity: 1,
        altitude: 2,
        axis: Axis::Spatial,
    };
    let lifted = t.apply(&up, &content).expect("up stroke");
    let returned = t.apply(&down, &lifted).expect("down stroke");
    assert_eq!(
        returned, content,
        "descent off the carried inverse returns the content byte-identically"
    );
}

#[test]
fn a_movement_is_data_the_same_pair_serves_every_arity() {
    // Movement{arity} multiplies the SAME operator — no counterpart mechanism
    // exists for "apply twice" (LAW XV: a movement is data, never a clone).
    let v = Multivector::<Space>::from_planes(
        &[(
            PlaneIndex::single(1).expect("e1"),
            VectorModule::<Space>::from_vec(vec![rs(1, 2)]),
        )],
        RANK,
        h(),
    )
    .expect("in rank");
    let x = v.e0_wedge().expect("carrier");
    let t = Transport::<Space>::unipotent_from(&x).expect("closes");
    let content = Multivector::<Space>::identity(RANK, h());
    let once = t
        .apply(
            &Movement {
                stroke: Stroke::Up,
                arity: 1,
                altitude: 0,
                axis: Axis::Gauge,
            },
            &content,
        )
        .expect("ok");
    let twice = t
        .apply(
            &Movement {
                stroke: Stroke::Up,
                arity: 2,
                altitude: 0,
                axis: Axis::Gauge,
            },
            &content,
        )
        .expect("ok");
    let once_then_once = t
        .apply(
            &Movement {
                stroke: Stroke::Up,
                arity: 1,
                altitude: 0,
                axis: Axis::Gauge,
            },
            &once,
        )
        .expect("ok");
    assert_eq!(twice, once_then_once, "arity is composition count, as data");
}
