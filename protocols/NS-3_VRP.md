# NS-3 — VRP: Verification Receipt Protocol

**Status:** landed at N1; §2.2 TLV event encoding paid at N2 (conformance §4
met by `engine/cfe` — witnesses `ns3_conformance.rs`). Written from the
working v0.4 chronicle organ
(`/opt/ipsal-autic-v0.4/iae/src/chronicle/`, exercised by
`iae/tests/chronicle_receipts.rs`, 960 attested boundaries live in the sealed
build). First consumer: the world chronicle — every player and agent action in
Netstratum. Supersedes PROTOCOL_STACK.md §3 (VRP).

VRP is the game's ground truth (Charter §7, SEAL B): an append-only,
parent-linked chain of receipts, one per **closure** (a completed, verified
action — never a provisional or per-tick state). Swap the renderer, upgrade
the engine, migrate the settlement chain — the chronicle replays the universe
from sequence zero.

---

## 1. RFA semantics

### 1.1 The receipt

A receipt is the tuple:

| field | type | meaning |
|---|---|---|
| `sequence` | u64 | position in the chain, dense from 0 |
| `stamp` | u64 | the world-clock value the receipt is stamped with (§1.3) |
| `parent` | 32 bytes | the previous receipt's digest; sequence 0 carries the **genesis parent**, 32 zero bytes |
| `own` | 32 bytes | this receipt's digest (§2.1) |
| `event` | closure event | What closed (the event vocabulary is opened per landing, §2.2) |
| `provenance` | list of u64 | the forging path identifiers — WHO/HOW, named without being consulted by any game rule |

### 1.2 The append-only law

The chronicle exposes exactly one write: `append(event, stamp, provenance)`.
The implementation computes `parent` and `own` internally — **callers cannot
supply either**, so a forged link cannot enter through the API. There is no
remove, no mutate, no reorder; append-only is structural, not policed.

### 1.3 The stamp law and the tick law

Time is a separate organ from the chain:

- **The world clock** is its own counter, starting at 0. It advances only by
  **3-of-3 unanimous timekeeper attestation of the exact successor value**.
  A 2-of-3 quorum leaves the clock open and unmoved, with every keeper's
  answer carried whole in the typed result. A unanimous attestation of any
  non-successor value (skip or replay) also refuses — the clock cannot jump
  backward or forward.
- **The stamp law:** appending a receipt stamps it with the clock value it is
  given; the append signature has no clock access, so a receipt can never
  move time. Correlation, never causation, in both directions: closures don't
  tick the clock, ticks don't create receipts.

### 1.4 The verification read

`verify_chain` recomputes every receipt's digest from its own content and
checks both links (`parent` equals the predecessor's `own`; `own` equals the
recomputed digest). The result is typed: `Intact`, or `Broken { at }` naming
the first failed sequence. Any single altered byte of content anywhere in a
receipt changes its digest and breaks the chain at that point (measured in
§3.3).

## 2. Wire format

### 2.1 The digest layout (normative)

`own = BLAKE3( LE64(sequence) ‖ LE64(stamp) ‖ parent[32] ‖ event_bytes ‖ LE64(k) for each provenance k )`

where `LE64` is 8-byte little-endian and `event_bytes` is the event's
canonical encoding (§2.2). The digest is the receipt's identity; hashes are
lawful at this layer only and never carry game-rule content.

### 2.2 The event encoding

The v0.4 ancestor hashes a session-stable textual encoding of the typed event
(its Rust `Debug` form) — sound within one build, explicitly provisional for
the wire (its own code comment homes the codec at the persistence landing).
NS-3 inherited that provisional encoding, and the debt is paid: **the normative Netstratum
event encoding is TLV** (NS-1 §2 — the record layout, the ratio carrier, and
the permanent tag registry), landed with the first engine consumer at N2 as
specified. `event_bytes` in §2.1 is the event's single TLV record. One tag,
one meaning — a retired tag's code is never reused (the v0.4 permanent
tag space is the working precedent and its collision-cure history the
cautionary record). Receipts over real NS-1 event bytes are vectored at
NS-1 §3.3.

### 2.3 What may chronicle

Closures only: a completed trade, a sealed lease split, a domain transfer, a
verified build — never per-tick simulation state, never speculative or
provisional values. This is what keeps the chain small enough to anchor
(NS-7) and replay.

### 2.4 The receipt file (P2-PERSIST — the chronicle on disk)

The save is the chain: one append-only file per world
(`var/worlds/<name>/chronicle.tlv`), a TLV stream of framed receipts.
The frame (tag 20, permanent): `LE64(sequence) ‖ LE64(stamp) ‖ parent[32]
‖ own[32] ‖ LE32(provenance count) ‖ LE64(k)… ‖ the event's TLV record`.

Laws, all measured (`persist_boot.rs`):
- **The only way back in:** loading re-verifies every link
  (`Chronicle::from_receipts`); a tampered file refuses with its break
  named. Unknown tags in the stream skip (SEAL A at the file layer).
- **The file only grows:** sync appends exactly the receipts the file
  lacks; a file holding history the chronicle does not have refuses
  (`HistoryDiverged`) — a past is never clobbered silently.
- **Resume is a reading, not a walk:** the clock reinstates at the highest
  stamped value; un-receipted ticks and accrued workspace from the prior
  era are honestly lost — what has no receipt does not persist.

## 3. Test vectors — extracted from the live v0.4 organ

Produced by driving the sealed build's chronicle directly (harness: read-only
path dependency on `/opt/ipsal-autic-v0.4/iae`; fixture identical to the
landed witness `chronicle_receipts.rs`). The `event_bytes` given are the
exact bytes hashed, so any independent implementation of §2.1 can verify
every digest below from this document alone.

### 3.1 The tick law, measured

```
clock genesis: 0
3-of-3 attest successor 1  -> Ticked, now = 1
2-of-3 attest successor 2  -> QuorumOpen, now = 1   (refusal carried whole)
3-of-3 attest 5 (a skip)   -> QuorumOpen, now = 1   (the clock cannot jump)
```

### 3.2 The chain, byte-exact

Receipt 0 — `sequence=0`, `stamp=0`, `provenance=[3,7]`
`event_bytes` (86 chars of the fixture event, verbatim):
`Consilience { rule: ObserverFrame { plane: PlaneElement { exchange: Keep, row: Keep, col: Keep }, relabel: Relabel { pairs: [(1, 4), (4, 1)] } } }`
```
parent: 0000000000000000000000000000000000000000000000000000000000000000
own:    c86c17aa50e457a9e5ef0104af55c198d14c90a3647b27021c5f26989954ba28
```

Receipt 1 — `sequence=1`, `stamp=0`, `provenance=[11]`
`event_bytes`:
`Crystal { rule: ObserverFrame { plane: PlaneElement { exchange: Keep, row: Keep, col: Keep }, relabel: Relabel { pairs: [(1, 4), (4, 1)] } }, vantage_provenance: [Provenance("arc_3c9b0459")] }`
```
parent: c86c17aa50e457a9e5ef0104af55c198d14c90a3647b27021c5f26989954ba28
own:    aca043784d14a85836406a564d68defd3dd77b89267603eb3a90be50677eaf05
```

Receipt 2 — `sequence=2`, `stamp=1`, `provenance=[13]`
`event_bytes`:
`SunInstall { rule: ObserverFrame { plane: PlaneElement { exchange: Keep, row: Keep, col: Keep }, relabel: Relabel { pairs: [(1, 4), (4, 1)] } } }`
```
parent: aca043784d14a85836406a564d68defd3dd77b89267603eb3a90be50677eaf05
own:    877cf37994b93df067efb3ec363e3f00f503cb0b36df8ac54424ca6918496250
```

Chain verification over all three: `Intact`.

### 3.3 Tamper divergence

Receipt 0 rebuilt with one changed provenance value (`[3,8]` for `[3,7]`):
```
own:    366aa785fd766cb2e698e76d56e0cc26636559206174e70661fe809041339057
```
A single altered u64 diverges the digest completely; in a chain, every
successor's `parent` link then fails and `verify_chain` names the break.

## 4. Conformance

An implementation conforms when: (a) it reproduces every digest in §3.2 and
§3.3 from the stated byte layout; (b) its receipt store exposes no write
besides append and computes both hashes internally; (c) its clock refuses
2-of-3, skip, and replay per §3.1; (d) appending cannot change the clock and
ticking cannot append; (e) `verify_chain` names the first broken sequence.
