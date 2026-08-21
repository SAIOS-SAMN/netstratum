//! THE ATLAS — the manifold's total space (doc 55). Charts are `CellSheaf`
//! cochain complexes; a cover glues them along overlaps by restriction cochain
//! maps and gauge transitions. Two laws live here, both exact (integer ranks +
//! torsion, no float — 54_THE_NO_SCALAR_LAW):
//!
//!  - **§4 SOUNDNESS** — [`cocycle_holds`]: the transition functions compose
//!    consistently on a triple overlap (`g_ij·g_jk = g_ik`, `g_ii = I`). Without
//!    it the atlas is not a bundle and the glued cohomology is meaningless.
//!  - **§5 PROVABLE COGNITIVE INCREASE** — [`delta_h`]: the connecting-homomorphism
//!    rank of the Mayer–Vietoris sequence — the classes of the glued atlas that
//!    restrict to ZERO on every chart (born purely on the overlaps). By exactness
//!    `ΔH = Σ_k [ betti_k(overlap) − rank(j_k) ]`, where `j_k` is the induced
//!    restriction-difference `H^k(U)⊕H^k(V) → H^k(W)`. The two arcs glued into a
//!    circle give `ΔH = 1` (the loop no arc carries); a disjoint cover gives `0`.

use num_bigint::BigInt;
use num_traits::Zero;
use substrate::snf::smith_normal_form;

use crate::mat::{matvec, Mat};
use crate::sheaf::CellSheaf;

/// The kernel of `δ^k` (the `k`-cocycles) as a basis of column vectors in `C^k`.
/// For `A = δ^k` with SNF `D = U·A·V`, `A x = 0 ⟺ D(V⁻¹x)=0`, so the kernel is
/// spanned by columns `[rank..]` of `V`. When `C^{k+1}` is empty, `δ^k` is the
/// zero map and every cochain is a cocycle (the standard basis).
fn cocycle_basis(sheaf: &CellSheaf, k: usize) -> Vec<Vec<BigInt>> {
    let cols = sheaf.dim_cochain(k);
    if cols == 0 {
        return Vec::new();
    }
    if sheaf.dim_cochain(k + 1) == 0 {
        // δ^k = 0 : every cochain is a cocycle.
        return (0..cols)
            .map(|j| {
                let mut e = vec![BigInt::zero(); cols];
                e[j] = BigInt::from(1);
                e
            })
            .collect();
    }
    let s = smith_normal_form(&sheaf.coboundary(k));
    let rank = s.rank();
    // Columns [rank..cols) of V span the kernel.
    (rank..cols)
        .map(|j| (0..cols).map(|i| s.v[i][j].clone()).collect())
        .collect()
}

/// Stack column vectors into a matrix and return its exact ℤ-rank (= ℚ-rank).
fn rank_of_columns(columns: &[Vec<BigInt>]) -> usize {
    let cols = columns.len();
    if cols == 0 {
        return 0;
    }
    let rows = columns[0].len();
    if rows == 0 {
        return 0;
    }
    let mut m: Mat = vec![vec![BigInt::zero(); cols]; rows];
    for (j, col) in columns.iter().enumerate() {
        for (i, v) in col.iter().enumerate() {
            m[i][j] = v.clone();
        }
    }
    smith_normal_form(&m).rank()
}

/// A basis of harmonic representatives of `H^k(sheaf)` — cocycles reduced modulo
/// coboundaries, thinned to a maximal independent set (its size is `betti_k`).
fn cohomology_reps(sheaf: &CellSheaf, k: usize) -> Vec<Vec<BigInt>> {
    let mut kept: Vec<Vec<BigInt>> = Vec::new();
    for c in cocycle_basis(sheaf, k) {
        let r = sheaf.reduce_cochain(k, &c);
        if r.iter().all(|x| x.is_zero()) {
            continue; // a coboundary — trivial in H^k
        }
        // Keep it only if it raises the rank (stays independent of what we have).
        let mut trial = kept.clone();
        trial.push(r.clone());
        if rank_of_columns(&trial) > kept.len() {
            kept.push(r);
        }
    }
    kept
}

/// **§4 SOUNDNESS — the cocycle condition on a triple overlap.** Transition
/// functions `g_ij : U_i∩U_j → G` (square integer matrices acting on the fiber)
/// must satisfy `g_ij · g_jk = g_ik` and each `g_ii = I`. A tampered transition
/// breaks this and the atlas is not a bundle. Pure exact matrix identity.
pub fn cocycle_holds(g_ij: &Mat, g_jk: &Mat, g_ik: &Mat) -> bool {
    // g_ij (a×b) · g_jk (b×c) must equal g_ik (a×c).
    if g_ij.is_empty() || g_jk.is_empty() || g_ik.is_empty() {
        return false;
    }
    let (a, b) = (g_ij.len(), g_ij[0].len());
    let (b2, c) = (g_jk.len(), g_jk[0].len());
    if b != b2 || g_ik.len() != a || g_ik[0].len() != c {
        return false;
    }
    for i in 0..a {
        for j in 0..c {
            let mut acc = BigInt::zero();
            for t in 0..b {
                acc += &g_ij[i][t] * &g_jk[t][j];
            }
            if acc != g_ik[i][j] {
                return false;
            }
        }
    }
    true
}

/// **§5 PROVABLE COGNITIVE INCREASE — the connecting-homomorphism rank.** For a
/// two-chart cover `U, V` with overlap `W` and restriction cochain maps
/// `r_u_k : C^k(U) → C^k(W)`, `r_v_k : C^k(V) → C^k(W)` (indexed by grade `k`),
/// the Mayer–Vietoris connecting map's total rank
///
/// ```text
///   ΔH  =  Σ_k [ betti_k(W)  −  rank( j_k : H^k(U)⊕H^k(V) → H^k(W) ) ]
/// ```
///
/// counts the classes of the glued atlas that restrict to ZERO on every chart —
/// shared ideas no chart carries alone. `ΔH ≥ 0` always; `ΔH > 0` iff convergence
/// minted a genuinely global class. The restriction maps must be honest cochain
/// maps (they commute with `δ`); when a grade has no restriction the map is zero.
pub fn delta_h(
    u: &CellSheaf,
    v: &CellSheaf,
    w: &CellSheaf,
    r_u: &[Mat],
    r_v: &[Mat],
) -> usize {
    let grades = u.grades().max(v.grades()).max(w.grades());
    let mut total = 0usize;
    for k in 0..grades {
        let betti_w = w.cohomology(k).betti;
        if betti_w == 0 {
            continue; // nothing on the overlap at this grade → no new class here
        }
        // The induced restriction-difference on cohomology: apply r_u to each
        // H^k(U) rep and r_v to each H^k(V) rep, reduce each image in H^k(W), and
        // take the rank of the spanned subspace. (Sign on the V side does not
        // change the rank, so we drop it.)
        let mut images: Vec<Vec<BigInt>> = Vec::new();
        let apply = |restr: Option<&Mat>, rep: &[BigInt]| -> Vec<BigInt> {
            match restr {
                Some(m) if !m.is_empty() => w.reduce_cochain(k, &matvec(m, rep)),
                _ => vec![BigInt::zero(); w.dim_cochain(k)],
            }
        };
        for rep in cohomology_reps(u, k) {
            images.push(apply(r_u.get(k), &rep));
        }
        for rep in cohomology_reps(v, k) {
            images.push(apply(r_v.get(k), &rep));
        }
        let rank_j = rank_of_columns(&images);
        total += betti_w.saturating_sub(rank_j);
    }
    total
}
