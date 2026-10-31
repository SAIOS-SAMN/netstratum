# Netstratum — the Charter

This document defines the project's scope, its binding rules and the
protocol stack. Every other document in this repository refines it and may
not contradict it. Read it in full before the rest of the documentation.

Netstratum is a text-based, multi-planetary city-building and exploration
game in which autonomous agents play alongside human players (an "AI MUD").
Its design draws on Cities: Skylines (zoning, growth and trade) and Dark
Forest (a decentralized world with hidden information). The game is also the
reference implementation of the NS protocol stack (NS-1 to NS-7: DOP, RQP,
VRP, APERTURE, MESH, IPSAL-OS and Anchor), which is specified in Rational
Field Algebra (RFA) and executed by the Canonical Field Engine (CFE).

The repository holds three layers: the game (the client and the conductor),
the protocol specifications (`protocols/`) and the engine (`engine/`). They
share a repository so that a change to one layer is checked against the
other two in the same commit.

---

## 1. Specifications and mechanics are coupled (law xvii)

> **A protocol section is written only when the game mechanic that first
> consumes it is named. A game mechanic ships only when its protocol
> section and its test vectors exist.**

The protocol documents contain the shared test vectors, and the engine's
conformance tests consume them verbatim. A specification without an
executable test is incomplete; a mechanic without verification is
unfinished.

## 2. Infrastructure is not exposed to the player

Determinism, the TLV codecs, receipts and the refused functionals are
infrastructure. The player works with zoning, exploration, trade, fleets
and other players, not with the mathematics. Where mathematical elegance
conflicts with the player's experience, the specification is adjusted and
the engine absorbs the complexity. The engine's rule that a state
transition is either valid or rejected applies to state transitions, not
to game design.

## 2a. Rules are stated constructively

A quantity can be defined in two ways: directly, as a measurement of what
the state contains, or by complement, as what remains of a reference set
once something is removed (`x = U − y`). A complement depends on the
reference `U`: if `U` is misidentified, `x` measures the wrong object, and
nothing in `x` itself shows the error. A direct measurement depends only on
the state it reads. The documents therefore use a syntropic register, not a
negentropic one: *syntropy* is a positive quantity measured on what is
built, while *negentropy* is defined as the absence of entropy, a
complement.

Three rules follow, and they apply to every section below:

1. **The constructive rule is stated first.** Refusals, bans, slashing
   conditions and lints remain in force, but each is stated as a
   consequence of a constructive rule, because a prohibition only names the
   complement of a set the constructive rule defines. A rule that can only
   be stated as a prohibition has not yet defined that set.
2. **Convergence is the primary frame.** The mesh's process is convergence
   (NS-5, proof of convergence); a fork is a branch inside that process
   which convergence later annexes, not its opposite.
3. **Measurements take precedence over assumptions.** A refusal encodes an
   assumption about which states occur; a reading measures the state. Where
   they disagree, the assumption is the part that is wrong, so the reading
   applies.

**Evidence.** Both complement definitions used so far produced incorrect
results. The value law was stated as two ceilings without a direction until
`I4` and `I5` were restated as direct syntropy and entropy measures, and
capacity was implemented as what remains after stripping until it was
rebuilt as the measured circulation.

## 3. Design constraints

Dark Forest (0xPARC) showed that a decentralized world with hidden
information is feasible; the points at which it failed motivate the five
constraints below. Each is stated as a property of this system:

1. **Persistence.** The append-only VRP chronicle is the world state. The
   system has no reset, wipe or ephemeral round, and no operation exists to
   perform one.
2. **No floating point.** Every state transition uses exact rational
   arithmetic (RFA). There is no float emulation. If state is later bridged
   to zero-knowledge proofs, rational arithmetic keeps the circuits small.
3. **Consensus without a central server.** Peers verify each other's state
   by exact measurement (NS-5, proof of convergence). There is no central
   server or global RPC endpoint whose failure stops the world.
4. **Governed agents.** Automated agents are participants. Every agent,
   whether written by a player or run as an NPC, executes inside IPSAL-OS
   containment with dimensional permissions and generation-tagged receipts.
   Automation is governed, not banned.
5. **Immediate access.** The CLI client starts with one command and needs
   no wallet, gas, invite key or bridge.

## 3a. Observation uses lenses, never mirrors

Let `S` be the committed state.

- A **lens** is a function `L: S → R`, evaluated on demand. Its output is
  determined by `S`, so it carries no information beyond `S`
  (`H(L(S) | S) = 0`), and it cannot disagree with `S`: there is no second
  record to compare against.
- A **mirror** is a stored record `S′` with the intended invariant
  `S′ = S`. Every update `u` must then be applied twice, `S ← u(S)` and
  `S′ ← u(S′)`. An update applied to one record and not the other (a
  partial write, a failure between the two writes, a defect in either
  path) breaks the invariant, so `S′ ≠ S` is a reachable state.

The mirror's value is zero in both cases: while the invariant holds,
`H(S′ | S) = 0` and `S′` adds nothing; once it fails, `S′` is wrong. Its
cost is a second write path and a reconciler `r(S, S′)` that exists only to
repair the divergence, and that reconciler is itself a second update path,
which §11 rule 3 excludes. Inspection of the system's own state by a copy of
itself is the identity map (`α ≅ Ω`), which Article III prices at zero.

**Consequence.** No crate, module, type, field or specification primitive is
a mirror or is named one. Observation is a lens (`SYSTEM_SPEC` §3), as are
rendering, the observer, the wall and the journals. Every reading is derived
from `S`.

**History.** `engine/mirror` began as a read-only dashboard and became
load-bearing through successive changes, because no rule prevented it. §11
was written in response; the crate was removed (d606b01), together with
`cfe::opposition`, the reconciler it had required (c7cad9e). ANTI-MIRROR
followed: verification by opposition requires two records to compare, which
§11 rule 5 clause 1 forbids.

**Enforcement.** Standing tests hold the rule:
`recurrence_guard::the_dead_name_stays_dead` (the crate and its workspace
member stay removed), `recurrence_guard::the_anti_mirror_stays_excised`
(the reconciler stays removed) and `constitution_lint::no_mirror_in_the_law`
(the name stays out of the specifications).

## 4. Protocol-to-mechanic map

| Spec | Protocol | First game consumer |
|---|---|---|
| NS-1 | **DOP** — Dimensional Object Protocol | Colony, sector and profile orbs: every game object is a self-contained orb with provenance |
| NS-2 | **RQP** — Resonance Query Protocol | The scanner: search by structural resonance, not by string match |
| NS-3 | **VRP** — Verification Receipt Protocol | The world chronicle: every action is an append-only receipt linked to its predecessor |
| NS-4 | **APERTURE** — controlled disclosure | Sector-map sharing and private trade offers, disclosed at a granted level (summary, detail, full) |
| NS-5 | **MESH** — Proof-of-Convergence Consensus | Multi-node world synchronization (two machines running the same world) |
| NS-6 | **IPSAL-OS** — Entity Containment | Player, NPC and bot identity: the profile of every individual on the network |
| NS-7 | **Anchor** — external settlement bridge | High-value settlement: minting a rare world, large trades, and asset ownership anchored to an external chain |

## 5. Prior work

The following earlier repositories informed the design. Their structure
may be consulted; code is not ported without review, and every scalar,
scored or capped form in them is refused.

- `/opt/repos/`: convergence-engine (the mathematics), AI-TECHNOLOGY-WWW
  (the WWW4 protocol drafts that the NS specifications supersede),
  governance (a five-layer architecture), NEBULA-PRODUCTS (platform
  design) and AI-scientists (safety research).
- `/opt/ipsal-autic-v0.4/`: the sealed engine build, containing the
  chronicle (the predecessor of VRP), the sigil index (of RQP), consilience
  (of MESH), the TLV wire format, and the component map in
  `docs/GROUND.md`, which lists what is not to be rebuilt.
- **Naming:** the sovereign layer is named **ipsal-os**. The earlier name,
  SAIOS, is retired for this layer because of an external name collision.

## 6. Governance

`docs/BUILD_ORDER.md` records the build order, and the current landing is
read there rather than inferred. Each landing is written test-first, with
its failing and passing output recorded; landings proceed one at a time;
evidence is quoted verbatim. The repository's history is its git history,
and changes are committed only on the operator's approval.

## 7. Settlement on external chains

Existing blockchains (EVM and UTXO chains, referred to as WWW3) and the NS
mesh (WWW4) have separate roles; the mesh connects to the chains without
moving game logic onto them.

> **Game logic is defined in the NS specifications (RFA) and executed by
> the CFE. External chains are used only for settlement: timestamps, asset
> ownership and liquidity. No game rule is encoded in contract bytecode.**

**Anchoring in three levels.** This design avoids Dark Forest's approach of
writing every tick on chain:

1. **Local simulation.** CFE state transitions run locally at full speed,
   and each emits an NS-3 receipt. No gas or RPC call is involved.
2. **Mesh consensus.** Peers converge on state through NS-5 proof of
   convergence. Multiplayer synchronization happens here, without a chain.
3. **Periodic anchoring (NS-7).** At defined intervals, or on high-value
   events, the mesh computes a consensus root of the chronicle and posts
   that root to an external chain. The chain stores checkpoints, not ticks,
   so a fork, upgrade or outage of the chain does not stop the world.

**Compatibility guarantees.** An upgrade never breaks state:

- **SEAL A — codec compatibility.** The TLV wire format is additive: a
  reader skips unknown tags, and stored state round-trips byte-exact across
  every engine version.
- **SEAL B — the chronicle is authoritative.** The world is stored in the
  append-only NS-3 chronicle, not in a mutable database or a contract
  variable. The renderer can be replaced, the engine upgraded or the chain
  migrated; the chronicle replays the world from tick zero.
- **SEAL C — adapters, not logic.** External contracts are thin adapters
  (ERC-721, ERC-1155, account abstraction) that map wallet addresses to
  IPSAL-OS entity IDs. Moving to a different chain means rewriting an
  adapter, never changing game state.

## 8. Terminology

The substrate has two names, divided by responsibility:

- **RFA — Rational Field Algebra:** the mathematical specification
  vocabulary. Exact fractions p/q in canonical coprime form (gcd(p, q) = 1),
  the refused functionals, and no rounding drift. Protocol specifications
  refer to "RFA semantics".
- **CFE — Canonical Field Engine:** the executable runtime, the Rust
  workspace that evaluates RFA state transitions, runs the TLV codecs and
  drives the simulation.

Single mathematical set letters are not used as names, brands or shorthand
anywhere in the project: not for the rationals, not for the integers, not
as hyphenated tags and not inside derivations. Such shorthand attracts
unrelated associations online, and the mathematics is fully specified by
its proper names.

The standard vocabulary:

- **exact rational arithmetic** for the principle, and **exact-rational**
  as the adjective (for example "exact-rational embers", "an exact-rational
  state machine");
- **the exact rationals** or **the rationals** for the number domain, and
  **a pair of exact rationals** for a two-component value;
- **the symbolic-π extension** for the rationals with π adjoined as an
  exact symbol, whose values are **exact-rational polynomials in π**;
- **RFA** and **CFE** as defined above; both are spelled out and carry no
  single-letter shorthand.

## 9. Randomness

In Netstratum, **"stochastic" means chronicle-seeded, receipted, replayable
randomness: never ambient entropy and never floating point.** A mechanic
that draws chance (hazards, shocks, prospecting) draws from a PRNG seeded
by the previous tick's receipt digest, with probabilities as exact
rationals and draws as exact modular arithmetic. Replaying the chronicle
therefore reproduces every random outcome bit for bit. Wall-clock time,
operating-system entropy and floating-point sampling are refused at write
time, because each would break SEAL B (a replay reproduces the committed
world) and with it mesh consensus (NS-5) and anchoring (NS-7). Randomness is
derived from the recorded history.

## 10. Parallel development

When two engineering nodes, coder-01 and coder-02, worked this repository,
file ownership kept their merges free of conflicts:

- **coder-02** (the vocabulary node, main tree) owned `docs/` (except the
  substrate contract), `engine/cfe/src/world/` and `clients/cli/`;
- **coder-01** (the substrate node, a worktree) owned `engine/rfa-core/`,
  `engine/physics/`, `engine/solver/`, `engine/ledger/` (the registry and
  the observer), `docs/03_SUBSTRATE_CONTRACT.md` and the `members` line of
  the workspace `Cargo.toml`.

Neither node edited the other's files without a relay through the operator.
Runtime state (`var/`) is never tracked; the registry under it is a
directory of receipted records, never a source of truth (04 §5). Meaning
across the layers is pinned by `docs/03_SUBSTRATE_CONTRACT.md`, which states
each mechanic as mechanic, primitive and witness test, so that the layers
are held together by tests.

## 11. The ONE-TRUTH amendment

This section defines how state is stored, changed and observed: a coordinate
names each state, the kernel changes it, the chain records it, the galaxy
supplies its context and the ideology evaluates it.

**Principle.** The game keeps exactly one machine-readable copy of state,
and every reading is derived from it. The prohibition that follows is a
consequence of this principle (§2a).

**Consequence.** A second machine-readable copy of state, or a tool that
reconciles two copies after the fact, is not permitted. The opposition
module was removed under this section (c7cad9e); the ledger's display
exports are removed at step 4, once views are served from the chain.

**Rules (normative):**

1. **Single source of truth.** The game keeps exactly one machine-readable
   copy of state: a hash-chained sequence of state snapshots, each named by
   its coordinate TUniversal{S, G, T} and bound to its predecessor.
2. **Exports are write-only.** JSON files, dashboards and save-file copies
   are display output. No code parses them back in.
3. **A single validated update function.** Every state change goes through
   `admits(prior, next, step) -> Ok | typed refusal`, which is pure, total,
   free of I/O and independent of encoding: it operates on in-memory state
   and never produces or reads bytes. Serialization belongs to the codec
   (the snapshot encoder of rule 1) alone, so the update function and the
   chain cannot disagree about bytes. The invariants the function enforces
   (totals balance, permissions are checked, arithmetic is exact) are each
   proven by a witness test written test-first. The function is versioned
   like this charter: a change to it is an amendment.
4. **A single input boundary.** Every external party (a person, a tool or
   another node) submits requests to the inbox queue; the update function
   validates and applies them. State is never edited directly.
5. **The review lint (applied to every change).** Clause 1: a change that
   adds a second readable copy of state, or a reconciling check between
   copies, is rejected, however useful it appears. Clause 2: a change that
   puts state-changing logic outside the update function, or extends the
   update function without an amendment, is rejected likewise.

**Build order.** Each step depends on the previous one; other work is
paused meanwhile.

- **Step 1 — the update function**, extracted and sealed per rule 3. Its
  specification (the six invariants; the equations for the coherence
  functional, the constitution digest, the commitment and the fold gate;
  and the convergence algorithm, which is dimensional resolution rather
  than gradient descent) is in `engine/kernel/CHARTER.md`. That document is
  subordinate to this section and is amended at charter level (bumping
  `kernel::VERSION`).
- **Step 2 — the snapshot codec** (coder-04): the canonical encoding of the
  whole committed state, which the hashes bind. It carries a schema
  version; a change to the state's field set is an amendment, and existing
  chains verify under the version they were written with. The committed
  state's field set is frozen until codec v1 lands, and any change touching
  it is coordinated with the codec owner first. The commitment is
  `H(schema ‖ state_bytes ‖ predecessor ‖ coordinate)`; only the spine
  produces it, and the update function never reads it.
- **Step 3 — the chain and the vertical axis.** Snapshots are chained.
  Identity, station, lineage, roster, galaxies and eras become chained
  state, changed only through the inbox. The seeds file is genesis input,
  hashed once at adoption and not read again.
- **Step 4 — views first, then removal.** Read-only views (the observer,
  the wall and the journals) are served from the chain before the ledger's
  display exports are removed, so the operator's view is never
  interrupted. The exports are then removed, not deprecated (the
  opposition module is already removed).
- **Step 5 — the mesh.** Nothing (code, claims or documents) replicates to
  other nodes until this universe runs on the chain. The consensus claims of
  the previous mesh specification are suspended until then.

**Adoption.** Each existing world's event log is replayed once under the
sealed update function; the resulting state becomes that world's first
chained snapshot, with the old log hashed in as history. No running world is
deleted.

**Nurseries.** A world is created and run on a single node, its nursery.
Because its commitment format and its validation are the mesh's own, it can
join the mesh without conversion. Joining is verification: a node presents
its chain, peers re-run the same kernel over every step, and admission
follows. A puppet universe is a nursery that never obtained confirmation, so
the sandbox rules and the Sybil analysis use the same terms.

**Consensus weight.** Consensus weight is
earned by certificate-verified genuine difference: proof of useful work.
This is the primary mechanism and it is not yet built. The secondary
mechanism, specified in NS-5 §7.6, penalizes fabrication: an open
fork is permitted, and a node that forks openly is a lawful nursery; only
fabricated confirmation is slashed. A node proven to have fabricated
confirmation forfeits its work and its stake, through the existing
`ForwardSlashed` and forfeit mechanics. Work anchoring makes a network of
puppet nodes costly, and slashing on proven fabrication removes their
stake. Three economic parameters remain to be set (what verified work earns, the
size of the stake and the slashing amount); they are set together, because
the reward and the penalty act on the same balance.

**Rationale.** The former `engine/mirror` began as a read-only dashboard
and became load-bearing through successive changes, because no rule
prevented it. Rule 5 is that rule.
