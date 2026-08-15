//! THE HOMOLOGICAL ECONOMY (doc 52) — production and research are ONE peel.
//!
//! A colony's physical tension is a deficit vector `δ = Demand − Supply` over
//! the capacity axes (the `Resource` carriers — coordinate labels of a
//! `ℤ`-module, NOT control-flow enums). Peeled against the buildable structures
//! through the SAME `overlay` the cognition uses, `δ` splits into two halves of
//! one `Binding`:
//!
//!   * **COEFFICIENTS `cᵢ` → PRODUCTION.** How many of each known structure
//!     cover the deficit. The seeded builder samples the `cᵢ` distribution to
//!     choose what to raise this cycle — no cartesian enum cascade, no
//!     per-plane sweep, O(axes), not O(planes).
//!   * **RESIDUAL `γ` → RESEARCH.** The irreducible deficit no existing
//!     structure can span. Passed to the Mind's proposer as the topological
//!     defect to bridge; a verified guess mints a NEW structure-generator.
//!     Economic failure IS the tech-tree trigger — Production and Research are
//!     the same coheterological equation, read from two parts of one output.

use cfe::dop::{zone_upkeep, zone_yield, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::vocabulary::Entry;
use iae::resolve::overlay;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};

/// The lattice a rational rate is quantized onto to become a `ℤ`-module
/// coordinate — the melt grid the market already speaks. Rates are exact
/// rationals; the peel works over integers, so a rate `r` becomes `⌊r·LATTICE⌉`.
pub const LATTICE: i64 = 64;

/// The capacity axes of the physical economy: the closed `Resource` carriers.
/// (Space and play-minted elements extend this basis; a structure's footprint
/// rides the Space axis, and the verifier conserves it — doc 52 §2 / the
/// operator's Space-as-coordinate ruling. Slice 1 covers the closed carriers.)
pub fn resource_axes() -> Vec<Resource> {
    Resource::ALL.to_vec()
}

/// Quantize an exact rate onto the lattice: `⌊r·LATTICE⌉` as an integer.
fn quantize(r: &Ratio) -> BigInt {
    (r * ratio(LATTICE, 1)).round().to_integer()
}

/// A zone's NET capacity as a `ℤ`-module vector over `axes`: its yield MINUS
/// its upkeep, quantized onto the lattice. This is the `CHANNEL_YIELD −
/// CHANNEL_UPKEEP` coordinate the fold entry carries — the algebraic boundary
/// of the structure. Metabolism-aware, so a biotic and a plasmic line read
/// different vectors from the same kind.
pub fn zone_capacity(kind: ZoneKind, metabolism: u8, axes: &[Resource]) -> Vec<BigInt> {
    let mut v = vec![BigInt::zero(); axes.len()];
    let (yield_res, yield_rate) = zone_yield(kind, metabolism);
    if let Some(i) = axes.iter().position(|r| *r == yield_res) {
        v[i] += quantize(&yield_rate);
    }
    for (upkeep_res, upkeep_rate) in zone_upkeep(kind, metabolism) {
        if let Some(i) = axes.iter().position(|r| *r == upkeep_res) {
            v[i] -= quantize(&upkeep_rate);
        }
    }
    v
}

/// THE GENERATORS ARE THE FOLD (doc 52 / operator ruling). Map a species'
/// `Axis::Zone` fold (`seed ∪ minted`) into the buildable capacity generators
/// the peel reduces against — one net-capacity vector per structure the
/// civilization KNOWS, seed or self-invented. A minted entry resolves through
/// its base kind (the modulation the channels carry rides the same
/// `zone_yield`/`zone_upkeep` reading); an entry whose code no longer names a
/// buildable kind is skipped. NOT a hardcoded menu — the array grows as the
/// civilization mints.
pub fn fold_generators(
    fold: &[Entry],
    metabolism: u8,
    axes: &[Resource],
) -> Vec<(ZoneKind, Vec<BigInt>)> {
    fold.iter()
        .filter_map(|entry| {
            let code = entry.base.unwrap_or(entry.code);
            let kind = ZoneKind::from_wire(code)?;
            Some((kind, zone_capacity(kind, metabolism, axes)))
        })
        .collect()
}

/// THE STANDING SUPPLY — the aggregate net capacity of the zones already
/// raised, as a `ℤ`-module vector over `axes`: `Σ` of each standing zone's
/// `zone_capacity` (yield − upkeep). This is what the colony ALREADY
/// produces per tick before any new build, netted for the upkeep the
/// standing infrastructure itself eats. Reading it as one vector is the
/// peel's `supply` argument — no per-plane sweep, one fold over the zones.
pub fn standing_supply(kinds: &[ZoneKind], metabolism: u8, axes: &[Resource]) -> Vec<BigInt> {
    let mut sum = vec![BigInt::zero(); axes.len()];
    for &kind in kinds {
        for (i, c) in zone_capacity(kind, metabolism, axes)
            .into_iter()
            .enumerate()
        {
            sum[i] += c;
        }
    }
    sum
}

/// A live civic reading — everything the beat draws from one peel: the
/// production/research split (`peel`) and the one structure Production
/// samples to raise this cycle (`build`, `None` when the deficit is covered
/// or purely irreducible). The `build` index is resolved back to its
/// `ZoneKind` through the same fold the generators came from, so what
/// Production names is always a structure this specific civilization KNOWS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CivicReading {
    pub peel: EconomicPeel,
    pub build: Option<ZoneKind>,
}

/// RUN THE LIVE PEEL (COG Slice 1). From a species' real committed state —
/// its `Axis::Zone` fold (`seed ∪ minted`), its sealed metabolism, the
/// zones already standing, and the metabolic demand its population places —
/// compute the one economic peel the beat acts on. Demand is a set of
/// `(carrier, exact rate)` loads (population food = `pop × per_capita`,
/// leisure, …); each is quantized onto the lattice and summed onto its
/// axis. The generators are drawn STRICTLY from the fold, so a civilization
/// that has minted a new structure peels against it and one that has not
/// never sees it — the buildable set is the civilization's own vocabulary,
/// not a compiled menu.
pub fn civic_reading(
    fold: &[Entry],
    metabolism: u8,
    standing: &[ZoneKind],
    demand_loads: &[(Resource, Ratio)],
    seed: [u8; 32],
) -> CivicReading {
    let axes = resource_axes();
    let generators = fold_generators(fold, metabolism, &axes);
    let buildables: Vec<Vec<BigInt>> = generators.iter().map(|(_, v)| v.clone()).collect();
    let supply = standing_supply(standing, metabolism, &axes);
    let mut demand = vec![BigInt::zero(); axes.len()];
    for (res, rate) in demand_loads {
        if let Some(i) = axes.iter().position(|r| r == res) {
            demand[i] += quantize(rate);
        }
    }
    let peel = peel_economy(&demand, &supply, &buildables);
    let build = sample_build(&peel.coefficients, seed).map(|i| generators[i].0);
    CivicReading { peel, build }
}

/// The peel of a colony's deficit against its buildable structures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EconomicPeel {
    /// Per buildable: how many to raise to cover the deficit (PRODUCTION).
    pub coefficients: Vec<BigInt>,
    /// The TWIST — the invariant factors `> 1` the binding carried (`ℤ/dᵢ`).
    /// Retained from the resolve (it was discarded before COG-2) so the Mind
    /// can PRICE the mint that bridges this deficit: torsion is shear work.
    pub torsion: Vec<BigInt>,
    /// The irreducible deficit no structure spans — the Mind's feedstock (RESEARCH).
    pub residual: Vec<BigInt>,
    /// True iff the deficit is fully covered by known structures (`γ = 0`).
    pub covered: bool,
}

/// Peel `deficit = max(0, demand − supply)` against the buildable capacity
/// generators (each a net-capacity vector over the same axes). Only real
/// shortfalls are tension — a surplus axis is not a deficit. Returns the
/// production coefficients and the research residual, both exact.
pub fn peel_economy(
    demand: &[BigInt],
    supply: &[BigInt],
    buildables: &[Vec<BigInt>],
) -> EconomicPeel {
    let deficit: Vec<BigInt> = demand
        .iter()
        .zip(supply)
        .map(|(d, s)| {
            let gap = d - s;
            if gap.is_negative() {
                BigInt::zero()
            } else {
                gap
            }
        })
        .collect();
    let dim = deficit.len();
    // Generators as COLUMNS: `axes` rows × `buildables` cols.
    let cols: Vec<Vec<BigInt>> = (0..dim)
        .map(|i| {
            buildables
                .iter()
                .map(|g| g.get(i).cloned().unwrap_or_else(BigInt::zero))
                .collect()
        })
        .collect();
    let binding = overlay(&deficit, &cols);
    EconomicPeel {
        coefficients: binding.coefficients,
        torsion: binding.torsion,
        residual: binding.residual,
        covered: binding.locked,
    }
}

/// **THE HOMOLOGY SETS THE BILL** (COG-2). The exact metabolic work to mint the
/// generator that bridges a residual obstruction — priced from the homological
/// complexity of the void, never a flat fee. A simple linear disconnect is
/// cheap; a wide, deep, or twisted void is dear. Two terms, both EXACT integers
/// (a ℚ-module cannot carry the `ln` a continuous Landauer bound would spend, so
/// the bit-length stands in for it):
///
///   * **THE BETTI EXTENT** — `Σ |residualᵢ|`, the total unspanned deficit the
///     new generator must cover. The free rank's magnitude: more independent
///     unbound directions, and a larger shortfall along each, cost more to
///     bridge (the `Σ βₖ·cₖ` term, read off the surviving obstruction).
///   * **THE TORSION SHEAR** — `Σ bitlen(dᵢ)` over the invariant factors `> 1`.
///     Twisting a manifold onto an imperfect match `ℤ/dᵢ` takes physical shear;
///     the higher the torsion the higher the friction to snap it into a rigid
///     crystal (the `Σ penalty(dᵢ)` term).
///
/// A pure resolve output → a pure price. The Mind pays this `W_mint` from its
/// accumulated LEISURE (COG-1); a starving mind whose leisure cannot cover it
/// physically cannot afford to think (COG-3).
pub fn mint_cost(residual: &[BigInt], torsion: &[BigInt]) -> BigInt {
    let betti_extent: BigInt = residual.iter().map(|x| x.abs()).sum();
    // COVERED IS FREE. A vanished residual means the deficit is spanned by
    // structures the civilization ALREADY knows — nothing is minted, so nothing
    // is paid, whatever standing twist the generator lattice carries. The
    // torsion shear is the friction of threading a NEW generator through that
    // twist, and it applies only when there is a genuine void to bridge.
    if betti_extent.is_zero() {
        return BigInt::zero();
    }
    let torsion_shear: BigInt = torsion.iter().map(|d| BigInt::from(d.bits())).sum();
    betti_extent + torsion_shear
}

/// A BLAKE3-seeded index in `[0, bound)` — the Mind's chance, replayable.
fn seeded_index(seed: [u8; 32], salt: &[u8], bound: u64) -> u64 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed);
    hasher.update(salt);
    let block = *hasher.finalize().as_bytes();
    let v = u64::from_le_bytes(block[0..8].try_into().expect("32 ≥ 8"));
    v % bound.max(1)
}

/// Seeded-stochastic pick of WHICH buildable to raise this cycle, weighted by
/// the peel's positive coefficients — production is sampled from the
/// distribution the homology dictates, never chosen greedily. `None` when no
/// structure is called for (the deficit is covered, or purely irreducible and
/// therefore Research's problem, not Production's).
pub fn sample_build(coefficients: &[BigInt], seed: [u8; 32]) -> Option<usize> {
    let weights: Vec<u64> = coefficients
        .iter()
        .map(|c| {
            if c.is_positive() {
                c.to_u64().unwrap_or(u64::MAX)
            } else {
                0
            }
        })
        .collect();
    let total: u64 = weights.iter().fold(0u64, |sum, w| sum.saturating_add(*w));
    if total == 0 {
        return None;
    }
    let roll = seeded_index(seed, b"build", total);
    let mut acc = 0u64;
    for (i, w) in weights.iter().enumerate() {
        acc = acc.saturating_add(*w);
        if roll < acc {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(x: i64) -> BigInt {
        BigInt::from(x)
    }

    fn seed(s: &[u8]) -> [u8; 32] {
        *blake3::hash(s).as_bytes()
    }

    #[test]
    fn coefficients_drive_production_a_covered_deficit_names_the_builds() {
        // Axes = [food, metal, leisure]. Buildables: extractor→food, mine→metal,
        // refinery→leisure. A food shortfall of 4, nothing else.
        let buildables = vec![
            vec![b(2), b(0), b(0)], // extractor
            vec![b(0), b(3), b(0)], // mine
            vec![b(0), b(0), b(1)], // refinery
        ];
        let peel = peel_economy(&[b(4), b(0), b(0)], &[b(0), b(0), b(0)], &buildables);
        assert!(peel.covered, "the deficit is spanned by known structures");
        // 2 extractors cover it (2 × 2 food = 4); mine and refinery uncalled.
        assert_eq!(peel.coefficients, vec![b(2), b(0), b(0)]);
        assert!(peel.residual.iter().all(|x| x.is_zero()));
        // Production picks the called-for structure (the extractor).
        assert_eq!(sample_build(&peel.coefficients, seed(b"orb")), Some(0));
    }

    #[test]
    fn the_residual_drives_research_an_irreducible_deficit_is_innovation_feedstock() {
        // Axes = [food, metal, knowledge]. Buildables produce food/metal but
        // NOTHING produces knowledge — the colony has hit a vocabulary wall.
        let buildables = vec![
            vec![b(2), b(0), b(0)], // extractor
            vec![b(0), b(3), b(0)], // mine
        ];
        let peel = peel_economy(&[b(0), b(0), b(5)], &[b(0), b(0), b(0)], &buildables);
        assert!(!peel.covered, "no structure covers the knowledge deficit");
        assert_eq!(peel.residual, vec![b(0), b(0), b(5)], "the irreducible defect → the Mind");
        // Building farms won't fix a knowledge wall — Production is silent here;
        // this γ is Research's problem (a Sullivan mint), not Production's.
        assert!(sample_build(&peel.coefficients, seed(b"orb")).is_none());
    }

    #[test]
    fn the_generators_are_the_zone_fold_not_a_hardcoded_menu() {
        // A biotic line's buildable generators = its Axis::Zone fold (here the
        // seed; a minted civilization's fold is larger). Each is a real
        // net-capacity vector read from the channels — yield positive, upkeep
        // negative — NOT an entry in a compiled menu.
        let axes = resource_axes();
        let fold = cfe::vocabulary::seed(cfe::vocabulary::Axis::Zone);
        let generators = fold_generators(&fold, 2, &axes); // 2 = biotic
        assert!(!generators.is_empty(), "the fold yields buildable generators");

        let (_, extractor) = generators
            .iter()
            .find(|(kind, _)| *kind == ZoneKind::Extractor)
            .expect("the extractor stands in the fold");
        assert!(extractor.iter().any(|c| c.is_positive()), "the extractor PRODUCES (a +yield axis)");
        assert!(
            extractor.iter().any(|c| c.is_negative()),
            "the extractor CONSUMES upkeep (a −axis) — a multi-axis boundary, not a scalar"
        );
        // A different metabolism reads a different vector from the same fold —
        // the structure's boundary is species-relative.
        let plasmic = fold_generators(&fold, 3, &axes); // 3 = plasmic
        assert_eq!(plasmic.len(), generators.len(), "same fold, per-species capacity");
    }

    #[test]
    fn the_homology_sets_the_bill_a_wider_or_twisted_void_costs_more() {
        // A pure FREE residual (no torsion): the bill is the void's extent —
        // Σ|residualᵢ|, what the new generator must span. Nothing to twist.
        assert_eq!(mint_cost(&[b(0), b(0), b(5)], &[]), b(5), "the free void costs its extent");
        // A deeper void along the same axis costs strictly more.
        assert!(
            mint_cost(&[b(0), b(0), b(10)], &[]) > mint_cost(&[b(0), b(0), b(5)], &[]),
            "a deeper deficit is dearer to bridge"
        );
        // More independent unbound directions cost more (two axes vs one).
        assert!(
            mint_cost(&[b(3), b(0), b(5)], &[]) > mint_cost(&[b(0), b(0), b(5)], &[]),
            "a wider void — more free rank — is dearer"
        );
        // A residual threaded through a TWISTED lattice pays the shear on top:
        // binding a new generator onto a ℤ/4 twist takes friction. bitlen(4)=3.
        assert_eq!(
            mint_cost(&[b(0), b(0), b(5)], &[b(4)]),
            b(5) + b(3),
            "the bill is the Betti extent plus the torsion shear"
        );
        // Higher torsion is more friction to snap into a rigid crystal.
        assert!(
            mint_cost(&[b(0), b(0), b(5)], &[b(255)]) > mint_cost(&[b(0), b(0), b(5)], &[b(4)]),
            "a higher invariant factor shears harder"
        );
        // COVERED IS FREE: a vanished residual mints nothing, whatever twist the
        // lattice carries — the shear is paid only to bridge a genuine void.
        assert_eq!(mint_cost(&[b(0), b(0), b(0)], &[b(4)]), b(0), "no void, no bill — even through a twist");
    }

    #[test]
    fn a_covered_deficit_is_free_to_the_mind_nothing_to_bridge() {
        // A deficit fully spanned by known structures leaves no residual and no
        // torsion — there is nothing to mint, so the Mind pays nothing.
        let buildables = vec![vec![b(2), b(0), b(0)], vec![b(0), b(3), b(0)]];
        let peel = peel_economy(&[b(4), b(0), b(0)], &[b(0), b(0), b(0)], &buildables);
        assert!(peel.covered, "the deficit is spanned");
        assert_eq!(mint_cost(&peel.residual, &peel.torsion), b(0), "no void, no bill");
    }

    #[test]
    fn the_bill_is_priced_from_the_real_peel_of_a_knowledge_wall() {
        // The residual-drives-research peel: nothing produces the knowledge
        // axis, so a deficit of 5 there is irreducible. The Mind's bill to mint
        // the missing generator is priced from THAT residual — the same homology
        // the research trigger reads, not a synthetic number.
        let buildables = vec![vec![b(2), b(0), b(0)], vec![b(0), b(3), b(0)]];
        let peel = peel_economy(&[b(0), b(0), b(5)], &[b(0), b(0), b(0)], &buildables);
        assert!(!peel.covered, "the knowledge wall stands");
        let bill = mint_cost(&peel.residual, &peel.torsion);
        assert!(bill > b(0), "an irreducible deficit carries a real bill: {bill}");
        // The bill is the void's extent (5 units of unspanned knowledge) PLUS
        // the shear of threading the new generator through the economy's own
        // standing twist: the food/metal generators (yields 2 and 3) span a
        // lattice with invariant factor ℤ/6, and bitlen(6) = 3. A more baroque
        // economy is harder to innovate in — the homology says so exactly.
        let extent: BigInt = peel.residual.iter().map(|x| x.abs()).sum();
        let shear: BigInt = peel.torsion.iter().map(|d| BigInt::from(d.bits())).sum();
        assert_eq!(extent, b(5), "the unspanned knowledge deficit");
        assert_eq!(bill, extent + shear, "extent plus the ambient lattice shear");
        assert_eq!(bill, b(8), "5 unspanned + 3 shear through the ℤ/6 twist");
    }

    #[test]
    fn production_is_sampled_stochastically_and_replayably() {
        // Coefficients call for 3 extractors and 1 refinery — a distribution,
        // not a greedy pick. Seeded from state → replayable.
        let coeffs = vec![b(3), b(0), b(1)];
        let pick = sample_build(&coeffs, seed(b"orb-7"));
        assert!(matches!(pick, Some(0) | Some(2)), "picks a called-for structure, weighted");
        assert_eq!(sample_build(&coeffs, seed(b"orb-7")), pick, "same seed → same build");
    }
}
