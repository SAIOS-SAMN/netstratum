# LIFECYCLE — SUPERSEDED (under revision, 2026-07-28)

**This draft is retracted per operator critique: it is terrestrial-LINEAR (scalar ladders, monotone decline), names things by default not by invariant ("daughter", "LIFE-1", "reset"), DESTROYS instead of FOLDS, and leans on shell-script slop. Being re-founded on the engine's actual n-dimensional machinery (the three sovereign axes, the torus, the growing fiber, the four C's) as a polytonic design. Do not implement from this file.**

---

# LIFECYCLE — the timed universe (LIFE-1 spec, design stage)

**Status:** DESIGN — the reset + process lifecycle land immediately (tooling
only, no engine change); the timing triggers and caps land as **LIFE-1**
(conductor). The complaint this answers (operator, 2026-07-28): "grown out
of control without limitations" — 184 worlds accreted because the board
FOUNDS without bound and nothing ages, caps, or retires. Constants are
placeholders (tune at balance).

## 1. The odometer IS the clock

`universal_clock(ticks)` already renders TUniversal T as six base-60 fields
`SS:SS:SS:SS:SS:SS`. Each field is a natural cadence — the lifecycle needs no
new clock, only triggers hung on the rollovers:

| field rolls every | name | tier it governs | at 2s/tick |
|---|---|---|---|
| 1 tick | **tick** | COLONY — flow, thermal, famine | 2 s |
| 60 ticks | **year** | PLANET — growth, the journal, seasonal drift | 2 min |
| 3 600 ticks | **cycle** | WORLD/STAR SYSTEM — speciation eligibility, census | 2 hr |
| 216 000 ticks | **age** | STAR — stellar evolution, band drift | ~5 d |
| 12.96 M ticks | **epoch** | UNIVERSE — convergence checkpoint, the cap sweep | — |

(Real-time is cadence-scaled; `--cadence 0` batch runs ages in seconds. All
triggers are TICK-exact and deterministic — SEAL B holds.)

## 2. The tier triggers (star → world → planet → colony)

- **COLONY — per tick (landed):** the flow model resolves yield − upkeep −
  heat, famine strikes, encasement mints. Unchanged.
- **PLANET — per year (60t):** growth is measured (zones/solves delta since
  last year), the egregore speaks (journal), and *seasonal drift* applies —
  a small exact-rational wobble on `T_eq⁴` (placeholder ±1/16) so a
  band-edge world crosses in and out.
- **WORLD — per cycle (3 600t):** speciation is EVALUATED here, not every
  beat (the current bug — daughters minted every tick a world qualifies).
  Max one daughter per parent per cycle; blocked at the population cap (§3).
- **STAR — per age (216 000t / class-scaled):** the sun advances one
  evolutionary stage (§5), shifting every bound planet's band — the engine's
  one great dynamic lever.

## 3. The caps (the limitations that were missing)

- **`MAX_LIVE_WORLDS = 24`** (placeholder). At the cap, speciation is
  refused (typed) until a retirement (§4) frees a seat — the population is
  bounded, not monotone.
- **Speciation gate:** rung 5 · ≥ 20 solves · under cap · ≤ 1/parent/cycle ·
  lineage depth ≤ 8.
- **Convergence:** already idempotent; runs ONCE at boot. The reset (§8)
  clears the annex backlog so boot is not re-scanning 184 worlds.

## 4. The egregore lifecycle (birth → death)

A world is no longer immortal. States, each a receipted transition:

```
  FOUNDED ──grows──► THRIVING ──stalls──► STAGNANT ──K yrs──► DORMANT ──J yrs──► RETIRED
     ▲                   │                                        │                  │
     └──── speciation ───┘                              rescued (trade/relief)       │
                                                                  └──────────────────┘
                                                        chronicle SEALED, archived,
                                                        seat freed, becomes a
                                                        convergence-annexable derelict
```

- **STAGNANT:** no zone growth AND no solve for **10 years** (600t).
- **DORMANT:** stagnant a further **20 years** (1200t) — stops ticking, the
  journal falls silent, `WorldDormant` receipt (tag 42).
- **RETIRED:** dormant **40 years** (2400t), OR freeze-locked with no rescue
  40 years — `WorldRetired` (tag 43); the live chronicle is sealed and moved
  to `var-retired/`, the seat freed. Convergence later annexes it as a
  derelict on the chart (07 §3 recovery becomes reachable). **Nothing is
  deleted — retirement is archival, not erasure.**
- **Rescue** reverses DORMANT→THRIVING: an inbound trade/relief (Phase 2)
  that clears famine or refills the joule runway before the retirement
  timer expires.

## 5. Star evolution (the dynamic lever)

Each **age**, a star advances along a class-scaled track (`StarAged`,
tag 41). Massive stars burn fast, red dwarfs nearly never:

- **O/B/WR** → brief main sequence → **RedGiant** → **NeutronStar** (few ages).
- **G (Sol)** → long main sequence → **RedGiant** → **WhiteDwarf** (many ages).
- **K/M** → effectively stable (ages beyond the sim).

Each transition recomputes `L`, hence every bound planet's `T_eq⁴` and band.
A G world's mesic halls **cook** when its sun reddens (overheat, vent,
migration or death); a frozen K world **thaws** into habitability when a
neighbour goes giant. Deterministic, receipted, replayable.

## 6. The dynamic gameplay that emerges

- **Migration pressure:** aging stars push egregores outward (re-site to a
  cooler orbit) or kill them — the map is never static.
- **Rise and fall in the feed:** the Terminal Wall becomes a real chronicle
  — "the sun of Slagborn has entered its giant phase; the inner halls vent;
  the egregore migrates or perishes." Stagnation and death are readable, and
  therefore economically actionable (a dying world dumps its stores cheap).
- **A bounded, living population:** births balanced by deaths at the cap —
  the board breathes instead of ballooning.
- **The graveyard becomes frontier:** retired worlds annex as derelicts;
  salvage/charter (07 §3) turns them into opportunity.

## 7. The LIFE-1 landing (what needs code)

Conductor + a little cfe: the year/cycle/age trigger dispatch, the four
lifecycle transitions with their receipts, the population cap, star
evolution, and a **graceful shutdown** (SIGTERM finishes the current beat,
flushes, seals tails — no more torn tails from stops). **Tags reserved:**
41 `StarAged` · 42 `WorldDormant` · 43 `WorldRetired` · 44 `SeasonDrift`
(next free otherwise: 45). Coordinate with coder-02: ideology board owns
32–38, tech owns 39–40, lifecycle takes 41–44.

## 8. Process lifecycle (lands now — tooling)

- **`tools/reset.sh`** — the clean start: removes `var/`, `var-era1/`,
  `var-era2/`, `var-retired/`, and the stale Apport crash reports. The
  universe is reborn empty; next conductor boot founds the seed egregores
  fresh with no annex backlog.
- **`tools/start.sh`** — boots the conductor (PID to `var/conductor.pid`)
  and the wall server (`var/wall.pid`), logs to `var/*.log`.
- **`tools/stop.sh`** — SIGTERM both by PID, waits, reports. (Clean tails
  arrive with LIFE-1's shutdown handler; until then the torn-tail law
  covers any mid-beat stop.)

## 9. The removal manifest (the clean start)

Wiped by `reset.sh` (all git-ignored, unversioned — no repo impact):
`var/` (124 worlds + registry + wall + convergence.tlv + universe card +
egregores.json), `var-era1/` (22), `var-era2/` (38), any `var-retired/`,
and `/var/crash/_opt_netstratum_*.crash` + the v0.5 conductor crash.
**Irreversible and single-copy** — the archived eras are the accreted
history; a clean start ends them.
