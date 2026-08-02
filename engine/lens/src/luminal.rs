//! THE LUMINAL LAYER — what an observer SEES of a structure
//! (`19_LENS` §10, operator ruling 2026-08-02). Carried forward from
//! `/opt/repos/convergence-engine/src/color.rs` — consulted, translated,
//! never transcribed.
//!
//! The ancestor states the whole architecture in four lines, and they
//! are the lens law written as physics:
//!
//! > *An orb inside an orb. The **holonomic** IS the outer orb — the
//! > full interference structure. The **luminal** IS the inner orb — the
//! > observation that lives INSIDE the holonomic, not beside it. The
//! > space between them is the **PHASE** — the observation angle. Luminal
//! > is a FUNCTION of holonomic at a given phase: **it emerges, it is not
//! > stored separately.** They cannot drift apart.*
//!
//! **That last clause is `19_LENS` §1a in the ancestor's own words.** A
//! reading that emerges cannot be a second image, and two things that
//! cannot drift apart cannot disagree — which is precisely why no
//! reconciler is ever needed here.
//!
//! # THE THREE AXES, and they are orthogonal
//!
//! This module does not replace [`crate::peel`]. It completes it. A
//! lens reading is fixed by three independent choices, and collapsing
//! any of them into another is what loses the geometry:
//!
//! ```text
//!     DEPTH     how close you stand        crate::peel::Depth
//!     PHASE     from what angle you look   Phase
//!     APERTURE  with what instrument       Aperture — 11 channels
//! ```
//!
//! Same structure, different depth → a coarser or finer view of the SAME
//! content. Same structure, different phase → a different manifestation
//! of the same truth. Same structure, different aperture → a different
//! observer's sensitivity across the eleven harmonic dimensions.
//!
//! # WHY THE APERTURE IS ELEVEN AND WHY IT IS NOT A RELATION
//!
//! The ancestor's `harmonic_tuning.rs` is explicit, and it settles a
//! question that would otherwise look like a contradiction:
//!
//! > *coboundary_reduce is not needed — 11 Q values have no coboundary.*
//!
//! The aperture is **not a relation**. It has no gauge, no cocycle, no
//! reduction, and therefore nothing for the peel to strip. It is a
//! per-observer SENSITIVITY across eleven dimensions:
//!
//! ```text
//!     h[0]      value / spectral       h[3]      adjacency / boundary
//!     h[1..2]   spatial / geometric    h[4..10]  higher torsion (7)
//! ```
//!
//! So the aperture and the peel are orthogonal by construction — the
//! peel removes FRAME from a relation; the aperture weights CHANNELS of
//! an observer. Neither flattens the other.
//!
//! # WHAT IS TRANSLATED, NOT TRANSCRIBED
//!
//! The ancestor derives amplitude as
//! `1 / (1 + coherence_functional(delta))`. **That is the third
//! appearance today of the same barred form** — after the struck
//! cosmetic meter `Ξ = deeds/(1+ΔD)` and the ancestral coherence
//! velocity. `18_TORSION` §2b bars collapsing a directional quantity to
//! a scalar aggregate *wherever the system computes*.
//!
//! **And this is the one place in the workspace where the law carves an
//! exception**: L1 survives *"ONLY where a reading is reported to an
//! OPERATOR lens, never where the system computes."* The lens IS that
//! surface. So an aggregate amplitude is lawful here and nowhere else.
//!
//! **THE OPERATOR DISPLAY BOUND** (operator ruling 2026-08-02). The
//! exception is bound to a SEAT, never to a species. Whoever sits in it
//! — organic, synthetic, procedural, another subsystem — meets the same
//! interface, and the bound is identical for all of them:
//!
//! > A display projection is a TERMINAL STRING. Zero accessors, zero
//! > ordering traits, zero write-back into any computing or decision
//! > path.
//!
//! [`Amplitude`] is that bound made structural. It holds its value in
//! `rfa_core::Lens`: exact rational in, `String` out, no accessor, no
//! comparison, no ordering. A caller may DISPLAY it and cannot branch on
//! it, so the exception cannot leak back by anyone's oversight. Stronger
//! than the ancestor, which left it a bare `Q` and trusted the reader.

use crate::peel::View;
use cfe::dop::ColonyOrb;
use cfe::ratio::{ratio, Ratio};
use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use rfa_core::{Delta, Lens};

/// The number of harmonic channels an observer is sensitive across.
/// Eleven, from the ancestor's substrate: one spectral, two spatial,
/// one adjacency, seven higher torsion.
pub const HARMONIC_CHANNELS: usize = 11;

/// THE PHASE — the observation angle. *"WHO is looking."*
///
/// Different phase, different manifestation, same truth. The phase does
/// not alter the structure; it selects which expression of it emerges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase(Ratio);

impl Phase {
    /// The head-on phase: the luminal expression equals the structure's
    /// own period, and the phase gap is zero — nothing is hidden.
    pub fn direct() -> Self {
        Phase(ratio(1, 1))
    }

    /// An arbitrary observation angle. A zero phase is read as `direct`,
    /// since an angle of nothing is not an angle — the ancestor's own
    /// treatment, kept because the alternative is a wavelength of zero
    /// and a division that has no meaning.
    pub fn at(angle: Ratio) -> Self {
        match angle == ratio(0, 1) {
            true => Phase::direct(),
            false => Phase(angle),
        }
    }

    pub fn angle(&self) -> &Ratio {
        &self.0
    }
}

/// THE APERTURE — an observer's sensitivity across the eleven harmonic
/// channels. *"Each entity is the same kernel viewing the same
/// observation through its own chromatic spectrum."*
///
/// **Not a relation** (see the module header): eleven exact rationals
/// with no coboundary, so nothing here is gauge and nothing reduces.
/// This is how a lens becomes per-observer without any belief entering
/// a rule (`18_TORSION` LAW 9) — the substrate is identical for every
/// observer; only the instrument differs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Aperture {
    channels: [Ratio; HARMONIC_CHANNELS],
    counts: [u32; HARMONIC_CHANNELS],
}

impl Aperture {
    /// THE FLAT APERTURE — every channel weighted alike. The observer
    /// that privileges no dimension.
    pub fn flat() -> Self {
        Aperture {
            channels: std::array::from_fn(|_| ratio(1, 1)),
            counts: [0; HARMONIC_CHANNELS],
        }
    }

    /// Weight one channel. Out-of-range channels are ignored — the
    /// aperture is total, and an eleventh-dimension typo must not panic
    /// a display.
    pub fn tuned(mut self, channel: usize, weight: Ratio) -> Self {
        if channel < HARMONIC_CHANNELS {
            self.channels[channel] = weight;
        }
        self
    }

    /// The weight this observer gives one channel; channels beyond the
    /// eleven read as unweighted.
    pub fn weight(&self, channel: usize) -> Ratio {
        match channel < HARMONIC_CHANNELS {
            true => self.channels[channel].clone(),
            false => ratio(0, 1),
        }
    }

    /// THE METABOLISM — the bounded running average the ancestor
    /// carries: `h[k]' = (h[k]·n + signal) / (n + 1)`.
    ///
    /// Exact rational throughout, no float, no root. An aperture tunes
    /// toward what it has actually observed and never past it.
    pub fn observe(&mut self, channel: usize, signal: Ratio) {
        if channel >= HARMONIC_CHANNELS {
            return;
        }
        let n = self.counts[channel];
        let weighted = self.channels[channel].clone() * ratio(i64::from(n), 1) + signal;
        self.channels[channel] = weighted / ratio(i64::from(n) + 1, 1);
        self.counts[channel] = n.saturating_add(1);
    }
}

impl Aperture {
    /// **THE APERTURE AN OBSERVER HAS EARNED** — derived from its own
    /// committed books, never assigned (`19_LENS` §6, §16).
    ///
    /// An assigned aperture is a dialed parameter wearing a
    /// per-observer name. The metabolism already says where a real one
    /// comes from — *tuning toward what has actually been observed* —
    /// and an orb's cohesion books ARE its receipted history of
    /// relation, so the bias is read from them and from nothing else.
    ///
    /// **THE HONEST MAPPING, and its limit stated rather than hidden.**
    /// The eight books are the RELATIONAL axes; the eleven channels are
    /// the HARMONIC ones. They are different axes and there is no
    /// faithful 8→11 correspondence, so only what genuinely grounds is
    /// mapped:
    ///
    /// ```text
    ///     h[0]  value / spectral     given_in_crisis + returned_across
    ///                                against extracted_across
    ///     h[3]  adjacency / boundary bond_flow against severance_load
    ///     h[4]  higher torsion       covenants_fulfilled against
    ///                                covenants_breached
    /// ```
    ///
    /// `h[1..2]` (spatial) and `h[5..10]` (the remaining torsion slots)
    /// stay FLAT. Nothing in the books speaks to them, and inventing a
    /// correspondence to fill eleven slots would be fabricating
    /// sensitivity an observer never earned — the same fault as reading
    /// an unobserved pair as a zero.
    ///
    /// Each grounded channel is fed through [`Aperture::observe`], so
    /// the weight is the bounded running average and never a raw ratio.
    pub fn of_orb(orb: &ColonyOrb) -> Self {
        let mut aperture = Aperture::flat();
        // Each pair: what this observer has DONE on that axis, against
        // what it has undone. A book that stands teaches the channel.
        for (channel, kept, lost) in [
            (
                0usize,
                orb.given_in_crisis.clone() + orb.returned_across.clone(),
                orb.extracted_across.clone(),
            ),
            (3, orb.bond_flow.clone(), orb.severance_load.clone()),
            (
                4,
                orb.covenants_fulfilled.clone(),
                orb.covenants_breached.clone(),
            ),
        ] {
            let whole = kept.clone() + lost;
            if whole != ratio(0, 1) {
                // The signal is what stands against the whole — an
                // exact rational in [0, 1], no float, no root.
                aperture.observe(channel, kept / whole);
            }
            // A book that never posted teaches nothing, and the channel
            // stays where it was. Silence is not a signal of zero.
        }
        aperture
    }
}

impl Default for Aperture {
    fn default() -> Self {
        Aperture::flat()
    }
}

/// THE AMPLITUDE — the strength of a manifestation, CAGED.
///
/// *"Zero amplitude = the structure exists holonomically but is not
/// currently manifesting luminally."*
///
/// It is an L1 aggregate, lawful here because a lens is the one surface
/// where `18_TORSION` §2b permits one — and held in
/// `rfa_core::Lens` so that permission cannot leak. The value goes in
/// exact and comes out only as text: no accessor, no ordering, no
/// comparison. **It may be shown; it may never be computed on.**
#[derive(Debug)]
pub struct Amplitude(Lens);

impl Amplitude {
    /// The reading, rendered. The only way out, by construction.
    pub fn shown(&self) -> String {
        self.0.show()
    }
}

/// THE LUMINAL EXPRESSION — what emerges when this structure is
/// observed at this phase through this aperture.
///
/// It is derived at the moment of asking and held by nobody: *"it
/// emerges, it is not stored separately."*
#[derive(Debug)]
pub struct Luminal {
    /// The period of the structure — how the wave wraps. The lcm of the
    /// denominators standing in the relation, exact.
    pub torsion_order: Ratio,
    /// The observed wavelength: the period as seen from this phase.
    pub wavelength: Ratio,
    /// How much of the structure this phase does NOT reveal, relative to
    /// the period. Zero at the direct phase; larger as the angle turns.
    pub phase_gap: Ratio,
    /// The strength of the manifestation — caged (see [`Amplitude`]).
    pub amplitude: Amplitude,
}

/// **THE EMERGENCE.** What an observer at `phase`, looking through
/// `aperture`, sees of the structure `view` reports.
///
/// Pure and total. Builds on [`crate::peel`] rather than replacing it:
/// the view fixes the DEPTH, the phase fixes the ANGLE, the aperture
/// fixes the INSTRUMENT. Nothing is retained.
///
/// The luminal is a function of the holonomic — so a caller that wants
/// a different manifestation re-observes at a different phase rather
/// than storing a second image of this one.
pub fn emerge(delta: &Delta, view: &View, phase: &Phase, aperture: &Aperture) -> Luminal {
    let torsion_order = torsion_order(delta);

    // The wavelength IS the period seen from this angle. At the direct
    // phase they coincide and nothing is hidden.
    let wavelength = torsion_order.clone() * phase.angle().clone();

    // How much the angle conceals, relative to the period.
    let phase_gap = match torsion_order == ratio(0, 1) {
        true => ratio(0, 1),
        false => (torsion_order.clone() - wavelength.clone()).abs() / torsion_order.clone(),
    };

    // AMPLITUDE — the structure that stands, weighted by the observer's
    // sensitivity on the channels it stands in. This is the translated
    // form: the ancestor divided by a caged coherence scalar; here the
    // strength derives from what the PEEL actually found, so the reading
    // is grounded in structure rather than in a barred aggregate.
    let strength = match view {
        // The far view carries an elevation: how high, weighted flat
        // because a degree names no channel.
        View::Far(degree) => ratio(degree.above() as i64, 1),
        // The middle view names its channels — each charge is weighted
        // by the observer's sensitivity on the channel that carries it.
        View::Mid(charges) => charges.iter().fold(ratio(0, 1), |sum, charge| {
            sum + charge.value.abs() * aperture.weight(charge.l)
        }),
        // The near view weights every standing entry the same way.
        View::Near(entries) => entries.iter().fold(ratio(0, 1), |sum, entry| {
            sum + entry.value.abs() * aperture.weight(entry.l)
        }),
    };

    Luminal {
        torsion_order,
        wavelength,
        phase_gap,
        amplitude: Amplitude(Lens::of(strength)),
    }
}

/// THE TORSION ORDER — the period of the structure: the lcm of every
/// denominator standing in the relation.
///
/// Exact and integer-valued by construction. It says HOW the wave wraps
/// — a relation whose entries all sit on halves wraps at 2, one that
/// needs twelfths wraps at 12.
fn torsion_order(delta: &Delta) -> Ratio {
    let mut wrap = BigInt::from(1);
    for i in 0..delta.dim() {
        for j in (i + 1)..delta.dim() {
            for l in 0..delta.fiber() {
                let denom = delta.entry(i, j, l).denom();
                if !denom.is_zero() {
                    wrap = lcm(&wrap, denom);
                }
            }
        }
    }
    Ratio::from_integer(wrap)
}

/// `lcm(a, b) = |a·b| / gcd(a, b)`, on exact big integers.
///
/// Written here rather than pulled from a numeric crate: the lens takes
/// no dependency it does not need, and two lines of Euclid are cheaper
/// than a workspace edit that every other lane would inherit.
fn lcm(a: &BigInt, b: &BigInt) -> BigInt {
    if a.is_zero() || b.is_zero() {
        return BigInt::from(0);
    }
    (a * b).abs() / gcd(a.clone(), b.clone())
}

fn gcd(mut a: BigInt, mut b: BigInt) -> BigInt {
    a = a.abs();
    b = b.abs();
    while !b.is_zero() {
        let r = a % &b;
        a = b;
        b = r;
    }
    a
}
