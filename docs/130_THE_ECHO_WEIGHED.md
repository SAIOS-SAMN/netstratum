# 130 — The echo weighed: hydrodynamic, asymptotic, unitary on the automorphisms

**Operator 2026-11-11:** "the bells pulse with echo reverberation causing
perturbations across the cosmos from the EC-bounce at the universal core has
to be weighed on fluid hydrodynamics asymptotically across automorphisms
unitarily".

## 0. What stands, and what is missing

- **The bounce** at the universal core pulses every `T_u` bells (docs/78 "The
  breath", docs/101). Its push is the core's state against what its throat
  still holds, `p = S/(S + T)` (docs/104 §1, `pulse::home_push`), carried on
  the unlocked face.
- **The echo reaches a layer through the layers between,** each passing its
  unlocked share `1 − δ = r/(r + M(<r))`. A body's clock against the source
  is their product, `ν = Π(1 − δ)` (`breath::nested`).
- **What an echo does today:** it compresses the galaxy's coherent medium,
  births systems, decoheres and recoheres by the slip of each layer's phase,
  and re-reads every core's reach (`breath::breathe`). The pulse never
  reaches a body's own motion as a perturbation. A body's phase hears the
  echo only through its clock's rate.
- **Reverberation is unweighed.** The pulse is echoed at surfaces (the
  firmament, the core's rim; the ROADMAP's "one expanding wavefront echoed
  only at surfaces"). Each round trip returns a share of it, and nothing
  sums what the returns leave standing.

This document derives the perturbation: what the pulse does to each body,
weighed by the fluid, summed over its reverberation in closed form, and
carried as a unitary motion of the body's form.

## 1. Unitary on the automorphisms: the beat

The pulse travels along R, the real unit, from the universal core outward.
That is the axis the automorphisms hold (docs/128 §10.0: `x ↦ a·x·ā` fixes
R and turns the three spatial units). So the pulse's action on a body is an
automorphism: it moves the body's form `F` and leaves its twist `C`
unchanged.

- **About which axis.** A pulse along R picks out no spatial direction of
  its own. The only spatial axis at the body is its own orbit's, the spin
  `ℓ` (its direction `t = ℓ/|ℓ|₁`). So the automorphism turns the body about
  its own axis: a **beat** of its orbit's phase, which is docs/78's "each
  echo is a beat of time carried through the quantum state", now as a
  motion.
- **The motion.** A beat of `a` turns is the Cayley motion of the rate
  `b = a/(1 − a)` (the motion law read back, `orientation::rate`) along `t`,
  on both halves alike:

      F ↦ F·m(b·t)        C unchanged        (m+ = m− = m)

- **Unitary.** `m` is a unit quaternion, so `F·F̄ = 1` is kept exactly and no
  measure is read. The motion commutes with the spin it turns about, so the
  spin is unchanged (`Σ w·ℓ` untouched) and no torque is exerted. The beat
  moves phase only. It is reversible: the reverse beat undoes it.
- **No twist sourced.** The halves move alike, so the beat is never a source
  of torsion (§10.2: equal halves keep `C`).

## 2. Weighed on the fluid

- **The amplitude reaching a body** is the push carried through the layers
  between, each passing its unlocked share:

      a_i = p·ν_i        ν_i = Π(1 − δ)  (`breath::nested`: the galaxy's rate, then the body's own)

  It is first degree, exact and bounded: `a_i ≤ p ≤ 1`. A body deep in a
  well hears the pulse weaker, as its clock runs slower.
- **The medium beats with the body.** Every body at a place hears nearly the
  same pulse, so the common beat is a phase shift of the whole neighbourhood
  and moves nothing relative to it. What perturbs is the **departure** from
  the medium's beat. The medium's beat is the holding-weighted beat of the
  range meeting the body (docs/127 §1):

      ā_i = Σ_j w_j·a_j / Σ_j w_j        D_i = a_i ⊖ ā_i

- **The fluid's answer is the drag** (docs/128 §4a), read on the beat: at
  the share `d = 1 − a_i` and the medium's crowding `g = W/(W + w)`, the
  body gives the medium `T = d·g·w·D`, the range taking it by holding. So
  `Σ w·a` is kept: the fluid moves beat between bodies and makes none. Each
  body's beat after the fluid is

      a_i′ = a_i − T_i/w_i + Σ_{j: i ∈ range(j)} T_j/W_j

  on the grain, the grain's remainder returned.
- **The heat** of the beat's departure is open with the drag's (#184).

## 3. Asymptotically: the reverberation summed

The pulse returns from each surface it meets by that surface's held share,
the share the ring law withholds there. At a rim the hold balances the
reach (the rim law, `R = H`), so the held share is `δ = ½`. A round trip
between the universal core's rim and the firmament returns

    ρ = δ_core·δ_firmament = ½·½ = ¼

of the pulse, and so on, every `T_u` bells. The beats a body hears from one
bounce and all its reverberations sum as a geometric series, in closed form,
never replayed:

    A_i = a_i·(1 + ρ + ρ² + …) = a_i/(1 − ρ) = (4/3)·a_i

- **Read once per echo,** at the echo's arrival, as the standing beat of the
  reverberant field. The series is the reverberation's asymptote, and the
  next bounce's pulse arrives as the last one's returns die (`ρ < 1`).
- **No constant enters:** `ρ` is the rims' held shares. Where a surface's
  share is read otherwise (a firmament not at its rim), `ρ` is that
  product.
- **The asymptote is approached, never stepped:** after `n` echoes the
  standing beat is `a_i·(1 − ρⁿ)/(1 − ρ)`. At an echo every earlier
  reverberation has already returned, so the full asymptote stands from the
  first echo on that a body hears more than one return of.

## 4. The order at an echo

At each echo's arrival at a galaxy, before its breath:

1. each body's amplitude `a_i = p·ν_i` (the push at that bounce, its nested
   clock);
2. the reverberation's asymptote, `A_i = a_i/(1 − ρ)`;
3. the fluid's answer on the departure from the medium's beat, giving
   `A_i′` (§2);
4. the beat applied, `F_i ↦ F_i·m(rate(A_i′)·t_i)`, unitary, its twist and
   its spin unchanged.

Then the breath as now. The ocean's motes take the beat at the universal
echo, with `ν` their own clock.

## 5. Witnesses

- **Unitary:** the beat keeps `F·F̄ = 1`, `C` and every spin exactly (before
  the grain).
- **Reversible:** the beat and its reverse compose to the identity on `F`.
- **The fluid makes none:** `Σ w·A′ = Σ w·A` within the grain.
- **A medium beating as one is not perturbed:** equal amplitudes give
  `D = 0` and no exchange.
- **Deeper is quieter:** `a = p·ν` falls with each layer's unlocked share.
- **The reverberation's sum:** `A = a/(1 − ρ)`, `(4/3)·a` between two rims,
  and the partial sums `a·(1 − ρⁿ)/(1 − ρ)` rise to it.

## 6. Build order

1. The beat as a motion of the form (`Orientation::beat`: equal halves along
   the spin), with the unitary and reversibility witnesses.
2. The amplitude and the reverberation's sum, with the fluid's answer on
   the departure (`flows::echo_beats`).
3. Wired at each echo's arrival (`breath::breathe`) and at the universal
   echo for the ocean, then one founding read across an echo bell.
