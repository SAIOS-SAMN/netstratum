# 113 — The bounce on the court: era-1 steps as Mycelium transitions

The court verifies by reconstruction: it rebuilds an object under a law
whose form it holds and reaches the same exact object. This document states
the era-1 universe's step, the reverberation of the bounce in the
Einstein–Cartan core, as the transitions the court rebuilds under
Mycelium's laws (L0, `laws::verify`).

**Status:** specified; one dependency (§4). It replaces the fixed two-body
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
| `motion` | the turn the bell gives that shell: one versor per shell, on each spatial plane its turning `t_k·δ` a received ring (E1, E20), composed over the planes | `flows::turn` |
| `fluxes`, `Source` | the release from the core's throat into the core, at the throat, its mass the released holding | E18 |
| `fluxes`, `Sink` | what returns to the throat: recycling and infall | E30 |
| `hues` | the fibre relation of the motes' kinds, where a kind changes (fusion, release, capture) | E25, E28, E29 |

One transition is one shell for one bell: the motes of a shell turn
together (one δ), so every pair's inversive distance is kept (Rigidity) and
one versor carries them all (Conformal).

## 3. What each law checks here

| Law | Era-1 statement |
|---|---|
| Conformal | the shell's turn is a versor that carries every mote's place |
| Momentum | what the motes exchange cancels; the core's reservoir pays the torque (`Σ m·ℓ + reservoir` exact, E20) |
| Rigidity | the shell turns as one: every pair of its motes keeps its inversive distance |
| Time | the bell advances |
| Reservoir | a change of kind is unitary on the fibre |
| Continuity | the shell's mass after is its mass before, plus the throat's release, less what returned (3Z conserved, docs/PHYSICS.md §6) |

## 4. The dependency: a turn as an exact versor

`rotator(s, b)` is exact for a rational half-angle tangent `b/s`. Era-1
phases are turn fractions read with S¹'s linear measure (`flows::in_step`),
so a shell's turn `t_k·δ` becomes an exact versor only through a chart that
maps a turn fraction to the rotor's coordinates exactly. Mycelium's v0.3.0
carries a turn-native chart reader; this mapping uses it.

## 5. Build

1. `conductor::court::transitions(before, after, bell)`: the transitions of
   one bell, by shell, with the release and return as fluxes.
2. `ns-hypha` offers them: `verify_local` on each, then the court's
   `verify_both`; the probe retires.
3. Witness: a bell of a fresh founding gives transitions that
   `verify_sized` reads with no breach and no unmeasured law; a mote moved
   off its shell's turn is named by Conformal, a mass created by
   Continuity.
