# NS-5 — MESH: two nodes, one world

> **Superseded in part — Charter §11, the ONE-TRUTH amendment
> (2026-07-31).** The consilience instruments of §2 and the
> reconciliation records of §3 (tag 65; refusal kinds 3–4) are
> retired and their code excised: a reconciling check between copies
> is unlawful. The lawful remainder of this document: the carrier
> (§3.3), the receipted handoff (§7), and the partition law (§7.5).
> The spine at TUniversal{S,G,T} replaces replication whole — state
> is position, the kernel is motion, the chain is the path — and this
> document rewrites entirely when the spine lands (the constitution
> ladder, #34–#37). Nothing in the superseded sections may be built
> against.

**Status:** N6a specced + MESH-1a landed (both 2026-07-30, reconciled
the same day — two seats wrote the two halves in parallel; this
document carries both whole) + MESH-1b landing (§7 — the receipted
handoff, tag 67).

> **Spine amendment (2026-07-31, arch-02).** The spine landed and
> walks: SPINE-1 (tag 84, two maps, four verbs) + the #36 rungs — the
> adoption door (`Spine::adopt`: a pre-chain world enters the mainline
> once, the debt stamped `judged=0`, a second adoption refuses) and
> KERNEL-OBSERVED play (every conductor world chains its committed
> state each beat at {S, G, T-tick}; the record repeats the blob's
> leading byte — never a schema constant; a penned world's spine
> stalls with its pen). The agreement law landed whole as
> `mesh::agree` (Σ_mesh · re-sync · resonance m=10 — the ancestor's
> majority finality stays dropped, per this document's own law).
> The code cites "NS-5 §9" for the spine's chapter; the full rewrite
> this header promises (spine as replication whole) is owed and lands
> with the spine uplink (present/receive for commitment segments). The three laws (§1) are the protocol frame, written
from the landed machinery (the N1 spec-from-working-code discipline).
The consilience first slice (§2) is landed: mechanic `engine/mesh`,
witnesses `engine/mesh/tests/mesh_consilience.rs`, 5 green, red-first
honored.
Working orders and the three operator rulings of 2026-07-30:
`docs/N6_DIRECTIVES.md`. Supersedes the stub and
`/opt/repos/convergence-engine/DIMENSIONAL_BLOCKCHAIN.md` +
HOW_TO_USE.md — carrying independent work, exact measurement, and
disagreement preserved; dropping the retired SAIOS vocabulary and the
ancestor's majority finality (witness weight > 0.5). Consilience is
unanimous or refused, structurally — the house law the world clock
already keeps (NS-3 §1.3, 3-of-3).

The mesh is how independent engine instances hold one universe: not
by trusting each other, and not by copying each other — by
measuring each other. Convergence is the proof; opposition is the
check; the fold is the merge that never merges.

**The identity ruling (2026-07-30):** at N6 the mesh is blake3 and
provenance only. Consilience-of-independent-computation over
exact-rational state reduction is the claim — no key-pair signatures
enter the registry here. External cryptographic identity arrives at
N7 (anchor), where the mesh meets parties it cannot re-compute.

## 1. The three laws

1. **Proof-of-convergence, not proof-of-authority.** A node's claim
   about a world is its chronicle. Two nodes agree when replay of the
   same receipts reaches the same committed projection — the landed
   CONVERGE-1 machinery run across a wire instead of a directory. No
   node is root; the four C's place every claim. The exact form of
   "reaches the same projection" is §2's consilience measurement,
   landed: C(Δ) = 0 exactly, or refused typed.
2. **The opposition check (12_UNION §2c).** Cross-node verification
   is by opposition, never byte-copying: a two-legged deed verifies
   when the far leg is the exact inverse of the near one (the
   aperture pair summing zero; the council reckoning summing to the
   locks; the Delta antisymmetry). Byte-canonical encoding exists
   only so digests agree; the question a node asks is never whether
   the bytes match its own — it is whether the far leg is the inverse
   of its own. The aperture
   pair-conservation reading composes here as the N6 mesh audit
   (BUILD_ORDER §3a).
3. **One writer per clock.** A world's chronicle has one authoring
   node at a time (the two-writer guard, made protocol). Authority
   moves by receipted Handoff, never by race: the yielding node seals
   a handoff receipt naming the successor; the taking node's first
   append cites it. A fork (two chains sharing a prefix) is not a
   failure — it is a divergence: both branches live, the ledger's
   rotation law names them, and convergence annexes both (annex,
   never merge — 07 §1). The fork's exact measurement is §2.3's
   diverged refusal — the residuals carried whole are what the annex
   then places. The handoff mechanic is §7, landed at MESH-1b.

## 2. Consilience — the landed first slice (MESH-1a)

### 2.1 The measured manifold (v2) and the structural census — struck

> **The census kernel is out** (operator ruling 2026-08-04):
> incompatible with the architecture (paraphrased: the build stands on a
> higher dimensional plane of multi-axial PoW++ with two algorithms, PoUW
> and PoWC, both converging on n-dimensional polynomials).
>
> **The code excised it four days before this text was struck.**
> `engine/mesh/src/lib.rs` records it: the reconciliation instruments —
> consilience, the manifold, the census, the triangle, tag 65 and
> refusal kinds 3–4 — went under the one-truth amendment on
> 2026-07-31. This section is the spec that outlived them, still
> carrying a wire format (§4's `LE32(census count) ‖ (census u8 ‖
> LE64(id))…`) for an instrument no code has emitted since.
>
> **Two independent reasons, and the second is the general one.** The
> one-truth amendment struck it because *what cannot diverge needs
> no reconciliation, and agreement between copies was never a
> measurement*. The ruling above strikes it again for its shape: a
> census is a flat enumeration — committed fields sorted into
> seventeen named families plus a tripwire code — and an enumeration
> cannot describe a state whose capacity is a polynomial fiber that
> grows. It is the same 2D construct as the Merkle corpus root, struck
> the same day for the same reason: enumerations fix what the
> architecture is built to let expand.
>
> **What replaces it is not a better census.** Equality between two
> states is not established by listing their fields; the two algorithms
> decide on structure — PoWC on whether a difference is genuine or
> gauge, PoUW on what verified work has earned — and both read the
> polynomial, never a roster.
>
> **Not struck, and the distinction is load-bearing:**
> `rfa_core::capacity::flux` calls its return an *egress census* —
> `Flux { sources, sinks, balanced }`, the count of divergence poles.
> That is a reading in the D/V algebra and is unrelated to this
> instrument beyond the shared word. A sweep on the word alone would
> dismantle the capacity meter, exactly as a sweep on "vacuity" would have
> dismantled the circulation it was built on.

The text below is kept as the record of what was specified, and is
**not implementable law**.

The v1 8-dimension projection was a keyhole (the coder-04 audit,
2026-07-31): two states differing only in structure measured C = 0
with an empty residual list — a difference the instrument could not
see. v2 closes it with one law:

> **The totality law.** Two committed states are equal IFF the
> residual list and the census are both empty. Every committed field
> is either a rational dimension (measured, residuals whole) or a
> censused structure (compared whole, exactly, named when it
> differs). Nothing is invisible.

**The rational dimensions**, enumerated per judgment over the union
span of the two states (an absent coordinate reads exactly 0):

1. the graded gauge key, grade-ascending (grade 0 first);
2. per orb ascending by orb id: the banked ledger over resource codes
   1–7 in code order; cumulative vented entropy; the eight ideology
   books in the inscribed order (given_in_crisis, drawn_in_crisis,
   returned_across, extracted_across, bond_flow, severance_load,
   forwards_fulfilled, forwards_breached); then the element stores
   over the union of that orb's element ids, id-ascending.

**The census**, every non-rational committed field compared whole by
exact equality, named typed when it differs:

- per orb (union-spanned): the orb with its rational fields set aside
  — name, zones, famine, thermal siting, overheat, last_touch — and
  an orb present on one side only censuses by its existence;
- the world families, each whole: entities · star · debts · bonds ·
  covenants · councils · offers · techs · wants · crossed · genesis ·
  terrain · instruments · practices · prospects · warded · veiled.

A state difference that neither list carries reports **unmeasured**
(census code 0) — the tripwire: a future committed field that joined
neither the manifold nor the census is visible, never silent. In
code the tripwire is the committed state's own exact equality.

Diverged spines whose residuals and census are both empty are two
histories arriving at one state — the judgment still refuses (the
spine forked; one writer per clock), but the measurement truthfully
reads zero difference. That is the confluence precondition the annex
path consumes.

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
`C(Δ) = 0` exactly. There is no threshold, no tolerance, no vote.
(The triangle is the opposition law in miniature: the shared-position
claim proves by the residual landing on zero, never by comparing
bytes.)

Arity ≥ 2 generalizes without new law: n nodes, one origin, the
shared-position claim pairwise zero; C(Δ) = 0 iff all agree.
Unanimity is the geometry.

### 2.3 The judgment

`consile(A, B)` over two intact chronicles of one world:

1. **The spine walk.** Compare receipts by own-digest from sequence
   0. The first differing sequence is the fork.
2. **Fork found → diverged.** Both chronicles replay whole (each is
   internally intact); both project (§2.1); the refusal carries the
   fork sequence, the residual list whole — every nonzero
   `(dimension, B_l − A_l)` in canonical form — and C(Δ) as the
   summary scalar, never the payload (the N4 §5 law). Disagreement is
   preserved for resolution, never voted away: both branches live,
   and the annex path (§1 law 3, CONVERGE-1) places them.
3. **No fork, unequal lengths → drift.** The shorter side is behind;
   the refusal names the missing half-open range `[from, to)`. Drift
   is cured by carrying the tail (§3.3) — never by re-voting history.
4. **Equal spines → the replication witness.** Both sides replay
   independently; the triangle closes; C(Δ) measures; zero issues the
   tag-65 receipt. Named honestly (the coder-04 audit, ruled into the
   text 2026-07-31): on an equal spine the zero witnesses byte-exact
   replication and independent replay determinism (SEAL B) — it is
   not convergence, because the inputs could not disagree.
   "Consilience" is reserved for measurements whose inputs could:
   a diverged pair (§2.3.2), an opposition reading (§1 law 2, the
   cfe-side instrument), a confluence of branches that did
   independent work (§7.5). A nonzero C on an equal spine would name
   a broken engine and refuses as diverged at the head — agreement
   is measured, never declared.

### 2.4 The refusal taxonomy

All refusals are typed and carried whole (§3.2):

- **Malformed** — a receipt frame that does not parse (side, frame).
- **Forged** — a chain that does not re-verify from its own content
  (side, first broken sequence). Verification is NS-3
  `verify_receipts`: recomputed digests, walked parent links. A
  chronicle in hand is intact by construction (the one-door law);
  forgery is caught at the boundary, in `receive`.
- **Drift** — same history, less of it (behind side, missing range).
- **Diverged** — forked history: fork sequence, residuals whole, C,
  and the census (§2.1) — every structural difference named typed.

### 2.5 Scope

MESH-1a is one world's chronicle between two nodes (operator ruling
2026-07-30). Multi-world aggregation — the §3.4 session's
universe-span sweep — belongs to MESH-1c / N7. Consilience is a read:
no receipt enters any world chronicle; mesh records live in the
mesh's own framing (§3), beside the world, never inside it.

## 3. The wire

NS-1 TLV frames — the house wire, never a third format (CBOR/BSON
refused on record, 2026-07-30). SEAL A carries across nodes: unknown
event tags sync and verify without decoding (the digest law needs
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

- 1 malformed: `side u8 ‖ LE64(frame)`
- 2 forged: `side u8 ‖ LE64(at)`
- 3 drift: `behind u8 ‖ LE64(from) ‖ LE64(to)`
- 4 diverged: `LE64(at) ‖ LE32(residual count) ‖
  (LE32(dimension) ‖ ratio(residual))… ‖ ratio(coherence) ‖
  LE32(census count) ‖ (census u8 ‖ LE64(id))…` — the census trails
  the coherence additively (MESH-1d, 2026-07-31): the landed decoder
  ignored trailing bytes, so a pre-census record decodes census-empty
  and the record's meaning never moved. Census codes: 0 unmeasured ·
  1 orb (id) · 2 entities · 3 star · 4 debts · 5 bonds · 6 covenants
  · 7 councils · 8 offers · 9 techs · 10 wants · 11 crossed ·
  12 genesis · 13 terrain · 14 instruments · 15 practices ·
  16 prospects · 17 warded · 18 veiled (id is 0 except code 1).
- 5 usurped: `LE64(at) ‖ LE64(lawful)` (MESH-1b, §7.2)
- 6 uncited: `LE64(at) ‖ LE64(handoff)` (MESH-1b, §7.2)

The residual list is the payload, whole; C is the trailing summary.
Kinds are additive (the SEAL A instinct): a reader meeting a kind it
does not know skips the record, never crashes.

### 3.3 The carrier (the transport ruling, 2026-07-30)

Peers exchange `chronicle.tlv` segments — the NS-3 §2.4 on-disk
framing is the wire; there is no second encoding of history. A passive
file/directory adapter carries whole files or appended tails
(`persist::sync` appends only what the far side lacks). The engine
never opens a socket; every adapter is logic-free (SEAL C). 100% of
the consilience logic witnesses in pure Rust, no sockets, no
flakiness.

### 3.4 The sync session (N6b — the socket adapter's shape)

When the socket adapter arrives it speaks the same frames, in a
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

The consilience half is landed and witnessed in
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
   refuses forged with the broken sequence named.
3. **A drifted peer names its range, and the tail cures it.** One
   side plays on; `consile` refuses drift `[from, to)` naming the
   behind side; the tail syncs across (append-only, only the missing
   receipts); `consile` then issues the receipt at zero.
4. **Divergent histories carry their residuals whole.** A common
   prefix, then different closures on each side; `consile` refuses
   diverged at the first differing sequence, the residual list naming
   each differing dimension with its exact rational difference —
   verified in the witness against the two replays' committed
   ledgers — and C(Δ) = 6·Σ R² recomputed exactly.
5. **The codec witness.** Tags 64/65/66 round-trip byte-exact (equal
   values, equal bytes; the diverged residual list whole), and an
   unknown future tag skips (SEAL A).

## 5. What the mesh retires, and when

- The CLI two-writer guard relaxes into the handoff receipt (MESH-1b
  / N6b).
- The SSE forerunner (webserve /stream) yields to TLV frames pushed
  on the same socket the sync speaks.
- The epistemological want (kind 1) becomes fully real: broken
  chains in reach now includes the far node's quarantines.

## 6. Deferred, recorded

Transport security (this is a prototype: the mesh trusts its LAN);
node identity beyond a name (blake3/provenance only at N6 by ruling —
ties to NS-6 operator profiles and enters at N7 with anchor). The
handoff under partition was ruled 2026-07-31 — §7.5 carries the law.

## 7. The receipted handoff (MESH-1b)

### 7.1 The law made mechanic

Tag 67, `ClosureEvent::HandoffSealed { from, to }` — a world-chronicle
deed, unlike consilience: authority changing hands is a state change
of the protocol layer, so it receipts (the lens law draws the line —
measurement reads, handoff closes). Wire: `LE64(from) ‖ LE64(to)`.
The world's own mechanics are blind to it: replay ignores the seal
(the game rules never consult the writing hand), and the walk below
is the mesh's reading, never a flag.

Authority is a walk of the spine:

- Before the first seal a chronicle is single-handed by construction
  — the pre-mesh era. The walk reads None; every append is lawful.
  The law activates at the first seal, so every landed world stays
  lawful (additive, the SEAL A instinct).
- The first `HandoffSealed` self-declares the incumbent: its `from`
  is recorded, not consulted (nothing earlier could attest it — the
  provenance law). Authority becomes `to`.
- After a lawful seal at sequence h: every subsequent receipt must
  carry the authority in its provenance (the forging path names the
  hand — one writer per clock), and the first subsequent receipt must
  also cite h in its provenance (the taking hand acknowledges where
  its pen came from). A seal standing last in the chain is lawful
  with its citation outstanding — judged when the next receipt
  exists.
- A seal whose `from` is not the authority refuses usurped at its own
  sequence, like any other unlawful append.

### 7.2 The refusals (kinds 5–6 on tag 66, additive)

- **Usurped { at, lawful }** — the receipt at `at` does not carry the
  lawful hand. One writer per clock, broken; the lawful hand named.
- **Uncited { at, handoff }** — the taking hand's first append does
  not cite the seal it took the pen from.

### 7.3 The doors

- `authority(chronicle)` — the walk, a pure read: `None` pre-mesh,
  the hand after seals, or the first violation refused typed.
- `yield_authority(chronicle, from, to, stamp)` — checks the walk,
  then seals with provenance `[from]` (plus the pending citation when
  the seal is itself the taking hand's first append). A stale hand
  refuses before anything appends.
- `append_as(chronicle, hand, event, stamp, provenance)` — the
  guarded write door: refuses usurped before writing; injects the
  hand at the provenance head and the citation when one is pending.
  The conductor and the CLI wire these at the consumer slice (the
  two-writer guard's retirement).

### 7.4 Vectors (mesh_handoff.rs)

1. **The handoff walks.** Pre-mesh None; the first seal
   self-declares; the cited first append; a second seal by the new
   hand; and the walk derives identically after a save/load round
   trip (SEAL B — authority survives the file).
2. **An uncited first append refuses** with the append and the seal
   both named.
3. **A stale hand refuses** usurped with the lawful hand named.
4. **A stale seal refuses at the door** — and the chronicle is
   unchanged (the refusal happens before the append).
5. **The codec.** Tag 67 rides the consolidated NS-1 wire witness
   (`dop_wire.rs`); kinds 5–6 round-trip byte-exact.

### 7.5 The consumer slice (#2, landed 2026-07-30 — coder-01)

**The hands.** The conductor writes as hand **1** (`cfe::vrp::
CONDUCTOR_HAND`). Every other seat derives its hand from its name:
`hand_of(name)` = the first 8 bytes of the name's digest, LE, with
the reserved values (0, 1) remapping to 2. No hand registry file —
the name is the registry.

**The funnel.** The chronicle itself holds the pen (`set_hand`, a
workspace field — never serialized; the receipts' provenance is the
record). While held, every append stamps the hand at the provenance
head and cites a seal standing last — so the ~20 engine deeds carry
the hand through one point, and the walk stays the only judge.
Pre-stamped provenance (the §7.3 doors) and unheld chronicles pass
untouched.

**The conductor's door** (`pen_service`, each beat boundary): a
`yield <world> <seat>` line in `conductor.ctl` asks; the seal lands
between beats (no beat is mid-flight), the line is consumed, and the
world is penned — beats, saves, journals, and the market all wait.
The file is then polled: when the walk on the carrier reads hand 1
again, the conductor re-resumes from the receipts (SEAL B) and
beats continue, the taking hand's deeds kept.

**The CLI's side.** `pen <world>` writes the ask; the walk-checked
`far_world` then loads, holds the pen, and cross-world deeds write
directly; `pen return <world>` yields back. The old founder-heuristic
guard is retired — a pre-mesh conductor world still refuses until
its first seal, but the refusal now names the door.

**Witness.** `pen_pins.rs::the_pen_passes_writes_and_returns` — the
full arc on one file, the walk green at every station.


### 7.5 The partition law (operator ruling 2026-07-31)

Authority on one spine never moves without its seal. An unreachable
writer stalls the spine: writes refuse (usurped, the lawful hand
named); reads, queries, and consilience measurements continue
normally. There is no silent mutiny — a successor assuming an
unsealed pen would put two hands inside one blake3 lineage,
non-deterministic split-brain, SEAL B broken.

The named escape is the fork the laws already bless (§1 law 3): a
stranded successor never appends to the primary spine — it diverges,
openly. A `Diverged { branch, gauge }` record (tags 43/44, the landed
vocabulary — never a parallel mesh concept) opens the node's own
branch; local writes accrue there. When the partition heals, the
branches do not vote on who was right — they measure (§2): exact-zero
distance confluences (`Confluent { partner, gauge }`); nonzero
distance stands preserved as typed divergence for the annex path —
never discarded, never overwritten.

### 7.6 The sandboxed nursery (operator ruling 2026-07-31)

**Open divergence is lawful; covert divergence is slashed.** A
declared fork is a nursery: a sandboxed branch under its own branch
coordinates, where a node may play, experiment, and diverge without
limit — and without consequence to the mainline, because the
mainline's coordinates cannot be written from a branch. One value per
address, always: the galaxy is structurally rip-proof from inside any
nursery, however immature the play.

NPCs and conductors track a nursery knowingly — branch coordinates
are addressable, so an polity can follow a divergence and the
mainline as two named paths and never decohere on re-convergence:
following was never a second claim on one address.

Re-convergence has one door: the kernel judges the merge as a lawful
transition at a mainline coordinate, or the nursery's content stays
in the nursery. Penalty attaches only to presenting nursery state as
mainline alignment (the alignment covenant: the pen dies, the work
dies, the stake dies). The lie is punished, never the disagreement.
Play in a nursery is free; no nursery can tear a hole in the galaxy
from inside it.

## 8. The bridge entity (ruled 2026-07-31 — spec, lands with MESH-1c)

Cross-world communication is a lawful, scope-bounded boundary — never
an un-sandboxed socket. At genesis every world instantiates its own
IPSAL bridge entity (`BridgeGenesis { bridge_id, scopes,
generation_tag }` — tag claimed at the landing cadence). The MESH-1c
TCP adapter is passive transport only (SEAL C); every remote handshake
and cross-world interaction terminates at the bridge, which enforces
containment: read and consile open by default; yield/handoff require
explicit grant receipts. The bridge is the world's one comms door
into the shared exchange — the per-world uplink the operator named.
Seeding is conductor-genesis work (coder-01); the session handshake
naming the bridge is adapter work (coder-03). One seam, two sides.

## 9. The spine (SPINE-1, landed 2026-07-31 — the ladder's #36 spine half)

The chain bound to the real clock. State is position; the kernel is
motion; the spine is the recorded path — two maps, four verbs, one
record, mechanized in `engine/mesh::spine`.

### 9.1 The coordinate and the record (tag 84)

A position on the one clock: `{branch ‖ S ‖ G ‖ T}` — `branch` the
nursery name ("" is the mainline, §7.6), `S` the wrapped torus triple
(three ratios; zeros until charted), `G` the gauge sheet (a wrapped
ratio), `T` the attested tick. The spine commitment record binds one
state to one position, forever:

    commit = BLAKE3( branch ‖ S ‖ G ‖ T ‖ schema ‖ kernel ‖ judged ‖
                     state_digest ‖ parent )

`schema` is the codec version that shaped the bytes; `kernel` the
judge version standing when the step was cut; `judged` the admission
honesty flag (§9.3); `state_digest` = BLAKE3 of the canonical state
bytes (the store key); `parent` the previous commitment on this
branch (32 zeros at mainline genesis). Wire: one TLV record, tag 84,
value = the hashed fields ‖ `commit[32]`; a record whose commit does
not recompute refuses on load.

### 9.2 The two maps and the four verbs

- **The spine map** — coordinate → commitment. Single-valued,
  append-only, derived from the record stream (`spine.tlv`); T
  strictly ascends per branch (the arrow — the clock only walks
  forward), so one value per address is structural, and a duplicate
  or backward record refuses the whole file on open.
- **The store** — state_digest → canonical bytes, content-addressed
  files. A store entry cannot be wrong, only absent; `fetch`
  re-hashes on every read and a tampered blob refuses forged.

The verbs: `resolve(coordinate)` — where the universe stands;
`fetch(state_digest)` — the geometry, verified on arrival;
`extend(coordinate, state_bytes, step?)` — the one write, §9.3;
`branch(from, name)` — the nursery opens: a genesis-copy record at
the fork position under the new branch name, parent-linked to the
mainline commitment (the same state entering a new address space —
the mainline is unwritable from inside it, §7.6).

### 9.3 The judged flag (admission honesty)

`extend` with a `Step` consults the kernel across the seam:
`kernel::lawful(prior, next, step)` — an `Unlawful` refusal (including
`Unjudged`, the sealed default) refuses the extend whole, and the
landed commitment carries `judged = 1`. `extend` without a step is
lawful at this phase and carries `judged = 0` — the coverage hole
stamped into the record, never hidden (a genesis extend is always
unjudged: nothing prior exists to judge from; a step offered at
genesis refuses). When kernel coverage completes (Phase 3), unjudged
extends retire by amendment. The kernel is consumed, never owned:
the spine binds `kernel::VERSION` into every commitment; the judge
never reads the spine.

### 9.4 What replicates

The spine is the uplink's cargo (the uplink law): peers exchange
record streams and store blobs — both self-verifying — and nothing
else. Drift is being behind on one branch (fetch the tail);
divergence is a declared branch (its own addresses); a second value
at an occupied address is unregisterable. Replication of the spine
is #18's transport work; nothing in this section opens a socket.
