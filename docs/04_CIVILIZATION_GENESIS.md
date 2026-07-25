# THE WORLD-BOOK, CHAPTER 2 — CIVILIZATION GENESIS

**Status:** DESIGN, ruled 2026-07-25 (operator direction + coder-01
substrate mappings). This chapter specifies WHO plays: entity genesis, the
three civilization axes, and the perception mechanics. Every mechanic here
is stated against a named substrate primitive (`engine/rfa-core`,
`engine/physics`) per the parallel-node protocol (CHARTER §10) — cite the
function, never re-derive the concept. The binding mechanic → primitive →
witness table lives in `docs/03_SUBSTRATE_CONTRACT.md` (coder-01's, the
authority where this chapter and the code could disagree).

**The founding insight (the "Project Hail Mary" law):** in this universe,
*two identical minds cannot verify anything*. A discovery becomes real only
when two measurably DIFFERENT civilizations confirm it independently.
Diversity is not flavor — it is the fuel of proof. You need the stranger
who speaks in 3D.

---

## 1. Entity genesis — the game now OPENS with creation

Before founding a colony, the player creates their civilization: three
choices, all committed as genesis receipts, all readable in the entity's
profile orb (NS-1/NS-6 — the profile IS the civilization).

### 1.1 Axis one — DISPOSITION (what counts as progress)

Not classes with different rules; different *value weightings over the same
receipts* (exact rationals, summing to 1). Everyone plays the same physics;
what differs is which history counts as winning:

| disposition | scores | the play style it rewards |
|---|---|---|
| **Expansionist** | territory closures (zones built, sectors claimed) | wide, fast, hungry |
| **Empiricist** | verified discoveries (minted axioms, confirmations) | instruments, probes, alliances of difference |
| **Epicurean** | surplus CONSUMED (wealth enjoyed, not hoarded) | burns what others bank — festivals as economy |

### 1.2 Axis two — METABOLISM (what is food)

The mineral/biotic split the colony loop already implies, made literal:
a mineral civilization metabolizes what a biotic one excretes and vice
versa. **Substrate:** one antisymmetric Delta entry per resource flow —
`Δ_ij = −Δ_ji` (`rfa_core::Delta`, CONTRACT §4). My waste and your food
are ONE tensor entry read from two ends; the two ledgers cannot desync
because they are not two numbers. Comparative advantage arises from biology, making trade
(N5) natural rather than bolted on.

### 1.3 Axis three — PERCEPTION BASIS (the Hail Mary axis)

Each civilization measures the world in its own frame: **planar** (the 2D
grid), **volumetric**, **harmonic** (frequency-basis — the civ that speaks
in music). **Substrate:** a civilization's frame is a coboundary; two civs'
raw measurements of the same structure differ by coordinate distortion
only (CONTRACT §1). Translation between frames IS
`rfa_core::coboundary_reduce` — the H¹ class is what survives translation;
what does not survive is the measured loss, shown to the player, never
silent. Misunderstanding an alien is a
typed refusal with its delta whole, not a dice roll.

## 2. The discovery law (class-equality-across-distinct-frames)

A discovery CONFIRMS when two civilizations' independently measured Deltas
reduce (`coboundary_reduce`) to the SAME canonical H¹ representative from
measurably DIFFERENT coboundaries (CONTRACT §2). Same class + same frame =
trivial confirmation, REFUSED (the arity predicate is exact — an echo is
not a witness). Same class + different frames = the mint fires.

- **Discovery vs. trade is a type distinction, not a rule:**
  `physics::FlowState` events (CONTRACT §3) — `Transfer` is reversible
  with its exact carried inverse (a trade can be unwound); `Mint` is
  irreversible BY CONSTRUCTION (the step witness reads its inversion
  `Violated` — that is its teeth, not a defect). A discovery cannot be
  un-known.
- **Gameplay:** the Empiricist's whole arc is building the instruments and
  the RELATIONSHIPS to get a second, different-frame measurement. Your
  rival's confirmation is worth more than your ally's echo.

## 3. Stress, shock, and recovery

Crises (famine at civilization scale, disasters, war damage) are **torsion
injections** into the civilization's Delta (CONTRACT §5; randomness
CONTRACT §6 — every draw routes through `physics::beacon` with a named
domain tag). The visible stress gauge is
`rfa_core::coherence_functional` (sum of squared residuals — zero iff
converged; the substrate contract pins the true reference values).
Recovery steps must pass `rfa_core::validate_step`'s three typed gates
(conservation, bounded magnitude, productivity) — a "recovery" that
doesn't measurably reduce stress refuses. Elasticity (absorbing a shock by
temporarily simplifying) is `physics::exhale`/`inhale` — lossless rank
reduction, the civilization drawing in its breath.

All randomness in shocks obeys CHARTER §9: chronicle-seeded, rational
probabilities, exact draws, bit-for-bit replayable.

## 4. What changes at the surface

- **Genesis flow (CLI):** `found` grows into a short creation dialogue —
  disposition, metabolism, basis, THEN the colony. Genesis receipts open
  the chronicle; the profile orb carries all three axes.
- **Scoring:** the status panel gains the disposition-weighted progress
  read — three civs on identical maps see three different "winning"
  numbers, exactly as intended.
- **Translation UI:** foreign content renders through YOUR basis, with the
  measured loss visible ("harmonic bubble, 3/4 of structure survives your
  planar read — build a resonance array to close the gap").

## 5. Board increments (proposed, root-first)

| # | Landing | Contains | Blocked by |
|---|---|---|---|
| **G1** | GENESIS + DISPOSITION | the creation dialogue; genesis receipts; disposition weights + the scored progress read | N-RFA (substrate lands first) |
| **G2** | METABOLISM TABLES | ruleset v2: per-civ upkeep/yield maps over the shared Delta; the waste↔food seam | G1 |
| **G3** | PERCEPTION + TRANSLATION | frames as coboundaries; the translation read with measured loss; the harmonic/volumetric bases | G1, N4 (the scanner is the first consumer) |
| **G4** | THE DISCOVERY MINT | class-equality-across-distinct-frames; trivial-confirmation refusal; Mint irreversibility | G3, N5/N6 (a second, different entity must exist to confirm) |

Sequencing note: G-landings interleave with the protocol board — G3 lands
WITH N4 (translation needs the scanner as its first consumer), G4 with
N5/N6 (confirmation needs a genuine second vantage). LAW XVII holds
throughout: no genesis axis ships before the mechanic that reads it.
