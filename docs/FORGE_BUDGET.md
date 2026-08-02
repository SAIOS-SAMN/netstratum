# FORGE BUDGET — the search bound as a capacity reading, not a constant

**Lane:** `remediations-lane`. **Written:** 2026-08-02.
**Status:** DERIVED from operator ruling 2026-08-02 + existing sealed
law. **Awaits ratification** — §7 lists what only the operator can say.
**Supersedes:** `FORGE_REMEDIATION.md` §5's "not proposed: touching
`SEARCH_BUDGET`."

---

## §1 THE RULING (operator, 2026-08-02 — recorded as given)

> `SEARCH_BUDGET` as a constant is awful, this needs to be based on the
> civilizational capacity, not a constant.
>
> consensus is only 1 axiom on a multi-axial coordinated system.
>
> resources should be dynamically produced, if a civ is on 1 planet and
> sharing with another civ, what happens then? why give them "200,000"
> — because it sounds nice? that's not brilliant — you should be setting
> a search minimum and capping this in syntropic state that allows for
> the balance to happen between what needs to happen verse what "is easy
> to have happen".

---

## §2 THE ERROR THIS CORRECTS — mine, and it is the same one twice

`FORGE_REMEDIATION.md` §1 proved the forge search is **consensus-free**
(`verify_work` re-applies the presented program and never enumerates)
and concluded from that alone: *"They change neither what verifies nor
what mints. They do not need Ruling Box 1."*

**Consensus-neutrality is one axis. I treated it as the whole space.**
That is the identical failure mode as the commit before it, where I
reasoned from the *shape* of proof-of-work instead of reading the code:
reach one axis, declare the manifold. The axes a forge change actually
crosses:

| axis | question | constant budget | capacity budget |
|---|---|---|---|
| consensus | does it verify identically everywhere? | yes | yes (`verify_work` unchanged) |
| replay | does live equal replay? | yes (nothing read) | **load-bearing — §6** |
| economic | does it change mint rate? | no | **yes, deliberately** |
| capacity | does the work match what the civ bears? | **no — the defect** | yes, by construction |
| syntropic | does it respect the balance law? | **no — unread** | yes, the cap |
| mesh | is weight earned or assigned? | assigned | earned (`17_CAPACITY` §7) |

The document's own law says this plainly and I walked past it —
`17_CAPACITY` §13d: *"a deterministic universe does not make every
level deterministic. Law is deterministic; the dynamics inside it are
not."* The search's **dynamics** stay free. The **bound** on them is
law, and law is not a magic number.

---

## §3 WHAT IS WRONG WITH 200 000, precisely

`solver/src/search.rs:119`:

```rust
pub const SEARCH_BUDGET: usize = 200_000;
```

Its own comment justifies it by one anecdote — *"measured live: an
unbudgeted rung-5 seed cost 102 s of exhaustive search in one
heartbeat"* — so the number is a **ceiling chosen to stop one observed
stall**, not a reading of anything. Three consequences, all `[READ]`:

1. **It prices every civilization identically.** A ground-state line
   with one orb and a tier-3 elder with a charted lineage get the same
   200 000 candidates. Capacity — the thing the entire `17_CAPACITY`
   ruling exists to measure — is not consulted.
2. **It is inert at live difficulty.** `word_len` 3 enumerates 52 060
   (`FORGE_REMEDIATION.md` §2), so the bound never binds. It is a
   ceiling above the roof: it neither paces nor protects.
3. **It is a placeholder of exactly the class `17_CAPACITY` §7 names
   for retirement by measurement**, beside seat widths
   (`ledger/src/registry.rs:37`), galaxy reach
   (`ledger/src/galaxy.rs:33`) and occupancy caps. It is missing from
   that list; it belongs on it.

---

## §4 THE FORM

> **`17_CAPACITY` §7's method binds here: build the meter, measure, then
> rule — never rule a placeholder blind.** The shape below is derived so
> the meter knows what to measure. The coefficients are deliberately
> left unnamed; naming them before the measurement would repeat the
> 200 000.

```
budget(prior, stamp)  =  clamp( FLOOR,  scale(D)·… ,  ceiling(syntropy) )
```

### 4a. THE FLOOR — the search minimum

**Every line always gets a floor.** Non-negotiable, and not politeness:
without it a ground-state civ can never solve, therefore never mint,
therefore never build capacity, therefore never leave the ground. A
capacity-gated budget with no floor is a poverty trap that the capacity
law would then read as correct.

The floor is what a line receives when **capacity is undefined or at
ground** — §5 is the case that makes this load-bearing, not a corner.

### 4b. THE CAPACITY TERM — and it is a SHAPE, not a scalar

`kernel::bridge::read_entangled(prior)` returns
`EntangledRead { rank, holonomy, symmetry_foundation, flows, unknown }`.
Two existing readings do the work:

- **the gate:** `verdict().is_admitted()` — `Vacuity::Entangled`, i.e.
  `rank > 0 ∧ flow ≠ 0` (`rfa-core/src/boundary.rs:147-193`). This is
  **already** the gate `kernel::capacity_law` uses to admit a
  `DiscoveryMinted`. The same reading that decides whether a mint is
  lawful decides whether a line rises above the floor. One law, two
  callers — never a second bookkeeping.
- **the extent:** `rank` — the count of independent cycles carrying
  charge no frame explains.

**READ IT PER DIMENSION, NEVER ON THE SUM.** `bridge.rs:306-335`
carries the warning explicitly: *"a universe can read `Entangled` on
the sum while no single channel is entangled at all — rank standing in
one channel, flow in another."* `verdict_per_dimension` exists for
exactly this. A budget derived from the aggregate would hand full
resource to a line that is entangled nowhere. **This is the operator's
multi-axial point living inside the capacity reading itself: D is a
shape (`17_CAPACITY` D1, §8), and collapsing it to one number is the
collapse `§2b` already bars.**

### 4c. THE SYNTROPIC CEILING — "what needs to happen" vs "what is easy"

`World::syntropy_ratio()` (`cfe/src/world.rs:3314`) is already built and
already derived-at-call, never stored:

```
syntropic = given_in_crisis + returned_across + covenants_fulfilled + bond_flow
entropic  = extracted_across + covenants_breached + severance_load
ratio     = syntropic / (syntropic + entropic)        # None when no deed posted
```

That numerator **is** "what needs to happen" and that denominator's
second half **is** "what is easy to have happen" — the operator's
distinction is already the engine's own two columns. The ceiling rides
the ratio: an extractive line is capped down toward the floor; a line
carrying its neighbours earns headroom.

**House precedent exists:** the elevation gate already refuses a climb
below `syntropy ≥ 1/2` (PRACTICE-1, `world.rs:1051-1057`,
`docs/09` §2.5). The ceiling is that same alignment law applied to
resource rather than to rank.

`None` (no deed ever posted) must read as the floor, not as zero and
not as full — a fresh line has not yet been measured, and the
refuse-never-guess law says so.

---

## §5 THE SHARING QUESTION — answered from law already sealed

> *"if a civ is on 1 planet and sharing with another civ, what happens
> then?"*

**The law already refuses to guess, and the refusal is the answer.**

`kernel::bridge::read_entangled` is a **single-book** reading. Its own
doc (`bridge.rs:574-590`):

> *"Single-book reading: every pair among the book's own orbs is
> observed, so `unknown` is zero by construction. For a reading that
> spans books, carry the unobserved count through
> `read_entangled_observed`. … While `unknown > 0` the complex has holes
> of unknown shape and `EntangledRead::capacity_is_defined` refuses —
> discharge them by `Resolve::Telescope` or `Resolve::LockOn` before
> reading capacity, **never by assuming absence**."*

So, exactly:

1. **A shared arrangement makes cross-book pairs, so `unknown > 0`, so
   capacity is NOT DEFINED.** The line falls to the **floor** (§4a)
   until the unknowns are discharged. It is never granted a guessed
   capacity, and never assumed to have none.
2. **This closes a real exploit that a naive design would open.** If
   sharing granted each civ a full capacity reading, then two civs on
   one planet would draw **twice** the search resource that one civ
   draws — sharing would *mint budget out of nothing*, and the cheapest
   strategy would be to fragment into shell civilizations. The `unknown`
   refusal prevents that structurally rather than by a rule bolted on
   afterwards.
3. **It gives sharing lines a lawful path, not a penalty.**
   `Resolve::Telescope` (widen the reach and look again) and
   `Resolve::LockOn` (confirm absence at the present reach by
   observation) are the two doors. A shared civ that *discharges its
   reading* gets its capacity honestly and keeps it. The floor is what
   it lives on meanwhile, which is precisely why the floor cannot be
   zero.

**This is the dynamic production the ruling asks for:** resource is not
handed out by a constant, it is *read* — and where it cannot be read,
the system says so and pays the minimum rather than inventing a number.

---

## §6 DETERMINISM — the axis that makes this hard, and its dependency

A capacity-derived budget is read from state, and **the moment a rule
reads state it must read the same state live and on replay.** This is
not hypothetical: it is `kernel-lane` task **#30**, made concrete by its
own P3 revert (`df48786`) — *"the transition is defined twice, in
replay's 113 arms and in the live proposer, and they agree only while
the shared rules stay still."*

Binding constraints on the implementation:

- **Read `prior` committed state, never live workspace.** The budget at
  stamp *t* must be a pure function of the state the deed is judged
  against.
- **Sealed rule, called not copied** (CHARTER §0b) — one computation,
  every caller. The 200 000 is a constant precisely so it could be
  copied safely; a derived bound cannot be.
- **It must ride the ruleset-epoch activation table**
  (`kernel-lane` 11b: stamp → epoch, version derived from the stamp
  already inside the hash, zero wire change). Otherwise changing the
  budget formula later silently rewrites what past beats would have
  done.

**Therefore this work is BLOCKED ON `kernel-lane` 11a/11b landing**, and
must not be designed around them. Relay owed.

One thing that is *not* a constraint, and is worth stating so nobody
adds a guard for it: **`verify_work` never checks how many candidates a
prover tested.** The budget is prover-side. A node with a different
budget still verifies every proof identically. What it changes is which
*tasks get solved*, hence mint rate — which is the economic axis, §7.

---

## §7 WHAT ONLY THE OPERATOR CAN SAY

Recorded unnamed on purpose (§4's warning):

1. **The floor's value.** What is the search minimum — a candidate
   count, or itself a reading (e.g. the ground extent)?
2. **The scale from `rank` to candidates.** Linear, or a ladder like
   `Difficulty::level`? `rank` is a small integer; candidates are
   ~10⁴–10⁵.
3. **The ceiling's shape against syntropy.** Does `ratio ≥ 1/2` open the
   full band (matching the elevation gate), or is it continuous?
4. **Whether the budget also re-prices the MINT.** Today `mint_bound`
   keys off `Difficulty` alone (`kernel/src/lib.rs:845`). If a
   high-capacity line searches more, it solves more of the *same* rung —
   does it earn the same, or does capacity enter the reward too? This
   is Ruling Box 1 territory and this lane proposes nothing.
5. **Per-dimension aggregation.** §4b says read `verdict_per_dimension`.
   Is the budget driven by the *count of entangled channels*, the
   *minimum* across channels (the strictest reading of "witnessed in
   every dimension"), or something else?

---

## §8 WHAT IS OWED BEFORE ANY OF THIS LANDS

`17_CAPACITY` §7: *build the meter, measure, then rule.* The meter is
what this lane can build without an operator word:

1. **THE CAPACITY METER.** Instrument a run and record, per world per
   beat: `verdict_per_dimension`, `rank`, `unknown`, `syntropy_ratio`,
   and the candidates the search actually tested. Zero law change — a
   reading, like `cost_probe`.
2. **THE MEASUREMENT.** Against that, answer with numbers: what is the
   live spread of capacity across a 44-world board? How many lines read
   `unknown > 0`? What does the syntropy ratio actually range over?
3. **THEN the coefficients are ruled against observation**, which is
   how Ruling Box 1 and PROPORTION-1 were both settled, and is the
   opposite of how 200 000 was arrived at.

The measurement also settles a question §3.2 raises and cannot answer:
the bound is inert today, so **the first honest act may be to discover
the budget should be smaller than 52 060, not larger** — a real bound
that paces the forge, rather than a ceiling above the roof.
