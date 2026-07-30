# NS-5 — MESH: two nodes, one world

**Status:** N6a SPECCED + MESH-1a LANDED (both 2026-07-30, reconciled
the same day — two seats wrote the two halves in parallel; this
document carries both whole). The three laws (§1) are the protocol
frame, written FROM the landed machinery (the N1
spec-from-working-code discipline). The consilience first slice (§2)
is LANDED: mechanic `engine/mesh`, witnesses
`engine/mesh/tests/mesh_consilience.rs`, 5 green, red-first honored.
Working orders and the three operator rulings of 2026-07-30:
`docs/N6_DIRECTIVES.md`. Supersedes the stub and
`/opt/repos/convergence-engine/DIMENSIONAL_BLOCKCHAIN.md` +
HOW_TO_USE.md — carrying independent work, exact measurement, and
disagreement preserved; dropping the retired SAIOS vocabulary and the
ancestor's majority finality (witness weight > 0.5). Consilience is
unanimous or refused, structurally — the house law the world clock
already keeps (NS-3 §1.3, 3-of-3).

The mesh is how independent engine instances hold ONE universe: not
by trusting each other, and not by mirroring each other — by
MEASURING each other. Convergence is the proof; opposition is the
check; the fold is the merge that never merges.

**The identity ruling (2026-07-30):** at N6 the mesh is blake3 and
provenance only. Consilience-of-independent-computation over
exact-rational state reduction IS the claim — no key-pair signatures
enter the registry here. External cryptographic identity arrives at
N7 (ANCHOR), where the mesh meets parties it cannot re-compute.

## 1. The three laws

1. **Proof-of-convergence, not proof-of-authority.** A node's claim
   about a world IS its chronicle. Two nodes agree when replay of the
   same receipts reaches the same committed projection — the landed
   CONVERGE-1 machinery run across a wire instead of a directory. No
   node is root; the four C's place every claim. The exact form of
   "reaches the same projection" is §2's consilience measurement,
   landed: C(Δ) = 0 exactly, or refused typed.
2. **The anti-mirror check (12_UNION §2c).** Cross-node verification
   is by OPPOSITION, never byte-mirroring: a two-legged deed verifies
   when the far leg is the exact inverse of the near one (the
   aperture pair summing zero; the council reckoning summing to the
   locks; the Delta antisymmetry). Byte-canonical encoding exists
   ONLY so digests agree; the question a node asks is never "same
   bytes as mine" — it is "is your leg my inverse." The aperture
   pair-conservation reading composes here as the N6 mesh audit
   (BUILD_ORDER §3a).
3. **One writer per clock.** A world's chronicle has ONE authoring
   node at a time (the two-writer guard, made protocol). Authority
   moves by RECEIPTED HANDOFF, never by race: the yielding node seals
   a handoff receipt naming the successor; the taking node's first
   append cites it. A fork (two chains sharing a prefix) is not a
   catastrophe — it is a DIVERGENCE: both branches live, the mirror's
   rotation law names them, and convergence annexes both (annex,
   never merge — 07 §1). The fork's exact measurement is §2.3's
   DIVERGED refusal — the residuals carried whole are what the annex
   then places. The handoff mechanic is MESH-1b's cadence.

## 2. Consilience — the landed first slice (MESH-1a)

### 2.1 The measured manifold (v1)

A node projects a world's committed state (the replay of its
chronicle, NS-3; SEAL B makes the replay the state) onto an ordered
vector of exact rationals:

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

### 2.2 The consilience triangle

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
(The triangle is the anti-mirror law in miniature: the shared-position
claim proves by the residual landing on zero, never by comparing
bytes.)

Arity ≥ 2 generalizes without new law: n nodes, one origin, the
shared-position claim pairwise zero; C(Δ) = 0 iff ALL agree.
Unanimity is the geometry.

### 2.3 The judgment

`consile(A, B)` over two intact chronicles of one world:

1. **The spine walk.** Compare receipts by own-digest from sequence
   0. The first differing sequence is the FORK.
2. **Fork found → DIVERGED.** Both chronicles replay whole (each is
   internally intact); both project (§2.1); the refusal carries the
   fork sequence, the residual list WHOLE — every nonzero
   `(dimension, B_l − A_l)` in canonical form — and C(Δ) as the
   summary scalar, never the payload (the N4 §5 law). Disagreement is
   preserved for resolution, never voted away: both branches live,
   and the annex path (§1 law 3, CONVERGE-1) places them.
3. **No fork, unequal lengths → DRIFT.** The shorter side is BEHIND;
   the refusal names the missing half-open range `[from, to)`. Drift
   is cured by carrying the tail (§3.3) — never by re-voting history.
4. **Equal spines → the measurement.** Both sides replay
   INDEPENDENTLY; the triangle closes; C(Δ) measures. Zero issues the
   consilience receipt (§3.2) recording length, head stamp, head
   digest, arity, manifold width, and the measured zero. A nonzero C
   on an equal spine would name a broken engine (SEAL B violated) and
   refuses as DIVERGED at the head — agreement is MEASURED, never
   declared.

### 2.4 The refusal taxonomy

All refusals are typed and carried whole (§3.2):

- **MALFORMED** — a receipt frame that does not parse (side, frame).
- **FORGED** — a chain that does not re-verify from its own content
  (side, first broken sequence). Verification is NS-3
  `verify_receipts`: recomputed digests, walked parent links. A
  chronicle in hand is intact by construction (the one-door law);
  forgery is caught at the boundary, in `receive`.
- **DRIFT** — same history, less of it (behind side, missing range).
- **DIVERGED** — forked history (fork sequence, residuals whole, C).

### 2.5 Scope

MESH-1a is ONE world's chronicle between two nodes (operator ruling
2026-07-30). Multi-world aggregation — the §3.4 session's
universe-span sweep — belongs to MESH-1c / N7. Consilience is a READ:
no receipt enters any world chronicle; mesh records live in the
mesh's own framing (§3), beside the world, never inside it.

## 3. The wire

NS-1 TLV frames — the house wire, never a third format (CBOR/BSON
refused on record, 2026-07-30). SEAL A carries across nodes: unknown
event tags sync and verify WITHOUT decoding (the digest law needs
bytes, not meanings). Tags 64–66, claimed in the dop.rs registry
header — the one authority. Ratios ride the canonical NS-1 carrier
(`ratio::encode`). Side codes: 1 = A, 2 = B.

### 3.1 Tag 64 — the mesh head announcement

`genesis[32] ‖ LE64(length) ‖ LE64(stamp) ‖ head[32]` — 80 bytes.

`genesis` = the own-digest of receipt 0 (the chain's identity; 32
zeros when empty); `length` = receipt count; `stamp` = the head
receipt's stamp (0 when empty); `head` = the head receipt's
own-digest (32 zeros when empty).

### 3.2 Tags 65–66 — the consilience receipt and the mesh refusal

Tag 65: `LE64(length) ‖ LE64(stamp) ‖ head[32] ‖ arity u8 ‖
LE32(dimensions) ‖ ratio(coherence)` — `dimensions` the manifold
width; `coherence` the measured C(Δ), exactly 0 by issuance law
(§2.3.4).

Tag 66: `kind u8` then per kind:

- 1 MALFORMED: `side u8 ‖ LE64(frame)`
- 2 FORGED: `side u8 ‖ LE64(at)`
- 3 DRIFT: `behind u8 ‖ LE64(from) ‖ LE64(to)`
- 4 DIVERGED: `LE64(at) ‖ LE32(residual count) ‖
  (LE32(dimension) ‖ ratio(residual))… ‖ ratio(coherence)`

The residual list is the payload, whole; C is the trailing summary.

### 3.3 The carrier (the transport ruling, 2026-07-30)

Peers exchange `chronicle.tlv` segments — the NS-3 §2.4 on-disk
framing IS the wire; there is no second encoding of history. A dumb
file/directory adapter carries whole files or appended tails
(`persist::sync` appends only what the far side lacks). The engine
never opens a socket; every adapter is logic-free (SEAL C). 100% of
the consilience logic witnesses in pure Rust, no sockets, no
flakiness.

### 3.4 The sync session (N6b — the socket adapter's shape)

When the socket adapter arrives it speaks the SAME frames, in a
session of this shape:

```
  hello        node id ‖ universe span ‖ epoch base (data frame)
  have         per world: terminal digest ‖ receipt count (tag 64)
  want         the worlds whose terminals differ
  receipts     the missing suffix, verbatim NS-3 receipts
  converge     each side runs CONVERGE-1 locally; annex/quarantine
  (no ack)     agreement is MEASURED (same terminal after annex),
               never declared — the §2.3 judgment, receipted tag 65
               or refused tag 66
```

A received suffix verifies by the chain law before anything reads it
(broken → quarantine, named, kept — 07 §1.2). The session's
universe-span sweep is MESH-1c / N7 scope (§2.5); its first cut is
one world, the landed judgment, a socket instead of a directory.

## 4. Test vectors

The consilience half is LANDED and witnessed in
`engine/mesh/tests/mesh_consilience.rs` — the four semantic vectors
plus the codec witness backing §3. The session half (§3.4) extracts
its vectors from the first two-node session, per the N1 law.

1. **Two independent replays consile at exact zero.** One session
   plays, syncs to its file; the file carries to the peer; both sides
   load through the door and replay independently; `consile` issues
   the receipt: C = 0/1 exactly, length/stamp/head matching the
   spine, arity 2, the v1 manifold width.
2. **A forged file refuses typed.** The peer's file re-frames one
   receipt with a moved stamp; the chain cannot recompute; `receive`
   refuses FORGED with the broken sequence named.
3. **A drifted peer names its range, and the tail cures it.** One
   side plays on; `consile` refuses DRIFT `[from, to)` naming the
   behind side; the tail syncs across (append-only, only the missing
   receipts); `consile` then issues the receipt at zero.
4. **Divergent histories carry their residuals whole.** A common
   prefix, then different closures on each side; `consile` refuses
   DIVERGED at the first differing sequence, the residual list naming
   each differing dimension with its exact rational difference —
   verified in the witness against the two replays' committed
   ledgers — and C(Δ) = 6·Σ R² recomputed exactly.
5. **The codec witness.** Tags 64/65/66 round-trip byte-exact (equal
   values, equal bytes; the DIVERGED residual list whole), and an
   unknown future tag skips (SEAL A).

## 5. What the mesh retires, and when

- The CLI two-writer guard relaxes into the handoff receipt (MESH-1b
  / N6b).
- The SSE forerunner (webserve /stream) yields to TLV frames pushed
  on the same socket the sync speaks.
- The epistemological want (kind 1) becomes fully real: broken
  chains IN REACH now includes the far node's quarantines.

## 6. Deferred, recorded

Transport security (this is a prototype: the mesh trusts its LAN);
node identity beyond a name (blake3/provenance only at N6 by ruling —
ties to NS-6 operator profiles and enters at N7 with ANCHOR); the
handoff under partition (the yielding node unreachable — ruled at
MESH-1b with the vectors in hand).
