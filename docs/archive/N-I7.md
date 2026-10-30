# N-I7 — the look costs leisure

**Lane:** intelligence port · **Status: named — implement this cut**
**Charter:** [`53_INTELLIGENCE_PORT.md`](../53_INTELLIGENCE_PORT.md) D0.4 · `52_COGNITIVE_METABOLISM` (COG-1/3/4/5)
**Companions:** N-I-VESSEL (`grown` per vessel, seated) · `civic::mint_cost` / `metabolic_exchange` · `leisure_bank_of` / `settle_leisure`

*Markdown is instruction only. The engine reads the bank and the beat.*

---

## 0 · One sentence

Thinking is still free — this cut makes the **look** cost leisure: a Fed
vessel thinks only if its orb's leisure bank can pay the observe price,
pays it, and then drains; an unpaid look is rest.

---

## 1 · What this is not

| Refused | Why |
|---|---|
| A new rate table | The price derives from the seated `mint_cost × metabolic_exchange`. No dial, no fresh constant. |
| Free think when the bank is 0 | That is the unpriced speech this cut ends. A broke line fasts — it thinks slowly across beats, not for free. |
| Desperation burn on the look | The burn (cannibalize people for a Eureka) stays on word mint only. A look you cannot afford is rest, never a burn. |
| Per-vessel leisure split | Leisure is the orb's book; the actor is the `automaton`. Splitting leisure per vessel is a later cut (with population individuation), not this one. |
| Population individuation | Held (same as N-I-VESSEL). |
| `vocabulary::fold` / a second vessel draining the word | N-I8 — the file after this. |
| A chat layer / hearing a word in another frame | N-I10. Never a message score. |
| Re-pricing the word mint | `mint_cost` on the mint stays, unchanged and additive. This adds a price on the look, not on the mint. |
| `HashMap` in cognition | Law II. The organ is still `Vec<Crystal>`; the basin index stays the `BTreeMap`. |
| Touching `Entity` / the SEAL-B surface beyond the existing leisure book | The debit rides `settle_leisure` (already committed + replay-exact). No new wire. |

---

## 2 · Why: unpriced speech

Word mint already pays: a confirmed word bills `mint_cost` in leisure
(COG-2/4/5) and a line with no surplus cannot afford the thought. But
`think_task` — the look itself, the peel-and-reduce of a pair — costs
nothing. So a starving-of-surplus line still looks every beat; only the
rare mint is metered. That is speech for free: an entity can observe and
reduce without limit and only pays when it happens to compose a word. This
cut closes it — **the look is the priced act**, and the mint is a further
bill on top. Metabolism already bounds how hard a mind composes (COG-3);
N-I7 makes it bound how often a mind looks.

---

## 3 · The gate

`research_step`, after Pin G and `pre_reach`, before the fresh look:

```
if !survives            → None            (Pin G, N-I5 — Short never asks the bank)
pre_reach(library) hit  → hit             (REDUCE against crystallized words — free, no fresh look)
else, the fresh look:
    price = observe_price()               (§4)
    if leisure_bank_of(orb) >= price:
        settle_leisure(orb, -price)        (pay FIRST)
        think_task(&task, grown[vessel])   (then drain THIS vessel — N-I-VESSEL)
    else:
        None                               (FASTING — rest, not a sweep)
```

- **Pin G fires first.** A Short line withholds ARC at survival and never
  reaches the bank (survival outranks thought; the bank is a Fed-line
  question only).
- **`pre_reach` is free.** Draining against words already crystallized is
  not a fresh look — the library read stays uncharged (it is reduce, not
  observe). The price is on the new peel-and-reduce.
- **Pay, then look.** The debit is on the look, not on a find: a paid look
  that withholds still paid. Looking costs; finding is separate.
- **Unpaid is rest.** The `None` arm declines the trajectory exactly as a
  barren beat does. `swept` does not move (N-I4 held) — a fast is not a
  sweep.

---

## 4 · The price (derived, not dialed)

The baseline look is the cheapest thought — a single degree-1 observation:

```
observe_price = ratio( civic::mint_cost(&[BigInt::from(1)], &[]) ) * civic::metabolic_exchange()
```

The same two functions the word mint already uses (`civic.rs:222` /
`civic.rs:268`), evaluated at the minimal residual. So the look costs a
fixed, small fraction of surplus — strictly less than any composed word
(`mint_cost` is monotone in residual extent), and it moves automatically if
the seated cost law ever moves. No new rate, no new constant.

(The degree may later scale with the observation's own homology — a deeper
peel costing more — but the baseline this cut installs is the degree-1
floor; do not build the scaling table here.)

---

## 5 · Fasting fill

The bank refills from surplus each beat: `cognition_step` (`lib.rs:2069`)
settles whatever food surplus survives into leisure, less
`leisure_dissipation` decay. So:

- A **rich Fed line** carries surplus > `observe_price` every beat and
  looks every beat — the mind runs continuously.
- A **poor Fed line** cannot pay each beat: it fasts, letting the bank fill
  across several beats until it can afford one look, then spends it. It
  thinks slowly — the metabolic clock on cognition (COG-3), now on the look
  as well as the mint.

This is the intended dynamic, not a failure: a line's thinking rate becomes
a reading of its economic surplus.

---

## 6 · SEAL-B

The debit rides `settle_leisure(orb, -price)` — the same committed,
replay-exact mutation the word mint already uses (the leisure book is on
the wire; `seal_b_era_pins` / `mint_loop_pins` are green with it today). The
price is a pure deterministic read (`mint_cost` × `metabolic_exchange`,
fixed inputs), so replay computes the same debit and the same gate verdict.
`Entity`'s schema is untouched; `grown` stays the non-committed per-vessel
workspace. No new deed, no schema bump this cut.

---

## 7 · File map

| File | Role |
|---|---|
| `engine/conductor/src/lib.rs` | `observe_price()` from `mint_cost([1]) × metabolic_exchange`; gate the `None => think_task` arm on `leisure_bank_of >= price`, `settle_leisure(-price)` before the drain |
| `engine/conductor/tests/` | the look costs (a Fed line's bank falls when it looks); a broke Fed line rests (fasting), no sweep; Pin G still first; a rich Fed line still mints |
| `engine/conductor/src/civic.rs` | Unchanged — reuse `mint_cost` / `metabolic_exchange`; do not add a rate |
| `engine/iae` · `engine/solver` | Unchanged — the organ does not know it was paid for |
| `engine/cfe/src/ipsal.rs` (`Entity`) | shut |
| survival (`ration_survives`/`express_survival`) | shut — survival is unpriced (Pin G is the survival gate; the bank is the ARC gate) |

---

## 8 · Pins (done when these are green)

| Pin | Law |
|---|---|
| **Look costs** | a Fed line that looks this beat has a strictly lower leisure bank after — the observe was debited. |
| **Fasting** | a Fed line whose bank < `observe_price` does not call `think_task` this beat; it rests, `swept` unchanged; a later beat (bank refilled) it looks. |
| **Pin G first** | a Short line never reads the bank — survival withholds ARC before the price is even asked. |
| **mint_loop (rich Fed line)** | a rich Fed line still researches and mints — the look price does not starve the mint. |
| **Word mint unchanged** | `mint_cost` on the mint still bills; the look price is additive, not a replacement. |
| **SEAL-B** | `Entity` untouched; `seal_b_era_pins` green with the look debit. |
| **N-I-VESSEL / N-I4 held** | the paid drain still hits this vessel's basin; a withheld (or unpaid) look is rest, not a sweep. |

Do not start N-I8, N-I10, or SOC until the look costs and `mint_loop` still
mints for a rich Fed line.

---

## 9 · Accept

```bash
# from engine/
cargo test -p conductor --test <n_i7_pin> --test mint_loop_pins --test seal_b_era_pins
cargo test -p iae -p solver   # unchanged organ stays green
```

Prior witnesses stay green (`vessel_pins`, `survival_pins`,
`survival_gate_pins`, `conductor_pins`). A red `an_era_brings_the_board_alive`
means the price starved the founding window — the look must be affordable to
a surplus-carrying Fed line, or the board never learns.

Idle when the look costs and `mint_loop` still mints for a rich Fed line.
**Landed** `c079dfd`. Next is [`N-I8.md`](N-I8.md).
