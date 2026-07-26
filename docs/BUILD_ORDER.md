# BUILD ORDER — the one carrier

**LAW: the current landing is read HERE, never inferred.** One landing at a
time; red-first with recorded output; spec + mechanic + test vectors in the
same landing (CHARTER §1); commit on explicit operator go.

## 0. The landed ledger

- **N0 LANDED** (`bd70ed2`, 2026-07-25): the scaffold — charter (Dark Forest
  constraints, §7 bridge governance, §8 terminology law), this carrier,
  NS-1..NS-7 stubs, the economy at design stage.
- **N1 LANDED** (2026-07-25): NS-3 VRP written FROM the working v0.4
  chronicle organ. Evidence: the sealed build driven read-only (path
  dependency, fixture identical to the landed witness); the tick law measured
  (3-of-3 ticks; 2-of-3, skip both refuse, clock unmoved); three receipts
  extracted byte-exact (genesis parent zero; chain `Intact`); tamper
  divergence measured (one changed u64 → digest `c86c17aa…` → `366aa785…`).
  Vectors are self-contained in NS-3 §3 — any implementation verifies from
  the document alone. Ruling carried: the normative TLV event encoding lands
  WITH the first engine implementation at N2 (the event vocabulary cannot
  precede the actions that close).
- **N2 LANDED** (2026-07-25): NS-1 DOP first cut + THE FIRST PLAYABLE SLICE.
  The workspace opened (`engine/cfe`, `clients/cli` — `cargo run -p
  netstratum-cli` is the one onboarding command). Evidence: clippy zero;
  **11/11 witnesses green, memory-capped** (NS-3 conformance: the inscribed
  v0.4 vectors reproduce on the fresh implementation, tamper vector included;
  the tick law refuses 2-of-3/skip/replay; the orb round-trips BYTE-exact;
  unknown tags skip — SEAL A measured; replay equals the committed world —
  SEAL B measured; exact accrual: 4×3/2 = 6, 3×3/2 = 9/2, no drift). The
  played session: found → zone×2 → tick 4 → collect → chain `Intact` (5
  receipts) → replay MATCHES. The N1 TLV debt PAID: the permanent tag
  registry at NS-1 §2.3 (events 1–3, orb 16–18); NS-1 vectors extracted from
  the running engine (`examples/ns1_vectors.rs`). The workspace invariant
  carried: accrued-uncollected yield has no receipt and does not persist.
- **N3 LANDED** (2026-07-25): NS-6 ipsal-os first cut — THE ENTITY LAYER.
  Identity (sovereign auto-registered at founding; agents contained by
  their principal), scoped grants (Build/Raze/Collect, wire codes 1–3),
  generation tagging (the NS-3 provenance field, until now empty, carries
  the forging entity's id — no receipt format change spent). Tags 10–12.
  Evidence: clippy zero; **26/26 witnesses green capped**; the three
  containment laws measured (containment precedes the world; a grant is a
  scope, not a blanket; revocation immediate and unilateral); no
  simulation rule reads provenance; entities/grants/revocations replay
  from receipts alone. One witness truth-update recorded: pre-founding
  refusals are now NoSuchEntity (identity precedes the colony). The CLI
  arc played end-to-end: agent refused → granted → collected with ITS tag
  (`by#2` beside the sovereign's `by#1`) → revoked → refused again; chain
  INTACT (9 receipts), replay MATCHES. NS-1 §3.3 vectors re-extracted
  (provenance participates in the digest). (2026-07-25, operator playtest feedback — presentation
  only, no protocol change): the 8×8 MAP grid rendered after every build
  (`map`), `row:col` coordinate input beside plain tile numbers, refusals as
  human sentences with the cure named (each of the playtest's three stumbles
  reproduced and answered), color highlighting (✓ closures green, ✗ refusals
  red, ◔ workspace yellow with "collect to keep", exact fractions bolded —
  `--plain`/`NO_COLOR` for scripts), described chronicle lines, `help`.
  Evidence: clippy zero; 11/11 witnesses green capped; the playtest session
  re-run end-to-end with every error case answering usefully. Exactness
  discipline held: fractions rendered whole ("3/2", never 1.5).

## 1. The board

| # | Landing | Builds | Proven by |
|---|---|---|---|
| **N0** | THE SCAFFOLD | the monorepo tree, the charter, this carrier, protocol stubs | the tree exists; the charter read whole |
| **N1** | NS-3 VRP, spec-from-working-code | the receipt spec written FROM the v0.4 chronicle organ (BLAKE3 append-only, parent-linked, quorum-stamped) with test vectors extracted from real chronicle output | the spec's vectors verify against a fresh engine implementation; tampering/reordering refuses typed |
| **N2** | NS-1 DOP + THE FIRST SLICE | one entity, one colony orb, CFE state ticks in the CLI; every action emits an N1 receipt | a playable tick loop; the orb round-trips its wire codec byte-exact; the chronicle replays the session |
| **N3** | NS-6 IPSAL-OS, first cut | the entity/profile container: identity, permissions, generation tagging for one human + one scripted agent | the agent acts only inside its granted scope; every agent action receipted with generation tag |
| **N4** | NS-2 RQP + THE SCANNER | resonance query over the orb population (sigil-address lookup, never a scan) | a structural query finds the planted match and refuses the planted near-miss |
| **N5** | NS-4 BEP + SHARING | detach a bubble (sector map / trade offer) with permissions and multi-depth content | recipient reads the summary, zooms to detail, cannot exceed granted depth; re-share honors the flag |
| **N6** | NS-5 MESH + TWO NODES | proof-of-convergence sync between two independent engine instances | two laptops converge on one world state by exact measurement; a forged state refuses |
| **N7** | NS-7 ANCHOR + WWW3 SETTLEMENT | the mesh computes one consensus root of the chronicle; a thin adapter posts it to a local WWW3 testnet; wallet↔ipsal-os entity mapping (CHARTER §7, the three seals) | a real chronicle segment anchors as one root; the world keeps ticking with the testnet down; a forged root refuses on verification |

Sequencing: N1 before N2 (receipts before the actions that emit them); N3
before N4 (identity before query scope); N6 before N7 (anchoring needs the
converged mesh root it anchors); N7 last. Each landing names its spec section
and its game consumer in the landing record.

## 1a. The substrate slot (parallel-node protocol, CHARTER §10)

- **N-RFA (coder-01, reserved):** `engine/rfa-core` (Delta,
  for_each_residual, coherence_functional, coboundary_reduce,
  validate_step) + `engine/physics` (FlowState, step_witness,
  exhale/inhale) — the clean-room math substrate, landing from the
  worktree on operator go, rebased onto N3 (disjoint files). Its
  companion `docs/03_SUBSTRATE_CONTRACT.md` is the mechanic → primitive →
  witness authority. Lore corrections it pins: the convergence-engine's
  coherence is SUM-OF-SQUARES in code (its README wrongly says L1); the
  incoherent 3-node reference vector measures C = 24, not 8.

## 2a. The genesis board (04_CIVILIZATION_GENESIS — G0–G4)

**G0 — THE DIRECTORY (identity now, mechanics at G1; joint landing):**
coder-01: registry module (`engine/mirror`), observer CIVILIZATION/WORLD/
PLANET columns, contract §8. coder-02: the three-step founding dialogue in
the CLI (pick/create civilization → pick/create world → name the planet;
species free-text). The topology and its three bindings at 04 §5. Axes are
RECORDED identity only — zero rule changes until G1 re-commits them as
genesis receipts.

G1 genesis+disposition (after N-RFA) → G2 metabolism tables → G3
perception+translation (lands WITH N4) → G4 the discovery mint (lands with
N5/N6). Details and sequencing in 04 §6.

## 1b. The planet board

- **P1 LANDED** (2026-07-25, requested by coder-01 as "D3"; boarded P1 to
  keep terrain's D3 name): MULTI-PLANET WORLDS. `World` now MEANS the
  container (04 §5 naming debt PAID by truth, not rename): multiple colony
  orbs under ONE clock / ONE chronicle (binding #1). CHARTERING (tag 13):
  a daughter colony's founding reserves MOVE from a source colony's bank —
  conservation holds, chartering mints nothing (the free-bootstrap exploit
  closed at design time). Per-orb workspaces and famine; planet names
  unique per world. Evidence: clippy zero; **77/77 workspace witnesses
  green capped** (chartering conservation; typed refusals incl.
  PlanetNameTaken/NoSuchOrb; a daughter famine at exactly t=11 never
  touches the mother; the archipelago replays from receipts alone). CLI:
  `chart <planet> [from <p>]`, `planet <name>`, `planets`, per-planet
  status/map, famine banners name the planet. Cross-territory adaptations
  (relayed): mirror/conductor/mirror_demo call sites updated mechanically
  to the new arities + ColonyCharted arm added to the mirror's renderer —
  no semantic changes.

## 2. The depth board (02_COLONY_LOOP — game depth before multiplayer)

Ruled 2026-07-25: the world must be worth sharing before N6 shares it. These
interleave with the protocol board; ruleset constants live in 02 §2 and are
LAW for replay.

- **D1 LANDED** (2026-07-25): costs, upkeep, famine, raze. Evidence: clippy
  zero; **17/17 witnesses green capped**, red-first honored (the knife-edge
  witness caught a zero-net entry lingering in the workspace — recorded red,
  cured: a zero net is not a datum). Measured: 2E+1H nets biomass EXACTLY 0
  over 8 ticks (iron 22 = 8×11/4); the third extractor refuses at 0 iron with
  the exact shortfall; famine strikes at EXACTLY tick 11 (5 − t/2 < 0);
  dormant zones neither yield nor drain (measured per-tick delta); raze
  refunds exactly half; relief at the collect where banked ≥ one-tick upkeep;
  the FULL crisis replays from receipts alone. Wire: tags 4–7 + 19 live
  (registry at NS-1 §2.3); pre-D1 receipts byte-identical (additive only);
  NS-1 §3 vectors re-extracted under ruleset v1. CLI: `raze`, famine
  banners, dormant zones lowercase on the map, costs in `help`, the
  CannotAfford message names the cure. The full arc played end-to-end:
  aggressive opening → earned third extractor → famine at tick 15 → raze →
  relief → chain INTACT (11 receipts), replay MATCHES.
- **D2 LANDED** (2026-07-25): THE INDUSTRIAL PIPELINE (02 §5). Materials
  alloy (3) + diamond (4); converter zones smelter (3) + synthesizer (4) —
  a converter's upkeep IS its input, its yield its output; multi-resource
  costs, upkeep, and refunds. **NO NEW EVENT TAGS** (the §5.3 ruling): the
  draft's ResourceSmelted/DiamondSynthesized/ProductionDormant refused —
  conversion settles as YieldCollected/UpkeepSettled, input starvation IS
  famine (measured: an under-fed synthesizer strikes ALLOY famine within
  ticks and goes dormant while the smelter keeps forging). Evidence: clippy
  zero; **21/21 witnesses green capped**; the smelter converts EXACTLY
  (2E+2H+1S nets +1/2, +1/2, +1 alloy per tick); multi-resource refusals
  name the exact shortfall per component (biomass gate, then the alloy
  gate); the tier climb replays from receipts alone; the CLI session played
  the full arc (S/Y glyphs live; the CannotAfford ladder taught the climb:
  "costs 20 alloy, have 4" → "costs 15 biomass, have 10"). Crystals
  DEFERRED to N4; slag/matrix intermediates DROPPED (02 §5.1 rulings).

| # | Landing | Contains | Proven by |
|---|---|---|---|
| **D1** | COSTS, UPKEEP, FAMINE | builds spend banked; per-tick net flows; famine/dormancy/relief; `raze` (half refund); tags 4–7 | the 2E+1H knife edge nets biomass exactly 0; a third extractor strikes famine on schedule; raze cuts the drain and the colony recovers; replay reproduces it all |
| **D2** | TERRAIN + ADJACENCY | the seeded chart (tag 8); Standard/Rich/Barren multipliers; the adjacency pair; the map renders terrain | terrain replays deterministically from the receipted seed; a rich-seam extractor yields 9/4 exactly; the habitat-beside-pit penalty measures ×3/4 |
| **D3** | SPOILAGE + MILESTONES | accrued biomass −1/4/tick floored at 0; the milestone ladder (tag 9) | spoilage floors at 0 and never banks; each milestone closes once, receipted |

