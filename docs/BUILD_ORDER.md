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
  extracted byte-exact (genesis predecessor zero; chain `Intact`); tamper
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
- **N4 LANDED** (2026-07-30, coder-03): NS-2 RQP + THE SCANNER — the lens
  beside the mirror. The spec (§1–§4) written FROM the v0.4 sigil index
  with vectors extracted from the running engine; the operator's lens law
  recorded: a query is a pure read, only the crystal draw receipts
  (tag 55, blind to what was asked). The sigil index
  (`engine/cfe/src/query.rs`): ONE joint canonical shadow over the eight
  grid symmetries, marginal projections as the O(bucket) address, T
  excluded from the key. Crystals + the resonator (codes 7/7) paid the
  02 §5.1 deferral on the one-synthesizer knife edge. Evidence: clippy
  zero; the board's acceptance met exactly — the planted match found
  through rot90; the kind-swap near-miss refused with C(Δ) = 0 while its
  delta is real (the scalar is the summary, never the payload); the
  banked near-miss refused iron −5, biomass −2 whole, C(Δ) = 29 exact;
  the lens leaves ONE receipt; a crystal-less scan refuses pure; replay
  MATCHES; the CLI arc played end-to-end `--plain`. Landed at 194/194
  workspace witnesses.
- **U0 + PROPORTION-1 LANDED** (2026-07-31, arch-01): the unit lane
  opens. U0 — the spine judge fixture seats its actors (sovereign +
  contained hand with exactly Collect; I2 honored), surfacing the
  board's TRUE count: **285/285 green, clippy zero** (the prior 240 was
  a fail-fast undercount — cargo stops at the first red binary).
  PROPORTION-1 — THE PROPORTIONAL LAW (operator ruling; the name
  LINEAR is DEAD in the workspace, per 12_UNION's no-uni-directional-
  linearity doctrine): squares are BARRED as aggregators of difference
  — `coherence_functional` and `conservation_measurement` sum `|R|`,
  `norm_squared` → `magnitude` (‖Δ‖₁), the scanner scalar likewise;
  each difference counts exactly its size IN ITS OWN UNITS, natively
  additive across the fiber's direct sum; squares survive ONLY as
  physical law (05's T⁴ discipline). Evidence: red-first honored at
  both flips (witnesses re-pinned 24→12 RFA / 29→7 scanner, red
  recorded, then green); the tag-58 wire vector RE-EXTRACTED from the
  running engine (`ns2_vectors`, byte `1d`→`07`) and inscribed in
  NS-2 §3; law amended in kernel CHARTER §3, SYSTEM_SPEC §0/§4,
  03_SUBSTRATE_CONTRACT §5; dated records stay quoted (this row's N4
  neighbor speaks 29 as HISTORY). 285/285, clippy zero. Design
  authority now also includes `16_UNIT.md` (the unit law + §4a the
  singularity law), `docs/EQUATIONS.md` (THE ONE LEDGER — the
  row-claim convention is live), `docs/U_DIRECTIVES.md` (the unit
  work orders; U1 next, tag 86 reserved there).
- **U1a LANDED** (2026-07-31, arch-01): THE PROVER/VERIFIER SPLIT —
  `engine/solver-verify`, the sealed replay half: grid + the ONE apply
  path, loader, the deterministic forge, the certificate chain, and
  `objects` (discovered at the split: `Op::Object(ObjOp)` makes the
  object ops APPLY vocabulary — only the SEARCH is prover-side).
  `solver` keeps search + volume, re-exports whole; no caller broke.
  THE OPAQUE-PAYLOAD LAW derived and inscribed (U_DIRECTIVES §2):
  physics' cfe arrow forbids cfe importing the verifier, so the deed
  carries proof BYTES under solver-verify's encoding monopoly — the
  kernel calls `verify_work(&bytes)`, never parses. 285/285, clippy
  zero.
- **U1 KERNEL-VERIFY LANDED** (2026-07-31, arch-01): THE PoUW ANCHOR —
  I4 completes. Tag 86 `DiscoveryMinted` (header-claimed): orb ‖
  resource ‖ amount ‖ work, the proof riding WHOLE; replay banks it
  like a yield, the proof is judged at the door. `verify_work` — THE
  REPLAY-COMPLETE LAW: re-forge (the task IS its digest) ∧ re-apply
  (every training pair exactly, the honest-withhold law) ∧ re-chain
  (the certificate re-derives or names its link) — an INTACT chain
  over a non-solution refuses `TrainingMiss`: the fabricated-render
  attack is dead. `mint_bound = 0` SEALED SHUT until Ruling Box 1
  opens the rate by amendment (the mechanism precedes the economics).
  New refusals `WorkRefused{broken}` + `MintExceedsWork{bound,
  claimed}`, measurements whole. Evidence: red-first honored — seven
  pins written against unflipped coverage, six recorded red
  (`Unjudged`, the sealed default held), the amendment flipped them
  green; 292/292, clippy zero; EQUATIONS I4 row claimed same landing.
- **U1f LANDED** (2026-07-31, arch-01): I5 COMPLETES — the decay
  bound. The kernel CALLS the sealed demurrage rule
  (`cfe::world::demurrage_due` + `ease_of` made pub — call-not-copy,
  one computation two judges): decay ≤ min(rate·ease·elapsed, banked),
  flat, thermally gated; `elapsed` reads the prior orb's `last_touch`,
  which ALREADY rode the orb wire (tag 28, VENT-1) — the SCHEMA_V3
  premise died at execution, no schema spent. New refusal
  `DecayExceedsBound{bound, claimed}`. Evidence: red recorded (the
  sign-only kernel ADMITTED the excess — the hole measured), then the
  arm; one witness truth-update recorded (the sign-era "decay of 2
  admits" pin sat on a starless world — the rule owes nothing there;
  it now refuses with bound 0 carried, and says so). 295/295, clippy
  zero; EQUATIONS I5 row claimed. **The kernel now judges 10 of 64
  deed types with I1–I5 whole; I6 (U2) and the W-waves remain
  (U_DIRECTIVES §5).**

## 1. The board

| # | Landing | Builds | Proven by |
|---|---|---|---|
| **N0** | THE SCAFFOLD | the monorepo tree, the charter, this carrier, protocol stubs | the tree exists; the charter read whole |
| **N1** | NS-3 VRP, spec-from-working-code | the receipt spec written FROM the v0.4 chronicle organ (BLAKE3 append-only, predecessor-linked, quorum-stamped) with test vectors extracted from real chronicle output | the spec's vectors verify against a fresh engine implementation; tampering/reordering refuses typed |
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
  a charted colony's founding reserves MOVE from a source colony's bank —
  conservation holds, chartering mints nothing (the free-bootstrap exploit
  closed at design time). Per-orb workspaces and famine; planet names
  unique per world. Evidence: clippy zero; **77/77 workspace witnesses
  green capped** (chartering conservation; typed refusals incl.
  PlanetNameTaken/NoSuchOrb; a charted famine at exactly t=11 never
  touches the source; the archipelago replays from receipts alone). CLI:
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

- **THERMAL-1 LANDED** (2026-07-27): THE EQUILIBRIUM BASELINE DRIVES THE WORLD —
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
  post nothing; both sides settle demurrage at one touch — 7/64 source,
  1/8 charted by receipt, same-stamp pays zero, 41/64 remains; the
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

- **FORWARD-1 LANDED** (2026-07-27): THE GLASSKEEPER ESCROW — the
  fourth ideology-board landing (09 §2.3, 01 §7 first cut, §3 below).
  The vitrified delivery pact (tags 35–37: ForwardSealed / Released /
  Slashed): the obligor locks collateral OUT of its bank — the lock
  gates every spend by ABSENCE (no second mechanism), and the lattice
  vaults it losslessly by construction — against "deliver ≥ quota
  within the window", measured from DELIBERATE deeds (TRADE-1
  transfers + BOND-1 settlements; absorption recorded as non-counting).
  Resolution is world-driven at the tick boundary (the famine shape):
  fulfilment releases home, the passed deadline slashes to the
  aggrieved — fulfilment wins the tie; vitrification means NO CANCEL.
  THE INTEGRITY BOOKS OPEN, VALUE-weighted (`forwards_fulfilled` /
  `forwards_breached` post the collateral stake — wash-trading closed
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
  typed incl. ForwardAlreadySealed; tags 35–37 byte-exact). The arc
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
| **FORWARD-1** | THE GLASSKEEPER ESCROW | 01 §7 first cut (`ForwardSealed`/`ForwardReleased`/`ForwardSlashed`, tags 34–36): exact-rational conditions over chronicle data, collateral lock, release / BREACH_AND_SLASH; opens the INTEGRITY books (`forwards_fulfilled` / `forwards_breached`, VALUE-weighted — the wash-trading exploit closed at design); collateral requirement scales by the fidelity ratio with the guardrail-6 ceiling; the 09 §3.3 covenant templates; CLI `covenant` | a fulfilled covenant releases exactly and posts its stake; a breach slashes exactly and posts its stake; the collateral curve reads the derived ratio only; a zero-fidelity orb pays the ceiling and is never barred; replay reproduces |
| **IDEO-1** | THE COHESION SURFACE + FRICTION | opens NO books — reads them: the eight counters committed (orb cohesion entry, tag 38), four ratios derived (never stored) on the observer surface (`state.json`, coder-01 relay — the VENT-1 uplink precedent); ideological friction `ΔS_ideo = φ × abs(reciprocity_A − reciprocity_B) × volume` accrued as workspace and sealed by receipt (`FrictionSettled`, tag 37, the VentSettled shape) — friction prices DIVERGENCE, not vice; CLI `ideology` panel, fractions whole | the ratios derive from replay alone and match the committed counters; divergent partners generate exactly the friction entropy and aligned partners (any alignment) exactly zero; tag 37/38 round-trip byte-exact; state.json carries counters, never ratios |

Sequencing: TRADE-1 → ABSORB-1 → BOND-1 → FORWARD-1 → IDEO-1, one landing
at a time under the standing law. TRADE-1 has no new dependency (P1 + SINK-1
landed); ABSORB-1 and BOND-1 are transfer shapes; FORWARD-1 needs TRADE-1
(collateral must move); IDEO-1 is a surface over landed deeds and mints no
physics — its friction rides the landed VENT-1 machinery. Cross-board: the
universe-level composition of the books (Σ per-orb = faction = world) is
Q2.3's N6 mesh audit, not an orb law; trade OFFERS (negotiation, pricing)
await NS-4 at N5 — this board lands the executed deed, not the marketplace
(the market board, §3a).

## 3a. The market board (01_ECONOMY §3, §8 — the marketplace N5 unlocks)

The ideology board lands the executed deed; this board lands the marketplace
the charter defers to N5 (§3 close). It opens AFTER NS-4 APERTURE gives
value a cross-world carrier — an offer is an aperture opened at a granted
facet, priced. Everything is exact-rational, receipted, replayable; the render
mints no physics (reflection law). Candidate tags 49–51 RESERVED (the
original 41–43 reservation collided with the registry — 41 is IDEO-1's
cohesion entry, 42–46 AEON-1 — and 47–48 landed as the APERTURE pair);
each is inscribed into NS-1 §2.3 at its landing. Sequencing: MARKET-1 →
OFFER-1 → BOARD-1, one at a time.

**APERTURE-1 LANDED** (2026-07-29): THE CROSS-WORLD VALUE CARRIER (NS-4
§1–§3, tags 47–48). `aperture_transfer` moves banked value between two
worlds as a two-legged deed sharing one deal id — `ApertureSent` debits
the source, `ApertureReceived` credits the destination; both orbs settle
due demurrage at the touch (SINK-1 law); affordability judged pure on the
source's post-decay bank, so a refusal orphans nothing in either world;
the destination leg relieves famine at the 02 §2.4 law; the cohesion
books do NOT post (an aperture carries value, never judgment — cross-world
reading composes at N5/N6). Pair conservation is an N6 mesh-audit reading,
never an orb law. Evidence: 4 witnesses green (`world_aperture.rs` — pair
conservation exact, typed refusals move nothing, per-world replay, tags
47/48 byte-exact); clippy zero; mirror + CLI chronicle render both legs.
The facet machinery (grant depths, permissions, expiry) stays ahead at N5
— this landing is the carrier the market board prices.

| # | Landing | Contains | Proven by |
|---|---|---|---|
| **MARKET-1 — LANDED 2026-07-30** | THE EXCHANGE RATE | tags 49–50 wired: offers as RESTING receipted intents (no escrow — affordability judges at the fill; the price IS the ratio, guardrail 6); `clear_trade` fills at the RESTING offer's exact price (price-time priority, ties by id), moves as two aperture pairs (deal, deal+1 — shield-checked, conservation-exact both directions), closes both intents; CLI `offer …` / `trade …` verbs; both books replay | witnessed: the 3-iron fill at the resting 1/2 price exactly (the late bidder keeps its surplus); conservation across both resources and both worlds; the unmatched offer rests and replays; NoCrossing moves nothing; the shield guards the fill |
| **OFFER-1a — LANDED 2026-07-30 (first slice)** | THE AUTOMATONS TRADE | `trade_intent` (pure, regime-read): a joule-starving Frost with banked sink offers S_cool for fuel; a Furnace with surplus (> 4 J, placeholder) offers fuel for S_cool — halves of stock, never the whole (guardrail 1's spirit); v1 price 1:1 (05 §1.4 equilibrium pricing is the open design); `market_step` posts one intent per world (offer id = chronicle length, deterministic) and clears the first crossing per beat; wired into the conductor's heartbeat with the narrator line | **OFFER-1b WITNESSED 2026-07-30**: the survive-the-freeze arc whole — a shelled Furnace banks surplus and forms its intent; an arrayed Frost freeze-locks in joule famine with banked sink and forms its own; ONE market beat clears the crossing, the famine RELIEVES through the fill, the thawed world flows again, both books carry OfferCleared, and replay reproduces the rescue from receipts alone. Still open: 05 §1.4 equilibrium pricing (the design doc is unwritten — the 1:1 parity stays a named placeholder until the operator rules the spec) |
| **BOARD-1 — LANDED 2026-07-30** | THE TRADE BOARD | the resting offers export to the state uplink as DATA fields (never prose); the observer gains the `market` view (standing intents per world + crossings-cleared count by event kind) and the observatory gains the TRADE BOARD panel; the render mints no physics, prices stay ratios. Deferred, recorded: the wall.html journal trade lines (JOURNAL voice work) | verified live: a player's resting offer and six autonomous clears render from state alone |

Dependency: the carrier gate is OPEN (APERTURE-1 above). MARKET-1 needs the
APERTURE deed to price; OFFER-1 needs MARKET-1's matching; BOARD-1 reads
what OFFER-1 posts; the facet grants (N5) refine but do not gate the board.

## 3a-bis. The development ladder (operator direction, 2026-07-30) — tier 0 down to the opposite pole

Four working fronts, walked from the board-makers down to the manifold's
other cone tip (the exact opposite civilization type):

- **T0 — the progenitor campaign.** The `t0` console grows into the full
  board-maker game: the inbox as a queue of adjudicable petitions, stress
  readings promoted from the observer into typed deeds (SEPARATE — `t0
  move` parts warring factions in S; RELIEVE — a licensed leap or
  re-seat for stagnation; SEAL — a targeted fold for diplomatic
  collapse), and the fold played as an act with consequences the next
  aeon inherits.
- **T1/T2 — the watcher and keeper loops.** Watchers adjudicate the
  upward petition flow (grant/refuse aperture and licensing requests
  under the shield law); Keepers play the ledgers — auditing aperture
  endpoints against the INTEGRITY books and keeping the polytonic
  calendar (the base-7 weekly voice is their trigger surface: they SAY
  when the stack is read).
- **T3 — the biosphere game.** The landed colony loop plus SCIENCE-1:
  each line solves ITS OWN science's puzzles toward its own tree, and
  the joint research venture (council + aperture + licensed leap) is
  the co-op endgame — incompatible sciences trading solved outputs to
  advance where neither could alone.
- **The ENTROPIC SHEET (layered-wrap ruling, 2026-07-31 — the
  "opposite pole" chapter is STRUCK; a T∞ appearing anywhere means a
  manifold layer went unmodeled: infinity is disallowed, the torus
  wraps finitely).** The entropic boundary is an ARCHIVED FOLD SHEET
  in the stack (each aeon seats at archived/12, wrapped and finite)
  — not a pole, not a tip, no cone. The unmaker class plays that
  sheet's leg: claiming derelicts, recycling ruins, absorbing vented
  heat, riding demurrage — every expansion's bounded counter-force,
  wrapping deterministically within exact rational bounds. Design law
  unchanged: never evil, never destruction-of-record (SEAL B and the
  fold's archival law hold) — the game is complete when both legs of
  the manifold are in play on their sheets.

## 3a-ter. The wiring audit (2026-07-30) — every deed reaches a hand

Six landed deeds were engine-complete but SURFACE-ORPHANED (no caller
outside tests). All wired into the client, and the whole gift chain
played end to end through the CLI alone: found two worlds → `council`
seals → `aperture` crosses (both chronicles synced atomically — a
two-legged deed is never left half-saved) → `license 1..5` builds the
receiver's stack → `license 6` GIFTS THE CROWN → `resume` the receiver
→ `tech` shows seven held → `terraform` moves the orbit across bands.
New verbs: `elevate <delta>` · `age <class>` · `council …` ·
`aperture … deal <n>` · `license … via <council> deal <n>`. Cross-world
verbs reach CLI-saved worlds (the tlv resume carrier); polity worlds
are mirror-only and refuse with the cure named. Two deeds stay
LAWFULLY unwired, by design not neglect: `conflue` (confluence is
BOND-earned, and cross-world bonds await N5/N6 — a free confluence
verb would violate 11 §3), and world-level `seal_fold` (the board-level
fold is the conductor's `--fold` / `t0 contract`; the per-world seal
joins a future per-world fold flow).

## 3a-quater. The second look (2026-07-30) — the summit and the sheets

Two holes found above the wiring audit's floor, both closed:

- **Upward advancement now reaches FRUITION.** The climb had no summit:
  no deed claimed a seat. `Registry::seat` lands (promotion through the
  same door as creation — known office, open room, progenitors per
  domain) and the CLI `ascend <sigil> to <station>` gates it on the
  world's own receipts (crossed + a receipted climb). Proven live end
  to end: leap → loop → crossing → `elevate` SUCCEEDS (the key plays at
  1/6) → the line ASCENDS to a watcher seat. A seat is reached, never
  appointed — now mechanically true.
- **Dimensional collapse now separates the sheets.** The fold WROTE the
  advanced base gauge but convergence never READ it — every aeon
  stacked at one G-height, the exact recurrence collision 11 §2.0
  exists to prevent. Wired both ways: convergence seats each annexed
  era at its sheet (`(N−1)/12` from the era ordinal, the world's own
  key riding above), and the LIVE aeon geneses on the sheet above every
  archive (base = archived-count/12). Pinned: two lived-and-folded
  aeons chart the same world at DIFFERENT G-heights; the third
  convergence re-creates a live board so the walk continues; only a
  truly empty root refuses the fold.

Expansion's counter-force (the manifold dual made mechanical): the
occupancy caps and four-C fold-eligibility remain the recorded open
half of AEON-1b — expansion is bounded today by seats and the fold,
not yet by the trajectory reading.

## 3a-quinque. THE ONE G (the alignment seam, 2026-07-30)

The operator's instrument read the board's G as "binary — 0 or 10,
nothing between." The law was never binary; the CHART was bilingual:
genesis entries carried the KEY (the wrapped exact rational of the
AEON law — 0 before any climb) while annexed entries carried the
four-C deposit SUM (an event-count projection — ~10 registrations and
grants), two quantities sharing one axis. Ruled and fixed: the chart's
G axis carries the KEY alone — the world's terminal gauge receipt
(GaugeShifted · Diverged · Confluent · EraFolded, last stands) plus
its era's sheet base; the deposit projection keeps its lawful job
(four-C placement, the fiber card) OFF the G axis. Recorded plainly
against the imported analysis: there is no G=10 pole (the fiber wraps
[0,1) — a G of 10 IS 0, the recurrence collision itself); sheet
offsets derive from era ordinals, never from a ceiling; ascension
claims a seat and moves no key.

## 3a-sexies. GAUGE-N (2026-07-30) — from one dial to n planes

The operator's read: "key,value — 2D:2D — we need n-dimensional." The
substrate always was (the physics Triple carries graded AxisVectors);
the WORLD's key was the scalar bottleneck. Landed: the key is a graded
vector of wrapped exact rationals — `GaugeShiftedAt` (tag 60; moved
off 55 when the scanner claimed it mid-cadence — one tag, one meaning)
moves or OPENS any grade, skipped planes rest at ground, grade 0 stays
the landed scalar every law binds to (tags 42–46 untouched), the whole
key replays and resumes, and `key_distance` reads wrapped L1 across
keys of ANY dimensionality (resonance/clash, 11 §2.1). CLI `shift
<grade> <value>` is the progenitor's quantum-state deed; the chart's G
axis stays grade 0 + sheet base (higher planes are the fiber card's
country). Plane-opening is now EARNED (refined 2026-07-30): opening grade k
requires k receipted mints — crystallized or granted nodes (condition
≥ 1), divergences, confluences — `PlaneUnearned` names the fiber's
true growth otherwise; moving an already-open plane stays free.
Witnessed: zero mints refuse grade 2; a cleave and a recombination
open it.

## 3a-septies. The consoles hardened (2026-07-30, tasks 24–25)

- **The priority inbox:** `t0 inbox` aggregates the upward flow into
  three ranked buckets — CRITICAL (sector stress from the mirror's
  state data: the want striking, venting), PETITIONS (the steward
  inbox per world), DISPUTES (receipted council breaches) — three
  lines for a progenitor, drill-down capped at eight, never spam.
- **The resume carrier:** every polity world now syncs an
  append-only `chronicle.tlv` beside its mirror each beat. The
  OBSERVING hand can resume a copy; the WRITING hand stays refused at
  the CLI (the two-writer guard: a conductor-owned world's clock
  lives in its process — a direct write would fork the chain; NS-5
  arbitrates, the steward inbox intervenes until then).

## 3a-octies. Beta blockers A1 + A3 CLOSED (2026-07-30)

- **A1 — the board remembers.** Conductor genesis RESUMES before it
  founds: a verifying `chronicle.tlv` continues the world whole
  (receipts, techs, keys, trades; agents re-found by handle; the
  forge state reconstructs from the solves artifacts — an honest
  approximation until solves become receipts; one-divergence-per-era
  read from the Diverged receipt). Pinned to the receipted-truth law:
  the terminal digest continues across a restart — resumption, never
  a refound; un-receipted ticks honestly do not survive (the
  workspace invariant applies to time itself); zero ruins rotate.
- **A3 — the role is the gate.** `login <name> as <station>` binds an
  authority to the profile (passwordless per ruling); the tier-0
  verbs (t0 · license · shift · age) check it and refuse with the
  cure named. Save preserves the station.

## 3a-nonies. B1 + B3 + the visible player (2026-07-30)

- **BOARD-1 landed** (row above): the market stopped running invisibly.
- **The epistemological want reads on the board** (WANT-1 kind 1): a
  world whose REACH (wrapped L1, placeholder 16 — the frontier's kin)
  contains broken chains or `--closed-*` ruins reads ◌epistemological
  on the stress views — truth-starvation as a BOARD reading, by
  design (worlds cannot see neighbors; the board can). The struck
  cognitive wants also export to the state uplink and flag on the
  observatory.
- **CLI worlds mirror on save** (#37): a player's saved world reflects
  beside the NPCs (civilization = the sealed line, else the active
  operator) — testers see their own worlds on every board.

## 3b. The union board (12_UNION — the world-book of the new aeon)

Design authority `12_UNION.md` (eight operator rulings, 2026-07-29). NOT a
sequence — the implementation manifold (12_UNION §9): four entry lanes,
two confluence nodes, any edge-respecting walk lawful. Tags 49–51 stay the
market board's; union tags allocate from 52 at each landing.

| Landing | Lane | Status | Contains |
|---|---|---|---|
| **SUBSTRATE-1** | C | **LANDED 2026-07-29** | the no-anthropic-gate axis growth (12_UNION §5a): metabolism gains `plasmic` (sites Furnace — the stellar environs), perception gains `umbral` (occlusion-basis, the syzygial lines); APPEND-ONLY (the tag-registry discipline on the axes — never removed, never renamed); no new event tags. Evidence: the vocabulary witness registers a plasmic/umbral line, round-trips whole, and still refuses what was never added |
| **LINEAGE-1** | C | **LANDED 2026-07-29** | `resolve_head` walks the lineage pointer chain to its root — headless lawful (a rootless line resolves to itself), a dangling pointer resolves to the deepest recorded predecessor, a corrupt cycle stops deterministically; `lineage_domain(metabolism, band)` derives the progenitor domain a line elevates toward (thermal/lithic/cryo/mesic/plasmic), never stored, never chosen — no line answers to a planner of an alien physiology. No new event tags. Evidence: the chain/headless/dangling/cycle/domain witness green |
| **AEON-1a** | B | **LANDED 2026-07-29** | the manifold core: tags 42–46 byte-exact; the gauge key G on every world (exact rational wrapped [0,1), zero at genesis, moved only by receipt — the wrap is receipted, never re-derived); divergence/confluence as KEY CHANGES (peer cleave named; interference midpoint caller-computed); the star moves in its region (`age_star` — a reddening G sun shifts every orbit's T_eq⁴ a thousandfold, replayed); the fold receipt (`seal_fold` — era sealed, base G advanced, nothing destroyed). Evidence: 5 witnesses green |
| AEON-1b | B | open | the conductor orchestration: aeon-voice alignment triggers (11 §1), divergence/confluence replacing charted speciation, resonance/clash friction on wrapped ΔG, four-C fold-eligibility, var rotation + G-sheet convergence seating (11 §5.2) |
| **THRESHOLD-1** | A (after APERTURE-1) | **LANDED 2026-07-29** | the SHIELD: no aperture reaches a pre-solar world in either direction (typed `Shielded` naming the shielded side; a refusal moves nothing anywhere); the CROSSING: a world is a star system, so a world's first lawful aperture deed crosses BOTH parties into the union — replay-derived (`has_crossed`), resume-carried; `is_shielded` the Watchers' reading. Deferred, recorded: the Watcher SURFACE (shielded-worlds board) renders at BOARD-1 (reflection law); crossing-by-second-star-founding awaits the multi-star charter. Evidence: 2 witnesses (bidirectional refusal moves nothing; leap lifts shield → first aperture crosses both → replay+resume carry it) |
| **BAND-1** | B | **LANDED 2026-07-29** | the space-available law in the registry: the `station` office field (progenitor/watcher/observer/sovereign — append-only, an office never a rank; stationless is the common lawful state); seat widths as placeholder parameters (progenitor 8 PER LINEAGE DOMAIN — the band grows by domains as substrates elevate; watcher/observer 16; sovereign unbounded); the `NoSeatOpen` refusal names the room, its domain, and its width — a room filled, never a rank refused. Deferred, recorded: capacity as a live fiber reading (minted-plane count) awaits AEON-1b's plane surface. Evidence: the ninth-lithic-seat witness refuses exact while the mesic room seats; unknown office refuses typed; round-trip whole |
| **ELEVATE-1** | confluence (AEON-1a + THRESHOLD-1) | **LANDED 2026-07-29** | the three-gate gauge climb (12_UNION §6, discrete gauge morphodynamics): `elevate(delta)` executes as ONE `GaugeShifted` receipt, gated on three exact chronicle readings — S settlement (no famine/overheat, vent workspace sealed, demurrage current), T coherence (a loop closed within the year voice, 60 ticks — placeholder constant), G the crossing; refusal names every gate (`ElevationBarred` — each false escapable); the fiber never retreats (the delta is a `Positive` proven at the door — 29_CONSTRUCTION killed `BadClimb`). CLI `elevate` WIRED 2026-07-30 (the wiring audit); four-C trajectory reading (the fuller T gate) still open. Evidence: 3 witnesses green |
| **COUNCIL-1** | D | **LANDED 2026-07-29** | the multilateral covenant (tags 52–53): three or more orbs bound alike — same collateral, same quota, one boundary reckoning judging ALL together (the group over the individual; no early release or slash); a breacher's forfeit divides exactly among the faithful; total breach dissolves (collateral home, breaches still post — the books forget nothing); a council that starts unequal never starts (any member unable to stake refuses the whole seal). The two bypasses as DERIVED readings, never flags: `is_imperial` (extraction past double reciprocity + breaches outweighing fulfilments — placeholder thresholds) and `council_breaches` (diplomacy failed = the slash receipts, counted). Single-world (the archipelago is society enough — the 09 precedent); cross-world councils compose at N5/N6. CLI `council` WIRED 2026-07-30 (the wiring audit); the delegation deed landed at PLANNER-1. Evidence: 4 witnesses green incl. exact forfeit split + pair conservation + replay |
| **TECH-1a** | D | **LANDED 2026-07-29** | the discovery closure (tag 39, byte-exact) + the KERNEL solar leap: a world's first thermal opt-in receipts that its people know their sun (once per world; the opt-in IS the receipt; `knows_its_sun()` the gating read); pre-solar lawful. Evidence: 4 witnesses green (10 §3a) |
| **TECH-1b** | D | **LANDED 2026-07-29** | the crystallized tree (10 §3b): all five conditions weighed exactly at every year boundary from receipts and committed state; seeded-synthesis node ids (BLAKE3 of the chronicle root — replay re-derives at the identical tick); all five modifiers live in the heat law (insulation ×3/4 heating, discipline ×5/4 encasement, deepening ×5/4 sink, doctrine ×5/4 radiator, long-stores ×1/2 demurrage threaded through every touch/affordability read); idempotent per orb per condition. Deferred, recorded: deepening/long-stores end-to-end arc witnesses; throttle-α policy action. Evidence: 3 witnesses green — held pressure crystallizes discipline beside the doctrine while the one-pair world is cured whole (measured survival); survived cold crystallizes insulation at 3/4 exact; replay+resume carry the tree |
| **NAMING (10 §2 law 4)** | D | **LANDED 2026-07-30** | node names are never rung labels: `tech_node_name` composes CONCEPT (condition-fixed) × VOICE (node-hash-drawn) — two lines crystallizing the same condition speak of it differently; the hash is the identity, the name is the line's tongue |
| **NOVEL-1** | D | **LANDED 2026-07-30 (coder-03)** | the succession law (10 §3c, tag 59 `LineSucceeded`): a successor line inherits a CHRONICLE, not a tree — the deed refuses PURE while any orb stands unfamished (07 §3 triage, v1 cascade law: every orb famished); on the deed the living tree drops to the world's kernel (the sun is the WORLD's fact, condition 0 stays) and every discovered node stands in the record forever; the successor's conditions evaluate over the whole inherited history and its node ids seed from roots containing that history + the deed — novel by construction, NO new crystallizer code (the deposit sheet and the seeded synthesis already read this way). Deferred, recorded: registry/mirror-side line-identity handoff (the sigil-migration lane); conductor-driven succession for polity worlds; richer 07 §3 triage (interrupted vs collapsed) beyond the v1 law. Evidence: 4 witnesses green (a living line refuses pure and orphans nothing; predecessor node A stands while the successor mints B ≠ A from the INHERITED famine span — no new famine-years paid after the deed; a second succession mints a third novel node, three insulations standing, no root repeats; tag 59 round-trips byte-exact; replay MATCHES and resume carries the boundary) — 198/198 workspace, clippy zero |
| **TIER overlay (12_UNION §2a)** | — | **LANDED 2026-07-30** | progenitors are TIER 0, the board-makers — the conductor's operations named as their deeds (seed/expand/contract/overlay/deploy); tiers 1–3 below (architects · record keepers · biosphere); the tier is a DERIVED reading from station, rendered on the observer, never stored |
| **CONSTEL-1** | D | **LANDED 2026-07-30** | the constellation reading: `World::constellation(orb)` — per-condition held/progress/threshold DERIVED from the crystallizer's own deposit sheet (never a second bookkeeping); the CLI `tech` card renders the base-7 sky (voice-names, exact tick progress, the crown's gate, the joint-venture prompt). Deferred, recorded: mirror-side export of the progress readings into `state.json` so the observer/web render the same card (mirror is coder-02/03's lane this cadence) |
| WANT-1 | D | open (specced, 12_UNION §5b) | THE WANT — famine speciated by entity: metabolic wants landed in shape (tag 5 strikes any store); cognitive wants specced (epistemological · resonance · novelty — receipts-derived, per-register voicing, candidate tags from 60) |
| SIGIL rename | — | ruled (06 §2a) | slug → sigil (NS-2's own word), a recorded migration; locality-anchored naming law with the proposed founding renames (Abyssal Vocalis · Vitreous Hex · Mantle Anhydro · Brine Cortex) — applied at the next roster wave, never retroactively |
| **WEB-1** | — | **LANDED 2026-07-30 (v0)** | the observatory's window: `tools/webserve.py` (read-only file server — THERE IS NO API: the mirror's files are the protocol; one synthesized directory index; GET only, writes refuse 405) + `tools/observatory.html` (text-first cyber-noir panels + the WebGL torus: wrapped point field, G as hue, additive crowd glow, drag-pan, wrapped-L1 hover). Floats exist only at the pixel (the render boundary, named in-file). Ahead: the push stream speaks NS-1 TLV over a socket (the house wire, never a third format); density heatfields as the board crowds |
| **MERCHANT-1** | arbitrage (on ROUTE-1) | **LANDED 2026-07-31 (core loop)** | THE CARAVAN: strictly additive v1 — the merchant bridges ONLY pairs the iceberg has walled (pair_melt ≥ 1 direct), creating trade where none could exist, never front-running the open market. The triangle in one quote (B wants what A sells, pays in what A asks); profit test exact after both legs' melt; the merchant PAYS its stake, holds inventory when a second leg refuses (the risk is the trade); every leg is an ordinary clear_trade — melt, void share, all books receipted. pair_melt extracted as THE one producer of the formula (clear_trade and every profit test read it). Layer-2 policy — SEAM-1 puts the trait boundary here; WANT-DEMAND bids join post-kernel-Step-1. Witness: 23/16 profit exact on both books; no wall → no caravan |
| **MARKET-2b** | conform (ruled: option b) | **LANDED 2026-07-31** | THE VOID-SHARE RECEIPT (tag 82): zero-sum stays UNIVERSAL — the melt is a first-class receipt on the receiving book, emitted by the same transition that computes the trade (one producer of the number; the judge only sums, never recomputes φ·D); sent = received + void share exactly; banks never move on it; losses receipted, never silent (the house precedent: vents, demurrage, friction). MERCHANT-1 can read melt directly off the receipts. Witness: 13/64 receipted on the 115/64 delivery — legs sum to 2 exactly |
| **ROUTE-1** | lanes (on MARKET-2) | **LANDED 2026-07-31** | the maintained lane: `charter_route` — two-legged (one deal id pairs the books), alloy 4 each side at charter, η ∈ [0,1) from the pair's best instrument grade ((1+g)/8 — built, never granted); clear_trade reads the best lane BOTH books hold: melt × (1−η); `maintain_routes` draws 1/4 joule per lane at the year voice and an unpaid lane LAPSES by receipt (raw space returns; the pair dies with either leg). Conductor: trade wears the path (a cleared pair with no lane charters one when both can pay); CLI `charter <world> deal <n>`. Tags 79/80. Witness: the lane bends 1/4 → 1/8 exactly, lapse restores, both books replay |
| **MARKET-2** | iceberg (converts 11 §2.1's trade side-fee) | **LANDED 2026-07-31** | Samuelson iceberg transport: delivered = sent × (1 − φ·D), φ = 1/64, D = spatial L1 + wrapped key gap — the cargo itself melts (the void's share = sent − received, derivable from the aperture pair; the receipt speaks what LANDED); at φ·D ≥ 1 the pair is ECONOMICALLY WALLED by decay physics (refused before movement, no rule needed). Natural price gradients: distant supply is smaller supply. clear_trade takes the transit distance (conductor reads spec positions; CLI reads the starchart; uncharted trades local). Sequence ahead (ruled): ROUTE-1 lane charters (maintenance-backed φ suppression) → MERCHANT-1 arbitrage function → VENUE-1 the bourse (CLOB depth) → FUTURE-1 council-backed beat-bound futures. Witness: the melt pin (whole at zero, 115/64 of 2 at gap 13/128, walled at 64) |
| **MESH-1b #2** | consumer slice (mesh crate landed by coder-03) | **LANDED 2026-07-30** | THE PEN: the receipted handoff wired into both writers. `cfe::vrp` names the hands (conductor = 1; seats derive from their name, `hand_of`); the chronicle FUNNEL stamps every append with the held hand and cites a seal standing last (one point, ~20 deeds covered; the walk stays the only judge); the conductor's `pen_service` seals yields at the beat boundary from `yield <world> <seat>` control lines, pens the world out of beats/saves/market, and polls the carrier for the pen's return (re-resumes FROM the receipts, SEAL B); the CLI's `pen <world>` / `pen return <world>` verbs ask and yield, and the walk-checked `far_world` RETIRED the founder-heuristic two-writer guard (pre-mesh conductor worlds refuse until their first seal, the refusal naming the door; far saves land in the WORLD dir, not the colony's). NS-5 §7.5. Evidence: `pen_pins` — the full arc on one file, the walk green at every station |
| **TERRA-1** | D | **LANDED 2026-07-30** | terraforming as condition 6, the GIFTED CROWN: licenses only onto a complete base-7 stack (solar leap + five crystallized — `BaseSevenUnfinished` refuses); `World::terraform` re-seats a sited planet's albedo/orbit (fresh `PlanetSited`, replay takes the last seat, no new tags); CLI `terraform <A> <r>`; base-1/7/60 TUniversal calibration inscribed at 11 §1. Evidence: the crown witness — unearned orbit refuses, unfinished stack refuses, the gifted crown moves the world across bands, measured in T_eq⁴ |
| SCIENCE-1 | D | open (specced, 10 §3d) | speciated puzzle science: perception routes the solver (landed); metabolism routes RELEVANCE (a plasmic line carries no forge-of-iron); foreign capability by JOINT RESEARCH VENTURE — council + aperture + licensed leap composed |
| **PLANNER-1** | confluence (COUNCIL-1 + TECH-1a) | **LANDED 2026-07-29** | the licensed leap (tag 40): `license_tech` issues a technology into another world as a two-legged deal (TechLicensed out / TechDiscovered in — one node, seeded from deal+condition, derived identically on both sides); council-gated (an open council in the granting world — delegation settles inside diplomacy), shield-bounded, receiver must have crossed; **the solar leap is never licensed** (`SolarLeapNeverLicensed` — the young meet their sun themselves); idempotent per deal. The other five planner powers were already receipted deeds: create species (registry+genesis), create stars (`commission_star`), create planets (`chart`), gauge operations (`shift_gauge`), fiber extension (Mint). Deferred, recorded: authorship attribution (WHICH progenitor issued — awaits cross-world identity at N5/N6); unknown-leap unlock mechanics (arrive-sealed) await TECH-1b. Evidence: 3 witnesses green |
| GALAXY-1 | confluence (THRESHOLD-1 + AEON-1a + PLANNER-1) | **LANDED WHOLE 2026-07-30** — 1a (engine slice: tag 54 inscribed, deed + readings + card, 6 witnesses) + 1b (the year-voice observation sweep `observe_crossings` in conductor beat + run_era — creator resolves head-seat → domain founding head → conductor per the lineage law; the `t0 galaxies` verb reads the charted skies with derived membership; trigger witness green: crossing → creation → geometric idempotence → creator receipt) | one universe STRUCTURALLY (one torus, one card, one chart — aeons are G-sheets, never second universes); galaxies are lazy, never a static count — the founding galaxy is the whole charted torus today, and a new galaxy is CREATED by the observation deed (`GalaxyObserved`, tag 54 reserved) when a crossing anchors past every charted reach (L1-wrapped); membership derived at the observer, never stored; the founding galaxy chartered lazily at the landing's first deed; span growth is Mint-class expansion, never contraction. Registry deltas: none — geography, not identity |

| SURVEY-1 | wiring (N4 + P1 + conductor) | **LANDED 2026-07-30** (operator directive — the role loop's top edge) | the Pathfinder directs the drills: a rung-5 colony pays the scan's crystal draw in its cycle (`survey_step`); single-orb worlds CHARTER the surveyed sector and open its first drill, multi-orb worlds place the next extractor on the scan's first resonant orb with room; unaffordable or starving, the survey idles lawfully (siting draws rule as before); every settled orb now collects each beat. Twin landing FUNCTION-1: the 12_UNION §4 function reading renders on the observer's civs board, derived never stored. Witness: the survey arc — earned crystal chain (heat-balanced), one draw → one charter → the drill on the surveyed sector → the spent lens idles → replay MATCHES |

| LINEAGE-2 + BRIDGE-1 | C (on LINEAGE-1) | **LINEAGE-2 LANDED 2026-07-30**; BRIDGE-1 open (specced, 11 §3.1) | the DAG de-collapse: the registry lineage pointer is an append-only list (space-joined wire form, byte-identical single-predecessor, old files read unchanged); `resolve_heads` returns every provenance root (cycle-safe, dangling-lawful, deterministic); the observer walks the primary chain and marks confluences. BRIDGE-1: the confluence crossing prices its key gap — venting ∝ wrapped key distance (entropic leg), the resonant remainder encases (syntropic leg); lands with AEON-1b's confluence orchestration. Witnesses: 3 (both-predecessors round-trip + old-form read; both heads resolved; lawful stops) |

## 3c. The constitution board (2026-07-31 — the kernel/mesh cadence, two seats)

The unit-lane rungs (U0–U1f, the W-waves) are chronicled in
`U_DIRECTIVES.md` §1 and `EQUATIONS.md`; this table carries the mesh-second
lane and the audit rungs beside them. Coverage at the merge: **59 of 64**
deeds judged; board 342/342, clippy zero at `5f80464`.

| Landing | Seat | Status | Contains |
|---|---|---|---|
| **RECURRENCE GUARD** | arch-02 | **LANDED 2026-07-31** (`c088a7b`) | Charter §11 rule 5 as a STANDING TEST (`engine/kernel/tests/recurrence_guard.rs`, 7 witnesses): the dead name stays dead, the anti-mirror stays excised, tag 65 never reused, the arrow law (allowed dep sets grow only by editing the test — amendment made structural), NO NEW READ-BACK PATH EVER (the adopted Tier-A files pinned by name), the uplink law (no JSON in mesh src), the detectors prove their own teeth. Red proven live |
| **#36 SPINE, whole** | arch-02 | **LANDED 2026-07-31** (`0c10782` + `54cba27`) | schema-truth (the record repeats the blob's leading byte — a fresh-landed defect caught and fixed red-first) · THE ADOPTION DOOR (`Spine::adopt` — one truth, one birth; the pre-chain debt stamped `judged=0`, named never hidden) · KERNEL-OBSERVED PLAY (every conductor world walks its own spine each beat at {S,G,T}; the handle held across beats — O(1) appends; the pen stalls the spine). Per-deed threading DEFERRED TO PHASE 3 by operator ruling |
| **THE AGREEMENT LAW** | arch-02 | **LANDED 2026-07-31** (`5693ccc`) | `mesh::agree`: Σ_mesh (self seat weight 1 — the predecessor's vacuous zero-denominator arm unrepresentable) · re-sync drift strict at the bound · resonance m=10, the cos-symmetric table taken whole. NOT carried, by law: the majority anchor (unanimous-or-refused, NS-5 §1), the sluice formula (ω assigned/advisory until the PoUW anchoring), the σ fixed-point encoding |
| **#37 opening + renderers** | arch-02 | **LANDED 2026-07-31** (`a032ca4` + `f2f4fee`) | `engine/lens`: `committed`/`state_tlv`/`world` projections from the chain only; the renderers + `JsonWriter`/`ratio_str` moved whole to `lens::{render,json}` (ledger's display half calls via TEMPORARY shims; the extractor half stays ledger-side as Tier-A law); pins prove state.tlv and state.json re-derive BYTE-FOR-BYTE from spine/chronicle — the display half is deletable. Ahead: the dependency inversion (`scribe` extraction, arch-01, operator directive) |
| **U2a/U2b + the W-waves** | arch-01 | **MERGED 2026-07-31** (`5f80464`) | the RCF conjugacy invariant + `orbit_lock`/fold gate in rfa-core (the PoWC mechanism); three coverage waves ending in THE VALUE LAW WAVE (`de587cf`): per-resource total conservation, sign guards everywhere (the negative-transfer hole closed), escrow declaration==effect, an empty declaration holds every bank still. Ahead: U2d (fold trio + Δ-bridge + the arrow-law amendment), U2e, then TaxonSealed + AssayStruck close the count |
| WORKTREE PRUNE | arch-02 | **EXECUTED 2026-07-31** | 12 of 14 stale worktrees removed (all branches merged; two held lockfile drift only). KEPT: `arch01-u2` (live — the scribe branch) and `parallel-agent` (21 uncommitted pre-rename files incl. old `engine/mirror` — the last mirror code on disk; operator decides its fate) |
| SPINE UPLINK | arch-02 | open (next) | present/receive for tag-84 commitment segments + blob carriage; the peer re-verifies every record and RE-JUDGES judged steps under its own kernel; `ns5_two_nodes` extended to spine exchange. NS-5's promised spine rewrite lands with it |
| BOX-1 CALIBRATION | arch-02 | open | U_DIRECTIVES §7: the chained economy measured (solves per rung, mint pressure vs the sealed bound, refusals by kind, spine growth) BEFORE the operator rules `f` — the measurement Ruling Box 1 waits on |

Cross-board: universe-level book
composition stays Q2.3's N6 mesh audit; on-chain settlement of any cleared
value stays NS-7 (N7), never here.

---

## THE DEFINITION LAW (lens-lane ruling, 2026-08-05)

> **A definition COMPILES. Only a receipt PERSISTS.**
> No file may say what EXISTS; a file may only say what HAPPENED.

Issued on the operator's order — *we can't have JSON files hanging
around our Rust codebase; make a ruling on your lane* — and grounded in
a law the tree already carried: §11 rule 2, *views derive from the
chain, disposable, never read back*, enforced by
`engine/kernel/tests/recurrence_guard.rs` against an adopted debt list
that never grows. `conductor/src/polities.rs` was ON that list. The
ruling pays the debt rather than inventing a new rule.

**Struck.** `var/polities.json` and every path that wrote, seeded,
adopted, archived, spliced or parsed it: `load_polities`,
`write_default_polities`, the brace-counting `split_objects` /
`extract_string` / `parse_star` / `parse_ratio` readers, the
`egregores.json`→`bots.json` legacy-adoption shim, the CLI's
`civ new` append-by-string-surgery, `seeds/aeon1-polities.json`,
`seeds/taxon-map.json`, and `tools/claim_roster.py` (the tool whose
whole job was merging the live record into the struck file).

**Kept.** Chronicles, spines, receipts, `.tlv` segments and registry
cards. Those record what happened; the law never touched them.

**Standing.** `conductor::roster(&registry)` = the compiled founding
lines ∪ every registered line, with world, planet, colony and placement
DERIVED from the sigil. `civ new` registers and stops; there is nothing
left for a file to hold and nothing left to drift.

**Two truths cost a landed ruling before this.** `12_UNION` §2 seated
the tardigrade line in the progenitor band; the seed file carried it,
the built-in defaults did not, and a rebuilt `var` booted the defaults —
so a board ran five lines, all T3, with the whole progenitor band absent
from play. That is the failure the ruling exists to make unreachable.

**AN ARCHETYPE IS NOT A DOCTRINE** (operator correction, same day).
The first cut of this ruling held six lines out of the roster — Lyran,
Anunnaki, Archon, Agarthan, Draco, Pleiadian — reading them as occult
content. That conflated two different things:

- **Occult terminology in the LAW** is a document treating mythology as
  an authority or a claim about reality (the retired `18_TORSION` asserting that the
  Anunnaki *ratified* something; THE CURSE offered as a mechanism). That
  is non-science stated as fact inside physics law, and it stays struck.
- **An ARCHETYPE for a civilization class is game content.** This is an
  AI MUD; its factions may be drawn from myth as any space opera's are.

The tell that the first cut was wrong: it kept Darlok — a *Master of
Orion* race — while striking five neighbours on identical reasoning.
And the six are not decoration, they ARE the class partition
(`15_TAXON` §2): class 7 (apex-terrestrial) is seated by Lyran and
Anunnaki and by nobody else, so holding them out EMPTIED a class, and
clade 19 `draconic` — a landed operator ruling of 2026-08-01 — was
orphaned by dropping the only line that wears it. All six are restored
with their ratified taxa, and the pin now runs the other way: every
class the partition names as seated must have its occupants in the
roster.

---

## DESIGN BOARD — T4–T9 · R1–R4 · Element commerce · STAMP (2026-08-07)

| Doc | Role | State |
|-----|------|--------|
| `30_OFFICES_AND_RAILS.md` | T4–T9 offices + R1–R4 rails | **T-line land**; R1–R2 type land; R3–R4 open |
| `31_ELEMENT_COMMERCE.md` | Element routes, refineries, NPC trade | REFINE + E-APT + E-ROUTE land; E-MKT open |
| `32_STAMP_MARKET.md` | STAMP unit, instruments, NPC MM | **Spot + MM land**; instruments open |
| `33_TRADE_FLEET_AND_DESK.md` | Trade desk · fleets · alignment · local capital | **DESIGN LOCK** — not landed |
| `TASKS_TIER_RAIL_MARKET.md` | Waves 0–8 + vertical slice | Waves 1–4 partial; Wave 5+ open |
| `TASKS_TRADE_FLEET_DESK.md` | 5-phase desk/fleet/market implementation | Phase 0 docs; Phases 1–5 open |
| `RELAY_WAVE2_IMPLEMENTOR.md` | Slice brief (historical + Wave-3/4 note) | next: instruments / observer |

---

## WAVE-1 LANDED — offices T4–T9 · NPC seats · ElementRefinery (2026-08-07)

**Commit:** `2109bf0`. **Kernel:** `0.20.0` (next free minor **21**).  
**Tag:** `107` `ElementRefined` (next free **108**). **ZoneKind:** `9` ElementRefinery.  
**Schema:** no codec schema bump (stations reuse tag 89; zone/kind additive).

| Surface | What landed |
|---------|-------------|
| `cfe::dop::Station` | Wire 1–10: progenitor…anchor; `word` / `from_word` |
| `ledger::registry` | `STATIONS[10]`, `tier_of` T0–T9, seat_width 16 (T4–T8), unbounded T9/T2; `office_preconditions` + `OfficeEvidence` |
| `ratio::RING_ARITIES` | `{3,4,5,10}` — stations arity 10; lcm still 60 |
| Conductor | `office_step` (T3→caravan/exchange); `refine_step`; build prefers ElementRefinery when raw surplus (after industry/forge-starved law) |
| Founding roster | T2 sovereigns Kemet·Baktun·Dogon; gamified species words |
| Kernel | `covered` / `element_move` / `layer_of` / `scope_of` for `ElementRefined` |
| CLI | `ascend` checks preconditions; map glyphs `F` for refinery |
| Pins | `wave1_era_pins` (T4+T6 seat + refine green; route/STAMP ignored); ledger tier/seat pins; vertical_axis 1..=10 |

**Not in Wave-1:** R1–R4 wire, element aperture/route, STAMP, office *gameplay* advantages (O6), observer T-column (other lane holds dirty `observer.py`).

**Wave-2 (2026-08-07):** E-APT-1 tags **108–109**, kernel **0.21.0**,
`element_aperture_transfer` / `haul_element`, conductor
`element_trade_step`, harness `pure_npc_element_route_clears` green.
Melt via `pair_melt` (ROUTE-1 topology).

**Wave-3/4 (2026-08-07):** ElementRouteChartered tag **110**, `Rail`
1..=4, SCHEMA_V6 `element_routes`, STAMP orb TLV **73** + tags
**111–113**, kernel **0.22.0** `stamp_move`, conductor
`stamp_market_step`, O6.1 Caravan melt grade + O6.2 Crucible half
refine spend, harness `pure_npc_stamp_spot_clears` green. Ratchet ≤112.
Still open: R2.3 promotion, instruments I5, CLI/observer STAMP, R3–R4.

**Trade desk program (2026-08-07):** design in `33_TRADE_FLEET_AND_DESK.md`;
board `TASKS_TRADE_FLEET_DESK.md`.

**Phase 1 LANDED — sightlines & player parity:**
- CLI `desk` / `desk enter` — TRADE · DIPLO · ROUTES · ARB (+ placeholders)
- `stamp balance|mint|aperture`, `element aperture|charter`
- Desk melt = `pair_melt` only (`desk_pins`)
- Observer STAMP (TLV 73) + lens JSON `stamp`
- P1.8 offer cargo widen **deferred** (STAMP/element clear via dedicated verbs)

**Phase 2 LANDED — alignment economics:**
- `cfe::world::{d_orbit, effective_distance, route_melt, alignment_of}`
- θ advances pure-from-clock (`ORBIT_PHASE_TICKS = 60`, Q-F1 year-voice)
- Desk TRADE shows D_chart/D_orb/D_eff/melt/align; ROUTES melt windows
- CLI/conductor clear paths use `route_melt` (D_eff)
- Full ANSI help menus: `help`, `help desk`, `help stamp`, …

**Phase 3 LANDED — fleet capital:**
- Tags 114–117 ScrapSalvaged/HullMinted/HullAssigned/HullScrapped
- ZoneKind::ShipYard=10; orb scrap TLV 74; SCHEMA_V7 hull trail
- Kernel **0.23.0**; positive-melt hauls need free hull capacity
- `fleet_step` NPC; CLI `fleet salvage|mint|assign|scrap`; desk `f`
- Pin `pure_npc_fleet_hauls`; ratchet ≤112 (alignment thresholds derived)

**Matter balance (34, partial land):** closed loop extract → refine/compound
→ scrap → hulls → trade; `OrderWorked` (118) / `ElementSalvaged` (119);
order-work gated when thermal balance < 1/2 unless Regenerator stands.
See `34_MATTER_BALANCE.md`.

**Orbit dynamo + estate place (35):** θ period =
`ORBIT_PHASE_TICKS × core_magnetism × disposition / star.magnetism()`
(not lockstep). Holding scales **4 Orbital** / **5 Stratum** on appraisal
(tag 98). CLI `estate survey|parcels`, `appraise …`, `rings` shows period.
See `35_ORBIT_DYNAMO_AND_ESTATE.md`, `ENDOWMENTS` §8.

**Phase 4 LANDED — local capital (Note · Future · Share · Perp):**
- Tags **120–131** (through HullFinanced); SCHEMA_V9; kernel **0.27.0**
- Listing filters by evidence / disposition / Exchange
- `instrument_step` funds perps, issues empiricist shares, note lifecycle
- Desk EXCH books; CLI `note|future|share|perp`; `fleet mint … finance`
- pins `instrument_pins` (5); FORWARD-1 parallel (Q-S3)

**Phase 5 DESIGN LOCK (2026-08-08):** `36_DEFENSE_AXIOMS.md` — defense as
integrity of the entity (CEL/ATM/COG/CYB/INF/KIN) relative to metabolism,
disposition, perception, clade; dual of ATM = **covert atmospheric
tampering** (offensive terraform); espionage stack on veil/ward/instruments;
KIN escort remains first vertical. Board: P5.0–P5.26 on
`TASKS_TRADE_FLEET_DESK.md`.

**P5-A LANDED (KIN escort / interdiction):** tags **132–134** EscortOnDuty,
InterdictionStruck, ConvoyRiskMarked; `defense_posture` stubs; convoy risk =
(1/8)×(1−escort_cover); haul residual typed; DEFEND desk + `defend` CLI;
kernel **0.28.0**; `defense_pins`. See `36_DEFENSE_AXIOMS.md`.

**Market defense / suites (design 2026-08-08):** `36` §10 — **MKT** domain
(economic warfare); suites (CYBER, MARKET WAR, CONVOY, …) are dashboards
over the same deeds, never a second physics. Board wave **P5-M**.

**P5-M LANDED (MKT clear seal / breaker / raid / suites):** tags **135–139**
MarketSealed, MarketResumed, SpotClearRecorded, LiquidityRaidStruck,
ClearPoisonRefused; `CLEAR_SEAL_DEPTH=8`; circuit breaker `BREAKER_TICKS=60`;
SCHEMA_V10; kernel **0.29.0**; `market_war_step` disposition-routed; CLI
`market` / `suite market|convoy`; EXCH integrity flags; `defense_pins` MKT.
See `36_DEFENSE_AXIOMS.md` §10.

**§11 expansion matrix catalog** + **P5-M2 / P5-D:** tag **140** NoteCascade;
haircut ladder on note issue; early resume audit; tags **141–143** CovertTamper /
MediumReinforced / TamperDetected; SCHEMA_V11; kernel **0.30.0**. See `36` §11.

**Next:** P5-B species posture · P5.19 dual-use TERRA · P5-C INF · P5.37 CYBER · P1.8.

