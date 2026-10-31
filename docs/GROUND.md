# Ground — the organ map, and the do-not-rebuild gate

**Status:** law (2026-08-02). `00_CHARTER.md` §7 has cited *"`docs/GROUND.md`'s
organ map — the do-not-rebuild gate"* since before this file existed.
**It did not exist.** The citation dangled: `constitution_lint` verifies
that a backticked code citation resolves to something in `engine/`, and
nothing verifies that a cited document does. This file pays that debt and does not invent a new frame
to do it — the organ map is `16_UNIT.md` §2, quoted here as the
authority it already is.

---

## §1 The organs — the recomposition target, quoted not invented

`16_UNIT` §2, **one binary, six organs**:

```text
sealed math core   rfa-core (Δ, C(Δ), Π_G, orbit_lock) · cfe::ratio · verify_certificate
physics            reversible kinematics · beacon ξ_k
the judge          kernel::admits — I1..I6 + the two proofs
the spine + codec  the chain (tag 84) · a single encoding
state semantics    cfe (deeds, orbs, the world)
control plane      steward seam · authority walk · the uplink door
```

**The arrow, also quoted:** *"The kernel calls the sealed math, never
copies it. The arrow direction stays one-way: kernel imports sealed math
and cfe; spine, mesh, and clients import the kernel; nothing imports
upward."*

**Outside the unit by design:** the conductor (a client of units), the
ledger's display halves, tools, and the solver's search half.

## §2 The gate — do not rebuild what already stands

**Before building a mechanism, find its organ above and look there.** The
recorded cost of skipping this is not hypothetical: `47_REGISTRY` §3
records tier implemented three times with no authority, and `20_FOUNDING`
§6a records a tithe drafted from scratch when `demurrage_due` — flat rate
× ease × elapsed, settled at the touch, kernel-bounded — was already the
algorithm.

**The three questions, in order.** Which organ owns this? What does that
organ already expose? If nothing, does the new thing belong to that organ
or has an organ boundary been found? **A mechanism that seems to need two
organs is usually one mechanism in the wrong one.**

## §3 Decomposition requires recomposition — the standing rule

**A plan to break a file apart is not a plan until it says what the parts
compose back into.** Splitting by size alone moves lines between files and
buys nothing; the parts must recompose into exactly one organ, presenting
one surface, with the arrow of §1 unchanged.

**The four recomposition invariants.** Any decomposition in this codebase
holds all four or it has not landed:

1. **One organ, one surface.** External callers' imports do not change.
   The parts are internal structure, never a new seam for other crates to
   reach into.
2. **The arrow is unreversed.** No part imports upward — nothing under an
   organ may reach the judge, the spine, or a client.
3. **Shared rules are named as shared.** Any rule read by more than one
   path lives in one module that says so. A rule that two paths read from
   two places is the drift that has already cost this codebase a landing
   (§4).
4. **Line count is A symptom.** No module exists to be small. Each names a
   concern that survives someone asking what it is.

## §4 The world.RS decomposition

**The decomposition landed (`666be81`); this §4 is now a record of the
before state, confirmed 2026-08-04.** `engine/cfe/src/world.rs` does not
exist; `world/` holds 19 files, 6,860 lines, largest `mod.rs` at 903.
The line-range table below measured the pre-split monolith and is kept
as evidence, not as a map — do not navigate by it.

### 4a. What was there — measured before the split

```text
engine/cfe/src/world.rs                          5,034 lines
   30–  172  ActionRefused                         142   the refusal vocabulary
  418– 3647  impl World                          3,229   THE LIVE PROPOSER — 105 pub fns
 3647– 3783  aperture_transfer                     136
 4226– 4838  replay                                612   THE SECOND DEFINITION
 4850– 5034  demurrage_due                         184   a SEALED RULE the judge also calls
```

`impl World`'s 105 functions carry a `foo` / `foo_as(actor)` pair for
nearly every deed, so the distinct deed surface is closer to 55. They
group cleanly, and the groups are the module boundaries — not invented,
read off the tree:

```text
chain / receipt   13   seal_*, chronicle, committed
economy / trade    8   collect, transfer, offers, post_offer, purse, aperture_transfer
identity / genesis 9   found, genesis, entity, taxon, ward_*, seat_station_as
tech / practice    6   techs, grant_tech_as, adopt_practice, release_practice
thermal            5   commission_star, site_planet, age_star, vent_pending_of
survey / terrain   5   chart, chart_terrain, terrain_seed
zones              2   zone, zone_as
crisis             2   sever_bond
```

### 4b. The seam that matters — this is kernel task #30, not a cleanup

**The file is 5,034 lines partly because it holds the transition twice:**
`impl World` proposes and `replay` re-derives. Kernel task #30 exists to
give the transition one definition. **These are the same problem.**

**A split by size makes it worse** — it puts the two definitions in two
tidy files and makes the duplication comfortable to live with. A split
along this seam removes it. Doing them separately means doing the work
twice.

**The measured narrowing, offered to that lane.** `replay`'s 612-line
body recomputes exactly one shared rule —
`dop::commensurate_weight(resource)` in `world/replay.rs`, in the
`ResourceTransferred` arm. It never calls `ease_of` or `demurrage_due`;
those arms apply the receipted amount. **So the double-definition
exposure is one call site, not 113 arms.** And the P3 divergence (Crystal
127/16 live against 15/16 replayed) is unlikely to be shared-rule
recomputation at all: **live held more than replay**, which points at
state reached without a receipt.

### 4c. The target — five modules under one organ

```text
engine/cfe/src/world/
  mod.rs        the single surface — re-exports so no caller's import changes
  state.rs      World, ColonyOrb, the committed shape
  refusal.rs    ActionRefused                                    (142 lines today)
  deeds/        the ~55 distinct deeds, by the §4a groups
  rules.rs      THE SEALED RULES — demurrage_due, ease_of, and every
                other rule read by BOTH the proposer and the judge
  replay.rs     the second definition, until #30 collapses it
```

**`rules.rs` is the load-bearing module and the first one to cut.** It is
small, it is surgical, and it makes invariant §3.3 structural instead of
cultural: after it exists, "which rules do two paths read?" is answered by
a directory listing rather than by a bisect. **The kernel lane's P3
revert is exactly the failure it prevents** — a sealed rule's values moved
and the two readers disagreed, and nothing named the rule as shared.

### 4d. Sequencing — the parked patch is a hard constraint

**The kernel lane holds a saved P2 patch: 27 hunks across five files,
including `engine/cfe/src/world/`,** parked pending #30 rather than
re-derived. **Any restructure of `cfe::world` invalidates 474 lines of
standing work**, so this is not a lane-local decision.

**The order that costs least:**

1. **#30 first** — one definition of the transition. It is already on that
   lane's board, it gates P1/P2/P3, and it establishes the seam §4c is
   organised around.
2. **Re-apply P2** against the collapsed transition.
3. **Then cut**, starting with `rules.rs`, along a seam that by then
   exists rather than one this document guessed at.

**The piece worth doing sooner:** if `rules.rs` is extracted before
#30, it helps #30 rather than competing with it — the shared surface is
what #30 has to reason about. It conflicts with P2 only as a move of
`demurrage_due`, which reconciles more cheaply than a restructure. **That
is a coordination call for the kernel lane, not this one**, because the
patch is theirs.

## §5 What this document does not do

It does not move code. Nothing here is landed, and the decomposition is
a plan with a named blocker rather than a task ready to claim.

**And it records its own gap:** `constitution_lint` catches an unresolved
code citation and would not have caught this file's absence while the
charter cited it. **A citation to a document should resolve the same way
a citation to a symbol does** — that is a small
addition to a lint that already exists, and it belongs with the registry
lint (`47_REGISTRY` §5).
