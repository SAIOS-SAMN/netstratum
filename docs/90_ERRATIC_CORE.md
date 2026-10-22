# 90 — The erratic core: the orthogonal release and the first layer of order

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
  the LOOP plane (1, the circulation's: "a continuum jetstream carries matter
  out of the singularity along the axis and home through the disc", 2026-10-16).
  A mote carries a spin in each, `ℓ_o` and `ℓ_c`, and a phase in each.
- **One kick law** (`flows::kick`): the own family pushes in the orbit plane as
  a SPIN, `Δℓ_o = κ_o·mean_response(ψ_o, 2δ)`, `κ_o = p·δ(R)`, `p` the push.
  `mean_response` is S¹'s linear measure (`1 − 2|ψ|`) averaged over the
  passage, so the sign of a kick is which hemisphere of the bell the body
  stands on.
- **The parent's release on the loop plane** (`flows::kick`/`turn`): "the
  parent's moves the mote along the loop (`p·ρ·δ_u`, a PHASE)". This is the
  orthogonal release. It is felt as a shift of the loop phase only, never as
  loop spin: `ℓ_c` is set only by ocean arrivals and no kick changes it (B7).
- **The hold returns** (`flows::star_kick`, the galaxy's band): whatever a
  push gives, the hold returns the held hemisphere, `½·δ(R)·ℓ` each echo.
- **The tilt** (docs/85 §6d): a body's split of its turning across its
  planes is an angle, `χ = 90°·ℓ_c/(ℓ_o + ℓ_c)`, read, never set.
- **The first layer of order** (`formation::lock_step`, the stars' gyroscope
  return in `breath`): an ordered body's displacement returns toward its lock
  by `d/(d + turned)` each echo (`d` the lock's width, `turned` its own turns
  that echo); the orbit "breathes" only as its spin changes (`ring`).

## 2. The derivation

**D1. The orthogonal release is a spin, by the one kick law.** A push in a
plane is a spin in that plane; the orbit plane already reads it so. Read the
parent's release on the loop plane by the same law:

```text
Δℓ_c = κ_c · mean_response(ψ_c, 2δ),     κ_c = p·ρ·δ_u
```

(the release's push `p`, the body's share `ρ`, the parent's deficit at it
`δ_u`: the factor the code already applies to the loop phase). Its PULL is
the hold's return on the same plane, `−½·δ(R)·ℓ_c` each echo. Nothing new is
stated: the orbit plane's push and the band's return, read in the second
plane.

**D2. The orthogonal spin swings the reach (eccentricity from two planes).** A
body's reach is its balance, `R = ℓ/H` (`ring`). Its loop spin projects onto
its orbit plane by where it stands on the loop, the same linear measure
the kick reads, `response(ψ_c) = 1 − 2|ψ_c|` (+1 on the outbound leg, −1 on
the inbound):

```text
R(ψ_c) = (ℓ_o + ℓ_c · response(ψ_c)) / H
```

so each loop the reach swings between `(ℓ_o − ℓ_c)/H` and `(ℓ_o + ℓ_c)/H`:
an elongated orbit, its elongation `e = ℓ_c/ℓ_o` (in degrees,
`e = χ/(90° − χ)`, the tilt read the other way). No ellipse is stated and
nothing is squared: eccentricity is the second plane's spin seen in the
first, frame by frame in closed form (law 5).

**D3. Erratic where the release is strong and the order is absent.** Each
echo the loop spin takes the release's kick, signed by the hemisphere
(`mean_response`), and gives back the hold's half. So it wanders: a sum of
signed kicks that the hold leaks away.
- Near the core the release is strongest: `δ_u` near its largest, and
  `p·ρ·δ_u` large against the return. The kicks' signs follow each body's
  own loop phase against the bell, so neighbouring bodies are kicked apart:
  reaches swing by large, uncorrelated amounts. That is the ERRATIC core.
- Past the innermost construction the bodies LOCK (`lock_step`): an ordered
  body's displacement returns by `d/(d + turned)` each echo, which damps its
  loop spin as it damps its period. Read on the loop spin:
  `ℓ_c ← ℓ_c · turned/(d + turned)` for a locked body. The first ordered layer
  holds its orbits near-round against the same release.
- Where the release is weak (far from the core) the loop spin stays near
  its born value and the orbits near-round, ordered or not.

## 3. The prediction (what proves or refutes it)

On one universe, before and after, read each star's and each world's reach
every echo:
- **the inner bodies** (the home galaxy's stars; any body within its core's
  bulge) show reach swings comparable to their reach (`e` of order 1),
  uncorrelated between neighbours;
- **the first ordered layer** (locked bodies past them) shows swings well
  below their reach;
- **the conservation stays exact**: the core's reservoir pays every loop
  spin it gives (`Σ m·ℓ_c + reservoir` constant), as the orbit plane's does.

If the inner reaches do not swing more than the ordered layer's, the theory
fails for this physics.

## 4. What it touches

- **The galaxy-band ruling (2026-10-21, "pull it back and make it static")**
  held back the band's ORBIT-plane push, which had suppressed star formation.
  D1 drives the LOOP plane only and leaves the orbit plane as ruled, but it
  is still the core's push re-entering, so it needs the operator's word.
- **R2** (docs/89): this is "the planes" driven, in its first form.
- **Shared with xylarium:** `kick`, `turn` and the band's law were built with
  it. D1 and D2 change what the loop plane means (a spin, not only a phase).
