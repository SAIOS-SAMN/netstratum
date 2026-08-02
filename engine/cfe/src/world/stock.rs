//! THE REGENERATING STOCK — the economy's capital (20_FOUNDING §6g).
//!
//! Before this module the engine had no underlying asset: `zone_yield`
//! returned a constant per tick and a vein's `richness` never
//! decreased. Extraction drew on nothing, so nothing could be owned,
//! improved, or invested in.
//!
//! A stock is a quantity extraction DRAWS DOWN and regeneration
//! RESTORES:
//!
//! ```text
//! S(t+1) = S(t) + R(S) − E        R(S) = r · S · (1 − S / S_max)
//! ```
//!
//! **RESEARCH RAISES `rate` AND `ceiling`, NEVER THE EXTRACTION
//! MULTIPLIER.** A multiplier on a flow is a one-time step that lifts
//! the line and stops. A rise in the regeneration rate lifts the
//! CEILING, so the same research pays forever and pays more the larger
//! the stock it acts on. That is the difference between a tech tree and
//! a list of upgrades.
//!
//! # THE LATTICE — why `step` quantizes, and it is not a rounding
//! convenience
//!
//! The logistic term is quadratic in `S`, so in exact rationals its
//! denominator SQUARES every tick. Measured on this recurrence:
//!
//! ```text
//! tick  1 →      4 denominator bits      tick 10 →   6,465
//! tick  5 →    193                       tick 16 → 414,305
//! ```
//!
//! Sixteen ticks of ONE stock reach a denominator of 414,305 bits, and
//! the arithmetic itself becomes the bottleneck. This engine has
//! already ruled on a gentler version of exactly this failure: the
//! demurrage rate is FLAT and never proportional because `banked ×
//! (31/32)^n` was measured live at ~2^1100 after 220 beats. The
//! logistic form is far worse — 2^414305 after 16.
//!
//! So `S` lives on a fixed rational lattice. Regeneration is computed
//! EXACTLY and then floored to that lattice, which is a deterministic,
//! replayable operation over exact rationals — never a float, never a
//! silent rounding. Denominators stay bounded by [`LATTICE`] forever,
//! and the logistic behaviour survives intact: the curve still
//! saturates, a tended stock still outgrows a neglected one, and
//! over-extraction still collapses toward the floor.
//!
//! # THE SEED FLOOR
//!
//! A stock never reaches zero. It clamps at a strictly positive
//! `seed_floor` — scorched-earth extraction is punished by a stock too
//! small to regenerate meaningfully, never by permanent death. Left
//! fallow, a floored stock climbs back.

use crate::ratio::{ratio, Ratio};

/// The rational lattice `S` is quantized to: denominators are bounded
/// by this forever. A power of two so the floor is cheap and the bound
/// is exact.
pub fn lattice() -> Ratio {
    ratio(1024, 1)
}

/// One regenerating stock — the underlying asset extraction draws on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceStock {
    /// `S` — what stands now.
    pub current: Ratio,
    /// `S_max` — the carrying ceiling. RESEARCH expands this.
    pub ceiling: Ratio,
    /// `r` — the intrinsic regeneration rate. RESEARCH raises this.
    pub rate: Ratio,
    /// `S_min` — the seed core. Strictly positive; extraction is
    /// capped so the stock never breaches it, and it can always climb
    /// back.
    pub seed_floor: Ratio,
    /// THE LATTICE RESIDUE — what flooring shaved, CARRIED rather than
    /// discarded.
    ///
    /// Flooring without this is a silent mass leak: measured at 0.347
    /// units discarded over 400 ticks, 0.346% of the total, which the
    /// kernel's I1 conservation arm would refuse the moment a stock
    /// reached committed state. The residue makes `current + residue`
    /// exactly the pre-floor quantity at every step, so nothing enters
    /// or leaves the system. It stays bounded (measured 27 bits over
    /// 400 ticks) because it is always smaller than one lattice step.
    ///
    /// It may read slightly NEGATIVE when the seed floor lifts a stock
    /// — that is a recorded debt repaid on the next step, never matter
    /// conjured.
    pub residue: Ratio,
}

impl ResourceStock {
    /// Floor a value onto the lattice — exact, deterministic, and
    /// identical on every node, so a replay reproduces it bit-for-bit.
    fn onto_lattice(value: &Ratio) -> Ratio {
        (value.clone() * lattice()).floor() / lattice()
    }

    /// `R(S) = r · S · (1 − S / S_max)` — the syntropic term, computed
    /// exactly. Zero at the ceiling and zero at nothing; largest in
    /// between, which is what makes a tended stock compound.
    ///
    /// Reads zero rather than negative if `current` somehow stands
    /// above `ceiling`, so an over-full stock decays toward the ceiling
    /// through extraction rather than regenerating backwards.
    pub fn regeneration(&self) -> Ratio {
        let zero = ratio(0, 1);
        if self.ceiling <= zero || self.current <= zero {
            return zero;
        }
        let headroom = ratio(1, 1) - self.current.clone() / self.ceiling.clone();
        match headroom > zero {
            true => self.rate.clone() * self.current.clone() * headroom,
            false => zero,
        }
    }

    /// Advance one tick against a REQUESTED draw, returning what was
    /// actually drawn.
    ///
    /// Extraction is capped at what stands above the seed core rather
    /// than the stock being clamped up afterwards — clamping up would
    /// CREATE matter, so the draw is limited instead of the stock being
    /// topped back. A civilization asking for more than the land holds
    /// gets what the land holds, and the books still balance.
    pub fn step(&mut self, requested: &Ratio) -> Ratio {
        let zero = ratio(0, 1);
        let standing = self.current.clone() + self.regeneration() + self.residue.clone();
        let headroom = standing.clone() - self.seed_floor.clone();
        let available = match headroom > zero {
            true => headroom,
            false => zero.clone(),
        };
        let drawn = match *requested > available {
            true => available,
            false => requested.clone(),
        };
        let moved = standing - drawn.clone();
        let mut landed = Self::onto_lattice(&moved);
        if landed < self.seed_floor {
            landed = self.seed_floor.clone();
        }
        // current + residue == moved, exactly, at every step.
        self.residue = moved - landed.clone();
        self.current = landed;
        drawn
    }

    /// RESEARCH — raise the intrinsic regeneration rate. The growth
    /// mechanism: this pays forever and pays more the larger the stock.
    pub fn research_rate(&mut self, gain: &Ratio) {
        self.rate = Self::onto_lattice(&(self.rate.clone() + gain.clone()));
    }

    /// RESEARCH — expand the carrying ceiling, lifting the bound the
    /// curve saturates against.
    pub fn research_ceiling(&mut self, gain: &Ratio) {
        self.ceiling = Self::onto_lattice(&(self.ceiling.clone() + gain.clone()));
    }

    /// FLOURISH (§6f.3) — reinvest surplus directly into the stock, so
    /// abundance becomes productive base instead of a number that sits.
    pub fn reinvest(&mut self, surplus: &Ratio) {
        let raised = self.current.clone() + surplus.clone();
        let capped = match raised > self.ceiling {
            true => self.ceiling.clone(),
            false => raised,
        };
        self.current = Self::onto_lattice(&capped);
    }

    /// The extraction this stock can support without shrinking — its
    /// sustainable flow, which is exactly its regeneration.
    ///
    /// This is the operand `20_FOUNDING` §6a wants for a ward's capture
    /// capacity and `19_REGISTRY` §4b wants for opening a room: both
    /// name a growth built from a regenerating base, and this is that
    /// base's flux.
    pub fn sustainable_draw(&self) -> Ratio {
        self.regeneration()
    }
}
