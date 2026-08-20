//! IAE-3 — THE TASK → OBSTRUCTION BRIDGE, as a transduction.
//!
//! An ARC grid is transduced into the **relational defect field**: entities
//! (objects/colors) become the vertices of a cellular complex, their pairwise
//! relationships the edges, and their triples the 2-cells that carry the
//! obstruction. This is a transduction, not a projection — the grid's
//! structure is carried into the defect complex through the exact coboundary,
//! nothing is flattened.
//!
//! The measurement is the antisymmetric relational field `Δ_ij = f_j − f_i`
//! (convergence-engine's `encode_relational`) — which is precisely `δ⁰`
//! applied to the entity potential `f`. So for a single consistent grid the
//! field is a coboundary and its obstruction `δ¹Δ = δ¹δ⁰f = 0` vanishes by
//! construction. The obstruction is non-zero exactly when a triple of
//! relationships fails to close — the doc's "carry A→B→C→A and it does not
//! return; that failure IS the torsion, the torsion IS the signal." That
//! residual, and its harmonic `H¹` reduction, is what the resolver (IAE-4)
//! must explain.

use crate::mat::{matvec, Mat};
use crate::sheaf::{CellSheaf, Incidence};
use num_bigint::BigInt;

/// A minimal ARC grid observation — row-major integer colors. `iae` stays
/// grid-library-agnostic; the conductor adapts a real `Grid` to this at wire
/// time (IAE-7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridObs {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<i64>,
}

impl GridObs {
    pub fn new(rows: usize, cols: usize, cells: Vec<i64>) -> Self {
        assert_eq!(cells.len(), rows * cols, "GridObs: cell count must be rows·cols");
        GridObs { rows, cols, cells }
    }

    pub fn at(&self, r: usize, c: usize) -> i64 {
        self.cells[r * self.cols + c]
    }
}

/// The color entities of a grid: one per distinct non-background color, each
/// an exact integer coordinate vector `(value, count, Σrow, Σcol)`. Sorted by
/// color value — deterministic, no float. This is the "value shell" (k ≈ 10
/// colors); the object/cell shells are coarser/finer strata of the same
/// construction.
pub fn color_entities(grid: &GridObs, background: i64) -> Vec<Vec<BigInt>> {
    // Gather per color: count, Σrow, Σcol. A flat associative scan keeps the
    // result an exact function of the grid (no hashing nondeterminism).
    let mut acc: Vec<(i64, i64, i64, i64)> = Vec::new(); // (value, count, Σrow, Σcol)
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            let v = grid.at(r, c);
            if v == background {
                continue;
            }
            match acc.iter_mut().find(|e| e.0 == v) {
                Some(e) => {
                    e.1 += 1;
                    e.2 += r as i64;
                    e.3 += c as i64;
                }
                None => acc.push((v, 1, r as i64, c as i64)),
            }
        }
    }
    acc.sort_by_key(|e| e.0);
    acc.into_iter()
        .map(|(v, n, sr, sc)| vec![BigInt::from(v), BigInt::from(n), BigInt::from(sr), BigInt::from(sc)])
        .collect()
}

/// `m × m` identity as an exact integer matrix.
fn identity(m: usize) -> Mat {
    (0..m)
        .map(|i| (0..m).map(|j| BigInt::from((i == j) as i64)).collect())
        .collect()
}

/// THE RELATIONAL DEFECT COMPLEX 𝒟 on `n` entities with coordinate module
/// `ℤᵐ`: the 2-skeleton of the (n−1)-simplex — all vertices, edges (pairs),
/// and triangles (triples) — with constant stalk `ℤᵐ` and identity restriction
/// morphisms, simplicial incidence signs. Its `H¹`/triple-residual is where
/// the obstruction lives. (Heterogeneous stalks in [`CellSheaf`] are for the
/// cross-shell transduction; within one shell the coordinate module is a
/// uniform `ℤᵐ`, which is not the homogeneous-collapse flaw — that was forcing
/// DISTINCT strata into one type.)
///
/// Edges are ordered lexicographically `(i<j)`, triangles `(i<j<k)`; the
/// measurement in [`measure`] follows the same order.
pub fn relational_complex(n: usize, m: usize) -> CellSheaf {
    let edges: Vec<(usize, usize)> =
        (0..n).flat_map(|i| ((i + 1)..n).map(move |j| (i, j))).collect();
    let tris: Vec<(usize, usize, usize)> = (0..n)
        .flat_map(|i| ((i + 1)..n).flat_map(move |j| ((j + 1)..n).map(move |k| (i, j, k))))
        .collect();
    let edge_idx = |i: usize, j: usize| edges.iter().position(|&e| e == (i, j)).expect("edge present");

    let id = identity(m);
    let mut inc = Vec::new();
    // vertex ◁ edge: δ⁰f(i,j) = f_j − f_i.
    for (e, &(i, j)) in edges.iter().enumerate() {
        inc.push(Incidence { face_grade: 0, face_idx: i, coface_idx: e, sign: BigInt::from(-1), restriction: id.clone() });
        inc.push(Incidence { face_grade: 0, face_idx: j, coface_idx: e, sign: BigInt::from(1), restriction: id.clone() });
    }
    // edge ◁ triangle: ∂(i,j,k) = (j,k) − (i,k) + (i,j).
    for (t, &(i, j, k)) in tris.iter().enumerate() {
        inc.push(Incidence { face_grade: 1, face_idx: edge_idx(i, j), coface_idx: t, sign: BigInt::from(1), restriction: id.clone() });
        inc.push(Incidence { face_grade: 1, face_idx: edge_idx(i, k), coface_idx: t, sign: BigInt::from(-1), restriction: id.clone() });
        inc.push(Incidence { face_grade: 1, face_idx: edge_idx(j, k), coface_idx: t, sign: BigInt::from(1), restriction: id.clone() });
    }

    let mut dims = vec![vec![m; n]];
    if !edges.is_empty() {
        dims.push(vec![m; edges.len()]);
    }
    if !tris.is_empty() {
        dims.push(vec![m; tris.len()]);
    }
    CellSheaf::new(dims, inc).expect("the relational 2-skeleton is a valid simplicial complex")
}

/// The entity potential as a 0-cochain on 𝒟 (the entities flattened in order).
pub fn entity_potential(entities: &[Vec<BigInt>]) -> Vec<BigInt> {
    entities.iter().flatten().cloned().collect()
}

/// THE MEASUREMENT, as a transduction: the relational field `Δ = δ⁰f` (a
/// 1-cochain on 𝒟). Carrying the entity potential through the coboundary IS
/// the chain map — no projection. Because it is a coboundary, its obstruction
/// vanishes; the obstruction only appears for a field measured directly (a
/// transformation that is not a single global potential).
pub fn measure(complex: &CellSheaf, entities: &[Vec<BigInt>]) -> Vec<BigInt> {
    matvec(&complex.coboundary(0), &entity_potential(entities))
}

/// THE OBSTRUCTION: the triple residual `δ¹c` of a relational 1-cochain `c` —
/// whether every triple of relationships closes. Zero iff the relationships
/// are globally consistent; otherwise the residual is the raw torsion, and
/// [`CellSheaf::reduce_cochain`] folds `c` to its irreducible `H¹` class.
pub fn obstruction(complex: &CellSheaf, cochain: &[BigInt]) -> Vec<BigInt> {
    matvec(&complex.coboundary(1), cochain)
}
