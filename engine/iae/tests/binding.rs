//! Witnesses for the non-collapsing binding (doc 51 §6, "bind, don't fold").
//!
//! The overlay retains the whole binding: `generators · coefficients +
//! residual = obstruction` holds exactly, so nothing collapses — the manifold
//! wraps the problem and records how. Torsion is the ℤ/d flex, carried whole.

use iae::resolve::overlay;
use num_bigint::BigInt;

fn b(x: i64) -> BigInt {
    BigInt::from(x)
}

fn matvec(a: &[Vec<BigInt>], x: &[BigInt]) -> Vec<BigInt> {
    a.iter()
        .map(|row| row.iter().zip(x).map(|(w, v)| w * v).sum())
        .collect()
}

#[test]
fn the_binding_reconstructs_the_obstruction_exactly_nothing_collapses() {
    // Generators as columns (3 rows × 2 cols): g0 = (1,0,1), g1 = (0,1,1).
    let g = vec![vec![b(1), b(0)], vec![b(0), b(1)], vec![b(1), b(1)]];
    for obs in [vec![b(4), b(2), b(5)], vec![b(-3), b(7), b(1)], vec![b(0), b(0), b(0)]] {
        let bind = overlay(&obs, &g);
        // generators·coefficients + residual == obstruction — the manifold
        // binds to the problem, it does not discard it.
        let recon: Vec<BigInt> = matvec(&g, &bind.coefficients)
            .iter()
            .zip(&bind.residual)
            .map(|(a, r)| a + r)
            .collect();
        assert_eq!(recon, obs, "the binding reconstructs the obstruction whole");
    }
}

#[test]
fn torsion_is_the_flex_that_binds_an_imperfect_match() {
    // A single generator (2): the obstruction 5 binds as 2·gen with a ℤ/2
    // twist and residual 1 — the torsion is the flex, retained exactly (a ℤ
    // module carries it; a ℚ vector space could not).
    let g = vec![vec![b(2)]]; // 1×1 column (2)
    let bind = overlay(&[b(5)], &g);
    assert_eq!(bind.coefficients, vec![b(2)], "5 = 2·2 + 1");
    assert_eq!(bind.torsion, vec![b(2)], "the ℤ/2 twist is the flex");
    assert_eq!(bind.residual, vec![b(1)]);
    assert!(!bind.locked);

    // 4 locks cleanly (empty residual) with the torsion still recorded — the
    // geometry overlapped completely, and the binding says how.
    let locked = overlay(&[b(4)], &g);
    assert!(locked.locked);
    assert_eq!(locked.coefficients, vec![b(2)]);
    assert_eq!(locked.torsion, vec![b(2)]);
}

#[test]
fn no_generators_leaves_the_whole_obstruction_as_residual() {
    let empty: Vec<Vec<BigInt>> = Vec::new();
    let bind = overlay(&[b(3), b(-1)], &empty);
    assert_eq!(bind.residual, vec![b(3), b(-1)], "nothing to bind — all residual");
    assert!(bind.coefficients.is_empty());
    assert!(!bind.locked);
}
