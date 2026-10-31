//! IAE-3 — the TASK → obstruction bridge, as a transduction.
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
//! return; that failure is the torsion, the torsion is the signal." That
//! residual, and its harmonic `H¹` reduction, is what the resolver (IAE-4)
//! must explain.

use crate::mat::matvec;
use crate::sheaf::CellSheaf;
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
        assert_eq!(
            cells.len(),
            rows * cols,
            "GridObs: cell count must be rows·cols"
        );
        GridObs { rows, cols, cells }
    }

    pub fn at(&self, r: usize, c: usize) -> i64 {
        self.cells[r * self.cols + c]
    }
}

/// The entity potential as a 0-cochain on 𝒟 (the entities flattened in order).
pub fn entity_potential(entities: &[Vec<BigInt>]) -> Vec<BigInt> {
    entities.iter().flatten().cloned().collect()
}

/// The measurement, as a transduction: the relational field `Δ = δ⁰f` (a
/// 1-cochain on 𝒟). Carrying the entity potential through the coboundary is
/// the chain map — no projection. Because it is a coboundary, its obstruction
/// vanishes; the obstruction only appears for a field measured directly (a
/// transformation that is not a single global potential).
pub fn measure(complex: &CellSheaf, entities: &[Vec<BigInt>]) -> Vec<BigInt> {
    matvec(&complex.coboundary(0), &entity_potential(entities))
}

/// The obstruction: the triple residual `δ¹c` of a relational 1-cochain `c` —
/// whether every triple of relationships closes. Zero iff the relationships
/// are globally consistent; otherwise the residual is the raw torsion, and
/// [`CellSheaf::reduce_cochain`] folds `c` to its irreducible `H¹` class.
pub fn obstruction(complex: &CellSheaf, cochain: &[BigInt]) -> Vec<BigInt> {
    matvec(&complex.coboundary(1), cochain)
}
