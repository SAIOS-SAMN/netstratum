# The world-book, chapter 2 — civilization genesis

**Status:** design, ruled 2026-07-25 (operator direction + coder-01
substrate mappings). This chapter specifies who plays: entity genesis, the
three civilization axes, and the perception mechanics. Every mechanic here
is stated against a named substrate primitive (`engine/rfa-core`,
`engine/physics`) per the parallel-node protocol (Charter §10) — cite the
function, never re-derive the concept. The binding mechanic → primitive →
witness table lives in `docs/03_SUBSTRATE_CONTRACT.md` (coder-01's, the
authority where this chapter and the code could disagree).

**The founding insight (the "Project Hail Mary" law):** in this universe,
*two identical minds cannot verify anything*. A discovery becomes real only
when two measurably different civilizations confirm it independently.
Diversity is not flavor — it is the fuel of proof. Proof needs the
stranger who speaks in 3D.

---

## 1. Entity genesis — the game now opens with creation

Before founding a colony, the player creates their civilization: three
choices, all committed as genesis receipts, all readable in the entity's
profile orb (NS-1/NS-6 — the profile is the civilization).

### 1.1 Axis one — disposition (what counts as progress)

Not classes with different rules; different *value weightings over the same
receipts* (exact rationals, summing to 1). Everyone plays the same physics;
what differs is which history counts as winning:

| disposition | scores | the play style it rewards |
|---|---|---|
| **Expansionist** | territory closures (zones built, sectors claimed) | wide, fast, hungry |
| **Empiricist** | verified discoveries (minted axioms, confirmations) | instruments, probes, alliances of difference |
| **Epicurean** | surplus consumed (wealth enjoyed, not hoarded) | burns what others bank — festivals as economy |

### 1.2 Axis two — metabolism (what is food)

The mineral/biotic split the colony loop already implies, made literal:
a mineral civilization metabolizes what a biotic one excretes and vice
versa. **Substrate:** one antisymmetric Delta entry per resource flow —
`Δ_ij = −Δ_ji` (`rfa_core::Delta`, contract §4). One side's waste and the
other's food are one tensor entry read from two ends; the two ledgers
cannot desync because they are not two numbers. Comparative advantage arises from biology, making trade
(N5) natural rather than bolted on.

**SUBSTRATE-1 (12_UNION §5a):** the vocabulary grows append-only —
**plasmic** joined mineral and biotic (plasma intelligences of the
stellar environs; sites in the Furnace regime, for the mesic band would
starve a current sheet). The antisymmetric Delta law holds unchanged: a
plasmic line's waste heat is a mineral line's smelter draft.

### 1.3 Axis three — perception basis (the Hail Mary axis)

Each civilization measures the world in its own frame: **planar** (the 2D
grid), **volumetric**, **harmonic** (frequency-basis — the civ that speaks
in music), and — SUBSTRATE-1, append-only — **umbral** (occlusion-basis:
the syzygial lines read the world by what blocks what; 12_UNION §5a). **Substrate:** a civilization's frame is a coboundary; two civs'
raw measurements of the same structure differ by coordinate distortion
only (contract §1). Translation between frames is
`rfa_core::coboundary_reduce` — the H¹ class is what survives translation;
what does not survive is the measured loss, shown to the player, never
silent. Misunderstanding an alien is a
typed refusal with its delta whole, not a dice roll.

## 2. The discovery law (class-equality-across-distinct-frames)

A discovery confirms when two civilizations' independently measured Deltas
reduce (`coboundary_reduce`) to the same canonical H¹ representative from
measurably different coboundaries (contract §2). Same class + same frame =
trivial confirmation, refused (the arity predicate is exact — an echo is
not a witness). Same class + different frames = the mint fires.

- **Discovery vs. trade is a type distinction, not a rule:**
  `physics::FlowState` events (contract §3) — `Transfer` is reversible
  with its exact carried inverse (a trade can be unwound); `Mint` is
  irreversible by construction (the step witness reads its inversion
  `Violated` — that is its teeth, not a defect). A discovery cannot be
  un-known.
- **Gameplay:** the Empiricist's whole arc is building the instruments and
  the relationships to get a second, different-frame measurement. A
  rival's confirmation is worth more than an ally's echo.

## 3. Stress, shock, and recovery

Crises (famine at civilization scale, disasters, war damage) are **torsion
injections** into the civilization's Delta (contract §5; randomness
contract §6 — every draw routes through `physics::beacon` with a named
domain tag). The visible stress gauge is
`rfa_core::coherence_functional` (`C = Σ |R|` — the sum of residual
magnitudes; squares were barred as aggregators of difference by the
proportional law, operator ruling 2026-07-31, and this line's earlier
"sum of squared residuals" is struck with it). Zero iff the cocycle
condition holds — the differences compose — never "iff converged"
(operator correction 2026-08-01): a civilization may read zero and
still carry its whole irreducible H¹ core, which is its genuine
difference and is never drained as stress. The substrate contract pins
the true reference values.
Recovery steps must pass `rfa_core::validate_step`'s three typed gates
(conservation, bounded magnitude, productivity) — a "recovery" that
doesn't measurably reduce stress refuses. Elasticity (absorbing a shock by
temporarily simplifying) is `physics::exhale`/`inhale` — lossless rank
reduction, the civilization drawing in its breath.

All randomness in shocks obeys Charter §9: chronicle-seeded, rational
probabilities, exact draws, bit-for-bit replayable.

## 4. What changes at the surface

- **Genesis flow (CLI):** `found` grows into a short creation dialogue —
  disposition, metabolism, basis, then the colony. Genesis receipts open
  the chronicle; the profile orb carries all three axes.
- **Scoring:** the status panel gains the disposition-weighted progress
  read — three civs on identical maps see three different "winning"
  numbers, exactly as intended.
- **Translation UI:** foreign content renders through the player's basis, with
  the measured loss visible ("harmonic bubble, 3/4 of structure survives a
  planar read — build a resonance array to close the gap").

## 5. The topology (ruled 2026-07-25, all three nodes aligned)

```
CIVILIZATION  who — a species-level identity ("humans", "deep-chorus");
              lives in var/registry/, SPANS worlds
WORLD         where — a collection of planets sharing ONE clock, ONE
              chronicle, ONE receipt chain (= one replay unit, one mesh
              unit at N6, one anchor root at N7)
PLANET        the settled body — terrain (D3), physical metabolism
COLONY        a civilization's settlement ON a planet (today's 8×8 grid)
```

Three bindings that keep this honest:

1. **The chronicle stays per-world.** One clock, one chain, all planets in
   a world share history — the world is the consensus unit. Never split
   per planet.
2. **The registry is a directory, never ground truth.** A registry entry
   is pre-genesis identity: convenient, readable, editable. The moment G1
   lands, the binding copy of a civilization's axes is its genesis receipt
   in the world's chronicle — the registry is a cache of receipted truth,
   and no mechanic may ever read the registry where a receipt exists
   (SEAL B discipline applied to identity).
3. **Registry entries are founder-amended only, append-only as a
   directory** — anyone may join a civilization (settle as it); only its
   founder amends its entry, until G1 rules collective governance. This
   keeps a shared directory from becoming an edit war.

**Naming debt, registered:** the engine struct `World` currently means
"one colony simulation" — the degenerate 1-world=1-planet=1-colony state.
The word "world" is hereby reserved for the container; the struct gets
renamed at the multi-planet landing (before N6), not before.

## 6a. The swarm register — eusocial and web-weaver lines (ruled 2026-07-30)

The operator seats the **Klethaxu** (eusocial ant swarms) as a founding
swarm archetype, with an arachnid web-weaver strain to follow at its own
wave. The register law (06) binds: myth + physiology here, and only what
ties to the engine is mechanically real.

**The axes carry it unchanged — no new values.** The vocabulary is
closed and grows only by landing (12 §5a); the swarm lines map onto what
stands: Klethaxu read **expansionist · biotic · planar** (the trail is
the thought — chemical gradients on the surface graph); the arachnid
strain will read **harmonic** (lattice tension across the web's edges);
Hivelight already holds **volumetric** (the comb computes). Three
collective architectures, three perceptions — the axes doing the work,
no "pheromonal" value needed. A distinct perception value remains
lawful only as a substrate-style landing with the first line that
cannot be read otherwise.

**The split, as register physiology (not mechanics):** the Klethaxu are
path-flow — tunnels, foraging streams, high-volume iron/biomass moved
along linear networks; the web strain is mesh-trap — boundary graphs,
tension lattices, joule/crystal capture at the perimeter. Function
readings stay derived (12 §4, FUNCTION-1): a tunneling extraction line
will read miner or weaver from its zones, never by declaration.

**SWARM-1 (candidate, design stage — nothing ships here).** The
mechanical ideas worth a future landing, held to Charter §1: sub-node
density (a colony as many micro-units per zone), parallel-probe
pathfinding offered to the forge/solver lane, swarm logistics. Every
efficiency claim is a placeholder parameter at that landing; no species
is exempt from the touch law (demurrage binds all — "near-zero transfer
loss" is not a register fact). Corrections recorded against the
proposing sketch: a people enters in the bands (tier reading derived,
Band III until the crossing) — no species archetype is seated on tier
0; and the solver tie-in is a research direction for the forge lane,
not a landed model.

## 6. Board increments (proposed, root-first)

| # | Landing | Contains | Blocked by |
|---|---|---|---|
| **G1** | Genesis + disposition — **landed 2026-07-30 (coder-03)** | tag 61 `GenesisSealed`: the three axes sealed as the binding world-chronicle receipt (§5 binding 2 — the registry is a cache from the receipt on; no mechanic reads it where a receipt exists); entry in the bands (§6a, never a tier-0 seat); once per world; unknown axis codes refuse typed naming the closed vocabulary; disposition weights (placeholder, exact rationals summing to 1 over territory/discovery/consumed) drive `scored_progress` — identical physics, different worth, None unsealed. v1 surface: the `genesis` CLI verb (the §4 creation dialogue composes at the console lanes); the `status` line renders the sealed line + scored read. Evidence: 7 witnesses green (`world_genesis.rs` — seal/replay/resume; re-seal refuses; unknown axis pure; NoSuchEntity precedes NoColony, the N3 law; scored reads diverge 13/5 vs 23/5 exact on one arc; unsealed reads None; tag 61 byte-exact); 209/209 workspace, clippy zero; arc played `--plain` | N-RFA ✓ (substrate landed) |
| **G2** | Metabolism tables — **landed 2026-07-30 (coder-03, the vent)** | ruleset v2 as the ration-substitution law over the shared flows (`dop::zone_yield`/`zone_upkeep`, keyed by the sealed metabolism — never registry data): every biomass food component reads the line's ration (mineral eats iron; plasmic eats joules), the habitat's biomass line is the farm slot (mineral farms iron; plasmic farms nothing — the fiat law, 05 §1.2/01 §6.1: joules enter only by encasement, so **plasmic agriculture is the thermal loop**); biotic and unsealed read v1 byte-identical (no silent constant change — measured); famine strikes and relieves on the true store; the status panel renders the line's own map. The waste↔food seam v1 is the ration complementarity over the landed trade machinery; the antisymmetric Delta integration composes with the substrate lane. Evidence: 4 witnesses green (`world_metabolism.rs` — v1 identity byte-exact; mineral accrues one store at +7/4 exact; the mesic band starves a current sheet on joule; the Furnace regime feeds it with surplus via encasement); 220/220 workspace, clippy zero. **Wave 3a seated with it:** Coronal Choir → progenitor domain-head (the first plasmic board-maker; tier 0 tri-planar on metabolism; +8 seats; the Tunborn fiat precedent; name passes the locality law — coronal place ‖ current-sheet medium ‖ choir manifestation) | G1 ✓ |
| **G3** | Perception + translation — **landed 2026-07-30 (coder-03)** | the translation law as a read-time annotation on the lens (NS-2 §4.2a — a recorded correction: the sigil is frame-free, the H¹ class is what survives translation, frames quotient out and join T outside the key); `translation_survival` exact (same basis 1 · the §4 example verbatim 3/4 · umbral pairs 1/2 · unsealed whole); the tag-57 record extends additively (old readers parse the prefix — SEAL A on the wire, measured); `translated_read` the pure N5-ready cross-frame seam; the CLI scan renders the §4 translation line with the cure named. Evidence: 5 witnesses green (`rqp_translation.rs`); 225/225 workspace, clippy zero | G1 ✓, N4 ✓ |
| **G4** | The discovery mint | class-equality-across-distinct-frames; trivial-confirmation refusal; Mint irreversibility | G3, N5/N6 (a second, different entity must exist to confirm) |

Sequencing note: G-landings interleave with the protocol board — G3 lands
with N4 (translation needs the scanner as its first consumer), G4 with
N5/N6 (confirmation needs a genuine second vantage). Law xvii holds
throughout: no genesis axis ships before the mechanic that reads it.
