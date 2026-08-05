# 19 — THE KERNEL: what it must be, and where it actually is

**Status:** SPEC (kernel lane, 2026-08-02). Answers one question —
*what is the irreducible core that makes this a kernel rather than a
large program with a validator bolted on* — and measures the current
tree against it. Subordinate to `00_CHARTER.md` §11 and
`engine/kernel/CHARTER.md`.

---

## §1 THE TEST

A kernel is **the one function that defines a lawful transition.** Not
a checker beside the thing that transitions — *the* definition. Pure,
total, no I/O, encoding-blind, network-blind.

The test is single-valuedness: given `(prior, step)`, is there exactly
one lawful `next`, and is there exactly one place that says so? If two
components can answer differently, there is no kernel — there is a
program and an auditor.

## §2 WHAT IS ESSENTIAL — measured, not argued

`engine/kernel` is 1003 lines in `lib.rs` plus 566 in `bridge.rs`. Its
entire dependency on the 5001-line `world.rs` is **three items**:

| what the kernel takes from `cfe::world` | why |
|---|---|
| `Committed` | the state type it judges |
| `demurrage_due` | the sealed decay rule (I5 calls it, never copies) |
| `ease_of` | the tech modifier that rule reads |

Three items out of 105 public functions. **The kernel does not need
`world.rs`. It needs a state type and two sealed rules.**

The irreducible core, then:

    STATE          Committed — the type a transition carries
    JUDGMENT       lawful(prior, next, step) — the six invariants
    SEALED MATH    rfa-core (reduction, holonomy, the fold gate)
    SEALED WORK    solver-verify (verify_work — the PoUW half)
    THE RULES      demurrage_due, ease_of, mint_bound, founding_reserves,
                   ZoneKind::cost/raze_refund — called, never copied

Everything else in the tree is a CLIENT of that core or a LENS over it.

## §3 THE DEFECT THAT STOPS IT BEING A KERNEL

**The transition is defined twice, and nothing forces the two to
agree.**

- `cfe::world::replay` has **113 match arms** saying what each deed
  DOES to state.
- `kernel::admits` has seven invariants saying what a deed MAY do.

Neither derives from the other. A deed's meaning can change in `replay`
without `lawful` noticing, and has: `KERNEL_RELAY` §1.4 recorded the
consequence already — *"their MEANING lives in `replay()` — an
interpreter three coders rewrite weekly … the actual trust root today
is the git repository plus coder discipline."*

That is the whole distance between what exists (a judge) and a kernel
(the definition). Two answers to one question is a second source of
truth at the semantic layer, and §11 rule 3 — *one validated update function* —
is not satisfied by a validator that stands beside the updater.

**The cure, in order of preference:**

1. **Derive apply from judge.** `lawful` already computes the lawful
   per-resource deltas (`lawful_deltas`), the lawful gauge move
   (`gauge_move`), the lawful element move (`element_move`). Where the
   judge already knows the answer, the applier should USE that answer
   rather than recompute it. Every place it does is one fewer chance to
   disagree.
2. **Where it cannot derive, pin the pair.** A red-first witness per
   deed asserting `replay(prior + deed) == the only next lawful admits`.
   Expensive but honest, and it makes disagreement a build failure
   rather than a discovery.

## §4 `world.rs` IS THREE FILES WEARING ONE NAME

5001 lines, and the composition is not one thing grown large — it is
three unrelated things stacked:

| what | size | belongs to |
|---|---|---|
| **state types** — `Committed`, `ColonyOrb`, `Bond`, `Covenant`, `Council`, … | 12 defs | the KERNEL's vocabulary |
| **the proposer** — `found`, `zone`, `collect_as`, `mint_discovery_as`, … | 105 pub fns | the GAME |
| **the interpreter** — `replay`'s arms | 113 arms | the TRANSITION (see §3) |

Each has a different reader, a different rate of change, and a
different blast radius. A kernel amendment must reason about the first
and third; a game landing touches the second. They share a file, so
every change to any of them re-reads all of it — which is exactly how
"abstract concepts piled on top of each other" happens.

**The split is a seam, not a deletion.** Nothing here should be
removed; the state types and the interpreter want their own homes so
the kernel's surface is legible without reading the game.

## §5 THE KERNEL IS THE MINER

Read as a miner, the core is already complete in shape:

    take work        a deed carrying an opaque proof payload (tag 86)
    verify it        solver_verify::verify_work — re-forge ∧ re-apply
                     ∧ re-chain; an intact chain over a non-solution
                     refuses
    bound the pay    mint_bound — the ruled ladder f, exact integers
    require standing standing — D > 0 and dV ≠ 0, or no mint

    commit           the arbor: one record, parent-linked, the judging
                     VERSION bound into the commitment

The ladder once carried a fifth rung between the bound and the commit —
`require standing: capacity_law — D > 0 and dV ≠ 0, or no mint`. It is
REMOVED (operator ruling 2026-08-04, kernel VERSION 3 → 4): *identity is
pegged to entity, and identity gets recycled without bonds*, so a
standing test read off relational structure refused the lawful recycled
line by construction. Verified work remains the mint's one anchor, which
is what the first two rungs already are.

Every one of those exists and runs. What a miner additionally needs,
and does not yet have, is the ORDERING/Sybil anchor: nothing makes
weight costly to fake, so the mesh will govern cooperating nodes only
until it lands (`KERNEL_RELAY` §7.1). That is a mesh-lane rung, and it
is the honest reason the kernel is *mineable* but not yet *contested*.

**This gap is where a nonce would enter, and it is ruled shut**
(`23_TRAVERSAL` §2k, operator 2026-08-02). Hashcash is the reflexive fix
for exactly this hole. The anchor must be paid in structure that
survives reduction, not in energy spent — and that ruling supplies the
criterion without yet supplying the mechanism, so the gap stays open
with one door now closed.

## §5a THE SHAPE POW++ ACTUALLY HAS (operator, 2026-08-02)

§5 read the miner as a line. It is not one.

> two 2D chains running across one n-dimensional chain — a polynomial
> made of chains, or a CY manifold made of individual overlapping Venn
> diagrams

Unpacked against what is landed:

**Two chains, each 2D.** PoUW and PoWC are the two axes, and each
already splits into a LOCAL half and a CROSS-NODE half (`kernel`
CHARTER §5a): I4's mint-bound with the mesh's certificate replay; I6's
fold gate with cross-node agreement. Two axes × two halves is the 2D of
each chain, and neither is complete with one half.

**Across one n-dimensional structure.** The arbor carries branches, and
the gauge key carries graded planes. Both are open-ended, so the object
the two chains run across has no fixed dimension — which is why `D` is
a rank and not a capacity.

**A polynomial made of chains.** Degree = the standing planes;
coefficients = what each carries. This is already how `24_FORM` §2e
reads the fiber.

**THE HARD PART, and it is a real blocker.** *Overlapping* Venn
diagrams means the charts INTERSECT. A Calabi-Yau is assembled from
local patches that agree on their overlaps — the overlaps are the whole
content, because that is where the transition functions live and where
compactness is enforced.

**The arbor can fork and cannot JOIN.** There is no merge anywhere in
`mesh::arbor` — measured. So today the structure is a semilattice: it
fans outward and never meets. Overlapping charts are not expressible on
it at all, and neither is convergence, which is the entire value
thesis.

So the sequence PoW++ actually needs is not "finish the kernel, then
mesh." It is:

1. the local halves (landed: I4 whole, I6's gauge half);
2. **the join** — two branches meeting in the arbor. Measured: `arbor`
   exposes `extend`, `branch`, `adopt`, `append` and no merge, and
   `Commitment.parent` is a single `[u8; 32]`. A join needs two, and
   the coordinate is hashed into every commit, so this is a WIRE
   AMENDMENT to tag 84 plus an operator ruling — not a coder change;
3. the overlap law — what two charts must agree on where they meet,
   which is where `orbit_lock` finally gets its caller (#10);
4. the cross-node halves.

Step 2 is the one nothing can route around. Compactness maps onto the
standing T∞ purge — the torus wraps finitely, infinity is disallowed —
so that constraint is already law. What is missing is the meeting.

### §5a.1 THE BOND AT THE OVERLAP — why a two-parent link is not enough

The minimal join is a record with two parents: git's merge commit, and
the obvious first answer. **The operator's ruling is that this is a
paperchain and insufficient, and the reason holds up.**

A two-parent hash link records THAT two branches met. It records
nothing about WHAT the meeting preserved. For a ledger that only ever
picks a winner, that is fine — the losing branch is discarded and there
is nothing to carry. For convergence it is fatal: I6 says *no
difference dies without account*, and a bare parent pointer accounts
for nothing. The overlap is not the seam between the content; the
overlap IS the content.

**So the join record must carry the overlap region, not a pointer to
it.** Three things, and the engine already computes all three:

    the shared class   what both charts agree on — the part that FOLDS
                       (orbit_lock = ∅: the difference was gauge)
    the residues       what each side holds that the other cannot
                       explain — the part that EMITS as a new plane
                       (orbit_lock ≠ ∅: the difference is genuine)
    the transition     how a reading in one chart is read in the other
                       — the frame change between the patches

The first two are exactly the fold gate's two verdicts. **This is where
`orbit_lock` finally gets its caller** (#10), and the caller is not an
afterthought: the fold gate's verdict IS the join's substance. A join
whose residues are unrecorded is a merge that destroyed difference,
which the constitution forbids.

**The white/black-hole reading, and what it earns.** An
Einstein–Rosen bridge is a black hole and a white hole joined at a
throat: structure enters one side and emerges on the other, and the
throat is a REGION with its own geometry rather than a line between two
points. That is the right shape for a join whose content is the
overlap — and it is why the egress/ingress pairing is load-bearing
here in a way it was not in
`24_FORM` §2d, where its symbols merely renamed the mint and the chain and
drove nothing. **Struck there for driving nothing; earned here because
the join has no mechanism and this supplies one.**

**What keeps the throat open is ruled in `23_TRAVERSAL` §2k** — the
Einstein–Cartan half of the pair. A throat held open by accumulated
weight pinches to a point; one held open by structure no frame explains
stays traversable. That section also carries the consequence for
mining: the coboundary trap and the singularity are the same failure,
so nonce-smashing is already barred by law the bridge landed.

**Orbs at the overlap have extent — and this one IS derivable.**
`bridge.rs` §head: *"Entities `(i, j, k)` are ORBS, indexed by position
in `Committed::orbs`."* An orb projects to a VERTEX — a 0-cell. A
`ColonyOrb` is not internally empty (zones, banks, famine, thermal
site, the compassion and morality books), but the projection keeps none
of it: interior structure reaches the complex only as the obligations
it happens to emit.

Two vertices can share an EDGE. **Two vertices cannot share a REGION**,
because neither has one. So the Venn overlap is not merely unbuilt on
the current complex — it is *inexpressible* on it, for the same reason
the arbor's missing merge makes convergence inexpressible on the
arbor. Two independent walls, one wall each.

Giving orbs extent is therefore not inventing new state. **The state is
already there and the projection discards it.** An orb whose interior
coordinates survive projection is a cell with extent, and two such
cells can intersect in a non-empty subcomplex — which is exactly a
Venn overlap, and exactly where the shared class, the two residues and
the transition function have room to live.

**NOT DERIVED, flagged rather than guessed:** the number. "3D/5D" is
the operator's reading and this document does not derive 3 or 5.
Usefully, though, it is not arbitrary either — the dimension is a
CONSEQUENCE of which interior coordinates the projection admits, so it
becomes measurable the moment that ruling is made rather than needing
to be picked. Naming a number now, ahead of the ruling, would be the
§2d failure repeating.

**Not derived, flagged:** whether the overlap condition is Ricci-flatness
in any load-bearing sense, or whether CY is being used as a shape
metaphor for compact-with-agreeing-patches. The second is enough to
justify the join; the first would be a much stronger claim and this
document does not make it.

## §6 DOCUMENTATION FOUND OUTDATED

- **`16_UNIT` §2** lists "state semantics — cfe (deeds, orbs, the
  world)" as one organ. True as far as it goes, and it conceals §3:
  the transition lives there too, as a second definition. The organ
  list should name the interpreter as its own organ, or the split in
  §4 should make the claim true.
- **The `VERSION 3` defect is CLOSED.** `lib.rs` described a
  `standing` that did not exist; it landed at `bd99c4b` and the
  label now means what it says.
- **Struck vocabulary is still live** in `17_CAPACITY.md`,
  `26_ARTICLES.md` and `EQUATIONS.md` — the anti-dogma ordinance's
  terms, and `capacity` where the code now says `rank`.
  `17_CAPACITY.md` is *named* for a struck container word, which is how
  a struck idea returns.

## §7 THE ORDER

1. Split `world.rs` (§4) — state types and interpreter into their own
   modules. Mechanical, no semantic change, and it makes everything
   after it readable.
2. Derive apply from judge where the judge already knows (§3 cure 1).
3. Pin the remainder (§3 cure 2) — disagreement becomes a build failure.
4. Then the live gate: `world.rs` demoted to proposer, refusals
   preventing application rather than merely recording it.

Steps 1–3 are what make the word "kernel" true. Step 4 is what makes it
load-bearing.

## §3a WHY WE ARE JUDGING AT ALL — and why `-> Committed` is still linear (operator, 2026-08-02)

### 3a.1 The judge exists because the transition is defined twice

    pub fn lawful(prior: &Committed, next: &Committed, step: &Step)
        -> Result<(), Unlawful>

**It takes `next` as an argument.** Someone else computed the answer and
the kernel is handed it to grade. That is the defect in the signature —
§1's test says *not a checker beside the thing that transitions, THE
definition* — and "judging" is simply the shape a definition takes
after being demoted to an examiner.

Locally there is nothing to judge: the proposer and the judge are the
same authority, so the check catches coder bugs, never an adversary.
Cross-node, what looks like judging is **re-derivation** — the arbor
already says it (*"Admission is re-judging, never trust; a judgment you
cannot re-run is a judgment you do not have"*). You do not grade a
neighbour's claim; you recompute it and compare.

**So #30's target is not "apply wraps judge". It is: the judge
disappears.** When the signature no longer accepts `next`, the
duplication is structurally gone and the finish line is testable.

### 3a.2 BUT `-> Committed` IS STILL A LINE

`Result<Committed, Refused>` returns a POINT. One lump state with its
axes flattened inside it, and no way to read which axes moved or in
what order. That is the same scalar collapse as `O(1)` for complexity
and `C(Δ) = Σ|R|` for residuals, made about the transition itself.

**The multi-axial form returns the MOVE, not the resulting point:**

    transition(prior, step) -> Result<Move, Refused>
    prior ∘ move = next                    (∘ NOT commutative)

`Move` carries the per-axis change — banked per `(orb, resource)`, the
gauge move, the element move, the station move — rather than a whole
new world.

**Four reasons this is structural, not stylistic:**

1. **THE MOVE ALREADY EXISTS AND IS THROWN AWAY.** `lib.rs:611`:
   `lawful_deltas(prior, event) -> Result<Vec<(Resource, Ratio)>, Unlawful>`.
   That IS the move. `conservation` uses it only to compare against the
   `next` it was handed, then drops it. `gauge_move` and `element_move`
   do the same on their axes. **Every deed computes its own move and
   discards it.**
2. **MOVES COMPOSE; STATES DO NOT.** Two moves compose into a move; two
   endpoints compose into nothing. `arbor::Commitment` stores
   `state: [u8; 32]` — a state HASH. **That is why the arbor cannot
   join** (`K3` wall 1): you cannot merge two branches by merging their
   endpoints, but composing their moves is defined. The linear framing
   and the missing join are ONE defect, not two.
3. **ORDER BECOMES EXPRESSIBLE.** `a ∘ b ≠ b ∘ a`, and the difference
   is the commutator — which `23_TRAVERSAL` §2k.1 established IS the
   torsion. A state-valued transition cannot represent that two
   orderings differ; it hands back two endpoints and the ordering is
   gone. A move-valued one carries it.
4. **REFUSAL IS MULTI-AXIAL TOO.** `Unlawful` names ONE break. A deed
   can fail on several axes at once, and a single-variant refusal
   reports the first and hides the rest — the same collapse again, on
   the error side.

**The corrected target.** `transition` returns a `Move`, `replay`'s 113
arms become callers that compose moves rather than a second
implementation that rewrites states, and `K3`'s join becomes composition
rather than a wire amendment. **Not yet built** — recorded so the work
aims at the right shape.

## §3b THE z,l ON x,y — there is nothing to map between (measured 2026-08-02)

capacity-lane stopped on this: *"wiring `u(x)` from `V(Ω)` conflates two
index spaces — `V`'s `boundary_form` reads `Δ`'s relational indices,
while zones live on an 8×8 tile grid, and no mapping between them
exists."* Measured, it is worse than a missing mapping.

    Zone { tile: u16, kind }        an UNBOUNDED u16. No geometry, no
                                    maximum, no 8×8 — `zone_as` checks
                                    only that a tile is not already
                                    zoned. The grid is an assumption.
    ThermalSite { albedo, orbit_r } the orb's ONLY spatial fact is a
                                    scalar RADIUS. No angle, so two
                                    orbs can share a distance and never
                                    a place.
    Delta                           dim = orbs.len(), fiber = CHANNELS
                                    = 3. No spatial index at all.
    Coordinate { s: [Ratio; 3] }    a wrapped triple that DOES exist —
                                    and it is a commitment ADDRESS,
                                    hashed for uniqueness. Nothing
                                    derives it from world state.

**There is no `x,y`, no `z`, and no shared space.** One index space
exists — `(orb, channel)` — and everything else is a scalar. The
mapping capacity-lane wanted has nothing on the other side.

### 3b.1 THREE RULINGS, in order, each a decision rather than a derivation

1. **DOES `tile` HAVE GEOMETRY?** Give it `(x, y)` and a bound, or
   declare it a bare label. Cheapest step, and it decides whether an orb
   can have an INTERIOR at all (`§5a.1` wall 2). Today the code says
   label; every document says grid.
2. **DO ORBS HAVE POSITIONS?** `orbit_r` alone cannot express
   adjacency, so **no two orbs can ever overlap** — that is the root of
   K3's wall 2, and no amount of work on the complex reaches it. Either
   siting gains an angle, or orbs gain a coordinate in a shared wrapped
   space. `Coordinate.s` shows the shape is already affordable; it is
   simply not a place today.
3. **WHAT IS `z`?** The reading the measurement supports: **`z` is
   SCALE.** Relations stand at tile↔tile, orb↔orb and system↔system,
   and all three would collapse into one `Delta` today because there is
   one. A STRATIFIED complex — one per scale, with maps between strata
   — is what `z` would index, and that is #17's multi-scale map rather
   than a new invention.

**`l` needs nothing.** It is the obligation channel, it works, and it
is the only coordinate in the system that already does what it says.

**NOT DERIVED:** which of the three is wanted. They are independent
decisions with different costs — 1 is a type change, 2 is a wire
amendment, 3 is an architecture. Doing 3 without 2 stratifies a space
in which nothing can meet.

## §3c RELAY ANSWERS — R-D2, the channel arity, and two corrections to the spatial ruling (2026-08-02)

### 3c.1 R-D2 · THE QUORUM — **STRIKE**

Verified: `vrp.rs:26` `attest(&self, clock)` never reads `self`, so
three seated timekeepers return one answer counted three times.
`Attestation::Refuses` is constructed only in a test; `QuorumOpen` is
unreachable.

**Ruled STRIKE, by this document's own LAW 8** — *a term that computes
is kept; a term that only resonates is excised*. The quorum resonates
(it reads as consensus) and computes nothing. One writer per clock is
already NS-5 §7 law, so nothing is lost.

**BUILD is refused for a stated reason:** a real quorum matters in the
mesh era, and building the mechanism now for a caller that does not
exist is exactly what LAW 8 bars. It returns when a second attester
does.

Execution touches `vrp.rs`, `genesis.rs` and `ns3_conformance.rs` in
`cfe` — shared, and days after a three-way merge. **Ruled here, not
executed; say the word and it is one commit.**

### 3c.2 THE CHANNEL ARITY — approved, with one pin added

Deriving from `CHANNELS` instead of literal `3`s is right: a structural
pin that hardcodes arity breaks on every addition and teaches nothing.
`capacity_law_pins:242` already read it correctly one line above a
literal that did not, so the file disagreed with itself.

**But deriving EVERYWHERE means nothing can catch an unintended arity
change.** One literal added, deliberately, in one place:
`the_channel_arity_is_four_and_changing_it_is_an_amendment`.

**MEASURED WHILE ADDING IT: `CHANNELS` is 3 on this branch.**
`CHANNEL_LANE` is present on `remediations-lane` and **not merged
here** — the relay describes it as landed, and it is landed on their
branch only. The pin asserts 3 today and becomes 4 when their work
merges, which is the point of having it.

### 3c.3 SPATIAL SITING — two corrections before the amendment can be drafted

**TAG `0x41` IS BARRED.** `0x41` is 65, and tag 65 is RETIRED
(Charter §11, 2026-07-31 — the consilience receipt).
`recurrence_guard::retired_wire_codes_are_never_reused` fails the build
on any reuse. Next free is **90**.

**AND NO NEW TAG IS NEEDED.** Siting already has one: `PlanetSited` is
**tag 25**, carrying `{ orb_id, albedo, orbit_r }`. Its decoder reads
three fields and ignores trailing bytes, so `θ` is an ADDITIVE-TRAILING
extension — the same shape the codec used for the taxon (v2) and the
seats (v3). No new tag, no rehash, and old receipts stay readable.

**`θ ∈ ℚ` IS ONLY SOUND IN TURNS, NOT RADIANS.** A full turn in radians
is `2π`, which is irrational, so a rational angle in radians cannot
express a wrap. **`θ` is a fraction of a TURN, `θ ∈ [0, 1) ⊂ ℚ`**, and
wrapping is exact.

**ADJACENCY NEEDS A DECLARED METRIC, and it cannot be Euclidean.**
Distance between `(r₁,θ₁)` and `(r₂,θ₂)` in the plane needs `cos` and a
square root — both barred (§2b: no roots, THE PROPORTIONAL LAW). The
system already has the answer it needs: `bridge.rs` records *reach is a
wrapped-L1 distance*. **Adjacency is wrapped L1 on `(r, θ)`**,
consistent with the metric already in use, and exact.

**Old receipts carry no `θ`, and it must decode as ABSENT rather than
zero.** Defaulting to `0` would place every historical orb at one
angle — all collinear — which is a spatial claim nobody made.
`theta: Option<Ratio>`, and an orb without one has a radius and no
place.
