# 116 — The placeholders deduced

Each value [PHYSICS.md](PHYSICS.md) §5 lists as not yet derived, read in its
code and replaced by a law already in the record where one closes. Four of
them rest on one hidden assumption, an isotropic body; with each body's
tilt carried as its lead, they go together.

**Status:** deduced; built in two passes (§6).

## 1. Already derived, not built (docs/104 §1.2)

| Placeholder | Replacement |
|---|---|
| A body's push `½·Q/(A + Q)` | `Q/(A + Q)`: its open frames over its whole |
| The core's push, "regenerated ½" | `S/(S + T)`: its state against what its throat still holds |
| The release `s = ½·c/(c + base)` | `s = p·c/(c + base)`, `p` the core's push above |
| The heating face `½(1 − δ)` | `u·e`: the unlocked share times the body's own open share |

These change the founding (the release and every push), so they are built
on their own, with a read (§6, pass 2).

## 2. The isotropic body

| Placeholder | Where | Deduction |
|---|---|---|
| A third in each plane | `flows::tilt_of`, where a body carries no tilt | A body with no lead of its own stands at its enclosure's plane, the attractor's fixed point (E38); its torsion is 0. |
| A mean over the three planes, `/3` | `flows.rs:1194`, `formation.rs:1169` (contact over the planes) | Each plane weighted by the tilt, `Σ_k t_k·(…)`, as `flows::bonded` reads `C_Δ`. The planes count equally only for an isotropic body. |
| `bell·(2/3)·d_k`, the release's base phase | `breath.rs:1889` | The motion law, `2·d_k` turns a bell, shared over three planes isotropically: `2·d_k·(1/3)`. Per plane it is `2·d_k·t_k`, `t_k` the core's lead, which the release already writes into the mote's tilt. |

## 3. Compounding in place of `ln 2`

`life::lost` approximates `e^(−λ)` by whole halvings with `1/ln 2 ≈
1442/1000`; `life::halvings` and `life::window_echoes` use `ln 2 ≈ 693/1000`.
Netstratum's law for a share kept over repeated passes is share compounding,
`(1 − x)^n` over the body's own rings (Mycelium's `forms::shares`, E25), and
`compound::rings_until` counts the rings until a share falls below a level.
Each use is read on its own rings with its own per-ring share; the
logarithm leaves the code.

## 4. Read, not imported

| Placeholder | Deduction |
|---|---|
| The tier shares (`pulse::tier`) | Only `tier(1).medium` (ε = 1/100) is still read, by `life::window_echoes`, which is the retired shedding law: breath sheds by the coherence drop (`breath.rs:1310`). Re-derived on that law with compounding (§3), `tier` has no caller and retires. |
| `BELL_YEARS` 97 850 | The home year is the home world's local year (its own comment): years per bell is `1/P_home`, the period in bells (`clocks::local_years`), read once a home world stands. |
| `HYDROGEN_K` 157 800, the 279 K anchors | Kelvin serves only era 0's thermal bands. On era 1's own scale a phase state is heat against grip, `a + τ` against `κ_w·h` (docs/100 §9.2). Kelvin retires with era 0's thermal model (#32). |

## 5. What stands, what needs a read, what does not close

**Stands** (a derivation already):
- the enfold cap `d/4` (the rim law, rim 2 in an enclosure's own units, and
  gravity's reach);
- the luminal ½ and the founding's phase (the inversion's fixed point read
  where it holds, docs/104 §1.2);
- the 60⁻ᵏ ladder and the 60⁶ grain (the carrier's resolution, not physics).

**Needs one more read:**
- the double pass, `ν = 2·r/T_u` and `nuclides::source_clock` = 2: the 2 counts
  the two faces a ring is read on, which holds only where every ring passes
  both; under docs/114 T4 the passage is an event, so it is the faces
  passed;
- the galaxy winding `4δ` an echo (`pulse.rs:1291`): the motion law's `2δ`
  times the same double pass, apparently; a duplicate of `quantum::twist`;
- `T_u/4`, a core ring in bells (three copies): **derived**. At the rim the
  reach equals the holding within (the rim law, deficit ½), so
  `a(R_rim) = 2·R_rim` and `T_u = 2·a(R_rim) = 4·R_rim`: the ring is the rim
  itself, `T_u/4 = R_rim` (`pulse::established_period`). Read: `R_rim`
  1.136 and 1.250 at the first two echoes, against the stored `T_u` 186.

**Does not close:**
- `CONTACT_K` (a contact's grip): two atoms meeting without pairing; docs/115
  gives no grip for it;
- `BLUEPRINT_DOUBLINGS`, `CELL_BITS`, `GENOME_BITS`, `VIABLE_PEOPLE`: the life
  layer's information anchors; no physics derives them.

## 6. Build

1. **Pass 1, the assumptions (§2–§4):** the isotropic body, compounding,
   `window_echoes` on the coherence law and `tier` retired. A before/after
   read on a fresh 2^40 universe: conservation, the identities, systems and
   worlds, the carrier's widths, the time.
2. **Pass 2, the pushes (§1):** the founding re-derived, with its own read.
3. `BELL_YEARS` and kelvin with #32.
