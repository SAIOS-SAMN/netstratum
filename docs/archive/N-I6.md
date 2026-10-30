# N-I6 — the residual names the deed

**Lane:** intelligence port · **Status: IMPLEMENT — charter seated `c623f84`**  
**Charter:** [`53_INTELLIGENCE_PORT.md`](53_INTELLIGENCE_PORT.md) D4.3–D4.6 · D7 · Pins E, F, H  
**Companions:** N-I5 (`solver/src/survival.rs`) · `cognition.rs` (comes down) · `kernel::admits`

*Markdown is instruction only. The engine reads constructors and the beat.*

---

## 0 · One sentence

N-I5 peeled the ration book. This landing thinks it: the leftover
names one inverse-carrying deed, `kernel::admits` verifies, and the
utility deciders leave the live path.

---

## 1 · What this is not

| Refused | Why |
|---|---|
| `if famine { Habitat }` | Need orders the book. The residual names the deed. |
| A second `think_*` / `BookThought` | `reduce_pair` is the organ. ARC `think` stays the grid seat. |
| Two books as one `iae::think` train | Ration and matter are sequential seats, not two pairs of one unique. `think` still needs ≥2 same-class observations. Do not lower that gate. |
| `lookahead::project` on the live path | Clones are not thought (D4.5). |
| `soc::choose_build` as fallback | Digest draw is the costume Pins E/F exist to kill. |
| Fall back to `settlement_choice` when the named deed cannot pay | Withhold. An unaffordable residual is rest, not a score. |
| `solver` imports `cfe` | Conductor peels the world. Solver thinks measurements. |
| Entity `grown_basis` / SEAL-B write | Grown stays on Polity. |
| Vocabulary fold / un-ignore cleave·refine | N-I8. |
| Leisure debit / ARC mint gate | N-I7. |
| Thermal book / labor book / diet rewrite | Later. `#34`-class harvest/silicate stays shut. |
| Species name as destiny | The column is still the ration direction. |

---

## 2 · The N-I5 lesson is load-bearing

`cycle` today is `build_step → … → tick → ration_survives → collect → research_step`.

`build_step` runs **before** tick. Naming a deed there reads yesterday's
larder — the same false-starve that made vein-fed lines look empty
post-collect.

**The survival think and its express sit in the N-I5 window:**

```
tick → peel ration (same read as Pin G) → think survival → express ≤1 deed
     → collect → research_step only if Fed
```

`build_step` is no longer a second builder. One zone or one collect-as-cure
or one raze per beat is a metabolic bound, not a design dial. Pre-tick
`build_step` either moves into that window or becomes a no-op. Do not
leave Policy *and* think both raising a zone in one cycle.

The famine early-return (`!orb.famine.is_empty() { return }`) is the
freeze-build D4.1 strikes. Survival think **still runs** when the book
is Short — that is how it names the cure. Pin G still withholds ARC.

---

## 3 · Organ

`ration_reads` currently throws the remainder away and returns
`Fed` / `Short`. Surface the leftover.

| Seat | Peel | Close means | Residual names |
|---|---|---|---|
| Ration first (always) | mouths × species column, same as N-I5 | every mouth fed | nothing on this book — Need may open Matter |
| Ration Short, accrued covers the shortfall | accrued workspace vs the leftover | DERIVE identity | **Collect** (Pin H). The cycle collect *is* the deed. No zone. |
| Ration Short, accrued does not cover | standing habitat support vs mouths; extractor drain vs habitat | leftover on the **ration** fiber | **Habitat** or **raze Extractor** (Pin E). Never a third Extractor — that is the wrong column. |
| Ration Fed, Matter opened | alloy vs iron as two sites on the matter fiber | leftover direction | **Synthesizer** (alloy > iron) or **Smelter** (iron > alloy) (Pin F) |
| Settled orb, zero sites that *yield* the ration | owe one production site on the ration fiber; have none | leftover on the ration column | the unique `industry_relevant` zone whose `zone_yield` is that ration. Founding is this peel, not Policy bootstrap. |
| No inverse-priced deed | — | — | **Rest**. Chronicle a withhold, not a failed sweep. |

The render is a small closed table from leftover class → deed, the
survival twin of ARC `render_frame`. It does not consult disposition,
`Policy`, `utility`, or `instinct` weights.

**Inverse is typed (D0.3):** raze inverts build; collect inverts the
draw just priced. A deed that cannot invert is refused, not expressed.

`kernel::admits` is the verifier. Express through the doors that already
exist (`zone_as`, `raze_as`, the cycle `collect_as`). Do not invent a
shadow commit.

---

## 4 · Need orders books, not zones

`Need` / `instinct` return as **which book is observed**, never which
`ZoneKind` is built.

- A Short ration book is observed alone. Matter stays shut.
- A Fed ration book may open Matter. Thermal / leisure stay shut (N-I7).
- `instinct.rs` stays an ARC-family orderer. Do not point it at Habitat.

Hunger never invents a zone the leftover did not name.

---

## 5 · Deciders come down

Delete from the **live path** (and then from the file):

- `cognition::settlement_choice`
- `cognition::industrial_choice`
- `cognition::score_outcome`
- `lookahead::project` inside `build_step`
- `soc::choose_build` as the surplus picker this beat

Their *readings* (iron vs alloy, mouths vs habitats, accrued vs owe)
become the measurements in §3. Disposition / `Policy` caps /
`industry_relevant` remain **RENDER gates** (cannot express a deed the
books cannot pay, cannot raise a pathway the metabolism cannot touch).
They are not a second choice among viable zones.

Regenerator-first / encase-first / forge-starved-hold (D4.6) are not
this cut's books. They must **not steal a Short ration beat**. Do not
re-express the thermal tree. Do not delete it. A Short leftover
outranks those arms until a thermal book exists.

---

## 6 · File map

| File | Role |
|---|---|
| `engine/solver/src/survival.rs` | surface leftover; `think_survival` / deed render; no `cfe` |
| `engine/solver/tests/survival_pins.rs` | keep 6/6; add E / F / H at the measurement layer |
| `engine/conductor/src/lib.rs` | move express into the post-tick window; one deed; Pin G still gates ARC |
| `engine/conductor/src/cognition.rs` | delete the three deciders |
| `engine/conductor/tests/` | Pin E world · Pin F world · Pin H collect-identity · Pin G held |
| `engine/iae` · `engine/cfe/src/ipsal.rs` · harvest | shut |

`solver` still does not import `cfe`. Conductor peels (`labor_force`,
`sustenance` **after tick before collect**, `accrued_of` the ration
resource, zone counts, alloy/iron, `ration_of` → direction) and hands
exact numbers / `FullMeasurement`s to the solver.

---

## 7 · Pins (done when these are green)

| Pin | Law |
|---|---|
| **E** | one Habitat, two Extractors, ration residual ≠ 0 → think names Habitat or raze-Extractor. A third Extractor is never expressed. No disposition dial. |
| **F** | ration Fed, alloy > iron → Matter leftover names Synthesizer. iron > alloy → Smelter. Inverse is the draw just priced. |
| **H** | accrued covers the ration leftover → DERIVE closes as Collect. No zone. No vocabulary. |
| **G held** | unclosed book still does not call `think_task`. Rest, not a sweep. |
| **N-I4 held** | search off the beat. |
| **SEAL-B** | Entity schema untouched. |
| **mint_loop** | a Fed line still researches and mints. |

Do not start N-I7, N-I8, N-I10, lifespan, or dogma until E, F, and H
are green.

---

## 8 · Accept

```bash
# from engine/
cargo test -p solver --test survival_pins
cargo test -p conductor --test survival_gate_pins --test mint_loop_pins --test seal_b_era_pins
# plus the new Pin E / F / H conductor witnesses
```

Prior N-I5 witnesses stay 6/6 and 2/2. A red `an_era_brings_the_board_alive`
is the same class of bug as N-I5 — measure in the post-tick window.

Idle after this seats unless the next file is named. N-I7 is leisure
gating ARC, not this cut.

---

## 9 · Implement order (serial)

1. **Solver.** Keep `ration_reads` → `Fed`/`Short` (Pin G, 6/6). Surface
   the leftover. Add `think_survival` + closed `SurvivalDeed` render
   (Collect / Habitat / RazeExtractor / Synthesizer / Smelter / Rest).
   Measurement-layer E / F / H. No `cfe`.
2. **Cycle.** After `ration_survives`, before `collect`: peel →
   `think_survival` → express ≤1 deed through `zone_as` / `raze_as`.
   Pin H Collect expresses nothing extra — cycle collect *is* the deed.
3. **`build_step`.** Stop raising zones. Famine early-return comes down
   (that function only). `gather_for_forge` may stay pre-tick — it is
   not a scored zone. Regenerator / encase / forge-hold do not fire
   while this landing has no thermal book. Do not leave Policy *and*
   think both raising a zone.
4. **Deciders off the live path**, then delete
   `settlement_choice` / `industrial_choice` / `score_outcome`.
   `lookahead::project` and `soc::choose_build` lose their `build_step`
   callers. Do not delete `soc.rs`.
5. **Conductor pins** E / F / H worlds. Hold G, N-I4, SEAL-B, mint_loop.

**Founding trap.** Zero habitats ⇒ `labor_force == 0` ⇒ N-I5 Fed
(empty pair). If express is Rest forever, the era never builds a first
zone and `mint_loop` / `an_era_brings_the_board_alive` die. The
founding row above is the cure: peel owe-one / have-none on the ration
fiber. Do not revive extractor-first Policy to save the pin.
