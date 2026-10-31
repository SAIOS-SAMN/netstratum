//! IAE-6 — the graded mind: three manifolds, holonomic containment.
//!
//! The persistent vocabulary a civilization carries, graded across the three
//! nested manifolds of thought (doc 51, the sensory strata):
//!
//! ```text
//!     harmonic   innermost   "sound"          resonance / pairwise relation
//!        ⊂
//!     luminal    middle      "sight"          spatial structure
//!        ⊂
//!     holonomic  outermost   "interpretation" + CONTAINMENT
//! ```
//!
//! The mind's **breathing is the construction of its geometry**: it *unfolds*
//! (condenses a new generator when an obstruction is irreducible) and *folds*
//! (resolves a field against everything it has learned). The inner manifolds
//! grow freely — the only constraint anywhere is the holonomic (outer) shell,
//! the containment law (`the inner crystal may not outgrow the exterior`).
//!
//! Resolution is **O(n) in the mind's rank** — reduction against `n` learned
//! generators, not a constant-time lookup. The mind is not free; it pays for
//! what it knows, and that is the honest cost of a growing algebra.

use crate::proposer::{propose_and_verify, Mint};
use crate::resolve::{overlay, reduce_modulo, Binding};
use num_bigint::BigInt;
use num_traits::Zero;

/// The three nested manifolds, inner → outer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Manifold {
    /// Innermost — resonance, "sound". Unconstrained.
    Harmonic,
    /// Middle — spatial structure, "sight". Unconstrained.
    Luminal,
    /// Outermost — interpretation and containment. The only bounded shell.
    Holonomic,
}

/// Why a condensation is refused. Only the holonomic shell ever refuses on
/// capacity; a dimension mismatch is a construction error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainmentRefusal {
    /// The outer shell is full — the containment law, the only capacity bound.
    HolonomicShellFull { capacity: usize },
    /// The generator does not live in the mind's cochain dimension.
    DimensionMismatch { expected: usize, got: usize },
}

/// A civilization's learned geometry — generators graded across the manifolds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mind {
    dim: usize,
    harmonic: Vec<Vec<BigInt>>,
    luminal: Vec<Vec<BigInt>>,
    holonomic: Vec<Vec<BigInt>>,
    holonomic_capacity: usize,
}

impl Mind {
    /// A new mind over cochains of dimension `dim`, with a bounded holonomic
    /// (outer) shell. The harmonic and luminal manifolds are unbounded.
    pub fn new(dim: usize, holonomic_capacity: usize) -> Self {
        Mind {
            dim,
            harmonic: Vec::new(),
            luminal: Vec::new(),
            holonomic: Vec::new(),
            holonomic_capacity,
        }
    }

    fn stratum_mut(&mut self, m: Manifold) -> &mut Vec<Vec<BigInt>> {
        match m {
            Manifold::Harmonic => &mut self.harmonic,
            Manifold::Luminal => &mut self.luminal,
            Manifold::Holonomic => &mut self.holonomic,
        }
    }

    /// The rank — total generators across all manifolds (the mind's geometry).
    pub fn rank(&self) -> usize {
        self.harmonic.len() + self.luminal.len() + self.holonomic.len()
    }

    /// Every learned generator as the columns of a `dim × rank` matrix — the
    /// O(n) reduction basis, ordered inner → outer.
    pub fn generators(&self) -> Vec<Vec<BigInt>> {
        let cochains: Vec<&Vec<BigInt>> = self
            .harmonic
            .iter()
            .chain(&self.luminal)
            .chain(&self.holonomic)
            .collect();
        (0..self.dim)
            .map(|i| cochains.iter().map(|c| c[i].clone()).collect())
            .collect()
    }

    /// Condense (unfold): mint `obstruction` as a new generator in `manifold`.
    /// The holonomic shell refuses when full — the containment law; the inner
    /// manifolds never refuse on capacity.
    pub fn condense(
        &mut self,
        obstruction: Vec<BigInt>,
        manifold: Manifold,
    ) -> Result<(), ContainmentRefusal> {
        if obstruction.len() != self.dim {
            return Err(ContainmentRefusal::DimensionMismatch {
                expected: self.dim,
                got: obstruction.len(),
            });
        }
        if manifold == Manifold::Holonomic && self.holonomic.len() >= self.holonomic_capacity {
            return Err(ContainmentRefusal::HolonomicShellFull {
                capacity: self.holonomic_capacity,
            });
        }
        self.stratum_mut(manifold).push(obstruction);
        Ok(())
    }

    /// Bind (overlay, not fold): wrap `field` onto the mind's geometry,
    /// retaining the whole binding — which generators overlapped it, the
    /// torsion twist that let them flex onto it, and the unbound residual. The
    /// manifold binds to the problem; it does not vanish (doc 51 §6). O(n) in
    /// the mind's rank.
    pub fn bind(&self, field: &[BigInt]) -> Binding {
        overlay(field, &self.generators())
    }

    /// The irreducible residual alone — the unbound remainder of the binding.
    pub fn resolve(&self, field: &[BigInt]) -> Vec<BigInt> {
        reduce_modulo(field, &self.generators())
    }

    /// Did the mind's geometry lock onto `field` completely (empty residual)?
    pub fn resolved(&self, field: &[BigInt]) -> bool {
        self.bind(field).locked
    }

    /// Learn — the seam (IAE-5) tied to condensation (IAE-6). The proposer,
    /// seeded, guesses a bridge for `obstruction` from `base`; the exact
    /// verifier admits the first that (together with what the mind already
    /// knows) resolves it; that bridge condenses into `manifold`. Returns the
    /// mint, or `None` if unbridged within budget / refused by containment.
    pub fn learn(
        &mut self,
        seed: [u8; 32],
        base: &[Vec<BigInt>],
        obstruction: &[BigInt],
        manifold: Manifold,
        max_tries: usize,
        weight_range: i64,
    ) -> Option<Mint> {
        let existing = self.generators();
        let obs = obstruction.to_vec();
        let verify = |cand: &[BigInt]| {
            // Does the candidate, added to what the mind already holds, resolve
            // the obstruction? (The exact Universe test — O(n) in the rank.)
            let mut cols = existing.clone();
            for (i, row) in cols.iter_mut().enumerate() {
                row.push(cand[i].clone());
            }
            reduce_modulo(&obs, &cols).iter().all(|x| x.is_zero())
        };
        let mint = propose_and_verify(seed, base, verify, max_tries, weight_range)?;
        self.condense(mint.generator.clone(), manifold).ok()?;
        Some(mint)
    }
}
