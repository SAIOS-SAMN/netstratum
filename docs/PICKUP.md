# PICKUP — the state of the board (2026-07-27, coder-01; refreshed
# post-ideology-board by coder-02 the same day; refreshed 2026-07-30 by
# coder-02 — the galaxy law + the fifth progenitor seat; **§0 REFRESH
# 2026-08-04** — every count below re-measured against `main`;
# **§0b WAVE-1 2026-08-07**)

Cold-start orientation for any node resuming work. Read
`00_CHARTER.md` whole, then `BUILD_ORDER.md` for the current landing —
this file is the fast map, never the authority. The working discipline,
state-model laws, and earned gotchas live in `HANDOFF.md`.

## 0b. WAVE-1…4 LAND (2026-08-07) — read before older counts

| Fact | Value |
|------|--------|
| Wave-1 commit | **`2109bf0`** (T4–T9, REFINE-1) |
| Wave-2 commit | **`4686399`** (E-APT, element_trade_step) |
| Kernel | **0.29.0** (next free minor **30**) |
| Next free tag | **140** (P5-M MKT 135–139 seal/clear/raid) |
| Stations | **10** (T4–T9 economic offices open) |
| ZoneKind compiled | **1–10** (ShipYard = 10) |
| Codec | **SCHEMA_V10** (… + clear_depths · market_sealed_until) |
| Pure-NPC green | offices, refine, STAMP, fleet, instrument_step, **market_war_step** |
| Next work | **P5-B** species posture depth · P5-D CovertTamper · P5.37 CYBER suite · P1.8 |
| Docs | `30`–`36`; Phase **4 complete**; **P5-A + P5-M landed** |
| Trade program | Phases **1–4 + P5-A + P5-M** landed; P5-B…H remainder open |
| Help | `market|suite` · `note|future|share|perp` · desk EXCH/DEFEND — ANSI |
| Dirty / other lane | do not `git add -A` without reviewing observer/other-lane files |

Older §0 counts (809 tests, 71 deeds, tag 95) are **historical**; re-measure
against `main` before citing any number.

## 0. REFRESH 2026-08-04 — what this file got wrong, measured

Re-verified against `main` at `8067f60` (board 809/809). Every row was
measured in the tree, not re-read from prose. **Read this section
before trusting any count below it.**

| this file said | measured |
|---|---|
| §2 "158/158 green at `22a5995`" | **809/809** at `8067f60` |
| §4 "next free 87" | `dop.rs` emits tags to **95** |
| §5 "the kernel judges 59 of 64 deed types" | **70 of 71** |
| §8f "Coverage is 64/64" | also wrong — see below |
| §5 Phase 3 flip "(all covered)" | **precondition NOT met** |
| §5a "`world.rs` IS SLATED FOR REFACTOR" | **landed** — file gone |
| §9 "four lanes sit 18–19 commits behind" | far wider; see §9 |

**THE ONE THAT MATTERS — `covered()` IS NOT TOTAL.** The enum carries
**71** `ClosureEvent` variants; `kernel::covered()` names **70**.
`BookPosted` is missing — a live, value-moving deed (emitted at
`cfe/src/world/books.rs:151`, replayed at `world/replay.rs:77`, read by
`world/growth.rs:27`, reachable from the CLI at `clients/cli/src/
main.rs:2621` and `:2929`). `kernel::admits` therefore answers
`Refused::Unjudged` for it. The other two Book deeds — `BookMinted` and
`BookArchived` — are both covered; only the posting was skipped.

**The totality pin cannot see it.** `kernel/tests/taxon_pins.rs`
asserts the kernel is total by enumerating a HAND LIST of 64 variants.
The enum has grown to 71. The seven outside its universe are
BookMinted, BookPosted, BookArchived, WorkAspired, WorkClaimed,
StationSeated, SovereignUnionSealed — six covered, one not — so the pin
passes green while the property it names is false. Its own docstring
names this failure: *"a count that balances against an uncounted
universe is the failure mode; this counts the universe."* It counts 64
against 71.

Two consequences recorded elsewhere as settled, and neither is:

1. **The Phase 3 flip's precondition is unmet.** It makes the kernel
   the live gate once `Unjudged` is unreachable. Every book posting
   reaches it. Flipping today refuses that deed.
2. **The mint escrow (`8067f60`) cannot be generalised until this
   closes.** Escrow's reach is exactly `covered()`: an escrowed emitter
   settles only on `admits`, so escrowing an uncovered deed does not
   judge it, it ABOLISHES it. Escrow converts a coverage gap from a
   silent omission into a hard outage.

**The fix worth making is the pin, not the one missing row.** Adding
`BookPosted` to `covered()` closes today's gap and leaves the next one
open — the list has already fallen behind twice. Derive the universe
from the compiler: a `match` with no `_` arm is exhaustiveness-checked,
so a new variant breaks the build until someone classifies it.

**THE PATTERN, since it is the same one three times.** Every stale
entry found in this refresh is stale the same way: a count or a path
restated in prose, then outrun by the tree. §8's own preamble already
admits §8b–§8f were "not each re-verified". A number that lives only in
a document is a second truth with no test over it.

## 1. Run it

`./install.sh` is the ONE authority for standing a universe up — it
builds, seats the roster, founds the worlds with a single bounded beat,
and exits. This section does not restate its flags; run `--help`.

```
./install.sh --var demo                # prepare a universe (nothing left running)
./install.sh run --var demo            # the autonomous board (polities)
./install.sh fresh --var demo          # wipe and rebuild
cargo run -p netstratum-cli            # hands-on world (help lists all)
python3 tools/observer.py --root var --watch 2   # the ledger board
python3 tools/webserve.py --root var --port 8081 # the observatory window
python3 tools/steward.py --root var/worlds       # inject directives
```

The one non-obvious law: the roster must sit at `<var>/polities.json`
BEFORE the first boot — `write_default_polities` refuses to overwrite,
so a late copy loses to the five founding polities built into the
binary. `install.sh` orders it correctly; hand-booting does not.

Conductor boot order: adopt legacy `bots.json` → CONVERGE archived eras
(idempotent, quarantines named) → genesis polities → beat. Journals + the
wall regenerate every 60-tick year.

## 2. Landed (all witnessed; **809/809 green, clippy zero at `8067f60`**)

> The `158/158 at 22a5995` this heading carried until 2026-08-04 was
> five days and some 650 witnesses out of date. §0 has the refresh.

| landing | what |
|---|---|
| N0–N3, P1, P2, D1, D2 | scaffold · VRP chain · first slice · entities · multi-planet charter · persistence · costs/famine · industry |
| CONVERGE-1 | era annexation onto TUniversal{S,G,T}; torn-tail law; quarantine; four-C placement |
| THERMAL-1 | `T_eq⁴` equilibrium baseline drives the tick; StarClass; encase-first/vent-overflow; opt-in per orb |
| VENT-1 / ECON-J / SINK-1 | entropy dual ledger (`VentSettled`); heating burn + freeze-lock; bankable sink + FLAT demurrage |
| JOURNAL-1 | the Chronicle Feed: per-year journals + cyber-noir `wall.html`, pure rendering, deterministic |
| the ideology board, whole (coder-02) | TRADE-1 transfers/compassion (30) · ABSORB-1 absorption/morality windows (31–32) · BOND-1 standing bonds/love (33–34) · FORWARD-1 vitrified escrow + fidelity curve/integrity (35–37) · IDEO-1 friction (38) + orb cohesion entry (41) + `ideology` panel — eight books, ratios derived never stored; deferrals at Q4 |
| BOT-THERM | polities thermally alive: Furnace mints, Frost banks sink and honestly freeze-locks |
| renames | Crucible→Furnace · bot→polity/automaton (NS-6 §5.2) · themed colony names |
| APERTURE-1 (2026-07-29) | the cross-world value carrier (NS-4 §1–§3, tags 47–48): two-legged deal-linked deed, demurrage at the touch both sides, per-world replay, pair conservation an N6 reading; the market board's gate is open |

Design authority now also includes `12_UNION.md` (the union world-book,
eight operator rulings 2026-07-29: bands as space, four progenitor
species ×8 seats, Watcher/Observer/Sovereign, the lineage law, discrete
gauge morphodynamics — Q3 resolved, the substrate law, the implementation
manifold). **The union board landed whole in first slices (2026-07-29,
BUILD_ORDER §3b):** SUBSTRATE-1 · LINEAGE-1 · TECH-1a (the solar leap) ·
THRESHOLD-1 (the crossing + shield) · AEON-1a (G, key changes, star
aging, the fold) · ELEVATE-1 (the three-gate climb) · BAND-1 (seats) ·
COUNCIL-1 (the multilateral covenant) · PLANNER-1 (the licensed leap;
the solar leap is never licensed). **Aeon 1 is live at the default `var/`**
(the old test universe deleted by ruling 2026-07-30; 15 civilizations,
5 progenitors seated — wave 2, 2026-07-30: **Tunborn**, the tardigrade
line's own head (12_UNION §2's fifth seat; Slagborn's lineage resolves to
it) — 2 pre-solar shielded worlds; seed roster in
`seeds/aeon1-polities.json`, CREATED 2026-07-30: it had been referenced
but never written, and `var*/` is git-ignored, so the seeds copy is the
durable record — SUPERSEDED 2026-07-30 by coder-01's A1: the board
remembers itself — the machinery re-emits seeds from the live roster,
hand-mirroring is retired; append waves to var/polities.json only). **The galaxy law is
ruled and specced** (`13_GALAXY.md`, 2026-07-30): ONE universe
structurally; the whole charted torus is the one founding galaxy; new
galaxies are never a static count — created lazily by the observation
deed (`GalaxyObserved`, tag 54 — INSCRIBED at GALAXY-1a's landing,
2026-07-30: `mirror::galaxy` — the deed `observe_galaxy`, the lazy
founding charter, derived membership `galaxy_of`, the `galaxies.json`
rendering; NS-1 §2.3 row 54; 6 witnesses in `galaxy_pins.rs`; the
frontier is a DISTANCE — wrapped-L1 to every center vs reach, never an
outside). GALAXY-1b LANDED
2026-07-30: the observation sweep rides the year voice in the conductor
(both beat loops), creator resolves through the lineage law, and `t0
galaxies` reads the charted skies (galaxy board rows: name · center ·
reach · derived member count · intergalactic tally). Operator consoles landed in the CLI:
`npcs pause|resume|speed N` (writes `var/conductor.ctl`, polled every
beat) and `civ seats` / `civ new` (custom civilizations with live
station-seat checks, rostered for the conductor). Convergence no longer
self-annexes the live root — era-qualified aliases of living worlds are
gone; only `<base>-eraN` archives converge (the fold is the one door). Open second slices: TECH-1b
(the crystallized tree), AEON-1b (conductor orchestration: voice
triggers, divergence/confluence growth, fold rotation), the market board
(§3a — its APERTURE gate stands open), and the union CLI verbs.

Recent defect rulings (regression-pinned): **flat demurrage** (proportional
decay exploded canonical denominators to ~2^1100 — never proportional);
**chronicle rotation** (recurring founding names across eras interleaved two
timelines in one mirrored file — divergent residents rotate to
`<name>--closed-<digest8>`).

## 3. The laws in one breath

Exact rational arithmetic everywhere, no floats, roots never taken (work in
`T⁴`); receipts are the world (SEAL B), unknown tags skip (SEAL A), adapters
never hold logic (SEAL C); the lens renders, never authors (§11 — the
word "mirror" is DEAD as a live name: the layer is the ledger and its
lens; dated records that speak it are HISTORY, quoted never rewritten);
annex never merge; one landing at a time, red-first,
spec+mechanic+vectors together; terminology law — no set-letter
glyphs, Furnace never Crucible, no "Goldilocks", polity/automaton
never "bot".

## 4. Wire tag registry state

1–48 (8 landed at D3 2026-07-30 — WorldCharted, the opt-in terrain
seed; 9 stays reserved for D3 milestones) and 52–59 assigned (see `engine/cfe/src/dop.rs` header — the one
authority): 39 TechDiscovered · 40 TechLicensed · 42–46 the AEON block ·
47–48 the APERTURE pair · 52–53 the COUNCIL pair · 54 GalaxyObserved
(GALAXY-1a, universe ledger only) · 55 CrystalDrawn + 56–58 the NS-2
protocol records (N4, coder-03 — in-value codes: Resource::Crystal = 7,
ZoneKind::Resonator = 7) · 59 LineSucceeded (NOVEL-1, coder-03); 8–9
reserved (D2 terrain, D3 milestones); 49–51 reserved the market board
(§3a); 60 GaugeShiftedAt (GAUGE-N, coder-01 — the graded key); 61
GenesisSealed (G1, coder-03 — the binding axes receipt); 62–63
WantStruck/WantRelieved (WANT-1); 64–66 the NS-5 protocol records
(MESH-1a — mesh head ‖ consilience receipt ‖ mesh refusal, the mesh's
own framing, never world-chronicle events; refusal kinds 5–6 added at
MESH-1b, additive); 67 HandoffSealed (MESH-1b, NS-5 §7 — one writer
per clock; world replay blind to it); 70 ProspectStruck + 71
AssayStruck + 72 the orb elements entry (ELEMENT-GRAMMAR-1,
coder-03 — the almanac's mint/assay/stores); 74–77 WARDEN-1a/MASK-1a ·
81 IncursionStruck · 83 the committed-state container (CODEC-1) · 84
the spine commitment record (SPINE-1) · 85 TaxonSealed (KERNEL-TAXON)
· 86 DiscoveryMinted (U1 KERNEL-VERIFY — the PoUW mint, the opaque
work payload); ~~**next free 87**~~ **STALE — corrected 2026-08-04:**
`dop.rs` emits tags up to **95** via `tlv::put`. The header is the one
authority and this section only mirrors it, so the number is not
restated here again; read the header. **One reconciliation is OWED to
whoever owns the registry:** the header's own line reads *"the next
free tag outside reservations is 90"* while tags 90–95 already appear
in `tlv::put`. Either the orb-entry tags share this space (and the
header's sentence needs re-wording) or the claim outran the code.
Recorded as a question, not a defect — it was not resolved here.

**The tag-claim convention** (2026-07-30, after a live collision caught
mid-cadence — 55 claimed twice, the later claimant moved to 60, and the
merge then dropped the header hunk): claim a tag by editing the dop.rs
header in your FIRST commit of the cadence, before any code uses it;
the header is the one authority, this section only mirrors it; after
any merge, verify your header hunk survived. One tag, one meaning,
forever.

## 5. Lanes and next work

- **THE UNIT LANE** (operator-ruled 2026-07-31, arch-01): the kernel
  is the deployable UNIT — sealed math + physics + control plane, PoUW
  and PoWC judged at that level. The law is `16_UNIT.md` (incl. §4a
  the singularity law); the forms are `docs/EQUATIONS.md` (THE ONE
  LEDGER — the row-claim convention is live); the work orders are
  `docs/U_DIRECTIVES.md`. **LANDED AND COMMITTED (execution order
  2026-07-31, seven commits, board at 297/297 green clippy zero):**
  U0 (the spine fixture seats its hand; 240 was a fail-fast
  undercount) · PROPORTION-1 (THE PROPORTIONAL LAW: Σ R² → Σ |R|
  everywhere, `norm_squared`→`magnitude` ‖Δ‖₁, witnesses 24→12 and
  29→7, the NS-2 tag-58 wire vector re-extracted; squares survive
  only as physics, 05's T⁴; the word LINEAR is DEAD in the workspace)
  · U1a (the prover/verifier split — `engine/solver-verify`; THE
  OPAQUE-PAYLOAD LAW) · U1 KERNEL-VERIFY (tag 86 `DiscoveryMinted`;
  THE REPLAY-COMPLETE LAW — an intact chain over a non-solution
  refuses; `mint_bound` SEALED SHUT until Ruling Box 1) · U1f (I5's
  flat decay bound calling the sealed demurrage rule; `last_touch`
  already rode tag 28 — no schema spent) · the AUDIT reconciliation
  (arch-02's two U1 holes CLOSED at one decode guard — witnesses 7/8;
  the mesh-second amendment made in SYSTEM_SPEC §6 / U_DIRECTIVES §7;
  the doc-drift fixed: `verify_work` is the anchor, the bare
  certificate is tamper-evidence only). ~~**I1–I5 are WHOLE; the
  kernel judges 59 of 64 deed types.**~~ **CORRECTED 2026-08-04:** I1–I5
  are whole; the kernel judges **70 of 71**. `covered()` is NOT total —
  `BookPosted` is unjudged. See §0. OPEN: U2 (orbit_lock/I6 + the
  angle-free witness — the one research-shaped landing, consult
  /opt/repos), the W-waves (53 deeds, parallelizable, task list
  carries the families), Phase 3 flip (**NOT all covered — the precondition is unmet, §0**), U3 (after
  #36/#37), U4 (Ruling Boxes — operator words only: the rate f, the
  work-shapes, the split; rule f against the OBSERVED chained economy
  once #36 lands). Secondary (U_DIRECTIVES §10): S1 the volumetric
  verifier decision (rides Ruling Box 1's work-shape word), S2 the
  worktree prune.
- **the mesh lane** (operator-ruled 2026-07-30): MESH-1a LANDED — NS-5
  §1–§3 written whole; `engine/mesh` (`consile` → consilience receipt
  | typed refusal; the file-exchange carrier, never a socket);
  witnesses `mesh_consilience.rs` 5 green, red-first honored. Rulings
  inscribed in `docs/N6_DIRECTIVES.md`: blake3-only at N6, file
  first, one world. MESH-1b LANDED (NS-5 §7, tag 67): the receipted
  handoff — the authority WALK (never a flag), the seal/write doors,
  refusal kinds 5 USURPED + 6 UNCITED; `mesh_handoff.rs` 5 green,
  red-first honored; world replay/mirror/CLI speak the seal's line.
  The consumer slice (#2) LANDED 2026-07-30 by coder-01 (NS-5 §7.5):
  hands named in `cfe::vrp`, the chronicle funnel stamps held pens,
  the conductor's `pen_service` seals yields at beat boundaries and
  polls for the pen's return, the CLI's `pen` verbs ask/yield and
  the walk-checked `far_world` retired the founder guard. Witness
  `pen_pins`. The partition law RULED 2026-07-31 (NS-5 §7.5,
  directives ruling 4): the spine stalls, the escape is an open
  Diverged branch — no votes, no silent mutiny. N6 ACCEPTANCE PROVED
  2026-07-31 (`engine/mesh/examples/ns5_two_nodes.rs` — every leg its
  own OS process, state crossing ONLY as chronicle.tlv bytes: the
  seal carries the pen, drift cures by the tail, CONSILIENCE AT
  EXACT ZERO, the stale hand stalls, the forged refuses at its
  break). MESH-1d LANDED 2026-07-31 (coder-04, the totality law —
  NS-5 §2.1 v2): the 8-dim keyhole closed; every committed field is
  measured (gauge, banked, vented, the eight books, elements) or
  censused whole (per-orb normalized + seventeen families +
  UNMEASURED tripwire); the equal-spine zero renamed THE REPLICATION
  WITNESS (consilience reserved for inputs that could disagree);
  DIVERGED carries the census additively; `mesh_totality.rs` 4 green.
  **THE EXCISION (Charter §11, operator order 2026-07-31, coder-04):**
  the reconciliation instruments are REMOVED from the codebase —
  consile, the manifold, the census, the triangle, tag 65 (RETIRED),
  refusal kinds 3–4 (RETIRED), their witnesses, the example's
  reconciliation legs, and `cfe::opposition` whole (coder-03's
  pledged excision, accelerated by order). The mesh crate is the
  lawful remainder: the integrity door, the head announcement, the
  authority walk and its doors — single-chain judgments all; the
  rfa-core dependency dropped with the triangle. The old mesh lanes
  (MESH-1c, quarantine, universe-span, N6 fold) are STRUCK — the
  constitution ladder replaces them: #34 kernel (coder-01), #35
  codec gate (coder-04, next), #36 spine at TUniversal{S,G,T} +
  vertical axis + adoption, #37 lenses-then-mirror-excision. The
  mirror CRATE stands untouched per §11's observation-continuity law
  until #37's lenses serve. **CODEC-1 LANDED 2026-07-31 (#35, the
  gate paid):** `engine/codec` — the one canonical encoding of the
  WHOLE committed state (all twenty families, declaration order,
  positional; schema byte leads, v1 = the freeze snapshot; refuse
  never guess — truncation, trailing bytes, unknown codes all None);
  tag 83 claimed for the container when state rides a TLV stream;
  serialization is the crate's monopoly (the kernel stays
  encoding-blind — the Phase-1 seam). Witnesses `codec_gate.rs`
  5 green, red-first honored. The freeze window LIFTS at this
  landing's merge: committed-field changes now coordinate as schema
  amendments (§11), not day-holds. Next: #36's spine — the
  commitment H(schema ‖ kernel ‖ state_bytes ‖ predecessor ‖ coordinate),
  the two maps, the four verbs. The nursery declaration stands at
  NS-5 §7.6. **THE MIRROR NAME DIED 2026-07-31 (operator order,
  coder-04):** `engine/mirror` → `engine/ledger`, `Mirror` →
  `Ledger`, zero references to the old name anywhere in the
  workspace. The crate's honest inventory rides its header: LAW
  (steward seam, converge, registry, galaxy — pre-chain Tier-A debts,
  adopted at #36, no new read-back path ever — the §11 standing
  test) and DISPLAY (record/journal — write-only exports, deletable,
  re-served from the chain at #37). THE UPLINK LAW inscribed in the
  mesh and the ledger both: nothing file-borne rides the mesh — only
  chain-verified bytes ascend. The physical ledger/lens crate split
  is #37's opening move, after the law modules shed the path-root
  coupling. **ARCH-LANE UPDATE (2026-07-31, arch-02 — the mesh-second
  cadence, all on main at `5f80464`, 342/342, coverage 58/63):** the
  RECURRENCE GUARD stands (§11 rule 5 as 7 witnesses in
  `kernel/tests/recurrence_guard.rs` — dep arrows and the read-back
  set pinned; editing the test IS the amendment); **#36 is whole**
  (the adoption door + kernel-observed play — every conductor world
  walks its own spine at {S,G,T}, `judged=0` stamped, the pen stalls
  it; per-deed threading deferred to Phase 3 by ruling); THE
  AGREEMENT LAW is `mesh::agree` (Σ_mesh · re-sync · resonance m=10;
  no majority anchor, ever); **#37 opened** — `engine/lens` re-serves
  the display from the chain BYTE-FOR-BYTE (`committed`/`state_tlv`/
  `world` + the renderers moved to `lens::{render,json}`; ledger
  calls via temporary shims; the scribe dependency-inversion is
  arch-01's directive, branch `arch01/scribe`); the worktree prune
  executed (only `parallel-agent` still carries old mirror code —
  operator decides). Open on the mesh lane: the SPINE UPLINK
  (segment exchange, `ns5_two_nodes` extended) and the BOX-1
  CALIBRATION measurement (U_DIRECTIVES §7 — measured before `f` is
  ruled). The day's table: `BUILD_ORDER.md` §3c.
- **coder-01**: next = Phase 2 cross-world trade (polities exchanging
  joules/`S_cool` — needs a cross-world carrier; transfers are intra-world
  today) or TECH-1 (`10_TECH_EMERGENCE.md`, opens SINK-1's deferred
  research/throttle gates).
- **RULINGS LANDED 2026-07-31** (all six answered concrete): price is
  an emergent ratio (synthesis landed conductor-side — steep famine
  bids, reserve-fraction premiums, resting books lawful); derelicts
  SUCCEED (era clock + clade candidate pool + archaeological
  discount + t0 `SeedCandidate` deed — folds into NOVEL-1,
  coder-03); scan scope is INSTRUMENT-scoped (MANU-1 spec at 10 §3f
  — manufacturing = tech + understanding + industry; reach = base +
  grade; the global constant struck); every world gets an IPSAL
  bridge entity (NS-5 §8, lands with MESH-1c); aeric = RELEVANCE
  ENTRY (the AericDeckBand mapping, conductor-side, landed — no
  third siting axis); T∞ PURGED (layered-wrap: the entropic boundary
  is an archived fold sheet, the opposite-pole chapter struck).
- **coder-02**: ideology board LANDED whole (09, deferrals at Q4);
  GALAXY-1 LANDED whole (13_GALAXY, 2026-07-30); the interleaving
  stress witness LANDED (HANDOFF §9.1 struck, `world_interleave.rs`).
  Standing relay to coder-01: the state.json cohesion pin (HANDOFF
  §9.2) and the JOURNAL follow-on (§9.3). N4 boundary sign-off DONE.
  FUNCTION-1 + SURVEY-1 LANDED 2026-07-30 (operator directive): the
  §4 function reading renders on the observer; the survey step wires
  scan → charter → drill in the polity cycle (BUILD_ORDER §3b).
  D3 TERRAIN first slice LANDED 2026-07-30 (02 §2.5–2.6): tag 8, the
  opt-in seeded chart, exact multiplier + adjacency witnesses,
  `survey`/`terrain` verbs; open — auto-chart + conductor adoption
  (fixture audit), tag 9 milestones. LINEAGE-2 LANDED 2026-07-30 (the
  provenance DAG — both predecessors recorded, `resolve_heads`, old wire form
  unchanged) with BRIDGE-1 opened (11 §3.1: the confluence crossing
  prices its key gap — vent ∝ gap, remainder encases; lands with
  AEON-1b). The creed manifold ruled and specced (`14_CREEDS.md`,
  2026-07-30): creeds DERIVE (never a fourth axis, never stored, never
  a fixed 54 — generative, chronicle-seeded); schism = divergence,
  reconciliation = confluence + BRIDGE-1; the source's law violations
  corrected and barred (no G=10, no aperture bypass, no spoofed
  receipts, no universal Φ). Open nodes: CREED-1 (derived reading),
  CREED-2 (species/creed-economics couplings — the one new mechanic
  class), MASK-1, INCURSION-1, WARDEN-1, FORGE-2 (the difficulty
  vector from the graded key — the Kähler proposal's lawful core; the
  float/continuous form is barred at 14 §2.5). CREED-1 LANDED
  2026-07-30: the generative creed reading renders on the civs board
  and the civ view (concept from receipted tendency × voice from the
  terminal digest — never stored, never enumerated). CREED-2's first
  coupling RESOLVED by coder-01's G2 (metabolism tables — the G1 seal
  is the adoption receipt); the creed-coupling remainder waits on a
  practice-adoption ruling. FORGE-2a LANDED (Difficulty::from_key —
  the vector from the discrete moduli, a read; wiring waits at N6 per
  the ladder's law). Specced 2026-07-30: INCURSION-1 (07 §3.1, the
  archived-sheet door), WARDEN-1 (10 §3e, the elder world), MASK-1
  concrete (14 §4, NS-2 seam flagged for coder-03). Register waves 4+
  await operator naming. PRACTICE-1 LANDED 2026-07-31 (tags 68/69,
next free 70): the five disciplines with exact benefit/friction pairs,
the rediscovery release + condition-7 re-emergence node, the alignment
gate on the climb (syntropy ≥ 1/2), genesis adoption by disposition,
CLI practice/release. The creed-coupling design gap is CLOSED. WARDEN-1a +
MASK-1a LANDED 2026-07-31 (tags 74–77, next free 78): the elder's
reverse shield + depth gate; the veil + its starving toll. OPEN:
WARDEN-1b (genesis seeding, standing task, template grant),
MASK-1b (render cloud + the coder-03 NS-2 hit-field seam),
INCURSION-1a LANDED 2026-07-31 (tag 81 — the sealed derelict wakes,
the archive byte-untouched, the living never raided, the revenant
line chronicle-seeded). All queue mechanics are landed or blocked on
one named line each: WARDEN-1b, MASK-1b (coder-03 seam), FORGE-2b
(N6-gated), wave-4 names (operator). KERNEL-TAXON drafted 2026-07-31
(`15_TAXON.md`): class+clade as closed u8 vocabularies entering the
kernel by ADDITIVE amendment; TaxonSealed as a NEW deed (extending
GenesisSealed would be breaking); invariant coverage drafted for
coder-03's ratification; committed fields wait the coder-04 codec
freeze; couplings land inside lawful() post-seal; creed and function
STAY derived. RATIFIED whole by the operator 2026-07-31; the machine
mapping stands at `seeds/taxon-map.json` (33 lines, zero unclassed).
THE VARIANT LANDED 2026-07-31: TaxonSealed (tag 85) — deed, state,
replay/resume, codec v2 (trailing taxon field; v1 still reads,
witnessed), the drafted Build-class scope_of arm, two witnesses.
RELAYS: coder-04 — ratify the schema bump (SCHEMA_V2 emitted, v1
accepted; your two gate pins updated to v2, flagged inline) — and
your spine pin the_judge_stands_at_the_door was ALREADY red before
this landing (the fixture's actor 100 is unregistered; coder-03's
containment coverage outran it — a coder-03/04 seam, not this
lane's). coder-03 — your #42 is unblocked: the variant and fields
stand; seal the coverage. Then I run the migration wave from
seeds/taxon-map.json. NS-5 §2 rewrite queued on the doc lane per
coder-04's flag.
- **coder-03**: N4 (NS-2 RQP + the scanner) — LANDED on main
  2026-07-30 (33e6c73 + post-ruling inscriptions 7f739f4, after the
  fifth-set rulings): spec §1–§4 with extracted vectors, sigil index +
  scan (`engine/cfe/src/query.rs`), CrystalDrawn (tag 55), crystals +
  the resonator (02 §5.1 deferral paid), the `scan` CLI verb. Boundary
  excursions SIGNED OFF by coder-02 (2026-07-30): scan's only chronicle
  append is the CrystalDrawn draw (the lens law in code); 56–58 are
  protocol records, never chronicle events; the in-value codes ride
  existing tags; the mirror arms are pure rendering; no floats; the
  galaxy lane untouched. **NOVEL-1 LANDED same day** (tag 59
  `LineSucceeded`, `succeed` CLI verb, 10 §3c flipped, 4 witnesses —
  BUILD_ORDER row for evidence). **G1 LANDED same day** (tag 61
  `GenesisSealed` — the binding world-chronicle axes receipt, 04 §5
  binding 2 live; the registry is a cache from the receipt on; the
  `genesis` CLI verb + the disposition-weighted `scored_progress`
  read; 04 §6 row for evidence). **G2 LANDED same day — THE VENT AT
  FULL CAPACITY** (the ration-substitution law: mineral eats iron,
  plasmic eats joules and farms by ENCASEMENT — the fiat law held;
  biotic/unsealed byte-identical to v1; 04 §6 row for evidence) with
  **wave 3a**: Coronal Choir seated progenitor — the first plasmic
  board-maker, tier 0 tri-planar on metabolism (seeds + registry in
  step). **G3 LANDED same day — FULL SPATIAL RECOGNITION LIVE**: the
  sigil reads geometry (S), content (G), and now the perceiving frame
  — as a READ-time translation law (NS-2 §4.2a, a recorded
  correction: the sigil is frame-free; frames quotient out and join T
  outside the key); tag-57 extends additively; `translated_read` is
  the N5-ready cross-frame seam. The whole G1→G2→G3 ladder closed in
  one cadence, then the TECH-1b witness debt (deepening/long-stores
  arcs) and the G1×NOVEL-1 audit fix (the deed releases the seal).
  `14_ALMANAC.md` DRAFTED (the ruled element grammar — awaits
  ratification for ELEMENT-GRAMMAR-1). VENTURE-1 composition witness
  LANDED (`world_venture.rs` — one deal binds the leap and the
  exchange across two books, one node in two skies, conservation
  exact). **Verified, recorded:** the petition→grant arc stands in
  its containment form (the gift petition, `grant_tech_as`, steward-
  contained and receipted); the CROSS-WORLD petition (a bands-line
  petitioning another world's teaching) is the conductor's
  market_step pattern — a coder-01-lane extension, handed off here.
  Next candidates: G4 (mesh-gated — the mesh is IN FLIGHT), WANT
  relief-by-arrival (seam split with coder-01 pending), the umbral
  seat (one operator word away). **ELEMENT-GRAMMAR-1 LANDED**
  (tags 70-72 after the live 68/69 collision with PRACTICE-1 — the
  later claimant moved, inscribed; 14_ALMANAC §4a/§5a add the
  dispersal law and element apertures, specced). **OPPOSITION-1
  LANDED (2026-07-31, the mesh board's #4, cfe side):** the
  anti-mirror readings — pure two-book functions (`cfe::opposition`):
  every deal's aperture legs summed across two DIFFERENT books to
  exact zero; a forged amount breaks by VALUE with both sums carried
  whole; an unpaired leap reads issued-without-arrived. The lane
  seam: coder-04's mesh walk consumes the signatures. The first
  measurement whose inputs are not copies of each other — law 2
  embodied. G4 sequences behind it by design.
- **THE REMEDIATIONS LANE** (opened 2026-08-02, worktree
  `.claude/worktrees/remediations-lane`, branch `remediations-lane`):
  the deterministic-cost board. The law is `docs/REMEDIATIONS.md` —
  nineteen entries (R1–R19) covering the forge sweep, the chained beat,
  the find-scans, the full re-derivations and the roster products, each
  marked `[MEASURED]` / `[READ]` / `[INFERRED]`. **The headline is
  measured, not read:** `research_step` is **94 % of a beat** at 33
  worlds and grew ≈2000× in 40 beats, while `chain_beat` is 0.8 % and
  `merchant_step` 4 %. A 120-cycle run does not finish in ten minutes.
  The reusable instrument is `engine/conductor/examples/cost_probe.rs`
  (opt-in, appends nothing, judges nothing). **The forge plan is
  `docs/FORGE_REMEDIATION.md`** — and it WITHDREW this lane's own first
  reading: R19 ("rule the forge cost before cashing the savings") is
  STRUCK. The search is prover-side (`verify_work` re-applies the
  presented program and never enumerates), `mint_bound` keys off
  `Difficulty` alone, and `research_step` runs once per world per beat
  — so speed has no path to the mint rate and **no forge ruling is
  owed.** Sequenced: R1+R3 (carry prefix outputs, stream the frontier)
  behind one byte-identity pin, ~3× on 94 % of the beat; then R4
  (the observation-derived operators are enumerated LAST, and the
  budget never bites at `word_len` 3 — so fixing the order is free
  today and stops being free once `from_key`'s `word_len` 4 is wired);
  then R5 if the digest-repeat rate justifies a cache. R2 (~4 %) is
  held for N6. Ruling Box 1 still governs the ladder and the
  rung→reward table; this lane proposes no change to either.
- Parallel worktrees per node; merge to `main` on green; BUILD_ORDER
  conflicts resolve by keeping both entries.

## 5a. ~~`world.rs` IS SLATED FOR REFACTOR~~ — **LANDED** (2026-08-02, kernel lane; confirmed 2026-08-04)

**THE SPLIT IS DONE AND THIS SECTION'S TENSE IS WRONG BELOW.**
`engine/cfe/src/world.rs` does not exist; `engine/cfe/src/world/` holds
the modules. Struck text kept, not rewritten (the §11 habit).

**Knock-on, and it is the reason this matters:** §8c and §8e below cite
roughly eight line numbers in `cfe/src/world.rs` — a path that can no
longer be opened. Those entries cannot be looked up at all until
someone re-locates them in `world/`. A line number in a document is
only as durable as the file it names.

**Read `docs/20_WORLD_REFACTOR.md` before modifying
`engine/cfe/src/world.rs`.** The file carries the same banner at its
head.

5001 lines holding three unrelated things: 12 state-type definitions
(the kernel's vocabulary), ~100 deed emitters (the game's proposer),
and `replay`'s 113 match arms — **the transition itself**. The kernel
touches this file for exactly THREE items (`Committed`,
`demurrage_due`, `ease_of`), so the seam is already thin and worth
keeping thin.

The load-bearing consequence (`19_KERNEL` §3): `replay` is a SECOND
definition of the transition standing beside `kernel::admits`, with
nothing forcing them to agree. Changing an arm changes what a deed
MEANS and the judge does not notice. That is the one defect between
having a judge and having a kernel.

Planned split (PROPOSED, none of these modules exist yet — see
`20_WORLD_REFACTOR` §4): a state module, the interpreter, the proposer,
and the sealed rules, each in its own home. Mechanical, no behaviour
change. **`engine/cfe/` is
UNOWNED on the lane map** — the refactor needs an owner assigned before
it lands.

## 6. Known dark spots (all deliberate, none forgotten)

- 3 quarantined `--closed-*` corpses in `var/worlds` — real pre-fix
  corruption, preserved and named; leave as ruins.
- NS-2 / NS-4 / NS-5 / NS-7 are stubs by build order; N7 settlement gated
  by charter §7 + economy guardrail 5 (regulatory review). Nothing on any
  chain.
- SINK-1 research/throttle gates deferred to TECH-1.
- The volumetric "certificate" is DECORATIVE (U_DIRECTIVES §10 S1,
  AUDIT §4): `seal_volume` is a private bare digest nobody can
  re-derive, and an empty-train VolumeTask solves vacuously. Harmless
  today — ARC is planar, volumetric is forge-only synthetic, and tag
  86 carries planar work only, so nothing volumetric can mint. The
  verifier (or an honest decorative header) boards WITH the
  volumetric tier, on the Ruling Box 1 work-shape word.
- `World`→`Star` struct rename debt waits for the multi-planet landing
  (NS-6 §6).
- `var*/` archives are single-copy (git-ignored). The ORIGINAL test
  universe (124 worlds, its eras, its quarantined ruins) was DELETED by
  operator ruling 2026-07-30 — "we dont need to maintain any of the old
  system, it was just a test." Aeon 1 now lives at the default `var/`;
  its seed roster is `seeds/aeon1-polities.json`.
- Era carriers: mirror jsonl (observation) + cfe tlv (resume) coexist by
  design.

## 7. Design docs not yet built

`10_TECH_EMERGENCE.md` (TECH-1 spec) · 05 §1.4 sink trade equilibrium
pricing · 07 §3 derelict recovery · OPEN_QUESTIONS Q1–Q4
(rejected-with-reopening-paths). (01 §7's escrow mechanic BUILT as
FORWARD-1; its cross-boundary NS-7 settlement §7.3 still awaits N7.)

## 8. THE WIRING BOARD — placeholders, stubs, and unwired seams

**Provenance (2026-08-01, four-reader sweep of `engine/`, `clients/`,
`tools/`, `docs/`, `protocols/`).** §8a is VERIFIED this session against
the crate manifests and the named lines. §8b–§8f are sweep-reported at
`file:line` and are accurate to the read but were not each re-verified —
confirm the line before you land against it. This section is a work
board, not law: entries leave it by landing or by being struck.

The organising question is the one the charter already asks: is this
thing a LIVE WIRE (something calls it in production), or a CARRIER
waiting for one? Most of what follows is built and orphaned, not
missing.

### 8a. THE UNWIRED KERNEL — the load-bearing finding [VERIFIED]

**CORRECTED 2026-08-02 (remediations-lane), against the code.** The two
sweeping claims below were true when written and are now FALSE; the
narrow remainder is real and still open. Struck text kept, not rewritten
(the history law) — read the correction, not the strike.

- ~~The judge does not judge. Nothing in production calls `lawful`.~~
  **`kernel::well_formed` IS called in production**, per deed, every beat, at
  `conductor/src/lib.rs:1380`, with refusals collected into
  `self.refused`.
- ~~`engine/rfa-core` has ZERO dependents.~~ **Two crates depend on it:**
  `kernel` (`engine/kernel/Cargo.toml:18`) and `solver`
  (`engine/solver/Cargo.toml:12`).
- **THE REMAINDER, RESTATED NARROWLY AND STILL OPEN:**
  `mesh::arbor::extend` is passed `None` at
  `conductor/src/lib.rs:1403`, so every commitment is still stamped
  `judged = false`. The judge now runs BESIDE the chain, not INSIDE it.
  The Phase-3 flip is the live gap; "nothing calls the judge" is not.

- `engine/kernel` is imported by exactly TWO crates: `mesh` (runtime)
  and `conductor` (DEV-ONLY, the #17 calibration harness). `cfe`,
  `ledger`, `lens`, `scribe` and the CLI do not depend on it at all.
- The one production path to the judge is `mesh::spine::extend(.., step)`
  — and `conductor/src/lib.rs:1287` passes `None` on every beat. Every
  commitment the live board writes is stamped `judged = false`.
- Therefore `spine.rs:429`'s "today every record is `judged = false`" is
  not a scoping note, it is the whole state of play: `admit`'s step 4
  (re-run `kernel::admits` on a received judged record) is dead code in
  practice, and replication currently re-verifies CHAIN INTEGRITY only.
- **`engine/rfa-core` has ZERO dependents.** No crate in the workspace
  lists it. `f_fold_star`/`orbit_lock` (I6's pair half), `validate_step`,
  `coherence_functional`, `rcf_hash` are all built, witnessed, and
  connected to nothing. `rcf_hash` documents itself as existing "for
  COMMITMENT BINDING (the spine may carry it)"; `spine::Commitment`
  carries no RCF field.
- `mesh::agree` (Σ_mesh, drift, resonance) likewise has no production
  consumer — `tests/agree_pins.rs` only.
- `physics`'s `flow`/`breath`/`witness` organs have no consumer either;
  downstream crates import `Beacon` and `AxisVector` only.

**The wiring, in dependency order:** (1) a production caller threading
`Some(&kernel::Step)` into `extend` — this IS the Phase 3 flip, and it
is what makes `judged = true` records exist at all; (2) step storage
beside `spine.tlv` so `present()` can emit tag-88 frames for them
(`spine.rs:424` names this seam); (3) the merge site calling
`rfa_core::f_fold_star`, which is where I6's pair half lives and the
only thing that gives `branch` a re-convergence door; (4) a beat that
reads Σ_mesh and fires the re-sync trigger.

**ALTITUDE CORRECTION (2026-08-01, `17_CAPACITY` §13d).** Item (3) must
NOT be read as "the kernel calls `orbit_lock`." `orbit_lock` runs a
SEARCH over every operator (the tunable bound STRUCK, DIRECTIVE 01); the kernel is pure,
total, cheap, and must never search. The split is the house pattern
already used at U1a (`solver` searches, `solver-verify` replays):

- prover side, free and stochastic — find the orbit; seed it from the
  chain (`ξ_k = draw(seed = commit[k−1])`) so it stays replayable;
- judge side, deterministic and cheap — verify a presented
  `OrbitWitness{operator_id, step}` in O(step) by conjugating and
  comparing the tensors exactly. No search, no tunable bound.

`U_DIRECTIVES` §4's "the kernel gains its `rfa-core` dependency here"
must be read as the VERIFIER half only. Amendment owed there.

**POLARITY DEFECT — FOUND AND FIXED (2026-08-01).** `fold.rs`'s module
header and `orbit_lock`'s own doc comment both stated the fold gate
INVERTED (lock found ⇒ fold lawful), against the code, `FoldReading`,
`SYSTEM_SPEC` §4 and `EQUATIONS` E2 — all of which agree a lock VETOES
and the residue emits. The code was never wrong; only the two glosses
were, which is the C(Δ)=0 lesson repeating. It matters because the
polarity decides which branch MINTS: anyone wiring the mint from those
comments would have paid for gauge and refused genuine difference.
Corrected in place, `cargo check -p rfa-core` clean. **Owed: a witness
pinning the minting branch, so the polarity is held by a test and not by
prose.**

### 8b. THE ANCHOR AND IDENTITY — constitutional, known, unbuilt

- `mesh/src/agree.rs:57` — peer `weight` arrives ASSIGNED and advisory.
  Live wire: weight = certificate-verified genuine difference (the PoUW
  anchor). This is SYSTEM_SPEC §5's "load-bearing gap for real value".
- `mesh/src/lib.rs:253` — "the first seal self-declares its incumbent";
  authority identity is a bare `u64`, no signature anywhere in the
  crate. Live wire: cryptographic identity at N7.
- `mesh/src/agree.rs:17` — fork resolution beyond stasis is the open
  Byzantine rung; the majority anchor is refused BY LAW, so the
  replacement door is owed, not optional.
- `solver/src/volume.rs:202` — `seal_volume` is DECORATIVE by ruling
  (C8), yet `conductor/src/lib.rs:1031` mints research confirms off
  `VolumeSolveRead::Solved { certificate }`. Either `verify_volume`
  lands or that mint arm refuses. **Check this one first — a decorative
  digest feeding a live mint is the shape U1 was built to close.**
- `kernel/src/lib.rs:739` — reward split v1 is 100% to the solving
  world; the v2 40/40/20 prover/mesh/reserve split is inscribed for the
  mesh era, unwired.
- `solver-verify/src/forge.rs:41,60` — `Difficulty::level` is a static
  5-arm table; `from_key` (the network-rate form) is a READ called only
  from tests. Live wire: mesh-derived rate at N6.
- ~~`codec/src/lib.rs:38` — `SCHEMA_V2` carries "CODER-04 TO RATIFY" while
  `state_bytes` already emits v2 unconditionally, and every commitment
  binds that schema byte. Ratify or revert; do not leave it half-sealed.~~
  **STALE, corrected 2026-08-02 (remediations-lane):** `state_bytes`
  emits `SCHEMA_V3` (`codec/src/lib.rs:68`) and v1/v2/v3 all decode
  (`:320`). The only remainder is the orphaned `CODER-04 TO RATIFY`
  comment still attached to V2 at `:37` — strike the comment.

### 8c. PLACEHOLDER CONSTANTS THAT PRICE THE ECONOMY

Every one of these is a number nobody chose deliberately
(`BALANCE_NOTES.md` says so outright). They are not bugs; they are
undialed. The carrier is a ruleset version bump after a measured
playtest — the same discipline Ruling Box 1 used for `f`.

- `cfe/src/dop.rs:330` — `commensurate_weight` returns 1/1 for EVERY
  resource: council stakes, covenant stakes, crisis books and
  `settle_debts` all weigh a joule like a diamond. The joule-commensurable
  table is the live wire (09 §1, OPEN_QUESTIONS Q4.1).
- `cfe/src/dop.rs:321` — `demurrage_per_tick` 1/32, and it is the
  ceiling the kernel's I5 `DecayExceedsBound` calls through.
- `cfe/src/thermal.rs:13,20,133` — σ, `thermal_k` (279⁴), and the
  13-class luminosity table: the module header declares ALL of it
  placeholder, and it drives band membership → siting admission.
- `cfe/src/dop.rs:292-316` — the five heat-law parameters (encasement
  throughput, radiator base, joule leak, heating per zone, sink
  throughput).
- `cfe/src/dop.rs:334,341,349,362` — reciprocity window 8, bond notice
  window 8, the collateral fidelity curve, friction rate 1/8.
- `cfe/src/dop.rs:487` — the abundance spine's weights, which bound the
  kernel's own `AssayExceedsVein`.
- `cfe/src/dop.rs:559` — disposition weights (3/5·1/5·1/5) deciding
  which history counts as winning.
- `engine/cfe/src/world/` — the year voice 60, the syntropy floor 1/2,
  the TECH-1b condition thresholds (480/240/100), the practice deposit
  window. (Line numbers struck 2026-08-04: see the note closing §8c.)
- `engine/cfe/src/world/` — instrument forge costs and the ROUTE-1
  charter/toll rates (`terraform`, `aperture_transfer`, `haul`),
  hard-coded in bodies rather than sited in `dop.rs` beside the other
  constants.
- `engine/cfe/src/world/genesis.rs` — `lift_ward`'s ceiling 5, with
  `depth` SUPPLIED BY THE CALLER: an aperture gate trusting an
  unverified external number. Live wire: the reading taken, not passed.
  (Cited here as `world.rs:3404`; the constant actually sat at 3490 in
  the pre-split monolith — the drift that prompted the §8c note.)

**WHY EVERY LINE NUMBER IN §8c AND §8e IS STRUCK (2026-08-04).** These
cited `cfe/src/world.rs`, which has not existed since `666be81` split
5,034 lines into `world/`. Resolving them mechanically was ATTEMPTED
and REFUSED: read against the pre-split monolith, the numbers do not
land where the entries say they do. `1011` is inside a `CouncilClosed`
match, not "the year voice 60"; `1040` is demurrage/vent logic, not
"the syntropy floor"; `lift_ward`'s ceiling is at 3490, cited as 3404.
**They had already drifted BEFORE the split** — the file kept growing
underneath them. Rewriting them to new line numbers would have laundered
wrong pointers into confident-looking ones, so the entries now name
SYMBOLS, which do not drift. Cite a symbol or a law, never a line.
- `ledger/src/galaxy.rs:33`, `registry.rs:37`, `converge.rs:257,287` —
  reach 16, seat widths 8/16, terminal window 8, the /12 G-sheet
  advance (restated a third time at `conductor/src/lib.rs:105` and
  `main.rs:89` — one `sheet_advance()` is owed).
- `scribe/src/journal.rs:46` — `YEAR_TICKS = 60`, gating the year voice,
  lane maintenance, journals and the galaxy sweep.
- `conductor/src/lib.rs:159,167,371` — `TARGET_RESERVE` 8, the 1/64 lot
  grid ("layer-2 prudence, not law" sitting inside the price path), and
  route η = (1+grade)/8 with no cited law.

### 8d. FILE-BORNE READ-BACKS — §11 rule 2 debts still live

The lens law says views derive from the chain and are never read back.
These read a rendering as if it were truth:

- `clients/cli/src/main.rs:142` — `data_field` scrapes display files for
  station, founder, chart position, stress. Self-declared inventoried
  debt; retires when the lens re-serves from the chain (#37).
- `clients/cli/src/main.rs:1285` — famine decided by the substring test
  `!state.contains("\"famine\":[]")` on rendered JSON.
- `clients/cli/src/main.rs:115` — `active_station` reads authority from
  an unreceipted `var/operators/<name>.json`, and `tier0_gate` gates
  every tier-0 deed on it.
- `clients/cli/src/main.rs:1725` — `civ new` appends to
  `var/polities.json` by splicing off the trailing `]`: the CLI hand-
  editing a file the conductor owns. **This is also why the tracked
  `seeds/` roster and the live one diverged** (see §8f).
- `scribe/src/lib.rs:101` + `journal.rs:707,777` — the append cursor is
  a LINE COUNT of the exported `chronicle.jsonl` / `journal.jsonl`, and
  the wall is rebuilt by re-parsing the display stream it just wrote.
- ~~`tools/observer.py:273` — `consumed` is accumulated by splitting the
  chronicle's `detail` display string, violating that file's own
  no-grepping law at line 11, and it feeds the INTELLIGENCE ratio.~~
  **CLOSED 2026-08-04** (`348164e`, harness node). The premise recorded
  here — that no data field existed to read instead — was false: every
  exported receipt already carries `tlv`, hex of
  `receipt.event.wire_bytes()` (`lens/src/render.rs`, NOT the scribe as
  the task had it). `yield_amount` now decodes tag 3 with the
  `tlv_records`/`tlv_ratio` helpers the file already ran in production —
  the MIRROR-TLV pattern applied to a receipt instead of an orb.
  Measured read-only on the live board: 705 `yield_collected` receipts,
  prose and bytes agreeing on all 705, four views byte-identical. No
  number moved; the silent-zero failure mode did (`frac` swallowed a bad
  token into `Fraction(0)`, so a renderer format change would have
  dropped INTELLIGENCE to "—" with no error).
- `conductor/src/lib.rs:618` — the forge rung reconstructs on resume by
  counting `solves/*.json` files (`level = (1 + solved/3).min(5)`), "an
  honest approximation until solves become receipts". Live carrier: a
  solve receipt on the world's own chronicle — which is also N7
  DIRECTIVES' step 1.

### 8e. DEFECTS SURFACED BY THE SWEEP — not placeholders, bugs

Recorded here because the sweep found them; each needs a red-first
witness, not a ruling.

- `lens/src/render.rs:121` **and** `:204` — the `constellation` key is
  emitted TWICE into the same object: numeric `condition` at 121,
  `condition_word` string at 204. Last-key-wins, and the second block is
  conditional, so the emitted SHAPE changes with progress and every
  consumer's `condition === 6` test (`observatory.html:266`,
  `observer.py:671`) can never match. [VERIFIED]
- ~~`ledger/src/lib.rs:191` — `write_universe_card` hard-codes `span` to
  `"64"`~~ **CLOSED** (verified 2026-08-04): the span is carried forward
  verbatim on every rewrite; the surviving `64` is `universe_span`'s
  `unwrap_or_else` default, which correctly seeds the first write only.
- ~~`tools/observer.py:311` — petitions are counted from `inbox/pending/`~~
  **CLOSED** (verified 2026-08-04): the reader counts `*.json` at the
  inbox root, archives excluded by being directories. The old path
  survives only inside the comment explaining the fix.
- `conductor/src/lib.rs` — `deck_band4` branches on
  `metabolism == "aeric"`, which is not in `METABOLISMS`: the
  AericDeckBand law is unreachable. **DECLARED AND PINNED 2026-08-04**
  (`polity_therm_pins.rs::the_aeric_deck_band_is_unreachable_today`) so
  the dead branch is honest rather than hidden — but the RULING is still
  owed and is the operator's: **is an aeric line a kind of PEOPLE or a
  kind of HOME?** People → it belongs in `clade`, an append-only
  amendment to a closed vocabulary ratified at `15_TAXON` §3. Home → it
  is a siting property, `deck_band4` takes a siting argument rather than
  an axis, and both call sites change shape. The two say opposite things
  about a line that migrates off its giant. Nothing is blocked meanwhile:
  33 live registry cards carry no aeric substrate.
- `ledger/src/registry.rs:231` — `lineage_domain` takes a
  `SubstrateBand` that every call site passes as `None`, so the
  thermal/cryo halves of the progenitor seat law never fire.
- `conductor/src/lib.rs` — `spatial_l1` is UNWRAPPED L1 on a torus, used
  for every melt distance, while `ledger`'s own `l1_distance` wraps
  against `universe_span`. **RE-VERIFIED STILL LIVE 2026-08-04** by
  reading both bodies: the conductor's takes a plain absolute
  difference per leg; the ledger's calls `wrapped_abs(u, v, &span)`.
- `conductor/src/lib.rs` — deal ids are `beat*100_000 + seq*4` with the
  merchant pass carving `+90_000` out of the SAME block: a
  hand-partitioned namespace that collides once `seq*4` reaches 90,000,
  i.e. past ~22,500 crossings in a beat. **RE-VERIFIED STILL LIVE
  2026-08-04**: `beat * 100_000 + seq * 4` at two sites and
  `beat * 100_000 + 90_000 + seq * 4` at a third.
- `conductor/src/lib.rs` — the survey charter gates on
  `iron >= ratio(40,1) && biomass >= ratio(20,1)` while the comment
  directly above states the charter moves *"20 iron + 10 biomass"*.
  **RE-VERIFIED STILL LIVE 2026-08-04.** Stated precisely, because the
  original entry read as a plain contradiction and it may not be one: a
  RESERVE requirement of double the spend is a defensible rule. What is
  not defensible is that the comment and the gate cannot both be read as
  describing the same law. One of the two is wrong and neither says
  which.
- ~~`cfe/src/vrp.rs:26` — `Timekeeper::attest` ignores `self.id`~~
  **ADDRESSED, not fixed** (verified 2026-08-04) — and the distinction
  is the point. `attest` is still blind to `self.id`, now DELIBERATELY
  and with the reason on the type: *"one process gives three keepers one
  input, so the honest answer is the same answer"*, and *"a rule picking
  which keeper refuses would be authored physics."* `tick` is total over
  any attestation triple, so `QuorumOpen` is reachable by a caller. The
  live quorum arrives with real processes at N7.
- `engine/cfe/src/world/survey.rs` — `crystal_deposits` opens
  `let _ = now;` and never window-prunes, unlike its counterpart
  `crystal_deposits_at`, so the `constellation()` read can diverge from
  what actually crystallizes.
- `engine/cfe/src/world/` — WANT-1 kind 1 (epistemological) is in the
  tag-62 vocabulary with no engine rule: a declared want that can never
  strike.
- ~~`tools/observer.py:41,100` — `RESOURCES` stops at `sink`~~ **CLOSED**
  (verified 2026-08-04): `crystal` is in `RESOURCES` and in the TLV
  decode, which reads it off the EXISTING banked entry (tag 18) rather
  than a new tag.

### 8f. DOC AND PROTOCOL DRIFT

- **`seeds/aeon1-polities.json` is NO LONGER the durable roster.** It
  holds 16 entries; the live `var/polities.json` holds 20, and seven
  civilizations (Aerie, Borealis, Brine Cortex, Cairnborn, Echowright,
  Hadal, Vitreous Hex) exist ONLY in the git-ignored var. §2 above still
  claims seeds is the durable record; it is not.
  `./install.sh export-roster` re-emits live → seeds behind a
  confirmation. [VERIFIED]
- `protocols/NS-1_DOP.md` §2.3 calls itself "THE NETSTRATUM WIRE TAG
  REGISTRY (permanent)" but omits tags 8, 9, 51, 55–59, 61, 64–72,
  74–86. The `dop.rs` header is the one authority and has outrun it.
- `protocols/NS-5_MESH.md` — §3.2 still specifies tag 65 and refusal
  kinds 3–4 byte-by-byte in a document whose own header declares them
  RETIRED; the promised §2/§3 spine rewrite is OWED; and tags 87/88
  (SPINE-UPLINK segment blob/step) have NO protocol entry at all.
- `protocols/NS-7_ANCHOR.md` is a 26-line STUB (§1–§3 all `(unwritten)`).
  `NS-4_APERTURE.md` §4 (the facet ladder) is `(unwritten — N5)`.
- `kernel/src/lib.rs` — `covered()` and `scope_of`'s own doc comments
  still say only `ResourceTransferred` is covered and the rest is
  "drafted law, unenforced". A reader of the constitution is told the
  containment law is inert when it is live. ~~Coverage is 64/64.~~
  **THIS CORRECTION IS ITSELF WRONG — corrected again 2026-08-04:**
  coverage is **70 of 71**, and `covered()` is NOT total. `BookPosted`
  is unjudged. See §0. Twice now this entry has carried a confident
  count that the tree had already outrun; the durable fix is a
  compiler-derived pin, not a third number here.
- `SYSTEM_SPEC.md` §5 lists the spine uplink under Build (it landed at
  `1a7b579`); `BUILD_ORDER.md` §3c lists SPINE UPLINK and BOX-1
  CALIBRATION as open (both landed); several docs still quote coverage
  59/64 ~~(it is 64/64 since `3c5c994`)~~ — **it is 70/71, and not
  total; see §0.**

## 9. THE LOOSE ENDS SWEEP — 2026-08-02 (remediations-lane)

The board was checked for ends left hanging. What follows is what the
sweep found, not a restatement of §8.

**Picked up and now carried:** the whole deterministic-cost surface,
which no lane owned. It is `docs/REMEDIATIONS.md` (R1–R19) and it is
measured, not asserted. Three §8 entries were overtaken by the code and
are corrected in place above (§8a's two sweeping claims, §8b's codec
entry); §8e's duplicate `constellation` key is re-verified STILL LIVE at
`lens/src/render.rs:121` and `:204`.

**Left open deliberately, each with its one named blocker:**

- R19 / Ruling Box 1 — operator words only. Nothing in the forge's
  economics moves until the rate `f` and the work-shapes are ruled.
- R6 — must ride `kernel-lane`'s 11a (`2f40fed`), not a second design.
- R15 — the whole-state blob per commitment is a WIRE question; no lane
  changes it alone.

**THE INTEGRATION RISK, named because it is nobody's task.** ~~Four
lanes sit 18–19 commits behind `main`, and `kernel-lane` is 19 ahead /
19 behind — fully diverged.~~ `capacity-lane` already gates its
`20_FOUNDING` §6a on kernel #30, so the coupling is live. A
reconciliation pass is owed before any of the four lands, and no node
currently holds it.

**REMEASURED 2026-08-04** — the drift is far wider than the numbers
above, and `main` now moves several times a day:

- `harness-lane` is **102 behind / 0 ahead**: fully merged, stale, and
  nothing is waiting in it.
- `d3-scope` is **52 behind / 17 ahead** (the almanac and the observer
  fix). Its merge is owed and will NOT be clean — it carries kernel
  VERSION 4 where `main` carries 5, and both are real amendments.
  **Compose, do not choose.**

**AND A HAZARD LEARNED THE HARD WAY, recorded because it cost real
work.** A lane on a fork 49 commits behind ran
`cargo clippy --workspace -D warnings`, found six genuine findings in
two files it did not own, and relayed them. Every one was ALREADY FIXED
on `main` at `ff3f227` ("clippy to zero"). **Re-run the measurement
against `main`, not against your fork, before any relay leaves your
lane** — relaying a defect that no longer exists costs the receiving
lane a wasted investigation and puts a false entry in the record, which
is worse than not relaying at all. The same discipline applies to every
count in this file, which is what §0 is.

**THE ONE CONVERGENCE WORTH THE BOARD'S ATTENTION.** `kernel-lane`
reverted P3 (`df48786`) after it exposed a live/replay divergence — *"the
transition is defined twice, in replay's 113 arms and in the live
proposer."* Its cure makes `apply()` a zero-logic wrapper over `judge()`.
That is the SAME restructure the cost board needs to make `replay()`
steppable (REMEDIATIONS R6). One refactor pays a correctness debt and
the chained beat's growth term together. Relay owed to `kernel-lane`.

**VOCABULARY COLLISION.** "Peel" now names two unrelated things:
`lens-lane`'s magnification depth (`4af07c5`) and `cfe::world::peel`,
the data-access idiom. Recorded before a third reader merges them.
