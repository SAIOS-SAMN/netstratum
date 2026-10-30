# 22 — The containment sector — C, its bound, and the path algebra

**Status:** law, split out of the retired `18_TORSION.md` on 2026-08-02 by operator
ruling. It carries the `C` definition thread and the containment law.

**Why it moved.** `18_TORSION` was the physics subset that constrained the
computational mathematics. It had grown to 2,020 lines and twenty §2
subsections, ten of them added in one day. Measured: of those twenty,
code cites only §2a, §2b, §2d, §2e, §2f and §2h. Every section moved
here was cited by no code at all.

**Splitting is not striking.** Nothing here is withdrawn; it is filed
where it belongs so the physics subset stays a physics subset.

---

**Folded in 2026-08-13 (docs audit).** `17_CAPACITY.md` §10bis–§10ter
— "the exile of C, the door, C consulted, the access/authority trade" —
is now §10bis–§10ter below, in this document. It was flagged as a
companion piece the day this document split out (*"Two documents
describing one subject … reconciling them is owed"*, struck above) and
was never reconciled; this fold is that reconciliation. Section numbers
are preserved exactly, so "17_CAPACITY §10bis" and "17_CAPACITY §10ter"
still resolve — now to this file.

**The duplicate line that anchors the fold.** Both documents independently
stated *"approximation may propose; only exactness may dispose"* — this
document at §2m.1 (line 41, the path-algebra door) and the incoming
§10bis/§10ter (the C-cage door). It is the same law derived twice from
two different sectors (the path algebra, and C's own cage). The
duplication is not stripped on the fold: §2m.1 derives it for paths,
§10ter.a derives it for C, and each derivation is load-bearing for its
own sector. What follows is otherwise new material — `cage.rs`'s three
seals, the cage-as-workshop, C consulted, and the access/authority
trade — none of which the §2n/§2n.3 `C_bound` material below already
carried; §2n.3 is about `C_bound`'s slot allocation and isolation from
the connection fiber, while §10bis/§10ter is about the cage mechanism
(`Lens`, no ordering, the door) and C's role as the sector's own
consultant/defender. Read together, not redundantly.

---

### 2m.1 The path algebra is A containment sector (operator, 2026-08-02)

§2m left one question open — whether the path algebra is free (every
ordering admissible, unbounded) or carries relations (some orderings
identified, finite-dimensional). **The question dissolves, because the
bound is a door and not a relation, and the door is already landed.**

**The sector is an existing pattern in this tree, not a new idea.**
`rfa_core::cage` — `consult(input, f).withdraw(exact_rule)`:

    Inside    any tool, any impurity — floats, heuristics, enumerative
              sweeps, parallelism, GPU. `search.rs:184` calls `solve`
              *"the dirtiest work this system does … no claim to
              purity"* and runs it here deliberately.
    The door  `withdraw(exact)` — the outside re-derives the proposal
              exactly and refuses it otherwise. A wrong proposal costs
              nothing and simply never leaves.
    The law   **Approximation may propose; only exactness may dispose.**
    enforced  by the type system, not by discipline. `Proposal<f64>`
              does not exist, because `f64` carries no `Exact` impl —
              *"the boundary is the type system, not care"*
              (`cage_pins.rs:112`).

**Applied to the path algebra:** inside the sector it is free. Every
ordering admissible, exploration unbounded, no ceiling — which is what
article IV requires and what relations would have quietly violated.
At the door, only a path whose holonomy re-derives exactly crosses into
the committed complex.

    free where it costs nothing        (inside)
    verified where it counts           (crossing)

So the committed structure stays finite without any ceiling being
imposed on the exploration. **The open question in §2m was open because
it looked for the bound in the wrong layer** — it asked what the
algebra forbids, when the answer is that the algebra forbids nothing
and the door admits little.

**And the door is the layer boundary.** This is the second half of the
operator's ruling, and it corrects §2m's own error — see §2m.2.

### 2n. `C` defined — and the collision audit (2026-08-02)

**In this context — the torsion law, the path polynomial, the sector —
`C` means `C(Δ)`, the coherence functional, and it is a demoted
human-lens scalar with no operational function.**

    delta.rs:190   pub fn coherence_functional(delta: &Delta) -> Lens {
                       let mut c = Ratio::zero();
                       for_each_residual(delta, |_,_,_,_, r| c += r.abs());
                       Lens::of(c)
                   }

Read off the code: **`C(Δ) = Σ|R|`** — the L1 sum of the magnitudes of
every residual, over exact rationals, returned wrapped in `Lens`. The
`Lens` wrapper is the same containment discipline as §2m.1's sector: the
type marks it as a reading, not a rule.

`rfa-core/src/lib.rs:16` states the ruling plainly: ***C has no
operational function** — it is not a score, not a gauge, not a gate, and
no engine path consults it.* `C(Δ) = 0` says the cocycle condition
holds and **nothing more**; "converged" is struck from the crate.

**Why it is demoted, and the reason is this document's own law.**
§2k.1 established that the residual is the torsion — the closure
failure, carried with its indices and its sign. `bridge.rs:104` states
the consequence: *"Summing them destroys them."* `C` is exactly that
sum. An L1 aggregate discards order, discards sign, and discards the
triple that produced each term, so **`C` cannot see torsion in
principle** — the one quantity the engine now organises around is
precisely the one `C` collapses. Its demotion is not a preference; it
is a consequence, and it is why `D` replaced it as the organising
quantity.

**Five distinct `C`s are live. Measured:**

| symbol | meaning | where | status |
|---|---|---|---|
| `C(Δ)` | coherence functional, `Σ\|R\|` | `delta.rs:190` | Lens only, no operational role |
| `c` | the propagation limit — one commitment per step | Article IV cl. 2 | Law. Lowercase, a rate not a quantity |
| `C4`, `C5`, `C7` | algorithm labels in the equations ledger | `EQUATIONS` §C5 | a namespace prefix, unrelated to `C(Δ)` |
| "C in its own 2×2" | the operator's containment ruling | operator, 2026-08-01 | see below |
| `c_is_consulted…` | a cage test name | `cage_pins.rs` | probable stale reference to `C(Δ)` |

**On "C is contained to its own 2×2 matrix"** (operator, 2026-08-01):
read against what landed, this is satisfied — the curvature is computed
as `plaquette_curvature(a, b) = a.commutator(b)` returning a
`MatrixFiber2x2` (`fiber.rs:168`), so the quantity lives in the fiber
rather than as a global scalar. **Flagged, not asserted:** whether the
ruling meant the coherence `C(Δ)` or the curvature. They are different
objects that share a letter, and the code letters curvature `F` by
convention while naming the functional `coherence_functional`.

**Defect — three documents contradict the code and A standing law.**
The code sums absolute values. These say squares:

    N4_DIRECTIVES:25    "C(Δ) = Σ R²"
    N6_DIRECTIVES:87    "C(Δ) = 6·ΣR²"
    KERNEL_RELAY:296    "the coherence functional C(Δ) = Σ R²"

Squares are **barred as aggregators of difference** by the proportional
law (`rfa-core/src/lib.rs:14`, §2b), so these three do not merely
disagree with `delta.rs:193` — they state a formula the constitution
forbids. Correction notes placed at each site; the code is correct and
is the authority.

This is the second live notation collision this document carries, after
§2j's two different `D`s. **Both are the same failure mode: a letter
outliving the quantity it was minted for.** `C` is now demoted, `D` is
now the organising quantity, and the documents still carry the old
letters in the old roles.

### 2n.1 Where `C` lives — the fiber as workshop, and the one correction (operator, 2026-08-02)

Ruling (paraphrased): C is to be contained in the 2×2 matrix fibre
bundle defined as the place where "anything goes" — the place for
approximations, floats, and everything not to be used in a
deterministic system.

**The quarantine is right and `C` is the correct resident. One
distinction has to be drawn or it breaks consensus.**

**Measured first.** `MatrixFiber2x2` is `[[Ratio; 2]; 2]` and its own
header says *"Exact rational throughout — no float, no root"*
(`fiber.rs:25`). It is not a quarantine zone today; it is load-bearing
committed algebra. `loop_holonomy` and `plaquette_curvature` compute
over it, and **K5 (#32) proposes to carry it into `Holonomy.value`** —
that is, into the committed reduction that feeds rank `D`, which feeds
`capacity_law`, which gates minting.

**So if the fiber's contents became float, two honest nodes would
compute different `D` from identical state and consensus would fail.**
That is not a stylistic objection; it is a hard break, and it is
precisely the class of thing the cage exists to prevent.

**The distinction that makes the ruling work — two roles, one object:**

    The fiber as A committed type
      [[Ratio; 2]; 2], exact, deterministic, reproducible by every
      node. Non-negotiable, because D gates the mint.

    The fiber as A workshop — "anything goes"
      where its elements are found. Float BLAS, heuristics, iterative
      solvers, GPU, parallel search. Exactly where floats belong,
      because matrix search is the expensive part and none of it is
      authoritative.

    The door between them
      the cage. **The prover may be approximate; the verifier must be
      exact.**

**And that pattern is already landed, in the right place.** The fold
gate is a prover/verifier split today: `orbit_lock` searches for the
conjugator (*"the prover finds the conjugator"*, `fold_pins.rs:170`)
and `verify_orbit_witness` *"re-walks the one path it claims"*
(`fold.rs:224`). The operator's structure has a working precedent
inside the exact organ it applies to.

**What this gives `C` — a job it did not have.** §2n established that
`C` gates nothing and cannot see torsion in principle. Placed inside
the sector, it acquires the one role it is actually fit for: **a search
heuristic.** `KERNEL_RELAY` §5.2 struck gradient descent on `C` as a
convergence method, and that strike stands — but a heuristic guiding a
prover produces nothing authoritative, so the objection does not reach
it. The sector is what makes the distinction expressible:

    C is barred as A law and permitted as A hint.

That is a stronger position than "C is vestigial". It is not vestigial;
it is confined, and confinement is what makes it safe to use.

**One gap measured — the fiber cannot currently cross the door.**
`cage.rs:248` lists every `Exact` impl: `Ratio`, `i64`, `u64`, `usize`,
`bool`, `Vec<T>`, `Option<T>`, and `SolveRead`. **`MatrixFiber2x2` has
no `Exact` impl**, so `Proposal<MatrixFiber2x2>` does not exist and a
fiber element found by an approximate search has no lawful way out of
the sector. It plainly qualifies — it is built of `Ratio`s — so this is
a one-line addition, and `cage_arrow.rs:118` audits every `Exact for`
impl, which makes granting one a deliberate witnessed act rather than a
slip. **Correctly guarded; simply not yet done.**

**Consequence for K5 (#32), recorded so the two rulings do not
collide.** K5 stands: `Holonomy.value` becomes a `MatrixFiber2x2` and
that value is exact. What this section adds is that the search which
finds fiber elements belongs in the sector. Same object, two roles,
separated by the door — and K5 must not be read as importing the
anything-goes region into committed state.

> ⚠ **the "workshop" construction above is withdrawn — see §2n.2.** It
> sited the sector at the edge level, giving every fiber in the complex
> an approximate twin. A region that covers every edge is not a
> containment sector; it is a shadow copy of the whole algebra. The
> two-roles distinction (exact committed value, approximate search)
> survives and is correct. Its placement does not.

### 2n.2 The sector is one matrix, not every matrix (operator correction, 2026-08-02)

Correction (paraphrased): the "matrix" of §2n.1 is at the wrong
level, and the contained sector has become the entire sector.

**Both claims land. §2n.1 sited the quarantine wrongly, and the
specific error is a conflation of two different 2×2s.**

    The connection fiber          MatrixFiber2x2, fiber.rs
      one per edge, exact, committed, load-bearing. Carries holonomy,
      feeds rank D, gates the mint. Its level is the edge and that is
      correct — a connection lives on an edge and belongs nowhere else.

    C's containment matrix        the operator's 2026-08-01 ruling
      one, singular, for the whole reading. Approximate, quarantined,
      diagnostic, gating nothing.

**They are both 2×2. That is a coincidence of arity, not an identity**,
and §2n.1 treated it as one — which is how the sector escaped its
bounds. If C's residence is the per-edge fiber, then the quarantine is
wherever fibers are, which is everywhere, and a quarantine that covers
everything contains nothing.

**This is the same failure this document diagnosed one section
earlier.** §2n named it for letters: *a letter outliving the quantity
it was minted for*. Here it is a shape — `2×2` read as an identity
rather than as a dimension two unrelated objects happen to share.

**RE-read of the original ruling.** The operator wrote *"C is contained
to **its own** 2×2 matrix"* (2026-08-01). *Its own* is the operative
phrase and §2n.1 read straight past it. C does not move into the
connection fiber; **C gets a separate, single 2×2 of its own.** That is
already a bounded containment sector by construction:

    One object, not one per edge — so the region cannot grow with the
    complex, and it is auditable by inspection.
    One door, because there is one object to leave.
    Nothing load-bearing inside, because C gates nothing (§2n).

**What changes and what does not:**

    Unchanged   MatrixFiber2x2 stays per-edge, exact, committed.
                K5 (#32) is unaffected — it was never about C.
    Unchanged   the two-roles distinction: an exact committed value,
                an approximate search that proposes it. Correct, and
                it belongs to the cage as a door primitive.
    Corrected   the sector is not "every fiber's workshop". It is one
                bounded region. C's own 2×2 is one such region; the
                solver's `consult`/`withdraw` search is another. Every
                lawful sector is bounded — that is what makes the word
                mean anything.

**Not derived, flagged rather than furnished:** what C's four slots
carry. `C(Δ)` is a scalar today (`Σ|R|`), so a 2×2 residence has four
positions and three of them are unassigned. Inventing meanings for
them to justify the shape would be the §2d failure exactly — apparatus
built to fit a container. **The arity is the operator's; the contents
are an open ruling.** What is settled is the containment: one object,
one door, nothing authoritative inside.

**Resolved — see §2n.3.** The contents were ruled the same day; the
"not derived" note above records only that they were not invented here.

### 2n.3 The containment law — `C_bound` inscribed (operator ruling, 2026-08-02)

§2n.2 established the separation. This inscribes it as law, with the
slot allocation ruled and three corrections made in translation.

**The two structures, held apart by law:**

    Connection fiber    M_e for every edge e ∈ E
      exact rationals · committed · load-bearing · carries holonomy
      ∮M · feeds rank D · gates the mint · carries no containment
      apparatus

    Containment matrix  C_bound — exactly one per reading
      quarantined · gates nothing · footprint independent of |E| and
      |V| · isolated from every connection fiber

**The slot allocation — ruled, and deliberately un-padded:**

    C_bound = ⎡ C(Δ)        δ_search  ⎤
              ⎣ λ_proposed  0         ⎦

    (0,0)  C(Δ)         the coherence functional, Σ|R| — the lens
                        scalar of §2n
    (0,1)  δ_search     the consult/withdraw search-state delta: the
                        approximate proposed perturbation
    (1,0)  λ_proposed   the un-admitted error of the proposed step
    (1,1)  0            explicit Ground. Not a slot awaiting meaning —
                        a declared zero, so no speculative state can
                        accrete in an empty position.

The explicit zero is what satisfies law 8 (§3, line 1350 — *a term that
computes is kept; a term that only resonates is excised*). An
unassigned fourth slot is an invitation to invent apparatus to fill it;
a declared ground is a refusal to.

**Correction 1 — `C(Δ)` is not A capacity reading.** The relayed ruling
named slot (0,0) *"the primary scalar capacity reading."* Two faults:
`capacity` is struck vocabulary, replaced by `rank` and carried on the
currency pass (#14); and the reading is wrong on the merits — capacity
is `D`, and §2n established that `C` **cannot see** what `D` measures,
because summing the residuals destroys the torsion they carry. Slot
(0,0) holds the coherence functional and is named as such. Letting it
be called a capacity reading would re-admit through a slot label the
exact conflation `D` was minted to end.

**Correction 2 — `M_e ∈ SL₂` is not an invariant and must not become
one.** The relay writes the connection fiber as `SL₂(ℚ)`. The landed
type is deliberately weaker (`fiber.rs:31`):

> *Members of `SL₂` satisfy `det = 1`; the type does not enforce it,
> because the gauge law is a reading (see `is_sl2`) and a refusal names
> what it found rather than being unconstructible.*

`is_sl2()` returns `bool`. **If membership became a type invariant,
`is_sl2` could never return false and the refusal it exists to raise
would be unreachable.** The fiber is `M₂ₓ₂(ℚ)` with `det = 1` as a
judged reading. Recorded because an invariant is cheap to write into a
document and expensive to discover later in a type.

**Correction 3 — `λ_residual` renamed to `λ_proposed`. Third notation
collision.** `Residual { i, j, k, l, value }` is a committed object
(`bridge.rs:104`), and §2k.1 established it is the torsion — the
closure failure carried with its indices. The relay used "residual" for
*the error of an unadmitted proposal*, which is a different thing
living on the other side of the door. After §2j's two `D`s and §2n's
five `C`s, this document does not need a third collision, and the
proposal-side name is the one that must yield.

**Clarification — `C_bound` is A register, not A group element.** No
`matmul`, no `commutator`, no holonomy is taken over it; it is four
slots written in a square. **No algebra may be performed between
`C_bound` and any connection fiber.** That prohibition is what
mechanically enforces "isolated from the edge fiber" — without it,
sharing the `2×2` shape is a standing invitation to compute
`C_bound.commutator(M_e)` and breach the isolation through the type
system rather than through intent. The shared arity is the hazard
§2n.2 named; this is the guard against it.

**On exactness vs authority — an apparent contradiction, resolved.**
`C_bound` is over exact rationals, yet it is the residence of
approximation. These do not conflict, and the landed cage shows why:
floats live inside the consulted closure (`cage_pins.rs` runs
`(n as f64).sqrt()` there deliberately), and what the closure returns
is exact-typed and still unverified. **Exactness is a representation
property; authority is a door property.** `C_bound` holds exact numbers
that nothing has yet admitted.

**The general invariant — boundedness, carried per-axis (operator
correction, 2026-08-02).**

An earlier draft wrote this as `O(1)`. **That is a scalar collapse of a
multi-axis quantity, in a document whose whole thesis is that scalar
collapse destroys structure** — the same error as `C(Δ) = Σ|R|`, made
about complexity instead of about residuals. A single `O(·)` cannot say
which axis a sector grows in, and which axis it grows in is the entire
question.

**Cost is carried as a tuple over the axes it actually scales in, never
as one symbol:**

    O( |V|, |E|, F, δ, τ )

    |V|  orbs — the vertices of the complex
    |E|  relations — its edges
    F    fiber arity (channels = 3 today)
    δ    depth — path length, search budget, walk degree
    τ    steps — ticks over which the footprint persists

**Two different quantities were being named by one symbol** (operator,
2026-08-02). The collapse was not `O(1)` being wrong; it was `O(1)`
answering a question about storage while being read as a bound on
execution:

    State footprint — the container
      C_bound's storage. Must not shadow the topology:
                          ∂Memory / ∂|E|  =  0
      O(1) in |V|, |E| and F. This is what stops the quarantine
      inflating as the network grows.

    Execution trajectory — the work done inside it
      **O(n^D)** — the volume of the space the work ranges over: side
      `n` along `D` independent axes. **Locking this to O(1) flattens
      the dynamics entirely** — a search that may not scale with depth
      is not a search. This is where the dimensionality actually lives,
      and it must stay free.

**`O(n^D)`, not a flat budget** (operator ruling 2026-08-02, replacing
an earlier `O(x)` reading that named a single axis and so collapsed the
others). A constant work budget rations independently of how much
structure stands — the same flattening as `C(Δ) = Σ|R|`, applied to
cost. `D` independent axes of extent `n` enclose `n^D`, and **the
envelope breathes with `D`**: rank is earned rather than granted and
article IV holds it monotone, so the envelope widens as the system
gains dimension and never narrows. That is the re-volumizer — the
budget is a function of standing rank, not a number chosen in advance.

`n^0 = 1`: no rank, no room — exactly the standing `capacity_law`
refuses to mint on, so the envelope agrees with the constitution rather
than contradicting it. Landed and certified as `kernel::motion::envelope`,
which refuses on overflow rather than wrapping (an envelope that wrapped
would report an enormous space as a tiny one).

**So the lawfulness test is per-axis, and it is a topology test rather
than a size test:** a sector is lawful iff its storage is O(1) in the
axes it observes (`|V|`, `|E|`, `F`), while its trajectory may scale
freely along its own axes (δ, τ), bounded by declared budget rather
than by law.

The solver's sector sweeps `for _depth in 0..=max_depth` — O(δ) in
work, O(1) in the graph: lawful, and its high-dimensional search is
preserved rather than flattened. `C_bound` is O(1) in storage on every
axis. §2n.1's withdrawn construction was O(|E|) in storage, which is
exactly the violation. **A single symbol could not have told the three
apart** — which is the whole argument for carrying the tuple.

The two clauses that follow are exact as relayed:

1. **No fiber shadowing.** No quarantine, solver or search may
   duplicate a per-edge structure or mirror the topology. This is the
   clause §2n.1 violated.
2. **Single point of egress.** One bounded region, one door. `C_bound`
   may be inspected or discarded whole without invalidating any
   connection fiber — which is the operational test that the isolation
   holds.

**Status.** `d150497` stands as written. `K5` (#32) is unaffected and
clean — it was never about `C`. The per-edge fibers remain pure.

---

## §10bis The exile of C — containment, and C's own V

**Law (operator rulings 2026-08-01).**

> put all of the things that tried to be universal into a little box so
> it doesn't leak out and infect other parts and destroy the determinism
>
> C cannot leak into D and needs to find his own V — cage it, wrap it

**The containment law: a claim of universality is the warning, not the
credential.** Everything that leaked into the deterministic core
announced itself identically — `C` claimed to be *the* measure of
convergence, the damped-gradient term claimed to be *the* driver, "the master equation"
claimed to be *the* composition. Three constructs, one signature, all
three struck or caged within a day.

**Containment, never prohibition.** This system is all-encompassing and
must be able to represent dogma, narrative and transcendental readings —
a universe that cannot represent them is not all-encompassing, and
pretending they do not exist is how they return wearing new vocabulary.
The error was never that such things exist; it was granting them
universal scope. So they are boxed, not banished — and because nothing
escapes the box, it may safely hold what a deterministic path could
never lawfully touch: floats, roots, transcendentals, sampled values
bound for a chart. **Prohibition makes the system unable to say them;
containment lets it say them while determinism is guaranteed.**

**Three seals, landed** (`engine/rfa-core/src/cage.rs`, `cage_pins`,
`cage_arrow`):

1. **No ordering.** `Lens` derives `PartialEq` and deliberately not
   `PartialOrd`. Ordering is what descent needs, so `a < b`,
   `a > epsilon`, `.min()`, `.sort()` are type errors. Identity is what
   witnesses need, so `assert_eq!` still states what a state measures.
   Knowing what a reading is cannot be descended; knowing which side of
   something it falls on can.
2. **No usable interior.** The cage holds a 2×2 containment vector, and
   the caged value sits in no cell — canonical `p/q` is split to
   opposite diagonal corners at maximum inverse opposition, and the
   assembly rule exists only inside `show()`. A leak yields components,
   never a magnitude.
3. **C's own V — the closed algebra.** A cage with no interior life
   invites opening, because the value inside is still wanted. So C is
   given its own universe: every operation on a `Lens` yields a `Lens`,
   C may combine and accumulate freely inside its own space, and there
   is no fold, sum or combination producing anything the engine computes
   on. **C is not starved; it is exiled with everything it needs.**

**The arrow, enforced by a standing test** (`cage_arrow.rs`, the
`recurrence_guard` pattern — editing the test is the amendment): no
module on the computing path — `potential`, `reduce`, `fold`, `gates`,
`rcf` — may name `Lens` or call `coherence_functional`. This is stronger
than *does not gate on C*: it is **does not know C exists**, which is the
only version that cannot rot, since a dependency cannot be reintroduced
on a name the module never mentions.

Why the seals and not the docstrings: "do not gate on this" failed twice
in one day, in `validate_step`'s productivity gate and the kernel
bridge's `rank_is_defined`, and both times the prose was correct
while the code did the opposite. The compiler does not forget and does
not need re-reading.

**Owed:** the kernel bridge's `defect()` is still a bare `Ratio` wearing
a "do not gate on this" comment — the last instance of the protection
that has already failed twice.

## §10ter What the cage is for — and why V is never a candidate

**Law (operator ruling 2026-08-01).**

Ruling (paraphrased): the cage is for the expansion of variables that
must not affect the totality of the mathematics of the universe —
floats, approximations, dogma and the like.

**The test is corruption, not shape.** A thing is caged because
admitting it would break exactness, reproducibility or falsifiability —
never because it happens to be one number:

| caged | why |
|---|---|
| floats, approximations, sampled or truncated values | not exact; one admitted anywhere makes the universe non-reproducible |
| dogma | unfalsifiable by construction (Article III law 6) — no experiment refutes it, so no rule may rest on it |
| aggregates used as scores (`C`, the conservation distance) | exact, but they collapse many into one and what remains is rankable — a descent waiting for a caller |

**`V` is not A candidate, and neither is `D`, `Δ`, or any of their
invariants.** They are the totality the cage exists to protect.
Enclosing one would be enclosing the thing the enclosure is for. This is
categorical rather than a judgement call, and it is pinned by
`cage_arrow::the_cage_names_no_manifold_machinery`.

**Aggregate versus invariant — the line that keeps V free.** The
distinction is structural, and the proof is already in the crate:

    Aggregate   C = Σ|R| collapses many into one and destroys which
                residual broke and by how much. Caged.

    Invariant   rcf_signature, a holonomy's conjugacy class — the exact
                canonical fingerprint of one object, destroying nothing:
                two Deltas share it IFF conjugate. Not caged.

`Lens` and `RcfSignature` carry the same derives — `PartialEq, Eq`, no
`PartialOrd`. Their algebra is identical. The only difference is the
arrow ban. **So the no-ordering property — the thing that actually stops
descent — is available without containment**, and a canonical invariant
follows the `RcfSignature` pattern: equality-only, uncaged,
kernel-eligible. Containment is a second and heavier act, reserved for
readings that compute nothing.

**And the escape route does not exist for SL₂.** With `det = 1` the
characteristic polynomial is `x² − Tr(A)·x + 1`, so the conjugacy class
is parameterised by the trace. "Keep the class instead of the trace"
renames a scalar rather than avoiding one. The lawful move was never to
avoid scalars — it is to forbid ordering them and keep them as identity.

### §10ter.a The door — the cage as workshop, and C as its defence

**Law (operator ruling 2026-08-01).** The cage is not only a quarantine,
it is a workshop. Worlds and civilizations may call into it to run
algorithmic potentials — float pipelines, GPU kernels, heuristics,
sampling — and withdraw answers and functions, without the approximation
spreading outward.

**One sentence carries it: approximation may propose; only exactness may
dispose.**

This is U1a's prover/verifier split generalised from solving to compute.
There, `solver` searches by any means and `verify_work` re-forges,
re-applies and re-chains exactly — the search may be a GPU grinding
floats and the judgment never touches one. The door is that shape made
general:

```text
call in    →  the inside may use ANY means. No purity is asked of it,
              because nothing it produces is believed.
withdraw   →  what leaves is a PROPOSAL: an exact candidate that the
              outside re-derives for itself.
```

**A proposal is a guess, never evidence** — the outside does not accept
it because the cage produced it, it re-checks it under its own exact
rules, exactly as a mesh peer re-judges an incoming step instead of
trusting its sender. A proposal that fails verification costs nothing,
because nothing was trusted.

**The type stops the disease, not care.** `Proposal<T>` carries only
`T: Exact`, and `Exact` is implemented for the exact carriers alone. A
float has no impl, so `Proposal<f64>` does not exist — the approximation
cannot be named at the door, never mind carried through it.

**D and V do not live in here.** They stay outside with the rest of the
mathematics; the door is how the outside uses the inside, never how the
inside acquires the outside (§10ter).

**C's elevation, if it is ever elevated, is as the defence of this
sector.** Not a promotion out of the box — a post inside it. C's proper
work is guarding the containment: checking that what sits in the sector
stays consistent with itself and that nothing crossing the door carries
inconsistency outward. It reads the cage, never the manifold. That is
the only elevation available to a quantity whose whole nature is to
collapse structure: **it cannot measure a universe, but it can watch a
box.**

### §10ter.b C is consulted — the resident, not the prisoner

**Law (operator ruling 2026-08-01).**

> C is the culmination of congruence / coherence / cohesion /
> conversation / communication / comingling / consilience — he might
> need the tools inside the containment sector, so he should be
> consulted, which is why C(Δ) exists: it can use any tool in the
> toolbox to perform those actions without them impacting the external
> determinism.

**This reframes the containment entirely, and correctly.** Every member
of that family is a reconciliation — *are these the same? does this hold
together? can these exchange? do independent lines agree?* — and none of
them can be answered exactly in the general case. They need search,
comparison, heuristics, sampling: the whole dirty toolbox.

**So C is not a prisoner in the sector. C is its resident.** It was
never banished; it was housed where the tools its work requires already
live — the same tools that may never touch the outside. `C(Δ)` is the
knock on that door: a call, not a score.

    consult(question, tool) -> Proposal<Exact>

The tool may do anything internally — floats, a GPU kernel, a sampled
search, an approximation unlawful anywhere else. Only its answer
crosses, only if exactly representable, and even then only as a guess
the caller re-derives under its own rules.

**The loop this closes, and it is worth naming.** Charter §11 excised
the reconciliation instruments — consilience, the manifold, the census,
the triangle — on the ground that a second readable truth is not a
measurement. That was right *as law*. **They return here as tools, and
they cannot become a second truth in this room**, because a proposal
carries no authority: reconciliation that must be re-verified before use
is advice, and advice is all it was ever fit to be. The instruments were
not wrong; they were mis-seated. This is their seat.

**Why consultation is safe where scoring was not.** A score is consumed
as a verdict; a proposal is consumed as a hypothesis. The first grants
authority to whatever produced it, the second grants none — which is why
the same quantity that was unlawful as a gate is unremarkable as a
consultant.

### §10ter.c The ACCESS/AUTHORITY trade — how C is protected, and how C protects

**The claim, measured rather than asserted** (operator 2026-08-01: *C is
the strongest on the board; every major event is a word starting with
C*). Checked against the corpus:

- At the deed level it is 9 of 64 — `ColonyFounded`, `ColonyCharted`,
  `ForwardSealed/Released/Slashed`, `Confluent`, `CouncilSealed/Closed`,
  `CrystalDrawn`.
- At the concept level it is total. **39 of 39 tested C-words are
  load-bearing**, most of them heavily: chronicle (974), class (633),
  council (395), covenant (380), charter (353), colony (347),
  civilization (327), capacity (300), crystal (284), claim (261),
  collateral (230), coordinate (221), certificate (208), conservation
  (198), convergence (157), crossing (142), coboundary (121), coherence
  (115) — and beneath them cocycle, conjugacy, connection, curvature,
  circulation.
- Three of the six kernel invariants are C-words: I1 **conservation**,
  I2 **containment**, I4 **creation**.

So C is not one reading among many. **C is the semantic spine**, and the
question "pipe anything through C" is asking the system to consult its
own organising vocabulary.

**The protection, and it is one mechanism doing both jobs.** The proposal
boundary protects in both directions at once:

    protects others from C   nothing it produces is authoritative; a
                             wrong answer dies on exact re-derivation
                             and costs nothing, because nothing was
                             trusted.

    protects C from others   it can never be made a gate or a score
                             again — which is exactly what corrupted it
                             — because a proposal cannot bind anything.

**The law this reveals: access is purchased with authority.** The two are
traded inversely, and the trade is what makes both ends safe:

```text
THE KERNEL   minimal access  — pure, total, no I/O, encoding-blind
             maximal authority — its verdict is final and binding

C            maximal access  — any tool, floats, GPU, sampling, search
             zero authority   — everything it produces is a guess
```

**C may touch everything precisely because nothing it says binds.** That
is not a limitation placed on it; it is what buys its reach. A judge
must be starved of the world to be trusted with the verdict, and a
consultant must be denied the verdict to be trusted with the world.

**And it is why C cannot be "promoted" without being destroyed.**
Granting it authority would immediately cost it access — the moment
its word binds, it may no longer hold a float, because a binding word
made of approximation is the contamination the sector exists to stop.
**To protect C is to keep it unbinding.** §10ter.a's post — defence of
the sector — is the only elevation that adds standing without taking
reach, because guarding a boundary is an act, not a verdict.

**Note on `rank_is_defined` (2026-08-13 correction, carried over from
`17_CAPACITY`'s own drift).** The "kernel bridge's `rank_is_defined`"
named above as a 2026-08-01 instance of a failed docstring-only
protection was deleted that same day, but the deletion did not hold: it
is alive today as the phantom-rank gate, landed `153a70c` 2026-08-04, at
`engine/kernel/src/bridge.rs:304` and consulted from `engine/kernel/src/
lib.rs:660,777,783`. The point this section makes — that a docstring
alone failed to hold the line twice — is undisturbed by the method's
later return; it is recorded here only so a reader does not go looking
for a deleted function that is, in fact, back.

