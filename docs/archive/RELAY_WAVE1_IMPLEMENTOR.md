# RELAY — TO THE WAVE-1 IMPLEMENTOR NODE

> **ARCHIVED 2026-08-13.** Superseded/dead weight per the doc audit — kept for history, not live spec. See docs/BUILD_ORDER.md, docs/archive/HANDOFF_2026-08-13.md, or the doc it names as its successor for current status.

> **LANDED `2109bf0` (2026-08-07).** Wave-1 exit met: stations T4–T9,
> NPC T4/T6 seats, REFINE-1 + pure-NPC refine; harness green for those
> arcs. **Do not re-implement.** Standing law below remains authority for
> later waves. Next brief: `RELAY_WAVE2_IMPLEMENTOR.md`.

You are building the first vertical slice of the tier/rail/market arc
(`TASKS_TIER_RAIL_MARKET.md`). This brief is the accumulated law you
need that the task DAG does not carry. Read it before you write a line;
most of it exists because something already went wrong.

---

## 1. WHAT YOU ARE BUILDING, AND HOW YOU KNOW YOU ARE DONE

The slice: **stations T4–T9 → REFINE-1 + NPC build → element aperture +
route + NPC trade → STAMP spot + NPC market makers → R1–R2 only.**
Perps, R3 and R4 are explicitly out.

**The done bar is a pure-NPC era:** refined elements exist, a route
clears, and a STAMP spot clears, with **zero player actions**.

**Build that harness FIRST, red.** Not last. Every wave then turns part
of it green, and you always know what you have. The reason is measured,
not stylistic: this codebase has twice produced features that passed
every test and were wrong the moment a person drove them.

- `World::bench` listed every set of elements that would bond. Green
  board. In live play *every* set it offered was refused by the place
  check, because a compound shares one phase and an orb sits in one
  thermal band. A green trap.
- The vein wear law's first cut passed its tests and let one
  regenerator saturate the cap in a single tick, collapsing the whole
  ecology to a binary.

A green board proves **lawful**. It does not prove **playable**. The
NPC harness is what proves playable, so it comes first.

---

## 2. THE ORDER, AND WHY IT IS THIS ORDER

1. **Claim registers** (§3). Before any code.
2. **Done-bar harness, red.**
3. **T1.1 + T1.3** — stations T4–T9, seat widths, `NoSeatOpen`.
4. **T1.4 — the NPC path to T4/T6.** *This is Wave 1's exit criterion,
   not its last row.* The DAG lists it fourth; treat it as the gate. If
   offices are reachable only from the CLI, Waves 3–5 get built on a
   path no automaton walks, and the anti-click-to-win rule ("NPCs must
   clear markets without a player") is unmeetable by the time you
   discover it. Wave 1 is done when the conductor seats a T4 and a T6
   unaided.
5. **T1.2** — ascend gates with typed refusals.
6. **REFINE-1.**

---

## 3. SHARED REGISTERS — CLAIM BEFORE YOU BUMP

Four registers are shared across lanes and collide silently. **The
ledger block is the register, not the constant.** Write your claim in
the ledger, then bump.

| Register | Where the ledger lives | Next free |
|---|---|---|
| Kernel minor | `engine/kernel/tests/conservation_pins.rs` | **20** |
| `ClosureEvent` tag | header comment in `engine/cfe/src/dop.rs` | **107** |
| Codec state schema | `engine/codec/src/lib.rs` (`SCHEMA_V*`) | **v6** |
| `ZoneKind` wire | `dop.rs` — 1–8 compiled, **9+ are minted kinds** | — |

This is not bureaucracy. Two lanes minted kernel `0.13.0` independently
this week; the collision was resolved as a sequence and both ledgers
carry the scar. `R2.1` in your DAG already says "wire tags claimed
first" — that instinct is correct and this is why.

**Minor vs patch is DERIVED, not chosen:** a minor moves when the gate
*admits or refuses* something it did not before. A patch cannot alter a
verdict.

---

## 4. WHAT A NEW `ClosureEvent` FORCES YOU TO TOUCH

Adding a deed is mechanical but two of the sites are **not
compiler-enforced** and have been missed before.

| Site | Forced by compiler? |
|---|---|
| `dop.rs` encode arm + decode arm | yes |
| `world/replay.rs` arm | yes |
| `kernel::layer_of` (World vs Civilizational) | yes — exhaustive match |
| `kernel::scope_of` (Build/Raze/Collect) | yes — exhaustive match |
| `lens/render.rs` + CLI receipt line | yes |
| **`kernel::covered()`** | **NO — by hand** |
| **`taxon_pins::every_deed()` + its count** | **NO — by hand** |

A deed missing from `covered()` refuses `Unjudged` forever. A deed
missing from `every_deed()` makes the totality pin pass while proving
less than it claims. Coverage currently stands at **67 deeds, total**.

---

## 5. THE LAWS THAT WILL BITE YOU

### 5.1 Doors and facts (29_CONSTRUCTION)

If a refusal is decidable **from the value alone**, it dies at a
constructor and the verb never asks again. If it depends on the
**chain**, it stays a fact at the verb forever, because what the chain
holds grows by receipt.

Worked example you can copy: `almanac::Compoundable::new` proves matter
bonds (shells agree, phases agree, binds saturate — all set-local), so
past that door `gem()` and `knowledge()` are total, no `Option`
survives. Locality, pressure and workability read the star, the strata
and the heat, so they stay at the verb.

For your arc: seat width is a door. *Whether this line has the deeds to
ascend* is a fact.

### 5.2 One home, and call-not-copy

A law gets **one function**. The emitter settles with it; the gate
**re-derives by calling the same function**. Never two implementations
that "agree while nobody edits them."

Precedents: `dop::vein_wear`, `almanac::recipe_of`,
`vocabulary::zone_base`, `world::pair_melt`.

### 5.3 Receipt authority — for what the gate cannot re-derive

`kernel::admits(prior, next, step)` receives **committed state, which
contains no chronicle**. So anything fold-dependent is unreachable to
it. The established answer: **the receipt declares it, and the gate
reads the declaration.**

Precedents: tag 101 declares the ceilings its draw ran under;
`ZoneBuilt` declares the compiled base it was priced against;
`AssayStruck` declares the vein's after-richness.

**This will hit you directly.** T4–T9 offices, STAMP instruments and
rail promotions are all fold-or-registry dependent. Decide early, per
deed: can `admits` re-derive this from committed state? If not, the
receipt carries it, or the state does.

### 5.4 Additive-trailing schema growth

New wire or snapshot fields append at the **tail**. An older reader
stops before them and decodes a whole record.

I inserted the treaty block mid-stream between the routes and the
practices. `world_taxon`'s v1 pin failed instantly because every field
after it misaligned. Moved to the tail. **Do not insert mid-stream**,
and note that `world_taxon` hand-truncates a blob to simulate v1 — if
you add a trailing run, that truncation count moves with it. That pin
bites on purpose.

### 5.5 SEAL B — replay equals live

Every deed needs a replay arm, and `replay(chronicle) == committed()`
must hold. Put that assertion in every arc pin you write.

The split I found this session and how it happened: breaking a peace
loads the breaker's severance book. Live did it; replay did not. Cause
— the verb strikes the displaced pact in **its own receipt before**
sealing the new one, so by the time replay saw the seal, the peace was
already gone and the breach was invisible. Fix: the strike receipt
already carried `broken`, so the load posts **there**. Lesson: when a
deed emits more than one receipt, be exact about which receipt carries
which consequence.

### 5.6 No cartesian sweeps — peel and shear

Operator ruling 2026-07-30, carried in `peel`'s own doc: *no cartesian
find-scan repetition; peels and shears where there were iterations.*

- **PEEL** — lift one thing out by key.
- **SHEAR** — one pass depositing into a sheet; never rescan per pair.

**The test: does the law already partition the space you are walking?**
If it does, walk the partitions; the space between them was never a
candidate. `World::bench` swept every subset of a ledger and filtered —
55,000 candidates for 24 elements. Sheared onto the `(shell, phase)`
planes: **7.32s → 0.04s**, same reading.

This applies directly to your market: an order book is not a cartesian
product of bids and asks. Match on the plane the price law partitions.

**If you optimise, prove the reading did not move.** Keep the naive
form as a reference oracle in the pin and assert set-for-set agreement
(`engine/cfe/tests/bench_shear_pins.rs` is the pattern).

### 5.7 The ratchet — no headroom

`tuned_constant_ratchet` counts tuned `ratio(a,b)` literals across all
source trees including the CLI. **Ceiling 112, currently at 112.**
Zero and one are exempt ("algebra"). Everything else must be
**derived, measured, or minted**. The ceiling does not rise.

Two things it caught this session:
- Three `ratio(1,1)` identities — exempt, fine.
- The CLI defaulting treaty terms (half the melt for a concord, double
  for a war). **Correctly refused.** What a pact costs is the parties'
  business; an engine that picks it writes policy nobody agreed to. The
  fix was to require the caller to state it.

**Flagged for Wave 4:** Q-S1's linear crystal→STAMP curve is a tuned
dial. It will not pass. Derive it, measure it, or mint it as a
chartered parameter (`cfe::parameter::Parameter` is the precedent —
seed values with a fourfold band and a T0 charter path).

### 5.8 The opaque-payload law

`cfe` never parses a work payload; `recurrence_guard` fails the build
on any attempt. The kernel calls the sealed verifier. Arrows point one
way: kernel → sealed math, nothing upward.

### 5.9 Verify-time search is prohibited

Operator directive 2026-08-07. A mint request carries its **complete
resolution path**; the gate re-applies it in time linear in the path.
Nothing searches. Absent path → `WorkBroken::PathAbsent`, refused
ungranted. Applies to any new mint source you introduce, including
STAMP.

---

## 6. VERIFICATION BAR

- **Full board green** before every commit. Not a crate sweep — the
  full board caught cross-crate breakage twice.
- **Clippy zero.**
- **Ratchet green.**
- **`replay == committed`** in every arc pin.
- **Play it in the CLI.** Script stdin, read what a player sees.

### The environment trap — read this twice

This is a zero-exit-status environment; commands are wrapped so they
exit 0. **That wrapper also makes a killed board look finished.** A
backgrounded `timeout N cargo test …` cut off at the limit exits 124,
and the harness notification reads *"completed (exit code 0)"* — the
wrapper's status, not cargo's. The partial log tallies a clean
"13 passed, 0 failed", which is indistinguishable from success.

**A board is green only if you prove it FINISHED:**
1. capture and read `EXIT $?` from the cargo run itself (124 = killed);
2. count `Running` lines against the number of test files, and account
   for every missing one;
3. confirm the known-slow tests actually reported `ok`.

I nearly reported a killed board as green twice. `conductor`'s
`mint_loop_pins` takes ~26 minutes; budget for it and do not wrap it in
a short timeout.

---

## 7. RULINGS AND DEFAULTS IN FORCE

All of `Q-T1…Q-S4` are codeable as written (`OPEN_QUESTIONS.md`).
Notable:

- **Q-T2** — T5 Crucible is *advantage only*; T3 may refine.
- **Q-R1** — sealed strand receipt file is acceptable offline.
- **Q-S2** — STAMP demurrage off in v1.
- **Q-S3** — keep FORWARD-1 parallel until NOTE-1 is green.
- **Q-S1** — linear placeholder curve; **see §5.7, it will not pass the
  ratchet as a literal.**

Standing, unresolved, and not yours to decide: the seed census-class
ruling, the η native ruling, and the IS-3 deed grant.

---

## 8. COLLISIONS AND TERRITORY

- **`engine/lens/src/render.rs` and `tools/observer.py` currently hold
  another party's uncommitted work**, including an `elements` array
  added to the uplink. `31_ELEMENT_COMMERCE` and the observer columns
  (T1.5, R2.6) touch exactly that surface. **Reconcile before editing
  those two files.** Never `git add -A`; never checkout a shared file.
- **`#38(b)`, the conductor corpus path, is d3-scope's.** They hold the
  forge-zeroing half. Do not touch it unilaterally.
- **Once-per-task minting is emitter-only.** The CLI bench reads the
  chronicle for a prior `DiscoveryMinted` carrying that task_id. The
  gate cannot: `cfe` may not parse the payload and `admits` gets no
  receipts. **Any new mint path you build must repeat that check or
  worlds double-mint.** Closing it properly needs the solved set in
  committed state — an unclaimed kernel amendment.

---

## 9. STARTING STATE (historical — pre–Wave-1)

`main` @ `48bc7d3` at brief write. **Post-land:** `main` @ **`2109bf0`**,
kernel **0.20.0**, tag **107** claimed, next free tag **108** / minor **21**.

Wave-1 verification (implementor-measured, not the 958 figure): ledger /
kernel / cfe / lens / codec / mesh / scribe / ratio green; `wave1_era_pins`
+ `ground_pins` + `arbor_observed_pins` + `research_unlocks_industry` green;
clippy zero; ratchet at 112. Conductor full suite still budget
`mint_loop_pins` separately.

## 10. LANDING RECORD (append-only)

| Commit | Slice |
|--------|--------|
| `2109bf0` | Station 5–10; registry T4–T9; office_preconditions; office_step; ElementRefinery + ElementRefined; wave1_era_pins; T2 founding sovereigns |
