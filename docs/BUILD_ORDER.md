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

- **P2-PERSIST LANDED** (2026-07-25, joint ruling with coder-01): THE WORLD
  REMEMBERS. NS-3 §2.4 (the receipt file: tag 20 frames, one append-only
  `chronicle.tlv` per world); `cfe::persist` (load through the
  verification door; sync appends only the tail; divergent history refuses
  typed, never clobbered); `World::resume` (committed core replays; clock
  reinstates at the highest PROVEN stamp; the workspace honestly vanishes
  — what has no receipt does not persist, now measured ACROSS the process
  boundary). CLI `save`/`resume`. Evidence: clippy zero; **92/92 workspace
  witnesses green capped** incl. the two-boot witness (boot 2 byte-exact,
  re-earns nothing, era two appends 1 frame, boot 3 replays both eras);
  tamper and divergence refusals; resumed agent authority (the grant
  survives the boundary, and only the grant). The real-CLI double boot
  demonstrated end-to-end. Dark Forest constraint #1 (persistence by
  construction) is now LIVED, not just inscribed.

- **CONVERGE-1 LANDED** (2026-07-27): ERA CONVERGENCE — THE TIMELINES
  UNIFY. `07_ERA_CONVERGENCE.md` inscribed and landed whole: TUniversal
  {S,G,T} persisted (universe card at every beat AND at convergence);
  boot converges BEFORE genesis (conductor main + `run_era`); annex,
  never merge — `EraAnnexed` (permanent tag 21) appends to the universe
  ledger `convergence.tlv` (real cfe chain machinery); placement is a
  measurement (the four C's: congruence deposits onto the three axes,
  coherence residuals ride the fiber cards, convergence places by the
  terminal-window step); epoch-shift stacks eras end-to-end on one T
  axis. THE TORN-TAIL LAW measured into 07 §1.2 on the real archives: a
  killed writer's final NUL/half-written line is an append that never
  landed — the world annexes at its verified prefix. Evidence: clippy
  zero; **105/105 workspace witnesses green** (6 converge pins: annex
  onto one axis with exact offsets and TUniversal stamps; placement
  deterministic across independent roots and fiber-separated by loop
  content; mid-chain tamper quarantines NAMED at its sequence while the
  rest annex; torn tail annexes at prefix; double-converge appends
  NOTHING; tag-21 byte-exact round trip). THE REAL RUN: eras discovered
  `var-era1 → var-era2 → var`; **128 worlds annexed, 0 quarantined**
  (the torn-tail law recovered all 46 NUL-tail worlds); second run
  `adopted 0 · already 128` — idempotent on the real archives; epoch
  base 2151 = TUniversal `00:00:00:00:35:51`. Operator tool:
  `cargo run -p mirror --example converge_run -- <var-root>`.

- **THERMAL-1 LANDED** (2026-07-27): THE MASTER EQUATION DRIVES THE WORLD —
  with the syntropic encasement (05 §1.2, the operator's expansion ruling:
  the linear entropic-only design refused). The sun commissions
  (StarCommissioned, tag 24; NS-6 §7 StarClass codes fix L); planets site
  their orbits (PlanetSited, tag 25: albedo + orbit r); the tick runs the
  two-armed heat law in exact rationals: stellar load `T_eq⁴/K` + factory
  heat − radiator, ENCASE FIRST (shells mint joules — Resource 5, zone
  kind 5 — the 01 §6.1 commodity supply law made literal), only the
  OVERFLOW vents (yields degrade 1/(1+overflow); OverheatStruck/Cleared,
  tags 22–23, the famine-strike shape); accrued joules leak 1/8 outside
  the cryo vault (T⁴ ≤ 100⁴ lossless — the Frost banks what the Furnace
  mints). Opt-in by construction: no sun ⇒ the pre-thermal engine,
  byte-identical. CLI: `star <class>` · `site <albedo> <r>` · `thermal` ·
  `zone … enc`. Evidence: clippy zero; **111/111 workspace witnesses
  green** (6 thermal: dormant-without-sun; mesic Sol runs clean at exact
  5/4 iron; the furnace mints 15/8 joules/tick with overflow 1/2 venting
  at exactly 2/3 efficiency, strike/clear chronicled once each; the vault
  boundary flips at exactly 100⁴; typed refusals; replay MATCHES with
  star+site+overheat and the orb round-trips byte-exact with tag 26).
  Deferred honestly: cryo-side minting needs converter-scale factories a
  founding colony cannot afford (a measured balance fact); thermal trade
  routes and entropy rights await the economy landings.

- **VENT-1 + ECON-J + SINK-1 LANDED** (2026-07-27): THE TWO-SIDED THERMAL
  BALANCE SHEET. VENT-1: vented overflow accumulates as workspace and seals
  by ONE `VentSettled` receipt at collect (tag 27) into the per-orb
  cumulative `entropy_vented` ledger (orb entry tag 28); conservation
  measured exact (banked + leaked + vented = total excess). ECON-J: sited
  sub-mesic orbs burn joules per active zone for heating; unpaid ⇒ joule
  famine ⇒ FREEZE-LOCK (no flows at all); relief requires the one-tick
  heating bill covered. SINK-1: bankable sink (Resource 6) captured from
  vault-band headroom by cryo-lattice arrays (zone kind 6) — the
  infrastructure gate; the research and throttle-α gates DEFERRED to the
  tech-tree landing (05 §1.4, recorded). Q2's receipted demurrage landed:
  un-vaulted banked joules/sink decay 1/32 per elapsed tick, settled
  openly at touch (`DemurrageSettled`, tag 29, anchored on `last_touch`);
  the vault charges nothing. Observer uplink: `state.json` carries
  star_class, entropy_vented, and the derived (never stored) syntropy
  ratio. Evidence: clippy zero; **116/116 workspace witnesses green**
  (5 new: the books balance exactly at 15/4 + 1/4 + 1 = 5; the cold orb
  freeze-locks; the vault captures 19999/10000 sink and pays no
  demurrage; warm banked joules decay exactly 15/64 over 4 ticks by
  receipt; tags 27/29 byte-exact; replay MATCHES with the dual ledger).

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

## 3. The ideology board (09_IDEOLOGY — the dual cohesion ledger)

Ruled 2026-07-27 (operator relay; design authority `09_IDEOLOGY.md`): love,
compassion, morality, and integrity enter the engine as MEASURED dual-entry
ledgers over receipted deeds — eight cumulative exact-rational counters per
orb, four ratios derived at read, never a stored score. The "unitary
morphodynamics" formulation is REFUSED and recorded (OPEN_QUESTIONS Q3).
The sequencing law of this board: **the ledger cannot precede the deeds it
measures** — each landing opens exactly one axis's books, and the surface
lands last, reading what is already receipted. Everything witnesses
single-world (the P1 archipelago is society enough); cross-empire reading
composes at N5/N6. Candidate tags 30–38 are RESERVED here (14–15 stay
untouched); each is inscribed into NS-1 §2.3 only at its landing.

| # | Landing | Contains | Proven by |
|---|---|---|---|
| **TRADE-1** | THE TRANSFER SUBSTRATE | orb-to-orb transfer of banked resources within one world (`ResourceTransferred`, tag 30 — the chartering conservation precedent, tag 13); demurrage settles both sides at touch (SINK-1 law); opens the COMPASSION books (`given_in_crisis` / `drawn_in_crisis` — posting iff the counterparty is famine- or overheat-struck, uncleared, at the deed's stamp); CLI `send` | a transfer moves exactly what leaves and the two orbs' books balance; a transfer into a famine-struck orb relieves at the 02 §2.4 law and posts to the giver's crisis book exactly; a draw FROM a struck orb posts to the taker's entropic book; replay reproduces the ledger |
| **ABSORB-1** | THE ABSORPTION TRANSACTION | routed overflow: a hot orb vents into a partner's banked sink instead of the void (`OverflowAbsorbed`, tag 31); the 05 §1.4 invariant `sink_consumed = heat_absorbed = vented_after_shed` becomes inter-orb; compensated and uncompensated forms; opens the MORALITY books (`returned_across` / `extracted_across`, the reciprocity window `W`); CLI `absorb` | the three-way invariant measures exact across two orbs; absorbing a struck partner's overflow clears its strike and posts compassion; an uncompensated dump posts to the dumper's extraction book at window expiry, exactly; replay reproduces |
| **BOND-1** | STANDING BONDS | the standing order (`BondSealed`/`BondSevered`, tags 32–33): continuous per-tick flow, workspace-accrued, settled by ONE receipt at touch anchored on `last_touch` (the demurrage shape, tag 29); insolvent settlement breaks the bond as a receipted default; opens the LOVE books (`bond_flow` / `severance_load` — orderly severance posts NOTHING); CLI `bond` / `sever` | a bond streams n ticks and settles the exact accumulated value in one receipt; severing mid-winter drops the standing load on the partner's next heating bill, measured; an orderly severance posts zero; replay reproduces |
| **COVENANT-1** | THE GLASSKEEPER ESCROW | 01 §7 first cut (`CovenantSealed`/`CovenantReleased`/`CovenantSlashed`, tags 34–36): exact-rational conditions over chronicle data, collateral lock, release / BREACH_AND_SLASH; opens the INTEGRITY books (`covenants_fulfilled` / `covenants_breached`, VALUE-weighted — the wash-trading exploit closed at design); collateral requirement scales by the fidelity ratio with the guardrail-6 ceiling; the 09 §3.3 covenant templates; CLI `covenant` | a fulfilled covenant releases exactly and posts its stake; a breach slashes exactly and posts its stake; the collateral curve reads the derived ratio only; a zero-fidelity orb pays the ceiling and is never barred; replay reproduces |
| **IDEO-1** | THE COHESION SURFACE + FRICTION | opens NO books — reads them: the eight counters committed (orb cohesion entry, tag 38), four ratios derived (never stored) on the observer surface (`state.json`, coder-01 relay — the VENT-1 uplink precedent); ideological friction `ΔS_ideo = φ × abs(reciprocity_A − reciprocity_B) × volume` accrued as workspace and sealed by receipt (`FrictionSettled`, tag 37, the VentSettled shape) — friction prices DIVERGENCE, not vice; CLI `ideology` panel, fractions whole | the ratios derive from replay alone and match the committed counters; divergent partners generate exactly the friction entropy and aligned partners (any alignment) exactly zero; tag 37/38 round-trip byte-exact; state.json carries counters, never ratios |

Sequencing: TRADE-1 → ABSORB-1 → BOND-1 → COVENANT-1 → IDEO-1, one landing
at a time under the standing law. TRADE-1 has no new dependency (P1 + SINK-1
landed); ABSORB-1 and BOND-1 are transfer shapes; COVENANT-1 needs TRADE-1
(collateral must move); IDEO-1 is a surface over landed deeds and mints no
physics — its friction rides the landed VENT-1 machinery. Cross-board: the
universe-level composition of the books (Σ per-orb = faction = world) is
Q2.3's N6 mesh audit, not an orb law; trade OFFERS (negotiation, pricing)
await NS-4 at N5 — this board lands the executed deed, not the marketplace.

