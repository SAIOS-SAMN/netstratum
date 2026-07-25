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
| 2 | event: ZoneBuilt | LE64(orb_id) ‖ LE16(tile) ‖ kind u8 (1 extractor, 2 habitat) |
| 3 | event: YieldCollected | LE64(orb_id) ‖ resource u8 (1 iron, 2 biomass) ‖ ratio |
| 16 | orb header | LE64(orb_id) ‖ name utf8 |
| 17 | orb zone entry | LE16(tile) ‖ kind u8 |
| 18 | orb banked entry | resource u8 ‖ ratio |

## 3. Test vectors — extracted from the running engine

Session: found "meridian" → zone tile 4 extractor → 3 ticks → collect.
(`cargo run -p cfe --example ns1_vectors` reproduces every byte.)

### 3.1 The orb, byte-exact

```
101000000001000000000000006d6572696469616e   tag16 len16: orb 1, "meridian"
1103000000040001                             tag17 len3:  tile 4, extractor
120c000000010001000000090100000002           tag18 len12: iron, 9/2
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

```
receipt 0  stamp 0  parent 00…00 (genesis)
           own 6fd67c76f56c0812a77af13061b92cca898683f02db01e5a627e30562b379c70
receipt 1  stamp 0  parent 6fd67c76…
           own 2b75afc415e8bea409207e8f8ac4e914552a5220abfe8a02f6dfd8bf577bbb26
receipt 2  stamp 3  parent 2b75afc4…
           own 8f8cef2122828202a928b7d9579b3eef057b0299bd22466f8cd1ce203420f394
```
Chain `Intact`; replay of the chronicle equals the committed world
(measured: `the_chronicle_replays_the_committed_world`).

## 4. Conformance

An implementation conforms when: (a) equal orb state emits identical bytes
(canonical ratios + canonical order); (b) decode ‖ re-encode round-trips
byte-exact; (c) unknown tags skip without error; (d) committed state
contains only receipted values — replaying the receipts reproduces it
exactly; (e) the §3 vectors reproduce.
