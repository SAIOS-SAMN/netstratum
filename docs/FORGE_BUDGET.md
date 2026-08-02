# FORGE BUDGET — the search bound as a capacity reading, not a constant

**Lane:** `remediations-lane`. **Written:** 2026-08-02, amended same day
(named coefficients + axial capabilities, operator ruling).
**Status:** DRAFT for ratification. Rows **B0–B8** claim their place in
`EQUATIONS.md` §7 under the row-claim convention (§0.2).
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

And on this file's first draft:

> coefficients need names: this is an inefficiency that requires
> remediations — provide axial capabilities and provide summarization

**That correction is upheld and the first draft was wrong.** It left
every coefficient unnamed, reasoning that naming them would repeat the
200 000. Naming is not valuing. A **named** coefficient with an
unmeasured value is a declared unknown: referenceable, amendable by
ruling, versionable by epoch, and impossible to inline by accident. An
**unnamed** one is a hole that the first implementer fills with a
literal — which is exactly the causal history of 200 000. §4 names them;
§7 still refuses to value them.

---

## §2 THE ERROR THIS CORRECTS — mine, and it is the same one three times

`FORGE_REMEDIATION.md` §1 proved the forge search is **consensus-free**
and concluded: *"They change neither what verifies nor what mints. They
do not need Ruling Box 1."* Consensus-neutrality is one axis; I treated
it as the whole space — the same failure as the commit before it, where
I reasoned from the *shape* of proof-of-work instead of reading the
code. Reach one axis, declare the manifold.

The system's own law says it: `17_CAPACITY` §13d — *"a deterministic
universe does not make every level deterministic. Law is deterministic;
the dynamics inside it are not."* §9 is the standing cure: the axes as
capabilities, each with its own reading and its own refusal, so no
future landing can satisfy one and claim the rest.

---

## §3 WHAT IS WRONG WITH 200 000, precisely

`solver/src/search.rs:119` — `pub const SEARCH_BUDGET: usize = 200_000;`

Its own comment justifies it by one anecdote: *"measured live: an
unbudgeted rung-5 seed cost 102 s of exhaustive search in one
heartbeat."* It is a ceiling chosen to stop one observed stall. Three
consequences, all `[READ]`:

1. **It prices every civilization identically.** A ground-state line
   with one orb and a tier-3 elder draw the same 200 000.
2. **It is inert at live difficulty.** `word_len` 3 enumerates 52 060,
   so the bound never binds — a ceiling above the roof, pacing nothing.
3. **It belongs on `17_CAPACITY` §7's retire-by-measurement list**
   beside seat widths (`ledger/src/registry.rs:37`) and galaxy reach
   (`ledger/src/galaxy.rs:33`), and is missing from it.

---

## §4 THE NAMED COEFFICIENTS — rows B0–B8

> **Chapter four's law binds every row** (`EQUATIONS` §0.3): state what
> the coefficient **measures**, never what it is worth. A name is a
> handle; a value is a ruling (§7).

| row | symbol | measures | domain | status |
|---|---|---|---|---|
| **B0** | `SEARCH_BUDGET` | — superseded — the flat 200 000 at `search.rs:119` | ℕ⁺ | **PLACEHOLDER, struck by this law** |
| **B1** | `β_floor` | the candidates a line receives when its capacity is undefined or stands at ground — **the search minimum** | ℕ⁺ | DRAFT · unvalued |
| **B2** | `ρ_D` | how the per-dimension capacity verdict yields ONE extent **without collapsing the shape** | a named reading, not a number | DRAFT · unchosen |
| **B3** | `β_quantum` | candidates granted per unit of `ρ_D` | ℚ⁺ | DRAFT · unvalued |
| **B4** | `α_axis` | per-axis syntropic standing — compassion · love · integrity · morality — **never summed** | ℚ⁴ ∩ [0,1]⁴ | DRAFT · unbuilt |
| **B5** | `σ_gate` | the per-axis ratio at or above which that axis opens its band | ℚ⁴ ∩ [0,1]⁴ | DRAFT · unvalued |
| **B6** | `β_ceiling` | how the alignment vector becomes one multiplier — order-preserving, **never a sum** | [0,1]⁴ → [0,1] | DRAFT · unchosen |
| **B7** | `β_cap` | the candidates no line exceeds at any capacity — the latency protection 200 000 was reaching for | ℕ⁺ | DRAFT · unvalued |
| **B8** | `ε_unknown` | what a line whose capacity reading spans books receives | ℕ⁺ | DRAFT · proposed `≡ β_floor` |

### B-E1 · THE BUDGET EQUATION

```
unknown(prior) > 0   ⇒   budget = ε_unknown                        (B8)

otherwise
budget(prior, t) = min( β_cap ,
                        max( β_floor ,
                             ⌊ β_quantum · ρ_D(prior) · β_ceiling(α_axis(prior)) ⌋ ) )
```

Integer by construction (a candidate count). `β_floor ≤ β_cap` is a
standing obligation on the ruling, not an assertion here.

### 4a. B1 `β_floor` — why a floor is structural, not politeness

Without it a ground-state line can never solve → never mint → never
build capacity → never leave the ground, **and the capacity law would
then read that trap as correct.** The floor is the term that keeps the
capacity reading from being self-confirming.

### 4b. B2 `ρ_D` — read per dimension, never the aggregate

`kernel::bridge::read_entangled(prior)` yields
`EntangledRead { rank, holonomy, symmetry_foundation, flows, unknown }`.
The gate is `verdict().is_admitted()` — `Vacuity::Entangled`, i.e.
`rank > 0 ∧ flow ≠ 0` (`rfa-core/src/boundary.rs:147-193`) — **the same
reading `kernel::capacity_law` already uses to admit a
`DiscoveryMinted`.** One law, two callers; never a second bookkeeping.

**`bridge.rs:306-335` carries the warning that decides B2:** *"a
universe can read `Entangled` on the sum while no single channel is
entangled at all — rank standing in one channel, flow in another."*
`verdict_per_dimension` exists for exactly this. A budget from the
aggregate would hand full resource to a line entangled nowhere.
Candidate readings for the ruling: the **count** of channels reading
`Entangled`; the **minimum** rank across them (strictest — matches
*"witnessed in every dimension"*); or the **sum** of rank over entangled
channels only.

### 4c. B4/B5/B6 `α_axis` — FOUR AXES, and why the obvious reading is barred

**The first draft proposed riding `World::syntropy_ratio()`. That is
barred, and the bar is already recorded.** `EQUATIONS` §6 **S2** carries
the live defect:

> *"`syntropy_ratio` collapses all four axes into one scalar and TWO
> rules read it"* — against `09` §1.3's ruling that *"no oracle, no
> single number … the four axes never collapse into one figure"* and
> §3.4.2's refusal of *"the single goodness number."* *"As written the
> document and the code contradict."*

A budget ceiling riding that ratio would **inherit an unresolved
contradiction and add a third reader to it.** So `α_axis` reads the four
named axes of `09` §2 directly, each already computable from committed
orb fields:

| axis | `09` § | syntropic term | entropic term |
|---|---|---|---|
| **compassion** — absorption | 2.1 | `given_in_crisis` | *(none in the current sum)* |
| **love** — resonance | 2.2 | `bond_flow` | `severance_load` |
| **integrity** — fidelity | 2.3 | `covenants_fulfilled` | `covenants_breached` |
| **morality** — reciprocity | 2.4 | `returned_across` | `extracted_across` |

Morality's ratio is **already built and already sealed** —
`reciprocity_reading` (`cfe/src/world.rs:4905`) is exactly
`returned/(returned+extracted)`. Call it; never write a second.

**Compassion has no entropic counterpart in the present sum**, so its
ratio is not yet well-formed. Named here as an open row rather than
papered over — §7.6.

`β_ceiling` must be **order-preserving and must not be a sum**: a sum is
the collapse `09` §1.3 refuses, re-entering through the ceiling. The
minimum across axes is the reading that respects the refusal (a line is
capped by its weakest standing), and it is the natural mirror of B2's
strictest option.

**The symmetry worth naming:** both sides of B-E1 are multi-axial, and
both have a recorded defect from collapsing — capacity across `CHANNELS`
(`bridge.rs:306`), alignment across the four axes (`EQUATIONS` §6 S2).
The operator's *"multi-axial coordinated system"* is not a framing
placed on this law from outside; it is the law's own shape, twice.

---

## §5 THE SHARING QUESTION — answered from law already sealed

> *"if a civ is on 1 planet and sharing with another civ, what happens
> then?"*

**The law already refuses to guess, and the refusal is the answer.**
`read_entangled` is single-book (`bridge.rs:574-590`):

> *"every pair among the book's own orbs is observed, so `unknown` is
> zero by construction. For a reading that spans books, carry the
> unobserved count through `read_entangled_observed`. … While
> `unknown > 0` … `capacity_is_defined` refuses — discharge them by
> `Resolve::Telescope` or `Resolve::LockOn` before reading capacity,
> **never by assuming absence**."*

1. **Sharing makes cross-book pairs → `unknown > 0` → capacity is NOT
   DEFINED → the line draws `ε_unknown` (B8)** until discharged. Never a
   guessed capacity; never an assumed absence.
2. **This closes an exploit a naive design opens.** If sharing granted
   each civ a full reading, two civs on one planet would draw **twice**
   what one draws — sharing would mint budget from nothing, and the
   cheapest strategy would be fragmenting into shell civilizations. The
   `unknown` refusal prevents it structurally, not by a bolted-on rule.
3. **It gives sharing lines a lawful path, not a penalty.**
   `Resolve::Telescope` (widen the reach, look again) and
   `Resolve::LockOn` (confirm absence by observation) are the doors. The
   floor is what a line lives on meanwhile — which is precisely why
   `β_floor` cannot be zero.

**This is the dynamic production the ruling asks for:** resource is
*read*, and where it cannot be read the system says so and pays the
minimum rather than inventing a number.

---

## §6 DETERMINISM — the axis that makes this hard

The moment a rule reads state it must read the same state live and on
replay. That is `kernel-lane` **#30**, made concrete by its own P3
revert (`df48786`): *"the transition is defined twice, in replay's 113
arms and in the live proposer, and they agree only while the shared
rules stay still."*

- **Read `prior` committed state, never live workspace.**
- **Sealed rule, called not copied** (CHARTER §0b). The 200 000 was a
  constant precisely so copying it was safe; a derived bound is not.
- **Ride the ruleset-epoch activation table** (`kernel-lane` 11b:
  stamp → epoch, derived from the stamp already inside the hash, zero
  wire change) — else amending B1–B8 later silently rewrites what past
  beats would have done.

**BLOCKED ON `kernel-lane` 11a/11b.** Relay owed.

Not a constraint, stated so nobody guards it: **`verify_work` never
checks how many candidates a prover tested.** The budget is prover-side;
a node with a different budget verifies every proof identically. What it
changes is which tasks get solved — the economic axis, A3.

---

## §7 WHAT ONLY THE OPERATOR CAN SAY — the rows awaiting a value

1. **B1 `β_floor`** — a candidate count, or itself a reading (the ground
   extent)?
2. **B2 `ρ_D`** — count of entangled channels, minimum rank across them,
   or sum over entangled channels only?
3. **B3 `β_quantum`** — `rank` is a small integer; candidates are
   10⁴–10⁵. Linear, or a ladder like `Difficulty::level`?
4. **B5 `σ_gate` / B6 `β_ceiling`** — per-axis thresholds or one shared?
   Step (matching the climb gate's 1/2) or continuous? Minimum across
   axes, or another order-preserving non-sum?
5. **B7 `β_cap`** — and note §8's likely finding: the honest cap may be
   **below** 52 060, making the bound real for the first time.
6. **B4 compassion's entropic term** — §4c: compassion has a syntropic
   term and no entropic counterpart, so its ratio is not well-formed. Is
   the counterpart `extracted_across` shared with morality, a new
   committed field, or is compassion read as a level rather than a
   ratio?
7. **Does capacity re-price the MINT?** Today `mint_bound` keys off
   `Difficulty` alone (`kernel/src/lib.rs:845`). If a high-capacity line
   searches more, it solves more of the *same* rung — same reward, or
   does capacity enter it? Ruling Box 1 territory; this lane proposes
   nothing.

---

## §8 THE METER IS BUILT, AND IT REFUSES THE RULING — measured 2026-08-02

`17_CAPACITY` §7 said *build the meter, measure, then rule.* The meter
is `engine/conductor/examples/capacity_meter.rs` (a reading — appends
nothing, judges nothing, changes no law). **It paid for itself on the
first run, and what it found stops B1–B8 from being instantiated.**

Measured across the same 44-world, 80-cycle board as `REMEDIATIONS` §1:

| reading | result across all 44 lines |
|---|---|
| orbs per book | **1** — every book |
| bonds · covenants · debts | **0 · 0 · 0** — every book |
| `rank` | **0** — every book |
| `degree` | `Degree(1)` — GROUND, every book |
| entangled channels (of 3) | **0** — every book |
| `unknown` | 0 — every book |
| love · integrity · morality | **— · — · —** (nothing posted), every book |
| compassion level | **0** — every book |

### 8a. WHY THE ZERO IS STRUCTURAL, NOT LOW

`project_complex` (`bridge.rs:373-381`) builds the complex over **the
book's OWN orbs**, edged by bonds, covenants and debts, and returns no
structure at all when `dim < 2` — *"One orb relates to nothing."*

**Every book has exactly one orb.** So `rank = 0` here is not a small
reading, it is a **structurally unreachable** one: no arrangement of
play on the current board can raise it, because the relation the
reading is made of cannot exist inside a single-orb book.

### 8b. THREE CONSEQUENCES, each load-bearing

1. **`kernel::capacity_law` can never admit a `DiscoveryMinted` on this
   board.** It gates on `Vacuity::Entangled` (`rank > 0 ∧ flow ≠ 0`) and
   **nothing is entangled anywhere.** Mints occur today only because the
   judge runs BESIDE the chain, not inside it — `extend(.., None)` at
   `conductor/src/lib.rs:1403` (PICKUP §8a's narrow remainder).
   **Therefore flipping Phase 3 today would halt minting entirely.**
   That is a Phase-3 precondition nobody has recorded, and it is found
   here by measurement rather than by argument.
2. **The ruled envelope evaluates to `β_floor` for every line, forever.**
   With `ρ_D = 0` universally, `β_floor + β_quantum · ρ_D^D` **is**
   `β_floor`. Executing the ruling on this board replaces one constant
   (200 000) with another constant (`β_floor`) and calls it a reading.
   The form is right; the substrate is absent.
3. **B4/B5/B6 have no signal either.** No alignment axis has posted on
   any line, so the ceiling is undefined for all 44 — `None`, which by
   refuse-never-guess reads as the floor, again.

### 8c. WHAT THIS MAKES THE NEXT WORK

The budget cannot become a reading until the reading becomes reachable.
Two doors, and the choice is the operator's:

- **(a) MULTI-ORB PLAY.** The conductor never settles a second orb, so
  no intra-book relation is possible. Capacity becomes reachable the
  moment a line holds two orbs that bond, covenant or owe.
- **(b) THE CROSS-BOOK DOOR.** Cross-world APERTURE deals *do* clear
  (the market is live), so relations exist — they are just not in one
  book. `read_entangled_observed` + discharging `unknown` via
  `Telescope`/`LockOn` is exactly the path, and it is the **same door
  B8 was written for**. The sharing case and the reachability case turn
  out to be one question.

**Until one lands, `SEARCH_BUDGET` should not be replaced.** Swapping a
measured-inert constant for an unmeasurable one is not remediation.

---

## §8bis THE RULING, AMENDED — depth is the resource, and the constants dissolve

**Operator ruling 2026-08-02 (the Ruling Box) recorded as given, then
amended on the operator's instruction (*"change the ruling"*) against
the meter's readings.** The received envelope was:

```
budget = β_floor + β_quantum · ρ_D^D                    β_cap ≤ 52 060 (B7)
```

**Its instinct is right and is kept:** `ρ_D^D` has *the same shape as
the space it bounds* — enumeration is `|V|^depth` — and the floor is
ADDED, not clamped, so it cannot be competed away. The amendment carries
both properties and fixes what the code refuses.

### 8bis.a THE AMENDMENT — bound the DEPTH, and the sum follows

The search is breadth-first **by depth** (`search.rs:140`). So the
lawful bound is on depth, and the candidate count is its consequence:

```
budget(prior, rung) = frontier( |V|, δ(prior, rung) )

    frontier(V, δ) = Σ_{d=0}^{δ} V^d          the exhaustive sweep to depth δ
```

**Every constant dissolves into arithmetic.** Measured by the meter:

| \|V\| | δ=0 | δ=1 | δ=2 | δ=3 |
|---|---|---|---|---|
| 14 | 1 | 15 | 211 | 2 955 |
| 21 | 1 | 22 | 463 | 9 724 |
| 30 | 1 | 31 | 931 | 27 931 |
| 37 | 1 | **38** | 1 407 | **52 060** |

**`frontier(37, 3) = 52 060` — exactly the bound the Ruling Box named
for `β_cap`.** The operator's number was already the derived quantity;
it simply had not been written as one. `β_cap` is therefore not chosen
at all: **it is the exhaustive sweep at the rung's own `word_len`.**

### 8bis.b THE DEPTH LADDER

```
δ(prior, rung) = min( word_len(rung) ,  1 + earned(prior) )     capped by alignment (8bis.d)
                 └─ B7, derived ─┘      └─ B1 ─┘ └─ B2/B3 ─┘

unknown(prior) > 0   ⇒   δ = 1                                              (B8)
```

- **B1 `δ_floor = 1`, unconditionally.** Every line may always sweep the
  whole vocabulary once. Derived value `1 + |V|` ∈ [15, 38] — not a
  chosen number. **Its trigger is `rank == 0`, never `D == 0`:** `D` is
  `GROUND + rank` and is never zero (`bridge.rs:288`,
  `capacity.rs:138-146`; the field doc: *"Reading this field as `D` is
  what put `D = 0` back into a system whose law says D is never zero."*).
  Measured: all 44 lines read `rank 0`, `Degree(1)`.
- **B3 `β_quantum` = ONE DEPTH PER EARNED CHANNEL.** The quantum is not
  a candidate multiplier — it is a depth increment, and the exponential
  is supplied by `frontier` rather than by the coefficient.

### 8bis.c B2 `ρ_D` — BOTH POLES AND THE CHARGE, never rank alone

The received `ρ_D^D` keyed on rank alone, which re-commits the
operator's own ruling of the same day (`bridge.rs:176-199`): *"your
entire definition is wrong if dimensional capacity is a count of
rooms."* Rank is the DIMENSION of the cycle space; `holonomy` carries
each cycle's EXACT CHARGE; *"a universe with one heavily-charged cycle
and one with a single faint one read the SAME rank and are not the same
universe."* Amended:

```
earned(prior) = # channels l ∈ 0..CHANNELS such that
                  verdict_per_dimension(prior)[l] == Entangled     (both poles)
                  AND  max |holonomy value in l|  ≥  θ_charge      (the volume)
```

Per channel (never the aggregate — `bridge.rs:306`), both poles (never
`Entangled` inferred from rank — `boundary.rs:147`), and the charge read
as a magnitude (never replaced by its count — `18` §2b). **`θ_charge` is
the one genuinely new coefficient the amendment introduces, and it is a
VOLUME threshold — precisely the term the received ruling was missing.**

### 8bis.d B4/B5/B6 — alignment enters the arithmetic, as a CAP on depth

The received box had no place for `β_ceiling`, so axis **A5 did not
enter the arithmetic at all** — dropping the axis the original ruling
asked for. Amended: alignment caps the depth.

```
β_ceiling(α_axis) = word_len(rung)   if  weakest(α_axis) ≥ σ_gate
                  = 1                otherwise          (the no-mint depth)

weakest(α) = min over the axes that have POSTED — never a sum, never an average
```

`min` is the order-preserving non-sum B6 requires: **a line is capped by
its weakest standing**, which is the natural mirror of B2's per-channel
strictness. An axis that has posted nothing is not counted as zero
(refuse-never-guess); a line with no axis posted reads `weakest = None`
and takes the floor.

### 8bis.e WHY THIS IS ECONOMICALLY COHERENT — the check the received box could not make

Reading the depth ladder against `Difficulty::level` and `mint_bound`:

| δ | reachable rung | `word_len` | `mint_bound` | requires |
|---|---|---|---|---|
| 1 | levels 0–1 | 1 | **0** | nothing — the floor |
| 2 | levels 2–4 | 2 | 0 · 1 · 4 | **1 earned channel** |
| 3 | level 5 | 3 | **7** | **2 earned channels** |

- **No poverty trap:** a ground line can always climb rung 1.
- **No free mint:** minting begins at δ = 2, which requires at least one
  channel that is `Entangled` *and* carries real charge.
- **And that is the SAME condition `kernel::capacity_law` already
  enforces** on `DiscoveryMinted` (`Vacuity::Entangled`). One law, two
  enforcements, agreeing — the budget stops being a second bookkeeping
  and becomes the same gate read earlier.
- `CHANNELS = 3` gives δ ≤ 4, and `word_len` ≤ 3 caps it: the third
  channel is headroom for rungs not yet forged.

### 8bis.f WHAT REMAINS OWED

| row | state after amendment |
|---|---|
| B1 `δ_floor` | **DERIVED = 1** (`1 + \|V\|` candidates). Trigger corrected to `rank == 0`. |
| B2 `ρ_D` | **FORM SETTLED** — per-channel, both poles, charge magnitude. |
| B3 `β_quantum` | **DERIVED = 1 depth per earned channel.** |
| B7 `β_cap` | **DERIVED = `frontier(\|V\|, word_len)`.** Not a constant. |
| B8 `ε_unknown` | **DERIVED = `δ_floor`.** |
| B4 `α_axis` | form settled; **compassion's denominator still open** (§7.6) |
| B5 `σ_gate` | **STILL OWED** — absent from the received box; one rational in [0,1]. Precedent: the climb gate's 1/2. |
| **θ_charge** | **NEW, OWED** — the volume threshold. Unmeasurable today (§8: all charge is empty). |

**Two numbers, not eight.** Everything else is now arithmetic on `|V|`
and the rung. And both remaining numbers are **unmeasurable on the
current board** — §8's finding stands: no line carries charge or a
posted axis, so `θ_charge` and `σ_gate` cannot be ruled against
observation until reachability lands (task #15).

**The pattern across all three collapses now recorded** — aggregate over
channels, four axes into one scalar, charge into a count — is one law
stated three times: *a directional quantity may not be replaced by a
scalar aggregate wherever the system computes.* §9 is the standing cure.

---

## §9 AXIAL CAPABILITIES

Six axes. Each is a **capability** — a thing the system can read,
decide and refuse — not a checkbox. A landing satisfies an axis only by
naming which capability it exercised. **Satisfying one is not
satisfying the manifold**; §2 is what that error cost twice.

### A1 · CONSENSUS — *does it verify identically everywhere?*
- **reads** the presented `WorkProof` — nothing else
- **decides** `Ok(Difficulty)` or refuses
- **refuses** `WorkBroken::{Malformed, TrainingMiss, OutputMismatch, ChainDiverged}`
- **owns** no coefficient — **parameter-free by design, and that is the property**
- **authority** `solver_verify::verify_work` (`work.rs:266`) · **LANDED**
- **budget's crossing:** none. The search is prover-side; `verify_work`
  never enumerates a candidate.

### A2 · REPLAY — *does live equal replay?*
- **reads** `prior` committed state at a stamp
- **decides** whether a sealed rule yields one value for both paths
- **refuses** by divergence — silently, which is why it needs a guard
- **owns** the ruleset-epoch boundary (stamp → epoch)
- **authority** `kernel-lane` #30 / 11a / 11b · **IN FLIGHT, BLOCKING**
- **budget's crossing:** total. B-E1 reads state, so it cannot land
  before this axis holds.

### A3 · ECONOMIC — *does it change what mints, or how much?*
- **reads** `Difficulty`
- **decides** `mint_bound` — the rung→reward table (0 · 0 · 0 · 1 · 4 · 7)
- **refuses** `Unlawful::MintWithoutStanding`
- **owns** the rung→reward table — **Ruling Box 1, untouched here**
- **authority** `kernel::mint_bound` (`kernel/src/lib.rs:845`) · **LANDED**
- **budget's crossing:** indirect but real. Budget changes *which tasks
  get solved*, hence mint frequency — never mint *amount*.

### A4 · CAPACITY — *does the work match what the line can bear?*
- **reads** `verdict_per_dimension`, `rank`, `unknown` — **per channel**
- **decides** floor versus scaled band
- **refuses** by `unknown > 0` — capacity undefined, discharge by
  `Telescope` or `LockOn`, never by assuming absence
- **owns** **B1 `β_floor` · B2 `ρ_D` · B3 `β_quantum` · B8 `ε_unknown`**
- **authority** `kernel::bridge` · `17_CAPACITY` · **LANDED (unread by the forge)**
- **budget's crossing:** this axis *is* the ruling. It is the axis the
  flat constant never consulted at all.

### A5 · SYNTROPIC — *does it respect the balance law?*
- **reads** the four named axes of `09` §2 — compassion · love ·
  integrity · morality — **as a vector**
- **decides** the ceiling multiplier
- **refuses** collapse to one scalar (`09` §1.3, §3.4.2)
- **owns** **B4 `α_axis` · B5 `σ_gate` · B6 `β_ceiling`**
- **authority** `09_IDEOLOGY` §2 · `world::reciprocity_reading`
  (morality, LANDED) · **the other three axes UNBUILT as ratios**
- **budget's crossing:** the ceiling. Carries `EQUATIONS` §6 **S2** as an
  inherited open defect — see §4c; do not add a third reader of the
  collapsed ratio.

### A6 · MESH-WEIGHT — *is standing earned or assigned?*
- **reads** verified capacity contributed
- **decides** peer weight in `Σ_mesh`
- **refuses** — **nothing today; weight arrives assigned and advisory**
- **owns** the weight map
- **authority** `mesh/src/agree.rs:57` · `SYSTEM_SPEC` §5 ·
  **THE OPEN GAP** (`17_CAPACITY` §7 names capacity as its honest basis)
- **budget's crossing:** shared basis. A capacity-derived budget and an
  earned mesh weight read the *same* meter; building §8's meter serves
  both, and neither should build a second.
