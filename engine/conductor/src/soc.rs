//! **SOC — THE SPECIES MANIFOLD & THE OPEN WORKBENCH.**
//!
//! SOC-1 opened `build_step`'s surplus phase into a stochastic draw over an
//! option surface (was a fixed `industrial.or(refine).or(settlement)`
//! priority). SOC-2 (operator directive 2026-08-16, *the algebra of affinity*)
//! replaces the scalar `match disposition` weight — a dimensional collapse — with
//! a genuine **homological pairing** against the species' own metabolic manifold.
//!
//! **The manifold `M_civ`.** A species' nature is not a tensor of scalars (a
//! flat vector space cannot describe a *hole* — a metabolism a clade cannot run).
//! It is a chain complex kept graded, never flattened. Here `M_civ` is the
//! species' **metabolic graph**: `C₀` = the resource traits the clade can touch
//! (an environment source `⊥` plus every resource reachable from it), `C₁` = the
//! conversion pathways its metabolism can run (`industry_relevant`, input→output
//! from `cost`/`zone_yield`). Its boundary `∂₁` is exact-integer, and its
//! homology is read off the substrate's Smith Normal Form — `Ker(∂₁)`, `Im(∂₁)`,
//! and the ranks — never a scalar comparison.
//!
//! **The mapping operator `𝒯`.** An infrastructure project `s` requires a
//! pathway from an input trait to an output trait — a class `[ω]` in `H₀(M_civ)`,
//! namely `[1_out − 1_in]`. The affinity is the operator's step function on the
//! vanishing of that obstruction:
//!
//! ```text
//!   w_s = 𝒯([ω]) = { W_base · genus(M_civ)   if [ω] = 0  (native bounding)
//!                   { 0                        if [ω] ≠ 0  (a hole in the phase space)
//! ```
//!
//! `[ω] = 0` iff `1_out − 1_in ∈ Im(∂₁)` — input and output are both present and
//! metabolically **connected**: the society's own geometry bounds the
//! requirement. `[ω] ≠ 0` iff a trait is absent or unreachable (a plasmic line
//! reaching for the iron forge: `Alloy` is not in its complex, the pairing
//! returns a non-trivial harmonic representative, `w_s → 0`) — the building falls
//! out of the partition `Z` with **no boolean override and no species table**.
//!
//! **Intelligence is genus.** `genus = Σ βₖ` (`β₀` components + `β₁` independent
//! metabolic cycles). A recycling (regenerative) metabolism closes loops — its
//! `β₁` rises — so a topologically richer society scores its native
//! infrastructure higher, the geometric truth that a system cannot deploy a
//! structure more complex than its own topological degree. `genus` is common to
//! every native candidate in one beat, so it cancels in the draw `P(s)=w_s/Z`
//! (natives are equiprobable, non-natives excluded) and surfaces where it
//! belongs: the reach/depth coupling (`bridge::read_entangled`).
//!
//! **Deterministic niche construction.** The draw among the native set is seeded
//! by the line's `chronicle_digest`, domain-tagged `b"workbench"` to decorrelate
//! from the Mind's `b"instinct"` draw. Replay-exact, `ℤ/ℚ` throughout.

use crate::civic::{Drive, LATTICE};
use cfe::dop::{metabolic_harvest, Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use num_bigint::BigInt;
use substrate::snf::{smith_normal_form, Smith};

/// **THE CIVILIZATION READS ITSELF (COG-DIRECTION Slice 1).** The same reduce organ
/// that solves an ARC grid (`reduct::peel`) is turned on the civilization's OWN
/// material self — not a hardcoded `if biomass then food`. Its element holdings are
/// projected into the FIVE almanac families (the ONT lens, past iron+biomass:
/// `element_properties(..).0` is the family 1..5) and laid on a 1×n frame; `peel`
/// covers them into family-bodies. That structural reading is combined with the
/// thermodynamic books — `cohesion_ratio(syntropy, entropy) ∈ [0,1]`, the fraction
/// of a line's total activity that is ORDER CAPTURED rather than SHED — to DERIVE a
/// direction: a line that captures more order than it vents (`cohesion > ½`) AND
/// sustains material diversity (≥2 families present — a mono-family field is a
/// collapse, a hole the reduce organ sees) is ORDERING; else it is DISSIPATING.
/// Meaning emerges from the syntropic↔entropic combination, read through the organ,
/// never a scalar threshold on a legacy resource. Slice 1 EMITS this live to witness
/// on the wire; Slice 2 lets it drive the trajectory and retires the FSM.
#[derive(Debug, Clone)]
pub struct DirectionRead {
    pub elements: usize,
    /// Distinct almanac families present (fiber coverage) — material diversity.
    pub families: usize,
    /// Connected family-bodies in the peel cover — the material's structure.
    pub bodies: usize,
    /// syntropy / (syntropy + entropy) ∈ [0,1]; None when the books are cold.
    pub cohesion: Option<Ratio>,
    /// Derived: is the line composing its material self (ordering) or dissipating?
    pub ordering: bool,
}

impl DirectionRead {
    pub fn verdict(&self) -> &'static str {
        match self.ordering {
            true => "ordering",
            false => "dissipating",
        }
    }
}

/// Turn the reduce organ on the civilization itself. `holdings` is the orb's element
/// ledger (id-ascending), `class` its star (for the family projection), and the two
/// books its captured order and vented entropy. `None` when the line holds no
/// elements (nothing material to read yet).
/// The civilization's own material self, laid on a 1×n frame: each held
/// element is a site, its gauge direction its almanac FAMILY (1..5 -> 0..4,
/// the ONT projection past the iron coordinate). Shared by `observe_direction`
/// (the peel reads this occupancy) and the per-beat gauge sample (TU-1 / G:
/// `MANIFOLD-1a` stamps this SAME occupancy on the beat's receipts, raw — the
/// reduce organ's actual reading, not a re-derived summary).
pub fn gauge_occupancy(
    holdings: &[([u8; 32], Ratio)],
    class: cfe::thermal::StarClass,
) -> Vec<(reduct::peel::Site, reduct::peel::GaugeDirection)> {
    use reduct::peel::{GaugeDirection, Site};
    holdings
        .iter()
        .enumerate()
        .map(|(i, (id, _amount))| {
            let (family, _tier, _richness) = cfe::dop::element_properties(id, class);
            (
                Site { row: 0, col: i as i64, layer: 0 },
                GaugeDirection { index: family.saturating_sub(1) },
            )
        })
        .collect()
}

pub fn observe_direction(
    holdings: &[([u8; 32], Ratio)],
    class: cfe::thermal::StarClass,
    syntropy: &Ratio,
    entropy: &Ratio,
) -> Option<DirectionRead> {
    use reduct::peel::{peel, CellFrame, FullMeasurement};
    if holdings.is_empty() {
        return None;
    }
    let cols = holdings.len();
    // A 1×n frame is whole by construction (each cell carried exactly once),
    // so `try_new` admits it.
    let occupancy = gauge_occupancy(holdings, class);
    let measurement =
        FullMeasurement::try_new(5, CellFrame { rows: 1, cols: cols as u32, layers: 1 }, occupancy).ok()?;
    let cover = peel(&measurement);
    let families = cover.value.supports.len();
    let bodies = cover.object.bodies.len();
    let cohesion = cfe::dop::cohesion_ratio(syntropy, entropy);
    let ordering = cohesion
        .as_ref()
        .map(|c| *c > ratio(1, 2))
        .unwrap_or(false)
        && families >= 2;
    Some(DirectionRead { elements: cols, families, bodies, cohesion, ordering })
}

/// The base affinity a natively-bounded building carries before the genus scale.
/// A bare integer (no `ratio` dial — the ratchet is unmoved); the draw needs only
/// the proportions, and this common base cancels in `P(s)=w_s/Z`.
const W_BASE: u64 = 1;

/// The universe of metabolic conversions, each a directed pathway `input → output`
/// (`None` input = drawn from the environment source `⊥`). Assembled from the
/// standing tables — `ZoneKind::cost` names the input, `yield_per_tick` the
/// output — so a change to the economy's chemistry flows here with no restatement.
/// `Habitat`/`Regenerator` restore the metabolism's RATION (species-relative), so
/// they are resolved against `metabolic_harvest`, not listed here.
fn conversions() -> [(ZoneKind, Option<Resource>, Resource); 6] {
    [
        (ZoneKind::Extractor, None, Resource::Iron),
        (ZoneKind::Smelter, Some(Resource::Iron), Resource::Alloy),
        (ZoneKind::Synthesizer, Some(Resource::Alloy), Resource::Diamond),
        (ZoneKind::Resonator, Some(Resource::Alloy), Resource::Crystal),
        (ZoneKind::Encasement, Some(Resource::Iron), Resource::Joule),
        (ZoneKind::CryoArray, Some(Resource::Iron), Resource::Sink),
    ]
}

/// **THE METABOLIC TRAIT SEED** — the resources a clade draws from its
/// environment BEFORE any conversion: its constitutive nature, not a universal
/// baseline. This reads the primary-resource axis the science law already tables
/// (`science.rs`: mineral is pressure/metal work → Iron, biotic is life →
/// Biomass, plasmic rides resonance/energy → Joule, never Iron). Biotic is the
/// generalist that also mines iron for tools; a plasmic energy-being has no iron
/// in its world at all — the operator's law, *why would a plasmic want iron?*
/// The ration (`metabolic_harvest`) always lies in this set.
///
/// This is the SEED of the reachability, so it decides which pathways can exist:
/// a trait absent here, and unreachable by any relevant conversion, is a genuine
/// hole in the species' phase space — the homological exclusion, not a table of
/// forbidden buildings.
fn metabolic_traits(metabolism: u8) -> Vec<Resource> {
    match metabolism {
        1 => vec![Resource::Iron],                    // mineral — metal & pressure
        3 => vec![Resource::Joule],                   // plasmic — energy, no iron
        _ => vec![Resource::Biomass, Resource::Iron], // biotic — life, mines tools
    }
}

/// The pathway a buildable REQUIRES: `(input trait, output trait)`. A source
/// building draws its output from the environment (`None` input). Buildings with
/// no metabolic conversion (civic/refinery infrastructure) return `None` — they
/// carry no thermodynamic obstruction and are natively bounded by definition.
fn requirement(kind: ZoneKind, ration: Resource) -> Option<(Option<Resource>, Resource)> {
    match kind {
        ZoneKind::Extractor => Some((None, Resource::Iron)),
        ZoneKind::Habitat => Some((None, ration)),
        ZoneKind::Regenerator => Some((None, ration)),
        ZoneKind::Smelter => Some((Some(Resource::Iron), Resource::Alloy)),
        ZoneKind::Synthesizer => Some((Some(Resource::Alloy), Resource::Diamond)),
        ZoneKind::Resonator => Some((Some(Resource::Alloy), Resource::Crystal)),
        ZoneKind::Encasement => Some((Some(Resource::Iron), Resource::Joule)),
        ZoneKind::CryoArray => Some((Some(Resource::Iron), Resource::Sink)),
        _ => None, // civic infrastructure — no metabolic pathway to obstruct
    }
}

/// **THE ALGEBRAIC RANK** (operator ruling 2026-08-16 — SNF as a presentation,
/// not a scalar) — the society's intelligence read as the invariant PAIR the
/// Smith Normal Form of `∂₁` yields, because a chain complex of free ℤ-modules
/// IS a free resolution (a generators-and-relations presentation):
///
/// - **Free generators** — the algebraic DIMENSION (a Krull-dimension analog):
///   `components` (`β₀`, independent capabilities reachable from the seed) plus
///   `cycles` (`β₁`, independent metabolic loops — regeneration is a CYCLE, a
///   syzygy, never a new variable, so dimension alone would miss it).
/// - **Relations** — the torsion coefficients `dᵢ > 1` (the `ℤ/dᵢ` twists /
///   syzygies). Empty while `M_civ` is a graph (graph homology is torsion-free);
///   non-empty once higher cells enter. The nonlinear `ℚ[x]/I` / Gröbner frontier
///   (RCP-2, DIO-3b) is DEFERRED — and, when built, carries modular truncation
///   from line one; the linear core stays SNF, bounded and cheap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgebraicRank {
    /// `β₀` — free rank of `H₀`: independent capability components.
    pub components: u64,
    /// `β₁` — free rank of `H₁`: independent metabolic cycles (recycling loops).
    pub cycles: u64,
    /// The torsion coefficients `dᵢ > 1` — the relations/syzygies. Torsion-free
    /// (empty) for a metabolic graph; the seam higher relations enter through.
    pub relations: Vec<BigInt>,
}

impl AlgebraicRank {
    /// The free rank — the algebraic DIMENSION, `components + cycles`. This is the
    /// scalar the draw summarises as `genus`; the pair loses nothing, it makes the
    /// split (and the relation structure) explicit.
    pub fn dimension(&self) -> u64 {
        self.components + self.cycles
    }
}

/// **`M_civ` — THE SPECIES METABOLIC COMPLEX.** Its vertices (`⊥` + the reachable
/// resource traits), its exact boundary `∂₁` factored to Smith Normal Form once,
/// and its genus. Every affinity query pairs against this without refactoring.
pub struct SpeciesManifold {
    /// Vertex 0 is the environment source `⊥`; the rest are the reachable traits.
    traits: Vec<Resource>,
    /// The SNF of `∂₁` (rows = |V|, cols = |E|) — `Im`, `Ker`, and rank in one.
    boundary: Smith,
    /// `genus = Σ βₖ = β₀ + β₁` — the society's topological degree.
    pub genus: u64,
    ration: Resource,
}

impl SpeciesManifold {
    /// Assemble the complex for a species. `metabolism` (u8) sets the ration
    /// trait; `metabolism_name` gates which conversions are relevant (a plasmic
    /// line never forges); `recycles` (a regenerative economy) adds the return
    /// pathway that closes a metabolic loop and raises `β₁`.
    pub fn of(metabolism: u8, metabolism_name: &str, recycles: bool) -> Self {
        let ration = metabolic_harvest(metabolism).0;

        // Reachability from ⊥: the traits the clade can actually touch. The
        // metabolic trait seed (its constitutive nature — mineral iron, biotic
        // life, plasmic energy) plus the ration seed it; relevant conversions
        // extend it to a fixpoint. A conversion whose input is unreachable, or
        // that its metabolism cannot run, never contributes its output — that is
        // how a hole (a missing trait) forms, with no exclusion table. A plasmic
        // line never seeds Iron, so the whole iron forge is unreachable from it.
        // THE FRONTIER PEEL (not a sweep). The relevant conversions form a DAG
        // rooted at the seed traits; walk it ONCE with a worklist, firing each
        // conversion exactly when its input trait arrives. This replaces a
        // fixpoint that re-scanned every conversion each round until nothing grew
        // (O(rounds·E·V), the cartesian find-scan the standing ruling forbids):
        // now every trait is dequeued once and every relevant conversion fired
        // once. Source conversions (⊥ input) add nothing to reachability — the
        // environment hands a plasmic line no iron; their output must already be
        // a native seed.
        let relevant: Vec<(Option<Resource>, Resource)> = conversions()
            .into_iter()
            .filter(|(kind, _, _)| crate::industry_relevant(metabolism_name, *kind))
            .map(|(_, input, output)| (input, output))
            .collect();

        let mut reachable: Vec<Resource> = metabolic_traits(metabolism);
        if !reachable.contains(&ration) {
            reachable.push(ration);
        }
        let mut frontier: std::collections::VecDeque<Resource> =
            reachable.iter().copied().collect();
        while let Some(arrived) = frontier.pop_front() {
            for (input, output) in &relevant {
                if *input == Some(arrived) && !reachable.contains(output) {
                    reachable.push(*output);
                    frontier.push_back(*output);
                }
            }
        }
        reachable.sort_by_key(|r| *r as u8);

        // The edges — ONE projection over the same relevant set (each conversion
        // whose endpoints are both reachable is an edge), plus the ration source
        // edge (Habitat) and the recycling return edge (ration → ⊥).
        let idx = |r: Resource, traits: &[Resource]| -> usize {
            traits.iter().position(|t| *t == r).map(|p| p + 1).unwrap_or(0)
        };
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for (input, output) in &relevant {
            let input_reachable = match input {
                None => true,
                Some(r) => reachable.contains(r),
            };
            if input_reachable && reachable.contains(output) {
                let from = input.map(|r| idx(r, &reachable)).unwrap_or(0);
                edges.push((from, idx(*output, &reachable)));
            }
        }
        edges.push((0, idx(ration, &reachable))); // the habitat source
        if recycles {
            edges.push((idx(ration, &reachable), 0)); // the loop the recycler closes
        }

        // ∂₁ : C₁ → C₀, an exact integer matrix (rows = |V|, cols = |E|); column
        // e carries −1 at its tail and +1 at its head. SNF gives Im/Ker/rank.
        let n_vertices = reachable.len() + 1;
        let mut boundary_matrix: Vec<Vec<BigInt>> =
            vec![vec![BigInt::from(0); edges.len()]; n_vertices];
        for (e, (from, to)) in edges.iter().enumerate() {
            boundary_matrix[*from][e] += BigInt::from(-1);
            boundary_matrix[*to][e] += BigInt::from(1);
        }
        let boundary = smith_normal_form(&boundary_matrix);
        let rank = boundary.rank();
        // A graph carries no homological torsion: β₀ = |V| − rank, β₁ = |E| − rank.
        let beta0 = (n_vertices - rank) as u64;
        let beta1 = (edges.len() - rank) as u64;
        let genus = beta0 + beta1;

        SpeciesManifold { traits: reachable, boundary, genus, ration }
    }

    /// The vertex index of a trait (`None` = the environment source `⊥` at 0),
    /// or `None` if the trait is absent from the complex (unreachable — a hole).
    fn vertex(&self, trait_: Option<Resource>) -> Option<usize> {
        match trait_ {
            None => Some(0),
            Some(r) => self.traits.iter().position(|t| *t == r).map(|p| p + 1),
        }
    }

    /// **THE HOMOLOGICAL PAIRING.** `[ω] = [1_out − 1_in] = 0` in `H₀` iff the
    /// class lies in `Im(∂₁)` — input and output present and connected. Exact
    /// membership via the SNF witnesses: in the `U` basis, `b ∈ Im(∂₁)` iff each
    /// coordinate below the rank vanishes and each on-rank coordinate is divisible
    /// by its invariant factor (all `1` for a torsion-free graph, so the test is
    /// exactly connectivity — computed, not assumed).
    pub fn bounds(&self, kind: ZoneKind) -> bool {
        let Some((input, output)) = requirement(kind, self.ration) else {
            return true; // civic infrastructure carries no metabolic obstruction
        };
        let (Some(in_v), Some(out_v)) = (self.vertex(input), self.vertex(Some(output))) else {
            return false; // a trait is absent — the pathway cannot even be expressed
        };
        if in_v == out_v {
            return true; // the trait bounds itself (a source into its own store)
        }
        let rows = self.boundary.rows;
        let mut b = vec![BigInt::from(0); rows];
        b[out_v] += BigInt::from(1);
        b[in_v] += BigInt::from(-1);
        let ub = matvec(&self.boundary.u, &b);
        let factors = self.boundary.invariant_factors();
        for (i, coord) in ub.iter().enumerate() {
            match factors.get(i) {
                Some(d) => {
                    if (coord % d) != BigInt::from(0) {
                        return false;
                    }
                }
                None => {
                    if *coord != BigInt::from(0) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// **THE MAPPING OPERATOR `𝒯`.** `W_base · genus` for a natively-bounded
    /// building, `0` for an obstructed one. The whole SOC-1 scalar `match
    /// disposition` collapses to this pairing.
    pub fn affinity(&self, kind: ZoneKind) -> u64 {
        if self.bounds(kind) {
            W_BASE.saturating_mul(self.genus)
        } else {
            0
        }
    }

    /// **THE ALGEBRAIC RANK** — read the intelligence as the SNF invariant PAIR
    /// (generators ⊕ relations) rather than the collapsed `genus` scalar. Free
    /// ranks straight off the factored boundary: `β₀ = rows − rank`,
    /// `β₁ = cols − rank`; torsion = the invariant factors `> 1`. Exact, and no
    /// new computation — `∂₁` was already factored in `of`. `dimension()` recovers
    /// `genus`, so the draw is untouched; this only makes the presentation legible.
    pub fn algebraic_rank(&self) -> AlgebraicRank {
        let rank = self.boundary.rank();
        AlgebraicRank {
            components: (self.boundary.rows - rank) as u64,
            cycles: (self.boundary.cols - rank) as u64,
            relations: self
                .boundary
                .invariant_factors()
                .into_iter()
                .filter(|d| *d > BigInt::from(1))
                .collect(),
        }
    }
}

/// Exact integer `A · x`.
fn matvec(a: &[Vec<BigInt>], x: &[BigInt]) -> Vec<BigInt> {
    a.iter()
        .map(|row| {
            row.iter()
                .zip(x)
                .fold(BigInt::from(0), |acc, (aij, xj)| acc + aij * xj)
        })
        .collect()
}

/// **THE WORKBENCH CHOOSER** — the civic partition-function draw. Over the
/// candidate set `S_valid` (viability resolved upstream by `build_step`'s gates),
/// weight each by the manifold's `𝒯` affinity and sample `P(s)=w_s/Z` from the
/// line's `chronicle_digest`. A non-native building weighs `0` — excluded from
/// `Z` with no boolean override. `None` only when no candidate is natively
/// bounded (the beat builds nothing, exactly as the old priority returned `None`).
/// **THE DEFICIT-CLOSER** (SOC-3) — does building `kind` relieve the `binding`
/// drive's deficit? A society under METABOLIC or COGNITIVE pressure is starved of
/// SUPPLY — the ration flow that feeds the whole cascade — so the settlement/food
/// buildings close it; a HOMEOSTATIC deficit (entropy shed faster than contained)
/// closes with sinks; an EXPANSIVE one with the industry that reaches new niches.
/// (The live cascade surfaces Metabolic/Cognitive as binding today; the full map
/// is future-proof for when the higher tiers are wired.)
fn serves(kind: ZoneKind, binding: Drive) -> bool {
    use ZoneKind::*;
    match binding {
        Drive::Metabolic | Drive::Cognitive => {
            matches!(kind, Extractor | Habitat | Regenerator | Granary)
        }
        Drive::Homeostatic => matches!(kind, Encasement | CryoArray),
        Drive::Expansive => matches!(kind, Smelter | Synthesizer | ShipYard),
    }
}

/// **THE WORKBENCH CHOOSER, NEED-TILTED** (SOC-3). Over the native surface, the
/// homological `affinity` weight is TILTED toward the building that closes the
/// society's BINDING drive, scaled by how starved that drive is:
///
/// ```text
///   w_s = affinity(s) · ( L + γ_num · [s closes the binding drive] )
/// ```
///
/// `L` is the melt lattice; `γ_num ∈ [0, L]` is the binding drive's unmet fraction
/// (`deficit / demand`) quantised onto it. A society that STARVES (`γ_num → L`)
/// leans hard toward the closer (up to twice as likely); a society at ease
/// (`binding = None`, `γ_num = 0`) reads `w_s = affinity·L` for every native — the
/// uniform SOC-2 draw, so the society EXPLORES when comfortable and BUILDS WHAT IT
/// NEEDS when pressured. The shared `L` and `affinity` (genus) cancel in the draw;
/// a non-native still weighs `0`. Exact-integer, digest-seeded, replay-exact.
/// **A ZONE'S WORKBENCH WEIGHT** — the homological `affinity` (genus) of building
/// `kind` for this species, tilted toward the binding drive when the building
/// closes it: `affinity · (L + γ·[serves])`. A non-native weighs `0`.
pub fn zone_weight(
    kind: ZoneKind,
    manifold: &SpeciesManifold,
    binding: Option<Drive>,
    gamma_num: u64,
) -> u64 {
    let lattice = LATTICE as u64;
    let tilt = match binding {
        Some(drive) if serves(kind, drive) => gamma_num,
        _ => 0,
    };
    manifold
        .affinity(kind)
        .saturating_mul(lattice.saturating_add(tilt))
}

/// **A ROLE CLOSES A DRIVE** (RCP-2 F4) — the recipe analogue of `serves`: does a
/// recipe delivering `role` relieve the `binding` drive? A supply-starved line
/// (Metabolic/Cognitive) is closed by FOOD or COGNITION; a Homeostatic one by a
/// SINK; an Expansive one by the ENERGY/STRUCT that reaches new niches. Mirrors
/// the zone `serves` map so a recipe and a building compete on equal footing.
fn role_serves(role: usize, binding: Drive) -> bool {
    use cfe::morphogen::{COGNITION, ENERGY, FOOD, SINK, STRUCT};
    match binding {
        Drive::Metabolic | Drive::Cognitive => role == FOOD || role == COGNITION,
        Drive::Homeostatic => role == SINK,
        Drive::Expansive => role == ENERGY || role == STRUCT,
    }
}

/// **A RECIPE'S WORKBENCH WEIGHT** (RCP-2 F4) — a minted recipe competes in the
/// SAME partition as the zones, weighted by the manifold's topological REACH
/// (`Σβ_k`, the algebraic dimension — the recipe's analogue of a zone's genus
/// affinity) tilted toward the binding drive when the recipe's role closes it:
/// `reach · (L + γ·[role serves])`. A society of greater reach raises richer
/// recipes more readily, exactly as a stronger affinity raises a native building.
pub fn recipe_weight(
    role: usize,
    rank: &AlgebraicRank,
    binding: Option<Drive>,
    gamma_num: u64,
) -> u64 {
    let lattice = LATTICE as u64;
    let tilt = match binding {
        Some(drive) if role_serves(role, drive) => gamma_num,
        _ => 0,
    };
    (rank.dimension() as u64).saturating_mul(lattice.saturating_add(tilt))
}

/// **THE PARTITION DRAW** — the shared engine under the workbench: sample an index
/// `P(i) = w_i/Z` from a weight vector against a caller-salted `draw_seed`. `None`
/// when every weight is `0` (nothing native — the beat builds nothing). The seed
/// is the FINAL draw seed; each caller salts its own so independent draws off the
/// same chronicle digest do not correlate. Exact-integer, replay-exact.
pub fn partition_draw(weights: &[u64], draw_seed: [u8; 32]) -> Option<usize> {
    let total: u64 = weights.iter().fold(0u64, |sum, w| sum.saturating_add(*w));
    if total == 0 {
        return None;
    }
    let roll = u64::from_le_bytes(draw_seed[0..8].try_into().expect("32 >= 8")) % total;
    let mut acc = 0u64;
    for (i, w) in weights.iter().enumerate() {
        acc = acc.saturating_add(*w);
        if roll < acc {
            return Some(i);
        }
    }
    None
}

pub fn choose_build(
    candidates: &[ZoneKind],
    manifold: &SpeciesManifold,
    binding: Option<Drive>,
    gamma_num: u64,
    seed: [u8; 32],
) -> Option<ZoneKind> {
    let weights: Vec<u64> = candidates
        .iter()
        .map(|kind| zone_weight(*kind, manifold, binding, gamma_num))
        .collect();
    // The zone draw salts the chronicle digest with "workbench" — the SAME salt
    // and cumulative walk as before this factoring, so a zone-only beat is
    // byte-identical.
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed);
    hasher.update(b"workbench");
    let draw_seed = *hasher.finalize().as_bytes();
    partition_draw(&weights, draw_seed).map(|i| candidates[i])
}

/// **THE ABUNDANCE MENU** (COG-EMERGENCE-2). Above the survival floor — which
/// `express_survival` raises as the SOLE zone-builder (N-I6) — a line with
/// surplus faces ONE choice, not three authored `_step` gates. Each act is a
/// candidate in the same kind of fold the zone workbench draws from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Investment {
    /// Deepen the mind — seat cognition, lift the compose-depth. Closed by the
    /// cognitive γ (`civic::cognitive_deficit`): an unspanned residual weighed
    /// against a shallow mind's headroom.
    Deepen,
    /// Reach a new niche — acquire territory. Closed by frontier pressure.
    Expand,
    /// Synthesize new matter — seat chemistry. Closed by the chemical residual.
    Synthesize,
    /// Adopt a discipline the line's chronicle has EARNED but not yet taken up
    /// — a practice already in the fold (seed ∪ minted, `28_VOCABULARY`) this
    /// line doesn't hold. Closed by the culture γ: how many such practices
    /// stand unclaimed. Physics still gates the actual adoption (a tempered
    /// discipline needs its hardship receipts) — the will is topological, the
    /// same law every other arm already keeps.
    Culture,
    /// **RCP-2 F4** — raise an emergent recipe (`recipe::discover` →
    /// `build_recipe_as`): a hashed `∏ elements → role` transformation from
    /// the line's own held almanac, gated by topological reach, never a
    /// `ZoneKind`. Closed by the craft γ: how many of the five universal
    /// roles (`morphogen::ROLE_DIM`) the line's almanac COULD fill but hasn't
    /// yet raised a recipe for. A FED line grows its doing-layer
    /// proactively — the discovery search itself stays honest (a role no
    /// held element can fill draws nothing, an honest hole, not a forced
    /// pick).
    Craft,
    /// Bank the surplus and raise nothing — the rest/explore default that wins
    /// when no deficit pulls.
    Hoard,
}

impl Investment {
    /// The menu, in fold order. `Hoard` is the untilted baseline and MUST stay
    /// last — `resolve_investment`'s impossible-zero-weight fallback indexes
    /// `MENU.len() - 1`.
    pub const MENU: [Investment; 6] = [
        Investment::Deepen,
        Investment::Expand,
        Investment::Synthesize,
        Investment::Culture,
        Investment::Craft,
        Investment::Hoard,
    ];
}

/// **THE ABUNDANCE WEIGHTS** — the workbench principle carried to the surplus
/// choice: `w = reach · (L + γ)`, the line's topological richness (`reach`, its
/// algebraic dimension — the same instinct/history the recipe draw reads)
/// TILTED by the deficit each act would close. `Hoard` carries no tilt
/// (`w = reach·L`), so a line at ease reads a UNIFORM menu and the draw
/// EXPLORES; a line under a sharp cognitive γ leans hard toward `Deepen`. NO
/// affordability enters — the weight is topological tension ALONE. The shared
/// `reach` cancels in the draw, exactly as the shared genus does for zones.
pub fn investment_weights(
    reach: &AlgebraicRank,
    cognitive_gamma: u64,
    expansion_gamma: u64,
    synthesis_gamma: u64,
    culture_gamma: u64,
    craft_gamma: u64,
    convergence: u64,
) -> [u64; 6] {
    let lattice = LATTICE as u64;
    let dim = reach.dimension().max(1); // a line always has some reach to spend
    let tilt = |gamma: u64| dim.saturating_mul(lattice.saturating_add(gamma));
    // **SOC-5 — THE COLLECTIVE RESOLVE.** `Hoard` is the society's default rest —
    // bank the surplus, raise nothing. A society that has SELF-ORGANISED around a
    // shared deficit (a concentrated caste census, `caste_convergence` ∈ [0, L])
    // lowers that floor in proportion: it COMMITS its surplus to the shared
    // objective rather than banking it. Convergence 0 — a covered economy, no
    // caste — leaves the floor whole (`dim·L`), so a line at ease rests and
    // explores EXACTLY as before this slice. The tensions still set the DIRECTION
    // (which act); the collective's focus sets only the DECISIVENESS (whether to
    // act at all). Emergent: the concentration is a fold of distributed caste
    // draws, no central plan. Bounded — full convergence zeroes the floor, never
    // negative.
    let hoard = dim.saturating_mul(lattice.saturating_sub(convergence));
    [
        tilt(cognitive_gamma), // Deepen
        tilt(expansion_gamma), // Expand
        tilt(synthesis_gamma), // Synthesize
        tilt(culture_gamma),   // Culture
        tilt(craft_gamma),     // Craft
        hoard,                 // Hoard — the floor the collective resolve lowers
    ]
}

/// **THE ABUNDANCE RESOLVER** (COG-EMERGENCE-2) — the single draw that will
/// replace `cognitive_investment_step`, `chemistry_step`'s seat-decision, and
/// `expand_step`'s gate. The line does not evaluate three `if`s; it FEELS three
/// tensions and draws one act against its own chronicle digest (salted
/// `abundance`, independent of the zone draw). Two lines with the same surplus
/// and the same tensions but DIFFERENT histories (seeds) can choose
/// differently — design, not convergence. If the chosen act is unaffordable,
/// physics rejects it AFTER the fact; its will stays mathematically its own.
/// Replay-exact. (Not yet on the live path — EMERGENCE-2b wires `cycle()`.)
pub fn resolve_investment(
    reach: &AlgebraicRank,
    cognitive_gamma: u64,
    expansion_gamma: u64,
    synthesis_gamma: u64,
    culture_gamma: u64,
    craft_gamma: u64,
    convergence: u64,
    seed: [u8; 32],
) -> Investment {
    let weights = investment_weights(
        reach,
        cognitive_gamma,
        expansion_gamma,
        synthesis_gamma,
        culture_gamma,
        craft_gamma,
        convergence,
    );
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed);
    hasher.update(b"abundance");
    let draw_seed = *hasher.finalize().as_bytes();
    // Every weight is `dim·L >= 1 > 0`, so the draw always resolves; the floor
    // (`Hoard`) is the fallback only in the impossible total-zero case.
    let i = partition_draw(&weights, draw_seed).unwrap_or(Investment::MENU.len() - 1);
    Investment::MENU[i]
}

/// **THE CASTE WEIGHTS** (SOC-4) — the unspanned deficit γ (`EconomicPeel.residual`,
/// what no standing structure spans) read as the society's unmet work, per axis.
/// `|γ_i|` is how much internal specialisation axis `i` calls for. A covered
/// economy weighs all-zero — a homogeneous society, no caste.
pub fn caste_weights(residual: &[BigInt]) -> Vec<u64> {
    residual
        .iter()
        .map(|g| num_traits::ToPrimitive::to_u64(&num_traits::Signed::abs(g)).unwrap_or(u64::MAX))
        .collect()
}

/// **THE CASTE DRAW** (SOC-4) — which residual-axis caste one unit of the society
/// joins, weighted by the unspanned deficit on each axis. The society does not
/// ASSIGN castes by a table; each unit is DRAWN toward the work that is most
/// unmet, off the line's own digest (salted `b"caste"` to decorrelate from the
/// zone `b"workbench"` and abundance `b"abundance"` draws). `None` when the
/// economy is covered (every residual zero) — no caste forms. Replay-exact.
pub fn caste_draw(residual: &[BigInt], seed: [u8; 32]) -> Option<usize> {
    let weights = caste_weights(residual);
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed);
    hasher.update(b"caste");
    let draw_seed = *hasher.finalize().as_bytes();
    partition_draw(&weights, draw_seed)
}

/// **THE CASTE CENSUS** (SOC-4) — partition `members` of the society across the
/// residual axes: each member DRAWS its caste independently (salted by index), so
/// the census is a stochastic partition that SELF-ORGANISES proportional to the
/// residual, never a centrally-assigned quota. Per-axis head counts (len =
/// `residual.len()`); all zero when the economy is covered. Two societies with
/// the same residual but different digests organise DIFFERENTLY — emergence, not
/// a caste table. Replay-exact.
pub fn caste_census(residual: &[BigInt], members: u64, seed: [u8; 32]) -> Vec<u64> {
    let mut census = vec![0u64; residual.len()];
    for m in 0..members {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&seed);
        hasher.update(&m.to_le_bytes());
        let member_seed = *hasher.finalize().as_bytes();
        if let Some(axis) = caste_draw(residual, member_seed) {
            census[axis] += 1;
        }
    }
    census
}

/// **THE COLLECTIVE RESOLVE** (SOC-5) — how sharply the society has CONVERGED on
/// a shared objective, read off its caste census as a lattice fraction ∈ `[0, L]`.
/// The measure is the dominant caste's SHARE of the organised workforce: everyone
/// drawn to one axis → `L` (a society of one mind, fully focused); an even spread
/// over `k` axes → `L/k` (organised but diffuse); a covered economy that seats no
/// caste → `0` (nothing to converge on — rest). This is the SHAPE of the deficit
/// the scalar `residual_extent` cannot see: two societies carrying the same total
/// void differ here iff one faces a single sharp deficit and the other a scatter.
/// The abundance fold reads it as the surplus's DECISIVENESS (`investment_weights`
/// lowers `Hoard` by it), never its direction. Replay-exact — a pure fold.
pub fn caste_convergence(census: &[u64]) -> u64 {
    let total: u64 = census.iter().sum();
    if total == 0 {
        return 0; // no caste seated — a covered economy, at ease, converges on nothing
    }
    let dominant = census.iter().copied().max().unwrap_or(0);
    // The dominant share, scaled to the lattice: focus, not headcount (the raw
    // count would explode the weight; the fraction is grained by construction).
    dominant.saturating_mul(LATTICE as u64) / total
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed(n: u8) -> [u8; 32] {
        *blake3::hash(&[n]).as_bytes()
    }

    // A balanced (biomass, metabolism 2) line runs the full forge chain.
    fn balanced() -> SpeciesManifold {
        SpeciesManifold::of(2, "aqueous", false)
    }

    #[test]
    fn a_native_pathway_pairs_to_zero_and_scores_positive() {
        // THE 𝒯 LAW: a balanced line touches Iron and Alloy and the Iron→Alloy
        // conversion is relevant, so the forge pathway is bounded — its class
        // vanishes, its affinity is W_base·genus > 0.
        let m = balanced();
        assert!(m.bounds(ZoneKind::Smelter), "the iron forge is native to a balanced line");
        assert!(m.affinity(ZoneKind::Smelter) > 0);
        // The sources are always native — the environment bounds them.
        assert!(m.bounds(ZoneKind::Extractor) && m.bounds(ZoneKind::Habitat));
    }

    #[test]
    fn a_plasmic_line_is_homologically_excluded_from_the_iron_forge() {
        // THE HOLE, WITNESSED (the operator's lithophage): a plasmic metabolism
        // cannot run the Smelter/Synthesizer, so Alloy and Diamond are never
        // reachable — they are ABSENT from its complex. The forge pathway cannot
        // even be expressed; the pairing is a non-trivial obstruction; 𝒯 → 0.
        // No boolean gate, no species table — a topological fact.
        let plasmic = SpeciesManifold::of(3, "plasmic", false);
        assert!(!plasmic.bounds(ZoneKind::Smelter), "the iron forge is a hole for a plasmic line");
        assert_eq!(plasmic.affinity(ZoneKind::Smelter), 0, "obstructed → excluded from Z");
        assert_eq!(plasmic.affinity(ZoneKind::Synthesizer), 0, "the deeper forge too");
        // A plasmic line never mines ore either — iron is simply not in its world.
        assert_eq!(plasmic.affinity(ZoneKind::Extractor), 0, "no iron to extract");
        // Yet it still settles — the ration house (its Joule energy) is native.
        assert!(plasmic.affinity(ZoneKind::Habitat) > 0);
    }

    #[test]
    fn recycling_raises_the_genus_a_regenerative_society_is_topologically_richer() {
        // INTELLIGENCE IS GENUS: closing the ration loop (a regenerative economy)
        // adds an independent cycle β₁ — the manifold's genus rises. The same
        // clade, recycling, is a topologically more complex society.
        let extractive = SpeciesManifold::of(2, "aqueous", false);
        let regenerative = SpeciesManifold::of(2, "aqueous", true);
        assert!(
            regenerative.genus > extractive.genus,
            "the recycling loop lifts the genus ({} > {})",
            regenerative.genus,
            extractive.genus
        );
    }

    #[test]
    fn intelligence_reads_as_the_algebraic_presentation_not_a_scalar() {
        // OPERATOR RULING: reach reports the SNF invariant PAIR — free generators
        // (algebraic dimension) ⊕ relations (torsion) — the two projections of the
        // chain complex's free-resolution presentation, on the boundary already
        // factored in `of`.
        let extractive = SpeciesManifold::of(2, "aqueous", false);
        let regenerative = SpeciesManifold::of(2, "aqueous", true);
        let ext = extractive.algebraic_rank();
        let reg = regenerative.algebraic_rank();

        // The dimension scalar equals the genus the draw uses — nothing lost, the
        // split just made explicit.
        assert_eq!(ext.dimension(), extractive.genus);
        assert_eq!(reg.dimension(), regenerative.genus);

        // Regeneration is a RELATION, not a new generator: it adds a metabolic
        // CYCLE (β₁), the syzygy a recycling loop introduces — the component count
        // is unchanged, only the cycle rank rises. Krull dimension alone (free
        // generators) would miss it; the pair does not.
        assert_eq!(reg.components, ext.components, "recycling adds no new capability axis");
        assert!(reg.cycles > ext.cycles, "it adds an independent cycle ({} > {})", reg.cycles, ext.cycles);

        // A metabolic graph is torsion-free, so the relation coefficients are
        // empty — the seam higher cells (the RCP-2 / DIO-3b nonlinear frontier)
        // enter through.
        assert!(ext.relations.is_empty() && reg.relations.is_empty(), "graph homology carries no torsion yet");
    }

    #[test]
    fn the_metabolic_trait_seed_diverges_the_native_set_across_species() {
        // THE ENRICHMENT, WITNESSED: viability is now the species' constitutive
        // nature, not a universal baseline. A mineral line's world is built of
        // iron (the forge is native); a plasmic line's world has no iron at all
        // (the forge is a hole); a biotic line is the generalist that mines iron
        // for tools yet lives on biomass. Three species, three different sets of
        // what is even buildable — with no per-building exclusion table.
        let mineral = SpeciesManifold::of(1, "lithic", false);
        let biotic = SpeciesManifold::of(2, "aqueous", false);
        let plasmic = SpeciesManifold::of(3, "plasmic", false);

        assert!(mineral.bounds(ZoneKind::Smelter), "the iron forge is native to a mineral line");
        assert!(biotic.bounds(ZoneKind::Smelter), "the generalist biotic line mines iron too");
        assert!(!plasmic.bounds(ZoneKind::Smelter), "a plasmic line has no iron in its world");
        assert!(!plasmic.bounds(ZoneKind::Extractor), "and never reaches for the ore");
        // Yet each still settles — the ration house is native to all.
        assert!(mineral.bounds(ZoneKind::Habitat) && plasmic.bounds(ZoneKind::Habitat));
    }

    #[test]
    fn the_draw_excludes_obstructions_and_stays_replayable() {
        let plasmic = SpeciesManifold::of(3, "plasmic", false);
        // Offered the iron forge and its own ration house, a plasmic line can
        // only draw the settlement — the forge is weight 0, out of the partition.
        let surface = [ZoneKind::Smelter, ZoneKind::Habitat];
        for n in 0u8..16 {
            assert_eq!(
                choose_build(&surface, &plasmic, None, 0, seed(n)),
                Some(ZoneKind::Habitat),
                "the obstructed forge is never drawn"
            );
        }
        // And the draw is replayable: same digest, same building.
        let m = balanced();
        let full = [ZoneKind::Smelter, ZoneKind::Extractor, ZoneKind::Habitat];
        assert_eq!(choose_build(&full, &m, None, 0, seed(5)), choose_build(&full, &m, None, 0, seed(5)));
    }

    #[test]
    fn among_natives_the_workbench_is_open() {
        // With every candidate native (equal W_base·genus), the draw spreads
        // across the surface as the digest varies — the workbench is genuinely
        // open, not a fixed priority.
        let m = balanced();
        let surface = [ZoneKind::Smelter, ZoneKind::Extractor, ZoneKind::Habitat];
        let mut seen: Vec<ZoneKind> = Vec::new();
        for n in 0u8..48 {
            if let Some(k) = choose_build(&surface, &m, None, 0, seed(n)) {
                if !seen.contains(&k) {
                    seen.push(k);
                }
            }
        }
        assert!(seen.len() >= 2, "the draw explores more than one native building");
    }

    #[test]
    fn the_binding_drive_tilts_the_workbench_toward_the_closer() {
        // SOC-3 LAW: a society STARVED on its metabolic tier (binding = Metabolic,
        // γ maxed) leans toward the food/settlement building that closes it far
        // more often than a SATED society, whose draw stays uniform. The society
        // builds what it NEEDS under pressure and EXPLORES at ease.
        let m = balanced();
        let surface = [ZoneKind::Smelter, ZoneKind::Extractor]; // industry vs food
        let full = LATTICE as u64; // maximal deficit fraction
        let starving = (0..90u8)
            .filter(|i| {
                choose_build(&surface, &m, Some(Drive::Metabolic), full, seed(*i))
                    == Some(ZoneKind::Extractor)
            })
            .count();
        let sated = (0..90u8)
            .filter(|i| choose_build(&surface, &m, None, 0, seed(*i)) == Some(ZoneKind::Extractor))
            .count();
        assert!(
            starving > sated,
            "starvation tilts toward the food-closer ({starving} vs {sated} of 90)"
        );
        // The tilt is a lean, not a lock: industry is still sometimes drawn even
        // while starving (the workbench stays open).
        assert!(starving < 90, "even a starving line still explores ({starving})");

        // And a SATED society (binding None) reads exactly the SOC-2 uniform draw:
        // the tilt vanishes, every native equally weighted regardless of drive.
        for i in 0u8..16 {
            assert_eq!(
                choose_build(&surface, &m, None, 0, seed(i)),
                choose_build(&surface, &m, Some(Drive::Metabolic), 0, seed(i)),
                "γ = 0 collapses the tilt to the uniform SOC-2 draw"
            );
        }
    }
}
