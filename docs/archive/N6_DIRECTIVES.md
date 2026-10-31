# N6 Directives — the mesh working orders (2026-07-30)

> **Archived 2026-08-13.** Superseded/dead weight per the doc audit — kept for history, not live spec. See docs/BUILD_ORDER.md, docs/archive/HANDOFF_2026-08-13.md, or the doc it names as its successor for current status.

Plain language. Scope: the N6 landing (NS-5 MESH, first slice MESH-1a).
Supersedes nothing; folds into NS-5 and Pickup when N6 lands whole.

**History — this document's equations are dated, not law** (annotated
2026-08-01; text unedited per the quoted-never-rewritten rule). Three
later changes overtook it: the proportional law (2026-07-31) struck
`Σ R²` for `Σ |R|`; the zero-point correction (2026-08-01) struck
reading `C(Δ) = 0` as sameness — zero means the cocycle condition
holds, and an irreducible H¹ core survives it untouched; and MESH-1d
renamed the equal-spine zero the replication witness, reserving
"consilience" for inputs that could disagree. Standing forms:
`docs/EQUATIONS.md` C2/C3.

## Review verdict

The NS-5 stub is verified against the tree 2026-07-30:

- `protocols/NS-5_MESH.md` is 19 lines; §1 RFA semantics, §2 wire
  format, §3 test vectors all read *(unwritten)*. The stub's standing
  commitments: consilience at arity ≥ 2; receipts the shared spine;
  forged or drifted state refuses typed; no central server, no global
  transaction log; first consumer two-node world synchronization.
- Both predecessors exist at `/opt/repos/convergence-engine/`
  (DIMENSIONAL_BLOCKCHAIN.md — the two-chain shape; HOW_TO_USE.md —
  two laptops, sovereign bubbles, the membrane). Carried: independent
  work, exact measurement, disagreement preserved never voted away.
  Dropped: the SAIOS vocabulary (retired at NS-6) and Level finality at
  witness weight > 0.5 — majority is not the house law. The clock
  already walks by 3-of-3 unanimity (`vrp.rs`); consilience is
  unanimous or refused, structurally.
- The predecessor lore correction stands (BUILD_ORDER §1a pin): coherence
  is sum-of-squares in code, not L1; the reference vector measures
  C = 24, not 8. NS-5's vectors come from Netstratum's code, never the
  predecessor's prose.
- Tag registry: the next free tag is **64** (dop.rs header, the single
  authority — Pickup §4's "next is 59" note is stale; 60–63 landed as
  GaugeShiftedAt, GenesisSealed, WantStruck, WantRelieved). NS-2's
  56–58 set the precedent NS-5 follows: protocol records in the
  protocol's own framing, never world-chronicle events.
- The machinery is ready: `vrp::verify_receipts` is public and names
  the first break; `persist::load` is the only file loader (Broken,
  Malformed, HistoryDiverged refusals landed); `world::replay` is
  public and yields the whole `Committed` projection; `rfa-core`
  carries `Delta`, `coherence_functional`, and the residual iterator,
  exact throughout.
- `rfa-core` depends on `cfe` (the ratio carrier), so the mesh
  mechanic cannot live in `cfe` without a cycle. It is a new crate,
  `engine/mesh`, depending on both — one module per protocol seam.

## Decisions

1. **Identity: blake3/provenance only at N6.** No ed25519, no key-pair
   dependencies enter the wire registry at MESH-1a.
   Consilience-of-independent-computation over exact-rational state
   reduction is the claim of N6. Signature wrappers wait for N7, where
   external cryptographic identity enters the protocol.
2. **Transport: file exchange first.** A passive reader exchanging
   `chronicle.tlv` segments between directories satisfies SEAL C whole
   and lets 100% of the consilience logic and vectors witness in pure
   Rust — no sockets, no flakiness. TCP is a later, logic-free adapter.
3. **Scope: one world.** MESH-1a is two independent engine instances
   converging on one world's chronicle. Universal multi-world
   aggregation belongs to MESH-1c / N7.
4. **Partition: the spine stalls; the escape is an open
   fork.** Authority never moves without a seal — an unreachable
   writer leaves its spine read-only (writes refuse usurped; reads
   and consilience continue). A stranded successor continues only on
   an explicit Diverged branch (tags 43/44, the landed vocabulary);
   healing measures the branches — exact zero confluences, nonzero
   stands preserved as typed divergence for the annex path. No votes,
   no elections, no silent mutiny. NS-5 §7.5 carries the law whole.

## The consilience construction (NS-5 §1, summarized)

Two nodes independently replay one world's chronicle and project the
committed state onto the v1 measured manifold: the graded gauge key
(grade-ascending) then, per orb ascending by orb id, the banked ledger
over resource codes 1–7. The union of grades and orbs spans the
manifold; an absent coordinate reads exactly 0.

The claim "two paths, one position" is a triangle: a 3-node Delta with
an origin O, Δ(O→A) = node A's projection, Δ(O→B) = node B's
projection, Δ(A→B) = 0. The cocycle residual over the canonical triple
is R = B − A per coordinate; `coherence_functional` sums the six
ordered triples, so ~~C(Δ) = 6·ΣR²~~ **C(Δ) = 6·Σ|R|** — exact, and zero
iff every residual is zero. Consilience is C(Δ) = 0 exactly; anything
else refuses typed
with the residual list carried whole (N4 directive 5: C is the summary
scalar, never the payload). Arity ≥ 2 generalizes: n nodes, one shared
position claimed pairwise; unanimity is the geometry, not a vote.

**Corrected 2026-08-02 (`22_CONTAINMENT` §2n).** The formula above read
`ΣR²`; the code sums absolute values (`delta.rs:193`, `c += r.abs()`)
and the proportional law bars squares as aggregators of difference.
The multiplier 6 and the zero-iff claim are unaffected — `Σ|R| = 0` iff
every residual is zero exactly as `ΣR²` would be. The surrounding
argument therefore stands; only the aggregator was misstated. Note
also that `C` is lens-only and gates nothing, which the parenthetical
("C is the summary scalar, never the payload") already anticipated.

## Directives

1. **Worktree.** Main holds in-flight uncommitted work
   (`clients/cli/src/main.rs`, `engine/cfe/src/world/`,
   `engine/conductor/src/lib.rs`). MESH-1a works in
   `.claude/worktrees/mesh-n6`, branch `mesh/n6`, off main; merge only
   on green: build, full suite, clippy zero. This landing never edits
   `cfe::world` — `replay` is already public.
2. **One landing.** Spec + mechanic + vectors together, red-first:
   NS-5 §1/§2/§3, the `engine/mesh` crate, the dop.rs tag header, and
   the witnesses land as one unit.
3. **Tags 64–66 claimed** (dop.rs header, first commit, before code):
   64 mesh head announcement · 65 consilience receipt · 66 mesh
   refusal. All three are NS-5 protocol records in NS-5's own framing —
   they never enter the world chronicle. The measurement is a read;
   no world state changes; the lens law in mesh form.
4. **The mechanic.** `engine/mesh`: `receive(bytes, side)` — the
   ingestion door (frame parse + chain verification; a tampered file
   refuses Forged with its break named); `consile(a, b)` →
   `ConsilienceReceipt | MeshRefused` (Drift names the missing range;
   Diverged names the fork and carries the residuals whole);
   `head(chronicle)` — the announcement. Wire codecs for 64/65/66
   round-trip byte-exact and honor SEAL A.
5. **Exactness.** No floats anywhere. Residuals are exact rationals in
   canonical form; C(Δ) carries whole.
6. **The four vectors** (`engine/mesh/tests/mesh_consilience.rs`):
   (a) two independent replays of one spine consile at exactly zero;
   (b) a forged file refuses typed, break named; (c) a drifted peer
   refuses with its missing range, and the tail exchange cures it;
   (d) divergent histories refuse with the fork sequence and the exact
   residual list. Plus the codec witness backing §2.
7. **After landing.** Refresh the Pickup §4 registry note (64–66
   claimed, next free 67) and one lane line. Nothing else in Pickup.

## Flagged, out of scope

- `engine/rfa-core/Cargo.toml`'s description contains a set-letter
  glyph ("exact ℚ") — a Charter §8 terminology-law violation predating
  this landing. One-line cure, its own commit, not MESH-1a's.
