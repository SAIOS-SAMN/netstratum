# 105 — Polarity equilibria: the master equation of a two-pole exchange

Operator 2026-10-28: "you are missing derivable formulas and master equations
for polarity equilibriums"; "understood - proceed". Builds on docs/104 (the
inverse duals; §1.1 the count; §1.2 every share a pair share), docs/97 (the
nucleus), xylarium's founding survival (`elements.rs` `founded`, 11.0 R4: a
reading relayed 2026-10-28, inverted here through our polarity).

## 1. The master equation

Two poles A and B; each ring, a share α of A passes to B and a share β of B
passes back:

    A′ = A·(1 − α) + B·β
    B′ = B·(1 − β) + A·α

- **Conserved:** `A + B` exact; the exchange booked both ways.
- **Its equilibrium:** the share standing on B, `x* = α/(α + β)`, a pair
  share of the two rates (docs/104 §1.2). No ½: it reads ½ only where the
  rates are equal.
- **Its approach, closed form:** `x_n = x* + (x₀ − x*)·(1 − α − β)ⁿ`
  (compounding over rings, as the release's `(1 − s)ⁿ`; never stepped, never
  a square of a quantity).

## 2. The rates are the faces at the body's own place

Under inverse gravity (docs/104 §0) the HELD share is `g = 1 − δ` and the
UNLOCKED share `u = δ`, `δ = H/(R + H)` at the body's reach. What BINDS goes at
the held face's rate, what FREES at the unlocked face's: `α = g`, `β = u`, so
`x* = g/(g + u) = g`. Never a constant: the rates are where the body stands.

At the throat (`R = H`, the ring) `g = u`: the fixed point, the only place a
share reads ½, read there and only there (docs/104 §1.2).

## 3. The pairs

| pair | to the held pole (α) | back (β) | equilibrium | replaces |
|---|---|---|---|---|
| **nucleons**, proton ↔ neutron | g: a proton binds as a fibre's partner | u: a neutron freed to a lone end | neutron share `g` | `nuclides::exchange` (`p·δ`, `n·½(1 − δ)`) and its fixed point `2δ/(1 + δ)` |
| **electrons**, free ↔ bound | g | u | bound share `g` | the halving `(1 + R)/2` (`nuclides::held`) |
| **capture ↔ release** (a captured nucleon kept) | a fibre holds by its own pairing; a lone end at `g` | u | survival `κ + (1 − κ)·g` | `κ + (1 − κ)·(1 + R)/2` |
| **grip** (a nucleus's fibres held) | — | — | `κ·g` | `κ·(1 + R)/2` |
| **heat**, heating ↔ cooling | heating in, `u·e` (the unlocked face on the open frames) | cooling out, the pull back `g` and the light `e`, `g + e − g·e` | `a* = u·e/(u·e + g + e − g·e)` | the cooling keep with no counterweight; the heating face `½(1 − δ)` |
| **push ↔ pull** (where a body settles) | — | — | `u = g`: `δ = ½`, the ring | — |

Phase leaves survival, grip and the electrons' bound share: it stays only
where xylarium keeps it, heat stripping and decay (its relayed reading).

## 4. The founding, read through it

The bounce's matter crosses at the THROAT, `R = H`, where `g = u` (the fixed
point, lawful there). There:

- **The seal window:** deuterium (`κ = 1`) grips `κ·g = g_throat`; its heat
  falls to that grip on the source's clock, `δ_window = (1 + g_throat)/ν`.
- **The neutron share:** the nucleons' equilibrium at the window, `g` at
  `δ_window`, `1 − δ_window` (inverse gravity).
- **The climb:** each capture survives `κ + (1 − κ)·g_throat`, no phase; the
  table ends where less than one whole nucleus arrives (docs/97 §14).

Xylarium's (c), inverted: its survival `a·h` reads the crossing's carried
share (the fixed point) and the held share; ours reads the held face at the
crossing for the lone ends and the fibres' own pairing for the bound ones.

## 5. Build order (each with its witness)

1. **`polarity`:** `equilibrium(α, β)`, `after(x₀, α, β, n)`, `faces(δ)` →
   `(g, u)`. Witness: conserved; the equilibrium is a fixed point of the
   ring; the closed form at N and 4N agrees with stepping.
2. **The nucleons' exchange** on it. Witness: nucleons conserved; the
   equilibrium share stands under a pass.
3. **Survival, grip, the electrons' bound share** read at `g` of the body's
   place (`nuclides`); every caller passes its place's `g`. Witness: the
   seals keep every capture; survival rises with `g`; at the throat it reads
   the fixed point.
4. **The founding re-read** (§4). Witness: nucleons conserved; the densest
   element read, never set.
5. **Heat's equilibrium and the counterweight** (docs/104 §2). Witness: the
   heat ledger closes each echo.
6. **Read:** re-found, breathe to bell 1700.
