# 50_WIN_CONDITIONS — closing a playthrough (PROPOSAL, no code lands with this doc)

**Status: PROPOSAL, 2026-08-13.** Not a ruling. A grep across `engine/`,
`clients/`, and `docs/` for `win_condition|game_over|victory|end_game`
returns nothing — there is currently no way for a playthrough to
conclude, and every mechanic landed to date (founding, zones,
population, taxation, ideology, tech) runs forever with no closure.
This doc exists to give the operator concrete candidates to rule on,
grounded in what has actually landed, rather than leaving "how does the
game end" unspecified indefinitely.

**Why this is a proposal and not a landing:** picking a win condition
picks what the game is ABOUT — it is exactly the class of decision this
project routes through an operator ruling (see `[[code-is-the-law-anchoring]]`
in memory; every prior systemic law in this codebase, from the tithe's
sizing to the seat-scope ladder, was ruled before it was coded). Nothing
here should be built until one of the options below (or an amendment) is
chosen.

## 1. What already exists to build a condition FROM

Every candidate below is a READING over state this session (or a prior
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
  property of that coordinate's TRAJECTORY (where a line has traveled),
  not favor one axis over another by construction.

## 2. Candidate end-states (pick one, several, or none of these)

### A. Collapse / loss condition (arguably more load-bearing than a win)
`stability_of` and the population logistic already model failure paths
(famine, starved mortality, `#30` quarantine decay). A LOSS condition —
population reaches 0, or stability holds below a threshold for N beats
— may be the more honest thing to define first: the game already
tracks how a civilization DIES; it just never CLOSES the chronicle when
it does. Low implementation cost (a threshold check at the existing
population/stability touch, `world/rejuvenation.rs` / `distribution.rs`),
and every other candidate below needs a loss condition to be meaningful
anyway (a "win" with no possible loss is not a game).

### B. Economic closure — the purse crosses a threshold
`purse()` already sums a line's whole economy. A target (in STAMP, per
`32_STAMP_MARKET.md`'s universal unit of account — deliberately NOT a
raw resource total, since resource identity is metabolism-relative per
this session's zone-cost work) closes the game when crossed. Cheapest
to implement; least thematically satisfying on its own (a "number go up"
ending sits oddly next to the disposition/metabolism/perception design
this project has invested heavily in).

### C. Evolutionary/civilizational maturity — the arc completes
`evolution_stage()` (`44_LINEAGES §2c`) already stages a line's growth.
A final stage (not yet defined — the arc currently has no ceiling) could
BE the win condition: reaching it is "the civilization has become what
it was becoming." Fits the project's own thesis (44_LINEAGES: "a
civilization is a coordinate... given depth by four dimensions") more
than a resource-count win would. Cost: needs the evolutionary arc's
depth dimensions (2b/2d, castes/environment-coupling — still open per
`GAME_SPEC.md §3`) fleshed out enough that a "final stage" means
something, not just a number increment.

### D. Diplomatic closure — cohesion crosses a threshold with every
known civilization
Uses the already-landed ideology ledger (`09_IDEOLOGY.md`). A "concord"
ending: every civilization in a shared World reaches sustained positive
cohesion. Thematically strong (matches the Project-Hail-Mary founding
insight: "a discovery becomes real only when two measurably DIFFERENT
civilizations confirm it independently" — a concord win rewards the
game's own central mechanic). Cost: needs more than one civilization
routinely coexisting in a played World to matter, which today mostly
happens via `chart()`-derived colonies of the SAME line, not distinct
civilizations (see `GAME_SPEC.md`'s multi-civilization note — one
`World` seals exactly one `genesis`).

### E. No win condition — only a loss condition, by design
A legitimate fifth option: some strategy/builder games (the genre this
project explicitly targets, per the "Cities:Skylines-style civilization
builder" framing this session opened with) have no win state at all —
only a standing to maintain and a collapse to avoid. If this is the
ruling, (A) alone is the whole answer and B/C/D become optional
score-tracking, not closure conditions.

## 3. What this doc does NOT decide

- Whether multiple end-states can coexist (e.g., A always active, C as
  an optional "achievement" ending).
- The exact thresholds (population 0? stability below what, for how
  long? which STAMP figure?) — these are balance rulings, deferred the
  same way every other numeric constant in this codebase is (see the
  `tuned_constant_ratchet` discipline — do not add a threshold literal
  without an operator sign-off).
- Whether an ended game is REPLAYABLE past its close (does the
  chronicle keep accepting deeds after a win/loss receipt, the way
  `ColonyAlreadyFounded` blocks re-founding, or does closure freeze the
  World the way `GenesisAlreadySealed` freezes axis-picking?).

## 4. Recommended sequencing once ruled

Land the LOSS condition (A) first regardless of which win condition is
chosen — it is cheap, it is honest about what the game already
simulates, and every other candidate needs it to mean anything. Then
layer whichever win condition(s) the operator rules on top.
