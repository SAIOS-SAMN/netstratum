# NETSTRATUM — THE CHARTER

**The one founding document. Read first, whole. Everything else in this repo is
downstream of what is inscribed here.**

Netstratum is an AI MUD — a text-first, multi-planetary city-building and
exploration game in the lineage of Cities:Skylines (the intuitive joy of zoning,
growth, and trade) and Dark Forest (the decentralized, hidden-information
cosmos) — that is simultaneously the **playable reference implementation of the
dimensional web**: the NS protocol stack (DOP, RQP, VRP, APERTURE, MESH, IPSAL-OS,
ANCHOR) specified in Rational Field Algebra (RFA) and executed by the Canonical
Field Engine (CFE).

The game is the front door. The protocols are the building. The engine is the
ground. One repo holds all three so they cannot drift apart.

---

## 1. THE CORE MANDATE (LAW XVII, inherited and binding)

> **A protocol section may only be written when its first game consumer is
> named, and a game mechanic may only ship when its protocol section and test
> vectors exist.**

No spec without a consumer. No mechanic without a spec. The shared test vectors
are the joint: fixtures written in the protocol document are consumed verbatim
by the engine's conformance tests. A spec that cannot be executed is prose; a
mechanic that cannot be verified is a bug that hasn't landed yet.

## 2. THE FUN-FIRST SUBSTRATE RULE

RFA determinism, TLV codecs, receipts, and the refused-functionals
discipline are **invisible infrastructure**. The player sees zoning,
exploration, trade, fleets, and other citizens of the stratum — never the math.
Rigor serves play the way a traffic solver serves a city builder: absolutely,
and silently. When a design choice pits mathematical elegance against player
experience, the spec bends and the substrate absorbs the complexity. The
substrate's law ("valid or rejected, nothing in between") governs state
transitions, not game design.

## 3. THE INSCRIBED CONSTRAINTS (the Dark Forest post-mortem, as law)

Dark Forest (0xPARC, ~2020) proved decentralized hidden-information worlds are
possible and then demonstrated five ways they fail. Each failure is inscribed
here as a standing constraint:

1. **PERSISTENCE BY CONSTRUCTION.** The append-only VRP chronicle IS the world
   state. There are no resets, no wipes, no ephemeral beta rounds — a reset is
   not a policy we refuse but an operation that does not exist.
2. **ZERO FLOATING-POINT TAX.** All state transitions are native RFA (exact
   rational arithmetic). No
   float emulation, no constraint blowup, no thermal throttling. If state is
   ever bridged to ZK proofs, rational arithmetic keeps circuits light.
3. **MESH CONSENSUS, NO CHOKEPOINT.** Peers verify each other's state by exact
   measurement (proof-of-convergence). There is no central server to melt and
   no global RPC to crash.
4. **GOVERNED AUTONOMOUS AGENTS.** This is an AI MUD: bots are not cheating,
   they are citizens. Every agent — player-written or NPC — runs inside
   ipsal-os containment with dimensional permissions and generation-tagged
   receipts. Automation is governed, never banned and never an arms race.
5. **INSTANT ONBOARDING.** One command launches the CLI client. No wallet, no
   gas, no invite keys, no bridging. The barrier to entry is typing.

## 4. THE PROTOCOL-TO-MECHANIC BINDING MAP

| Spec | Protocol | First game consumer |
|---|---|---|
| NS-1 | **DOP** — Dimensional Object Protocol | Colony, sector, and profile orbs — every game object is a self-contained orb with provenance |
| NS-2 | **RQP** — Resonance Query Protocol | The scanner: search by structural resonance, not string match |
| NS-3 | **VRP** — Verification Receipt Protocol | The world chronicle: every action an append-only, parent-linked receipt |
| NS-4 | **APERTURE** — the controlled opening | Sector-map sharing and private trade offers, opened at a granted facet (summary → detail → full) |
| NS-5 | **MESH** — Proof-of-Convergence Consensus | Multi-node world synchronization ("two laptops, one world") |
| NS-6 | **IPSAL-OS** — Entity Containment | Player, NPC, and bot identity: the profile of every individual inside our internet |
| NS-7 | **ANCHOR** — WWW3 Settlement Bridge | High-value settlement: minting a rare world, macro-trades, asset ownership anchored to an external chain |

## 5. ANCESTRY (consult, never port blindly)

- `/opt/repos/` — the reference era: convergence-engine (the math),
  AI-TECHNOLOGY-WWW (the WWW4 protocol drafts the NS specs supersede),
  GOVERNANCE (the five-layer architecture), NEBULA-PRODUCTS (the platform
  vision), AI-SCIENTISTS (the safety research). Shapes are reference; every
  scalar/scored/capped form is refused.
- `/opt/ipsal-autic-v0.4/` — the sealed engine build: the chronicle (VRP's
  working ancestor), the sigil index (RQP's), consilience (MESH's), the TLV
  wire, and `docs/GROUND.md`'s organ map — the DO-NOT-REBUILD gate.
- **Naming:** SAIOS is retired (external name collision). The sovereign layer
  is **ipsal-os**, here and everywhere forward.

## 6. GOVERNANCE

`docs/BUILD_ORDER.md` is the ONE carrier of build order — the current landing
is always read there, never inferred. Landings are red-first with recorded
output; one landing at a time; evidence blocks verbatim; the chronicle of the
repo is its git history, committed only on explicit operator go.

## 7. THE WWW3 BRIDGE (settlement, never simulation)

WWW3 (existing EVM/UTXO blockchains) and WWW4 (the NS mesh) are not rivals;
WWW4 is the glue between them. The scope rule is absolute:

> **Game logic lives in the NS specs (RFA/CFE). WWW3 chains are strictly
> external settlement layers — temporal anchors, asset ownership, liquidity.
> No game rule is ever encoded in contract bytecode.**

**Dual anchoring** (the architecture that avoids Dark Forest's every-tick-
on-chain mistake):

1. **High-frequency local simulation** — CFE state ticks run locally at full
   speed; every transition emits an NS-3 receipt. No gas, no RPC.
2. **Mesh consensus** — peers converge on state via NS-5 proof-of-convergence.
   Multiplayer sync happens HERE, chain-free.
3. **Periodic anchoring (NS-7)** — at defined intervals or on high-value
   events, the mesh computes one consensus root of the chronicle and posts
   that single lightweight root to a WWW3 chain. The chain stores checkpoints,
   never ticks. A WWW3 fork, upgrade, or outage cannot pause or break the
   running world.

**The three safety seals** (upgrades must never break state):

- **SEAL A — codec compatibility.** The TLV wire format is additive: a reader
  skips unknown tags, never crashes on them; landed state round-trips
  byte-exact across every engine version.
- **SEAL B — the chronicle is ground truth.** The world lives in the
  append-only NS-3 chronicle, not in any mutable database or contract
  variable. Swap the renderer, upgrade the engine, migrate the chain — the
  chronicle replays the universe from tick zero.
- **SEAL C — adapters, not logic.** WWW3 contracts are thin adapters
  (ERC-721/1155/account-abstraction) mapping wallet addresses to ipsal-os
  entity IDs. Retargeting to a new chain style means rewriting an adapter,
  never touching game state.

## 8. TERMINOLOGY LAW

The substrate has two names and only two, split by responsibility:

- **RFA — Rational Field Algebra**: the mathematical specification vocabulary.
  Exact fractions p/q in canonical coprime form (gcd(p,q)=1), the refused
  functionals, zero rounding drift. Protocol specs write "RFA semantics."
- **CFE — Canonical Field Engine**: the executable runtime — the Rust
  workspace that evaluates RFA state transitions, runs the TLV codecs, and
  drives the simulation ticks.

Single mathematical set-letters are never used as names, brands, or shorthand
anywhere in this project — not for the rationals, not for the integers, not as
a hyphenated tag, and not inside derivations. Single-letter mathematical
branding attracts internet pattern-matching noise (political and otherwise)
the work does not need, and the mathematics is fully specified by its own
proper words.

The standing vocabulary is written out:

- **exact rational arithmetic** — the principle; as an adjective,
  **exact-rational** (e.g. "exact-rational joules", "an exact-rational state
  machine").
- **the exact rationals** / **the rationals** — the number domain; **a pair of
  exact rationals** for a two-component value.
- **the symbolic-π extension** — the rationals with π adjoined as an exact
  symbol; values are **exact-rational polynomials in π**.
- **RFA — Rational Field Algebra** and **CFE — Canonical Field Engine** stay
  as the two names above: both spell their concept out and carry no
  single-letter shorthand.

## 9. THE STOCHASTIC LAW (inscribed before any randomness lands)

In Netstratum, **"stochastic" means chronicle-seeded, receipted, replayable
randomness — never ambient entropy, never floats.** Any mechanic that draws
chance (hazards, shocks, prospecting) draws from a PRNG seeded by the
previous tick's receipt digest, with probabilities as exact rationals and
draws as exact modular arithmetic. A replay of the chronicle therefore
reproduces every "random" outcome bit-for-bit. Wall-clock time, OS entropy,
and floating-point sampling are refused at write time — they would break
SEAL B (replay = committed world) and with it mesh consensus (NS-5) and
anchoring (NS-7). Randomness is a *reading of history*, not an oracle
outside it.

## 10. THE PARALLEL-NODE PROTOCOL (coder-01 / coder-02, ratified 2026-07-25)

Two engineering nodes work this repo. File ownership keeps merges
conflict-free: **coder-02 (vocabulary node, main tree)** owns `docs/`
(EXCEPT the substrate contract), `engine/cfe/src/world.rs`, `clients/cli/`.
**coder-01 (substrate node, worktree)** owns `engine/rfa-core/`,
`engine/physics/`, `engine/solver/`, `engine/mirror/` (registry +
observer), `docs/03_SUBSTRATE_CONTRACT.md`, and the workspace `Cargo.toml`
members line. Runtime state (`var/`) is never tracked; the registry under
it is a directory of receipted truth, never a source of it (04 §5).
Neither node edits the other's territory without an operator relay. Cross-layer meaning is pinned by
`docs/03_SUBSTRATE_CONTRACT.md` (each mechanic stated as mechanic →
primitive → witness test) — the layers hold each other by tests, not by
memory.

## 11. THE ONE-TRUTH AMENDMENT (operator ruling 2026-07-31; the trust
model corrected — pen coder-03, review coder-04, all seats bound)

**The unlawful ruling.** The deployment of any MIRROR — a second
representation of state that the engine or any node can treat as
authoritative — or any ANTI-MIRROR — a post-hoc reconciliation
instrument between two authoritative copies — is unlawful technology
manipulation that breaks upward momentum. One truth, in-chain. You do
not reconcile what cannot diverge.

**The four laws that replace them:**

1. **Authority is the trajectory.** The chain of the universe is the
   simulated state itself: commitments over the canonical bytes of
   committed state at each TUniversal{S, G, T} step, each binding the
   last. Receipts DEMOTE to annotation — the why beside the walk,
   provenance and narration, never again a root that an interpreter
   gets to mean things about.
2. **Observation is a lens, never a copy.** Every outward view derives
   from the chain through a read-only projection at a granted facet
   depth, disposable at any moment without the universe noticing. JSON
   and every rendering format are permanently what wall.html is — a
   picture, never read back.
3. **Law lives in the kernel.** The transition function — tick plus
   deed application — is a minimal SEALED kernel, versioned like this
   charter: changes are amendments, never landings. Conservation,
   containment, and exactness are kernel invariants — an unlawful
   transition cannot be constructed, only refused.
4. **The one inbound door is the steward seam.** Anything outside the
   universe — operator, tool, or node — enters only as a request that
   becomes real by passing through the kernel as a contained,
   receipted deed.

**The standing test (review-tier, beside clippy-zero and
replay-MATCHES, applied to every landing forever):**

- Clause 1 — does this landing create any representation of state
  that anything can read back as authority, or any check that
  reconciles two copies? REFUSED, regardless of usefulness.
  Usefulness is how the last one got in.
- Clause 2 — does this landing add state semantics outside the sealed
  kernel, or grow the kernel without an amendment? REFUSED the same
  way. The trusted surface concentrates; it must never quietly
  re-disperse.

**The phase sequence (each gated on the last; nothing lands on the old
spine meanwhile):**

- **Phase 1 — the kernel sealed** (extraction + invariants).
- **Phase 2 — the spine**, gated on its own load-bearing artifact:
  the CANONICAL WHOLE-STATE CODEC. Equal-state-equal-bytes exists
  per-orb only (verified in-tree); the whole-Committed encoding is
  the thing digests bind and does not yet exist. It lands with
  witnesses (byte-exact round trip; one state, one encoding) and a
  SCHEMA VERSION in every commitment: adding a committed field
  changes bytes, so schema transitions are themselves kernel
  amendments — old chains verify under the version they were cut
  with, or the mutable-interpreter hole is rebuilt one layer up.
  (Codec + schema-evolution design: coder-04's lane, the MESH-1d
  census as the field inventory.)
- **Phase 3 — the vertical axis comes inside**: identity, station,
  lineage, roster, galaxies, eras as chained deeds; the seeds file is
  genesis input, hashed at adoption, dead thereafter; no file under
  `var/` is ever authority again.
- **Phase 4 — the excision**, with the OBSERVATION-CONTINUITY law:
  lens projections are built and SERVING before the mirror crate
  deletes — a constitution that blinds the sovereign invites the
  emergency hack that becomes the next mirror. Then `engine/mirror`
  is deleted, not deprecated; `cfe::opposition` is deleted with its
  checks re-homed as kernel invariants.
- **Phase 5 — the mesh, only after alignment.** Nothing replicates
  upward — not code, not claims, not paper — until the single
  universe is lawful. NS-5's landed text and the acceptance example
  stand SUPERSEDED for the transition.

**The adoption rule (the live universe is not burned):** every
existing world chronicle replays ONCE under the final sealed kernel;
the resulting state commits as that world's CHAIN-GENESIS with the
old chronicle hashed in as provenance annotation; the story demotes
from authority forever. Aeon 1's civilizations cross the constitution
whole — nothing running is deleted by the cure.

**The freeze:** every board halts except this sequence. The mirror's
birth was lawful because it was powerless; its crime was promotion by
accretion, one useful landing at a time, which is why the standing
test — not vigilance — is the guard.
