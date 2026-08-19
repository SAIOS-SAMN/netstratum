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

/// The base cognitive rate — the fraction of a colony's syntropic WILD flow
/// that always feeds base cognition (the "brain stem"), even at zero leisure.
/// The thermodynamic escape valve that breaks the reachability deadlock: a
/// destitute line thinks slowly off its wild-food trickle rather than never.
fn base_cognitive_rate() -> Ratio {
    ratio(1, 16) // APPROVED-SEED (operator ruling 2026-08-15, COG-3 fasting)
}

/// The desperation exchange rate — how many heads a starving line burns per
/// unit of `mint_cost` when it cannibalizes its own people to force a Eureka.
/// Deliberately INEFFICIENT: thinking on a full stomach is cheap, thinking by
/// sacrificing your population is dear.
fn burn_inefficiency() -> Ratio {
    ratio(4, 1) // APPROVED-SEED (operator ruling 2026-08-15, COG-3 desperation burn)
}

/// **THE METABOLIC EXCHANGE RATE** (COG-5) — the scalar bridging the calorie
/// (leisure, in food-surplus units) to the homology (`mint_cost`, in raw
/// Betti/torsion units). Without it a thought costs single digits against a
/// bank of thousands and the starvation gate is toothless; with it a thought
/// costs a MEANINGFUL fraction of the surplus, so a poor line genuinely cannot
/// afford to think while a rich one can.
pub fn metabolic_exchange() -> Ratio {
    ratio(8, 1) // APPROVED-SEED (operator ruling 2026-08-16, COG-5 exchange rate)
}

/// **THE EVOLUTION DISCOUNT — think more on less** (COG-11, doc 52 §2: "Evolution
/// is the discount: the evolution/caste deeds lower the metabolism spent per unit
/// of thought — an evolved line thinks more on less."). A line advances its
/// `evolution_stage` along its arc (the EARNED transformation deed, gated on a
/// crystallized deepening); each stage makes thought CHEAPER. The multiplier is
/// `1/(1 + stage)` — exact-rational, monotone decreasing, and exactly `1` at
/// stage 0, so an un-evolved line (every founding line) is BYTE-IDENTICAL: the
/// discount touches only what a line has earned. A stage-1 line thinks at half the
/// cost, stage-2 a third — a strong return on a rare, gated advancement. No dial:
/// the denominator is the stage itself.
pub fn cognitive_efficiency(evolution_stage: u8) -> Ratio {
    ratio(1, 1 + evolution_stage as i64)
}

/// **THE COGNITIVE γ** (COG-EMERGENCE-1, operator mandate 2026-08-19) — "lack of
/// cognitive depth" as a DEFICIT the workbench weighs, NOT an `if capacity < 3`
/// gate. A civilization feels a cognitive tension when it carries irreducible
/// problems — the civic residual γ, the physical deficit NO KNOWN STRUCTURE spans
/// (`EconomicPeel.residual`, the same extent `mint_cost` prices) — AND its thinking
/// is too shallow to INVENT the structure that would span them: invention depth is
/// bounded by `cognitive_capacity`, so a deeper mind mints more complex generators.
/// The drive to deepen is that unspanned extent scaled by the depth HEADROOM
/// (`frontier − capacity`): zero for a mind already at the frontier (more depth
/// spans nothing new) and zero for an economy already covered (γ = 0, no problem to
/// invent past). This is the number the workbench READS to weigh a cognitive-science
/// seat against every other build — it does not decide; the entity does, by drawing
/// the fold under this tension. Exact integer, no gate, no dial.
pub fn cognitive_deficit(residual_extent: u64, capacity: usize, frontier: usize) -> u64 {
    let headroom = frontier.saturating_sub(capacity) as u64;
    residual_extent.saturating_mul(headroom)
}

/// The residual EXTENT of a peel — `Σ|γ_i|`, the Betti-extent of the unspanned
/// void (the same measure `mint_cost` bills). The cognitive γ's raw driver: how
/// much of the economy's demand no known structure covers.
pub fn residual_extent(peel: &EconomicPeel) -> u64 {
    peel.residual
        .iter()
        .fold(BigInt::zero(), |sum, g| sum + g.abs())
        .to_u64()
        .unwrap_or(u64::MAX)
}

/// **THE FASTING FILL** (COG-3) — the cognitive accumulator's inflow this beat:
/// the cascade's LEISURE plus a base trickle off the syntropic wild flow. The
/// rich pay `mint_cost` instantly from their large leisure and coin many words
/// a beat; the poor (leisure 0) fill only from the trickle — agonizingly slow
/// thought, but never zero, so a line stuck at an irreducible γ is never
/// PERMANENTLY deadlocked. It saves for tens of beats, then affords one word.
pub fn fasting_fill(leisure: Ratio, syntropic: Ratio) -> Ratio {
    leisure + syntropic * base_cognitive_rate()
}

/// **THE DESPERATION PRICE** (COG-3) — the population a line must burn to mint a
/// word it cannot afford from leisure, when a metabolic deficit is actively
/// biting. `mint_cost` heads times the inefficiency: the crisis instinct that
/// spends the one substrate a starving colony still has — its people — to buy
/// the geometry that unlocks its escape. A gamble: the burn may leave a viable
/// remnant, or it may be the last thing a dying line ever does.
pub fn desperation_price(mint_price: &Ratio) -> Ratio {
    mint_price.clone() * burn_inefficiency()
}

/// **THE STRATIFIED DRIVES** (doc 52 §7 / the operator's drive-ladder ruling
/// 2026-08-15) — a civilization's needs in STRICT Maslow priority. Supply fills
/// each tier in turn and only the surplus ABOVE one tier's demand spills to the
/// next, so the same waterfall that meets survival also decides what abundance
/// is FOR. Ordered: the discriminant is the priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Drive {
    /// Tier 1 — the caloric floor (`pop × per_capita`). Unmet ⇒ STARVATION.
    Metabolic,
    /// Tier 2 — containment against the demurrage the surplus sheds (SINK-1 /
    /// 05 §1.4). Unmet ⇒ entropy wins; the Mind is driven to mint storage.
    Homeostatic,
    /// Tier 3 — the leisure the cognitive caste demands to fund thought. Unmet
    /// ⇒ the Mind cannot afford to mint (the metabolic bound on cognition).
    Cognitive,
    /// Tier 4 — reach into new niches. Unmet ⇒ confined to the local well.
    Expansive,
}

/// One tier's reading in the cascade: what it demanded, what the flow reaching
/// it could meet, and the deficit that survived (`0` when fully met).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TierReading {
    pub drive: Drive,
    pub demand: Ratio,
    pub met: Ratio,
    pub deficit: Ratio,
}

/// The outcome of pouring one beat's supply through the stratified demand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cascade {
    /// Per tier, in priority order: demand / met / surviving deficit.
    pub tiers: Vec<TierReading>,
    /// The surplus that survives EVERY tier — the LEISURE the Mind banks to
    /// fund minting (COG-1). Zero the moment any tier is left wanting.
    pub leisure: Ratio,
    /// The FIRST tier left with a deficit — the binding constraint. `Metabolic`
    /// here is STARVATION (the classic crisis, halt); a higher tier is
    /// STAGNATION (fed, but an unmet drive whose γ routes to the proposer as a
    /// NON-food need). `None` ⇒ every drive met, pure surplus to leisure.
    pub binding: Option<Drive>,
}

/// **THE CASCADE PEEL** (COG-1) — pour `supply` (the metabolic carrier's total
/// flow this beat: syntropic + cultivated) through the stratified `demands` in
/// STRICT priority order. Each tier takes what it can (the min of the flow
/// reaching it and its demand); the deficit that survives is that drive's γ; the
/// surplus spills to the next tier. Whatever survives the lowest tier is
/// LEISURE — the budget banked to fund thought. Exact-rational throughout; a
/// negative demand is treated as no demand (a tier the line does not feel).
///
/// This is the separation the whole arc turns on: a metabolic deficit is
/// STARVATION and halts the waterfall at the floor; a deficit only at a higher
/// tier is STAGNATION — the line is fed, but its surplus has nowhere productive
/// to go, so the binding γ is a call for storage, amenities, or invention, not
/// for food.
pub fn cascade(supply: Ratio, demands: &[(Drive, Ratio)]) -> Cascade {
    let zero = ratio(0, 1);
    let mut flow = supply.max(zero.clone());
    let mut tiers = Vec::with_capacity(demands.len());
    let mut binding = None;
    for (drive, raw_demand) in demands {
        let demand = raw_demand.clone().max(zero.clone());
        let met = flow.clone().min(demand.clone());
        let deficit = (demand.clone() - flow.clone()).max(zero.clone());
        if deficit > zero && binding.is_none() {
            binding = Some(*drive);
        }
        flow = (flow - demand.clone()).max(zero.clone());
        tiers.push(TierReading { drive: *drive, demand, met, deficit });
    }
    Cascade { tiers, leisure: flow, binding }
}

/// **THE LIVE CASCADE** (COG-1) — read a colony's stratified demand from its
/// real committed state and pour the beat's food flow through it. Supply is the
/// total metabolic flow this beat (the syntropic WILD gather plus the CULTIVATED
/// yield — the same two the population eats). The tiers, in priority:
///   * **Metabolic** = `population × per_capita_ration` — the caloric floor.
///   * **Cognitive** = `cognition caste × per_capita_ration` — the leisure the
///     thinkers draw to fund thought (the cognition caste is the mouths that
///     eat perspective, not bread).
///
/// The Homeostatic tier is DEFERRED until containment state lands — a colony
/// with no storage structure feels no containment demand yet, so it is omitted
/// rather than stubbed at a false deficit. The surplus surviving both tiers is
/// the LEISURE the beat banks (`settle_leisure`); the `binding` tier names the
/// crisis: Metabolic is STARVATION, Cognitive is STAGNATION (fed, but the Mind
/// is short of the leisure to think).
pub fn cascade_reading(world: &cfe::world::World, orb: u64) -> Cascade {
    let per_capita = world.per_capita_ration();
    let pop = ratio(world.population_of(orb) as i64, 1);
    // The cognition caste — the line's thinkers, the mouths that eat leisure.
    let cognition = ratio(world.caste_of(orb, cfe::dop::Caste::Cognition) as i64, 1);
    let supply = world.syntropic_flow(orb) + world.cultivated_yield(orb);
    let demands = [
        (Drive::Metabolic, pop * per_capita.clone()),
        (Drive::Cognitive, cognition * per_capita),
    ];
    cascade(supply, &demands)
}

// ── ONT-4: THE LEISURE MANIFOLD ────────────────────────────────────────────
//
// The leisure bank was ONE scalar per orb, stepped each beat by the geometric
// demurrage `bank × (1 − d)` — a stored scalar through a recurrence with a
// multiplication, so its exact-rational denominator accreted the whole causal
// history and detonated (~beat 117, the collapse ONT-2 unmasked once the vein's
// mask was gone; see [[scalar-collapse-derivation-law]]). ONT-4 eradicates it on
// BOTH counts the operator ruled (2026-08-18): (1) NO scalar collapse — the
// geometric recurrence is replaced by FLAT, floored per-beat flows, so every
// stratum stays a bounded `1/grain` integral and no denominator can explode;
// (2) DIMENSIONAL VOLUME PRESERVED — leisure is no longer collapsed to a 1-D
// shadow but banked PER DRIVE-LADDER STRATUM (a manifold over `Cascade.tiers`),
// its volume the sum over strata, so a want's surplus is earmarked by the drive
// level it survived to rather than melted into one fungible number.

/// **THE LEISURE MANIFOLD** (ONT-4) — the cognition budget as a manifold over the
/// drive ladder, not a scalar. Each stratum is one drive tier's standing surplus,
/// a bounded floored-flow integral; the total [`budget`](Self::budget) is their
/// sum (the volume). Held separate so minting a want draws the stratum that want
/// funds, and no tier's balance ever multiplies itself.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LeisureManifold {
    /// Per drive tier (survival tiers never banked), in priority order: the
    /// stratum's standing balance. Absent ⇒ that stratum holds nothing.
    pub strata: Vec<(Drive, Ratio)>,
}

impl LeisureManifold {
    /// **THE VOLUME** — the scalar budget, `Σ` over strata. The dimensional
    /// structure lives in `strata`; this is its projection for a scalar reader
    /// (the COG-13 gate, the mint), so nothing downstream must change to read it.
    pub fn budget(&self) -> Ratio {
        self.strata.iter().fold(ratio(0, 1), |sum, (_, bal)| sum + bal.clone())
    }

    /// One stratum's standing balance (zero if the tier banks nothing).
    pub fn of(&self, drive: Drive) -> Ratio {
        self.strata
            .iter()
            .find(|(d, _)| *d == drive)
            .map(|(_, bal)| bal.clone())
            .unwrap_or_else(|| ratio(0, 1))
    }

    /// Set a stratum's balance, dropping it when empty (so the manifold carries
    /// no zero rows and two equal manifolds compare equal).
    fn set(&mut self, drive: Drive, bal: Ratio) {
        self.strata.retain(|(d, _)| *d != drive);
        if bal > ratio(0, 1) {
            self.strata.push((drive, bal));
        }
    }
}

/// Floor a flow onto `1/grain` — the ONT-2b discipline (floor the FLOW at the
/// source, NEVER the accumulator), so each stratum stays a bounded `1/grain`
/// multiple forever and its denominator cannot explode. `grain` is a bound
/// variable, so `ratio(grain, 1)` mints no tuned dial.
fn floor_flow(x: &Ratio, grain: i64) -> Ratio {
    let g = ratio(grain, 1);
    (x.clone() * g.clone()).floor() / g
}

/// **SETTLE THE LEISURE MANIFOLD** (ONT-4) — fold one beat into the per-stratum
/// banks: each tier's `surplus` IN, its `spend` OUT, and a FLAT `decay` (a
/// bounded per-beat drain, NOT `bank × d`) — every flow floored to `1/grain` and
/// the balance floored at zero. No stratum multiplies its own balance, so the
/// geometric recurrence that detonated is gone by construction and replay
/// recomputes the identical banks. Attribution — how the beat's surplus splits
/// across tiers — is the CALLER's (it reads the live cascade); this is the pure
/// bounded accounting the wire settles against.
pub fn settle_leisure_manifold(
    prior: &LeisureManifold,
    surplus: &[(Drive, Ratio)],
    spend: &[(Drive, Ratio)],
    flat_decay: &Ratio,
    grain: i64,
) -> LeisureManifold {
    let zero = ratio(0, 1);
    let decay = floor_flow(&flat_decay.clone().max(zero.clone()), grain);
    let read = |list: &[(Drive, Ratio)], drive: Drive| {
        list.iter()
            .find(|(d, _)| *d == drive)
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| zero.clone())
    };
    // Every tier named by the prior manifold or this beat's flows.
    let mut tiers: Vec<Drive> = prior.strata.iter().map(|(d, _)| *d).collect();
    for (drive, _) in surplus.iter().chain(spend.iter()) {
        if !tiers.contains(drive) {
            tiers.push(*drive);
        }
    }
    let mut next = LeisureManifold::default();
    for drive in tiers {
        let inflow = floor_flow(&read(surplus, drive), grain);
        let outflow = floor_flow(&read(spend, drive), grain);
        // FLAT decay, not geometric: a bounded per-beat drain the stratum sheds,
        // so a want cannot hoard a lifetime's surplus, without a self-multiply.
        let bal = (prior.of(drive) + inflow - outflow - decay.clone()).max(zero.clone());
        next.set(drive, bal);
    }
    next
}

/// The EROI friction margin μ (> 1): a thought must return MORE than it costs,
/// or the surplus is better banked. A bare integer, not a `ratio` dial — the
/// ratchet does not count it — set by the COG-13 ruling (operator 2026-08-16):
/// the return must at least DOUBLE the cost to clear the investment threshold.
const FRICTION_MARGIN: i64 = 2;

/// **COG-13 — THE COGNITIVE INVESTMENT GATE (EROI)** (operator ruling
/// 2026-08-16). Reflexive spending (`afford == spend`) violates Energy Return
/// on Investment: a mind does not burn accumulated surplus just because the
/// baseline cost is met. It banks, and invests only when the return justifies
/// the expenditure AND a stress-scaled resilience reserve is intact. This
/// replaces the boolean drain with a stratified decision:
///
/// ```text
///   Commit = ( B ≥ W + R(γ) ) ∧ ( γ·T_gen > μ·W  ∨  B ≥ W + R_max )
/// ```
///
/// - `B` the leisure bank, `W` the thought's price (homological cost × the
///   metabolic exchange), both in the same food-surplus units.
/// - **ROI**: the expected return is the deficit the geometry targets (`γ`, the
///   binding tier's unmet demand) carried over a generation `T_gen`
///   ([`World::lifespan_of`]); it must exceed the cost by the friction margin
///   `μ` ([`FRICTION_MARGIN`]). A cheap topology that solves a real deficit
///   clears it; an over-complex one (high `W`) is rejected and the line waits
///   for a cheaper configuration.
/// - **Dynamic reserve** `R(γ) = R_max·(1 − s)`: the buffer collapses with
///   stress `s` (the binding tier's deficit fraction). Sated (`s = 0`) the
///   reserve stands at `R_max` — a generation of the thinkers' own upkeep —
///   so the bank ACCUMULATES; under crisis the floor drops toward zero and the
///   line spends down to fund its escape (below which the desperation burn, in
///   the caller, takes over).
/// - **Curiosity**: a massively overflowing bank (`B ≥ W + R_max`) funds
///   speculative thought even at `γ = 0`, where ROI is undefined.
///
/// Pure and total; `γ` and the tier demands are read from the live cascade, so
/// the gate speaks the same stratified language as the rest of the economy.
pub fn cognitive_investment(world: &cfe::world::World, orb: u64, price: &Ratio) -> bool {
    let zero = ratio(0, 1);
    let bank = world.leisure_bank_of(orb);
    // The hard floor — a thought cannot be paid from an empty bank.
    if bank < *price {
        return false;
    }
    let cascade = cascade_reading(world, orb);
    let t_gen = ratio(world.lifespan_of() as i64, 1);
    // R_max — the sated reserve: a GENERATION of the cognition caste's own
    // upkeep, so a prosperous line banks deeply before indulging curiosity.
    let cognitive_demand = cascade
        .tiers
        .iter()
        .find(|tier| matches!(tier.drive, Drive::Cognitive))
        .map(|tier| tier.demand.clone())
        .unwrap_or_else(|| zero.clone());
    let reserve_max = cognitive_demand * t_gen.clone();
    // γ and the stress fraction from the BINDING tier (zero when sated).
    let (gamma, stress) = match cascade
        .binding
        .and_then(|drive| cascade.tiers.iter().find(|tier| tier.drive == drive))
    {
        Some(tier) if tier.demand > zero => {
            (tier.deficit.clone(), tier.deficit.clone() / tier.demand.clone())
        }
        _ => (zero.clone(), zero.clone()),
    };
    // Systemic resilience: the reserve collapses proportionally to stress.
    let reserve = reserve_max.clone() * (ratio(1, 1) - stress);
    if bank < price.clone() + reserve {
        return false;
    }
    // Thermodynamic viability (ROI) OR the curiosity overflow.
    let roi_ok = gamma * t_gen > ratio(FRICTION_MARGIN, 1) * price.clone();
    let curiosity = bank >= price.clone() + reserve_max;
    roi_ok || curiosity
}

/// **THE DEFECT CLASSIFICATION** (COG-12) — the void's homological SHAPE tells
/// the Mind what KIND of tool is missing, so the leap into an irreducible void
/// is not blind. When γ can be inverted against known structure there is no
/// search — Production builds the answer; a search happens ONLY for an
/// irreducible residual (a Sullivan extension, a genuinely new dimension), and
/// even then the exact SNF split the peel already carries dictates the class:
///
///   * a void dominated by **free residual** (an unspanned new direction) needs
///     a tool that MAKES structure that was not there — the STRUCTURE families
///     (Tile, Object);
///   * a void dominated by **torsion** (a twist onto existing structure) needs a
///     tool that RELATES what is — the CORRESPONDENCE families (Symmetry, Recolor).
///
/// This reuses the engine's OWN family semantics (`need.rs`: Novelty→structure,
/// Resonance→correspondence), so the economic void and the solver's hungers
/// speak one language. Returns the indicated families FIRST — ordering only, the
/// observation still supplies every candidate; empty when γ = 0 (no defect).
///
/// EXACT is the free-vs-torsion character, read from the SNF. The one bridge is
/// WHICH families are "structure" vs "correspondence" — the engine's declared
/// convention, not a derived identity (the economic ℤ-module and the grid-op
/// families are distinct spaces).
pub fn classify_defect(residual: &[BigInt], torsion: &[BigInt]) -> Vec<solver::need::Family> {
    use solver::need::Family;
    let free: BigInt = residual.iter().map(|x| x.abs()).sum();
    let twist: BigInt = torsion.iter().map(|d| BigInt::from(d.bits())).sum();
    if free.is_zero() && twist.is_zero() {
        return Vec::new(); // covered — no void to classify, no search
    }
    let structure = [Family::Tile, Family::Object]; // MAKE what was not there
    let correspondence = [Family::Symmetry, Family::Recolor]; // RELATE what is
    // The dominant character leads; the other trails (still reachable).
    if free >= twist {
        structure.iter().chain(correspondence.iter()).copied().collect()
    } else {
        correspondence.iter().chain(structure.iter()).copied().collect()
    }
}

/// **THE STRESS INDEX σ** (COG-7) — a colony's thermodynamic stress in `[0, 1]`,
/// the blend coefficient between instinct (exploit) and curiosity (explore). It
/// reads the live cascade: a line whose METABOLIC floor is unmet is starving and
/// stressed to the maximum (σ = 1 — pure instinct, no energy for research);
/// otherwise σ is the unmet fraction of its COGNITIVE tier — a line whose
/// thinkers are fully fed reads σ = 0 (sated, free to explore), one whose leisure
/// only partly covers them reads a genuine blend. Exact-rational; no new dial.
pub fn stress(world: &cfe::world::World, orb: u64) -> Ratio {
    let c = cascade_reading(world, orb);
    if c.binding == Some(Drive::Metabolic) {
        return ratio(1, 1); // starving — no metabolic luxury for curiosity
    }
    for tier in &c.tiers {
        if tier.drive == Drive::Cognitive && tier.demand > ratio(0, 1) {
            return (tier.deficit.clone() / tier.demand.clone()).min(ratio(1, 1));
        }
    }
    ratio(0, 1) // sated
}

/// σ quantised onto a 64-lattice as `(numerator, 64)`, for the exact-integer
/// blend — the same melt grid the rest of the economy speaks. A caller with no
/// world clock hands this straight to [`instinct::blend`].
pub fn stress_quantized(world: &cfe::world::World, orb: u64) -> (u64, u64) {
    // Quantise onto the existing melt lattice (no new dial — the same grid the
    // market and the peel already speak).
    let scaled = (stress(world, orb) * ratio(LATTICE, 1)).round().to_integer();
    let num = scaled.to_u64().unwrap_or(0).min(LATTICE as u64);
    (num, LATTICE as u64)
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

    fn r(n: i64, d: i64) -> Ratio {
        ratio(n, d)
    }

    #[test]
    fn the_cascade_separates_starvation_from_stagnation() {
        use Drive::*;
        // STARVATION — supply below the metabolic floor. The waterfall halts at
        // tier 1: the binding constraint is Metabolic, no surplus survives, and
        // the higher tiers get nothing.
        let starving = cascade(
            r(3, 1),
            &[(Metabolic, r(10, 1)), (Homeostatic, r(5, 1)), (Cognitive, r(8, 1))],
        );
        assert_eq!(starving.binding, Some(Metabolic), "an unmet floor is starvation");
        assert_eq!(starving.tiers[0].met, r(3, 1));
        assert_eq!(starving.tiers[0].deficit, r(7, 1), "the caloric shortfall");
        assert_eq!(starving.tiers[2].met, r(0, 1), "nothing reaches cognition");
        assert_eq!(starving.leisure, r(0, 1), "a starving line banks no leisure");

        // STAGNATION — fed past the floor, but the surplus cannot cover the
        // cognitive demand. The binding constraint is a HIGHER tier (Cognitive),
        // NOT Metabolic: the line is fed yet its Mind is starved of leisure. This
        // is the Sated crisis the flat model could never express.
        let stagnant = cascade(
            r(25, 1),
            &[(Metabolic, r(10, 1)), (Homeostatic, r(5, 1)), (Cognitive, r(30, 1))],
        );
        assert_eq!(stagnant.tiers[0].deficit, r(0, 1), "the floor IS met — not starving");
        assert_eq!(stagnant.tiers[1].deficit, r(0, 1), "containment met too");
        assert_eq!(stagnant.binding, Some(Cognitive), "the binding γ is a cognitive need, not food");
        assert_eq!(stagnant.tiers[2].met, r(10, 1), "10 of the 30 cognitive demand covered");
        assert_eq!(stagnant.leisure, r(0, 1), "no surplus survives an unmet cognitive tier");
    }

    #[test]
    fn abundance_survives_the_lowest_tier_as_leisure() {
        use Drive::*;
        // Every tier met, with room to spare: the surplus that survives the
        // whole waterfall is the leisure the Mind banks. 100 − (10+20+30) = 40.
        let rich = cascade(
            r(100, 1),
            &[(Metabolic, r(10, 1)), (Homeostatic, r(20, 1)), (Cognitive, r(30, 1))],
        );
        assert_eq!(rich.binding, None, "no drive left wanting");
        assert!(rich.tiers.iter().all(|t| t.deficit == r(0, 1)), "all tiers met");
        assert_eq!(rich.leisure, r(40, 1), "the surplus above every drive is leisure");
    }

    #[test]
    fn priority_is_strict_a_lower_tier_binds_first() {
        use Drive::*;
        // Both homeostatic and cognitive go unmet, but the FIRST unmet tier is
        // the binding one — priority is strict, not a max over deficits.
        let c = cascade(r(12, 1), &[(Metabolic, r(10, 1)), (Homeostatic, r(9, 1)), (Cognitive, r(9, 1))]);
        assert_eq!(c.binding, Some(Homeostatic), "the lower unmet tier binds, not the larger deficit");
        assert_eq!(c.tiers[1].met, r(2, 1), "homeostatic takes the 2 surplus after the floor");
        assert_eq!(c.tiers[2].met, r(0, 1), "nothing spills past the unmet homeostatic tier");
        // A negative demand is simply a drive not felt — it passes surplus on.
        let unfelt = cascade(r(10, 1), &[(Metabolic, r(4, 1)), (Cognitive, r(-3, 1))]);
        assert_eq!(unfelt.binding, None, "a drive not felt is never a deficit");
        assert_eq!(unfelt.leisure, r(6, 1), "the unfelt tier passes its surplus straight through");
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
    fn fasting_breaks_the_deadlock_the_poor_still_think_slowly() {
        // The rich: large leisure dwarfs the trickle — they coin instantly.
        let rich = fasting_fill(r(500, 1), r(80, 1));
        assert!(rich > r(500, 1), "leisure plus a trickle exceeds leisure alone");
        // The destitute: ZERO leisure, but a live syntropic flow still feeds a
        // base cognitive trickle — the accumulator inches up, never stuck at 0.
        let poor = fasting_fill(r(0, 1), r(80, 1));
        assert!(poor > r(0, 1), "a zero-leisure line still fills from the wild trickle: {poor}");
        // The truly barren: no wild flow either (unsited/dead world) — no trickle,
        // no thought. The floor is survival, not free cognition.
        assert_eq!(fasting_fill(r(0, 1), r(0, 1)), r(0, 1), "no wild flow, no trickle");
        // The trickle is a small fraction of the wild flow — slow, not free.
        assert!(poor < r(80, 1), "the trickle is a fraction of the flow, not all of it");
    }

    #[test]
    fn the_desperation_burn_is_dear_paid_in_people() {
        // A word that costs 3 leisure costs 3×inefficiency HEADS to burn — the
        // crisis exchange is deliberately worse than thinking on a full stomach.
        let leisure_price = r(3, 1);
        let heads = desperation_price(&leisure_price);
        assert!(heads > leisure_price, "burning people is dearer than spending leisure: {heads}");
        // A costlier thought burns proportionally more of the colony.
        assert!(desperation_price(&r(6, 1)) > desperation_price(&r(3, 1)), "a deeper Eureka costs more lives");
    }

    #[test]
    fn the_void_shape_classifies_the_missing_tool() {
        use solver::need::Family;
        // A FREE-dominated void (unspanned direction, no twist) → the Mind must
        // MAKE structure: the structure families lead.
        let structure = classify_defect(&[b(0), b(0), b(9)], &[]);
        assert_eq!(structure.first(), Some(&Family::Tile), "an unspanned void leads with structure");
        assert!(structure.contains(&Family::Symmetry), "correspondence still reachable (ordering only)");
        // A TORSION-dominated void (tiny free part, heavy twist) → the Mind must
        // RELATE existing structure: the correspondence families lead.
        let twist = classify_defect(&[b(1)], &[b(255), b(255)]);
        assert_eq!(twist.first(), Some(&Family::Symmetry), "a twisted void leads with correspondence");
        // A COVERED deficit is no void — nothing to classify, no search at all.
        assert!(classify_defect(&[b(0), b(0)], &[]).is_empty(), "covered → no search");
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
