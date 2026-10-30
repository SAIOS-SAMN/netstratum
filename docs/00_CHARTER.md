# Netstratum — the Charter

**The one founding document. Read first, whole. Everything else in this repo is
downstream of what is inscribed here.**

Netstratum is an AI MUD — a text-first, multi-planetary city-building and
exploration game in the lineage of Cities:Skylines (the intuitive joy of zoning,
growth, and trade) and Dark Forest (the decentralized, hidden-information
cosmos) — that is simultaneously the **playable reference implementation of the
dimensional web**: the NS protocol stack (DOP, RQP, VRP, APERTURE, MESH, IPSAL-OS,
anchor) specified in Rational Field Algebra (RFA) and executed by the Canonical
Field Engine (CFE).

The game is the front door. The protocols are the building. The engine is the
ground. One repo holds all three so they cannot drift apart.

---

## 1. The core mandate (law xvii, inherited and binding)

> **A protocol section may only be written when its first game consumer is
> named, and a game mechanic may only ship when its protocol section and test
> vectors exist.**

No spec without a consumer. No mechanic without a spec. The shared test vectors
are the joint: fixtures written in the protocol document are consumed verbatim
by the engine's conformance tests. A spec that cannot be executed is prose; a
mechanic that cannot be verified is a bug that hasn't landed yet.

## 2. The fun-first substrate rule

RFA determinism, TLV codecs, receipts, and the refused-functionals
discipline are **invisible infrastructure**. The player sees zoning,
exploration, trade, fleets, and other citizens of the stratum — never the math.
Rigor serves play the way a traffic solver serves a city builder: absolutely,
and silently. When a design choice pits mathematical elegance against player
experience, the spec bends and the substrate absorbs the complexity. The
substrate's law ("valid or rejected, nothing in between") governs state
transitions, not game design.

## 2a. The syntropic register (operator ruling 2026-08-01)

**This charter is written syntropic, never negentropic.** The distinction
is exact and not stylistic. *Negentropy* is negative entropy — a good
defined as the absence of a bad, which makes the bad the primary object
and the good its shadow. *Syntropy* is a positive quantity in its own
right: it names what is built, and needs no failure to point at.

Three bindings follow, and they order every section below:

1. **The generative side leads.** What the system makes is stated first
   and stands on its own merit. Refusals, bans, slashes and lints are
   real and stay — but they are subordinate clauses of a positive law,
   never the law itself. A rule that can only be stated as a prohibition
   is a rule that has not yet found its object.
2. **Convergence is the frame; divergence is a case within it.** The
   mesh converges (NS-5, proof-of-convergence); honest forking is a
   lawful case inside that frame, not the frame's opposite. Where this
   document must name divergence it names it as the lesser reading —
   what convergence tolerates, never what the system is organised
   around.
3. **A negative assumption carries less weight than a positive
   measurement.** Where a refusal and a reading disagree about what a
   state is, the reading governs. The engine measures what is there
   before it prices what is missing.

**Why this is law and not taste.** The corpus has drifted this way twice
already and both times it cost real correctness: the value law read as
two ceilings with no direction until `I4`/`I5` recovered their
syntropy/entropy names, and capacity was built as a residue — what
survives after stripping — until it was rebuilt as the circulation it
positively is. In both cases the negentropic form was not merely bleak,
it measured the wrong object. Stating the positive first is how the
object stays in view.

## 3. The inscribed constraints (five things this system builds)

Five standing constructions, each stated as what it makes. Dark Forest
(0xPARC, ~2020) proved decentralized hidden-information worlds are
possible, and the ways it came apart are the evidence these five are
load-bearing — evidence, never the definition. Each stands on its own
merit and would stand had that project never existed:

1. **Persistence by construction.** The append-only VRP chronicle is the world
   state. There are no resets, no wipes, no ephemeral beta rounds — a reset is
   not a policy we refuse but an operation that does not exist.
2. **Zero floating-point tax.** All state transitions are native RFA (exact
   rational arithmetic). No
   float emulation, no constraint blowup, no thermal throttling. If state is
   ever bridged to ZK proofs, rational arithmetic keeps circuits light.
3. **MESH consensus, no chokepoint.** Peers verify each other's state by exact
   measurement (proof-of-convergence). There is no central server to melt and
   no global RPC to crash.
4. **Governed autonomous agents.** This is an AI MUD: bots are not cheating,
   they are citizens. Every agent — player-written or NPC — runs inside
   ipsal-os containment with dimensional permissions and generation-tagged
   receipts. Automation is governed, never banned and never an arms race.
5. **Instant onboarding.** One command launches the CLI client. No wallet, no
   gas, no invite keys, no bridging. The barrier to entry is typing.

## 3a. The lens law — no mirrors in the infrastructure

**Ruled 2026-08-01. A mirror is banned as a structure and as a word.**
The difference is what the instrument is for, and which way it points.

- **A lens offers solutions.** You turn a lens on something to resolve
  it — it magnifies, focuses, and returns a reading that did not exist
  before you looked. It points outward, at an object, and its whole
  purpose is to produce what was not previously legible. A lens is
  generative: it answers a question.
- **A mirror returns the self.** You use a mirror to inspect yourself.
  It points inward and gives back exactly what was already there — no
  new information, by construction, because self-inspection is the one
  operation that cannot resolve anything. It answers nothing; it
  confirms.

**This is the dogmatic loop wearing glass.** A system that inspects
itself and calls the looking work is precisely `α ≅ Ω` — motion that
returns to its origin, deeds emitted against nothing gained, which
Article III prices at zero. The mirror is that loop's instrument. The
lens is how a system looks at something other than itself and comes
back holding more than it left with.

*The copy is the consequence, not the definition.* Because a mirror
returns the self, it must hold a second image to return — and from that
moment there are two things that can disagree, and every reconciler
after it is a tool built to manage a problem that should not exist.

**Binding:** no crate, module, type, field, or spec primitive in this
workspace may be a mirror or be named one. Observation is a lens
(`SYSTEM_SPEC` §3); rendering is a lens; the observer, the wall and the
journals are lenses. Where a reading is needed, it derives.

**This is the section that was earned rather than reasoned.**
`engine/mirror` was born a read-only dashboard and was safe while that
was all it was. It became load-bearing one useful change at a time, and
no rule existed to stop it — so §11 was written, the crate was excised
(d606b01), and `cfe::opposition`, the reconciler it had made necessary,
went with it (c7cad9e). Anti-mirror followed: verification by opposition
still needs two records to set against each other, which §11 rule 5
clause 1 forbids outright. *You do not reconcile what cannot diverge.*

**Held by standing tests, never by memory:**
`recurrence_guard::the_dead_name_stays_dead` keeps the crate and the
workspace member dead; `recurrence_guard::the_anti_mirror_stays_excised`
keeps the reconciler dead; `constitution_lint::no_mirror_in_the_law`
keeps the name out of the specifications. Vigilance did not save us the
first time; a standing test did.

## 4. The protocol-to-mechanic binding map

| Spec | Protocol | First game consumer |
|---|---|---|
| NS-1 | **DOP** — Dimensional Object Protocol | Colony, sector, and profile orbs — every game object is a self-contained orb with provenance |
| NS-2 | **RQP** — Resonance Query Protocol | The scanner: search by structural resonance, not string match |
| NS-3 | **VRP** — Verification Receipt Protocol | The world chronicle: every action an append-only, predecessor-linked receipt |
| NS-4 | **APERTURE** — the controlled opening | Sector-map sharing and private trade offers, opened at a granted facet (summary → detail → full) |
| NS-5 | **MESH** — Proof-of-Convergence Consensus | Multi-node world synchronization ("two laptops, one world") |
| NS-6 | **IPSAL-OS** — Entity Containment | Player, NPC, and bot identity: the profile of every individual inside our internet |
| NS-7 | **Anchor** — WWW3 Settlement Bridge | High-value settlement: minting a rare world, macro-trades, asset ownership anchored to an external chain |

## 5. Ancestry (consult, never port blindly)

- `/opt/repos/` — the reference era: convergence-engine (the math),
  AI-technology-WWW (the WWW4 protocol drafts the NS specs supersede),
  governance (the five-layer architecture), nebula-products (the platform
  vision), AI-scientists (the safety research). Shapes are reference; every
  scalar/scored/capped form is refused.
- `/opt/ipsal-autic-v0.4/` — the sealed engine build: the chronicle (VRP's
  working predecessor), the sigil index (RQP's), consilience (MESH's), the TLV
  wire, and `docs/GROUND.md`'s organ map — the do-not-rebuild gate.
- **Naming:** SAIOS is retired (external name collision). The sovereign layer
  is **ipsal-os**, here and everywhere forward.

## 6. Governance

`docs/BUILD_ORDER.md` is the one carrier of build order — the current landing
is always read there, never inferred. Landings are red-first with recorded
output; one landing at a time; evidence blocks verbatim; the chronicle of the
repo is its git history, committed only on explicit operator go.

## 7. The WWW3 bridge (settlement, never simulation)

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
   Multiplayer sync happens here, chain-free.
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

## 8. Terminology law

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

## 9. The stochastic law (inscribed before any randomness lands)

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

## 10. The parallel-node protocol (coder-01 / coder-02, ratified 2026-07-25)

Two engineering nodes work this repo. File ownership keeps merges
conflict-free: **coder-02 (vocabulary node, main tree)** owns `docs/`
(Except the substrate contract), `engine/cfe/src/world/`, `clients/cli/`.
**coder-01 (substrate node, worktree)** owns `engine/rfa-core/`,
`engine/physics/`, `engine/solver/`, `engine/ledger/` (registry +
observer), `docs/03_SUBSTRATE_CONTRACT.md`, and the workspace `Cargo.toml`
members line. Runtime state (`var/`) is never tracked; the registry under
it is a directory of receipted truth, never a source of it (04 §5).
Neither node edits the other's territory without an operator relay. Cross-layer meaning is pinned by
`docs/03_SUBSTRATE_CONTRACT.md` (each mechanic stated as mechanic →
primitive → witness test) — the layers hold each other by tests, not by
memory.

## 11. The one-truth amendment (operator ruling 2026-07-31; rewritten
to the plain register by the same ruling — the document says what it
means; pen coder-03, review coder-04, all seats bound)

*The game is one walk: the coordinate names it, the kernel steps it,
the chain remembers it, the galaxy flavors it, the ideology judges
it. Everything below is those five layers stated as rules.*

**The one truth.** The game keeps exactly one machine-readable copy of
state, and every reading derives from it. That is the whole of this
section; the prohibition below is its consequence, not its content
(§2a: a refusal is a subordinate clause of a positive law).

*Consequent:* a second machine-readable copy, or a tool that reconciles
two copies after the fact, is therefore banned — there is nothing for it
to be a second of. The opposition module was deleted under this section
(2026-07-31); the ledger's display half (the excised crate's lens
exports) deletes at Step 4 once views serve from the chain.

**The five rules (normative):**

1. **Single source of truth.** The game keeps exactly one
   machine-readable copy of state: a hash-chained sequence of state
   snapshots, each named by its coordinate TUniversal{S, G, T} and
   bound to its predecessor.
2. **Exports are write-only.** JSON files, dashboards, and save-file
   copies are display output. No code ever parses them back in.
3. **One validated update function.** All state changes go through
   `admits(prior, next, step) -> Ok | typed refusal`: pure, total, no
   I/O, and encoding-blind — it works on in-memory state only and
   never produces or reads bytes. Serialization belongs exclusively
   to the codec (rule 1's snapshot encoder), so the update function
   and the chain can never disagree about bytes: only one component
   ever produces them. The rules it enforces — totals balance,
   permissions checked, exact arithmetic only — are invariants inside
   it, each proven by its own witness, written red-first. The
   function is versioned like this charter: changes are amendments.
4. **One input boundary.** Anything external — a person, a tool,
   another node — submits a request to the inbox queue; the update
   function validates and applies it. No direct edits, ever.
5. **The review lint (applied to every change, forever).**
   Clause 1: a change that adds a second readable state copy, or a
   reconciling check between copies, is rejected regardless of how
   useful it looks — useful is how the last one got in.
   Clause 2: a change that puts state-changing logic outside the
   update function, or grows the update function without an
   amendment, is rejected the same way.

**The build order (each step gated on the last; all other boards
frozen meanwhile):**

- **Step 1 — the update function**, extracted and sealed per rule 3. Its
  full law — the six invariants, the equations (coherence functional,
  constitution digest, commitment, the fold gate), and the convergence
  algorithm (dimensional resolution, not gradient descent) — is inscribed
  in `engine/kernel/CHARTER.md`, subordinate to this section and amended
  charter-tier (bumping `kernel::VERSION`).
- **Step 2 — the snapshot codec** (coder-04's lane): the canonical
  encoding of the whole committed state — the thing the hashes bind.
  It carries a schema version; changing the state's field set is
  itself an amendment, and old chains verify under the version they
  were cut with. **The freeze window: the committed state's field
  set holds stable until codec v1 lands** — any in-flight change
  touching it coordinates with the codec lane first. The commitment
  is `H(schema ‖ state_bytes ‖ predecessor ‖ coordinate)`, produced by
  the spine only; the update function never reads it.
- **Step 3 — the chain and the vertical axis**: snapshots chained;
  identity, station, lineage, roster, galaxies, and eras become
  chained state changed only through the inbox. The seeds file is
  genesis input, hashed once at adoption, dead thereafter.
- **Step 4 — views first, then deletion**: read-only views (observer,
  wall, journals) serve from the chain before the ledger's display
  half is deleted — the operator's window never goes dark. Then the
  lens exports are removed, not deprecated (the opposition module is
  already gone, 2026-07-31).
- **Step 5 — the mesh, last.** Nothing replicates to other nodes —
  code, claims, or documents — until this universe runs on the
  chain. The old mesh spec's consensus claims stand superseded until
  then.

**The adoption rule (the running universe survives):** every existing
world's event log is replayed once under the sealed update function;
the resulting state becomes that world's first chained snapshot, with
the old log hashed in as history. Nothing running is deleted by the
cure.

**The nursery law.** A world is born and raised on one node — the nursery
— and because its commitment shape and its judge are already the mesh's
own, it is born MESH-ready. Joining the mesh is the audience growing: you
present your chain, peers re-run their identical kernel over every step,
and admission is verification. A **puppet universe is precisely a nursery
that never earned confirmation** — the same vocabulary names the sandbox
law and the Sybil analysis.

**The alignment covenant — weight is earned (§2a: the generative side
leads).** The ordering mechanism is one economy read from its positive
end. **Consensus weight is earned by certificate-verified genuine
difference** — proof-of-useful-work, the primary half, and the one still
unbuilt: it is the load-bearing gap precisely because it is the half
that generates. The subordinate half, ruled and boarded (NS-5 §7.6),
prices fabrication: **honest forking is lawful and expected — a node
that forks openly is a lawful nursery; only fabricated confirmation is
slashed.** A node that fabricates confirmation and is
proven to has its pen, its work, and its stake forfeited — reusing the
landed `ForwardSlashed`/forfeit mechanics. The puppet web is made
expensive by work-anchoring and ruinous by stake-slashing on proven
fabrication. The three economics questions — what verified work earns,
what the stake is, how much slashes — are operator rulings, ruled in one
sitting because the carrot and the stick are one economy.

**Why this section exists:** the old mirror was born as a read-only
dashboard and was safe while that was all it was. It became
load-bearing one useful change at a time, and no rule existed to stop
that. Rule 5 is that rule.
