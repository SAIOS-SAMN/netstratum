# 21 — THE KERNEL PARAMETERS: the four numbers that price the system

**Status:** SPEC (kernel lane, 2026-08-02). Task #21 named "the four
kernel-visible literals" without listing them. This measures them,
states what each refusal calls through, and defines what each must
become under THE DYNAMIC PARAMETER LAW (#16).

Subordinate to `19_KERNEL` and `engine/kernel/CHARTER.md`.

---

## §1 THE MEASUREMENT — the kernel's entire parameter surface

`engine/kernel/src/lib.rs` reaches outside itself exactly three times:

    lib.rs:721   cfe::dop::founding_reserves
    lib.rs:885   cfe::world::demurrage_due
    lib.rs:889   cfe::world::ease_of

and holds one priced rule of its own:

    lib.rs:845   mint_bound(&Difficulty)

**That is the whole surface. Four parameters, and every economic
quantity the kernel can refuse on passes through one of them.** The
other ~43 placeholders on `PICKUP` §8c price the GAME; these four price
the KERNEL, which is why they carry an amendment and the rest carry a
ruleset bump.

## §2 THE FOUR, DEFINED

### P1 · `mint_bound` — THE PRICE OF COGNITION

    kernel/lib.rs:845          drives: Unlawful::MintExceedsWork (I4)

    level 3  →  1/1
    level 4  →  4/1
    level 5  →  7/1
    otherwise → 0/1

**What it is:** the ceiling on what a verified solve may mint. The
single most consequential number in the system — it is the issuance
schedule of the currency.

**Measured properties:**
- **Three paying rungs.** Levels 0, 1 and 2 earn ZERO. An NPC that
  solves a rung-2 task is paid nothing.
- Rungs are `1, 4, 7` — arithmetic, common difference 3.
- Off-ladder earns zero by construction: *"an off-ladder difficulty
  cannot name its own price"* (`discovery_pins.rs:213`).
- **It is a pure function of the WORK'S SHAPE and nothing else.** It
  reads no prior state at all.

**The defect that makes it #16's first case:** a mint that cannot see
the ledger cannot respond to it. The price of cognition is fixed
forever regardless of how much has already been issued, how many nodes
are working, or what rank stands. That is an issuance schedule with no
feedback term.

### P2 · `demurrage_per_tick` — THE SINK

    cfe/dop.rs:321             drives: Unlawful::DecayExceedsBound (I5)
    reached through            cfe::world::demurrage_due

    1/32 per elapsed tick

**What it is:** the decay rate on un-vaulted banked joules/sink,
settled by receipt at touch. **FLAT, never proportional** — the sealed
rule computes `min(rate · ease · elapsed, banked)`, and the flatness is
deliberate (`world.rs:4840`, "the 2^1100 denominator lesson").

**Measured:** one literal, constant, blind to supply. It is the only
destruction path the kernel bounds, so it is the only counterweight to
P1 — and the two cannot see each other.

### P3 · `ease_of` — THE MODIFIER ON THE SINK

    cfe/world.rs:4832          scales P2's bound

    condition-5 tech held  →  1/2
    otherwise              →  1/1

**What it is:** the tech modifier `demurrage_due` reads. Halves decay
for an orb holding a condition-5 tech.

**Measured:** **binary, with a hard threshold.** Two literals and an
`== 5` test — there is no gradient, so the modifier is a cliff. An orb
at condition 4 gets nothing; at condition 5 it gets the full halving.

### P4 · `founding_reserves` — THE SEED

    cfe/dop.rs:295             drives: I1 conservation at genesis

    Iron 20/1 · Biomass 10/1

**What it is:** what a founding orb may hold without it having been
minted. **Besides P1, this is the only lawful creation from nothing in
the system.**

**Measured:** two literals, per-founding, constant. Supply therefore
rises with orb count on a fixed slope that nothing observes.

## §3 WHAT THE FOUR ADD UP TO

    P1 ISSUES        per unit of verified work — fixed
    P2 DESTROYS      per tick of un-vaulted holding — fixed
    P3 MODIFIES      the destruction — binary
    P4 SEEDS         per founding — fixed

**None of the four reads prior state.** Total supply therefore follows
mechanically from how many nodes mine and how many orbs are founded,
with no feedback anywhere in the loop. That is the finding: **the
system has a monetary base with no monetary policy**, and #16 exists to
close it.

## §4 WHAT THEY MUST BECOME — and the constraint that makes it cheap

THE DYNAMIC PARAMETER LAW (#16) requires retirement to **bounded
functions of prior state**. The binding constraint is what `lawful`
actually holds when it judges:

    lawful(prior: &Committed, next, step)

A dynamic parameter may read `prior` and nothing else — no clock, no
network, no I/O — or the kernel stops being total and encoding-blind.

**MEASURED, and it is the good news: everything needed is already
reachable from `prior`.**

    total supply       Σ banked over prior.orbs           — no new state
    orb count          prior.orbs.len()                   — no new state
    standing rank D    bridge::read_entangled(prior).rank — landed
    potential dV       the same read, same pass           — landed
    elapsed            step.stamp − orb.last_touch        — landed

So each of the four can become `f(prior)` **without a schema change,
without a wire change, and without new committed state.** The amendment
is to the RULE, not to the record — which means each is a VERSION bump
and a red-first witness, not a migration.

**The shape the law requires**, stated without choosing the policy:

    every parameter becomes  f(prior) with a DECLARED BOUND
    the bound is checkable from prior alone
    f is monotone in its argument, so the direction of response is
      law rather than an accident of arithmetic
    the current literal is f evaluated at the genesis state, so the
      amendment is continuous and no live world jumps

That last clause is what makes this landable on a running game.

## §5 WHAT IS AN OPERATOR RULING, NOT A CODER CHOICE

**The policy itself.** Whether the mint softens as supply rises,
whether decay stiffens, whether rank is a multiplier or a gate — these
are monetary decisions and they are not mine to make. This document
defines the parameters, measures their present values, proves the
readings are available, and stops there.

Four decisions are owed, in the order they bite:

1. **P1's feedback term.** What does the mint respond to — supply,
   rank, node count, or nothing (i.e. is the fixed schedule
   deliberate)? Blocks everything else, because P1 is the issuance.
2. **Do rungs 0–2 pay?** Today they pay zero, which means the easiest
   verified cognition is worth nothing. Deliberate or an artefact of
   the ladder having been dialed for rungs 3–5?
3. **P3's cliff.** Should the modifier be a gradient in tech condition
   rather than an `== 5` test? A cliff is a strategy target.
4. **P4's slope.** Does the seed shrink as orb count rises, or is a
   fixed per-orb grant intended?

## §6 SEQUENCE

1. **P1 first.** It is the issuance; every other parameter's correct
   value depends on what it does.
2. **P2 with it, or not at all** — a sink dialed against an unfixed
   source is dialed against nothing.
3. P3 and P4 follow, and are independent of each other.

Each lands as: red-first witness → rule change → VERSION bump →
`19_KERNEL` §6 updated. No schema bump on any of the four.
