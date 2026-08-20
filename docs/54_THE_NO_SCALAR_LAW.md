# 54 - THE NO-SCALAR LAW and the OCULAR re-founding

Operator ruling 2026-08-20. Standing law, no exception.

## 1 The law

A scalar has no home in this system. A scalar is a projection of a graded
object onto a single number, and a projection cannot be inverted: the moment a
vector, a cochain, or a holonomy is summed, normed, or quantized to one value,
its interpretation is gone and the reading is prone to collapse. Every organ's
interpretive state - what it sees, what it lacks, how it has organized - must
remain a graded object (a cochain, a divergence field, a holonomy list) carried
exact-rational, never a scalar and never projected onto a fixed integer grain.

There is no terminal-projection exception. A draw over options is a draw over the
graded classes themselves, not over a vector of scalar weights. A price is the
homological complexity of an obstruction, carried as its class, not a number.

This generalizes the scalar-collapse-derivation-law (ONT-2/3/4, which killed the
stored-scalar recurrence collapses) to an absolute: not "derive the scalar from
geometry" but "do not form the scalar at all."

## 2 What was verified

The engine already owns the loss-free organ; only the civic/economy layer was
built on the wrong one.

- `rfa_core` is a pure exact-rational cochain engine. `Delta` is a 1-cochain:
  antisymmetric, fiber-valued, every entry a `Vec<Ratio>`. `Support` is the cover
  - which relations are present, per coordinate. The cocycle condition is
  explicit: a relation composes iff `Delta_ik = Delta_ij + Delta_jk` on every
  triple. A three-orb obligation triangle that does not compose is a genuine H^1
  class - the obstruction no coordinate frame explains. That is the spherical
  venn: the nerve of a cover, and the irreducible content living on its overlaps.
- `holonomies(delta, support)` returns, per independent cycle, the exact charge
  the reconciled frame (`forest_potentials`) cannot account for - each carrying
  (pair, coordinate, exact value). Nothing sums, nothing rounds.
- `divergence(delta)_i = sum_j Delta_ij` is the signed source/sink reading:
  egress (a node produces more than it absorbs), ingress (it sinks), or balanced.
  The books close (sum of divergences is zero by antisymmetry). This IS an
  economy's net production-minus-consumption, per resource, exact, conserved.
- `coboundary_reduce` already clears denominators exactly (multiply by the lcm,
  divide by the gcd of numerators) when an integer representative is genuinely
  needed - the correct "exact lattice," not a fixed 64-grain.
- The `lens` crate is the ocular organ: `Observation::at(delta, support)` with
  aperture, depth, luminal, regard, and `peel(delta, support, depth) -> View`.
  "What does the entity see" is already answered here, on `rfa_core`.

The lone holdout is `conductor/{civic.rs, soc.rs}`: it is on `substrate::snf`
(Z-module Smith Normal Form) plus `LATTICE = 64` quantization plus scalar reads.
It does not even depend on `rfa_core` or `lens`. It re-invented a lossy shadow of
the organ sitting one crate away.

## 3 The scalar defects in the civic layer

Every one is the same disease - reading the economy through the wrong organ:

- `quantize` / `LATTICE = 64` (`civic.rs`): exact-rational rates projected onto a
  coarse fixed integer grain to feed SNF. Anything finer than 1/64 rounds to
  covered. Everything downstream inherits the floor.
- `residual_extent = sum |g_i|` (`civic.rs`): the deficit's L1 norm, direction
  discarded, feeding the abundance tensions. A deep Iron deficit and a deep
  Crystal deficit read identical.
- `caste_convergence = dominant / total` (`soc.rs`, SOC-5): the census vector
  reduced to a norm, the shared objective's identity thrown away.
- `stress_quantized`, `classify_defect`'s magnitude comparison, and the
  `LATTICE`-baseline partition weights: the same collapse, smaller.

## 4 The re-founding, without loss of interpretation

Re-found the civic organ on the ocular one. The economy is a connection, not a
quantized matrix:

- Nodes are the resource axes (with an environment/sink node). `Delta_ij` is the
  exact-rational net flow between them - conversions carry their yield rates,
  production and demand are flows to and from the environment node. `Support` is
  the species' runnable conversions (the SOC-2 metabolic manifold, read as a
  cover, not an SNF boundary matrix).
- What the society sees = `divergence` of that connection: the signed, per-axis,
  exact deficit (an unbalanced ingress is an unmet need; the sign and the axis
  both survive). Obstructions = `holonomies`: conversion cycles that do not
  compose. The price = the homological class of the obstruction, via
  `coboundary_reduce`, not a flat number.
- Every consumer - tensions, caste census, investment - reads the divergence
  field and the holonomy classes, never a norm. The collective converges on the
  actual class it cannot compose, direction intact.

Interpretation is preserved by construction: a holonomy and a divergence entry
carry (axis / pair, coordinate, exact rational value). Nothing rounds (exact
Ratio), nothing sums to a scalar (the field is the reading), nothing lands on a
fixed grain (denominators clear exactly, only when an integer representative is
needed).

## 5 The arc (OCULAR-1..n)

1. OCULAR-1: `conductor` depends on `rfa_core` (and `lens`). A module builds the
   economy as `Delta` + `Support` from the same committed state `civic_reading`
   reads today. Additive; witnessed that its `divergence` names the same signed
   deficit the SNF residual finds. No live behavior changes yet.
2. OCULAR-2: migrate the deficit read - the abundance tensions and `civic_peel`
   consume the divergence field, retiring `residual_extent`.
3. OCULAR-3: migrate the collective - the caste census and the investment draw
   read the divergence classes, retiring `caste_convergence`'s scalar and the
   `LATTICE` weights; the draw is over classes, not scalar weights.
4. OCULAR-4: delete `quantize`, `LATTICE`, and the `substrate::snf` dependency
   from the civic layer. `stress_quantized` and `classify_defect` fall with them.

SEAL-B stays green throughout (the beat stays deterministic); the numbers change
because they stop being rounded. Each slice commits green with its witness.

## 6 The complementary law — every ledger declares its grain at its fill site

Operator ruling 2026-08-20 (the third occurrence of one disease: leisure banks, the
d_orbit hypothesis, now element stocks). The no-scalar law above governs COGNITION,
where exactness is unbounded and protected forever. Its complement governs the
WORLD:

**Every conserved world-physics ledger declares its grain at its fill site.
Unbounded exactness is reserved for cognition, where it is law.**

A grain is a quantum of measure on a fluid medium — not a lattice. A lattice is a
rigid crystal that collapses dimensionality; the SMELT floor does the opposite: it
holds a *stock* to a measurable resolution while the medium stays fluid. We do not
say "lattice" here, and the word must not creep back in: names shape what gets
built, and a conserved ledger is a measured fluid, not a crystal.

A conserved ledger (element stocks, banked matter, leisure) is quantised to its own
grain by SMELT-flooring every fill at the site the delta lands — `smelt(x) =
floor(x·64)/64`, the 1/64 grain the melt law speaks. This is NOT the environment
grain-rate (`grain_rate`, a best-rational approximation of a *rate* at a call site);
a floor at a fill and a rate-grain at a call site are different instruments, and
element math must use neither rate-grain nor unbounded drift. A fill smelted below
one grain refuses whole; the shaved remainder never vanishes (it stays with the
giver, or the audited void records it — matter included).

The disease this prevents is not denominator blowup as such — it is discovering,
per hot loop, that a ledger never declared its grain. `haul_element` bypassed the
SMELT and element stocks drifted to 107,756-bit denominators, stalling `mass_of` at
96 s/beat; the trace convicted it (top_build 6 ms, mass_build 96350 ms). One
standing rule ends the series: when you add a conserved world-ledger, floor its
fills.

The forward fix stops NEW drift; the existing exploded stocks are floored once by a
committed **smelt-sweep event** in the era stream (a receipted deed, so
`replay == committed` holds across historical eras), carrying conservation-with-dust
(pre = post + recorded dust, bit-exact — residuals never vanish). A standing
negative witness iterates every conserved ledger after the sweep and asserts the
grain invariant, so a fourth occurrence fails the build, not a beat. (#136;
witnesses in `cfe/tests/element_smelt_pins.rs`.)
