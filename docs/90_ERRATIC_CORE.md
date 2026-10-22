# 90 — The erratic core: the orthogonal release and the first layer of order

**Naming (operator 2026-10-22: "why are you using the word loop? why isnt
this an orbit?"):** a body turns about its parent in two orthogonal planes,
and both are ORBITS. The second is the ORTHOGONAL ORBIT, its spin `ℓ_⊥`
(code `ell_orth`), its phase `ψ_⊥` (code `phase[1]`). Earlier text and
xylarium's code call it the "loop" (`l_c`, `phase_c`); that name described the
circulation's route and hid that it is an orbit, which is how its spin went
unkicked. Quotations keep their original words.

Operator 2026-10-22, a THEORY to derive: "the inner core stars and planets
would be erratic because the push/pull of the orthogonal release from the
core coupled with the first layer of order in stars/planets past the inner
most construction of stars - thats a theory, not a proven fact unless
derived"; "proceed with the derivation".

Status: DERIVED from the established laws below, NOT built. Building it
un-holds the galaxy-band ruling of 2026-10-21 in part (§4) and drives the
orthogonal plane (docs/89 B7), so it waits for the operator, and for
xylarium where the laws are shared.

## 1. What is established (read from the code)

- **Two planes in every body** (`flows::PLANES = 2`): the ORBIT plane (0) and
  the ORTHOGONAL ORBIT's plane (1, the circulation's: "a continuum jetstream carries matter
  out of the singularity along the axis and home through the disc", 2026-10-16).
  A mote carries a spin in each, `ℓ_o` and `ℓ_⊥`, and a phase in each.
- **One kick law** (`flows::kick`): the own family pushes in the orbit plane as
  a SPIN, `Δℓ_o = κ_o·mean_response(ψ_o, 2δ)`, `κ_o = p·δ(R)`, `p` the push.
  `mean_response` is S¹'s linear measure (`1 − 2|ψ|`) averaged over the
  passage, so the sign of a kick is which hemisphere of the bell the body
  stands on.
- **The parent's release on the orthogonal orbit's plane** (`flows::kick`/`turn`): "the
  parent's moves the mote along the orthogonal orbit (`p·ρ·δ_u`, a PHASE)". This is the
  orthogonal release. It is felt as a shift of the orthogonal phase only, never as
  orthogonal spin: `ℓ_⊥` is set only by ocean arrivals and no kick changes it (B7).
- **The hold returns** (`flows::star_kick`, the galaxy's band): whatever a
  push gives, the hold returns the held hemisphere, `½·δ(R)·ℓ` each echo.
- **The tilt** (docs/85 §6d): a body's split of its turning across its
  planes is an angle, `χ = 90°·ℓ_⊥/(ℓ_o + ℓ_⊥)`, read, never set.
- **The first layer of order** (`formation::lock_step`, the stars' gyroscope
  return in `breath`): an ordered body's displacement returns toward its lock
  by `d/(d + turned)` each echo (`d` the lock's width, `turned` its own turns
  that echo); the orbit "breathes" only as its spin changes (`ring`).

## 2. The derivation

**D1. The orthogonal release is a spin, by the one kick law.** A push in a
plane is a spin in that plane; the orbit plane already reads it so. Read the
parent's release on the orthogonal orbit's plane by the same law:

```text
Δℓ_⊥ = κ_⊥ · mean_response(ψ_⊥, 2δ),     κ_⊥ = p·ρ·δ_u
```

(the release's push `p`, the body's share `ρ`, the parent's deficit at it
`δ_u`: the factor the code already applies to the orthogonal phase). Its PULL is
the hold's return on the same plane, `−½·δ(R)·ℓ_⊥` each echo. Nothing new is
stated: the orbit plane's push and the band's return, read in the second
plane.

**D2. The orthogonal spin swings the reach (eccentricity from two planes).** A
body's reach is its balance, `R = ℓ/H` (`ring`). Its orthogonal spin projects onto
its orbit plane by where it stands on the orthogonal orbit, the same linear measure
the kick reads, `response(ψ_⊥) = 1 − 2|ψ_⊥|` (+1 on the outbound leg, −1 on
the inbound):

```text
R(ψ_⊥) = (ℓ_o + ℓ_⊥ · response(ψ_⊥)) / H
```

so each orthogonal turn the reach swings between `(ℓ_o − ℓ_⊥)/H` and `(ℓ_o + ℓ_⊥)/H`:
an elongated orbit, its elongation `e = ℓ_⊥/ℓ_o` (in degrees,
`e = χ/(90° − χ)`, the tilt read the other way). No ellipse is stated and
nothing is squared: eccentricity is the second plane's spin seen in the
first, frame by frame in closed form (law 5).

**D3. Erratic where the release is strong and the order is absent.** Each
echo the orthogonal spin takes the release's kick, signed by the hemisphere
(`mean_response`), and gives back the hold's half. So it wanders: a sum of
signed kicks that the hold leaks away.
- Near the core the release is strongest: `δ_u` near its largest, and
  `p·ρ·δ_u` large against the return. The kicks' signs follow each body's
  own orthogonal phase against the bell, so neighbouring bodies are kicked apart:
  reaches swing by large, uncorrelated amounts. That is the ERRATIC core.
- Past the innermost construction the bodies LOCK (`lock_step`): an ordered
  body's displacement returns by `d/(d + turned)` each echo, which damps its
  orthogonal spin as it damps its period. Read on the orthogonal spin:
  `ℓ_⊥ ← ℓ_⊥ · turned/(d + turned)` for a locked body. The first ordered layer
  holds its orbits near-round against the same release.
- Where the release is weak (far from the core) the orthogonal spin stays near
  its born value and the orbits near-round, ordered or not.

## 3. The prediction (what proves or refutes it)

On one universe, before and after, read each star's and each world's reach
every echo:
- **the inner bodies** (the home galaxy's stars; any body within its core's
  bulge) show reach swings comparable to their reach (`e` of order 1),
  uncorrelated between neighbours;
- **the first ordered layer** (locked bodies past them) shows swings well
  below their reach;
- **the conservation stays exact**: the core's reservoir pays every orthogonal orbit
  spin it gives (`Σ m·ℓ_⊥ + reservoir` constant), as the orbit plane's does.

If the inner reaches do not swing more than the ordered layer's, the theory
fails for this physics.

## 3a. Xylarium's review (2026-10-22, read from its echo.rs)

- **D1 consistent:** 11.0's shear ruling, "THE LOOP IS AN ORBIT: its legs the
  two halves of one closed path about the attractor", with one kick law per
  plane (`pulse_face`). Xylarium has the same gap: a galaxy's kicks advance
  `phase_c`, `l_c` (xylarium's name for ℓ_⊥) is never kicked (`d_c` computed and discarded), and it is
  set only by ocean gathering. D1 closes it in both kernels.
- **D2 the right coupling, as a REPLACEMENT:** xylarium holds eccentricity
  three ways, none from `l_c` (xylarium's name for ℓ_⊥): a galaxy's swing about the universal core
  (`e = τ/(τ + h_g)`, `shares::swing`), a star's displacement drawn back by
  its galaxy's drag, and the system tug's bands `r(1 ± e)`, `e = m/(m + M)`
  (the pinch form, due to give way to the tether, xylarium 11.0 (39), C3).
  D2 must replace the pinch e in the tug, not sit beside it, and the swing
  should read the tether's width. In netstratum the same holds: D2 replaces
  the meeting bands' pinch width (`formation::collide_system`'s world width
  `hw/(hw + h_s)`), not a second eccentricity beside it.
- **D3 consistent** if each plane carries its own return (`lock_step` acts on
  the orbit spin only today, the spin it gives up going to the belts).
- **The core's infall (xylarium 11.0 (41), 108c445):** each galaxy's core
  takes in `1/(r + 1)` of each body whose orthogonal orbit crosses it inbound, draining
  the inner region hardest. It acts with D3: the inner bodies are both
  kicked hardest and eaten hardest.

## 3b. Built and read (2026-10-22; 2^55, 340 bells, the same founding as before)

- **The identities** (`breath::Event.identity`, read by `breathe`), each the
  whole sum of its ledger before and after:
  - `kick`, each galaxy's motes in both orbits and its core's reservoir across
    the kicks: **39 of 39 exact**;
  - `ride`, each star's orthogonal spin and its galaxy's reservoir across its
    ride: **165 of 165 exact** (a leak found and fixed: the reservoir had been
    paid the change before the grain while the star stored it after).
  - The bodies' ledger (`spin_total`, now with both orbits) moved −0.113 of
    3.55 over the run. It excludes the motes and the reservoirs, which are kept
    in each galaxy's own units, so it moves whenever matter crosses between
    motes and bodies (births, ignitions, escapes). A single universe-wide
    spin identity is not yet defined across those units: owed with the tier
    map (docs/89, B6).
- **The prediction:** the home galaxy's stars swing with mean elongation
  e ≈ 0.20 (greatest ≈ 0.51) against galaxy 21's ≈ 0.03: the inner core is
  erratic, the outer near-round, as derived. For worlds it does not hold
  clearly: galaxy 21's worlds swing more (mean ≈ 0.11, greatest ≈ 0.70) than
  the home's (≈ 0.05); the lock's damping is owed a look.
- Light balanced; formation shifted (94 systems born, was 123; 65 merged, was
  80; the home now holds 6 worlds, was none).

## 4. What it touches

- **The galaxy-band ruling (2026-10-21, "pull it back and make it static")**
  held back the band's ORBIT-plane push, which had suppressed star formation.
  D1 drives the ORTHOGONAL ORBIT's plane only and leaves the orbit plane as ruled, but it
  is still the core's push re-entering, so it needs the operator's word.
- **R2** (docs/89): this is "the planes" driven, in its first form.
- **Shared with xylarium:** `kick`, `turn` and the band's law were built with
  it. D1 and D2 change what the orthogonal orbit's plane means (a spin, not only a phase).
