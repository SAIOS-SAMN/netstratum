# NETSTRATUM — THE CHARTER

**The one founding document. Read first, whole. Everything else in this repo is
downstream of what is inscribed here.**

Netstratum is an AI MUD — a text-first, multi-planetary city-building and
exploration game in the lineage of Cities:Skylines (the intuitive joy of zoning,
growth, and trade) and Dark Forest (the decentralized, hidden-information
cosmos) — that is simultaneously the **playable reference implementation of the
dimensional web**: the NS protocol stack (DOP, RQP, VRP, BEP, MESH, IPSAL-OS,
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
| NS-4 | **BEP** — Bubble Exchange Protocol | Sector-map sharing, private trade bubbles, permissioned hand-offs |
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

The standalone letter "Q" is never used as a name, brand, or shorthand in any
public-facing surface of this project. Single-letter branding attracts
internet pattern-matching noise the work does not need; the mathematics is
fully specified by its own proper names. (The symbol ℚ may still appear inside
formal mathematical derivations where it denotes the field of rationals and
nothing else.)

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
`engine/physics/`, `docs/03_SUBSTRATE_CONTRACT.md`, and the workspace
`Cargo.toml` members line. Neither node edits the other's territory without
an operator relay. Cross-layer meaning is pinned by
`docs/03_SUBSTRATE_CONTRACT.md` (each mechanic stated as mechanic →
primitive → witness test) — the layers hold each other by tests, not by
memory.
