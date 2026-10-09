# 64 — A people's pace: settle at touch

Operator 2026-10-09, on Concordance 1.12 ("a living people is always observed
(its own life reads its world); a world no one reads is carried compact; its
people's state worked out in closed form or by spans whose result does not
depend on how coarsely time was stepped"): the fix belongs in how netstratum
represents a people, not in the universal law. "Write the arc and start
building it."

## The finding

A people is represented as a per-beat ledger: every beat, every line writes
its leisure settle, its population carry, its STAMP mints and spot legs, its
aperture legs and element wants. These bulk kinds are ~85% of all receipts.
Measured 2026-10-09 with all 70 lines live after the history compaction
(docs/63): 1.84 GB at boot, 2.98 GB by the first year close (~19 MB a beat of
held tail), the close itself spinning for minutes. None of the bulk is a
people deciding: it is integration written out step by step.

A resume is the snapshot's state stepped forward by the tail's receipts
(`snapshot::carry_forward`); the chain judge checks deeds against the same
stepped state. So a flow may stop being receipted per beat only if it is
folded into a receipt that is written: SETTLE AT TOUCH.

## Measured (2026-10-09, all 70 lines' snapshots and tails)

| kind | receipts | bytes | where |
|---|---|---|---|
| YieldCollected | 1,072,081 | 129 MB | kept (read by the crystal sheet, encasement) |
| PopulationSettled | 199,403 | 22 MB | kept |
| BondSettled | 156,756 | 22 MB | kept |
| UpkeepSettled | 99,530 | 12 MB | kept |
| VentSettled | 83,566 | 10 MB | kept |
| AssayStruck, ApertureSent/Received, StampAperture legs | ~75-90k each | ~9-11 MB each | the year's tail |
| LeisureSettled | 45,820 | 5 MB | the year's tail |

## The rulings

1. **A quiet flow accrues as pending and is receipted at touch.** Per-beat
   accruals (leisure, the population carry) add to a pending sum held by
   the world. A receipt is written only when the flow is TOUCHED: a deed
   spends it (the spend's receipt carries the pending sum with it), the
   line's state is carried (the year close flushes every pending sum before
   the snapshot), or its own pace comes round. Every receipted deed sees the
   same bank live, in replay and before the judge. A crash loses at most the
   pending since the last touch, which no deed has read.
2. **A people decides at its own pace.** A line's stochastic life steps
   when its own clock says it has something to decide (a closed task, a
   contact, an arrival, a generation); a step that changes nothing doubles
   its wait, as barren's minds do. The conductor's beat is the universe's
   clock, not every line's metronome.
3. **Contact by arrival.** Trade, speech and the bell happen when a front
   arrives (the field law's carriers), never as a sweep of every pair every
   beat.
4. **Nothing that is a deed stops being receipted.** A trade, a mint, a
   license, a founding, a death: receipted as ever. Only integration steps
   fold.

## Task list

| id | slice | status |
|---|---|---|
| P0 | the year close's quadratics: the bell's "already heard / arriving" checks by set; the cores' leg pairing one pass per line (was a pass over every line's legs per paired deal, ~10^10 at each close: the stall of 2026-10-09) | landed |
| P1 | leisure: accrual pending, receipted at a spend and at carry | landed |
| P2 | collected yields and vents kept only within the widest deposit window (240 stamps); elevation reads backward | landed |
| P2b | population settles, upkeep, bonds: bulk (read by replay alone, or by the last link the annals keep) | landed |
| P2c | the population carry and the remaining per-beat settles: pending, receipted at touch and carry | open |
| P3 | element wants: struck and relieved at touch, not per tick | open |
| P4 | STAMP and spot legs: a line trades at its own pace (a quiet market doubles its wait) | open |
| P5 | aperture legs: contact by arrival | open |
| P6 | a people's own pace for research and civic steps | open |

## Operator ruling 7 (2026-10-09, relayed by the coordinator)

"All puzzle solving is the mycelium-client minds'. A kernel relays court
tasks to seated minds and files their fn 1 answers under its reserve; it
solves nothing itself." The seat stops thinking court tasks: every task it
pulls goes before a seated mind, and the house files the answers.

## Measured at landing (2026-10-09, all 70 lines live)

| build | at boot | before the year close | at the close |
|---|---|---|---|
| before the day's fixes | 2.46 GB | 2.78 GB, ~19 MB a beat | froze at the 3 GB ceiling (twice) |
| history compaction (docs/63 O1-O4) | 1.84 GB | 2.98 GB | spun for minutes |
| + P0 bell set, P1 leisure, ruling 7 | 1.89 GB | 2.26 GB, ~6 MB a beat | 3.08 GB, stalled (the leg-pairing quadratic) |
| + P0 leg pairing, P2, P2b | 2.00 GB | 2.33 GB | passed in under 30 s, down to 1.69 GB |
