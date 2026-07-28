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
| **N5** | NS-4 APERTURE + SHARING | detach a bubble (sector map / trade offer) with permissions and multi-depth content | recipient reads the summary, zooms to detail, cannot exceed granted depth; re-share honors the flag |
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

- **JOURNAL-1 LANDED** (2026-07-27, coder-node worktree, merged by
  coder-01): THE CHRONICLE FEED / TERMINAL WALL (08). Civilizations speak:
  one journal entry per 60-tick year, generated mirror-side as a pure
  RENDERING (no receipts, no new state — deleting every journal changes
  nothing, witnessed). Deterministic from the chronicle alone: year windows
  half-open and closed under the clock law; boundary state grounds through
  prefix replay of the real chain; flavor seeds from the year's last
  receipt digest (CHARTER §9); all quantities exact `p/q`. Full 08 §2
  mapping (joule stance vs the heating bill, famine/freeze-lock RED alerts,
  vent delta, sink offered to Furnace traders, exact growth, expansion
  notes, derived trade requests). Surfaces: `journal.jsonl` per world
  (append-only, idempotent, byte-identical regeneration) + the static
  self-contained cyber-noir `wall.html` (inline-only, filter bar) +
  conductor year-boundary hooks + CLI `journal`. Evidence: clippy zero;
  **123/123 workspace witnesses green** (7 new pins: regeneration
  byte-identical; famine red-alert names resource + dormancy + want;
  growth exact with the odometer pinned; the surplus/deficit stance flips
  at exactly banked == need; wall inline-only with the inscribed palette;
  reflection-only proof; terminology law asserted by code point).
  Deviations recorded in doc-comments: world-local odometer (regeneration
  must not depend on convergence state); two boundary facts witnessed via
  exposed pure helpers; Wanderer archetype label for mesic generalists.

- **TRADE-1 LANDED** (2026-07-27): THE TRANSFER SUBSTRATE — the first
  ideology-board landing (09, §3 below). `ResourceTransferred` (tag 30,
  NS-1 §2.3): banked value MOVES orb-to-orb inside one world — the
  chartering conservation law generalized; typed refusals (BadTransfer,
  NoSuchOrb, CannotAfford with the exact post-decay shortfall). The
  SINK-1 touch law EXTENDED: both sides settle due demurrage by receipt
  at the transfer's stamp (computed pure first — a refusal orphans
  nothing); a same-stamp touch pays zero. THE COMPASSION BOOKS OPEN
  (09 §2.1): `given_in_crisis` / `drawn_in_crisis` post iff the
  counterparty stands in receipted, uncleared famine/overheat at the
  deed's stamp — a replay fact, never a judgment call; weight-1
  commensuration (placeholder table inscribed); wire encoding of the
  books DEFERRED to IDEO-1 tag 38 as boarded. A gift covering the
  one-tick bill relieves at the landed law (02 §2.4). One truth-update:
  every settle is a touch — `UpkeepSettled` now anchors `last_touch` on
  replay (the upkeep-only collect had diverged live-vs-replay,
  latent since SINK-1; caught red, cured). Cross-territory (relayed,
  the P1 precedent): one mechanical renderer arm in `engine/mirror`.
  CLI: `send <amt> <res> to <planet> [from <planet>]`, relief banner,
  refusals name the cure. Evidence: clippy zero (warnings denied);
  **129/129 workspace witnesses green** (6 new: conservation exact —
  20 iron stays 20 across the move, books balance both sides; the
  1/4+1/4 biomass gift relieves EXACTLY at the one-tick bill, receipted
  once; crisis posting replays — drawn 2, given 1/2, healthy transfers
  post nothing; both sides settle demurrage at one touch — 7/64 mother,
  1/8 daughter by receipt, same-stamp pays zero, 41/64 remains; the
  upkeep-only collect anchors last_touch on replay; tag 30 byte-exact).
  The arc played end-to-end: famine struck outpost → 1/4 gift held →
  second 1/4 relieved with the banner → 100-iron refusal named "have
  10" → self-send refused → replay MATCHES.

- **ABSORB-1 LANDED** (2026-07-27): THE ABSORPTION TRANSACTION — the
  second ideology-board landing (09, §3 below). `OverflowAbsorbed`
  (tag 31): a hot orb's UN-SEALED pending overflow routes into a
  partner's banked sink instead of the void — the 05 §1.4 invariant
  `sink_consumed = heat_absorbed = vented_after_shed` made inter-orb,
  the ABSORBER's `entropy_vented` carrying the shed, the source's
  staying clean; emptying the pending CLEARS the overheat strike
  (relief, the famine shape — the imbalance may strike anew). THE
  MORALITY BOOKS OPEN (09 §2.4): every absorption opens a reciprocity
  window (`reciprocity_window()`, placeholder 8 ticks) owed source →
  absorber, held as committed `AbsorptionDebt` state (opened by tag 31,
  retired OLDEST-FIRST by TRADE-1 payments posting `returned_across`,
  lapsed by `AbsorptionLapsed` tag 32 — world-driven at the exact
  stamp, the unpaid remainder posting `extracted_across`; returned +
  extracted sum to the drawn whole). Compensated and uncompensated are
  ONE mechanic, two outcomes, measured by behavior — the engine judges
  nothing. The absorber settles due demurrage at the touch,
  affordability post-decay, computed pure. Tag note: 32 spent here on
  the lapse; the remaining board candidates renumber (bonds/covenant/
  friction in 33–38; the orb cohesion entry takes a post-40 tag at
  IDEO-1 — contiguity is not law; 39–40 stay tech-emergence).
  Cross-territory (relayed): two mechanical renderer arms in
  `engine/mirror`. CLI: `absorb <amt> from <planet> [into <planet>]`,
  ♻/❄ banners, `CannotAbsorb` names the standing pending. Evidence:
  clippy zero (warnings denied); **134/134 workspace witnesses green**
  (5 new: the three-way invariant exact across two orbs — 59997/10000
  sink minus 1, vented 1 on the absorber, source ledger clean at 1/2,
  strike cleared once; the unpaid window lapses by receipt at exactly
  stamp 27 = opened 19 + 8; partial absorb leaves the strike standing
  and oldest-first pay splits the books 1/2 returned + 1/2 extracted =
  the drawn whole; refusals name the exact pending 5/2 and sink
  shortfall; tags 31/32 byte-exact). The arc played end-to-end:
  freeze-relief by TRADE-1 → capture → absorb cleared the strike with
  the banner → the paid window lapsed nothing → replay MATCHES.
- **BOT-THERM LANDED** (2026-07-27, coder-node worktree): NPC THERMAL
  ADOPTION — the autarkies feel the star. Thermal genesis: every bot
  world founds under a REAL sun — `BotWorld::genesis` commissions the
  star and sites the planet through the landed THERMAL-1 engine (no
  engine code touched). Specs carry optional `star`/`albedo`/`orbit`
  fields (bots.json spellings "g", "m", "red-giant", …); absent fields
  derive from the registry axes (mineral→Furnace seat, biotic→mesic
  seat) through the chronicle beacon seeded by the world's OWN genesis
  receipt digest (CHARTER §9 — no wall clock, no ambient entropy; two
  foundings are byte-identical, witnessed). Recorded ruling: a Frost
  seat is never derived — settling the vault is a deliberate act; the
  seeded default file makes one: Frostwright (wood frog, NEW founding
  bot) on an M sun at unit orbit (the cryogenic vault), Slagborn pinned
  to a Furnace seat (G sun, albedo 1/2, half orbit — stellar load 2),
  the three mesic seats left underived-field so the derived path runs
  in every era. Thermal temperament in the loop (existing laws held:
  one build per beat, through the contained actor, never in famine):
  Furnace bots answer a heat ledger past their shell throughput with
  an encasement shell FIRST — the joules mint — under two priority
  rules earned red (below): shells come from SURPLUS (the colony
  hoards until shell + next extractor are both funded), and industry
  outranks new settlement (when only the biomass line blocks an
  unlocked smelter, the rations are held for the forge). Frost bots
  raise their sink charter (one cryo-lattice array on the founding
  iron), then hold every further build against the joule runway
  (`joule_runway_allows`: banked joules ≥ heating_per_zone × (zones+1),
  exact, boundary-inclusive) — famine avoidance honestly allowed to
  fail: Frostwright freeze-locks in the default era and the wall calls
  it RED, legitimate drama, not a bug. Mesic bots' temperament is
  untouched (their worlds are merely sited now — late-era factory heat
  can overflow; that is weather, not a law change). CAUGHT RED, CURED:
  the first full run broke `research_unlocks_industry` — a naive
  "shell when hot" Furnace bot spent its seed ore at beat 2 and
  flipped Slagborn from extractor-compounding (10 ext / 6 hab
  pre-thermal) into a starving habitat city (2 ext / 13 hab, no
  smelter by beat 45); the surplus-hoard and industry-hold rules are
  the cure (measured at beat 45: 5 ext / 3 hab / 2 shells / smelter
  standing / famine-free). Pipeline witnessed
  end-to-end, nothing re-implemented: star_class flows to state.json
  (mirror), year boundaries journal the drama (JOURNAL-1): the Frost
  year reads "RED ALERT — freeze-lock", heating bill 15 exact, 199/100
  S_cool offered to Furnace traders under the Coherent register; the
  Furnace year reads Emberkin, 11/4 vented ("the shells want
  widening"), an exact-rational joule bank.
  Phase 2 (trade execution against the wants the wall now shows)
  stays boarded, not landed. bots.json compat: pre-thermal files load
  unchanged (absent fields = derived seats); an unlawful pinned siting
  leaves the orb thermally dormant instead of killing the board.
  Evidence: clippy zero; **134/134 workspace witnesses green** (129
  baseline + 5 new pins, red-first recorded): thermal genesis
  deterministic on both derived and pinned paths; a default era mints
  hot AND banks cold with state.json carrying star_class 7; the joule
  runway measurably defers the Frost (1 zone vs the mesic control's
  city) and flips at exactly 1/2 vs 49/100; a thermal year reaches the
  wall carrying joule-bank/heating-bill/FREEZE-LOCK/S_cool and the
  Emberkin/Coherent registers; the old bots.json shape still loads and
  derives.

- **BOND-1 LANDED** (2026-07-27): STANDING BONDS — the third
  ideology-board landing (09 §2.2, §3 below). `BondSealed` (tag 33:
  rate/tick + term, 0 = open-ended; one bond per source/target/
  resource) and `BondSettled` (tag 34): the accumulated span settles by
  ONE receipt at each touch — collect, sever, term expiry — anchored
  like demurrage; its OWN tag by necessity (a manual transfer must stay
  distinguishable from bond flow or replay could not re-anchor). THE
  LOVE BOOKS OPEN: `bond_flow` (source's syntropic book) /
  `severance_load` (the cutter's entropic book = rate ×
  `bond_notice_window()`, placeholder 8). An ORDERLY end — the bond run
  to its term — posts NOTHING (leaving is not a sin); a manual sever or
  an insolvent settle posts the load (the default IS the mid-winter
  cut; recorded v1 rulings: the source side cuts; expiry-insolvency
  posts zero — the term was served; settle sites are collect/sever/
  expiry). Bond settlements post the love book only — the crisis books
  belong to deliberate deeds — but still compensate open ABSORB-1
  windows (settle_debts shared). Bond state committed (in Committed,
  the AbsorptionDebt precedent). Cross-territory (relayed): two
  mechanical renderer arms in `engine/mirror`. CLI: `bond <rate> <res>
  to <p> [for <term>]` · `sever <res> to <p>` · `bonds`, the ⚭/✂
  glyphs. Evidence: clippy zero (warnings denied); **146/146 workspace
  witnesses green** (6 new: a 6-tick span settles exactly 3 iron in ONE
  receipt and re-anchors; severing the heating lifeline mid-winter
  posts load 2 and the freeze returns on schedule — measured; a term
  bond streams exactly rate × term and expires posting NOTHING; an
  insolvent settle pays every iron it holds (15) and defaults with load
  32 in the same receipt; refusals typed incl. BondAlreadySealed /
  NoSuchBond; tags 33/34 byte-exact). The arc played end-to-end: the
  lifeline sealed → streamed at the touch → severed → the coldstore
  froze again → replay MATCHES.

- **COVENANT-1 LANDED** (2026-07-27): THE GLASSKEEPER ESCROW — the
  fourth ideology-board landing (09 §2.3, 01 §7 first cut, §3 below).
  The vitrified delivery pact (tags 35–37: CovenantSealed / Released /
  Slashed): the obligor locks collateral OUT of its bank — the lock
  gates every spend by ABSENCE (no second mechanism), and the lattice
  vaults it losslessly by construction — against "deliver ≥ quota
  within the window", measured from DELIBERATE deeds (TRADE-1
  transfers + BOND-1 settlements; absorption recorded as non-counting).
  Resolution is world-driven at the tick boundary (the famine shape):
  fulfilment releases home, the passed deadline slashes to the
  aggrieved — fulfilment wins the tie; vitrification means NO CANCEL.
  THE INTEGRITY BOOKS OPEN, VALUE-weighted (`covenants_fulfilled` /
  `covenants_breached` post the collateral stake — wash-trading closed
  as designed). The fidelity curve landed (`collateral_required`,
  placeholder): quota × (breached+1)/(fulfilled+1) clamped [1/4, 4] —
  the empty book is not a credential (full collateral), the ceiling is
  guardrail 6 made literal (zero fidelity pays 4×, NEVER barred); the
  priced collateral rides the seal receipt so replay never re-runs the
  curve. Recorded v1 rulings: collateral denominated in the pact's
  resource; one open pact per lane; the delivery-quota pact IS the
  landed 09 §3.3 template (01 §7.1's own example) — the Compassionate
  and Bond Covenant templates deferred; resolution banners deferred to
  the journal (chronicle carries the drama). Cross-territory (relayed):
  three mechanical renderer arms in `engine/mirror`. CLI: `covenant <q>
  <res> to <p> within <n>` · `covenants`, the ⛨/⚔ glyphs. Evidence:
  clippy zero (warnings denied); **153/153 workspace witnesses green**
  (7 new: a fulfilled pact releases exactly — 10−3−2−1+3 = 7 by the
  books; the missed deadline slashes 3 to the aggrieved at exactly
  stamp 21; the curve reads the books — 4 → 1 at fulfilled 3, floor
  1/4 and ceiling 4× clamped, receipt-verified; locked collateral
  refuses zone AND transfer with "have 7"; BOND-1 composes — the
  stream fulfils the quota and the bond outlives the pact; refusals
  typed incl. CovenantAlreadySealed; tags 35–37 byte-exact). The arc
  played end-to-end: vitrified → delivered → released; the doomed
  biomass pact slashed on schedule; replay MATCHES.

- **IDEO-1 LANDED** (2026-07-27): THE COHESION SURFACE + IDEOLOGICAL
  FRICTION — the fifth and FINAL ideology-board landing (09 §2.5,
  §3.2): the board CLOSES. Opened NO books — read them. (1) The orb
  cohesion entry (tag 41, the first post-40 tag; 39–40 stay
  tech-emergence; next free 42): the eight books enter the orb wire in
  fixed order, emitted ONLY when a book is nonzero — a zero-book orb
  encodes byte-identical to a pre-ideology one (SEAL A, witnessed).
  (2) The observer uplink (`state.json`, `engine/mirror` by relay —
  the VENT-1 precedent): the eight committed counters + the four
  derived (never stored) axis ratios via the landed `cohesion_ratio`
  helper. (3) Ideological friction: `ΔS_ideo = φ (placeholder 1/8) ×
  abs(reciprocity_A − reciprocity_B) × volume` on TRANSFERS only —
  recorded ruling: bonds and absorptions are EXEMPT by design (the
  standing loop is the friction-free channel; long-horizon love ducks
  the tax the way it already ducks demurrage); accrues as workspace
  against the mover, seals by `FrictionSettled` (tag 38, the
  VentSettled shape) into `entropy_vented` — real entropy. Friction
  prices DIVERGENCE, not vice: empty morality books read as 1
  (placeholder ruling — a fresh orb trades frictionless until
  measured); aligned partners at ANY alignment generate exactly zero
  and no receipt. (4) CLI `ideology`: the four axes in words + exact
  fractions ("compassion gave 3 · drew 0 — ratio 1"), never matrix
  algebra. Cross-territory (relayed): one renderer arm + the cohesion
  block in `engine/mirror`. Evidence: clippy zero (warnings denied);
  **158/158 workspace witnesses green** (5 new: the cohesion entry
  round-trips byte-exact and the empty ledger costs no bytes; the
  ratio derives — 1, 1/2, 0, and no-deeds-None; the measured extractor
  (lapsed window, reading 0) pays exactly φ × 1 × 2 = 1/4 sealed by
  receipt into its vented ledger, replay MATCHES; two unmeasured orbs
  generate exactly zero friction and no receipt; tag 38 byte-exact).
  The arc played end-to-end: absorb → lapse → the divergent transfer →
  the ideology panel read "compassion ratio 1 · morality ratio 0" on
  one orb — the engine judged nothing and measured everything.

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

