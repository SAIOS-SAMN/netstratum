# 53 — THE INTELLIGENCE PORT: ONE REDUCTION, TWO SEATS

**The ruling that makes the entities intelligent BY DESIGN rather than the
physics deciding for them. Downstream of `00_CHARTER`, `51_COGNITION`,
`52_COGNITIVE_METABOLISM`; binding on `iae`, `solver`, `cfe`, and the conductor's
`research_step` / `build_step`. Operator ruling 2026-08-17 (IPSAL v0.4 /
`R_SOLVER_DIRECTION` shape, translated — not transcribed).**

The port supersedes the half of `51_COGNITION` §1 that calls the Mind a
*stochastic, float-tolerant proposer that guesses candidate generators*. That
half is the costume. **Search, utility scores, and Policy `if/else` are the
costume.** An engine that *enumerates* or *samples* candidates and lets an exact
verifier dispose is brute force wearing cohomology; a partition-draw over
weighted candidates is the same sweep-and-pick with a prettier weight. Both are
retired here. The Universe half of §1 — exact `ℚ/ℤ`, deterministic, `kernel::admits`
and solver-verify commit only what conserves — **stays, absolute and unchanged.**

## D0 — THE LAW OF THE PORT (binding)

1. **One reduction.** peel → canonicalize-and-difference → drain against grown
   crystals → decline through a deduced frame → compose only on remainder
   support. Glue across observations. Express or withhold. The **same function
   at both seats** — ARC and survival are one `think`.
2. **Need orders; observation supplies.** Hunger may rank which book or which
   family is read first. It may NEVER invent an operator, a zone, or a ration the
   observation does not name.
3. **A thought carries Φ⁻¹ or it is not a thought.** A survival act that cannot
   invert (raze what was built, return the ration drawn) is refused *typed*.
4. **Thinking changes the thinker.** Every drain that closes joins that entity's
   `grown_basis`; the next think seeds from it. Nothing minted is discarded.
5. **No candidate loop.** `iae::proposer` and `solver::search` leave the live
   beat. `search` survives ONLY as a certificate instrument for an exhaustive
   proof-of-absence at a novelty gate — never as how a mind thinks.
6. **No float in either seat.** The `51 §1` "float-tolerant Mind" is retired.
   Exact `Ratio` / `BigInt` only.
7. **No hardcoded solutions.** No branch on task id, species name, or zone enum
   as destiny. Species identity is the ration (`metabolic_ration_of`), not a
   policy table.
8. **Same doors.** Sovereign at the CLI, automaton in `research_step` /
   `build_step`, a contained NS-6 agent: one `think`. No shadow planner.

## D1 — TWO SEATS, ONE ORGAN

```
                 iae::think (four moves + glue)
                  /                          \
         SEAT ARC                          SEAT SURVIVE
  grid pair → peel → TaskThought      book pair → peel → TaskThought
  express = Program + certificate      express = Deed (zone/collect/raze/rest)
  verify  = solver-verify::verify_work verify  = kernel::admits
                  \                          /
                   grown_basis on the entity
                   sun / vocabulary::fold on the civ
```

`think(observations, entity, membrane)` owns no algebra. Each seat only
transduces wire → fiber and renders a thought → Program or Deed. There is no
`solve_arc` and no `choose_zone` as two algebras.

## D2 — LIFT `iae` TO THE ORGAN SET (do this FIRST)

`engine/iae` today holds: `transport/` (Φ/Φ⁻¹ carried together), `cohomology.rs`
(`reduce_cochain`, the SNF quotient — the current drain), `resolve.rs`
(`Binding` = coefficients + torsion + residual + locked — the current
remainder), `mind.rs` (three strata Harmonic ⊂ Luminal ⊂ Holonomic),
`proposer.rs` (`propose_and_verify`, the seeded sampler). Missing: peel cover,
congruence chain with inverse, asymmetric drain, conjugate-on-support, sentence
compose, observation, thought manifold, grown basis.

| Organ | Lands in | Replaces / consumes |
|---|---|---|
| `peel` | `iae/src/peel.rs` | a `FullMeasurement` from a grid or a book |
| `congruence` | `iae/src/congruence.rs` | `ChainDelta` + inverse → signed remainder |
| `reduction` | `iae/src/reduction.rs` | `reduce_pair` / `Remainder` — REPLACES `cohomology::reduce_cochain` as the think DRAIN; SNF stays only as the kernel verifier if it still gates commits |
| `conjugate` | `iae/src/conjugate.rs` | `relate_chains` → DECLINE |
| `sentence` | `iae/src/sentence.rs` | COMPOSE-on-demand |
| `observation` | `iae/src/observation.rs` | one pair, cheap, mint-at-wall |
| `think` | `iae/src/think.rs` | the seat conductor; owns no algebra |
| `grown_basis` | on the NS-6 `Entity` (`cfe/ipsal.rs`) / polity mind | minted residuals as crystals |

`FullMeasurement` **does not exist today** (iae carries `Vec<BigInt>` inside
`Binding`); it is the common carrier both seats peel into — introduced in N-I1.
**Delete as a think path:** `iae::proposer::propose_and_verify`. Condensation is
`observe` minting a generation-gap wall into `grown_basis`, not N guesses until
SNF says yes. **Keep:** solver-verify replay; `vocabulary::fold`; `mind.rs` strata
as *where a crystal sits* (the holonomic shell is a compose-arity ceiling), never
as a second reducer.

## D3 — ARC SEAT: replace the live solve

Today `research_step` (conductor `lib.rs:2785–3006`) runs `forge_task(digest)` →
library fold → `pre_reach` → `propose_consulted_measured` (`solver/search.rs:511`,
the sector-door sampler). After the port:

```
forge_task(digest)          stays (curriculum from the chronicle)
  → peel each train pair     iae::peel
  → think(grown_basis, sun)  four moves
    Glued      → render ThoughtRule → Program → verify_work → mint (38 §4)
    Unglueable → the residual is the next observation (same unique, next beat)
    Withheld   → rest this beat (honest)
```

- **D3.1** `solver::think_task` is the ONLY producer `research_step` may call.
  `solve` / `sweep_vocabulary` / `propose_consulted_measured` leave the live beat.
- **D3.2** `derive.rs` is NOT the thinker — it is a hint family the DERIVE move
  may consult as "what the pair's own difference already is." Recolor/tile/crop
  stay as readings of the observation, never a 14-op menu.
- **D3.3** Need orders families (novelty / resonance / `civic_defect` shape). It
  does not widen a beam. The bound is remainder support + `cognitive_capacity`
  (already on the polity), not a `SEARCH_BUDGET` (already decommissioned).
- **D3.4** `pre_reach(library)` IS REDUCE against crystallized words — the first
  arm of `think`, not a special case in front of a search.
- **D3.5** The volumetric arm is the same `think` on a 3-form peel; `perception
  == "volumetric"` is a lens, not a second solver.
- **D3.6** Forged tasks TRAIN; they do not mint value. The `38 §4` sector door
  (unique · productive · conserved · convergent) is the only mint, unchanged.

**Pins A–D:** (A) a task whose rule is the frame difference closes at DERIVE with
empty `grown_basis`, no sweep, certificate verifies. (B) a residual that blocked
think at K is a crystal at K+1 on the SAME entity; the same pair class closes
ByDrain, no second vantage, no mint. (C) two offices, two independent closures,
same orbit → `VocabularyMinted`; one vantage twice does not. (D) nothing closes →
withhold, no grid emitted, `verify_work` not called.

## D4 — SURVIVAL SEAT: books are the observation

Entities cannot account because there is no pair: buildings eat (`zone_upkeep`
substitutes `metabolic_ration`), **people do not** — `labor_force` is a work
input, not a mouth. `build_step` is Policy + `cognition::settlement_choice`
utility + a `lookahead::project` clone. Basic accounting is a coboundary. Each
beat, per living entity, form book pairs (have | owe):

| Book | Have | Owe | Residual means |
|---|---|---|---|
| Ration | banked+accrued of `metabolic_ration_of(species)` | mouths × one-beat ration | famine / forage / habitat / rest |
| Thermal | `syntropy_contained` | `entropy_vented` of standing zones | regenerator / encase / withhold order-work |
| Labor | fed habitats + clones | zone crew upkeep | raze drain / raise habitat |
| Matter | iron / alloy / element stocks | next deed's cost | smelt / synth / prospect / withhold |
| Leisure | refined surplus after ration closes | mint-cost of standing ARC void | think ARC / build capacity / rest |

- **D4.1** Land `41_PROVISION §2` as a MEASUREMENT: `labor_force` is a mouth,
  demand = mouths × species ration. A line whose ration book does not close
  cannot think ARC this beat (52) — but survival think STILL runs; that is how it
  names the cure.
- **D4.2** Peel a book pair the way a grid pair is peeled — two columns of exact
  quantities at named sites. `FullMeasurement` is the common carrier; do not
  invent a second measurement type.
- **D4.3** think the ration pair FIRST (Maslow is leftover order, not a score).
  Ration Withheld/Unglueable → the deed is the residual's name (collect, raise
  Habitat, raze a drain, forage). Ration Glued (have covers owe; inverse = the
  draw just priced) → surplus may open Matter, then Thermal, then Leisure/ARC.
- **D4.4** DELETE `settlement_choice` / `industrial_choice` / `score_outcome`
  (`cognition.rs:36/78/113`) as deciders. Their readings (`iron_gap`, alloy-vs-iron,
  famished veto) become OBSERVATIONS fed to think. The famished veto is not a
  special case — a projection that starves is a ration pair that refuses.
- **D4.5** `lookahead::project` clones are not thought. If a deed's inverse cannot
  be priced from the current books, withhold. Do not simulate a future world.
- **D4.6** Regenerator-first / famine-freeze-build / encase-first survive ONLY if
  they re-derive as the unique glue of the thermal/ration pair. If still
  hardcoded arms, re-express or delete.
- **D4.7** Species identity enters ONLY as which ration column is peeled
  (biomass / iron / joule / element).

**Pins E–H:** (E) one habitat, two extractors, biomass residual ≠ 0 → think names
Habitat or raze-Extractor, never a third Extractor, no disposition dial. (F)
ration closed, alloy > iron → Matter residual names Synthesizer (inverse = the
alloy draw); Smelter is the inverse thought when iron > alloy. (G) ration
unclosed → `research_step` does not fire; the chronicle records a withhold, not a
failed sweep. (H) collect is the identity thought on a positive accrued
workspace — if accrued covers the ration residual, DERIVE closes as collect, no
vocabulary consulted.

## D5–D6 — ENTITY, NOT CIV-AVERAGE; METABOLISM PRICES THOUGHT

- **D5.1** `grown_basis` lives on the ENTITY (sovereign / agent / NPC seat,
  `cfe/ipsal.rs` — today only `entity_id + handle + principal + grants`), not only
  on the polity. The civ library is the SUN (consilience, `vocabulary::fold`); the
  vessel basis is fluid.
- **D5.2** An automaton `research_step` may REDUCE against the sun; it may not
  install. Two vessel deposits crystallize (fold / sun) — same polarity as
  xylarium L0.
- **D5.3** A granted agent thinks only inside its scopes: a Collect-only agent may
  DERIVE collect but may not render a Zone deed. Containment precedes the RENDER,
  not the measurement.
- **D5.4** The unique lens is lineage + K-path seed on perception (gauge), never on
  the mathematics — consilience needs the disagreement.
- **D6** Every think is a receipted debit (`52`): baseline ration for
  observe/reduce, leisure for mint/compose; `kernel::admits` sees it. Habitat idle
  restores leisure (rest, not a score). `cognitive_capacity` is the holonomic
  shell: compose arity ≤ seated apparatus (`HolonomicShellFull` refuses beyond);
  inner vessel crystals grow freely. `civic_defect` / `instinct::blend` may ORDER
  which leftover is observed — never pick a program or a zone.

## D7 — WIRE THE BEAT

`research_step` and `build_step` become two renders of one cycle, per entity, per
beat: (1) peel survival books; (2) think survival; (3) Glued → express deed →
`admits`, remainder → carry as next observation, Withheld → rest; (4) if ration
Glued AND leisure ≥ mint-cost → peel the forge/unique pair, think ARC, sector
door; (5) deposit crystals/voids; (6) trajectory observes the K-step (a READ of
closed-vs-attempted, never a search router). One zone or one solve per beat is a
metabolic bound (one receipt), not a design dial.

## D8 — LIFT vs REFUSE

**Lift (shape, not code, from `/opt/ipsal-autic-v0.4` — READ-ONLY, no Cargo
path-dep):** four-move refusal order; TaskThought glue/unglueable/withhold;
observe mint-at-wall → grown_basis; remainder routes (generation gap /
composition gap / frame shift); inverse-carry on every transport; honest
withhold; consilience ≥ 2; communication (emit/refract/assimilate) LATER.
**Refuse:** `substrate::Multivector` as a second ground (netstratum has rfa-core +
Ratio); a 17-letter table (use `Op`/Cayley/`vocabulary::fold`); `iae::proposer`
sampling, `search::solve` on the beat, `MAX_DEPTH`/`TOP_N`; Bevy / LLM narrator /
NPC-as-OS-process; folding forage leftovers into the ARC curriculum; 41/42 as a
second brain (lifespan is a clock on the mouth count).

## D9 — LANDING ORDER (serial, red-first, ONE organ per landing, NO bundling)

| ID | Landing | Done when |
|---|---|---|
| N-I0 | THIS doc + amend `51 §1` | doc exists; 51 contradiction closed |
| N-I1 | `iae::peel` + `reduce_pair` on `FullMeasurement`; drain census vs v0.4 | Pin A skeleton compiles; census exists |
| N-I2 | `observe` + entity `grown_basis` | Pin B |
| N-I3 | `think` four moves + glue; `solver::think_task` consumes it | Pin A green |
| N-I4 | `research_step` calls only `think_task`; search off the beat | forge still verifies; sweep counter = 0 on the live path |
| N-I5 | book peel; mouths eat (`41 §2`) | **LANDED** `641470e` · Pin G |
| N-I6 | survival think names a deed; delete the utility deciders | **LANDED** `fd7bbcc` · Pins E, F, H · [`archive/N-I6.md`](archive/N-I6.md) |
| N-I-VESSEL | basin per vessel (workspace, not `Entity`) | **LANDED** `4399627` · Pin B at vessel grain · [`archive/N-I-VESSEL.md`](archive/N-I-VESSEL.md) |
| N-I7 | metabolic debit on the LOOK; leisure gates the observe | **LANDED** `c079dfd` · [`archive/N-I7.md`](archive/N-I7.md) · unfunded line fasts; funded Fed line thinks |
| N-I8 | sun / `vocabulary::fold` from two vessel deposits | **LANDED** `ab01400` · [`archive/N-I8.md`](archive/N-I8.md) · Pin C (two offices mint), C− (one vantage does not) |
| N-I9 | same doors: CLI think verb = automaton path; NS-6 grant gates render | **waits** — product parity, not converse |
| N-I10 | refraction between entities | **LANDED** `3099b80` · [`archive/N-I10.md`](archive/N-I10.md) · peer drains or walls a folded word (primitive + steward swap) |

**Do NOT start N-I10, lifespan, or dogma until N-I6 is green. A social mind on a
body that cannot count is fiction.**

## D10 — FILE MAP (write here, not elsewhere)

`engine/iae/src/{peel,congruence,reduction,conjugate,sentence,observation,think}.rs`
= the organ. `engine/iae/src/proposer.rs` = retire from think, leave dead until
deleted. `engine/solver/src/think.rs` = the ARC arm (Task → measurements → think →
Program). `engine/solver/src/search.rs` = off the beat. `engine/solver/src/derive.rs`
= DERIVE readings only. `engine/solver/src/need.rs` = orderer only.
`engine/solver-verify/` = untouched Universe. `engine/cfe/src/books.rs` + world
labor/ration = have/owe columns. `engine/conductor/src/lib.rs` `research_step` /
`build_step` = renders. `engine/conductor/src/cognition.rs` = delete as decider;
keep disposition only as lens metadata. `engine/cfe/src/ipsal.rs` = entity
`grown_basis` store. `engine/cfe/src/vocabulary.rs` = sun fold.

**`solver` still does not import `cfe`.** Book peel lives in `cfe` (or conductor);
it emits `iae::FullMeasurement`. Solver thinks measurements, not worlds.

## D11 — STANDING REFUSALS WHILE IMPLEMENTING

No third crate path-dep on ipsal-autic. No GPU-offloaded proposer (there is no
proposer). No `search` "just for hard tasks" — hard is a remainder; turn the lens
or wait. No `if famine { build Habitat }` — that is the Policy being deleted. No
`tuned_constant_ratchet` growth — gaps are observations, not new dials. No
narration of inner lives — receipts stay leftovers (`ration·short·habitat`,
`pair·drain·word`).

---

**Intelligence is seated when Pin B and Pin E are both green:** a mind that minted
a wall on a unique recognizes it on the next unique, and a mind whose biomass book
is short builds or razes from that residual, not from a score. Everything else is
making the drain as strong as v0.4 and letting a second vantage hear it.
