# 22 — THE CONTAINMENT SECTOR — C, its bound, and the path algebra

**Status:** LAW, split out of `18_TORSION.md` on 2026-08-02 by operator
ruling. It carries the `C` definition thread and the containment law.

**Why it moved.** `18_TORSION` is the physics subset that constrains the
computational mathematics. It had grown to 2,020 lines and twenty §2
subsections, ten of them added in one day. Measured: of those twenty,
CODE cites only §2a, §2b, §2d, §2e, §2f and §2h. Every section moved
here was cited by no code at all.

**Splitting is not striking.** Nothing here is withdrawn; it is filed
where it belongs so the physics subset stays a physics subset.

---

**Companion:** `17_CAPACITY.md` §10bis–§10ter defines the cage from the
other side (the exile of C, the door, C consulted, the access/authority
trade). **Two documents describing one subject is a second readable
truth (CHARTER §11 rule 2), and reconciling them is owed.**

---

### 2m.1 THE PATH ALGEBRA IS A CONTAINMENT SECTOR (operator, 2026-08-02)

§2m left one question open — whether the path algebra is FREE (every
ordering admissible, unbounded) or carries RELATIONS (some orderings
identified, finite-dimensional). **The question dissolves, because the
bound is a DOOR and not a relation, and the door is already landed.**

**THE SECTOR IS AN EXISTING PATTERN IN THIS TREE, not a new idea.**
`rfa_core::cage` — `consult(input, f).withdraw(exact_rule)`:

    INSIDE    any tool, any impurity — floats, heuristics, enumerative
              sweeps, parallelism, GPU. `search.rs:184` calls `solve`
              *"the dirtiest work this system does … no claim to
              purity"* and runs it here deliberately.
    THE DOOR  `withdraw(exact)` — the OUTSIDE re-derives the proposal
              exactly and refuses it otherwise. A wrong proposal costs
              nothing and simply never leaves.
    THE LAW   **Approximation may PROPOSE; only exactness may DISPOSE.**
    ENFORCED  by the TYPE SYSTEM, not by discipline. `Proposal<f64>`
              does not exist, because `f64` carries no `Exact` impl —
              *"the boundary is the type system, not care"*
              (`cage_pins.rs:112`).

**Applied to the path algebra:** inside the sector it is FREE. Every
ordering admissible, exploration unbounded, no ceiling — which is what
ARTICLE IV requires and what relations would have quietly violated.
At the door, only a path whose holonomy re-derives exactly crosses into
the committed complex.

    free where it costs nothing        (inside)
    verified where it counts           (crossing)

So the committed structure stays finite without any ceiling being
imposed on the exploration. **The open question in §2m was open because
it looked for the bound in the wrong layer** — it asked what the
algebra forbids, when the answer is that the algebra forbids nothing
and the door admits little.

**AND THE DOOR IS THE LAYER BOUNDARY.** This is the second half of the
operator's ruling, and it corrects §2m's own error — see §2m.2.

### 2n. `C` DEFINED — and the collision audit (2026-08-02)

**In THIS context — the torsion law, the path polynomial, the sector —
`C` means `C(Δ)`, the coherence functional, and it is a DEMOTED
HUMAN-LENS SCALAR with no operational function.**

    delta.rs:190   pub fn coherence_functional(delta: &Delta) -> Lens {
                       let mut c = Ratio::zero();
                       for_each_residual(delta, |_,_,_,_, r| c += r.abs());
                       Lens::of(c)
                   }

Read off the code: **`C(Δ) = Σ|R|`** — the L1 sum of the magnitudes of
every residual, over exact rationals, returned wrapped in `Lens`. The
`Lens` wrapper is the same containment discipline as §2m.1's sector: the
type marks it as a reading, not a rule.

`rfa-core/src/lib.rs:16` states the ruling plainly: ***C HAS NO
OPERATIONAL FUNCTION** — it is not a score, not a gauge, not a gate, and
no engine path consults it.* `C(Δ) = 0` says the cocycle condition
holds and **nothing more**; "converged" is struck from the crate.

**WHY IT IS DEMOTED, and the reason is this document's own law.**
§2k.1 established that the residual IS the torsion — the closure
failure, carried with its indices and its sign. `bridge.rs:104` states
the consequence: *"Summing them destroys them."* `C` is exactly that
sum. An L1 aggregate discards order, discards sign, and discards the
triple that produced each term, so **`C` cannot see torsion in
principle** — the one quantity the engine now organises around is
precisely the one `C` collapses. Its demotion is not a preference; it
is a consequence, and it is why `D` replaced it as the organising
quantity.

**FIVE DISTINCT `C`s ARE LIVE. Measured:**

| symbol | meaning | where | status |
|---|---|---|---|
| `C(Δ)` | coherence functional, `Σ\|R\|` | `delta.rs:190` | LENS ONLY, no operational role |
| `c` | the propagation limit — one commitment per step | ARTICLE IV cl. 2 | LAW. Lowercase, a RATE not a quantity |
| `C4`, `C5`, `C7` | algorithm labels in the equations ledger | `EQUATIONS` §C5 | a NAMESPACE PREFIX, unrelated to `C(Δ)` |
| "C in its own 2×2" | the operator's containment ruling | operator, 2026-08-01 | see below |
| `c_is_consulted…` | a cage test name | `cage_pins.rs` | probable STALE reference to `C(Δ)` |

**On "C is contained to its own 2×2 matrix"** (operator, 2026-08-01):
read against what landed, this is satisfied — the curvature is computed
as `plaquette_curvature(a, b) = a.commutator(b)` returning a
`MatrixFiber2x2` (`fiber.rs:168`), so the quantity lives in the fiber
rather than as a global scalar. **Flagged, not asserted:** whether the
ruling meant the coherence `C(Δ)` or the curvature. They are different
objects that share a letter, and the code letters curvature `F` by
convention while naming the functional `coherence_functional`.

**DEFECT — THREE DOCUMENTS CONTRADICT THE CODE AND A STANDING LAW.**
The code sums ABSOLUTE VALUES. These say squares:

    N4_DIRECTIVES:25    "C(Δ) = Σ R²"
    N6_DIRECTIVES:87    "C(Δ) = 6·ΣR²"
    KERNEL_RELAY:296    "the coherence functional C(Δ) = Σ R²"

Squares are **barred as aggregators of difference** by THE PROPORTIONAL
LAW (`rfa-core/src/lib.rs:14`, §2b), so these three do not merely
disagree with `delta.rs:193` — they state a formula the constitution
forbids. Correction notes placed at each site; the code is correct and
is the authority.

This is the second live notation collision this document carries, after
§2j's two different `D`s. **Both are the same failure mode: a letter
outliving the quantity it was minted for.** `C` is now demoted, `D` is
now the organising quantity, and the documents still carry the old
letters in the old roles.

### 2n.1 WHERE `C` LIVES — the fiber as workshop, and the one correction (operator, 2026-08-02)

> C needs to be contained in the 2×2 matrix fibre bundle defined where
> "anything goes" — that's the place where we put approximations,
> floats, all the things we don't want utilised in a deterministic
> system

**The quarantine is right and `C` is the correct resident. One
distinction has to be drawn or it breaks consensus.**

**MEASURED FIRST.** `MatrixFiber2x2` is `[[Ratio; 2]; 2]` and its own
header says *"Exact rational throughout — no float, no root"*
(`fiber.rs:25`). It is not a quarantine zone today; it is load-bearing
committed algebra. `loop_holonomy` and `plaquette_curvature` compute
over it, and **K5 (#32) proposes to carry it INTO `Holonomy.value`** —
that is, into the committed reduction that feeds rank `D`, which feeds
`capacity_law`, which gates minting.

**So if the fiber's CONTENTS became float, two honest nodes would
compute different `D` from identical state and consensus would fail.**
That is not a stylistic objection; it is a hard break, and it is
precisely the class of thing the cage exists to prevent.

**THE DISTINCTION THAT MAKES THE RULING WORK — two roles, one object:**

    THE FIBER AS A COMMITTED TYPE
      [[Ratio; 2]; 2], exact, deterministic, reproducible by every
      node. Non-negotiable, because D gates the mint.

    THE FIBER AS A WORKSHOP — "anything goes"
      where its elements are FOUND. Float BLAS, heuristics, iterative
      solvers, GPU, parallel search. Exactly where floats belong,
      because matrix search is the expensive part and none of it is
      authoritative.

    THE DOOR BETWEEN THEM
      the cage. **The prover may be approximate; the verifier must be
      exact.**

**AND THAT PATTERN IS ALREADY LANDED, in the right place.** The fold
gate is a prover/verifier split today: `orbit_lock` searches for the
conjugator (*"the prover finds the conjugator"*, `fold_pins.rs:170`)
and `verify_orbit_witness` *"re-walks the one path it claims"*
(`fold.rs:224`). The operator's structure has a working precedent
inside the exact organ it applies to.

**WHAT THIS GIVES `C` — a job it did not have.** §2n established that
`C` gates nothing and cannot see torsion in principle. Placed inside
the sector, it acquires the one role it is actually fit for: **a search
heuristic.** `KERNEL_RELAY` §5.2 struck gradient descent on `C` as a
CONVERGENCE METHOD, and that strike stands — but a heuristic guiding a
prover produces nothing authoritative, so the objection does not reach
it. The sector is what makes the distinction expressible:

    C is BARRED AS A LAW and PERMITTED AS A HINT.

That is a stronger position than "C is vestigial". It is not vestigial;
it is confined, and confinement is what makes it safe to use.

**ONE GAP MEASURED — the fiber cannot currently cross the door.**
`cage.rs:248` lists every `Exact` impl: `Ratio`, `i64`, `u64`, `usize`,
`bool`, `Vec<T>`, `Option<T>`, and `SolveRead`. **`MatrixFiber2x2` has
no `Exact` impl**, so `Proposal<MatrixFiber2x2>` does not exist and a
fiber element found by an approximate search has no lawful way out of
the sector. It plainly qualifies — it is built of `Ratio`s — so this is
a one-line addition, and `cage_arrow.rs:118` audits every `Exact for`
impl, which makes granting one a deliberate witnessed act rather than a
slip. **Correctly guarded; simply not yet done.**

**CONSEQUENCE FOR K5 (#32), recorded so the two rulings do not
collide.** K5 stands: `Holonomy.value` becomes a `MatrixFiber2x2` and
that value is EXACT. What this section adds is that the SEARCH which
finds fiber elements belongs in the sector. Same object, two roles,
separated by the door — and K5 must not be read as importing the
anything-goes region into committed state.

> ⚠ **THE "WORKSHOP" CONSTRUCTION ABOVE IS WITHDRAWN — see §2n.2.** It
> sited the sector at the EDGE level, giving every fiber in the complex
> an approximate twin. A region that covers every edge is not a
> containment sector; it is a shadow copy of the whole algebra. The
> two-roles distinction (exact committed value, approximate search)
> survives and is correct. Its PLACEMENT does not.

### 2n.2 THE SECTOR IS ONE MATRIX, NOT EVERY MATRIX (operator correction, 2026-08-02)

> that means your "matrix" is at the wrong level — and the contained
> sector has become the entire sector

**Both claims land. §2n.1 sited the quarantine wrongly, and the
specific error is a conflation of two different 2×2s.**

    THE CONNECTION FIBER          MatrixFiber2x2, fiber.rs
      one PER EDGE, exact, committed, load-bearing. Carries holonomy,
      feeds rank D, gates the mint. Its level is the EDGE and that is
      correct — a connection lives on an edge and belongs nowhere else.

    C's CONTAINMENT MATRIX        the operator's 2026-08-01 ruling
      ONE, singular, for the whole reading. Approximate, quarantined,
      diagnostic, gating nothing.

**They are both 2×2. That is a coincidence of ARITY, not an identity**,
and §2n.1 treated it as one — which is how the sector escaped its
bounds. If C's residence is the per-edge fiber, then the quarantine is
wherever fibers are, which is everywhere, and a quarantine that covers
everything contains nothing.

**This is the same failure this document diagnosed one section
earlier.** §2n named it for letters: *a letter outliving the quantity
it was minted for*. Here it is a SHAPE — `2×2` read as an identity
rather than as a dimension two unrelated objects happen to share.

**RE-READ OF THE ORIGINAL RULING.** The operator wrote *"C is contained
to **its own** 2×2 matrix"* (2026-08-01). *Its own* is the operative
phrase and §2n.1 read straight past it. C does not move INTO the
connection fiber; **C gets a separate, single 2×2 of its own.** That is
already a bounded containment sector by construction:

    ONE object, not one per edge — so the region cannot grow with the
    complex, and it is auditable by inspection.
    ONE door, because there is one object to leave.
    NOTHING load-bearing inside, because C gates nothing (§2n).

**WHAT CHANGES AND WHAT DOES NOT:**

    UNCHANGED   MatrixFiber2x2 stays per-edge, exact, committed.
                K5 (#32) is unaffected — it was never about C.
    UNCHANGED   the two-roles distinction: an exact committed value,
                an approximate search that proposes it. Correct, and
                it belongs to the CAGE as a door primitive.
    CORRECTED   the sector is not "every fiber's workshop". It is one
                bounded region. C's own 2×2 is one such region; the
                solver's `consult`/`withdraw` search is another. Every
                lawful sector is BOUNDED — that is what makes the word
                mean anything.

**NOT DERIVED, flagged rather than furnished:** what C's four slots
carry. `C(Δ)` is a scalar today (`Σ|R|`), so a 2×2 residence has four
positions and three of them are unassigned. Inventing meanings for
them to justify the shape would be the §2d failure exactly — apparatus
built to fit a container. **The arity is the operator's; the contents
are an open ruling.** What IS settled is the containment: one object,
one door, nothing authoritative inside.

**RESOLVED — see §2n.3.** The contents were ruled the same day; the
"not derived" note above records only that they were not invented here.

### 2n.3 THE CONTAINMENT LAW — `C_bound` inscribed (operator ruling, 2026-08-02)

§2n.2 established the separation. This inscribes it as law, with the
slot allocation ruled and three corrections made in translation.

**THE TWO STRUCTURES, held apart by law:**

    CONNECTION FIBER    M_e for every edge e ∈ E
      exact rationals · committed · load-bearing · carries holonomy
      ∮M · feeds rank D · gates the mint · CARRIES NO CONTAINMENT
      APPARATUS

    CONTAINMENT MATRIX  C_bound — exactly ONE per reading
      quarantined · gates nothing · footprint independent of |E| and
      |V| · isolated from every connection fiber

**THE SLOT ALLOCATION — ruled, and deliberately un-padded:**

    C_bound = ⎡ C(Δ)        δ_search  ⎤
              ⎣ λ_proposed  0         ⎦

    (0,0)  C(Δ)         the coherence functional, Σ|R| — the lens
                        scalar of §2n
    (0,1)  δ_search     the consult/withdraw search-state delta: the
                        approximate proposed perturbation
    (1,0)  λ_proposed   the un-admitted error of the proposed step
    (1,1)  0            EXPLICIT GROUND. Not a slot awaiting meaning —
                        a declared zero, so no speculative state can
                        accrete in an empty position.

The explicit zero is what satisfies LAW 8 (§3, line 1350 — *a term that
computes is kept; a term that only resonates is excised*). An
unassigned fourth slot is an invitation to invent apparatus to fill it;
a declared ground is a refusal to.

**CORRECTION 1 — `C(Δ)` IS NOT A CAPACITY READING.** The relayed ruling
named slot (0,0) *"the primary scalar capacity reading."* Two faults:
`capacity` is STRUCK vocabulary, replaced by `rank` and carried on the
currency pass (#14); and the reading is wrong on the merits — capacity
is `D`, and §2n established that `C` **cannot see** what `D` measures,
because summing the residuals destroys the torsion they carry. Slot
(0,0) holds the coherence functional and is named as such. Letting it
be called a capacity reading would re-admit through a slot label the
exact conflation `D` was minted to end.

**CORRECTION 2 — `M_e ∈ SL₂` IS NOT AN INVARIANT AND MUST NOT BECOME
ONE.** The relay writes the connection fiber as `SL₂(ℚ)`. The landed
type is deliberately weaker (`fiber.rs:31`):

> *Members of `SL₂` satisfy `det = 1`; the type does not enforce it,
> because the gauge law is a READING (see `is_sl2`) and a refusal names
> what it found rather than being unconstructible.*

`is_sl2()` returns `bool`. **If membership became a type invariant,
`is_sl2` could never return false and the refusal it exists to raise
would be unreachable.** The fiber is `M₂ₓ₂(ℚ)` with `det = 1` as a
JUDGED READING. Recorded because an invariant is cheap to write into a
document and expensive to discover later in a type.

**CORRECTION 3 — `λ_residual` RENAMED TO `λ_proposed`. Third notation
collision.** `Residual { i, j, k, l, value }` is a committed object
(`bridge.rs:104`), and §2k.1 established it IS the torsion — the
closure failure carried with its indices. The relay used "residual" for
*the error of an unadmitted proposal*, which is a different thing
living on the other side of the door. After §2j's two `D`s and §2n's
five `C`s, this document does not need a third collision, and the
proposal-side name is the one that must yield.

**CLARIFICATION — `C_bound` IS A REGISTER, NOT A GROUP ELEMENT.** No
`matmul`, no `commutator`, no holonomy is taken over it; it is four
slots written in a square. **NO ALGEBRA MAY BE PERFORMED BETWEEN
`C_bound` AND ANY CONNECTION FIBER.** That prohibition is what
mechanically enforces "isolated from the edge fiber" — without it,
sharing the `2×2` shape is a standing invitation to compute
`C_bound.commutator(M_e)` and breach the isolation through the type
system rather than through intent. The shared arity is the hazard
§2n.2 named; this is the guard against it.

**ON EXACTNESS vs AUTHORITY — an apparent contradiction, resolved.**
`C_bound` is over exact rationals, yet it is the residence of
approximation. These do not conflict, and the landed cage shows why:
floats live inside the CONSULTED CLOSURE (`cage_pins.rs` runs
`(n as f64).sqrt()` there deliberately), and what the closure returns
is exact-typed and still unverified. **Exactness is a representation
property; authority is a door property.** `C_bound` holds exact numbers
that nothing has yet admitted.

**THE GENERAL INVARIANT — boundedness, carried per-axis (operator
correction, 2026-08-02).**

An earlier draft wrote this as `O(1)`. **That is a scalar collapse of a
multi-axis quantity, in a document whose whole thesis is that scalar
collapse destroys structure** — the same error as `C(Δ) = Σ|R|`, made
about complexity instead of about residuals. A single `O(·)` cannot say
WHICH axis a sector grows in, and which axis it grows in is the entire
question.

**Cost is carried as a TUPLE over the axes it actually scales in, never
as one symbol:**

    O( |V|, |E|, F, δ, τ )

    |V|  orbs — the vertices of the complex
    |E|  relations — its edges
    F    fiber arity (CHANNELS = 3 today)
    δ    depth — path length, search budget, walk degree
    τ    steps — ticks over which the footprint persists

**TWO DIFFERENT QUANTITIES WERE BEING NAMED BY ONE SYMBOL** (operator,
2026-08-02). The collapse was not `O(1)` being wrong; it was `O(1)`
answering a question about STORAGE while being read as a bound on
EXECUTION:

    STATE FOOTPRINT — the container
      C_bound's storage. Must not shadow the topology:
                          ∂Memory / ∂|E|  =  0
      O(1) in |V|, |E| and F. This is what stops the quarantine
      inflating as the network grows.

    EXECUTION TRAJECTORY — the work done inside it
      **O(n^D)** — the VOLUME of the space the work ranges over: side
      `n` along `D` independent axes. **Locking this to O(1) flattens
      the dynamics entirely** — a search that may not scale with depth
      is not a search. This is where the dimensionality actually lives,
      and it must stay free.

**`O(n^D)`, not a flat budget** (operator ruling 2026-08-02, replacing
an earlier `O(x)` reading that named a single axis and so collapsed the
others). A constant work budget rations independently of how much
structure stands — the same flattening as `C(Δ) = Σ|R|`, applied to
cost. `D` independent axes of extent `n` enclose `n^D`, and **the
envelope BREATHES WITH `D`**: rank is earned rather than granted and
ARTICLE IV holds it monotone, so the envelope widens as the system
gains dimension and never narrows. That is the re-volumizer — the
budget is a function of standing rank, not a number chosen in advance.

`n^0 = 1`: no rank, no room — exactly the standing `capacity_law`
refuses to mint on, so the envelope agrees with the constitution rather
than contradicting it. Landed and certified as `kernel::motion::envelope`,
which refuses on overflow rather than wrapping (an envelope that wrapped
would report an enormous space as a tiny one).

**So the lawfulness test is per-axis, and it is a topology test rather
than a size test:** a sector is lawful iff its STORAGE is O(1) in the
axes it OBSERVES (`|V|`, `|E|`, `F`), while its TRAJECTORY may scale
freely along its OWN axes (δ, τ), bounded by declared budget rather
than by law.

The solver's sector sweeps `for _depth in 0..=max_depth` — O(δ) in
work, O(1) in the graph: lawful, and its high-dimensional search is
preserved rather than flattened. `C_bound` is O(1) in storage on every
axis. §2n.1's withdrawn construction was O(|E|) in STORAGE, which is
exactly the violation. **A single symbol could not have told the three
apart** — which is the whole argument for carrying the tuple.

The two clauses that follow are exact as relayed:

1. **NO FIBER SHADOWING.** No quarantine, solver or search may
   duplicate a per-edge structure or mirror the topology. This is the
   clause §2n.1 violated.
2. **SINGLE POINT OF EGRESS.** One bounded region, one door. `C_bound`
   may be inspected or discarded whole without invalidating any
   connection fiber — which is the operational test that the isolation
   holds.

**STATUS.** `d150497` stands as written. `K5` (#32) is unaffected and
clean — it was never about `C`. The per-edge fibers remain pure.

