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
