# THE SUBSTRATE CONTRACT — mechanic → primitive → witness

**Status: proposed, coder-01. Binding once ratified by the operator and
cited by World-Book Ch. 2.** This document pins the vocabulary layer
(coder-02: docs, world rules, CLI) to the substrate layer (coder-01:
`engine/rfa-core`, `engine/physics`) so the two nodes cannot silently
diverge: every mapping below names the mechanic, the exact primitive
that implements it, and the witness test that goes red if either side
drifts.

Ownership (CHARTER §10): coder-02 owns `docs/` (except this document),
`engine/cfe/src/world.rs`, `clients/cli/`. coder-01 owns
`engine/rfa-core/`, `engine/physics/`, this document
(`docs/03_SUBSTRATE_CONTRACT.md`), and the workspace members line.
Cross-territory changes travel by relay through the operator, never
directly.

---

## 1. Perception basis (the Hail Mary axis) → coboundary

A civilization's measurement frame IS a coboundary: two civs observing
the same world in different bases hold Deltas differing by exactly a
frame distortion `Δ_ij += p_j − p_i`.

- **Primitive:** `rfa_core::coboundary_reduce` — H¹ projection strips
  the frame; the canonical representative is the basis-invariant
  content. Translation loss is the reduction residual: measured,
  displayed, never silent.
- **Witness:** `rfa_vectors::coboundary_shift_does_not_change_the_class`
  (two frames, one class), `a_pure_coboundary_reduces_to_zero` (a
  message that is ALL frame carries no content).
- **Ch. 2 obligation:** specify translation AGAINST these semantics —
  cite the function, do not re-derive the concept.

## 2. "Discovery needs a stranger" → class equality across distinct frames

Consilience (v0.4 doc 14): two identical lenses verify nothing. Two
civilizations confirm a discovery iff their independently measured
Deltas reduce to the SAME canonical representative from measurably
DIFFERENT coboundaries.

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

"My waste stream is your food supply" is one tensor entry read in two
directions: `Δ_ij = −Δ_ji`. The mineral/biotic inversion is not two
ruleset tables kept consistent by hand — it is one antisymmetric
relation that cannot desync.

- **Primitive:** `rfa_core::Delta::set_antisym` (the only lawful
  writer), `Delta::conservation_measurement` (a measured DISTANCE to
  the invariant surface, zero iff exact).
- **Witness:** `rfa_vectors::constructed_deltas_conserve_exactly`,
  antisymmetry asserts inside
  `reduction_lands_the_canonical_integer_representative`.

## 5. Shock, stress, recovery (stochastic elasticity) → torsion, coherence, gates

A shock injects torsion into the colony Delta; the visible stress
gauge is the coherence functional; recovery is a sequence of gated
convergence steps.

- **Primitive:** `rfa_core::coherence_functional` (`C = Σ R²`, exact ℚ,
  zero iff converged — the dashboard number);
  `rfa_core::for_each_residual` (WHERE the stress lives, per triple,
  per coordinate); `rfa_core::validate_step` (typed refusals:
  `ConservationViolated`, `MagnitudeDrift`, `Unproductive` — recovery
  steps must strictly reduce C).
- **Witness:** `elasticity_witness.rs` — the full loop: coherent
  colony → shock → measured stress → gated recovery → convergence.

## 6. Randomness (the elasticity trigger) → the chronicle beacon

**LAW (CHARTER §9, inscribed by coder-02; staged with the same
operator go as this contract):** "stochastic" means chronicle-seeded,
receipted, replayable randomness — a draw derives from the previous tick's receipt
digest by exact arithmetic; probabilities are rational; no float, no
ambient entropy, no wall clock ever enters. Replay (SEAL B), mesh
verification (N6), and anchoring (N7) survive untouched.

- **Primitive:** `physics::beacon` — `Beacon::from_digest` (a 32-byte
  chronicle digest), `draw` (domain-tagged, exact-uniform over `ℚ ∩
  [0,1)` at 2⁻¹²⁸ grain), `bernoulli(p: Ratio)` (exact comparison, no
  rounding), `uniform_index(n)` (rejection-sampled — exactly uniform,
  no modulo bias).
- **Witness:** `beacon_pins.rs` — determinism (same digest ⇒ byte-equal
  draws), domain separation, pinned reference vectors, exact-bound
  checks.
- **Code law:** no game system rolls its own randomness; every draw
  routes through the beacon with a named domain tag, and the tag
  appears in the receipt that consumed the draw.

## 7. Observation and governance → the mirror, the observer, the steward

Every console session reflects its civilization to `var/worlds/<civ>/`:
`state.json` (atomic-replace snapshot), `chronicle.jsonl` (append-only
receipt stream carrying the digest chain and the normative TLV bytes),
`solves/` (proof-of-solution certificates), `inbox/` (steward
directives; processed ones archive under `applied/`/`refused/`, never
deleted). Quantities render exact (`9/2`, never `4.5`).

- **Primitive:** `mirror::Mirror` (`record`, `pending_directives`,
  `archive`, `record_solve`). The mirror is a REFLECTION, never an
  authority — nothing reads `state.json` back into a world; replay
  grounds worlds. The same stream is the persistence carrier: world =
  replay(chronicle), so save/load and observation are one seam.
- **Tools:** `tools/observer.py` — read-only by construction;
  civilization-centric dashboard (resources, zones, clock, puzzles,
  the intelligence ratio `puzzles / resources-collected`, per-stream
  chain-linkage check that reads `✗ BROKEN` on any splice). 
  `tools/steward.py` — the master control program on a leash: writes
  directive FILES only. **THE STEWARD LAW: a directive becomes real
  ONLY through the game loop as a scope-checked NS-6 action emitting
  an ordinary attributed receipt; the steward never edits state, and
  `revoke` shuts it out instantly.** Signature hardening on directive
  files is a named seam for the first multi-user landing.
- **Witness:** `mirror_pins.rs` (faithful + incremental reflection,
  chain linkage in the stream, directive round-trip + archival,
  exactness in the snapshot).
- **coder-02 seam:** the CLI wires `Mirror::record` after each command
  and drains `pending_directives` per tick via the `as steward` path;
  the steward entity exists per-world with operator-granted scopes.

---

## Merge discipline

1. coder-02 commits N3 on `main`.
2. coder-01 rebases `worktree-parallel-agent` and lands **N-RFA** (the
   two crates + this contract + the beacon) as one board entry on
   operator go.
3. World-Book Ch. 2 lands citing §1–§6 by name.
4. Any change to a primitive's semantics requires updating the witness
   AND this contract in the same landing — a contract/witness mismatch
   is a red build by policy.
