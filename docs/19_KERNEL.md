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
- `kernel::lawful` has seven invariants saying what a deed MAY do.

Neither derives from the other. A deed's meaning can change in `replay`
without `lawful` noticing, and has: `KERNEL_RELAY` §1.4 recorded the
consequence already — *"their MEANING lives in `replay()` — an
interpreter three coders rewrite weekly … the actual trust root today
is the git repository plus coder discipline."*

That is the whole distance between what exists (a judge) and a kernel
(the definition). Two answers to one question is the mirror disease at
the semantic layer, and §11 rule 3 — *one validated update function* —
is not satisfied by a validator that stands beside the updater.

**The cure, in order of preference:**

1. **Derive apply from judge.** `lawful` already computes the lawful
   per-resource deltas (`lawful_deltas`), the lawful gauge move
   (`gauge_law`), the lawful element move (`element_law`). Where the
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
    require standing capacity_law — D > 0 and dV ≠ 0, or no mint
    commit           the arbor: one record, parent-linked, the judging
                     VERSION bound into the commitment

Every one of those exists and runs. What a miner additionally needs,
and does not yet have, is the ORDERING/Sybil anchor: nothing makes
weight costly to fake, so the mesh will govern cooperating nodes only
until it lands (`KERNEL_RELAY` §7.1). That is a mesh-lane rung, and it
is the honest reason the kernel is *mineable* but not yet *contested*.

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
coefficients = what each carries. This is already how `18_TORSION` §2e
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
overlap — and it is why `S⁺_out` (egress, positive divergence) and its
absorbing counterpart are load-bearing here in a way they were not in
`18_TORSION` §2d, where they merely renamed the mint and the chain and
drove nothing. **Struck there for driving nothing; earned here because
the join has no mechanism and this supplies one.**

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
  `capacity_law` that did not exist; it landed at `bd99c4b` and the
  label now means what it says.
- **Struck vocabulary is still live** in `17_CAPACITY.md`,
  `18_TORSION.md` and `EQUATIONS.md` — the anti-dogma ordinance's
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
