# The substrate contract — mechanic → primitive → witness

**Status: proposed, coder-01. Binding once ratified by the operator and
cited by World-Book Ch. 2.** This document pins the vocabulary layer
(coder-02: docs, world rules, CLI) to the substrate layer (coder-01:
`engine/rfa-core`, `engine/physics`) so the two nodes cannot silently
diverge: every mapping below names the mechanic, the exact primitive
that implements it, and the witness test that goes red if either side
drifts.

Ownership (Charter §10): coder-02 owns `docs/` (except this document),
`engine/cfe/src/world/`, `clients/cli/`. coder-01 owns
`engine/rfa-core/`, `engine/physics/`, this document
(`docs/03_SUBSTRATE_CONTRACT.md`), and the workspace members line.
Cross-territory changes travel by relay through the operator, never
directly.

---

## 1. Perception basis (the Hail Mary axis) → coboundary

A civilization's measurement frame is a coboundary: two civs observing
the same world in different bases hold Deltas differing by exactly a
frame distortion `Δ_ij += p_j − p_i`.

- **Primitive:** `rfa_core::coboundary_reduce` — H¹ projection strips
  the frame; the canonical representative is the basis-invariant
  content. Translation loss is the reduction residual: measured,
  displayed, never silent.
- **Witness:** `rfa_vectors::coboundary_shift_does_not_change_the_class`
  (two frames, one class), `a_pure_coboundary_reduces_to_zero` (a
  message that is all frame carries no content).
- **Ch. 2 obligation:** specify translation against these semantics —
  cite the function, do not re-derive the concept.

## 2. "Discovery needs a stranger" → class equality across distinct frames

Consilience (v0.4 doc 14): two identical lenses verify nothing. Two
civilizations confirm a discovery iff their independently measured
Deltas reduce to the same canonical representative from measurably
different coboundaries.

- **Primitive:** `coboundary_reduce(a) == coboundary_reduce(b)` with
  `a != b` — the arity-2 distinct-vantage check is an exact predicate,
  not a judgment call. Identical frames refuse as trivial.
- **Witness:** the class-equality half is pinned by
  `coboundary_shift_does_not_change_the_class`; the distinct-vantage
  refusal lands with the first consumer (N4/N6).

## 3. Discovery vs. trade → Mint vs. Transfer, at the type level

A trade can be unwound; a discovery cannot be un-known. The substrate
holds this distinction by construction, not by convention.

- **Primitive:** `physics::FlowEvent::Transfer` carries its exact
  inverse (`apply ∘ apply_negated = identity`, byte-exact);
  `physics::FlowEvent::Mint` appends planes the additive inverse can
  never reach — irreversible by construction. `physics::step_witness`
  reads a Transfer `Verified` and a Mint `Violated{axis}` — the
  violation is the mechanic's teeth, not a defect.
- **Witness:** `flow_pins::the_mint_extends_the_frame_irreversibly`,
  `flow_pins::the_step_witness_measures_all_three_verdicts`.

## 4. Inverted metabolisms → antisymmetry

"One side's waste stream is the other's food supply" is one tensor entry
read in two directions: `Δ_ij = −Δ_ji`. The mineral/biotic inversion is
not two ruleset tables kept consistent by hand — it is one antisymmetric
relation that cannot desync.

- **Primitive:** `rfa_core::Delta::set_antisym` (the only lawful
  writer), `Delta::conservation_measurement` (a measured distance to
  the invariant surface, zero iff exact).
- **Witness:** `rfa_vectors::constructed_deltas_conserve_exactly`,
  antisymmetry asserts inside
  `reduction_lands_the_canonical_integer_representative`.

## 5. Shock, stress, recovery (stochastic elasticity) → torsion, coherence, gates

A shock injects torsion into the colony Delta; the visible stress
gauge is the coherence functional; recovery is a sequence of gated
convergence steps.

- **Primitive:** `rfa_core::coherence_functional` (`C = Σ |R|`, exact
  rational, zero iff the cocycle condition holds — the differences
  compose; the dashboard number; the proportional law, operator ruling
  2026-07-31). **Never read zero as "converged"** (operator correction
  2026-08-01): zero says the relations compose, not that difference is
  gone — the H¹ core is invisible to C and survives reduction intact.
  Here that distinction is the mechanic: recovery drains the torsion a
  shock injected, and the colony's own structural difference is not
  stress and is never drained with it;
  `rfa_core::for_each_residual` (Where the stress lives, per triple,
  per coordinate); `rfa_core::validate_step` (typed refusals:
  `ConservationViolated`, `MagnitudeDrift` — **corrected 2026-08-13
  (doc audit): `Unproductive` was struck by operator ruling 2026-08-01**
  (`gates.rs:41-53` — "the productivity gate is struck"; `GateReading`
  now has 2 refusal variants, not 3). The "recovery steps must strictly
  reduce C" rule this variant enforced is gone with it.
- **Witness:** `elasticity_witness.rs` — the full loop: coherent
  colony → shock → measured stress → gated recovery → convergence.

## 6. Randomness (the elasticity trigger) → the chronicle beacon

**Law (Charter §9, inscribed by coder-02; staged with the same
operator go as this contract):** "stochastic" means chronicle-seeded,
receipted, replayable randomness — a draw derives from the previous tick's receipt
digest by exact arithmetic; probabilities are rational; no float, no
ambient entropy, no wall clock ever enters. Replay (SEAL B), mesh
verification (N6), and anchoring (N7) survive untouched.

- **Primitive:** `physics::beacon` — `Beacon::from_digest` (a 32-byte
  chronicle digest), `draw` (domain-tagged, exact-uniform over `rationals ∩
  [0,1)` at 2⁻¹²⁸ grain), `bernoulli(p: Ratio)` (exact comparison, no
  rounding), `uniform_index(n)` (rejection-sampled — exactly uniform,
  no modulo bias).
- **Witness:** `beacon_pins.rs` — determinism (same digest ⇒ byte-equal
  draws), domain separation, pinned reference vectors, exact-bound
  checks.
- **Code law:** no game system rolls its own randomness; every draw
  routes through the beacon with a named domain tag, and the tag
  appears in the receipt that consumed the draw.

## 7. Observation and governance → the lens, the observer, the steward

Every console session reflects its civilization to `var/worlds/<world>/`:

| File | Role |
|------|------|
| **`chronicle.tlv`** | **Authority** — resume carrier, SEAL B |
| **`state.tlv`** | **Machine mirror** — committed orb wire |
| `world.card` | World/planet identity card (TLV) |
| `solves/*.sol` | Proof-of-solution certificates (TLV) |
| `inbox/*.dir` | Steward directives → applied/refused (TLV) |
| `journal.jrn` | Year voice (framed TLV) |

**JSON is not architecture** — the system does not write, load, or migrate
`*.json` / `*.jsonl`. Full law: **`docs/JSON_DISPLAY_ONLY.md`**.

Quantities render exact (`9/2`, never `4.5`).

- **Primitive:** `scribe::record` / `scribe::record_solve` for the
  write side, `ledger::Ledger` (`pending_directives`, `archive`) for the
  steward inbox. (Formerly `mirror::Mirror`; `engine/mirror` was excised
  2026-07-31 — d606b01 — and mirror is deprecated as a framing whole,
  12 §2c. A lens derives from the one chain and holds no copy.) The lens
  is a derivation, never an authority — **nothing reads a display file back
  into a world**; replay of `chronicle.tlv` grounds worlds. Persistence
  and observation share one chain: world = replay(chronicle).
- **Tools:** `tools/observer.py` — read-only by construction;
  civilization-centric dashboard (resources, zones, clock, puzzles,
  the intelligence ratio `puzzles / resources-collected`, per-stream
  chain-linkage check that reads `✗ BROKEN` on any splice). 
  `tools/steward.py` — the steward program, scope-bound: writes
  directive files only. **The steward law: a directive becomes real
  only through the game loop as a scope-checked NS-6 action emitting
  an ordinary attributed receipt; the steward never edits state, and
  `revoke` shuts it out instantly.** Signature hardening on directive
  files is a named seam for the first multi-user landing.
- **Witness:** `scribe::journal_pins` / `rotation_pins` (faithful and
  incremental derivation, chain linkage in the stream) and
  `ledger_pins` (directive round-trip + archival, exactness in
  the snapshot). Formerly `mirror_pins.rs`, excised with its crate.
- **Chartering conserves (P1):** founding a new planet moves reserves
  from the source colony and mints nothing — conservation-by-movement
  is a substrate claim, not a rule of thumb. Primitive:
  `world::chart` / `chart_as` (Build scope). Witness:
  `world_planets::chartering_moves_the_reserves_and_mints_nothing`.
- **coder-02 seam:** the CLI wires `scribe::record` after each command
  and drains `pending_directives` per tick via the `as steward` path;
  the steward entity exists per-world with operator-granted scopes.

## 8. The topology and the registry — civilization → star → planet → colony

**Ruled by the operator 2026-07-25; star/cardinality ruling 2026-07-26.**
A civilization (species-level identity: "humans", "deep-chorus") spans stars.
A star — a star system named for its sun ("sol") — is the simulation
container: a collection of planets bound to one sun, sharing one clock, one
chronicle, one receipt chain (= one replay unit today, one mesh/consensus unit
at N6). This tier was formerly named "world"; it is renamed **star** because
the sun is what the planets orbit and what fixes their thermal regime
(`05_THERMAL_BALANCE.md` §1), and each star carries a `StarClass`
(NS-6 §7). A planet ("mars") is a settled body orbiting the star; a colony is
a civilization's settlement on a planet.

**Cardinality law (target, NS-6 §6):** `civilization 1:N star 1:N planet
1:N colony` — a star holds many planets, a planet hosts many colonies
(including colonies of different civilizations on one body). Today the runtime
cardinality is still degenerate (1★ = 1 planet = 1 colony) — the containers
are named now so they can grow at D3 (terrain/planets, multi-colony) and N6
(shared stars, mesh) without renaming. The chronicle stays per-star, never
per-planet. "World" is retired as a container term; where it appears
colloquially it means *a planet*, never the container.

- **Registry (`var/registry/<sigil>.civ` TLV):** who exists. Entries are
  created by players at founding (join the directory or add to it):
  name, species (free text — the encyclopedia grows itself), motto,
  the three genesis axes from the closed vocabulary (Ch. 2 §1), and
  the founder. **Identity only until G1**: recorded, displayed,
  joined; changing no rule. When G1 gives the axes mechanics, the
  entry is re-committed as a genesis receipt and becomes binding.
- **The registry is never Ground truth (04 §5, binding):** once
  genesis receipts exist, the registry is a browsing cache of
  receipted truth — no mechanic may ever read the registry where a
  receipt exists (an unreceipted side-channel replay cannot reproduce
  is SEAL B dying for identity). The directory is for browsing; the
  chronicle binds.
- **Join/amend (04 §5):** anyone may join a civilization; only its
  founder amends its entry, until G1 rules collective governance.
  (The current `Registry` API is create/load/list only — amendment
  does not exist yet, so the rule is enforced by absence; an `amend`
  gated on founder lands only with a consumer.)
- **Naming debt (04 §5, registered; updated 2026-07-26):** the engine
  struct `World` and the `worlds/<slug>/` path today mean "one colony
  sim"; the container tier is now named **star**. The struct renames to
  `Star` and the path to `stars/<slug>/` at the multi-planet landing,
  before N6 (see NS-6 §6). Code identifiers are untouched until that
  landing.
- **Primitive:** `ledger::registry::Registry` (`create` with typed
  refusals `AlreadyRegistered`/`UnknownAxis`, `load`, `list`);
  `scribe::record(world, planet, civilization, &World)` writing
  `worlds/<world>/world.card` + `planets/<planet>/state.tlv`;
  directives (`.dir`) and solves (`.sol`) at world altitude.
- **Witness:** `ledger_pins` (topology layout, per-world chain,
  registry vocabulary enforcement).
- **coder-02 seam:** founding becomes the character screen — pick or
  create civilization (registry), pick or create star (system), name the
  planet, then the normal `found`. Axis prompts offer exactly the
  closed vocabulary; species/motto free text.

---

## Merge discipline

1. coder-02 commits N3 on `main`.
2. coder-01 rebases `worktree-parallel-agent` and lands **N-RFA** (the
   two crates + this contract + the beacon) as one board entry on
   operator go.
3. World-Book Ch. 2 lands citing §1–§6 by name.
4. Any change to a primitive's semantics requires updating the witness
   and this contract in the same landing — a contract/witness mismatch
   is a red build by policy.
