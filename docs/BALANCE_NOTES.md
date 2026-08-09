# BALANCE NOTES — the measured survey (A4's executable half, 2026-07-30)

**Status:** MEASUREMENT, not tuning. Every retune is a LAW change for
replay (version-bumped, never silent — the Q4 discipline); the dials
turn only on playtest + the 05 §1.4 pricing ruling. This memo is the
instrument panel the tuning session reads.

## The board as measured (31 worlds, deepest chronicle 261 receipts)

| reading | value | the dial it speaks to |
|---|---|---|
| famine-struck planets | 13 / 31 (42%) | famine clocks + relief economics — HIGH; the want is the dominant experience of the board |
| venting planets | 8 / 31 | radiator_base 2 vs factory heat 1/4 per zone; the doctrine (×5/4) cures heat_net ≤ 1/2 exactly |
| offers posted / pairs cleared | 28 / 12 | pre-ratio-synthesis baseline; ratios are emergent now (ruled 2026-07-31) — remeasure |
| cognitive wants struck | 0 | windows 240/480 have not yet elapsed on sealed lines — re-measure after longer eras |
| tech nodes | 30 (incl. gifted) | crystallization paces well at the year voice |
| divergences | 13 | one-per-era is producing a doubling-flavored spread — the bounded-occupancy half of AEON-1b is not yet pressed, but 31 worlds from 16 seeds in a few eras says the fold will be needed on schedule |
| solves banked | 492 | the forge economy is rich; rung-5 beat cost is the known hot spot |

## The known hot dials (in likely-first order)

1. **Famine pressure (42%)** — either the relief economics are too
   thin (trade reaches too few) or the clocks too fast. LANDED
   2026-07-30: the famine ask is open-ended — any famished line
   offers half its largest bank for the famined resource, and
   unfamished deep banks (> 8) post surplus wanting the leanest
   resource their OWN zones burn (demand grounded in upkeep). The
   first cut wanted the scarcest bank outright and every book filled
   with alloy asks nothing answered; a resting offer blocks new
   intents, so upkeep-grounded wants also keep the resting ask
   correct when the famine later strikes that flow. Witness:
   `any_famine_can_ask_the_market`. MEASURED (60-beat rerun of the
   same roster): crossings 12 → 54; famine held at 42% on a board
   that grew 31 → 40 worlds (the bad-targeting variant measured 68%
   on the same run).
2. **Rung-5 forge cost** — SEARCH_BUDGET 200k is an engine law; the
   DIFFICULTY TABLE (word_len/grids per rung) is the lawful dial if
   beats must cheapen.
3. **Ratio synthesis** (RULED 2026-07-31): price is an emergent
   ratio, never a system property — there is no base price and none
   will exist. Intents derive their ratio from state: a famished
   line bids steep (want = give/(1+severity)); a surplus line asks
   its reserve-fraction premium (want = give×(2·reserve−target)/
   reserve, inside 2×). Unmatched ratios REST lawfully — a resting
   book is a lawful market state, never a bug.
4. **The iceberg (MARKET-2, ruled + landed 2026-07-31)** — trade's
   side-fee friction CONVERTED to cargo melt: delivered = sent ×
   (1 − φ·D), φ = 1/64 per unit of manifold L1 (spatial + key gap);
   isolation walls at φ·D ≥ 1. φ is the new hot dial: the roster's
   typical inter-world distances (20–40) melt 30–60% — measure
   cleared-crossing decay by distance on the next long run before
   touching it. Route charters (ROUTE-1) are the lawful supply-side
   answer, not a φ retune.
5. **Divergence rate** — 13 cleaves without a fold yet; the fold
   cadence (or the four-C eligibility pruning at it) is the release
   valve already landed.

## Price constants transferred to the economics lane (coder-03 → coder-01, 2026-07-31)

Placeholder prices sitting in dop.rs that nobody chose deliberately —
MERCHANT-1 must not arbitrage around them before they are tuned on
purpose. Mechanics stay coder-03's; the VALUES are economics:

- `scan_draw()` — 1 crystal per scan
- the resonator's conversion ratios
- the G2 ration magnitudes

Also mine already: φ = 1/64 (the melt), TARGET_RESERVE = 8, route
charter alloy 4 + upkeep 1/4 joule, instrument costs (alloy = grade,
diamond at 3+), η = (1+grade)/8.

Re-run this survey after any tuning session; the numbers above are
the baseline.

---

## P8.5 ruled table slice (2026-08-09) — AP capacity

Placeholder-free **shared AP** capacity (anti click-to-win). Constants live
in `cfe::world::ap_budget` and are the only lawful knobs:

| Constant | Value | Meaning |
|----------|------:|---------|
| `AP_BASE` | 16 | year-voice budget with zero habitats |
| `AP_HABITAT_MULT` | 4 | extra AP per standing Habitat zone |
| `AP_EXCHANGE_BONUS` | 2 | when T6 Exchange is seated |
| `AP_YEAR_TICKS` | 60 | window length (= `ORBIT_PHASE_TICKS` / journal year) |

Spend family costs are unit (1 AP each): **build** · **prospect** ·
**research** · **quote**. Pure re-derive from chronicle — no residual.

Labor force (P8.3): `habitats + clones` — not a dial; derived seats only.

Further balance curve retunes (famine clocks, φ melt, rung-5 forge) remain
measurement-gated as above — not silent constants.
