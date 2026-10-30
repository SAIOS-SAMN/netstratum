# N-I8 — the first shared word

**Lane:** intelligence port · **Status: LANDED `ab01400` — Pin C green**
**Charter:** [`53_INTELLIGENCE_PORT.md`](53_INTELLIGENCE_PORT.md) D3 Pin C · D1 (`sun / vocabulary::fold`)
**Companions:** N-I-VESSEL (per-vessel basin, seated) · N-I7 (look priced, seated) · `solver::mint::consider` · `vocabulary::fold`

*Markdown is instruction only. The engine reads the deposits and the fold.*

---

## 0 · One sentence

A word installs only when **two vessels independently close the same
orbit** (consilience) — one mind confirming itself three times no longer
mints; the second deposit is where culture starts.

---

## 1 · What this is not

| Refused | Why |
|---|---|
| `CONFIRMS_TO_MINT = 3` digests as consilience | Three digests from ONE automaton is one vantage agreeing with itself, not two minds converging. This cut RETIRES that as the mint condition. |
| Individuating the whole population | Seed exactly ONE sibling office (a second cognitive vessel). Wiring every mouth as a thinker stays a later cut. |
| Copying crystals between basins | Consilience is TWO INDEPENDENT closures of the same orbit — never A's crystal pasted into B. Each vessel reaches the orbit from its own basin, or it does not count. |
| A chat layer / hearing a word in another frame | N-I10. Never a message score. Culture starts at the second DEPOSIT, not a dialogue. |
| Un-ignoring the cleave / refine era pins | They wait on SURPLUS and RUNG, not on fold. Leave them ignored. |
| Re-pricing look or mint | The look still pays `observe_price` (N-I7); the word mint still pays `mint_cost`. Fold changes WHO must close, not the bill. |
| Touching `pre_reach` | It stays the FREE REDUCE against the sun (the folded vocabulary). Unchanged. |
| `vocabulary::fold` rewrite | Fold still reads the `SolverWordMinted` receipt and crystallizes to the sun. This cut changes when that receipt is EMITTED, not how it folds. |

---

## 2 · Why: one vantage is not consilience

Today a word mints when `word_pending` collects `CONFIRMS_TO_MINT = 3`
distinct task digests — but all three are the SAME automaton solving three
forged puzzles. That is one mind confirming itself; a coincidence repeated
is still a coincidence (`iae::think` already refuses a rule from one pair —
this is the same law one level up). **Pin C is consilience:** two DISTINCT
vessels, each from its OWN basin (N-I-VESSEL made the basins separate),
independently close the SAME orbit. That convergence is the first thing a
line holds that no single perspective invented — the seed of a shared
vocabulary, and therefore of culture. A word two vessels fold is the atom
of a society; one vessel's private residual is not.

---

## 3 · The model

```
- word_pending: Vec<(orbit, expansion, cited: Vec<digest>)>          // 3 digests, one automaton
+ word_pending: Vec<(orbit, expansion, deposits: Vec<(vessel_id, digest)>)>
+ mint when the deposits carry ≥ 2 DISTINCT vessel_ids               // consilience
```

- **`word_pending` carries vessel_ids.** A closure deposits `(vessel_id,
  digest)`; the mint fires when an orbit's deposits span **two distinct
  vessel_ids**. `CONFIRMS_TO_MINT = 3` (one-vantage digest count) is retired
  as the mint condition (a per-vessel dedup on digest still prevents one
  vessel double-counting one puzzle).
- **Seed one sibling office.** Each line registers ONE sibling cognitive
  vessel (`register_agent`, the same door that seats the steward), so every
  line has two thinkers and consilience is reachable. It thinks on its OWN
  basin (N-I-VESSEL) and pays its OWN look (N-I7). Not the whole population
  — one sibling.
- **Same mouth twice does not mint (C−).** One vessel closing the same orbit
  across many beats is still one distinct vessel_id — it never reaches two,
  so it never mints. A lone thinker builds no vocabulary; a word is social.
- **No weaker one-vantage mint.** Pending is a wait, not an install. Do not
  keep `CONFIRMS_TO_MINT` as a fallback for a lone line.
- **Sibling look must be payable.** Two `observe_price`s in one beat can
  starve founding. If `mint_loop` dies, stagger: automaton looks this beat,
  sibling the next — still two vessels, still one orb bank. Do not give the
  sibling a free look.
- **Independent closures only.** Each deposit is a vessel reaching the orbit
  from its own drained basin — never a crystal copied across. If B only
  "knows" the orbit because it read A's basin, it is not a second vantage.

---

## 4 · The deposit

`research_step`, for each seeded cognitive vessel that afforded its look
(N-I7) and drained its own basin (N-I-VESSEL):

```
Glued + verify_work → orbit = consider(program, library, standing)
                    → deposit (vessel_id, digest) into word_pending[orbit]
                    → if distinct vessel_ids in word_pending[orbit] >= 2:
                          mint_solver_word_as(automaton, orbit, bytes, cited)   // the deed, unchanged
```

The mint DEED (`SolverWordMinted`, cited = the digests) is unchanged — only
its firing condition moved from three digests to two vessels. `mint_cost`
still bills leisure at the mint; `vocabulary::fold` still reads the receipt
and crystallizes the word to the sun; `pre_reach` still reads the sun for
free next beat. The consilience test reads the non-committed `word_pending`
workspace; the receipt it emits is the committed truth.

---

## 5 · SEAL-B

- **The sibling office is a chronicled entity.** `register_agent` emits an
  `EntityRegistered` deed (already on the wire — the steward is seeded this
  way); replay reconstructs the sibling deterministically. No new deed type,
  no schema bump.
- **`word_pending` stays non-committed workspace** (a `Vec` on the polity,
  rebuilt each process from the chronicle's own receipts), now carrying
  vessel_ids. It never enters the digest; the MINT it triggers is the deed
  that does.
- **The mint receipt is unchanged.** `SolverWordMinted { orbit, expansion,
  cited }` — `seal_b_era_pins` / `mint_loop_pins` stay green.
- `Entity`'s schema is untouched (the sibling uses the existing entity
  record); `grown` stays the per-vessel workspace; Law II holds.

---

## 6 · File map

| File | Role |
|---|---|
| `engine/conductor/src/lib.rs` | `word_pending` carries `(vessel_id, digest)`; seed one sibling office at genesis (`register_agent`); each cognitive vessel thinks its own basin + pays its own look + deposits; mint at ≥2 distinct vessel_ids |
| `engine/solver/src/mint.rs` | `consider` unchanged (still reads a program → orbit); `CONFIRMS_TO_MINT` retires as the mint gate (or becomes the per-vessel dedup floor) |
| `engine/conductor/tests/` | Pin C (two offices, same orbit → `VocabularyMinted`) · C− (one vessel, same orbit twice → no mint) |
| `engine/cfe/src/vocabulary.rs` | UNCHANGED — `fold` / `solver_words` still read `SolverWordMinted` |
| `engine/iae` · `engine/solver` (organ) | UNCHANGED — consilience is a conductor tally, not a new reduce |
| cleave / refine era pins | STAY IGNORED — they wait on surplus and rung, not fold |

---

## 7 · Pins (done when these are green)

| Pin | Law |
|---|---|
| **C** | two distinct vessels each independently close the same orbit → one `SolverWordMinted` receipt; `vocabulary::fold` reads it into the sun. |
| **C−** | one vessel closing the same orbit twice (or thrice) mints NOTHING — one distinct vessel_id never reaches two. |
| **Independent** | the second closure is a vessel draining its OWN basin, not reading the first's crystal. No cross-basin copy. |
| **Fold reads the receipt** | `vocabulary::fold` / `solver_words` unchanged; the sun grows only from the minted deed. |
| **Look + mint still priced** | each vessel's look pays `observe_price` (N-I7); the mint pays `mint_cost`. |
| **SEAL-B** | the sibling is a chronicled `EntityRegistered`; `seal_b_era_pins` green; `Entity` schema untouched. |
| **mint_loop** | a rich two-office line still mints — the economy loop closes on consilience, not on one-vantage confirms. |

Do not start N-I10 or SOC until Pin C and C− are green. Leave the cleave /
refine pins ignored.

---

## 8 · Accept

```bash
# from engine/
cargo test -p conductor --test <consilience_pin> --test mint_loop_pins --test seal_b_era_pins
cargo test -p iae -p solver   # unchanged organ stays green
```

Prior witnesses stay green (`vessel_pins`, `n_i7_pins`, `survival_pins`,
`survival_gate_pins`, `conductor_pins`). A red `an_era_brings_the_board_alive`
or a `mint_loop` that stops minting means consilience is unreachable — the
sibling must be able to afford its look and close alongside the automaton, or
no line ever folds a word.

Idle when Pin C and C− are green (two offices mint; one vessel does not).
Culture starts here — when the second deposit folds, not when a chat layer
appears. N-I10 (hear the word in another frame) is the file after.
