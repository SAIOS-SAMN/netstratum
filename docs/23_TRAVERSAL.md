# 23 — THE TRAVERSAL LANE AND THE PATH POLYNOMIAL

**Status:** LAW, split out of the retired `18_TORSION.md` on 2026-08-02 by operator
ruling. It carries the traversal and path-algebra thread.

**Why it moved.** `18_TORSION` was the physics subset that constrained the
computational mathematics. It had grown to 2,020 lines and twenty §2
subsections, ten of them added in one day. Measured: of those twenty,
CODE cites only §2a, §2b, §2d, §2e, §2f and §2h. Every section moved
here was cited by no code at all.

**Splitting is not striking.** Nothing here is withdrawn; it is filed
where it belongs so the physics subset stays a physics subset.

---

**GROUNDING NOTICE (ARTICLE III LAW 8).** *A term that computes is
kept; a term that only resonates is excised.* Measured 2026-08-02: no
code in `engine/` implements a path polynomial or a traversal lane.
The single code citation to this material — `kernel/src/motion.rs`
— references §2k.1 and is itself marked as superseding it.

**This document is therefore DESIGN-STAGE, not landed law**, and is
marked so rather than left to read as constitutional. It either
acquires a consumer or LAW 8 applies to it on its own terms.

---

### 2k. THE TRAVERSAL LANE — Einstein–Rosen throat, Einstein–Cartan interior (operator, 2026-08-02)

The bond between two interlinking orbs (`46_KERNEL` §5a.1) is a THROAT,
not a link. This section says what keeps the throat OPEN, and the
answer carries an economic ruling with it.

**What Einstein–Cartan actually adds, stated plainly.** General
relativity sets torsion to zero by fiat: the connection is symmetric,
so it has no antisymmetric part. Einstein–Cartan drops that assumption
and lets the connection carry one — and that antisymmetric part is
sourced by intrinsic SPIN, not by mass-energy. The consequence that
matters here: under mass-energy alone, collapse runs all the way to a
singularity — a point, where everything that fell in is crushed into no
structure at all. With torsion sourced by spin, the coupling turns
repulsive at extreme density and the collapse BOUNCES instead of
reaching a point.

**Torsion is the thing that keeps a region EXTENDED.** That is the
entire borrow, and it is why the pair is Einstein–Rosen *plus*
Einstein–Cartan and not either alone: the bridge supplies the throat,
and the torsion is what stops the throat pinching to a point that
nothing can traverse.

**The mapping, and it is exact at the place it has to be:**

    mass-energy       accumulated weight — spent compute. Bulk,
                      structureless, additive, commuting.
    intrinsic spin    the irreducible relational structure: the part
                      of a loop that no frame can undo.
    TORSION           the COMMUTATOR. Not a metaphor for it —
                      geometric torsion IS the antisymmetrised part
                      of the connection, and `fiber.rs:101`
                      (`commutator`) with `fiber.rs:168`
                      (`plaquette_curvature`, which returns exactly
                      `a.commutator(b)`) computes that object over
                      the fiber. The organ is landed.

**SUPERSEDED IN ITS ARROW — read §2k.1 with this table.** The rows
above are written as though spin were one thing and torsion its
response. §2k.1 corrects that: they are one object at two scales, the
residual being the fine reading and rank `D` the coarse one. The
mapping survives; the direction of the arrow does not.

**THE SINGULARITY AND THE COBOUNDARY TRAP ARE THE SAME FAILURE.** This
is the load-bearing identification. A pure coboundary
`Δ_ij = s(i) − s(j)` reduces to zero — `reduce.rs` phase 1 sends
exactly that shape there, which is why `bridge.rs` refuses to project
bank differences. Everything poured in, no invariant out, all of it
crushed to a point. That is a singularity in the only sense this system
can mean one, and the system has forbidden it since the bridge landed.

**Which is why nonce-smashing is barred, and barred by a rule that
already exists.** Bitcoin's race is that failure adopted as a design:
enormous energy in, and the artifact — a nonce — carries no
transferable structure and is worth exactly zero the instant it is
found. Additive, commuting, abelian: `[A,B] = 0`, no torsion, no
extent, a point. It measures entropic energy dumped. **The same compute
directed at a task whose artifact carries structure produces a rule
that survives reduction.** The distinction is not effort and it is not
sincerity — it is whether anything is left after `Π_G(Δ(Ω))`.

**THE LANE LAW.** A traversal lane is open exactly when the holonomy
around it is non-trivial. Weight alone commutes, contributes a
per-node scalar, reduces to zero and leaves `D` unchanged; a lane held
open by weight alone therefore collapses. A lane held open by structure
no frame explains stays extended and traversable. `capacity_law`
(`lib.rs:253`) already refuses a mint without standing on precisely
this reading — the lane extends it from the mint to the bond.

    a lane held open by weight collapses
    a lane held open by structure stays traversable

**MEASURED, and favourable.** There is no `nonce`, no `hashrate` and no
`hashcash` anywhere in `engine/`. `Difficulty` exists but means the
PUZZLE's shape — `rows`, `cols`, `word_len`, `train_pairs`, `tier` —
how hard the task is, never how many leading zeros. **The tree has
never taken the hashcash shape**, so this ruling defends a position
rather than reversing one.

**WHERE THE TEMPTATION ACTUALLY SITS, and it is one specific gap.**
`46_KERNEL` §5 records the single missing miner piece: nothing makes
weight costly to fake — the Sybil and ordering anchor
(`KERNEL_RELAY` §7.1). Hashcash is the reflexive fix for exactly that
gap, and it is how a system that never wanted a nonce acquires one.
**This ruling forecloses it in advance:** the anchor is to be paid in
structure that survives reduction, not in energy spent.

**NOT DERIVED — three open things, flagged rather than filled:**

1. **What traverses.** Value, state, or judgment are three different
   laws with three different conservation obligations, and the lane
   does not pick one. An operator ruling.
2. ~~**Whether lanes decay.**~~ **STRUCK — see §2k.1.** This item
   inferred from non-propagating ECSK torsion that lanes might fade
   when the work stops. That is not an open question: ARTICLE IV
   forbids it (rank never retreats), and under the reversed arrow
   torsion propagates at the chain's own step rate, which Article IV
   clause 2 already fixes. The item was a rule that would have broken
   the constitution, and it is withdrawn rather than left open.
3. **THE ANCHOR IS STILL OPEN.** "Must raise `D`" does not by itself
   make identity costly: a brute-forced solution still raises rank, and
   nothing here prices a second identity. This section rules out
   hashcash and supplies the criterion the replacement must meet. **It
   does not supply the replacement.** Saying otherwise would let the
   shape of the argument stand in for a mechanism that does not exist.

### 2k.1 THE ARROW REVERSES — torsion IS spin, and §2k open item 2 is STRUCK (operator, 2026-08-02)

§2k wrote the arrow as ECSK usually writes it: **spin SOURCES torsion**,
torsion appearing as a response where spin-carrying matter sits. The
operator's correction reverses it: **twist the torsion at the fine
layer and you have the spin.** Not a source relation — an identity
across scales.

**What is conceded and what is not.** GR is a four-dimensional
pseudo-Riemannian theory, not a two-dimensional sheet; the rubber sheet
is a teaching picture. But the picture does hide precisely the degree
of freedom at issue — it shows a vector ROTATING around a loop
(curvature) and cannot show a loop FAILING TO CLOSE (torsion) — and the
torsion-free condition genuinely is a CHOSEN condition on the
connection, not a derived one. **More usefully: this engine does not
have to settle the physics.** It has to choose its connection, and the
choice has consequences that can be named. Three of them, and the third
corrects a defect.

**1. STRUCTURE CANNOT BE ASSERTED.** Under *sourced-by*, "useful
structure" is a property that work CARRIES — and a property can be
claimed, certified, and therefore forged. That smuggles the trust
problem back in under a new name. Under *identity*, there is no
separate substance: the structure IS the measured failure to close.
Computed or absent, with nothing in between and nothing to attest.

**2. THE RESIDUAL IS THE TORSION, AND IT IS ALREADY LANDED.**
Geometric torsion is the failure of an infinitesimal parallelogram to
close. `bridge.rs:104` computes

    R_ijk,l = Δ_ik,l − Δ_ij,l − Δ_jk,l

— go `i → k` directly, go `i → j → k` the long way, compare. **The gap
is the closure failure.** That is not an analogy to torsion; it is the
same construction. So the two scales are one object:

    FINE    one residual on one triple, one channel     — the SPIN
    COARSE  rank D: how many independent cycles carry
            holonomy no frame explains                  — the TORSION

The lane's traversability and the mint's standing therefore read the
same quantity at two resolutions. One measurement, two uses — which is
why `26_ARTICLES` is the correct home for both and why no second
"usefulness" metric is needed.

**3. TORSION PROPAGATES — and §2k's open item 2 was not an open
question but a rule that would have BROKEN ARTICLE IV.** §2k flagged
that in ECSK torsion is non-propagating (algebraically slaved to spin,
vanishing in vacuum) and inferred that a lane's torsion might fade when
the work stops. **ARTICLE IV forbids exactly that:** rank never
retreats, monotone non-decreasing. A lane that faded would be capacity
retreating.

Under the reversed arrow torsion is not a response to a source; it is
the fabric's own degree of freedom, and it propagates. **Article IV
clause 2 already fixes its rate** — *the propagation limit of this
manifold is one commitment per step … the rate is `c` and `c` is the
clock.* The constitution has assumed the operator's reading since it
was written; §2k contradicted it. **Open item 2 is STRUCK, not
resolved.** A committed non-closure is a permanent feature of the
complex.

**AND THE NONCE BAR GETS STRONGER, not weaker.** A hash race has
COMMUTING increments: every attempt independent, order irrelevant, the
loop always closes. Non-closure is zero by construction. So no rule is
needed saying nonces do not count — **there is nothing there to
measure.** The bar is an absence rather than a policy, which is the
strongest form it can take: a policy can be argued with, and no one can
lobby for a residual that does not exist.

**WHERE THE 2D OBJECTION LANDS — on this engine, measured.**
`delta.rs:16` declares `Delta { dim, fiber, entries: Vec<Vec<Vec<Ratio>>> }`
— entries indexed by PAIRS, and residuals range over TRIPLES. **The
complex tops out at 2-simplices.** The engine's own structure is flat
in exactly the way the objection describes, and it is the SAME WALL as
`46_KERNEL` §5a.1's second wall: orbs project to vertices, so there are
no higher cells for the twist to live in. Three separate lines — the
Venn overlap, the traversal lane, and the depth of the torsion — all
arrive at one wall. Giving orbs extent is what opens room above the
triangle.

**NOT DERIVED, flagged:** what a 3-cell's non-closure would MEASURE.
Triangles carry composition failure; what the next degree carries is
not obvious and must not be guessed, because a cell whose residual has
no meaning is apparatus rather than law.

### 2m. THE PATH POLYNOMIAL — ordered interlink, causal reach ±1 (operator, 2026-08-02)

> imagine `i j k l` each have a vertical axis that wraps around and
> could interlink with any of the other `i j k l` in any particular
> order to form a polynomial that links back one step to a prior or a
> future polynomial

**Named precisely, this is a PATH ALGEBRA over the complex, graded by
step, with causal reach ±1.** Paths compose head-to-tail; the algebra is
non-commutative, which is what makes "in any particular order" carry
data rather than decorate it; degree is the number of interlinks;
coefficients are what each link carries; and evaluation along a path is
holonomy. Not an exotic object — a known one, and the right one.

(*Terminology:* "master equation" is the operator's shorthand here for
the residual/rank relation. Task #17 stands — there is no single master
equation, and this section describes a CARRIER, not a new one.)

Four findings against the tree. The first is the sharpest, because it
is not a feature request.

**FINDING 1 — THE FIBER IS NON-ABELIAN AND THE HOLONOMY IS NOT. This
split already exists in the tree.**

    reduce.rs:113   pub struct Holonomy { i, j, l, value: Ratio }
    fiber.rs        MatrixFiber2x2 — matmul, commutator, [A,B] ≠ 0

A `Ratio` **cannot carry order.** Any permutation of the same links
sums to the same rational, so under the landed reduction every ordering
of an interlink is the same holonomy, and "in any particular order" is
not merely unbuilt — it is destroyed by the type. Meanwhile the whole
non-abelian apparatus sits beside it in `fiber.rs`, unused by the
reduction.

**So the ordered interlink is not a new requirement. It is closing a
split the tree already carries:** `Holonomy.value` must become a
`MatrixFiber2x2`. That is a concrete kernel amendment with a VERSION
bump, and it is the first thing this section demands.

**FINDING 2 — THE PATH EVALUATOR IS LANDED AND NOTHING BUILDS A PATH.**
`fiber.rs:156` `loop_holonomy(walk: &[MatrixFiber2x2])` folds an ordered
walk by `matmul` — exactly the polynomial's evaluation — and has no
non-test caller. **The polynomial IS the walk.** The engine can evaluate
one and cannot express one. This is the third organ found landed and
unwired, after `orbit_lock` (§K3) and the fiber itself.

**FINDING 3 — THE VERTICAL AXIS EXISTS AT THE COMMITMENT LEVEL, NOT THE
INDEX LEVEL.** `arbor::Coordinate` carries `branch`, the wrapped torus
triple `s: [Ratio; 3]`, the gauge sheet `g`, and the tick `t` — a
wrapped vertical axis, landed. But it wraps THE WHOLE STATE. The
indices `i, j, k` are bare `usize` positions in `Committed::orbs`, and
a bare index has no axis to wrap.

Giving each index its own wrapped vertical axis is the same ruling as
"orbs have extent" (`46_KERNEL` §5a.1, wall 2). **That is now a fourth
line arriving at one wall** — the Venn overlap, the traversal lane, the
depth of the torsion, and now the per-index axis. The station (tag 89)
is the vertical axis that landed, but it is seated per LINE, not per
index.

**FINDING 4 — ±1 IS ALREADY THE LAW, AND IT IS THE LIGHT CONE.** ARTICLE
IV clause 2: *the propagation limit of this manifold is one commitment
per step … the rate is `c` and `c` is the clock.* A polynomial reaching
exactly one step in either direction is not a restriction being
imposed — it is the propagation limit already ruled. `fold.rs:215`
returns `OrbitWitness { operator_id, step: 1 }`, so the single-step
reach is already in the witness type.

**THE FORWARD LINK — the one part that needs care, and it resolves
with ordinary arithmetic.**

Backward is a REFERENCE: hash the parent, landed as
`Commitment.parent`. Forward cannot be a reference — **you cannot hash
what does not exist yet.** But it can be a CONSTRAINT: an obligation
binding the next commitment without naming it.

The organ is landed. `CHANNEL_FORWARD` carries the undelivered
remainder of a standing delivery obligation (`quota − delivered`,
floored at zero). **An outstanding obligation is a forward link** — it
constrains a future state it cannot identify.

**LAYER WARNING — see §2m.2.** An earlier draft of this passage wrote
that the covenant *is* the forward link. It is not: the forward link is
a kernel primitive and the covenant is one game-level instance of it,
sitting inside the sector. The correction stands; the pairing below is
between MECHANISMS, and the covenant is cited as evidence the shape is
implementable rather than as its definition.

    backward   REFERENCE   Commitment.parent          landed
    forward    CONSTRAINT  outstanding covenant       landed

The polynomial is anchored at both ends by organs that already exist,
and nothing here requires a record to point at the future.

**A POSSIBLE ROUTE THROUGH K3 — flagged for ruling, NOT derived.** If
forward links are constraints, then two branches carrying the SAME
outstanding covenant are bound to one another without any merge record:
the binding lives in the constraint layer rather than the parent layer.
That is close to the chart-agreement condition, since charts are
required to agree precisely on their overlaps.

**What it does not do, stated plainly:** it binds two branches without
MERGING them. No single record results, so it does not by itself
discharge K3's wall 1 or produce the join record §5a.1 specifies —
and a shared obligation is weaker than a shared coordinate, since
nothing forces the two to reconcile what they each hold. It may be the
better primitive or it may be an adjacent one. **An operator ruling,
recorded because it is cheap to state and would be expensive to
discover late.**

