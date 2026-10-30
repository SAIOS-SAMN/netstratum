# 50_WIN_CONDITIONS — Closing a playthrough (ruled)

**Status: ruled, 2026-08-13 (operator).** *"There is no win state —
there is a failure state — otherwise the game should just continue in
perpetuity."* This settles §2 below: **option E (no win condition) +
option A (a loss/collapse condition) is the law.** Options B, C, D are
struck — not deferred, not "later," struck — a "number go up" or
"reach a threshold" win closes exactly the perpetual-play structure the
ruling names as the point. The game has a way to fail and no way to
"beat" it.

**Built (corrected 2026-10-08, measured in G2).** Option A's population
arm is live: `CivilizationCollapsed` (tag 174) closes a line once every
orb holds zero people and no fractional carry
(`cfe/src/world/science_lab.rs`), the world refuses every scoped act
after it, and `conductor::retire_collapsed` takes the line out of the
roster, its chronicle synced whole, so it is never again a party to
trade. The stability arm ("stability below a threshold for N beats") is
not built. That remains the open half of this spec.

## 1. What already exists to build a condition from

Every candidate below is a reading over state this session (or a prior
one) already computes — none require new simulation, only a threshold
and a closure receipt:

- **Population** (`#38-42`, `world/rejuvenation.rs`) — `population_of`,
  `population_capacity` (the logistic ceiling `K_pop`), lifespan-derived
  mortality.
- **Economy** — `purse()` (a line's whole economy, `20_FOUNDING §5`),
  the seat treasuries (`#32`, `world/economy.rs::seat_treasury`), STAMP
  (`32_STAMP_MARKET.md`, the universal unit of account).
- **Taxation & civics** (`#54-57`, `49_TAXATION.md`, `world/taxation.rs`,
  `world/distribution.rs::stability_of`) — levies, `GovernmentOffice`/
  `WelfareHub` zones, a welfare→stability loop already computing a
  single scalar "how well-governed is this colony" reading.
- **Ideology / cohesion** (`09_IDEOLOGY.md`, landed whole) — the dual
  cohesion ledger between civilizations.
- **Tech crystallization** (`10_TECH_EMERGENCE.md`, partial) — nodes
  crystallize from sustained pressure/surplus; no ceiling exists yet,
  so "tech victory" would need one defined.
- **Evolutionary arc** (`44_LINEAGES §2c`, landed `#51`) — a line's
  staged advancement, `World::evolution_stage()`.
- **44_LINEAGES' own framing** — a civilization is a coordinate in an
  open axis space, not a preset; any win condition should read as a
  property of that coordinate's trajectory (where a line has traveled),
  not favor one axis over another by construction.

## 2. The ruled shape — option A, alone

### A. Collapse / loss condition — the law
`stability_of` and the population logistic already model failure paths
(famine, starved mortality, `#30` quarantine decay). A loss condition —
population reaches 0, or stability holds below a threshold for N beats
— is the honest thing to define: the game already tracks how a
civilization dies; it just never closes the chronicle when it does.
Low implementation cost (a threshold check at the existing
population/stability touch, `world/rejuvenation.rs` / `distribution.rs`).
**This is the entire closure mechanic the game gets.**

### ~~B. Economic closure — the purse crosses a threshold~~ struck
~~`purse()` already sums a line's whole economy...~~ **struck by the
2026-08-13 ruling.** A "number go up" win is exactly the kind of
artificial stopping point the ruling rejects — the economy keeps
running after any threshold, same as before it.

### ~~C. Evolutionary/civilizational maturity — the arc completes~~ struck
~~`evolution_stage()`...~~ **struck.** The evolutionary arc
(`44_LINEAGES §2c`) stays open-ended by the same ruling — a line keeps
advancing for as long as it's played, with no "final stage" that ends
anything. `evolution_stage()` remains a live reading, just never a
closure trigger.

### ~~D. Diplomatic closure — cohesion crosses a threshold~~ struck
~~Uses the already-landed ideology ledger...~~ **struck**, same reason.
Cohesion stays a standing to maintain (or lose), not a target to hit
and stop at.

### E. No win condition — ruled, this is why A stands alone
Confirmed by the operator directly: some strategy/builder games (the
genre this project explicitly targets, per the "Cities:Skylines-style
civilization builder" framing this session opened with) have no win
state at all — only a standing to maintain and a collapse to avoid.
**(A) is the whole answer.** B/C/D's underlying readings
(`purse`/`evolution_stage`/cohesion) remain legitimate score-tracking —
a player or the observer can watch them — they are simply never gates
that end a chronicle.

## 3. What this doc does not decide

- The exact thresholds (population 0? stability below what, for how
  long?) — these are balance rulings, deferred the
  same way every other numeric constant in this codebase is (see the
  `tuned_constant_ratchet` discipline — do not add a threshold literal
  without an operator sign-off).
- Whether an ended game is replayable past its close (does the
  chronicle keep accepting deeds after a win/loss receipt, the way
  `ColonyAlreadyFounded` blocks re-founding, or does closure freeze the
  World the way `GenesisAlreadySealed` freezes axis-picking?).

## 4. What ships (tracked implementation task)

Only the loss condition (A). No further ruling is owed before building
it — the open questions in §3 (thresholds, replayability) are
implementation-detail decisions the builder can make and pin, not
another round of "which end-state." Land it, pin it, done.
