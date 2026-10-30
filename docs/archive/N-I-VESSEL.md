# N-I-VESSEL — think as a person

**Lane:** intelligence port · **Status: implement — grain + isolation only**
**Charter:** [`53_INTELLIGENCE_PORT.md`](../53_INTELLIGENCE_PORT.md) · Pin B at vessel grain
**Companions:** N-I6 (`survival`, seated) · `iae::adopt` / `iae::think` · `Polity.grown`

*Markdown is instruction only. The engine reads the basin and the beat.*

---

## 0 · One sentence

The ARC basin moves from the line to the vessel — `vessel_id → Vec<Crystal>`
— so a person's wall is their own, and a sibling working the same puzzle
does not inherit it.

---

## 1 · What this is not

| Refused | Why |
|---|---|
| A chat layer / a message score | Conversation is a crystal another basin drains or walls — the same `reduce_pair`. Not this cut, and never a score. |
| Pricing a civ-average `grown: Vec<Crystal>` | A line-pooled basin is not a person. Pricing it (N-I7) does not compress think — it averages it. Compress first. |
| Two-vessel fold (`vocabulary::fold` from two deposits) | N-I8. Two deposits cannot fold until two vessels exist. This cut makes the two vessels. |
| Hearing a word in another frame | N-I10. |
| SOC groupings / shared objective | SOC-4/5. A society is shared leftover direction + a fold — both downstream of this. |
| Individuating the whole population into vessels | **Hold.** Seed vessel is the automaton. The map admits many; wiring mouths as thinkers is a later cut. |
| `HashMap` as the basin or the index | Law II (AlignmentConfirmed): HashMap in cognition is dimensional collapse. Forbidden. |
| Moving `grown` onto `Entity` | Entity stays empty — the basin is non-committed workspace, keyed by vessel. SEAL-B untouched. |
| Touching the survival book | Survival stays the line's ration book (N-I5/N-I6). Only the ARC think basin becomes per-vessel. |
| Leisure debit on think | N-I7. Speech must be leisure-priced or culture is free — but pricing comes after the vessel exists. |

---

## 2 · Why the line pool is not a person

`Polity.grown: Vec<Crystal>` (today) is a CIV-average: every beat's think
drains and grows one pool, so two entities on the same line share one
memory of every wall. That is not a perspective — it is a hive with one
head. A perspective is the accumulated basin of one vessel: two people who
peeled different puzzles grow different crystal bases, so `reduce_pair`
drains against different histories and they read the same puzzle
differently. Compressing think to the vessel is the precondition for
conversation (a crystal one basin drains that another walls) and for
culture (a word two vessels fold). It comes before pricing (N-I7) and
before the fold (N-I8): you cannot price a person's thought, or fold two
people's word, until the person's basin exists.

---

## 3 · The data model

```
- grown: Vec<Crystal>                     // the line pool (civ-average)
+ grown: BTreeMap<u64, Vec<Crystal>>       // vessel_id → basin (index only)
```

**Law II — maps.** AlignmentConfirmed: *no HashMap in cognition.* A hash
bucket flattens a structured key and discards the fiber. `Q/ℤ` lives
**inside** `Crystal` / `Exact`, not in the key. You do **not** need a
BTree because the rationals demand one.

| Allowed | Role |
|---|---|
| `Vec<Crystal>` | the basin — the organ still thinks a list of leftovers |
| `BTreeMap<u64, Vec<Crystal>>` | **index of mouths**, ordered by `vessel_id` (`Ord`). Deterministic iteration for replay. Not the thought. |
| `Vec<(u64, Vec<Crystal>)>` + `.find()` | also lawful if you would rather not open a tree this cut |

| Forbidden | Why |
|---|---|
| `HashMap<u64, Vec<Crystal>>` | hash addressing; iteration order is not a replay law |
| Hashing a `Crystal` / remainder / `Ratio` as a lookup key | that **is** the collapse Law II names |

`entry(vessel).or_default()` on the BTree is bookkeeping: which `Vec`
`think_task` receives. `iae::think` / `adopt` never see the tree.

- **Seed vessel** is the line's cognitive entity (`automaton`).
- **Isolation:** wall on V lands in `grown[V]` only; W on the same unique
  walls afresh.
- **Population individuation stays later.** Do not fold it into this cut.

---

## 4 · The drain

`research_step` (post-N-I6) drains one basin:

```
- None => solver::think_task(&task, &mut self.grown),
+ let vessel = self.automaton;                       // the acting cognitive vessel
+ None => solver::think_task(&task, self.grown.entry(vessel).or_default()),
```

`iae::adopt` / `iae::think` are unchanged — they still take `&mut
Vec<Crystal>`; the conductor hands them the vessel's mouth, not the pool.
The organ never learns there is more than one basin; the vessel grain lives
entirely in which basin the conductor drains. Pin B (a wall at K is a
crystal at K+1) now holds per vessel: the same class the same vessel meets
next beat drains; a sibling meeting it walls afresh (it never saw the
crystal).

---

## 5 · SEAL-B

The vessel basin is a decision input, not committed state — like the line
pool it replaces, like `library`. `Entity`'s schema is untouched; the map
never enters the digest; replay rebuilds it deterministically by re-running
the same beats (the basin is a function of the replayed think sequence, and
the keying is deterministic). A pre-grown receipt still replays byte-exact:
`seal_b_era_pins` must stay green with the map in place. If a vessel basin
ever needs to persist across a process (resume), it re-derives from the
chronicle exactly as the line pool did — no wire change this cut.

---

## 6 · File map

| File | Role |
|---|---|
| `engine/conductor/src/lib.rs` | `grown: BTreeMap<u64, Vec<Crystal>>`; drain the acting vessel's mouth; init `BTreeMap::new()` at both ctors |
| `engine/conductor/tests/` | Pin B at vessel grain: same vessel drains next beat; a sibling on the same unique does not inherit the wall |
| `engine/iae` · `engine/solver` | Unchanged — the organ still takes `&mut Vec<Crystal>`; the grain is the conductor's keying |
| `engine/cfe/src/ipsal.rs` (`Entity`) | shut — the basin is not on the committed wire |
| survival (`ration_survives` / `express_survival`) | shut — survival stays the line's ration book |

`solver` still does not import `cfe`. The conductor owns the vessel grain.

---

## 7 · Pins (done when these are green)

| Pin | Law |
|---|---|
| **B at vessel grain** | a wall vessel V learns at beat K drains for V at K+1; a sibling W on the same unique walls afresh — W never inherited V's crystal. |
| **Isolation** | `grown[V]` and `grown[W]` are independent; no path reads across basins. Two vessels, two perspectives. |
| **SEAL-B** | `Entity` schema untouched; `seal_b_era_pins` green with the map. |
| **N-I6 held** | survival is the line's ration book; the express is the sole builder; Pin G still gates ARC. |
| **N-I4 held** | search off the beat; a think withhold is rest, not a sweep. |
| **mint_loop** | a Fed line still researches and mints — the drain is per-vessel, the mint unchanged. |

Do not start N-I7, N-I8, N-I10, or SOC until Pin B is green at vessel grain.

---

## 8 · Accept

```bash
# from engine/
cargo test -p conductor --test <vessel_pin> --test seal_b_era_pins --test mint_loop_pins
cargo test -p iae -p solver   # unchanged organ stays green
```

Prior witnesses stay green (`survival_pins`, `survival_gate_pins`,
`conductor_pins`). A red `an_era_brings_the_board_alive` is the same class
of bug — measure the vessel drain in the same post-tick window.

**Landed** `4399627`. Pin B green at vessel grain. Next is [`N-I7.md`](N-I7.md).
