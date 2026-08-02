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

---

## §10 `P3` REVERTED — it exposed a live/replay divergence (2026-08-02)

**`P3`'s gradient landed at `d52f3fd` and is reverted at `HEAD`.** It was
committed after `cargo test -p cfe` and `-p kernel` only, without a
workspace sweep. The sweep fails:

    conductor/tests/survey_pins.rs:207
    "replay MATCHES the surveyed world"
      live:   Crystal 127/16
      replay: Crystal  15/16

Confirmed by bisection: restoring the `== 5` cliff turns the pin green,
and the surveyed world holds techs at conditions **0 and 4** — both of
which the cliff treated identically as `1/1` and the gradient does not.

### 10a. THE FINDING IS WORTH MORE THAN THE FEATURE

This is `19_KERNEL` §3 and task **#30** made concrete. The transition is
defined twice — `replay`'s 113 arms and the live proposer — and **they
agree only while the shared rules stay still.** `ease_of` is a sealed
rule both paths call, and the moment its VALUES changed the two paths
produced different worlds from the same receipts.

Nothing was wrong with the gradient's arithmetic. The tests that
exercised it directly all passed, including one that drove it through
`demurrage_due`. **What failed is that a value change propagated
differently through two definitions of the same transition** — which is
precisely the defect #30 exists to close, and it was invisible until a
sealed rule moved.

### 10b. WHAT THIS COSTS EVERY REMAINING PARAMETER

`P1`, `P2` and `P3` all change sealed-rule VALUES. **Each of them will
hit this same wall**, and the order is now forced:

    #30 FIRST — derive apply from judge where the judge already knows,
                pin the pair where it cannot. Until then, any parameter
                amendment is a coin-flip on whether the two definitions
                drift.

The parameter work is not blocked on monetary policy any more. It is
blocked on the kernel having ONE definition of the transition, and that
is a kernel-lane task that was already on the board.

### 10c. PROCESS

`-p cfe` and `-p kernel` are not a sweep. `world.rs` is shared by
`conductor`, `ledger`, `scribe` and `lens`, so a rule change there is a
WORKSPACE change and must be tested as one. Recorded because the
failure was mine and the discipline is cheap.

The attempt is kept at `scratchpad/p2-attempt.patch` (474 lines,
`P2`'s supply threading through 12 call sites) — it built and passed its
own pins, and is worth re-applying after #30 rather than re-deriving.

---

## §11 FORWARD: THE UNIFIED TRANSITION + RULESET EPOCHS (operator, 2026-08-02)

The revert in §10 was the wrong instinct — it restored a STEP FUNCTION,
which is maximally discontinuous, to protect a bifurcated engine. The
operator's ruling is to forward-engineer the engine instead. Two parts,
and the second is cheaper than it looks.

### 11a. APPLY WRAPS JUDGE — one transition, zero parallel logic

`replay`'s 113 arms and the live proposer are two implementations of one
function. **`apply()` becomes a zero-logic wrapper over `judge()`**, so
a rule evaluated one way live is evaluated the identical way on replay.
That is #30 cure 1, and §10 is the proof it can no longer be deferred:
`P1`, `P2` and `P3` all move sealed-rule VALUES, so every one of them
hits the same wall until this lands.

### 11b. RULESET EPOCHS — and **NO WIRE AMENDMENT IS NEEDED**

The ruling asks for a rule-version stamp on the receipt so historical
worlds replay faithfully while new ticks run new physics. **Measured,
the receipt already carries what that requires:**

    Receipt { sequence, stamp, parent, own, event, provenance }
                         ^^^^^                    — vrp.rs:100

There is no `version` field, and adding one would change `own` — the
frame hash — on every receipt ever written, breaking every chain in
existence. **That cost does not have to be paid.** `stamp` is already
present, already monotone, and already inside the hash.

So the version is not stored, it is **derived**:

    an ACTIVATION TABLE maps stamp → ruleset epoch
    every sealed rule takes the stamp it is being evaluated AT
    a receipt is judged under the ruleset in force when it was written

`ease_of(techs, orb, at)` selects the cliff for `at < T₂` and the
gradient for `at ≥ T₂`. Old receipts replay under old rules and cannot
diverge; new ticks run the continuous physics. This is the standard
fork mechanism and it costs **zero wire change, zero schema change, and
zero rehashing.**

### 11c. WHAT THIS RETIRES — my continuity clauses were the wrong fix

§4 required every dynamic parameter to equal its old literal at genesis
*"so the amendment is continuous and no live world jumps."* **Under
ruleset epochs that requirement is unnecessary and is withdrawn.** No
world jumps because no history is ever re-evaluated under new rules —
the old rule keeps running on the receipts it governed.

That frees every parameter to take its correct form rather than a form
bent to meet a legacy value at one point. `P3`'s curve no longer has to
hit `1/1` and `1/2` exactly; `P1`'s feedback term no longer has to
collapse to `Base(rung)` at genesis; `P2` no longer needs `1/32` at zero
supply. **The constraint that shaped three of my translations is gone.**

### 11d. THE ONE COST, STATED PLAINLY

Sealed rules become APPEND-ONLY. A rule is never mutated — a new epoch
is added beside it, and every prior version stays in the tree forever,
because some receipt still needs it. `ease_of` will hold the cliff and
the gradient permanently.

That is how every real chain handles consensus changes and it is the
correct trade, but it changes the shape of a parameter amendment from
*edit a number* to *add an epoch*, and the tree grows monotonically with
the number of rule changes. Worth knowing before the fourth epoch, not
after.

### 11e. ORDER

    1. #30 — apply wraps judge; the activation table; sealed rules
             take `at`. Nothing else can land safely first.
    2. P3 re-inscribed as an epoch (the curve, unconstrained by the
       legacy endpoints).
    3. P2 re-applied from `scratchpad/p2-attempt.patch` — it built and
       passed its own pins; only the epoch plumbing is new.
    4. P1, then capacity-lane §6a's split sink, which inherits the same
       protection.

---

## §12 LANDED — the epoch mechanism, P3 and P2 (2026-08-02)

    65963b4  RULESET EPOCHS + P3 re-inscribed
    39dc3a8  P2 as EPOCH 1

**127 test suites, zero failures**, with both continuous rules live —
including `conductor/tests/survey_pins.rs`, the pin whose live/replay
divergence forced the §10 revert.

### 12a. WHAT THE MECHANISM IS

`cfe::epoch::epoch_at(stamp) -> Genesis | Continuous`, activating at
`CONTINUOUS_FROM = 100_000`. Every sealed rule takes the stamp it is
evaluated AT and dispatches on it. A deed written under EPOCH 0 is
judged by EPOCH 0 forever, so `replay` and the live proposer cannot
disagree even while the rules move.

**Zero wire change, zero schema change, zero rehashing** — the stamp
was already in the receipt and already inside the frame hash.

### 12b. THE TWO EPOCHED RULES

    ease_of(techs, orb, at)
      EPOCH 0  ease_cliff       `== 5` → 1/2, else 1/1
      EPOCH 1  ease_gradient    s = x/(1+|x|), descending, [1/2, 1]

    demurrage_per_tick(supply, at)
      EPOCH 0  sink_flat        1/32
      EPOCH 1  sink_by_supply   min(1/4, 1/32·(1 + excess/33))

Both EPOCH 0 functions are marked **never-edit** in the source: past
receipts depend on them, so changing one would rewrite history's
meaning. This is the append-only cost of §11d, now concrete.

### 12c. WHY `P2` READS SUPPLY AND NOT RANK — forced, not preferred

§9's blocker stands and the epoch mechanism does not dissolve it. Rank
lives behind `bridge::read_entangled` in `kernel`, above `cfe`, so the
proposer structurally cannot read it. `standing_supply` is the sealed
answer: one public function the kernel calls on `prior.orbs` and the
proposer calls on its own, so both judges reach one value from one
state (CHARTER §0b).

### 12d. WHAT IS NOT YET TRUE — stated plainly

1. **`apply()` still does not wrap `judge()`.** #30 part 2 is untouched:
   `replay`'s 113 arms and the live proposer remain two implementations
   of one transition. Epochs remove the DRIFT that value changes cause;
   they do not remove the DUPLICATION. A structural divergence — an arm
   that does something the judge would not admit — is still possible
   and still invisible.
2. **EPOCH 1 has no end-to-end live-vs-replay witness.** Every EPOCH 1
   pin is unit-level, because no test world reaches stamp `100_000` and
   driving one there costs 100k ticks. The mechanism is proven at the
   boundary and by construction; it is not yet proven by a surveyed
   world crossing it. **Landing #30 part 2 makes that guarantee
   structural rather than tested**, which is the better fix than a
   long-running witness.
3. **`P1` is not epoched.** `mint_bound` still reads only the work's
   shape. It is the issuance schedule and the last of the four.

### 12e. FOR CAPACITY-LANE

The split sink can now land without replay drift: `DemurrageSettled`'s
two-sided form becomes EPOCH 1 behaviour, EPOCH 0 keeps the one-sided
booking every existing receipt was written under, and no historical
world is re-judged. The wire amendment for the recipient leg is still
owed — epochs remove the drift risk, not the tag change.

---

## §13 WHY THE PROPOSER CANNOT VERIFY — corrected twice (2026-08-02)

> **§13's first two answers were both wrong.** It first said the arrow
> forbids it (accidental), then that the arrow is forced (architectural).
> The real answer is §13c: an explicit, tested LAW forbids it, and the
> arrow merely encodes that law. Both earlier readings are kept below
> because the sequence is the point — a compile error is not a reason,
> and a dependency graph is not a rationale.

    cfe  ←  physics  ←  solver-verify
    cfe  ←  rfa-core  ←  kernel

`cfe` is the bottom of the workspace arrow. **Every organ that JUDGES
sits above it**, so the proposer can never perform a kernel-grade check
on itself. Measured three separate times, each while trying to fix a
different defect:

    P2's rate      needs rank D      → bridge is in `kernel`      (§9)
    the mint bound needs mint_bound  → was in `kernel`
    the mint proof needs verify_work → solver-verify → physics → cfe

The third was attempted and reverted this session: adding
`solver-verify` to `cfe` is a **cyclic package dependency**, because
`physics` depends on `cfe`. An earlier note read only `solver-verify`'s
direct dependencies and missed it.

### 13a. WHAT THIS SETTLES ABOUT K4

K4's note called the fix *"demoting `world.rs` to a proposer across its
37 `close()` sites"*. That is not a preference — **it is forced.** No
amount of local checking can be added to the emitter, because every
check it would need lives on the wrong side of the arrow. The only
shapes available are:

1. **INVERT THE FLOW.** The emitter returns a PROPOSED deed and applies
   nothing; something above `cfe` verifies and admits. This is K4 as
   written, and the dependency structure is the reason it is the only
   real option.
2. **MOVE THE JUDGMENT DOWN.** Relocate `Difficulty`/`verify_work` (and
   the projection, for standing) beneath `cfe`. That inverts a
   deliberate design — `rfa-core` is GAME-BLIND by charter and
   `solver-verify` is the sealed verifier — so it trades one law for
   another and should not be done to dodge a refactor.

**Option 1, and the 37 sites are the work.** There is no one-commit
version, and the version I scoped as one commit does not compile.

### 13b. WHAT STAYS OPEN MEANWHILE

`live_gate_defect.rs` holds the measurement: the emitter admits a
1,000,000-unit mint against `b"not a proof at all"`, the live projection
hides it, and `replay` grants it. Nothing in this section closes that —
it explains why the closure is larger than one call site.


### 13c. THE OPAQUE-PAYLOAD LAW — the actual reason

`kernel/tests/recurrence_guard.rs::the_arrow_law_holds` states it:

    // Nothing imports upward into the kernel, and cfe never imports
    // the verifier or the codec at runtime (the opaque-payload law).
    let cfe_forbidden = ["solver-verify", "kernel", "codec"];

**`cfe` may not parse a work payload.** The game carries proofs as
opaque bytes and only the sealed verifier reads them; if the proposer
could interpret proof structure, the seal would be gone and the game
would know what a proof is made of.

So the emitter cannot verify — **by charter, not by accident.** The fix
attempted here (`cfe` → `solver-verify`, `mint_bound` moved down, the
emitter calling `verify_work`) built and passed its own tests and was
REVERTED: the guard caught it, which is what the guard is for.

**K4 stays the inversion.** The emitter proposes; something above `cfe`
verifies and admits. That was K4's original wording and it is right for
a reason neither earlier reading of §13 had.

### 13d. WHAT SURVIVED — the `ratio` leaf

`cfe::ratio` was 83 self-contained lines, and `physics` imported them
for `Ratio` alone. That single edge made `physics → cfe`, hence
`solver-verify → physics → cfe`, and put the state-mutating crate
beneath the sealed math tier for no stated reason.

Extracted to `engine/ratio` — a leaf with no internal dependencies.
`cfe` re-exports it so no caller moved. This removes an ACCIDENTAL
inversion while leaving the DELIBERATE one (§13c) exactly as charter
requires. 142 suites green.

---

## §14 THE STANDING DEADLOCK — `capacity_law` refuses every mint (verified 2026-08-02)

remediations-lane measured `MintWithoutStanding{Void}` 61 then 68 times
per 10 beats on the live board. **Verified here rather than accepted**
(`kernel/tests/standing_reality.rs`), because it is a verdict on this
lane's own invariant:

    orbs=1  rank=0  verdict=Void      -> is_admitted() == false

### 14a. IT IS A BOOTSTRAP DEADLOCK, not a tuning problem

`capacity_law` admits only `Vacuity::Entangled`, which needs `D > 0`.
`D` counts independent cycles in the obligation complex. A cycle needs
relations; relations are bonds, covenants and debts; those are made by
spending value. **So a world must mint to relate, and must relate to
mint.** A board of 44 books with one orb each has no obligations at
all, so it can never leave rank 0 by its own action.

Flipping Phase 3 today would halt minting entirely — the relay's
warning is exactly right, and this is why.

### 14b. THE DISTINCTION THE VERDICT IS MISSING

`Void` is returned both for *a world with no relations yet* and for *a
world whose relations carry nothing*. Those are not the same state. The
first is unformed and ought to be able to start; the second is
genuinely vacuous and is what the law was written to refuse. Collapsing
them is what turns a forgery gate into a bootstrap lock.

### 14c. WHAT IS OWED — an operator ruling, not a coder choice

**May a world with no standing obligations mint at all?** Three shapes,
and this lane will not pick one:

1. **A GENESIS WINDOW.** Early mints are exempt, the way
   `founding_reserves` already creates value without work (P4). Bounded
   by receipt count or stamp, so the exemption cannot be farmed.
2. **SPLIT THE VERDICT.** `Void` for relations-that-carry-nothing,
   something else for no-relations-yet, and admit only the latter.
   Requires no new state — the projection already knows which it is.
3. **DIFFERENT STANDING FOR THE FIRST RUNG.** Rungs 0–2 pay zero today
   (§2), so an unstanding world could still be allowed to solve them —
   earning nothing but forming the relations that later mints need.

Option 3 is the only one that costs no exemption and no new verdict:
the ladder's unpaid rungs become the bootstrap path. **Flagged as the
cheapest, not chosen.**

**Until this is ruled, K4's Phase 3 must not flip.** The gate would
work exactly as designed and stop the economy.

---

## §15 NEW VERDICT — V IS ALREADY IN THE LAW, AND THE PROJECTION MISREADS IT

**§14's diagnosis is withdrawn.** It said the verdict collapses two
states and needs a new one. Measured, the law already has both.

    boundary.rs:180
    dual_pole(rank, form) = match (rank > 0, form.is_non_vacuous()) {
        (false, false) => Void,        // no structure, NO POTENTIAL
        (false, true)  => Unframed,    // no structure, POTENTIAL STANDS
        (true,  false) => Stasis,
        (true,  true)  => Entangled,
    }

**`form.is_non_vacuous()` IS `V`.** The ruling's requirement — *add V to
the equation* — is already satisfied. And `Unframed` is exactly the
state the ruling names `Void::Uninitialized`: structure absent,
potential present. **No new variant is needed; the existing one is
unreachable.**

### 15a. THE ACTUAL DEFECT — measured

    orbs=1  rank=0  verdict=Void
    V side: flows = 0
    banked entries the world ACTUALLY holds: 2   (Iron 20, Biomass 10)

The world holds real potential and the verdict reads none, so the
`Unframed` row can never be taken at genesis and every founded world
falls to `Void`.

**Because `V` is projected from the SAME EDGES as `D`.** `boundary_form`
reads the obligation complex — bonds, covenants, debts — and so does
the rank. At genesis there are no obligations, so both are zero
together, by construction, always.

**That breaks §2f.** The entanglement law requires two poles *witnessed
in every dimension*. Two readings of one source are not two poles: they
cannot disagree, so the four-row table degenerates to two rows and the
Unframed and Stasis rows are dead.

### 15b. THE FIX — no new state, no exemption, no bootstrap window

**`V` must read standing POTENTIAL, not standing obligation.** A
founded world holding `Iron 20 · Biomass 10` has potential and no
structure — that is `Unframed` on the table as written. Banked reserves
are the potential the seed creates (P4), and they are already in
`Committed`.

Then, with no law changed:

    genesis           Unframed   potential, no structure
    obligations form  Entangled  both poles stand, the mint admits
    drained world     Stasis     structure, no potential
    nothing at all    Void       correctly refused

§14's three shapes are all withdrawn. A genesis window, a split verdict
and a bootstrap rung were three ways to work around a misprojection.

### 15c. THE MERGE IS REFUSED, and why

The ruling requires verifying **`D > 0` with `rank = 0`** before
merging. **That is unsatisfiable on this branch**, because here `D` IS
`rank` — `capacity_law` reads `read.rank` and nothing distinguishes
them.

It may well be satisfiable on `main`: lens-lane landed *"D IS A
DIMENSION — the charge field carried whole, never counted"* and *"D IS
POLYTONIC, and the floor is OCCUPANCY not the ground"*, which is
precisely the separation the condition assumes. **Refusing the merge
until that is measured rather than assumed** — merging first and
checking after is how the last three defects reached the tree.

**What must be measured on `main` before any merge:**

1. Does an `EntangledRead` there expose a `D` distinct from `rank`?
2. Does `capacity_law` read that `D`, or still `rank`?
3. On a founded one-orb world, what is the verdict?

If (3) is `Unframed`, the deadlock is already closed on `main` and this
section is the explanation rather than the fix. If it is still `Void`,
§15b is the fix and it belongs in the projection.
