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

### 4.1 Freeze-out on the growing S^360 (operator 2026-10-28: "this is still constant: the equilibrium g = ¼"; "add curvature of the S^360 sphere that grows from the core")

**Why ¼ was a constant.** The window's g read `1 − (1 + g_throat)/ν`, every
term a fixed point, and the exchange's rates summed to one (`g + u = 1`), so
one ring landed on the equilibrium: no history, no scale. Every founding
read the same share.

**The freeze-out.** A pole changes only while the bounce's heat exceeds the
grip that would hold it: each ring the exchange runs at the face share times
the heat's EXCESS over the grip,

    α = g·x,  β = u·x,  x = (a − grip)/a   (a > grip; else the poles stand)

so `1 − α − β = 1 − x` and the share APPROACHES its moving equilibrium over
the rings of the passage, by the master equation's closed form (the product
of `(1 − x_k)` over the rings, a reading of the passage, never stepped), and
FREEZES where the heat falls to the grip.

**The curvature (docs/78: "Space: S^360 of radius R, the Big Bang's
curvature"; distance the geodesic angle; "the core's pulse refocuses on the
far side; that round trip is the ring").** The bounce's front runs a GEODESIC
from the core over the sphere: at the core, across its equator, to the
antipode where it refocuses and returns. Its place is its geodesic angle
θ, read in half-turns (the antipode at 1), and the sphere's own measure there,
the front's step with the core, replaces the flat deficit:

    δ_S(θ) = 1 − θ       (1 at the core; the equator, the throat's fixed
                          point, at θ = ½ of the way; 0 at the antipode)

first degree, intrinsic to the sphere (never a chord, never a radius
squared). The faces at the front read `g = 1 − δ_S`, `u = δ_S`; the heat
`a = ν·δ_S − 1`.

**The sphere grows from the core.** The front advances one light-length a
ring; on a sphere of circumference C (in rings) that is `1/C` of a turn a
ring, so the rings from the core to the window are

    k_w = θ_w·(C/2),   θ_w = 1 − δ_S,w,   δ_S,w = (1 + grip)/ν

(the `/2`: the out leg of the round trip, the double pass's one leg, ruled).
A larger sphere holds its front in the hot cap for more rings: the exchange
runs longer and freezes nearer its moving equilibrium. The neutron share is
then a READING OF THE FOUNDING'S OWN STATE through C, never a fixed share.

**Open, the sphere's circumference at the founding.** C is the round trip
(TUniversal's echo, `T_u` bells × ν rings a bell): today `T_u` is read from the
founding cascade's outermost shell, which retires (docs/101); its derivation
from the universal core's own state, and its growth echo by echo as the core
pulses out its throat, is the same question owed with mycelium (docs/101 §6,
TUniversal's owner). Until it is ruled the founding reads C from the
universe's current `T_u`; the founding and `T_u` are then read together to
their fixed point (the composition sets the cascade's echo, the echo sets the
freeze-out).

## 5. Build order (each with its witness)

1. **`polarity`** (BUILT): `equilibrium(α, β)`, `after(x₀, α, β, n)`, `faces(δ)` →
   `(g, u)`. Witness: conserved; the equilibrium is a fixed point of the
   ring; the closed form at N and 4N agrees with stepping.
2. **The nucleons' exchange** on it. Witness: nucleons conserved; the
   equilibrium share stands under a pass.
3. **Survival, grip, the electrons' bound share** read at `g` of the body's
   place (`nuclides`); every caller passes its place's `g`. Witness: the
   seals keep every capture; survival rises with `g`; at the throat it reads
   the fixed point.
4. **The founding re-read** (§4; BUILT at the throat, its share ¼ a
   constant). Witness: nucleons conserved; the densest element read, never
   set.
4a. **The freeze-out on the growing S^360** (§4.1): the exchange at the
   heat's excess over the grip, the front's measure `1 − θ` on the sphere,
   the rings to the window `θ_w·C/2`, C the round trip. Witness: the share
   moves with C (a larger sphere freezes nearer its equilibrium); nucleons
   conserved; at x = 1 every ring the master equation's equilibrium.
5. **Heat's equilibrium and the counterweight** (docs/104 §2). Witness: the
   heat ledger closes each echo.
6. **Read:** re-found, breathe to bell 1700.
