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
| Civilization genesis (disposition/metabolism/perception) | **LIVE** | `04_CIVILIZATION_GENESIS.md`, `44_LINEAGES.md` | `line_synthesis`/`seal_genesis` landed; **no interactive picker** — CLI requires all 4 args typed |
| Civilization-design framework (5-axis coordinate space) | **LIVE + FRONTIER** | `44_LINEAGES.md` (2026-08-13, current) | disposition×metabolism×perception×class×clade is open/mintable; planet-fit (2a) and env-coupled demographics (2b) EXIST; evolutionary arc (2c) **just landed** (`#51`, `EvolutionAdvanced` tag 169); castes (2d) is the next frontier item |
| Population dynamics | **LIVE** | `40_REJUVENATION.md`, `42_LIFESPAN.md` | logistic growth/starvation/mortality at touch, `#38-42`, matches code exactly |
| Provisioning / rations | **LIVE** | `41_PROVISION.md` | half-harvest law, food-identity, matches `rejuvenation.rs` exactly |
| Distribution / granary / workbench-gate | **LIVE** | `43_DISTRIBUTION.md` | L1-L4 landed (`#45-48`), disposition factors wired; the WORKBENCH as a player-facing menu is still doc-only (§7) — no `depot_state()` query, no CLI render |
| Founding (homeworld acquisition) | **PARTIAL, worst gap** | `20_FOUNDING.md` | purse (§5) **is landed** (`world/economy.rs:776`); Elder/Ancient seats + the tithe (§1-3, §6a) are **not** — `#32`, HANDOFF's top-ranked item after the Phase-3 flip |
| Ideology / cohesion | **LIVE** | `09_IDEOLOGY.md` | five-landing board shipped whole, tag 38 |
| Tech emergence | **PARTIAL** | `10_TECH_EMERGENCE.md` | crystallized-node plumbing exists (`dop.rs`, `literature.rs`); no `TechCrystallized`-named deed |
| Era convergence | **DESIGN** (mislabeled) | `07_ERA_CONVERGENCE.md` | doc claimed "LANDING SPEC" — zero code for `TUniversal`/`convergence_pass`; corrected 2026-08-13 |
| Trade / fleet / desk | **LIVE (as trade capital)** | `33_TRADE_FLEET_AND_DESK.md` | 9 desk panes real, Phase 1-5 core complete; fleets exist for hauling, **not** for exploration/colonization — no ship-driven system discovery or frontier mechanic |
| Element economy | **PARTIAL** | `31_ELEMENT_COMMERCE.md` | REFINE-1 landed Wave-1; `#29` (element-eater ration settle) still open, on the 44_LINEAGES sequence as item 4 |
| Economy / money (STAMP, tariffs, escrow) | **DESIGN (mostly)** | `01_ECONOMY.md` | thermodynamic physics (`T_eq⁴`) real; domain/leasing/tariffs zero code; generic escrow exists, nothing Litho-Chalcogen-specific |
| Vocabulary / novelty mint | **LIVE (deed) / PARTIAL (fold)** | `28_VOCABULARY.md` | `VocabularyMinted` tag 96 real; `class_ceiling`/`clade_ceiling`/`axis_valid` are still hardcoded, NOT yet readings of the minted fold (doc corrected 2026-08-13) |
| Standing / vacuity | **STRUCK** | `25_STANDING.md` | `dual_pole`/`Vacuity` removed by operator ruling 2026-08-04 — doc had dead citations, corrected 2026-08-13 |

## 3. The civilization-design frontier (44_LINEAGES build sequence)

This is the live roadmap for "what a civilization IS" — read `44_LINEAGES.md`
whole before building anything species/archetype-shaped; do not re-derive a
sequence, one already exists and is being executed against:

1. **Evolutionary arc (2c)** — **LANDED 2026-08-13** (`#51`, `EvolutionAdvanced`
   tag 169). A line advances a staged evolution when it earns a crystallized
   tech condition; lifts provisioning capacity.
2. **Castes (2d)** — NEXT. Splits the undifferentiated `clone_count` into
   weighted specialized fractions (labor/cognition/industry). Seed:
   `world/labor.rs`.
3. **Environment-coupling extensions (2b+)** — per-clade laws on the landed
   carrying-capacity base (cooling cycles, moving footprints, flux-tied
   output).
4. **Plasmic population (`#29`)** — the element-eater demographics the
   plasmic lineages (Vapor-Lattice, Thermo-Solder) are waiting on.
5. **Planet-fit CREATION (2a+, `#34`)** — progenitor-gated fitted-world
   founding.

## 4. Ranked open work (HANDOFF's queue, corrected)

From `docs/HANDOFF_2026-08-13.md`, largest blast radius first — corrected
2026-08-13 where its own citations were wrong (see §5):

1. **`#31` Phase-3 flip** — kernel becomes the live enforcement gate, not
   an advisory shadow of `replay`. Highest leverage; every later phase
   plays differently after.
2. **`#32` Phase 1 founding** — Elder/Ancient seats + receipted tithe.
   Genuinely absent (verified 2026-08-13). Purse is already landed, contrary
   to HANDOFF's original (wrong) claim.
3. **`#29` element economy** — `ElementYieldCollected` deed.
4. **`#30` relief** — `PressureEbbed` deed, tag 168 (reserved).
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
