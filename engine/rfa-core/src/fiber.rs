//! The non-abelian FIBER — a `2×2` matrix connection on a relational
//! edge (directive 17, operator ruling 2026-08-01).
//!
//! **Why the flat fiber could not move.** With a scalar connection the
//! curvature is `F = dA` and the bracket vanishes identically:
//! `[A, A] = 0`. A closed loop of scalars therefore carries no field at
//! all — walking it sums to zero and the loop is a flat, dead circle.
//! That is why a ring of equal obligations read Stasis at every size:
//! not a fact about the ring, an artefact of the fiber.
//!
//! With a matrix connection the curvature keeps its second term —
//! `F = dA + A ∧ A` —
//! and `[A_i, A_j] ≠ 0` in general. The circulation is the field. A
//! closed loop of non-commuting connections carries flux, so the same
//! ring is a flywheel rather than a remnant, and it wakes without any
//! external charge bolted onto a vertex.
//!
//! **The holonomy is a product, not a sum.** That is the whole
//! difference. An abelian loop accumulates `Σ` and cancels; a
//! non-abelian loop accumulates an ordered product `A₁A₂…Aₙ`, and even
//! identical factors give `Aⁿ ≠ I`. Order stops being irrelevant, which
//! is what it means for a space to have a magnetic field.
//!
//! Exact rational throughout — no float, no root, and `l1_norm` for
//! magnitude so squares stay barred as aggregators.

use cfe::ratio::{ratio, Ratio};
use num_traits::Signed;

/// A `2×2` matrix connection along one relational edge, over exact
/// rationals. Members of `SL₂` satisfy `det = 1`; the type does not
/// enforce it, because the gauge law is a reading (see [`Self::is_sl2`])
/// and a refusal names what it found rather than being unconstructible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatrixFiber2x2 {
    pub m: [[Ratio; 2]; 2],
}

impl MatrixFiber2x2 {
    /// Build from four exact rationals, row-major.
    pub fn new(a: Ratio, b: Ratio, c: Ratio, d: Ratio) -> Self {
        Self { m: [[a, b], [c, d]] }
    }

    /// The identity connection — a relation that shifts no frame.
    pub fn identity() -> Self {
        Self::new(ratio(1, 1), ratio(0, 1), ratio(0, 1), ratio(1, 1))
    }

    /// The zero fiber — no connection at all.
    pub fn zero() -> Self {
        Self::new(ratio(0, 1), ratio(0, 1), ratio(0, 1), ratio(0, 1))
    }

    /// `det` — exact. `SL₂` requires this to be one.
    pub fn det(&self) -> Ratio {
        self.m[0][0].clone() * self.m[1][1].clone() - self.m[0][1].clone() * self.m[1][0].clone()
    }

    /// Is this connection volume-preserving (`det = 1`)?
    pub fn is_sl2(&self) -> bool {
        self.det() == ratio(1, 1)
    }

    /// `tr` — the trace.
    pub fn trace(&self) -> Ratio {
        self.m[0][0].clone() + self.m[1][1].clone()
    }

    /// Matrix product, exact.
    pub fn matmul(&self, other: &Self) -> Self {
        let (a, b) = (&self.m, &other.m);
        Self {
            m: [
                [
                    a[0][0].clone() * b[0][0].clone() + a[0][1].clone() * b[1][0].clone(),
                    a[0][0].clone() * b[0][1].clone() + a[0][1].clone() * b[1][1].clone(),
                ],
                [
                    a[1][0].clone() * b[0][0].clone() + a[1][1].clone() * b[1][0].clone(),
                    a[1][0].clone() * b[0][1].clone() + a[1][1].clone() * b[1][1].clone(),
                ],
            ],
        }
    }

    /// Entrywise difference.
    pub fn minus(&self, other: &Self) -> Self {
        let (a, b) = (&self.m, &other.m);
        Self {
            m: [
                [a[0][0].clone() - b[0][0].clone(), a[0][1].clone() - b[0][1].clone()],
                [a[1][0].clone() - b[1][0].clone(), a[1][1].clone() - b[1][1].clone()],
            ],
        }
    }

    /// **The lie bracket** `[A, B] = AB − BA` — the term that does not
    /// exist in a scalar fiber. Where this is non-zero the connection is
    /// non-abelian and a closed loop carries field.
    pub fn commutator(&self, other: &Self) -> Self {
        self.matmul(other).minus(&other.matmul(self))
    }

    /// Transpose — the congruence law's right factor.
    pub fn transpose(&self) -> Self {
        Self::new(
            self.m[0][0].clone(),
            self.m[1][0].clone(),
            self.m[0][1].clone(),
            self.m[1][1].clone(),
        )
    }

    /// L1 magnitude — the taxicab sum of entries. Squares stay barred as
    /// aggregators (the proportional law); no root is ever taken.
    pub fn l1_norm(&self) -> Ratio {
        self.m[0][0].abs() + self.m[0][1].abs() + self.m[1][0].abs() + self.m[1][1].abs()
    }

    pub fn is_zero(&self) -> bool {
        self.l1_norm() == ratio(0, 1)
    }

    pub fn is_identity(&self) -> bool {
        *self == Self::identity()
    }
}

/// **The LOOP holonomy** — the ordered product of the connections around
/// a closed walk.
///
/// This is where the abelian and non-abelian cases part company. A
/// scalar loop accumulates a sum and a balanced ring cancels to zero. A
/// matrix loop accumulates a product, so order matters and even
/// identical factors give `Aⁿ`, which is the identity only in special
/// cases. A holonomy that is not the identity is flux the loop is
/// holding.
///
/// An empty walk holds the identity, lawfully — nowhere to go, nothing
/// acquired.
pub fn loop_holonomy(walk: &[MatrixFiber2x2]) -> MatrixFiber2x2 {
    walk.iter().fold(MatrixFiber2x2::identity(), |carried, step| carried.matmul(step))
}

/// **The curvature of A plaquette** — `F = dA + A ∧ A` read on the
/// smallest closed cell the connection can carry.
///
/// The discrete form of the second term is the bracket: for two
/// directions `A` and `B`, the field is what fails to commute. The
/// first term vanishes on a plaquette whose connections are constant
/// along each direction, so on a uniform cell the curvature is the
/// bracket — and it is exactly zero for every scalar fiber.
pub fn plaquette_curvature(a: &MatrixFiber2x2, b: &MatrixFiber2x2) -> MatrixFiber2x2 {
    a.commutator(b)
}
