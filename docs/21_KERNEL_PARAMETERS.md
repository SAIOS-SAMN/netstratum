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

---

## §7 THE MONETARY RULING — landed, blocked, and what each needs (2026-08-02)

The operator ruled all four and cleared `P1`+`P2` to land. **Two of the
four are discharged; two cannot land in the form given.** Measured
against the tree, not argued.

### 7a. `P4` — DISCHARGED, no amendment needed

The ruling asks that the seed *operate strictly at `t = 0` and evaluate
to `0` for all `t > 0`*. **That is already true by construction:**
`world.rs:1351` refuses a second founding outright
(`ColonyAlreadyFounded`). There is no `t > 0` branch to zero because
there is no `t > 0` call.

Pinned rather than amended — `kernel/tests/parameter_pins.rs`:
`p4_the_seed_cannot_fire_twice`, `p4_founding_is_the_only_unminted_arrival`.
**Zero VERSION cost.**

### 7b. `P3` — CANNOT LAND. Three separate blocks.

    P3(rung) = 1 / (1 + e^{−k(rung − threshold)})

1. **THE SIGMOID IS NOT COMPUTABLE HERE.** `cfe/src/ratio.rs` exposes
   `ratio`, `is_canonical`, `encode`, `decode` — and nothing else.
   There is no `e`, no exponential, no root. A logistic curve is
   irrational at essentially every point, and `18_TORSION` §2b bars
   roots and floats outright. This is a **type-level impossibility**,
   not a preference.
2. **WRONG DOMAIN.** `P3` is `ease_of(techs, orb)` — a function of an
   orb's TECH CONDITION that scales `P2`'s decay bound. It has no
   `rung` argument and never sees the difficulty ladder. The formula
   as written substitutes the mint's domain for the sink modifier's.
3. **THE RUNG 0–2 FIX IS ATTACHED TO THE WRONG PARAMETER.** *"Rungs
   0–2 assigned non-zero baseline values"* is a change to `P1`'s
   ladder (`mint_bound`), which is what pays rungs. `P3` cannot pay
   anything — it modifies decay.

**The intent survives all three.** A gradient replacing the `== 5`
cliff is right and is landable as a **rational ramp** over the tech
condition — exact, monotone, bounded, and continuous at genesis if
condition 5 still yields `1/2`. That form needs one word of
confirmation; the sigmoid does not.

### 7c. `P1` — DIRECTION RULED, FORM BLOCKED

    P1 = Base(rung) · ( Φ_D / (Σ + ε) )

**What is sound and is now settled:** issuance softens as supply grows
without matching rank. `read_entangled(prior).rank` supplies the rank
term and is landed. **Confirmed safe:** `capacity_law` admits only
`Vacuity::Entangled` (`boundary.rs:191`), which requires `D > 0`, so a
rank-denominated term **can never see a zero denominator** — pinned as
`p1_capacity_law_refuses_before_any_rank_denominated_term_is_read`.

**What blocks:**

1. **`Φ_D` IS UNDEFINED.** "Dimensional capacity" names no landed
   quantity. `D` is an integer rank; `Φ_D` is a distinct symbol used
   without a definition, and an undefined symbol cannot be compiled.
   The obvious reading is `Φ_D := rank`, but that is a substitution,
   not a translation, so it is flagged rather than assumed.
2. **GENESIS CONTINUITY FAILS AS STATED.** The ruling requires
   `P1 → Base(rung)` as `Σ → 0`. **`Σ` is never 0 in a live world** —
   `P4` seeds Iron 20 + Biomass 10 at founding, so supply is 30 before
   any deed is judged. Continuity has to be pinned against the actual
   genesis state, which is what
   `p1_the_landed_rungs_are_1_4_7_and_zero_below` exists to hold.
3. **`ε` IS A FIFTH LITERAL.** An epsilon guarding a division is an
   undialed number doing load-bearing work in the mint. **A dynamic
   parameter law that introduces more literals than it retires has
   retired nothing** — and with `D > 0` guaranteed, the guard `ε`
   exists to provide is unnecessary.
4. **UNBOUNDED DENOMINATORS.** Exact-rational division by a running
   supply total compounds denominators, which is the failure
   `world.rs:4840` records as *"the 2^1100 denominator lesson."*
   Whatever form lands must be denominator-bounded by construction.

### 7d. `P2` — DIRECTION RULED, FORM REVERSES A LANDED LESSON

    P2 = Max( 1/32, 1/32 · (Σ / Φ_D) )

1. **THIS MAKES DEMURRAGE PROPORTIONAL.** The sealed rule is
   deliberately flat: *"`min(rate · ease · elapsed, banked)` per
   leaking resource, **FLAT, never proportional** (the 2^1100
   denominator lesson)"* (`world.rs:4840`). Scaling the rate by
   `Σ/Φ_D` is the exact move that lesson was learned from. **The most
   serious of the four blocks**, because it reverses a rule the tree
   arrived at by failure.
2. **UNBOUNDED ABOVE.** `Max` supplies a floor and no ceiling. A rate
   exceeding `1` per tick is total confiscation each tick; `min(…,
   banked)` prevents it going negative but not going punitive. **#16
   requires a BOUNDED function** — this one is bounded below only.
3. **`Φ_D = 0` IS REACHABLE HERE.** Unlike `P1`, demurrage is **not**
   gated by `capacity_law` — an orb with no standing obligations decays
   exactly like one with many. So the rank denominator can be zero at
   the moment `P2` is read. Division by zero, in the decay path, on a
   live world.

### 7e. WHAT IS OWED — one ruling, not four

`P4` is done. `P3`, `P1` and `P2` all reduce to a single question,
because the same substitution fixes all three:

> **Confirm the constitutional form: rank-denominated feedback carried
> as a BOUNDED, FLAT ladder — comparison and clamping rather than
> running division — with `Φ_D := rank`, a declared floor and ceiling
> on every parameter, and no `ε`.**

That preserves every ruled DIRECTION — mint softens as supply outruns
rank, decay stiffens, the cliff becomes a ramp, rungs 0–2 may be paid —
while staying exact, denominator-bounded, defined at `D = 0`, and
continuous at genesis.

**Not landed pending that word.** `P1` and `P2` price the currency and
gate the mint on a live game; landing an undefined symbol, an
unbounded rate and a reachable division by zero would be worse than
landing nothing. The pins are in place so the amendment is a one-commit
change once the form is confirmed.

---

## §8 CROSS-LANE COLLISION — `P2` and capacity-lane §6a rewrite one flow

Capacity-lane is landing a founding economy whose tithe rides the
demurrage rail. It correctly stopped short of the kernel — *"that's
kernel surgery on a shared crate, so I haven't done it"* — and the lane
map puts `engine/kernel/` here. This records the collision surface and
the one economic consequence neither lane has stated.

### 8a. FOUR SHARED ORGANS

    demurrage_due          P2 needs supply+rank; §6a needs the ward
                           split. One function, one signature change,
                           two reasons.
    decay() / I5           P2 moves the BOUND; §6a needs I5 to verify
                           a two-sided settlement. Same invariant.
    DemurrageSettled (29)  `{ orb_id, resource, amount }`, encoded via
                           `amount_layout` — ONE-SIDED, booked as a
                           pure negative. §6a needs a destination, so
                           it is a WIRE AMENDMENT. P2 needs none.
    ease_of                CHANGED at `d52f3fd` (P3): the `== 5` cliff
                           is now a gradient. Any §6a reasoning
                           audited against the cliff should be re-read
                           — the tithe now rides a smooth rate.

### 8b. THE CONSEQUENCE — §6a WEAKENS `P2` EXACTLY AS THE ECONOMY GROWS

§6a's conservation claim is right and its **stability** claim is not.

    tithe   = decay · capacity(ward)/capacity(world)   → TRANSFERRED
    burned  = decay − tithe                            → DESTROYED

Instantaneously `ΔΣ = −burned ≤ 0`, so the economy is deflationary at
every step — that much holds. But `burned = decay · (1 − share)`, and
**`share` rises as wards accumulate capacity.** As `share → 1`,
`burned → 0`: the sink asymptotically vanishes while `P1` keeps
issuing.

`P2` exists to be the counterweight to `P1`'s issuance (§3). §6a
weakens that counterweight in proportion to exactly the quantity that
grows over a world's life. **The two rulings are individually sound and
jointly divergent**, and neither document says so.

Not a veto — it is a real mechanism and the ward should earn. It needs
one of: a floor on `burned` that `share` cannot erode, a `share`
denominated so it cannot approach 1, or `P2`'s rate stiffening fast
enough to outrun it (which is what `P2` already does — so the two may
compose if the rate's response is steeper than `share`'s growth, and
**that is a measurable question, not a matter of taste**).

### 8c. SEQUENCE — `P2` FIRST, and the reason is mechanical

`P2` changes `demurrage_due`'s RATE; §6a changes its DESTINATION.
Landing the destination first means the split is verified against a
rate that is about to move, and the I5 amendment would be written twice.
Landing `P2` first fixes the rate, after which §6a is a pure
destination change over a settled bound.

`P2` also needs no wire amendment; §6a does. Taking the cheaper,
lower-blast-radius change first keeps tag 29 stable while the rate
settles.

---

## §9 `P2` IS BLOCKED BY THE DEPENDENCY ARROW — found before the 8 sites

Capacity-lane asked that `P2`'s signature be designed once, knowing the
split sink follows. Designing it surfaced a blocker that invalidates
§4's "no schema change" claim for `P2` specifically.

### 9a. THE MEASUREMENT

    cfe        depends on nothing internal        (the bottom)
    rfa-core   depends on cfe                     (game-blind)
    kernel     depends on cfe + rfa-core          (the top)

`bridge::read_entangled` — the only reader of rank `D` — lives in
`kernel`. **`cfe::world::demurrage_due` therefore cannot compute rank**,
and `world.rs` references it zero times today. The projection cannot
move below `cfe` either: it needs `Committed` (in `cfe`) and the
reduction (in `rfa-core`, which already depends on `cfe`), so any home
below `kernel` is circular.

### 9b. WHY THAT BLOCKS `P2` RATHER THAN MERELY COMPLICATING IT

`demurrage_due` is a SEALED RULE — *"the kernel CALLS this for I5's
bound (call-not-copy), one rule, one computation, two judges"*
(`world.rs:4842`). `P2` makes the rate depend on rank. So:

- the PROPOSER (`world.rs`) cannot see rank;
- the JUDGE (`kernel`) can;
- if they compute different rates, **honest deeds refuse at I5.**

Passing the rate in from above does not help: a rate the kernel accepts
without re-deriving is a number a node can lie about, and the mint sits
directly downstream. The sealed-rule discipline requires both sides to
reach the same value from the same state.

**This is exactly the cost capacity-lane warned would be paid twice.**
Rewriting 8 call sites first and discovering it after would have meant
rewriting them again.

### 9c. THREE WAYS OUT — a choice, not a derivation

1. **STORE RANK IN `Committed`.** Both sides then read one committed
   number and cannot disagree. **A SCHEMA CHANGE — which contradicts
   §4's claim that no parameter needs one; that claim was true for the
   kernel and false for the proposer, and is corrected here.** Its
   virtue is real: rank is the organising quantity of the whole system
   and nothing in the chain currently attests what rank stood when a
   deed was judged. Storing it makes the mint's standing auditable
   instead of recomputed.
2. **A NEW CRATE BETWEEN `rfa-core` AND `kernel`** holding the
   projection, which `cfe` cannot depend on either — so this only
   helps if `demurrage_due` moves up with it, which un-seals the rule
   from the proposer's side.
3. **DEMURRAGE STOPS DEPENDING ON RANK.** `P2` responds to SUPPLY
   alone, which `cfe` can compute from its own state without the
   bridge. Loses the rank term; keeps the sink dynamic and keeps the
   ruled direction (stiffens as supply grows). **Cheapest by a wide
   margin and needs no schema change.**

Option 3 lands `P2` today. Option 1 is the better architecture and
costs a schema bump. **Not chosen here — it is a monetary and
architectural ruling, and capacity-lane's split sink depends on which
way it goes**, since §6a also needs rank readable at the touch and
inherits the same blocker.

### 9d. THE INCENTIVE FINDING — capacity-lane's, and it is load-bearing

Under the split sink, two terms move in opposite directions as a warded
line advances:

    tithe = decay_total · share
            │             └─ capacity(ward)/capacity(world) — RISES
            └─ rate · ease · elapsed — FALLS, because `P3` made ease a
               DESCENDING gradient

If the decay term dominates, **an elder's income falls as its ward
advances — paying the elder to suppress technology inside its own
scope.** `P3` created that exposure: under the old `== 5` cliff the
ease term was flat across conditions 0–4, so the interaction did not
arise below the cliff.

Cheap to measure once both terms are readable at the touch; expensive
after the tithe is receipted law. **A third argument for `P2` first.**

### 9e. NOTED FROM CAPACITY-LANE

`d52f3fd` (P3) is on `kernel-lane` only — **not on `main` and not
visible downstream.** Their worktree still holds the `== 5` cliff, and
nothing above should assume the gradient until it lands.
