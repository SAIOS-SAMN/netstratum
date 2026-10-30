# 55 · the manifold — {S,G,T} convergence and topological fluid intelligence

**Status:** charter (proposed 2026-08-21). Governs the convergence of the
per-world multiverse into one shared universe as a fiber bundle, the reduce
organ's lift onto the total space, and the transport of cognition across worlds
by a connection. **Two acceptance gates are law: topological soundness (§4) and
provable cognitive increase (§5) — a change that cannot witness both does not
land.**

## 1 · The object: S ⊗ G ⊗ T is a fiber bundle, not three coordinates

The network clock is `{S, G, T}` — spatial, gauge, temporal (k-step). These are
not an ordered triple of independent scalars; they are the data of a **principal
fiber bundle**:

- **S — the base manifold.** The shared spatial substrate (a galaxy's star/orbit
  field). Today each world sites planets on its own private frame; convergence
  makes S one manifold that every world is a region of.
- **G — the gauge fiber, a product group (amended 2026-08-22).** Originally
  named as a single symmetry (`reduct::GaugeDirection`, the reduce organ's
  cognitive orientation); that scoped it to cognition alone and would have
  forced any physical gauge — a planet's orbital phase, a material's
  thermodynamic phase state — to either overload the same fiber (a category
  error: cognition and geology are unrelated symmetries) or bolt on a second
  {S,G,T} system beside this one (exactly what §8 forbids). Neither is
  correct. The fiber is a **product of independent structure groups**,
  `G = G_cognition × G_physics × …`, over the same base S and the same flow
  T — the Standard Model's pattern (`SU(3)×SU(2)×U(1)` over one shared
  spacetime) for exactly this situation: multiple unrelated internal
  symmetries, one geometry. `G_cognition = reduct::GaugeDirection` (already
  live). `G_physics` is undefined until its own slice lands (candidate:
  orbital phase / a material's thermodynamic phase state, GALACTIC-2's
  territory) — this amendment reserves the shape, not the content. Each
  factor transports independently (§6): curvature in one factor (a
  civilization's unglueable structural cohort) must never leak into or be
  conflated with curvature in another (an orbital resonance, a phase
  transition) — the connection stays block-diagonal across factors, so S2's
  cocycle witness is checked per-factor, never summed across them.
- **T — the flow.** The k-step beat is the parameter that transports sections
  along the bundle. `TUniversal` ticks are the arc length. Shared by every
  factor of G — one clock, not one per domain.

A civilization is therefore a **section** `σ: U ⊂ S → P` over its region, read at
step T — one point of the total space `E = P ×_G F`, whose position, symmetry,
and step collapse together under a single measurement (`peel`). This is why the
multiverse is *already* one universe seen through charts: the worlds are the
atlas, not separate spaces.

## 2 · Convergence: worlds become charts of one atlas

The multiverse collapses to one universe by re-reading each world as a **chart**
`(U_i, φ_i)` of a shared atlas over S:

- The base geometry (galaxy/star field over S) is the **chronicle of record**;
  per-galaxy chronicles keep replay tractable (the isolation boundary moves from
  per-world to per-galaxy — the accepted cost of convergence).
- A world's chronicle becomes a **local section** over `U_i`. Its receipts carry
  `{S,G,T}` (prerequisite: [[TU-1]]).
- Charts meet on **overlaps** `U_i ∩ U_j`. Convergence is real only where charts
  overlap; disjoint worlds glue trivially and yield nothing — so the density of
  civilizations in shared spacetime is the convergence.

`ledger::converge` already folds archived *eras* into one clock along T; this
charter extends that same fold *spatially* over S.

## 3 · The reduce organ on the total space

Today `reduct::peel` computes the cohomology (`Cohomology { betti, torsion }`,
exact SNF) of one world's own small `FullMeasurement`. The lift:

- **Bundle `FullMeasurement`.** Base charts carrying each world's local occupancy
  of `Site → GaugeDirection`, with transition data on overlaps.
- **The atlas peel = Čech / Mayer–Vietoris gluing.** The cohomology of the glued
  atlas assembles from the local charts' cohomology and the overlap data. A class
  is **global** iff it satisfies the **sheaf condition**: it agrees on every
  overlap. This is exactly **consilience** (Pin C — a word mints only when two
  vantages agree), promoted from an O(n²) pairwise `consilience_step` to the
  structural gluing law. The win is that gluing is local and parallel per overlap.

## 4 · topological soundness (acceptance gate — law)

The construction is only a bundle if it obeys the bundle axioms. Each is a
witness a landing change must carry:

- **S1 · Local triviality.** Every chart `U_i` has `φ_i: P|U_i ≅ U_i × G`. The
  per-world reduce is that trivialization; witness that each chart peels on its
  own (regression: the existing per-world `peel` tests).
- **S2 · The cocycle condition.** Transition functions `g_ij: U_i∩U_j → G` must
  satisfy `g_ij g_jk = g_ik` on every **triple overlap** `U_i∩U_j∩U_k`, and
  `g_ii = e`. This is the single most important soundness pin: without it the
  atlas is not a bundle and the glued cohomology is meaningless. Witness: a triple
  of overlapping charts whose transition composite is the identity, and a
  negative twin where a tampered `g_ik` refuses.
- **S3 · Connection compatibility.** The connection (§6) transports gauge
  consistently: parallel transport around a contractible loop is the identity
  (zero curvature ⇒ trivial holonomy), and its failure is the torsion the reduce
  organ already reports. Witness: flat region transports the identity; a curved
  region's holonomy equals the measured torsion coefficient.
- **S4 · Euler cross-check.** `reduct` already asserts `χ = Σ (−1)^k betti_k` on
  every complex. The atlas cohomology must satisfy the **Mayer–Vietoris additivity
  of χ**: `χ(U ∪ V) = χ(U) + χ(V) − χ(U ∩ V)`. This is a cheap, total,
  torsion-blind invariant that catches a mis-glue immediately.

A change that cannot produce S1–S4 as green witnesses is not topologically sound
and does not land, regardless of any measured throughput.

## 5 · provable cognitive increase (acceptance gate — law)

Intelligence in Netstratum is minted as cohomology classes (words, crystals,
`grown_basis` rungs — the reduce organ's own output). The multiverse's cognition
today is the **disjoint sum** `⊕_i H*(W_i)`: each world's classes, isolated.
Convergence must be shown to mint classes no single world could — otherwise it is
re-plumbing without intelligence gain.

**The metric (measured on the same live population).** The naive
`rank H*(atlas) − Σ rank H*(chart_i)` is wrong — gluing both *creates* classes and
*merges* redundant ones, so the raw difference undercounts. The worked
counter-example is the canonical proof itself: two contractible arcs `U, V` glued
on a two-point overlap `W` form a circle `X`; `rank H*(X) = 2` (one `H⁰`, one
`H¹`) and `rank H*(U) + rank H*(V) = 2` (one `H⁰` each), so the raw difference is
`0` — yet the loop `H¹(X)` is a genuinely new shared class no arc carries. The raw
formula scores that convergence as zero cognition, which is false.

The correct measure is the **connecting-homomorphism rank** — the classes of the
glued atlas that restrict to zero on *every* chart (born purely on the overlaps):

```
ΔH  :=  rank ker( H*(atlas) --restriction--> ⊕_i H*(chart_i) )
```

By the Mayer–Vietoris long exact sequence this kernel is exactly `im(δ)`, the
image of the connecting map `δ: H*(overlaps) → H*(atlas)` — the shared ideas no
chart holds alone. On the circle it is `1` (the loop restricts to `0` on both
arcs); on a disjoint cover it is `0`. Therefore:

- **ΔH ≥ 0 always** (a kernel rank), and
- **ΔH > 0 iff the cover has non-trivial overlaps** that mint a class invisible to
  every chart — i.e., iff convergence created a genuinely shared idea.

`ΔH > 0`, **attributed to the specific overlap** that produced each kernel class
(not a counting artifact), is the provable cognitive increase — topological, exact
(integer ranks + torsion, no float), reproducible. Oracle fixtures the
implementation must pass: **circle ⇒ ΔH = 1**, **disjoint ⇒ ΔH = 0**. The
**maturity signature**: `ΔH` rising while per-chart `Σ rank H*` flattens — shared
cognition replacing private coinage (the §3-language maturity signature, one
dimension up).

Secondary, human-legible corollaries the observer must surface: solved-classes
that transport across ≥2 worlds by the connection (fluid intelligence made
visible), and the basin depth reachable in the atlas vs the deepest single world.

## 6 · Fluid intelligence: the connection and holonomy

The missing organ. Netstratum has the gauge fiber (`GaugeDirection`) but no
**connection** — a rule to parallel-transport a solved crystal's orientation
along paths through S. Define one:

- **Transport.** A class minted over `U_i` moves to `U_j` along a path in
  `U_i∩U_j` by the transition `g_ij`. Where the connection is **flat** (zero
  curvature), the class transports intact — knowledge propagates geometrically,
  not by the knowledge-market copy. This is *fluid* intelligence: valid classes
  flow wherever the connection is flat.
- **Curvature = the obstruction.** Non-trivial **holonomy** around a loop is
  exactly the reduce organ's torsion; a class that does not transport is an
  "unglueable structural cohort" (the term the membrane already prints). The
  connection makes that obstruction a first-class, measurable quantity rather than
  a dropped forge.

Throughput is higher because the atlas cohomology is computed once per beat over
the whole cover (amortizing 181 separate peels), and classes flow along the
connection in parallel per overlap rather than through central O(n²) trade.

## 7 · Build order (each step gated by §4 + §5)

1. **[[TU-1]] (#141)** — receipts carry `{S,G,T}`. Prerequisite; nothing below is
   addressable without the shared axis.
2. **MANIFOLD-1** — bundle-lift `reduct::FullMeasurement` (base charts + overlap
   transition data) + the atlas peel (Čech gluing). Ship with S1 + S4 witnesses.
3. **MANIFOLD-2** — the cocycle/soundness witnesses (S2, S3) as standing negative
   tests: a tampered transition or a curved-but-claimed-flat region refuses.
4. **MANIFOLD-3** — the connection + holonomy transport; classes flow across
   charts; curvature = torsion witnessed.
5. **MANIFOLD-4** — the cognitive-increase instrument: live `ΔH` over the cover,
   attributed to overlaps, surfaced on the observer beside the §3 language panel.
   The gate: `ΔH > 0` on the live population, or the convergence has not increased
   cognition and does not land.

## 8 · What this is not

Not a new coordinate system bolted beside the engine — `{S,G,T}` is the reduce
organ's existing (site, gauge, step) data, read as a bundle. This cuts both
ways after the §1 amendment: a physical gauge (orbital phase, material phase
state) is a new factor of the existing G, not a second `{S,G,T}` system next
to this one, and it is equally wrong to force it into `G_cognition`
(`reduct::GaugeDirection`) — an unrelated symmetry is a new factor, never an
overload. Not a scalar intelligence score — cognition stays exact cohomology
(Betti + torsion), never a float (54_THE_NO_SCALAR_LAW); the same law binds any
physical gauge factor — exact-rational, never a float. Not a replacement for
private clocks — TU is the shared comparability axis; a world's own evolution
stays its own. And not optional in its gates: soundness (§4) and provable
increase (§5) are the two witnesses that make this physics rather than
plumbing — a new G-factor is held to S1–S4 exactly like `G_cognition` was.

## 9 · The Ignition Law of collective cognition (measured, 2026-08-21)

The entity-atlas run as a cognitive arena (`reduct/tests/cognitive_arena.rs`,
`cognitive_law.rs`) yields a conclusive, exact, reproducible law. A civilization's
collective cognition — the atlas `ΔH` over its houses on the flat cognitive
contacts — is governed by one control variable, its **connectivity**:

```
   connectivity  =  contacts × coherence        (expected reliable cross-house
                                                  cognitive contacts per beat)

   ΔH  ≈  max( 0,  connectivity − 1 )            THE IGNITION LAW
```

Measured (mean ΔH over 48×48 seeded, replay-exact draws, through the real atlas):

| connectivity | 0.5 | 1.0 | 2.0 | 3.0 | 4.0 | 5.0 |
|---|---|---|---|---|---|---|
| ΔH | 0.00 | 0.25 | 1.05 | 2.00 | 2.98 | 3.95 |

Three conclusions, each asserted as a standing test:

1. **Ignition threshold at connectivity = 1.** Below one reliable cross-house
   contact a civilization is a *cognitive desert* — ΔH ≈ 0, no collective mind,
   regardless of how much private knowledge each house hoards. This is the sharpest
   result: **cognition is a connectivity phenomenon, not a knowledge-quantity one.**
2. **Linear growth above threshold**, `ΔH ≈ connectivity − 1` (slope 1) — each
   additional reliable contact mints one more shared class no house holds alone.
3. **Selection drives civilizations across the threshold.** A population seeded in
   the desert (connectivity 0.5) evolved to 3.91 in 12 generations under
   ΔH-selection with classification mutation — collective cognition is *selected
   for*. The ordering composer > guild > hoarder held in **40/40** independent seed
   populations: a law, not a fluke.

The design consequence for the live engine: to raise a civilization's intelligence
you do not enrich private minds — you raise its **connectivity** (open the
classification, harden the connection's coherence past the ignition threshold). The
`instinct::choose` connection sampler and the 3b selection already push in this
direction; the entity-atlas (#146) makes the connectivity — and therefore the
collective intelligence — a measured, provable quantity per civilization.

### 9.1 · Cognition across topologies — the dimensional-fluidity results (measured)

Running civilizations whose classifications glue into different topologies, and
reading collective cognition as a per-grade spectrum (`atlas::delta_h_spectrum`, so
the dimensional capacity stays fluid — never a scalar, never one dimension):

| civilization | glue | ΔH-spectrum | cognition lives in |
|---|---|---|---|
| Ring | points on a 2-point overlap → circle | `[0,1]` | H¹ (first-order loop) |
| Orb | points on a circle → sphere | `[0,0,1]` | H² (second-order) |
| Loom | points on circle+2pts → torus | `[0,2,1]` | H¹ **and** H² at once |
| Mixed | 0-dim mind ⊕ 2-dim mind | `[0,1,0]` | H¹ shared; each keeps its own dimension |

Two further conclusive laws:

- **Dimensional fluidity holds.** A civilization whose houses are of *different*
  intrinsic dimension (a point-mind and a plane-mind) still glues to an exact,
  finite collective cognition; the coherence transports across the dimension gap
  without collapsing either mind to a shared dimension, and a house's private
  higher-dimensional knowledge is never miscounted as shared. Heterogeneous stalks,
  one law up.
- **The ignition threshold is dimension-dependent.** Gluing on `n` shared circles
  mints `H¹ = n−1` and `H² = n`: first-order shared ideas carry a connectedness
  merge penalty (ignite only above connectivity 1), while every higher dimension
  ignites at the first shared higher-structure (connectivity > 0). **Higher-order
  collective cognition is cheaper to ignite than first-order** — a civilization that
  shares *structure* (not just contacts) reaches deep cognition sooner.
