//! The carrier types: one exact-rational vector per axis, three axes per state.
//!
//! An `AxisVector` is a graded coefficient list over the `Ratio` carrier
//! (index = degree/plane). Addition aligns by degree and extends — no
//! coefficient is ever truncated. Canonical form: no trailing zeros, so
//! equal values compare equal byte-for-byte (the CFE canonicality law).

use ratio::Ratio;
use num_traits::Zero;

/// The three sovereign axes of a flow state. Cross-axis leakage is
/// impossible at the type level: every operation is per-axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Spatial,
    Temporal,
    Gauge,
}

/// One axis's content: exact rational coefficients by degree,
/// canonical (no trailing zeros).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AxisVector {
    components: Vec<Ratio>,
}

impl AxisVector {
    /// Build from raw coefficients; trailing zeros are stripped so the
    /// representation is canonical.
    pub fn from_vec(mut components: Vec<Ratio>) -> Self {
        while components.last().is_some_and(Zero::is_zero) {
            components.pop();
        }
        Self { components }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn components(&self) -> &[Ratio] {
        &self.components
    }

    pub fn is_zero(&self) -> bool {
        self.components.is_empty()
    }

    /// Degree-aligned exact addition; the result keeps every plane of
    /// both operands (canonicalized).
    pub fn add(&self, other: &AxisVector) -> AxisVector {
        let len = self.components.len().max(other.components.len());
        let zero = Ratio::zero();
        let mut out = Vec::with_capacity(len);
        for i in 0..len {
            let a = self.components.get(i).unwrap_or(&zero);
            let b = other.components.get(i).unwrap_or(&zero);
            out.push(a + b);
        }
        AxisVector::from_vec(out)
    }

    /// The exact negation — the carried inverse's raw material.
    pub fn negate(&self) -> AxisVector {
        AxisVector {
            components: self.components.iter().map(|c| -c).collect(),
        }
    }

    /// Append `seed`'s coefficients as new highest-degree planes — the
    /// irreversible growth primitive (a mint). The appended planes sit
    /// beyond every degree the additive inverse can align with.
    pub fn extend_planes(&self, seed: &AxisVector) -> AxisVector {
        let mut components = self.components.clone();
        components.extend(seed.components.iter().cloned());
        AxisVector::from_vec(components)
    }
}

/// The three-axis frame: one `AxisVector` per sovereign axis.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Triple {
    pub spatial: AxisVector,
    pub temporal: AxisVector,
    pub gauge: AxisVector,
}

impl Triple {
    pub fn axis(&self, axis: Axis) -> &AxisVector {
        match axis {
            Axis::Spatial => &self.spatial,
            Axis::Temporal => &self.temporal,
            Axis::Gauge => &self.gauge,
        }
    }
}
