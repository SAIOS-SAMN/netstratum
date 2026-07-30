# NS-1 — DOP: Dimensional Object Protocol

**Status:** LANDED at N2 (first cut — the colony orb). First consumer: the
first playable slice (`engine/cfe`, `clients/cli`). Supersedes
PROTOCOL_STACK.md §1. The spec grows additively as later landings add orb
kinds (sectors, fleets, profiles); tags are permanent.

A dimensional orb is a self-contained game object: its committed state, its
provenance (the NS-3 receipts that ground it), and enough context to stand
alone. An orb encodes to wire bytes canonically — equal state, equal bytes,
always — which is what makes byte-exact round trips and cross-node
comparison (NS-5) possible.

---

## 1. RFA semantics

### 1.1 The ratio carrier

Every quantity is an exact rational p/q in **canonical form**: coprime
(gcd(p,q)=1), positive denominator, reduced at construction. Canonical form
means ONE representation per value — 2/4 and 1/2 are the same bytes on the
wire (measured: `the_ratio_carrier_is_canonical`). Yield rates are ratios
(extractor: 3/2 iron per tick; habitat: 1/4 biomass per tick); accrual over
n ticks is exact multiplication, never drift (measured: 4 ticks × 3/2 = 6
exactly; 3 ticks × 3/2 = 9/2 exactly through the wire and back).

### 1.2 The committed/workspace boundary

An orb carries **committed state only** — what NS-3 receipts ground: the
header, the zones, the banked ledger. Accrued-but-uncollected yield is
WORKSPACE: it lives outside the orb, never encodes, and never persists.
**What has no receipt does not persist** (the v0.4 workspace invariant,
carried). Collection is the closure that moves value across the boundary,
one receipt per resource.

### 1.3 Canonical order

Zones encode tile-ascending; banked entries encode resource-ascending.
Order is part of canonical form — two nodes holding the same state emit
identical bytes.

## 2. Wire format

### 2.1 The TLV record

`tag u8 ‖ LE32(length) ‖ value` — additive per CHARTER SEAL A: a reader
skips unknown tags, never crashes (measured: `a_reader_skips_unknown_tags`).

### 2.2 The ratio encoding

`sign u8 (0 non-negative, 1 negative) ‖ LE32(len) ‖ numer magnitude BE ‖
LE32(len) ‖ denom magnitude BE` — magnitudes carry no leading zeros; zero's
magnitude is the empty string; a zero denominator refuses to decode.

### 2.3 THE NETSTRATUM WIRE TAG REGISTRY (permanent)

One tag, one meaning, forever; retired codes are never reused.

| tag | meaning | value layout |
|---|---|---|
| 1 | event: ColonyFounded | LE64(orb_id) ‖ name utf8 |
| 2 | event: ZoneBuilt | LE64(orb_id) ‖ LE16(tile) ‖ kind u8 (1 extractor, 2 habitat, 3 smelter [D2], 4 synthesizer [D2], 5 encasement [THERMAL-1], 6 cryo-array [SINK-1]) |
| 3 | event: YieldCollected | LE64(orb_id) ‖ resource u8 (1 iron, 2 biomass, 3 alloy [D2], 4 diamond [D2], 5 joule [THERMAL-1], 6 sink [SINK-1]) ‖ ratio |
| 4 | event: UpkeepSettled (D1) | LE64(orb_id) ‖ resource u8 ‖ ratio |
| 5 | event: FamineStruck (D1) | LE64(orb_id) ‖ resource u8 ‖ ratio (drawn) |
| 6 | event: FamineRelieved (D1) | LE64(orb_id) ‖ resource u8 |
| 7 | event: ZoneRazed (D1) | LE64(orb_id) ‖ LE16(tile) |
| 21 | event: EraAnnexed (CONVERGE-1 — universe ledger only) | LE64(offset) ‖ terminal[32] ‖ LE16(era len) ‖ era utf8 ‖ world utf8 |
| 22 | event: OverheatStruck (THERMAL-1) | LE64(orb_id) ‖ ratio (overflow) |
| 23 | event: OverheatCleared (THERMAL-1) | LE64(orb_id) |
| 24 | event: StarCommissioned (THERMAL-1) | class u8 (NS-6 §7 codes) |
| 25 | event: PlanetSited (THERMAL-1) | LE64(orb_id) ‖ ratio (albedo) ‖ ratio (orbit_r) |
| 26 | orb: thermal entry (THERMAL-1) | ratio (albedo) ‖ ratio (orbit_r) ‖ overheat u8 |
| 27 | event: VentSettled (VENT-1) | LE64(orb_id) ‖ ratio (amount) |
| 28 | orb: vent entry (VENT-1/SINK-1) | ratio (entropy_vented) ‖ LE64(last_touch) |
| 29 | event: DemurrageSettled (SINK-1) | LE64(orb_id) ‖ resource u8 ‖ ratio |
| 30 | event: ResourceTransferred (TRADE-1) | LE64(from_orb) ‖ LE64(to_orb) ‖ resource u8 ‖ ratio — banked value MOVES (conservation); both orbs settle demurrage at this touch; cohesion-book postings derive from this receipt + the counterparty's receipted crisis state, never stored |
| 31 | event: OverflowAbsorbed (ABSORB-1) | LE64(source_orb) ‖ LE64(absorber_orb) ‖ ratio — un-sealed overflow into the absorber's banked sink: sink_consumed = heat_absorbed = vented_after_shed (05 §1.4, the absorber's ledger carries the shed); opens a reciprocity window owed source → absorber |
| 32 | event: AbsorptionLapsed (ABSORB-1) | LE64(debtor_orb) ‖ LE64(creditor_orb) ‖ ratio — a window expired unpaid; the remainder posts to the debtor's extraction book (world-driven, the famine-strike shape) |
| 33 | event: BondSealed (BOND-1) | LE64(source_orb) ‖ LE64(target_orb) ‖ resource u8 ‖ ratio (rate/tick) ‖ LE64(term; 0 = open-ended) — one bond per (source, target, resource) |
| 34 | event: BondSettled (BOND-1) | LE64(source_orb) ‖ LE64(target_orb) ‖ resource u8 ‖ ratio (amount) ‖ severed u8 ‖ ratio (load) — ONE receipt settles the accumulated span at each touch; its own tag so replay can re-anchor the bond (a manual transfer must stay distinguishable); severed ends the bond, load 0 = orderly |
| 35 | event: CovenantSealed (COVENANT-1) | LE64(obligor_orb) ‖ LE64(beneficiary_orb) ‖ resource u8 ‖ ratio (quota) ‖ LE64(window) ‖ ratio (collateral) — the collateral rides the receipt so replay never re-derives the fidelity curve; the lock is a bank withdrawal (the vitrified vault, lossless by construction) |
| 36 | event: CovenantReleased (COVENANT-1) | LE64(obligor_orb) ‖ LE64(beneficiary_orb) ‖ resource u8 ‖ ratio (collateral) — quota met: collateral home, the stake posts fulfilled (world-driven at the tick boundary; fulfilment wins the deadline tie) |
| 37 | event: CovenantSlashed (COVENANT-1) | LE64(obligor_orb) ‖ LE64(beneficiary_orb) ‖ resource u8 ‖ ratio (collateral) — deadline passed unmet: BREACH_AND_SLASH to the aggrieved, the stake posts breached |
| 38 | event: FrictionSettled (IDEO-1) | LE64(orb_id) ‖ ratio (amount) — accrued ideological friction (φ × abs divergence × volume, transfers only) sealed into the mover's entropy_vented at its collect; aligned partners generate no receipt at all |
| 41 | orb: cohesion entry (IDEO-1) | ratio ×8 in fixed order: given_in_crisis ‖ drawn_in_crisis ‖ bond_flow ‖ severance_load ‖ covenants_fulfilled ‖ covenants_breached ‖ returned_across ‖ extracted_across — emitted only when a book is nonzero (a zero-book orb encodes byte-identical to a pre-ideology one); the four axis ratios DERIVE at read, never stored |
| 39 | event: TechDiscovered (TECH-1a, 10 §2.2) | LE64(orb) ‖ node_id[32] ‖ condition u8 ‖ modifier u8 ‖ ratio (magnitude) ‖ name utf8 — condition 0 is the KERNEL solar leap (12_UNION §5a: a world's first thermal opt-in receipts that its people know their sun — once per world); conditions 1–5 the crystallized emergent nodes (TECH-1b); tag 40 stays RESERVED for TechLicensed (blueprint trade; PLANNER-1's licensed leaps) |
| 42 | event: GaugeShifted (AEON-1a, 11 §2.1) | ratio (gauge) — the world's key moved on the torus fiber; the receipt carries the WRAPPED value [0,1), replay takes it verbatim (the last stands) |
| 43 | event: Diverged (AEON-1a, 11 §3) | ratio (gauge) ‖ branch utf8 — the line cleaved: a peer branch departs (its own chronicle — annex never merge); this world's key moves; no parent, no child |
| 44 | event: Confluent (AEON-1a, 11 §3) | ratio (gauge) ‖ partner utf8 — two bonded lines combined; the key settles at the interference (the torus midpoint, computed by the caller, receipted wrapped) |
| 45 | event: StarAged (AEON-1a, 11 §6) | class u8 — the star moved in its mass × composition × age region; luminosity recomputes, every sited orbit's T_eq⁴ shifts at the next tick |
| 46 | event: EraFolded (AEON-1a, 11 §5) | LE64(era) ‖ ratio (base gauge) — the era sealed into the archive sequence; the new aeon seats on its own G-sheet (base G advances) — archival by construction, nothing destroyed |
| 60 | event: GaugeShiftedAt (GAUGE-N, 11 §2.1) | LE16(grade) ‖ ratio (gauge, wrapped) — the key at a GRADE: the fiber as a graded vector; a shift at an unopened grade GROWS the key (skipped planes rest at ground); grade 0 is the landed scalar (tag 42 stays its shorthand forever) |
| 47 | event: ApertureSent (APERTURE-1, NS-4 §2) | LE64(deal) ‖ LE64(from_orb) ‖ resource u8 ‖ ratio (amount) — the SOURCE leg of a cross-world hand-off: banked value LEFT this world, bound to the deal id its far leg carries; replay sees only the debit — pair conservation is an N6 mesh-audit reading, never an orb law |
| 40 | event: TechLicensed (PLANNER-1, 12_UNION §2) | LE64(deal) ‖ LE64(from_orb) ‖ LE64(council) ‖ condition u8 ‖ node_id[32] ‖ name utf8 — the issuing leg of a cross-world technology hand-off (the aperture shape: the far world receipts TechDiscovered under the same deal, the same node — seeded from deal+condition so both legs derive it); council-gated, shield-bounded, receiver must have crossed; condition 0 (the solar leap) is NEVER licensed |
| 52 | event: CouncilSealed (COUNCIL-1, 12_UNION §7) | LE64(council) ‖ resource u8 ‖ ratio (quota) ‖ LE64(window) ‖ ratio (collateral) ‖ LE64 × N (members, rest/8) — three or more distinct orbs bound alike; every member's lock withdraws at the seal (refuses whole if any member cannot stake); deliveries to fellow members credit the giver's quota (transfers + bond flow) |
| 53 | event: CouncilClosed (COUNCIL-1) | LE64(council) ‖ LE64(member) ‖ fulfilled u8 ‖ ratio (banked) — one member's reckoning at the boundary (all judged together, never early); `banked` is exactly what returned to this member (own collateral + forfeit share if faithful; zero if breaching amid faithful; collateral home on total-breach dissolution); the integrity stake reads from the sealed council |
| 54 | event: GalaxyObserved (GALAXY-1, 13_GALAXY §3–§4 — universe ledger only) | LE16(observer len) ‖ observer utf8 ‖ LE16(creator len) ‖ creator utf8 ‖ LE16(name len) ‖ name utf8 ‖ ratio ×3 (center S, wrapped) ‖ ratio (reach) — the observation deed: appended beside EraAnnexed on `convergence.tlv`, never in a world chronicle (world replay ignores it, the explicit no-op arm); the frontier is a distance, never an outside |
| 48 | event: ApertureReceived (APERTURE-1, NS-4 §2) | LE64(deal) ‖ LE64(to_orb) ‖ resource u8 ‖ ratio (amount) — the DESTINATION leg: banked value ARRIVED from another world under the same deal; replay sees only the credit; relieves a famine strike at the 02 §2.4 law |
| 10 | event: EntityRegistered (N3) | LE64(entity_id) ‖ LE64(principal) ‖ handle utf8 |
| 11 | event: ScopeGranted (N3) | LE64(agent) ‖ scope u8 |
| 12 | event: ScopeRevoked (N3) | LE64(agent) ‖ scope u8 |
| 13 | event: ColonyCharted (P1) | LE64(orb_id) ‖ LE64(source_orb) ‖ name utf8 — the founding reserves MOVE from source (conservation) |
| 16 | orb header | LE64(orb_id) ‖ name utf8 |
| 20 | receipt frame (P2, NS-3 §2.4) | the on-disk framing of one whole receipt |
| 17 | orb zone entry | LE16(tile) ‖ kind u8 |
| 18 | orb banked entry | resource u8 ‖ ratio |
| 19 | orb famine entry (D1) | resource u8 |

## 3. Test vectors — extracted from the running engine (ruleset v1)

Session: found "meridian" (banks 20 iron / 10 biomass) → zone tile 4
extractor (spends 10 iron) → 3 ticks (nets +9/2 iron, −3/2 biomass) →
collect (settles both). (`cargo run -p cfe --example ns1_vectors`
reproduces every byte.)

### 3.1 The orb, byte-exact

```
101000000001000000000000006d6572696469616e   tag16 len16: orb 1, "meridian"
1103000000040001                             tag17 len3:  tile 4, extractor
120c0000000100010000001d0100000002           tag18 len12: iron, 29/2
120c000000020001000000110100000002           tag18 len12: biomass, 17/2
```
Round trip: decode ‖ re-encode reproduces the bytes exactly (measured:
`the_orb_round_trips_byte_exact`).

### 3.2 The events

```
ColonyFounded{1,"meridian"}:  011000000001000000000000006d6572696469616e
ZoneBuilt{1,4,Extractor}:     020b0000000100000000000000040001
YieldCollected{1,Iron,9/2}:   03140000000100000000000000010001000000090100000002
```

### 3.3 The session's receipts (NS-3 digests over the NS-1 event bytes)

Since N3, every entity-forged receipt carries the actor's generation tag in
provenance (here `[1]`, the sovereign — the tag participates in the
digest), and the sovereign's registration precedes the founding:

```
receipt 0  stamp 0  prov [1]  parent 00…00   (EntityRegistered founder)
           own 174f294771ac5f2f41cb9670edc08350198bd6d66de79891f5ae3e91c8f62e3e
receipt 1  stamp 0  prov [1]  parent 174f2947…   (ColonyFounded)
           own a4ee7b018d7fd97caa3c1d434f28cd36ebb0bfe04fff63cff4983f3978fb53a0
receipt 2  stamp 0  prov [1]  parent a4ee7b01…   (ZoneBuilt)
           own 71f3f22f7b215a8c2b919d8af75be8bacf98368b43a881683480115c9511f948
receipt 3  stamp 3  prov [1]  parent 71f3f22f…   (YieldCollected iron 9/2)
           own 3fdd13fd79c04ea095953307daca1120e231c6c349ada5cc8092f79c03e1fef1
receipt 4  stamp 3  prov [1]  parent 3fdd13fd…   (UpkeepSettled biomass 3/2)
           own 16e33872ca55c70ea17a7ee3164b847c661070e9456d539bbd1f493780e34542
```
Chain `Intact`; replay of the chronicle equals the committed world
(measured: `the_chronicle_replays_the_committed_world`).

## 4. Conformance

An implementation conforms when: (a) equal orb state emits identical bytes
(canonical ratios + canonical order); (b) decode ‖ re-encode round-trips
byte-exact; (c) unknown tags skip without error; (d) committed state
contains only receipted values — replaying the receipts reproduces it
exactly; (e) the §3 vectors reproduce.
