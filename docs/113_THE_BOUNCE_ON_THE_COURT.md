# 113 — The bounce on the court: era-1 steps as Mycelium transitions

The court verifies by reconstruction: it rebuilds an object under a law
whose form it holds and reaches the same exact object. This document states
the era-1 universe's step, the reverberation of the bounce in the
Einstein–Cartan core, as the transitions the court rebuilds under
Mycelium's laws (L0, `laws::verify`).

**Status:** specified; depends on a turn-valued Conformal check in Mycelium (§4). It replaces the fixed two-body
probe (`ns-hypha probe`) as what Netstratum offers the court.

## 1. The step

Each bounce of the universal core sends one front through S^n. The elements
the front meets, and passes through, shape the manifold: their motes turn,
the throat releases matter into the core, and matter returns through the
throat. One bell of that is one step.

## 2. The mapping

| Mycelium | Era 1 | Code |
|---|---|---|
| `heights (h, h + 1)` | the bell before and after | `breath::breathe`'s bell |
| `bodies: (BodyState, BodyState)` | each mote of one shell, before and after the bell: its `nucleus` the point of its place on its enclosure's sphere (reach and phases), its `body` the sphere of its spread `R(1 ± p)` | `flows::Mote`; `flows::fluid`'s shells (docs/110) |
| `motion` | the turn the bell gives that shell, in ℚ/ℤ: on each spatial plane its turning `t_k·δ` a received ring (E1, E20) (§4) | `flows::turn` |
| `fluxes`, `Source` | the release from the core's throat into the core, at the throat, its mass the released holding | E18 |
| `fluxes`, `Sink` | what returns to the throat: recycling and infall | E30 |
| `hues` | the fibre relation of the motes' kinds, where a kind changes (fusion, release, capture) | E25, E28, E29 |

One transition is one shell for one bell: the motes of a shell turn
together (one δ), so every pair's inversive distance is kept (Rigidity) and
one turn carries them all (Conformal, §4).

## 3. What each law checks here

| Law | Era-1 statement |
|---|---|
| Conformal | the shell's turn, in ℚ/ℤ, carries every mote's phase (§4) |
| Momentum | what the motes exchange cancels; the core's reservoir pays the torque (`Σ m·ℓ + reservoir` exact, E20) |
| Rigidity | the shell turns as one: every pair of its motes keeps its inversive distance |
| Time | the bell advances |
| Reservoir | a change of kind is unitary on the fibre |
| Continuity | the shell's mass after is its mass before, plus the throat's release, less what returned (3Z conserved, docs/PHYSICS.md §6) |

## 4. The turn is the motion

A shell's turn is a motion in ℚ/ℤ: on each spatial plane every mote's
phase advances by the same `t_k·δ` (mod 1). The court reconstructs the step
by turn arithmetic: it rebuilds each mote's phase after from its phase
before and the shell's turn, and finds the same exact phases. Rigidity
follows: equal advances on one shell keep every pair's turn difference.

No versor carries a turn fraction exactly except a multiple of 90°: by
Niven's theorem `tan(πt)` is rational for rational `t` only at 0 and ±1, so
`rotator(s, b)` (exact for a rational half-angle tangent) cannot carry
`t_k·δ`, and no chart maps one to the other. The turn stays a fraction of a
turn, never coordinates on the circle (that is squaring, `x² + y² = 1`).

This needs a turn-valued motion in Mycelium's Conformal check, which today
expects a versor (a Mycelium addition, pending; its release is the
operators' decision).

## 5. Build

1. `conductor::court::transitions(before, after, bell)`: the transitions of
   one bell, by shell, with the release and return as fluxes.
2. `ns-hypha` offers them: `verify_local` on each, then the court's
   `verify_both`; the probe retires.
3. Witness: a bell of a fresh founding gives transitions that
   `verify_sized` reads with no breach and no unmeasured law; a mote moved
   off its shell's turn is named by Conformal, a mass created by
   Continuity.
