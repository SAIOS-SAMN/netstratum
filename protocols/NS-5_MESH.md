# NS-5 — MESH: Proof-of-Convergence Consensus

**Status:** LANDING at N6, first slice MESH-1a (this document supersedes
the stub; first consumer: two-node synchronization of ONE world's
chronicle; mechanic `engine/mesh`, witnesses
`engine/mesh/tests/mesh_consilience.rs`). Working orders and the three
operator rulings of 2026-07-30: `docs/N6_DIRECTIVES.md`. Supersedes
`/opt/repos/convergence-engine/DIMENSIONAL_BLOCKCHAIN.md` (two-chain:
temporal anchors + dimensional receipts) and HOW_TO_USE.md ("two
laptops, one mesh") — carrying independent work, exact measurement, and
disagreement preserved; dropping the retired SAIOS vocabulary and the
ancestor's majority finality (witness weight > 0.5). Consilience is
unanimous or refused, structurally — the house law the world clock
already keeps (NS-3 §1.3, 3-of-3).

Independent engine instances converge on one world state by exact
measurement: consilience at arity ≥ 2, receipts as the shared spine, a
forged or drifted state refusing typed. No central server; no global
transaction log.

**The identity ruling (2026-07-30):** at N6 the mesh is blake3 and
provenance only. Consilience-of-independent-computation over
exact-rational state reduction IS the claim — no key-pair signatures
enter the registry here. External cryptographic identity arrives at N7
(ANCHOR), where the mesh meets parties it cannot re-compute.

## 1. RFA semantics

### 1.1 The measured manifold (v1)

A node projects a world's committed state (the replay of its chronicle,
NS-3; SEAL B makes the replay the state) onto an ordered vector of
exact rationals:

1. the graded gauge key, grade-ascending (grade 0 first);
2. then, per orb ascending by orb id, the banked ledger over resource
   codes 1–7 in code order.

Two nodes measuring together span the UNION of their grades and orb
ids; an absent coordinate (an unopened grade, an unfounded orb, an
unbanked resource) reads exactly 0. The projection is total,
deterministic, and canonical: one committed state, one point, one
encoding. Widening the manifold (new dimensions) is a spec extension
with a new manifold code — never a silent change (the SEAL A instinct
applied to geometry).

### 1.2 The consilience triangle

The claim "two independent paths, one position" is a triangle. Build
the 3-node Delta (rfa-core) over the manifold fiber:

- Δ(origin → A) = node A's projection
- Δ(origin → B) = node B's projection
- Δ(A → B) = 0 — the shared-position claim

The cocycle residual over the canonical triple (origin, A, B) is
`R_l = B_l − A_l` per coordinate. `coherence_functional` sums the six
ordered triples of the triangle, so **C(Δ) = 6·Σ R_l²** — exact
rational, and zero iff every residual is zero. Consilience is
`C(Δ) = 0` EXACTLY. There is no threshold, no tolerance, no vote.

Arity ≥ 2 generalizes without new law: n nodes, one origin, the
shared-position claim pairwise zero; C(Δ) = 0 iff ALL agree. Unanimity
is the geometry.

### 1.3 The judgment

`consile(A, B)` over two intact chronicles of one world:

1. **The spine walk.** Compare receipts by own-digest from sequence 0.
   The first differing sequence is the FORK.
2. **Fork found → DIVERGED.** Both chronicles replay whole (each is
   internally intact); both project (§1.1); the refusal carries the
   fork sequence, the residual list WHOLE — every nonzero
   `(dimension, B_l − A_l)` in canonical form — and C(Δ) as the summary
   scalar, never the payload (the N4 §5 law). Disagreement is
   preserved for resolution, never voted away; in-world, divergence
   already has its vocabulary (tags 43/44 — Diverged, Confluent).
3. **No fork, unequal lengths → DRIFT.** The shorter side is BEHIND;
   the refusal names the missing half-open range `[from, to)`. Drift
   is cured by carrying the tail (§2.4) — never by re-voting history.
4. **Equal spines → the measurement.** Both sides replay
   INDEPENDENTLY; the triangle closes; C(Δ) measures. Zero issues the
   consilience receipt (§2.2) recording length, head stamp, head
   digest, arity, manifold width, and the measured zero. A nonzero C
   on an equal spine would name a broken engine (SEAL B violated) and
   refuses as DIVERGED at the head — the measurement judges, nothing
   is assumed.

### 1.4 The refusal taxonomy

All refusals are typed and carried whole (§2.3):

- **MALFORMED** — a receipt frame that does not parse (side, frame).
- **FORGED** — a chain that does not re-verify from its own content
  (side, first broken sequence). Verification is NS-3
  `verify_receipts`: recomputed digests, walked parent links. A
  chronicle in hand is intact by construction (the one-door law);
  forgery is caught at the boundary, in `receive`.
- **DRIFT** — same history, less of it (behind side, missing range).
- **DIVERGED** — forked history (fork sequence, residuals whole, C).

### 1.5 Scope

MESH-1a is ONE world's chronicle between two nodes (operator ruling
2026-07-30). Multi-world aggregation and the universal ledger belong to
MESH-1c / N7. Consilience is a READ: no receipt enters any world
chronicle; mesh records live in the mesh's own framing (§2), beside
the world, never inside it.

## 2. Wire format

TLV throughout (`tag u8 ‖ LE32(length) ‖ value`, SEAL A: readers skip
unknown tags). Tags 64–66, claimed in the dop.rs registry header — the
one authority. Ratios ride the canonical NS-1 carrier
(`ratio::encode`). Side codes: 1 = A, 2 = B.

### 2.1 Tag 64 — the mesh head announcement

`genesis[32] ‖ LE64(length) ‖ LE64(stamp) ‖ head[32]` — 80 bytes.

`genesis` = the own-digest of receipt 0 (the chain's identity; 32
zeros when empty); `length` = receipt count; `stamp` = the head
receipt's stamp (0 when empty); `head` = the head receipt's own-digest
(32 zeros when empty).

### 2.2 Tag 65 — the consilience receipt

`LE64(length) ‖ LE64(stamp) ‖ head[32] ‖ arity u8 ‖ LE32(dimensions) ‖
ratio(coherence)`.

`dimensions` = the manifold width measured; `coherence` = the measured
C(Δ), exactly 0 by issuance law (§1.3.4).

### 2.3 Tag 66 — the mesh refusal

`kind u8` then per kind:

- 1 MALFORMED: `side u8 ‖ LE64(frame)`
- 2 FORGED: `side u8 ‖ LE64(at)`
- 3 DRIFT: `behind u8 ‖ LE64(from) ‖ LE64(to)`
- 4 DIVERGED: `LE64(at) ‖ LE32(residual count) ‖
  (LE32(dimension) ‖ ratio(residual))… ‖ ratio(coherence)`

The residual list is the payload, whole; C is the trailing summary.

### 2.4 The carrier

The transport ruling (2026-07-30): peers exchange `chronicle.tlv`
segments — the NS-3 §2.4 on-disk framing IS the wire; there is no
second encoding of history. A dumb file/directory adapter carries
whole files or appended tails (`persist::sync` appends only what the
far side lacks); TCP arrives later as an equally logic-free adapter
(SEAL C). The engine never opens a socket.

## 3. Test vectors

Witnessed in `engine/mesh/tests/mesh_consilience.rs`; the four
semantic vectors plus the codec witness backing §2.

1. **Two independent replays consile at exact zero.** One session
   plays, syncs to its file; the file carries to the peer; both sides
   load through the door and replay independently; `consile` issues
   the receipt: C = 0/1 exactly, length/stamp/head matching the spine,
   arity 2, the v1 manifold width.
2. **A forged file refuses typed.** The peer's file re-frames one
   receipt with a moved stamp; the chain cannot recompute; `receive`
   refuses FORGED with the broken sequence named.
3. **A drifted peer names its range, and the tail cures it.** One side
   plays on; `consile` refuses DRIFT `[from, to)` naming the behind
   side; the tail syncs across (append-only, only the missing
   receipts); `consile` then issues the receipt at zero.
4. **Divergent histories carry their residuals whole.** A common
   prefix, then different closures on each side; `consile` refuses
   DIVERGED at the first differing sequence, the residual list naming
   each differing dimension with its exact rational difference —
   verified in the witness against the two replays' committed ledgers
   — and C(Δ) = 6·Σ R² recomputed exactly.
5. **The codec witness.** Tags 64/65/66 round-trip byte-exact
   (equal values, equal bytes; the DIVERGED residual list whole), and
   an unknown future tag skips (SEAL A).
