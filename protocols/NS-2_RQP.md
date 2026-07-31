# NS-2 — RQP: Resonance Query Protocol

**Status:** LANDING at N4 (this document supersedes the stub; first
consumer: the scanner, `clients/cli` verb `scan`, mechanic
`engine/cfe/src/query.rs`, witnesses `rqp_scanner.rs`). Written FROM the
working v0.4 sigil index (`/opt/ipsal-autic-v0.4/iae/src/reduction/mod.rs`,
the SIGIL1 block — CHARTER §8 ratifies the lineage). Supersedes
PROTOCOL_STACK.md §2. Working orders: `docs/N4_DIRECTIVES.md`.

Search by structural convergence with a measurement, never by keyword. A
query is a structure; a hit is a resonance (sigil-address lookup,
O(bucket), never a scan of the population); a near-miss refuses with its
delta whole.

**The lens law (operator ruling 2026-07-30, recorded here):** SEAL B
replay is the REPRODUCTION — byte-identical rebuilding of the committed
world. RQP is the LENS — a pure structural read of the same receipted
truth. A query mutates nothing, orphans nothing, and never enters the
world chronicle; only its power draw (§1.5) closes and receipts. The
world's coordinate is TUniversal{S, G, T}; the lens reads along {S, G}
and leaves T in the ledger: **you query an orb by WHAT it is (S, G),
never by WHEN it was found (T)** — the ancestor's law, carried whole.

---

## 1. RFA semantics

### 1.1 The sigil address

Every orb projects a **sigil**: a frame-invariant canonical form of its
committed structure, computed pure at read time — never stored, never
receipted (the derived-reading law, the cohesion-ratio precedent).

The canonical form is the **joint shadow**: the lex-min over the eight
grid symmetries (four rotations × reflection — the plane group of the
8×8 chart, a structural constant bounded by group order, never a content
sweep) of the orb's zone list as sorted (row, col, kind) tuples, joined
with the banked ledger (resource-ascending, zeros absent — the NS-1
canonical order) and the thermal class. Kind is the LAST tuple
component, so the joint lex-min settles positions first, kinds after —
the position projection of the joint winner IS the position-only lex-min
(measured: `the_sigil_is_frame_invariant`).

The **address** is two projections of that one shadow (never two
independent canonicalizations — the v0.4 dimensional-collapse warning:
marginals computed apart drop the cross-terms):

- **S-canonical** — the (row, col) list of the joint winner: the zone
  geometry under the grid symmetry, kinds and amounts shed.
- **G-canonical** — the ascending resource-code set of the nonzero
  banked ledger, plus the thermal class: the substrate band
  (`thermal::band_of` — cryogenic/mesic/hyperthermal wire codes 1–3, 0
  for an unsited or starless orb). Presence, not amounts: the address
  is coarse on purpose — exactness lives in the resonance check (§1.2),
  the bucket only has to be honest.

**T is not in the address.** The world clock, the receipt stamps, the
orb's touch history — none of it keys the lookup. T rides the hit as a
ledger-side annotation (§1.3): the stamp of the reading's own power
draw, the one chronicle trace a query leaves.

*(RULED 2026-07-30 (merge sign-off, ruling 2): this v1 queryable
structure — zone geometry as S, banked signature + thermal class as
G — stands as drafted. Extension law in §4.)*

### 1.2 Resonance

Two orbs **resonate** iff their joint shadows are EQUAL — zones
congruent under some grid symmetry, banked ledgers equal ratio-for-ratio
in canonical form, thermal class equal. Equality is exact structural
identity of canonical forms; there is no threshold, no score, no
similarity. The lookup is by address bucket (O(bucket)); the verdict is
by joint comparison inside the bucket.

A scan names a **prober** (the orb that pays the draw, §1.5) and an
**exemplar** (the orb whose structure IS the query — the CLI's
query-by-example form; a structure-literal query is the same wire
record, §2.2). The exemplar never resonates with itself: the scanner
reads the population, not the probe. The prober is a candidate like any
other. The reading happens AFTER the draw closes: the lens reads the
committed world as it stands once open — the prober's own debited bank
included.

### 1.3 The hit

A hit carries: the resonant orb's id, and the T stamp of the reading
(the `CrystalDrawn` receipt's stamp, §1.5) — WHEN this lens was paid
for, never WHEN the orb became what it is. Multiple resonances all
report (the bucket is small; nothing truncates silently).

### 1.4 The near-miss refusal

A candidate in the same bucket whose joint shadow does NOT match refuses
**with its delta whole** (N4_DIRECTIVES directive 5). The refusal
carries, per candidate:

- the **banked residuals**, one per resource where the ledgers differ:
  the exact rational difference (probe − candidate), the full list,
  never a truncation;
- the **zone deltas**: the tiles where the joint winners disagree, each
  named (tile, probe kind, candidate kind — kind code 0 for an unzoned
  side);
- the **thermal pair** when the classes differ (impossible in v1 — the
  band is in the address — carried in the record so the surface never
  changes when a later ruling widens the bucket);
- the summary scalar `C(Δ) = Σ R²` over the banked residuals
  (`rfa_core::coherence_functional` — exact rational, zero iff the
  ledgers agree). **The scalar is the summary, never the payload**: a
  consumer that reads only C(Δ) has not read the refusal.

An empty bucket is its own typed verdict — `NoResonance`, carrying the
probed address and nothing else. It is NOT a delta against the whole
population: nearest-candidate search would be the O(population) scan
this protocol exists to refuse.

### 1.5 The power draw — resonance crystals

The scanner draws **resonance crystals** (02 §5.1, deferred to exactly
this landing — the name earns its meaning here: crystals are the
scanner's power source). The draw law, the SINK-1 touch shape:

- Affordability is judged PURE on the probing orb's banked crystals
  before anything moves: short of `scan_draw()` (placeholder: 1
  crystal), the scan refuses typed (`CannotAfford`), draws nothing,
  reads nothing, orphans nothing.
- A funded scan debits `scan_draw()` from the probing orb's bank and
  closes `CrystalDrawn` (tag 55) — the ONE chronicle event of this
  protocol. The receipt carries the probing orb and the amount,
  **deliberately blind to what was asked and what was seen**: the lens
  law in wire form. Replay replays the debit; it cannot replay the
  reading, and must not.
- The draw is spent on the LOOKING, hit or miss: a lens costs power to
  hold open regardless of what stands in front of it.

Crystals are produced by the **Resonator** (zone kind 7), the fourth
converter (02 §5.2 pattern — upkeep is input, yield is output): cost 10
alloy + 5 diamond; input 1/4 diamond per tick; output 1/8 crystal per
tick; converter-hot (factory heat 1). One synthesizer feeds one
resonator on the knife edge — diamond nets exactly 0 (the 2E+1H
precedent). Resource wire code 7. All constants placeholder parameters
in `dop.rs`, named fns beside the types they rule. The materials law
holds: produced by one zone, consumed by the scanner.

### 1.6 Scope

v1 scans the LOCAL world's orb population. Cross-world resonance
composes at N5/N6 (the mesh carries addresses, never worlds);
scanner-by-galaxy-region is post-GALAXY-1a and post-N4 (N4_DIRECTIVES
coordination note). G3 (perception frames, translation loss) reads
through this same lens when it lands — after G1, per the pending
operator ruling (N4_DIRECTIVES ruling 1).

---

## 2. Wire format

Additive TLV under SEAL A, tags from the permanent registry (dop.rs
header — the one tag authority). **Records 56–58 are NS-2 protocol
records in NS-2's own framing: they NEVER enter the world chronicle.**
Only tag 55 is a chronicle event.

### 2.1 Tag 55 — `ClosureEvent::CrystalDrawn` (the chronicle event)

    LE64(orb_id) ‖ ratio(amount)

The probing orb and the crystals drawn. Ratio encoding is the NS-1
canonical carrier (`ratio.rs`). Replay: debit `orb_id`'s banked
crystals by `amount`.

### 2.2 Tag 56 — the query record

    S: LE16(zone_count) ‖ zone_count × (row u8 ‖ col u8)
    G: LE8(resource_count) ‖ resource_count × (resource u8, ascending)
    thermal_class u8 (0 unbanded ‖ 1 cryogenic ‖ 2 mesic ‖ 3 hyperthermal)

The address only — a query carries no amounts, no kinds, no T.

### 2.3 Tag 57 — the hit record

    LE64(orb_id) ‖ LE64(t_stamp)

`t_stamp` is the CrystalDrawn receipt's stamp: ledger-side annotation,
never a key.

### 2.4 Tag 58 — the refusal record

    LE64(candidate_orb)
    LE8(banked_residual_count) ‖ n × (resource u8 ‖ ratio(probe − candidate))
    LE8(zone_delta_count)      ‖ n × (tile LE16 ‖ probe_kind u8 ‖ candidate_kind u8)
    probe_thermal u8 ‖ candidate_thermal u8
    ratio(C(Δ))

The residual list whole; the scalar last, as summary.

### 2.5 Codec vectors

Extracted from the running engine (`cargo run -p cfe --example
ns2_vectors`) — any implementation verifies from this document alone.
The subject is the witness board's planted pair (§3): the probe holds
extractor@tile 10 (1,2) + habitat@tile 21 (2,5), banked 10 iron + 5
biomass, unbanded; its rot90 twin (extractor@22, habitat@45, banked
equal) projects the IDENTICAL sigil:

    s_canonical: [(1,2), (2,5)]    g_resources: [1, 2]    thermal: 0
    joint zones: [(1,2,1), (2,5,2)]

Tag 56, the query record (the address only):

    380a00000002000102020502010200

Tag 57, the hit record (orb 3, stamp 178):

    39100000000300000000000000b200000000000000

Tag 58, the refusal record — the kind swap (delta real, C(Δ) = 0):

    3a1e000000040000000000000000020a00010215000201000000000000000100000001

Tag 58, the refusal record — the banked miss (iron −5, biomass −2,
C(Δ) = 25 + 4 = 29):

    3a2f00000005000000000000000201010100000005010000000102010100000002010000000100000000010000001d0100000001

Tag 55, `ClosureEvent::CrystalDrawn` (orb 1, 1 crystal; round-trips
byte-exact through `from_wire`):

    371300000001000000000000000001000000010100000001

---

## 3. Test vectors

Measured by `engine/cfe/tests/rqp_scanner.rs` (5 witnesses green; the
acceptance criteria on the board, BUILD_ORDER §1 N4 row). The board:
the genesis prober climbs the whole pipeline (2E+2H → 7E+7H → 4
smelters → synthesizer → resonator → one banked crystal, every
converter on its knife edge) and charters four daughters — A the
exemplar, B the rot90 twin, C the kind swap, D the once-collected.

1. **The planted match:** A (extractor@10, habitat@21) and B
   (extractor@22, habitat@45) — the same pattern under rot90, banked
   equal (charter reserves minus the same costs, never collected) —
   project ONE sigil; the scan finds B through the lens (measured:
   `the_sigil_is_frame_invariant`, `the_scan_finds_the_match_…`).
2. **The planted near-misses, delta whole:** C (B's tiles, kinds
   exchanged) refuses with zone deltas [(10, extractor↔habitat),
   (21, habitat↔extractor)], NO banked residuals, and **C(Δ) = 0 while
   the delta is real** — the scalar is the summary, never the payload.
   D (B's pattern, collected once at +4 ticks: 1E+1H nets iron +5/4,
   biomass +1/2 per tick) refuses with residuals iron −5, biomass −2
   whole and C(Δ) = 25 + 4 = 29, exact.
3. **The lens law:** the chronicle before and after the scan differs
   by EXACTLY one receipt — the CrystalDrawn debit, blind to the
   asking; the reading's stamp IS that receipt's stamp.
4. **The refusal orphans nothing:** a crystal-less scan refuses
   `CannotAfford { Crystal, need 1, have 0 }` typed and pure — no
   receipt, no draw, the chronicle unmoved.
5. **The empty bucket is its own verdict:** the prober's unique
   structure scans to zero hits, zero refusals — never a delta against
   the population.
6. **The resonator converts on the knife edge:** alloy +4 − 4 = 0 and
   diamond +1/4 − 1/4 = 0 accrue NOTHING; crystal accrues 1/8 × 8 = 1
   exactly; collect banks it; famine strikes nothing.
7. **Replay MATCHES** at the end of every arc —
   `assert_eq!(replay(world.chronicle()), world.committed())`.

---

## 4. The queryable structure (ruled 2026-07-30; the extension law)

### 4.1 The v1 cut, ratified

The operator ruled at merge sign-off (ruling 2): the v1 structure
stands as drafted in §1.1 —

| facet | projection | joint (the exactness) |
|---|---|---|
| S | zone geometry, lex-min over the eight grid symmetries | (row, col, kind) tuples under the one winning transform |
| G | nonzero banked resource SET + thermal class | the full banked ledger, ratio-for-ratio + the class |
| T | **never in the key** | rides the hit as the draw receipt's stamp |

### 4.2 The extension law

The structure grows ADDITIVELY, the SEAL A discipline applied to the
address: a new facet appends to the END of the query record (§2.2) and
to the joint; it never reorders or reinterprets an existing byte. An
implementation that predates a facet still buckets correctly on the
prefix it knows — it reports coarser resonance, never wrong resonance.
Every extension is an operator ruling recorded HERE, one row per facet,
with its landing named. The dimensional-collapse warning binds every
facet forever: projections derive from ONE joint shadow, never from
independent canonicalizations.

### 4.2a The translation law (G3, LANDED 2026-07-30) — a CORRECTION recorded

The §4.3 reservation drafted perception as a G extension. Landing it
under the axiom check REFUSED that shape: 04 §1.3's law is that the
H¹ class IS what survives translation — frames are what
canonicalization quotients OUT, so the observer's basis joins T
outside the key: **query by WHAT it is, never by WHEN it was found or
WHO reads it.** The sigil is frame-free (measured:
`the_sigil_is_frame_free`).

Translation is a READ-time law instead: every hit annotates with the
**survival ratio** between the prober's sealed basis and the
candidate's (`dop::translation_survival` — same basis 1; the 04 §4
example verbatim, harmonic through a planar eye 3/4; umbral pairs
1/2; unsealed reads whole; exact rationals, the loss SHOWN never
silent). The tag-57 hit record extends ADDITIVELY — the survival
rides behind the pre-G3 prefix; an old reader parses orb ‖ stamp and
never needs the tail (measured: `the_hit_record_extends_additively`).
`translated_read` is the pure cross-frame seam, N5-ready: resonance
is joint equality (the class), survival is the pair's frame distance.
Local scans read whole (one world, one sealed basis); the cross-frame
pairing composes at N5/N6 through the same table.

### 4.3 Reserved extensions (recorded, not landed)
- **The galaxy-region facet (post-GALAXY-1a, post-N4):** scanning by
  charted sky region — derived S-side membership (`galaxy_of`,
  wrapped-L1 to charted centers), never a stored tag. Composes with the
  N4_DIRECTIVES coordination note; a separate landing, gated on both.
- **Cross-world resonance (N5/N6):** the mesh carries ADDRESSES, never
  worlds; consilience at arity ≥ 2 compares joints across chronicles.
  Nothing in v1's wire changes — the records were framed for this.
