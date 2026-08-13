# GAME SPEC — the authoritative map of what netstratum is, right now

**Purpose.** `docs/` has 78 files and ~30,000 lines, and no single one of
them tells a reader what the game IS today without also making them chase
citations that may have moved, been struck, or never landed. This document
does not restate the law — it is an INDEX WITH A VERDICT: for each system,
what it is, whether it's built, and which doc is the authority to read next.
Where this doc and a subordinate doc disagree, the subordinate doc governs
its own subject; this doc's job is only to be honest about what's live.

**Provenance.** Compiled 2026-08-13 from a six-cluster audit of every file
in `docs/` against the actual code in `engine/`, `clients/`, `protocols/`
(not against what the docs claim about themselves). See the `docs-audit-lane`
branch history for the corrections this produced. Verified against `main`
at `138cecb` (the EVOLUTION deed, `#51`).

**Refreshed same day, 2026-08-13, against `main` at `23ad6de`** — this
was a fast-moving session: `#32` founding (tithe + seat treasury), the
guided genesis picker, metabolism-aware zone build costs, castes
(`44_LINEAGES §2d`, `#52`), taxation (`49_TAXATION.md`), and the civic
GovernmentOffice/WelfareHub zones all landed after the initial pass
below. Rows are corrected in place rather than re-audited whole —
treat any row not mentioning this refresh as still accurate from the
original pass.

---

## 0. What this is

A dual-headed PoW++ crypto engine with a civilization-building game on top,
framed in space-lore. AI NPCs solve ARC-AGI-style puzzles to prove
cognition; the proof feeds an in-game economy that overlays real-world
cryptocurrency. "Dual-headed" = Proof of Useful Work + Proof of
Work-Convergence, each with a kernel half (local) and a mesh half
(cross-node) — `engine/kernel/CHARTER.md` §5a. The kernel is playable now;
the mesh is deliberately built second (operator ruling 2026-07-31).

The player-facing shape is closer to a Cities:Skylines-style civilization
builder than to the mesh/consensus machinery underneath it: found a
civilization on a homeworld, zone and build, manage scarcity and growth,
watch a population live and die by the world's carrying capacity — with a
crypto settlement layer underneath that the player mostly doesn't see
directly yet (no interactive client renders it).

## 1. Authority order

`00_CHARTER.md` → `26_ARTICLES.md` (constitutional law) → `EQUATIONS.md` §1
→ system-specific docs → this doc. This spec never overrides a subordinate
doc's own law; it only reports which subordinate doc currently holds that
law and whether the code agrees with it.

## 2. System status board

Legend: **LIVE** = built, tested, matches its doc. **PARTIAL** = some
sub-mechanics built, others design-only, doc says which. **DESIGN** =
zero code, doc is honest about it. **DRIFTED** = doc claimed landed but
code disagrees (corrected in this pass, see §5).

| System | Status | Authority doc | What's actually live |
|---|---|---|---|
| Substrate primitives (Δ, coboundary, gates) | **LIVE** | `03_SUBSTRATE_CONTRACT.md` | 11/12 primitives confirmed exact; `GateReading` has 2 refusal variants (`Unproductive` struck 2026-08-01) |
| Capacity law (Δ(V)=D, the D/V ontology) | **LIVE** (core) | `17_CAPACITY.md` (split 2026-08-13) | §1-9 core; cage/containment mechanics live in `22_CONTAINMENT.md`; threat model in `48_CAPACITY_THREAT_MODEL.md` |
| Kernel enforcement (Phase 3) | **PARTIAL** | `39_PHASE3_FLIP.md` | Preconditions structural; the flip itself (kernel as the live gate, not `replay`) is NOT landed — top HANDOFF priority, `#31` |
| Colony loop (found/build/collect/famine) | **PARTIAL** | `02_COLONY_LOOP.md` | D1-D3 landed exact; D4 (spoilage, milestones) is unbuilt — tag 9 `MilestoneReached` doesn't exist |
| Civilization genesis (disposition/metabolism/perception) | **LIVE** | `04_CIVILIZATION_GENESIS.md`, `44_LINEAGES.md` | `line_synthesis`/`seal_genesis` landed; **interactive picker now landed** — bare `genesis` walks all 3 axes as menus, typed-args form unchanged |
| Civilization-design framework (5-axis coordinate space) | **LIVE + FRONTIER** | `44_LINEAGES.md` (2026-08-13, current) | disposition×metabolism×perception×class×clade is open/mintable; planet-fit (2a) and env-coupled demographics (2b) EXIST; evolutionary arc (2c, `#51`) AND castes (2d, `#52`) landed; environment-coupling extensions (cooling-cycle K ratchet, moving footprint) also landed same day |
| Population dynamics | **LIVE** | `40_REJUVENATION.md`, `42_LIFESPAN.md` | logistic growth/starvation/mortality at touch, `#38-42`, matches code exactly |
| Provisioning / rations | **LIVE** | `41_PROVISION.md` | half-harvest law, food-identity, matches `rejuvenation.rs` exactly |
| Distribution / granary / workbench-gate | **LIVE** | `43_DISTRIBUTION.md` | L1-L4 landed (`#45-48`), disposition factors wired; the WORKBENCH as a player-facing menu is still doc-only (§7) — no `depot_state()` query, no CLI render |
| Founding (homeworld acquisition) | **LIVE (fallback path)** | `20_FOUNDING.md` | purse (§5), Elder/Ancient seats (T16/T17), and the receipted tithe (§3, `TitheReceipted` tag 170) all landed 2026-08-13; `#32` is CLOSED for the FALLBACK siting path. PURCHASE/COMMISSION (§2a/2b, the elder-tithe path) still deferred |
| Ideology / cohesion | **LIVE** | `09_IDEOLOGY.md` | five-landing board shipped whole, tag 38 |
| Tech emergence | **PARTIAL** | `10_TECH_EMERGENCE.md` | crystallized-node plumbing exists (`dop.rs`, `literature.rs`); no `TechCrystallized`-named deed |
| Era convergence | **DESIGN** (mislabeled) | `07_ERA_CONVERGENCE.md` | doc claimed "LANDING SPEC" — zero code for `TUniversal`/`convergence_pass`; corrected 2026-08-13 |
| Trade / fleet / desk | **LIVE (as trade capital)** | `33_TRADE_FLEET_AND_DESK.md` | 9 desk panes real, Phase 1-5 core complete; fleets exist for hauling, **not** for exploration/colonization — no ship-driven system discovery or frontier mechanic |
| Element economy | **PARTIAL** | `31_ELEMENT_COMMERCE.md` | REFINE-1 landed Wave-1; `#29` (element-eater ration settle) still open, on the 44_LINEAGES sequence as item 4 |
| Economy / money (STAMP, tariffs, escrow) | **DESIGN (mostly)** | `01_ECONOMY.md` | thermodynamic physics (`T_eq⁴`) real; domain/leasing/tariffs zero code; generic escrow exists, nothing Litho-Chalcogen-specific |
| Vocabulary / novelty mint | **LIVE (deed) / PARTIAL (fold)** | `28_VOCABULARY.md` | `VocabularyMinted` tag 96 real; `class_ceiling`/`clade_ceiling`/`axis_valid` are still hardcoded, NOT yet readings of the minted fold (doc corrected 2026-08-13) |
| Standing / vacuity | **STRUCK** | `25_STANDING.md` | `dual_pole`/`Vacuity` removed by operator ruling 2026-08-04 — doc had dead citations, corrected 2026-08-13 |
| Taxation (craftable levies) | **LIVE** | `49_TAXATION.md` (renumbered 2026-08-13, was `45_TAXATION` — collided with `45_ALMANAC`) | levy model + continuity tax landed `#54/#55`; a levy is minted data (base/trigger/rate/sink) over the vocabulary precedent, not a hardcoded drain |
| Civic engine (stability/welfare) | **LIVE** | (no dedicated doc yet — see `world/taxation.rs`, `world/distribution.rs::stability_of`) | `GovernmentOffice`/`WelfareHub` zones + the welfare→stability loop landed `#56/#57` |
| Zone build costs | **LIVE** | `02_COLONY_LOOP.md` (cost table), this session's fix | `ZoneKind::cost()` now follows the same metabolism-substitution rule `zone_upkeep`/`zone_yield` already had — a Habitat's build price reads the civ's own ration, gated on a nonzero harvest rate so plasmic (rate 0) isn't priced in Joules it can't hold at founding |
| Win condition / game closure | **DESIGN, proposal only** | `50_WIN_CONDITIONS.md` (new, 2026-08-13) | zero hits anywhere for win/loss/victory/game-over — a playthrough currently cannot conclude. Candidates drafted for an operator ruling; nothing implemented |
| Autonomous play / beta-run capability | **LIVE, undocumented until now** | `BETA_RUNBOOK.md` (new, 2026-08-13) | `engine/conductor` already plays every registered civilization autonomously (no human input); `tools/observer.py` watches read-only. Verified live 2026-08-13: 22 civilizations, 5 beats, zero errors |

## 3. The civilization-design frontier (44_LINEAGES build sequence)

This is the live roadmap for "what a civilization IS" — read `44_LINEAGES.md`
whole before building anything species/archetype-shaped; do not re-derive a
sequence, one already exists and is being executed against:

1. **Evolutionary arc (2c)** — **LANDED 2026-08-13** (`#51`, `EvolutionAdvanced`
   tag 169). A line advances a staged evolution when it earns a crystallized
   tech condition; lifts provisioning capacity.
2. **Castes (2d)** — **LANDED 2026-08-13** (`#52`). Splits `clone_count`
   into weighted specialized fractions (labor/cognition/industry).
3. **Environment-coupling extensions (2b+)** — **LANDED 2026-08-13**: the
   cooling-cycle K ratchet (cold-driven lines gain ceiling) and moving
   footprint (mobility as physical state) both shipped same day as castes.
4. **Plasmic population (`#29`)** — still open. The element-eater
   demographics the plasmic lineages (Vapor-Lattice, Thermo-Solder) are
   waiting on. Now the LAST open item in this sequence.
5. **Planet-fit CREATION (2a+, `#34`)** — progenitor-gated fitted-world
   founding. Still open.

**As of 2026-08-13, 3 of 5 sequence items are landed** — the frontier has
moved to `#29` (element economy) and `#34` (planet-fit creation), both
already tracked in §4 below.

## 4. Ranked open work (HANDOFF's queue, corrected — refreshed same day)

From `docs/HANDOFF_2026-08-13.md`, largest blast radius first — corrected
2026-08-13 where its own citations were wrong (see §5), then refreshed
again later the same session as `#32` and other items landed:

1. **`#31` Phase-3 flip — STILL OPEN, STILL #1.** Kernel becomes the live
   enforcement gate, not an advisory shadow of `replay`. Untouched by
   everything else that landed this session — tithe, treasury, zone
   costs, taxation, civics all sit on top of enforcement that isn't
   actually enforced yet. Highest leverage of anything open.
2. ~~`#32` Phase 1 founding~~ — **CLOSED 2026-08-13.** Elder/Ancient seats
   + receipted tithe both landed for the fallback siting path (see §2's
   Founding row). PURCHASE/COMMISSION remain deferred, not blocking.
3. **A win/loss condition — NEWLY SURFACED, not on HANDOFF's original
   list.** Zero code anywhere closes a playthrough. See
   `50_WIN_CONDITIONS.md` for the proposal; this is a game-completeness
   gap distinct from `#31`'s engine-completeness gap, and may be
   equally urgent depending on what "a complete game" is meant to mean.
4. **`#29` element economy** — `ElementYieldCollected` deed. Now the
   last open item in the 44_LINEAGES sequence (§3).
5. **`#30` relief** — `PressureEbbed` deed, tag 168 (reserved).
5. Then the 44_LINEAGES sequence in §3, plus `#33`/`#34`/`#35`/`#5` per
   `rulings-2026-08-13` (demote market claim, progenitor delegation,
   kinetic war teeth, court rename).

## 5. Doc hygiene — what changed in the 2026-08-13 audit

A parallel doc-audit pass (branch `docs-audit-lane`) found and fixed three
repeatable failure modes across `docs/`:

- **Same-day numbering collisions** (no index existed to catch a lane
  grabbing a taken slot): `14_ALMANAC` → `45_ALMANAC` (`14_CREEDS` keeps
  slot 14 — it's cited by section number elsewhere, ALMANAC wasn't),
  `19_KERNEL` → `46_KERNEL` and `19_REGISTRY` → `47_REGISTRY` (`19_LENS`
  keeps slot 19, the only one cited externally by section number).
  `20_WORLD_REFACTOR` (a closed, past-tense inventory doc) moved to
  `docs/archive/` — `20_FOUNDING` keeps slot 20.
- **Dated-but-already-stale docs**: `HANDOFF_2026-08-13.md`'s
  `seat_width()` citation was fabricated (real: 3 call sites, not 7, all
  correct) and its "purse absent" claim was false — corrected in place.
  `PICKUP.md`'s next-free-tag counter is a live-moving number; re-verify
  against `dop.rs`'s header before trusting it (was 168 at audit time,
  is 170 as of `#51`'s tag 169 claim — see the merge in this branch).
- **Accretion docs**: `17_CAPACITY.md` (2248 lines, chronological
  self-correcting ruling log) split like `18_TORSION` was — core stays at
  17, cage mechanics folded into `22_CONTAINMENT`, threat model extracted
  to `48_CAPACITY_THREAT_MODEL.md`. Section numbers preserved across the
  move, so old citations still resolve.
- **Dead citations**: `Vacuity`/`dual_pole` (struck 2026-08-04) and
  `lens::standing` (`19_LENS §12`, removed 2026-08-04) were still cited as
  live in `25_STANDING.md` and `47_REGISTRY.md` (the latter self-
  contradicted its own table) — corrected.
- **Archived as dead weight** (self-marked HISTORY or fully superseded,
  moved to `docs/archive/` with a banner): `N4_DIRECTIVES.md`,
  `N6_DIRECTIVES.md`, the three `RELAY_*` wave-handoff docs,
  `FORGE_REMEDIATION.md` (superseded by `FORGE_BUDGET.md`),
  `AUDIT_2026-07-31.md`, `ENDOWMENTS.md`, `BALANCE_NOTES.md`,
  `KERNEL_RELAY.md`.

Full per-doc findings (STATUS/REDUNDANT/GAP for all 78 files) are in the
session transcript that produced this pass, not duplicated here — this
spec keeps only the verdicts that change what a builder should read next.

## 6. What still needs a live-code recheck

This spec is a snapshot. Two things it could NOT verify because they were
mid-flight in another node's uncommitted work at compile time:
`world/core.rs`, `world/distribution.rs` (env-coupling extensions may
already be landing as this is read) — re-run `git log --oneline -5` on
`main` before trusting §2's PARTIAL/DESIGN calls for those two rows.

## 7. Investigated and cleared, 2026-08-13 (don't re-investigate)

- **Naming-concatenation.** An operator question asked whether
  civilization/vocabulary names could compound into a
  `word-wordmore-wordmoremore` pattern. Checked every naming path:
  `world.found(name)` stores the caller's string verbatim; minted
  vocabulary entries (`VocabularyMinted`, tag 96) take `word`/`sigil`
  straight from input with a name-collision REFUSAL (not an append,
  `ActionRefused::MintNameTaken`); `SolverWordMinted` is a binary hash
  fingerprint, not a display name; `tools/observer.py`'s
  `display_name()` truncates/elides and a prior prefix-concatenation
  scheme was already removed there for being inconsistent. No
  reproducible bug found. If this is observed again, the cause is
  external to this codebase (get the exact screen/log it appeared on
  before re-opening this).
- **Autonomous play vs. cognition-driven play.** `engine/conductor`
  already runs every registered civilization unattended (see the
  Autonomous play row in §2 and `BETA_RUNBOOK.md`) — this is scripted/
  procedural policy, NOT solver/ARC-AGI-piloted strategy. `engine/solver/`
  proves useful work for the PoW layer and has no read/write path into
  a `Polity`'s decisions; the two systems are unrelated despite both
  reading as "AI" in casual framing. Don't conflate them in future asks.
