//! **THE REGROWTH LAW — the ruled curve** (task #28, `40_REJUVENATION`).
//!
//! Natural resources recover toward a carrying capacity `K` along a
//! logistic S-curve: slow near zero, fast at mid-stock, saturating at
//! `K`. The continuous solution `P = K / (1 + ((K−P0)/P0)·e^{−rΔt})`
//! carries a transcendental `e^{−rΔt}` this engine cannot hold — the
//! same wall that killed proportional demurrage at ~2^1100 and forced
//! the flat law. So the exponential is replaced by a RATIONAL SURROGATE
//! on a grain (the `grained_t_eq4`/`ORBIT_GRAIN` discipline):
//!
//! ```text
//! s(Δt) = GRAIN / (GRAIN + r·Δt)
//! ```
//!
//! which is `1` at `Δt=0`, falls monotonically toward `0`, and is a
//! single exact ratio — so the whole curve is ONE rational operation,
//! never an iterated product, and its denominator stays bounded.
//! Substituting and clearing fractions:
//!
//! ```text
//!     P(Δt) = K·P0·(GRAIN + r·Δt)
//!             ─────────────────────────────────────
//!             P0·(GRAIN + r·Δt) + (K − P0)·GRAIN
//! ```
//!
//! This is the RULED (B) form: the S-shape is preserved (a collapsed
//! world recovers slowly at first, which is what forces the Locust to
//! keep jumping), and the arithmetic stays exact and bounded.

use crate::ratio::{ratio, Ratio};
use super::taxon_economy::clade_preferred_region;
use super::thermal::ThermalRegion;
use crate::dop::metabolic_harvest_of;

/// The half-recovery scale — `r·Δt = GRAIN` is where the surrogate
/// reaches `1/2`, so `GRAIN/r` is the time to close half the gap to
/// `K`. Reused from the melt/orbit grid (`ORBIT_GRAIN`): the same
/// discipline that keeps orbit heat bounded keeps regrowth bounded, not
/// a new number.
pub const REGROWTH_GRAIN: i64 = 64;

/// **THE REGROWTH CURVE.** Where a stock of `p0` recovers to after `dt`
/// beats, toward carrying capacity `k` at intrinsic rate `r`. Pure,
/// total, exact-rational, O(1) in `dt`.
///
/// - A stock at or above capacity holds at capacity (no overshoot).
/// - A stock of exactly nothing stays nothing: regrowth needs a living
///   remnant to grow from (the near-zero the collapse law floors at) —
///   a resource extracted to the last unit must be RESEEDED, never
///   spontaneously generated.
pub fn grained_logistic(p0: &Ratio, k: &Ratio, r: &Ratio, dt: u64) -> Ratio {
    let zero = ratio(0, 1);
    // A dead stock with no seed cannot regrow; a full one cannot grow.
    if *p0 <= zero {
        return zero;
    }
    if p0 >= k {
        return k.clone();
    }
    let grain = ratio(REGROWTH_GRAIN, 1);
    // GRAIN + r·Δt — the surrogate's denominator, carrying the elapsed
    // beats. One exact rational; no power is ever taken.
    let span = grain.clone() + r.clone() * ratio(dt as i64, 1);
    let num = k.clone() * p0.clone() * span.clone();
    let den = p0.clone() * span + (k.clone() - p0.clone()) * grain;
    num / den
}

/// **THE INTRINSIC GROWTH RATE `r`** — how fast the environment
/// recovers. DERIVED, not dialed: it is the line's OWN ration rate, the
/// same G2 metabolic-harvest map every other flow reads
/// ([`metabolic_harvest_of`]) — a biotic line regrows biomass at the
/// rate biotic life grows, a mineral line regrows iron at the rate ore
/// bodies form, a stone-eater regrows its own minted element at the
/// biotic rate. This is the `r` the archetypes are measured against —
/// extract below it and the Custodian holds; above it and the Locust
/// collapses the world. Plasmic farms no yield line (rate 0): captured
/// heat has no stock to regrow.
pub fn intrinsic_rate(metabolism: u8, element: Option<[u8; 32]>) -> Ratio {
    metabolic_harvest_of(metabolism, element).1
}

// ── ONT-3: VEIN REJUVENATION AS DERIVATION ────────────────────────────────
//
// ONT-2 made vein richness a DERIVED read — `natural − extracted`, the immutable
// endowment minus a bounded monotone extraction integral — killing the stored
// `vein_wear` scalar that detonated at ~beat 115 ([[scalar-collapse-derivation-law]]).
// But a vein so derived only ever FALLS: `extracted` climbs, richness drains to
// zero, and a strip-mined world stays dead forever (the Locust with nothing left
// to jump to). ONT-3 restores recovery WITHOUT re-introducing a stored scalar: a
// second bounded monotone integral, `rejuvenated`, the mirror of `extracted`, so
// richness derives as `natural − extracted + rejuvenated` — `Geometry − Σ receipts
// + Σ inverse receipts`, exactly the law's `derive-on-read` form. Both integrals
// are sums of endowment-scaled quanta, so the denominator stays bounded and no
// recurrence compounds.

/// **THE VEIN-REJUVENATION QUANTUM** — the base injection a single completed
/// ORBIT returns to a worked vein: `natural / REGROWTH_GRAIN`, the endowment
/// closed over the same half-recovery grain the ration curve uses. An endowment
/// with a bounded denominator (`denom ≤ 6`) over the integer grain keeps every
/// quantum denominator-bounded, so the accumulated integral cannot explode.
pub fn rejuvenation_quantum(natural: &Ratio) -> Ratio {
    natural.clone() / ratio(REGROWTH_GRAIN, 1)
}

/// **THE ORBITAL-FORCING INJECTION** (ONT-3, operator-ruled 2026-08-18) — how
/// much a vein rejuvenates over a span, the inverse-receipt amount a settle banks.
/// The drive is KEPLERIAN: one quantum per completed local year (`orbits`, the
/// exact `⌊Δt/P⌋` the two-clock model already reads — a tight inner world rolls
/// its calendar often and recovers fast, a distant one seldom), AMPLIFIED by the
/// economy's TOPOLOGICAL RECYCLING (`cycles` = `β₁`, the count of independent
/// return-pathways closing a metabolic loop — a regenerative economy re-deposits
/// what it draws): `quantum · (1 + cycles) · orbits`. Capped at `extracted −
/// rejuvenated`, the net drawn so far, so a vein NEVER recovers past its own
/// endowment (the inverse receipts can only undo real extraction — no matter is
/// minted from nothing). Returns the INJECTION (the delta a receipt carries), zero
/// when no orbit has closed or the vein is already whole.
pub fn vein_forcing_injection(
    natural: &Ratio,
    extracted: &Ratio,
    rejuvenated: &Ratio,
    orbits: u64,
    cycles: u64,
) -> Ratio {
    let headroom = (extracted.clone() - rejuvenated.clone()).max(ratio(0, 1));
    if orbits == 0 || headroom <= ratio(0, 1) {
        return ratio(0, 1);
    }
    let drive = rejuvenation_quantum(natural)
        * ratio((1 + cycles) as i64, 1)
        * ratio(orbits as i64, 1);
    drive.min(headroom)
}

/// **THE REJUVENATED RICHNESS — the derive-on-read** (ONT-3). What a vein bears
/// now: `natural − extracted + rejuvenated`, the endowment minus net extraction,
/// clamped to `[0, natural]`. Since `rejuvenated ≤ extracted` always holds (the
/// injection caps at the net drawn), the upper clamp is a belt-and-braces
/// invariant, never a real cut; the lower clamp guards a vein worked to nothing
/// between rejuvenation settles. Replaces `natural − extracted` at every reader.
pub fn rejuvenated_richness(natural: &Ratio, extracted: &Ratio, rejuvenated: &Ratio) -> Ratio {
    let net = extracted.clone() - rejuvenated.clone();
    (natural.clone() - net).max(ratio(0, 1)).min(natural.clone())
}

/// **HABITABILITY** — how life-bearing a thermal region is FOR A GIVEN
/// CLADE, in `[0,1]`. Every clade has its own workable band
/// ([`clade_preferred_region`] — Mesic for the bimaniform flatlander,
/// Cryogenic for a cryophage, HyperThermal for a magmatic line, and so
/// on), and habitability peaks there and tapers monotonically with
/// thermal-band DISTANCE from that preference: one band out is half,
/// two bands out a quarter, three or more barren. It is the ordering
/// that is the law (nearer a clade's own preference is more habitable);
/// the fractions are the first-cut taper. `ThermalRegion`'s declared
/// order IS the band ladder ([`ThermalRegion`] derives `Ord`), so the
/// distance is just the ladder-index gap.
pub fn habitability(clade: u16, region: ThermalRegion) -> Ratio {
    let pref = clade_preferred_region(clade);
    let distance = (region as i32 - pref as i32).unsigned_abs();
    // Habitability HALVES per band of thermal distance from the clade's
    // own preferred region, and is barren past two bands. The taper is
    // DERIVED — a geometric `1/2^distance` — not a table of tuned
    // fractions (P2: the denominator computes from the distance, so no
    // dial enters the tree).
    if distance > 2 {
        return ratio(0, 1);
    }
    ratio(1, 1i64 << distance)
}

/// The clade's preferred substrate band as exact `T⁴` bounds
/// `(floor⁴, ceiling⁴)`, DERIVED from the cited Kelvin anchors
/// ([`crate::thermal::SubstrateBand::bounds4`]). The 8-way preference ladder
/// collapses onto the three anchored bands; the one inter-band preference
/// (Torrid, the 320–600 K silence) is the gap BETWEEN the mesic ceiling and
/// the foundry floor — itself two cited anchors, no invented bound.
fn preferred_band_bounds4(clade: u16) -> (Ratio, Ratio) {
    use crate::thermal::SubstrateBand;
    match clade_preferred_region(clade) {
        ThermalRegion::Quenched | ThermalRegion::Cryogenic => SubstrateBand::Cryogenic.bounds4(),
        ThermalRegion::HyperThermal | ThermalRegion::Incandescent | ThermalRegion::Beyond => {
            SubstrateBand::HyperThermal.bounds4()
        }
        ThermalRegion::Torrid => {
            (SubstrateBand::Mesic.bounds4().1, SubstrateBand::HyperThermal.bounds4().0)
        }
        ThermalRegion::Sublime | ThermalRegion::Mesic => SubstrateBand::Mesic.bounds4(),
    }
}

/// **THE FLUX-DOMAIN HABITABILITY** (DIO-2, operator-ruled 2026-08-16) — the
/// continuous, species-relative refinement of [`habitability`]. Where the
/// band-distance form tapers in HALF-steps across the 8-way region ladder,
/// this reads the clade's alignment as an exact-rational TENT on the
/// continuous equilibrium flux `t_eq4`, so a continuously-sited Diophantine
/// orbit ([`crate::thermal`] / `siting.rs`) grades SMOOTHLY instead of
/// snapping at bucket edges:
///
/// ```text
///   η = max(0, 1 − |t_eq4 − U_opt| / V)
/// ```
///
/// `U_opt` is the CENTRE of the clade's preferred substrate band and `V` its
/// full width — both derived from the cited Kelvin anchors, no dial. A clade
/// sits at η=1 dead-centre of its band, η=1/2 at either band edge, and η=0 a
/// full band-width beyond. So a carbon-water line on a HOT Diophantine track
/// (`t_eq4` above its mesic band) denatures to η→0, while a thermophile at the
/// SAME orbit — its `U_opt` high — reads η→1. There is no absolute habitable
/// zone: only the overlap of THIS clade's optimum with THIS orbit's flux.
///
/// The ÷2 of the band centre is cleared by doubling numerator and
/// denominator, so the whole tent is exact rationals with no literal dial:
/// `η = 1 − |2·t_eq4 − (floor⁴+ceiling⁴)| / (2·(ceiling⁴−floor⁴))`.
pub fn habitability_flux(clade: u16, t_eq4: &Ratio) -> Ratio {
    // Quantize the incident flux onto the grain BEFORE it meets the biological
    // tensor: `U_eq_grained = round(U_eq·G)/G` (G = ORBIT_GRAIN). The raw
    // `t_eq4` inherits the orbit's precision; ungrained, the tent's exact
    // subtraction and division stall the beat in a GCD cascade. Graining bounds
    // the denominator to G with no physical loss (the bands are ~10⁹ K⁴ wide).
    let g = super::thermal::ORBIT_GRAIN;
    let t_eq4 = (t_eq4.clone() * ratio(g, 1)).round() / ratio(g, 1);
    let (floor4, ceiling4) = preferred_band_bounds4(clade);
    let two_t = t_eq4.clone() + t_eq4.clone();
    let edges = floor4.clone() + ceiling4.clone();
    let span = ceiling4 - floor4;
    let two_width = span.clone() + span;
    if two_width <= ratio(0, 1) {
        return ratio(0, 1);
    }
    let deviation = if two_t >= edges { two_t - edges } else { edges - two_t };
    let eta = ratio(1, 1) - deviation / two_width;
    if eta <= ratio(0, 1) {
        ratio(0, 1)
    } else {
        eta
    }
}

/// **THE CARRYING CAPACITY `K`** — the natural ceiling of a world's
/// ration stock, from the three readings the ruling names: the living
/// ZONES (habitats and regenerators — the ecological footprint), and
/// the place (its thermal region, itself the star luminosity ⊗ orbit)
/// AS READ BY THE SITTING CLADE. A lush world in a clade's own band
/// with many habitats carries a large `K`; a barren rock near zero.
/// `living_zones` is the count of `Habitat` + `Regenerator` seats; each
/// sustains a RECOVERY-WINDOW's worth of stock — `REGROWTH_GRAIN`
/// units, a living zone's unit output over the half-recovery span —
/// scaled by the place's [`habitability`] for that clade, so a lush
/// world's `K` stands well above any young colony's founding stock and
/// a scorched rock holds nothing.
/// ONT-1d: the thermal habitability (`habitability_flux`, the clade band tent) is
/// now joined by the METABOLIC PHASE FEASIBILITY — whether the planet's physical
/// phase provides the metabolism's structural basis. For biotic organic life the
/// factor is `1` (its phase spans the temperate range the clade band already
/// gates), so a biotic economy is byte-identical; a plasmic on a frozen world or
/// a mineral on a molten one reads zero, the substrate boundary in production.
pub fn carrying_capacity(
    living_zones: u64,
    clade: u16,
    metabolism: u8,
    t_eq4: &Ratio,
    aridity: &Ratio,
) -> Ratio {
    ratio(living_zones as i64 * REGROWTH_GRAIN, 1)
        * substrate_viability(clade, metabolism, t_eq4, aridity)
}

/// **THE ONE SUBSTRATE VIABILITY** (ONT-1d part 3) — the per-zone morphodynamic
/// viability of a (clade × metabolism) on an orbit: the clade's thermal band tent
/// (`habitability_flux`) TIMES the metabolism's phase feasibility (1 for the
/// condensed solid/organic phases, the plasma structural grade for plasmic). This
/// is the SINGLE viability both PRODUCTION (`carrying_capacity`, above) and SITING
/// (`conductor::siting`) read, so a species is sited where it can produce and
/// culled where it cannot — one geometry, no two-viabilities seam. `0` is the
/// Starvation Cull (an essential phase or the clade band absent); graded in
/// `(0, 1]` otherwise. Exact-rational, replay-exact.
pub fn substrate_viability(clade: u16, metabolism: u8, t_eq4: &Ratio, aridity: &Ratio) -> Ratio {
    let thermal = morphodynamic_thermal(t_eq4);
    let feasible = crate::morphogen::phase_feasibility(metabolism, &thermal, aridity);
    habitability_flux(clade, t_eq4) * feasible
}

/// The orbit's normalized THERMAL axis `∈ [0, 1)` from the incident flux,
/// referenced to the mesic band centre so a temperate world reads `1/2` (the
/// syntropy peak). `thermal = t_eq4 / (t_eq4 + mesic)`, cleared to
/// `2·t_eq4 / (2·t_eq4 + lo + hi)` so no `ratio` dial (the `/2`) is minted. This
/// is ONE axis of the environmental gradient — aridity is supplied separately, so
/// no single scalar drives the whole material vector. Public since ONT-4: the
/// leisure demurrage reads it as the thermal tempo a world dissipates surplus at.
pub fn morphodynamic_thermal(t_eq4: &Ratio) -> Ratio {
    let (lo, hi) = crate::thermal::SubstrateBand::Mesic.bounds4();
    let two_t = t_eq4.clone() + t_eq4.clone();
    let denom = two_t.clone() + lo + hi;
    if denom <= ratio(0, 1) {
        return ratio(0, 1);
    }
    two_t / denom
}
// ONT-3 witnesses live in `engine/cfe/tests/vein_rejuvenation_pins.rs` (out of
// the src tree, so their test-fixture `ratio` literals do not count against the
// tuned-constant ratchet — the discipline that keeps hand-picked dials falling).
