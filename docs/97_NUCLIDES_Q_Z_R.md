# 97 — Nuclides: Q the proton, Z the neutron, R the electron's phase

Design direction (2026-10-26): the nuclides go beyond a single Z number, to
weights up to 1200, so that galaxies are well formed with enough resources to
build planets; this is built as the inverse of Xylarium, Q the proton, Z the
neutron and R the electron's phase state, carried by p as the pulse's
reverberation of the core's echo.

Status (2026-10-28): §1–§2, §4 built (41b2a6b); §3 R built (785a074, §13); §5 in nature's form; §8–§11 built; §12 built and partly superseded (its table cap 1200 by §14, ff0c985; "the galaxies carry the whole mix" retires with docs/101); §13 built (785a074, cd16484); §14 built (ff0c985).

Read against: Xylarium spec 11.0 (53) (Z the proton, Q the neutron, R the
electron's phase; ruled there) and xyl-cosmos `fibres.rs` (ends on S^11,
fibres, seals). Netstratum is the inverse: Q the proton, Z the neutron.

## The seam with Xylarium

The letters are swapped deliberately (operator, relayed by Xylarium
2026-10-26: Netstratum runs opposite to Xylarium, the Z/Q letters swapped
for the nucleons; Xylarium spec 11.0 (53), 5e4db24): Netstratum Q = proton,
Z = neutron; Xylarium Z = proton, Q = neutron; both R = the electron's phase.
Anything crossing between the kernels in Z/Q terms (a shared law, a bench, an
anastome reading) is translated at the seam, Z ↔ Q, and labelled on each side
("Q(proton)").

## 0. Why

The founding today is a chain over the charge alone, `count_Z =
count_{Z−1}·½·held_Z`, from 2^55 grains (5.23·10^72 nuclei): it ends at
Z 210 with a mean weight of 5.7 (Z ≈ 1.9) a nucleus. Matter is almost all
light; there is not enough heavy matter to build worlds.

## 1. A kind is a nuclide

- **Q, the proton:** the charge, the open pole. It sets the electrons and so
  the atom and its chemistry (`atom(Q)`).
- **Z, the neutron:** the held pole, neutral, the glue.
- **Weight `w = Q + Z`**, its nucleons: the mass gravity pulls and inertia
  answers. It replaces `3·charge`.
- **Conserved:** nucleons, through every fusion, capture and peel. Charge
  moves between Q and Z only with the electron phase (R) carrying the change.

## 2. The nucleus converges on S^11 (Xylarium's fibres, inverted)

Nucleon ends arrive one at a time on S^11, whose degree-l harmonics give
`d(l) = C(l + 11, 11) − C(l + 9, 11)` independent directions (1, 12, 77,
352, …; the same count as our atom's orientations).

- An end first claims a free direction alone. Lone ends are open: in
  Netstratum they are protons (Q).
- When every direction of the open degree is claimed, the next end binds into
  a lone end: a fibre, one proton and one neutron counter-turning.
- A degree closes when every direction holds a fibre; turning n holds the
  degrees 0..n−1.

So a nucleus of weight w has `Z = G` (its fibres) and `Q = G + free`, one
nuclide per weight, read off the process, never a table. Isotopes rise by
themselves: while a degree binds, Q holds and Z grows.

| w | Q | Z | κ = 2G/w |
|---|---|---|---|
| 1 | 1 | 0 | 0 (hydrogen) |
| 2 | 1 | 1 | 1 (seal) |
| 3 | 2 | 1 | 2/3 |
| 28 | 14 | 14 | 1 (seal) |
| 208 | 104 | 104 | 1 (seal) |
| 1092 | 546 | 546 | 1 (seal) |
| 1200 | 636 | 564 | 0.94 |

**Coherence** `κ = 2G/w`, the bound share; 1 at every closed degree.

## 3. R, the electron's phase state

`R = κ_e·(1 − 2·frac(p))`:

- `κ_e` the electrons' own fibred share on S^11 (the same convergence, Q
  electrons);
- `p` the pulse: the reverberation of the core's echo at the body's place
  (the phase of its galaxy's bell there, `Body.bell`);
- `1 − 2·frac(p)` the hemisphere law's response.

R > 0 bound, R < 0 freed, jumping: chemistry and light live here. The open
frames that set the push (`½·open/whole`) are R's domain, read from the
electrons, never from the nucleus.

## 4. What changes in the code

- **The kind:** an atom is named by its weight (`W0001` … ), its Q and Z read
  from the convergence; clusters as now (`W0016·2W0001`).
- **`chem::weight`** = Q + Z (nucleons), summed over a cluster's parts.
- **`chem::charge`** = Q, summed.
- **The atom, unpaired frames, hold and tightness:** read from `atom(Q)` as
  now.
- **Fusion:** w₁ + w₂ → the nuclide of weight w₁ + w₂ (nucleons conserved; the
  change of Q carried by R).
- **Every gravity and ledger site** that reads `3·charge` reads nucleons.

## 5. Open — the abundance law (for ruling)

The founding must spread the bounce's nucleons over weights to ≥ 1200 while
conserving them and leaving the hydrogen stars burn. Read on 2^55 grains:

| law | reaches | conserves nucleons | hydrogen |
|---|---|---|---|
| today's: `count_w = count_{w−1}·½·held` | Z 210 (w ≈ 600) | yes | 27% of the weight |
| `count_w = count_{w−1}·½(1 + κ_w)` (a capture survives bound, or lone on the held hemisphere) | w 1644 | no (counts sum past the nucleons) | — |
| the same as a capture chain, normalized to the nucleons | w ≫ 1200 | yes | none left (w = 2 seals, so every proton is captured to deuterium) |

The survival share `½(1 + κ)` derives from the hemisphere law and reaches
weight 1200. What is missing is how a capture consumes a free nucleon:
the chain must draw each capture from the free protons, so hydrogen stands
where the free pool is left. Candidates:

- **(a)** captures stop when the free protons are spent: the pool depletes as
  it feeds the chain, and what remains is hydrogen;
- **(b)** each level keeps the lone (unbound) share of what arrives, `1 − κ`,
  and passes on the bound share: hydrogen (κ = 0) keeps all its arrivals
  until the pulse turns it.

## 6. Build order (after §5 is ruled)

1. The nuclide convergence (`chem` or a new `nuclides.rs`): Q(w), Z(w),
   κ(w), exact; witness: the seals at 2, 28, 208, 1092.
2. The kind by weight; `weight`, `charge` read from it; fusion by weights.
3. The founding by the ruled abundance law; witness: nucleons conserved,
   weight reaches 1200.
4. R as the electron's phase, read at the galaxy's bell.
5. Re-read every identity and a founding.

## 7. Built: the founding in nature's form (direction of 2026-10-26: align on nature)

As nature's: the bounce leaves free protons and neutrons; captures run only on
the neutrons and stop when they are spent; the heavy weight is the stars' to
build over generations.

- **The neutron share:** a quarter of the nucleons, the unlocked face of the
  bounce's ring, `½(1 − δ)` at δ = ½. For ruling: nature's freeze-out leaves
  about an eighth (helium about a quarter of the mass; ours gives a half).
- **The captures:** each neutron binds a proton where the nucleus first seals,
  deuterium (w 2); deuterons pair to the next seal, w 4 (Q 2, Z 2), by the
  one law the stars burn by, `w + w → 2w`; an odd deuteron stands.
- **Hydrogen** is what no neutron reaches. Nucleons conserved exactly.

Read on 2^55 grains: hydrogen and weight 4 only. The cascade (`pulse::cascade`)
then sorts kinds into shells by weight, so the universe stands as two
galaxies of one element each. That is the next misalignment: nature's
galaxies share the primordial mix (docs/78: "every share carries the
manifest's composition in proportion").

## 8. Bounce nucleosynthesis: three components (direction of 2026-10-26: the isomorphic, unitary and automorphic components each landed, so that each is correct)

Each component is its own, with its own witness (`nuclides.rs`,
`the_three_components_stand_apart_and_each_holds`):

- **Automorphic, the structure.** A nucleon's pole is its leg: on the
  continuum leg (open, R forward) a proton, on the vacuum leg (held, R
  backward) a neutron. Conversion is the Möbius pass (`(δ, ψ, s) ↦ (1 − δ,
  ψ + ½, −s)`), an involution: `M∘M = id`. The swap Q ↔ Z maps the nucleus's
  convergence onto itself (every fibre one of each; every seal its own
  image). Nothing is imported from nature.
- **Unitary, the books.** One exchange a pass: protons cross to the vacuum leg
  by the held face, `p·δ`; neutrons to the continuum by the unlocked face,
  `n·½(1 − δ)`; whole nucleons, both sides booked, `p + n` unchanged.
- **Isomorphic, a reading only.** The founding's outcome set beside nature's
  (neutrons per proton, the weight-4 mass share, hydrogen's) and reported at
  the founding, never imposed.

The seal window and the bounce's heat: §9 (built).

## 9. The bounce's heat and the seal window (direction of 2026-10-26)

- **The heat.** A body at deficit δ turns `2δ` of its orbit a bell; the
  luminal bound allows one whole turn, so inside the ring the excess cannot be
  motion and is agitation: `a = max(0, 2δ − 1) = (H − R)/(H + R)`. Hot at the
  core (1), cold at the ring (0), cooling as the bounce's bubble expands; the
  substrate weight's form, first degree (`nuclides::bounce_heat`).
- **The seal window.** A fibre's two counter-turning ends part where the heat
  reaches half a turn, the ignition law's bound `a ≥ ½` one tier down; so the
  seal first holds where `2δ − 1 = ½`, δ = ¾ (`nuclides::seal_window`).
- **The neutron share** is the unitary exchange's own fixed point there,
  `p·δ = n·½(1 − δ)`, `n/(p + n) = 2δ/(1 + δ)` = 6/7 (`nuclides::neutron_share`):
  neutron-rich, the mirror of nature's, as the letters are.
- **The captures at the window:** each proton binds a neutron (deuterium, the
  first seal) and deuterons pair to weight 4; an odd deuteron stands. The
  unpaired neutrons return to the continuum by the unlocked face past the
  window, protons: hydrogen.

Read on 2^55 grains (the isomorphic reading, beside nature): neutrons per
proton among the nucleons 1/6 (nature about 1/7); weight 4 by mass 2/7
(about 1/4); hydrogen 5/7 (about 3/4). The static quarter is retired.

## 10. The heavy weight: the stars' captures (direction of 2026-10-26: weights up to 1200)

The founding makes weights 1, 2 and 4 only, as nature's. Everything heavier is
the stars' to build and return to their galaxies' gas, generation on
generation. One law, the nucleus's own convergence: a capture adds one
nucleon, `w → w + 1` (a proton where the end claims a direction alone, a
neutron where it binds; the pole is the leg, `nuclides::mobius`), and it
survives by the hemisphere law, bound or lone on the held hemisphere:
`σ_{w+1} = ½(1 + κ_{w+1})`. The seals (κ = 1) keep every capture: matter piles
there (28, 208, 1092), the abundance peaks read off the process. Every capture
draws one hydrogen nucleus, so nucleons are conserved exactly.

- **The slow capture (in a burning star):** each ring its clock receives, every
  nuclide w its heart holds passes `f_H·c·σ_{w+1}` of itself to w + 1, f_H
  hydrogen's share of the heart's nuclei, c the star's coherence with the echo
  (`nuclides::capture_slow`).
- **The rapid capture (at a star's death):** the matter a star returns whole
  (spent or decohered) runs the chain at once, each weight in turn passing
  `σ_{w+1}` of what stands there upward, until its hydrogen is spent
  (`nuclides::capture_rapid`).
- **Recycling** returns both to the galaxy's gas, so every generation is born
  heavier.

## 11. The heat and the grip as readings (ruling of 2026-10-26: `2δ − 1`, passing half a turn, stood as a physics constant)

- **The heat is read on the body's own clock.** A body at deficit δ receiving ν
  rings a bell (its own nested clock: the universal rate times each well's
  `r/(r + H)` down its chain, two rings a bell by the ring law) turns `ν·δ` a
  bell; the luminal bound (SUBSTRATE_C, one whole turn a K-step, the
  grammar's ruled bound) leaves the excess as agitation, `a = max(0, ν·δ −
  1)`. At the source ν = 2.
- **The grip is the nucleus's own.** A fibre is two counter-turning ends
  sharing one whole turn, half each; a nucleus holds by the share of it its
  fibres bind, so its grip is `½·κ_w`, κ_w = 2Z/w. Sealed nuclei grip the
  most; unsealed heavy ones part first. A nuclide holds while `a < ½·κ_w`.
- **The bounce's seal window** is where deuterium (κ = 1) first holds on the
  source's clock: `ν·δ − 1 = ½`, δ = 3/(2ν) (¾ at ν = 2), now read, never set.
- **A dying star's release:** each nuclide of its heart whose grip its heart's
  heat exceeds (`a = ν_star·h_s − 1`, ν_star its own clock, h_s its heart's
  share, its deficit at its centre) breaks into its nucleons; they are the
  rapid capture's fuel for the rest. Nucleons conserved.

## 12. A fresh founding with the full table to weight 1200 (direction of 2026-10-27: retire the starter galaxies and found afresh with the full table to weight 1200)

> Superseded in part: the cap at 1200 by §14 (the densest element is the
> founding's own, 1612 at 2^55 grains); the whole mix by docs/101 (no
> leftovers; each galaxy's composition its own).

- **The light, as §9:** the neutron share the exchange's fixed point at the
  seal window, each proton bound into a seed of weight 4 (deuterons paired;
  an odd one stands).
- **The table, from the same window:** the free neutrons the pairing leaves
  are captured by the seeds as they climb the nucleus's convergence. A seed
  arriving at w stays there with `1 − σ_{w+1}` and climbs on with `σ_{w+1} =
  ½(1 + κ_{w+1})` (the hemisphere law), so the seeds spread over every weight
  from 4 to 1200 (the operator's table; seeds reaching 1200 stand there), peaked
  at the seals. Each climb is a neutron captured; the neutrons the seeds do
  not take return to the continuum as hydrogen. Nucleons conserved exactly.
- Read: hydrogen about 0.45, weight 4 about 0.03, the table above 4 about 0.52
  by mass; every weight to 1200 stands (at 1200, about 10^-63 a nucleon).
- **The galaxies carry the whole mix** (docs/78: "every share carries the
  manifest's composition in proportion"): the cascade's beads keep their
  places and their masses, each taking the founding's composition in
  proportion to its mass, never a slice by weight.

## 13. R read, never averaged (ruling of 2026-10-27: formulas, never constants or static variables)

The ½ in the survival `½(1 + κ)` and in the grip `½·κ` was the hemisphere law
averaged over the electron's phase: the mean of `(1 + R)/2` over a whole turn
of p. Read at the body's own phase instead:

- **R, the electron's phase state (§3, built):** a nuclide of weight w holds
  `Q_w` electrons; they converge on S^11 as its nucleons do, so their fibred
  share is `κ_e = 2·Z(Q_w)/Q_w` (the same convergence on Q ends; 0 for one
  electron). At the pulse phase p,
  `R = κ_e·(1 − 2·frac(p))` (`nuclides::electron_phase`).
- **p, the pulse at the body's place:** its phase on the R loop
  (`flows::LOOP`) against the echo reaching it, its galaxy's bell there:
  `p = φ_LOOP − bell_LOOP`. At the bounce, the founding's phase is the turn
  its nucleons have made when the seal window opens, on the source's clock:
  `p = ν·δ_window = 1 + grip_2` (3/2 at ν = 2), so frac(p) = ½ and R = 0:
  the bounce is hot, its electrons free of the bound face. (Read first at
  p = 0, fully bound, the founding climbed nearly all its seeds and left
  hydrogen 0.04 by mass: the error the reading corrects.)
- **The held hemisphere** `h = (1 + R)/2` (`nuclides::held`): the share of a
  lone end standing on the held face. Its mean over p is ½, the old static.
  > **Corrected (2026-10-28, docs/104 §1.1):** `(1 ± R)/2` is a halving. The held
  > share is the count of the electrons' ends on the held face,
  > `h = (F_e + L_e·in_step(p, φ_held))/Q`; the built `(1 + R)/2` (785a074)
  > stands until the count is built with the founding re-derived on it.
- **Survival** `σ_{w+1} = κ + (1 − κ)·h` (bound, or lone on the held
  hemisphere); the seals keep every capture whatever p.
- **Grip** `κ_w·h`: a fibre's turn shared by its two counter-turning ends, the
  held end's share the held hemisphere.
- **The seal window** reads deuterium's grip: one electron, κ_e = 0, R = 0,
  h = ½, so δ = (1 + ½)/ν stands (¾ on the source's clock), now read.
- **ν, the rings received:** a star's clock is read from the rings it received
  this echo, `ν = 2·r_echo/T_u` (the 2 the double pass: each ring is read on
  both legs of the loop), never `2·rate`.

The founding, the slow and rapid captures, and a dying heart's release all
read h at their own p. A heart in step with its echo (p near 0) holds its
electrons bound (R → κ_e) and climbs further; one slipping toward the
opposite face frees them (R < 0) and its captures part.

The captures climb to the table's top and stop there, as the founding does
(`pulse::TABLE`, the operator's 1200): in step with its echo one burst reaches
it (witness: `r_is_read_at_the_pulse_and_its_mean_is_the_old_half`, and
`capture_rapid` at p = 0 reading 1200).

## 14. No fixed cap: the densest element is the founding's own (direction of 2026-10-27: compute the densest element by the formulas (in the range 1200–1700); found at 1700 with no fixed cap)

The seeds climb the convergence each passing `σ_{w+1}` of themselves on, in
whole nuclei; near the top `σ ≈ 0.89`, so the climbing seeds fall by about a
tenth a weight and the climb ends where less than one whole nucleus arrives.
The free neutrons never run short (about 3.7·10^72 left at 2^55 grains).
Read at 2^55 grains: the last nucleus stands at **weight 1612** (Q 976, Z
636): 1600 → 18 arriving, 1606 → 7, 1611 → 2, 1612 → 1 (it stays). A larger
founding reaches a few weights higher a decade of grains.

- **`pulse::TABLE` retires.** The founding climbs until no whole nucleus
  arrives (bounded only by the convergence's read).
- **The universe records its densest element** (`Universe.densest`, tag 25),
  the founding's heaviest nuclide; every later climb stops there: the
  stars' slow and rapid captures, and the burn's product `w + w → 2w` (a
  rung whose product passes it does not run). Nucleons conserved.
- Beside it (the cascade's own reading at 2^55 grains): the Wellspring holds
  every kind whose spread stays in its throat, `(K + s_i)(1 + s_final)/(w·H)
  ≤ 1`, K = 266, H = 2, s_final = 1/6: weight 171 and up, 1434 of 1600
  kinds; their mass about 6·10^-14 of the whole.
