# 97 — Nuclides: Q the proton, Z the neutron, R the electron's phase

This document defines the nuclides of era 1: each kind is a nucleus of Q
protons and Z neutrons, with R, the electron's phase state, carried by the
pulse share p as the reverberation of the core's echo. The set extends
beyond single-Z kinds to weights of 1200 and above (by §14, to the densest
element the founding produces), so that galaxies form with enough material
for planets. Netstratum is the inverse of Xylarium: in Netstratum Q is the
proton and Z the neutron; in Xylarium the reverse.

**Status (2026-11-16):** §15 scopes the table on its own dimensions, up to weight 1700–2500 (#213, open). It supersedes §14's one-chain climb as the target. Derived: §15.6–§15.9. The reach is fuel-bound (§15.11–§15.12). Next: V6, the bounce's places. The code still climbs one chain to 1508.

**Status (2026-10-28):** §1–§2 and §4 built (41b2a6b); §3, R, built
(785a074, §13); §5 in nature's form; §8–§11 built; §12 built and partly
superseded (its table cap of 1200 by §14, ff0c985; "the galaxies carry the
whole mix" retires with docs/101); §13 built (785a074, cd16484); §14 built
(ff0c985).

**Read against:** Xylarium spec 11.0 (53) (Z the proton, Q the neutron, R
the electron's phase; defined there) and xyl-cosmos `fibres.rs` (ends on
S^11, fibres, seals).

## The seam with Xylarium

The letters are swapped deliberately, because Netstratum is built as the
inverse of Xylarium (Xylarium spec 11.0 (53), 5e4db24): Netstratum Q = proton,
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
352, …; the same count as the atom's orientations).

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

## 5. Open — the abundance law (to be decided)

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

## 6. Build order (after §5 is decided)

1. The nuclide convergence (`chem` or a new `nuclides.rs`): Q(w), Z(w),
   κ(w), exact; witness: the seals at 2, 28, 208, 1092.
2. The kind by weight; `weight`, `charge` read from it; fusion by weights.
3. The founding by the chosen abundance law; witness: nucleons conserved,
   weight reaches 1200.
4. R as the electron's phase, read at the galaxy's bell.
5. Re-read every identity and a founding.

## 7. Built: the founding in nature's form

As nature's: the bounce leaves free protons and neutrons; captures run only on
the neutrons and stop when they are spent; the heavy weight is the stars' to
build over generations.

- **The neutron share:** a quarter of the nucleons, the unlocked face of the
  bounce's ring, `½(1 − δ)` at δ = ½. Open: nature's freeze-out leaves
  about an eighth (helium about a quarter of the mass; Netstratum's gives a
  half).
- **The captures:** each neutron binds a proton where the nucleus first seals,
  deuterium (w 2); deuterons pair to the next seal, w 4 (Q 2, Z 2), by the
  same law the stars burn by, `w + w → 2w`; an odd deuteron stands.
- **Hydrogen** is what no neutron reaches. Nucleons conserved exactly.

Read on 2^55 grains: hydrogen and weight 4 only. The cascade (`pulse::cascade`)
then sorts kinds into shells by weight, so the universe stands as two
galaxies of one element each. That is the next misalignment: nature's
galaxies share the primordial mix (docs/78: "every share carries the
manifest's composition in proportion").

## 8. Bounce nucleosynthesis: three components

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

## 9. The bounce's heat and the seal window

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

## 10. The heavy weight: the stars' captures

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
  nuclide w its star core holds passes `f_H·c·σ_{w+1}` of itself to w + 1, f_H
  hydrogen's share of the star core's nuclei, c the star's coherence with the echo
  (`nuclides::capture_slow`).
- **The rapid capture (at a star's death):** the matter a star returns whole
  (spent or decohered) runs the chain at once, each weight in turn passing
  `σ_{w+1}` of what stands there upward, until its hydrogen is spent
  (`nuclides::capture_rapid`).
- **Recycling** returns both to the galaxy's gas, so every generation is born
  heavier.

## 11. The heat and the grip as readings

In §9, `2δ − 1` and the half-turn bound stood as constants; here both are
read on the body's own clock.

- **The heat is read on the body's own clock.** A body at deficit δ receiving ν
  rings a bell (its own nested clock: the universal rate times each well's
  `r/(r + H)` down its chain, two rings a bell by the ring law) turns `ν·δ` a
  bell; the luminal bound (SUBSTRATE_C, one whole turn a K-step, the
  grammar's bound) leaves the excess as agitation, `a = max(0, ν·δ −
  1)`. At the source ν = 2.
- **The grip is the nucleus's own.** A fibre is two counter-turning ends
  sharing one whole turn, half each; a nucleus holds by the share of it its
  fibres bind, so its grip is `½·κ_w`, κ_w = 2Z/w. Sealed nuclei grip the
  most; unsealed heavy ones part first. A nuclide holds while `a < ½·κ_w`.
- **The bounce's seal window** is where deuterium (κ = 1) first holds on the
  source's clock: `ν·δ − 1 = ½`, δ = 3/(2ν) (¾ at ν = 2), now read, never set.
- **A dying star's release:** each nuclide of its star core whose grip its star core's
  heat exceeds (`a = ν_star·h_s − 1`, ν_star its own clock, h_s its star core's
  share, its deficit at its centre) breaks into its nucleons; they are the
  rapid capture's fuel for the rest. Nucleons conserved.

## 12. A fresh founding with the full table to weight 1200

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
  from 4 to 1200 (the table, `pulse::TABLE`; seeds reaching 1200 stand there), peaked
  at the seals. Each climb is a neutron captured; the neutrons the seeds do
  not take return to the continuum as hydrogen. Nucleons conserved exactly.
- Read: hydrogen about 0.45, weight 4 about 0.03, the table above 4 about 0.52
  by mass; every weight to 1200 stands (at 1200, about 10^-63 a nucleon).
- **The galaxies carry the whole mix** (docs/78: "every share carries the
  manifest's composition in proportion"): the cascade's beads keep their
  places and their masses, each taking the founding's composition in
  proportion to its mass, never a slice by weight.

## 13. R read, never averaged

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

The founding, the slow and rapid captures, and a dying star core's release all
read h at their own p. A star core in step with its echo (p near 0) holds its
electrons bound (R → κ_e) and climbs further; one slipping toward the
opposite face frees them (R < 0) and its captures part.

The captures climb to the table's top and stop there, as the founding does
(`pulse::TABLE`, its cap 1200): in step with its echo one burst reaches
it (witness: `r_is_read_at_the_pulse_and_its_mean_is_the_old_half`, and
`capture_rapid` at p = 0 reading 1200).

## 14. No fixed cap: the densest element is the founding's own

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

## 15. The table on its own dimensions: the scope (open, #213)

**Operator, 2026-11-16:** "the elemental table (which needs
multi-dimensional reconfiguration) needs to be up to weight 1700–2500 (not
bound to earthly elements) to form the universal core properly." Then:
"scope the resolution and definition to be exhaustive with opened vents."

**Why §14 falls short.** The climb is one chain. Each weight holds one
nuclide, and the seeds pass on `σ_{w+1} ≈ 0.89` of themselves at each
weight, so the population thins geometrically. The read is weight 1508 at
2^40 grains (l109: Q 872, Z 636; weights held 7..=1470) and 1612 at 2^55.
A larger founding gains only a few weights a decade of grains, so no
founding reaches 1700–2500 on one chain. The range is reached by opening
the table's dimensions, never by a cap or a count.

**Each vent below:**
- states where the table is collapsed today (the code);
- states the dimension to open (the resolution);
- states its witness (the definition of done).

No Earth table enters. Every channel and share is one of the founding's own
formulas or its inverse, and every share is placed by a formula (no leftovers).

### 15.1 The nuclide's own axes

| # | Collapsed today | Opened | Witness |
|---|---|---|---|
| V1 | One nuclide per weight. `nuclides::table` walks the convergence one end at a time and returns one `(Q, Z)` for each w. `chem::Kind::atom(w)` and a kind's parts `(count, w)` key matter by weight alone. | A nuclide is its point `(Z, Q)` on the convergence. Every pair the convergence's places hold is a kind: isotopes (one Q, many Z) and isobars (one w, many Q). Kinds are keyed by `(Z, Q)`, and weight is the reading `Z + Q`. | `Kind` round-trips `(Z, Q)`. Two nuclides of one weight are distinct kinds. `chem::weight` is `Z + Q` for every kind. |
| V2 | R, the electrons' phase, is read from the charge alone (`electron_phase(w, p)`) at one pulse phase. The manifest states R = 0. | R is the nuclide's third axis: the electrons' phase share signed by face (#205). A kind is `(Z, Q, R)`, and R stands at its rest point `R = −(Z − Q)/(Z + Q)` unless a formula moves it (mycelium NotAtRest, #357). | Every founded kind's R is at its rest point, or is moved by a named formula. Manifest v3 entries carry R exact. |
| V3 | A nucleus is one `(Z, Q)` pair for the whole body. There are no frames per plane (#110), and 3Z/3Q is restated but not held (#176). | The frames per plane `(z_k, q_k)` on the three planes, founded evenly `((3Z − u)/3, u/3)` and moved on one plane by each capture or exchange. 3Z/3Q is held, with R beside it. | `Σ_k (z_k + q_k) = 3w` for every nucleus. The pole swap `(z_k, q_k) ↦ (q_k, z_k)` is an involution keeping 3Z (docs/123 §2). |

### 15.2 The climb's channels and shares

| # | Collapsed today | Opened | Witness |
|---|---|---|---|
| V4 | One channel. `pulse::founding` climbs only by a neutron captured (`w → w + 1`, every climb "a neutron captured"). | The branches the founding's own formulas give, each with its inverse:<br>- **capture:** a neutron, `Z + 1`, at the held face; a proton, `Q + 1`, where free protons stand;<br>- **pole exchange:** one nucleon's pole swapped, `(Z, Q) ↦ (Z ∓ 1, Q ± 1)`, by the master equation's `g` and `u` (`nuclides::exchange`, applied per nucleus, not to the pool alone);<br>- **freeing:** a nucleon leaving at the unlocked face, the inverse of capture;<br>- **parting:** a heavy nucleus splitting as the merger's inverse (`coordinate::fission`), where its hold no longer keeps it.<br>The arriving population spreads over every branch instead of thinning along one. | Nucleons are conserved exactly over the founding. Charge is booked through every exchange. Each channel composed with its inverse is the identity on the books. |
| V5 | One survival per weight: `survival(w, p) = κ_w + (1 − κ_w)·g`, a scalar at one held share. | A survival for each nuclide and channel, `σ(Z, Q, R; channel)`, read by component: each plane's fibres bound at their own share, and the lone ends at the electrons' equilibrium. | It reduces to §14's σ on the one-chain path. Shares over a nuclide's channels sum to 1, staying included. |
| V6 | One place. The founding crosses at the throat's held share `p₀ = polarity::throat_held()` and reads no other place. | The bounce's own places. Each part of the climb is read at the held share of the place the front meets, on its coordinate `τ(r) = r + H(<r)` (docs/132 §4.1, E77). The table is the sum of what every place makes. | At one place the sum is §14's table. Nucleons summed over places are conserved. |
| V7 | One clock. One seal window and one neutron share for the whole founding (`neutron_share(seal_window(source_clock()))`). One pulse phase. | The exchange's fixed point at each place and time: the seal window read where each part of the bounce cools on its own clock, with the pulse phase per nucleus (§13). | Read at one place and time it is §8–§13's fixed point. The neutron share is never set. |
| V8 | The free nucleons left are all returned as hydrogen (`w = 1`). | Each free nucleon is placed by the exchange formula: a free neutron, a free proton, or captured, by its face. There is no remainder assignment. | The free nucleons' books close by the formula. Hydrogen is what the formula leaves, not the rest. |

### 15.3 The table's bounds and its readers

| # | Collapsed today | Opened | Witness |
|---|---|---|---|
| V9 | The read is bounded by weight (`nuclides::READ = 8192`). `Universe.densest` is one weight, and it caps every later climb: the stars' slow and rapid captures, and the burn's `w + w → 2w`. | The read is bounded on the convergence's degrees, not on weight. The densest is the table's frontier: the set of `(Z, Q)` edges where no whole nucleus arrives. Each later climb stops at its own frontier. | The frontier is the founding's own reading, never set. A star's capture past the frontier does not run. |
| V10 | Every reader keys by weight: `chem::weight`, `atom_w`, `capture_slow` and `capture_rapid`, the burn's rungs, the plasma layer's bound share per kind (docs/134), the capture phases (E75), and `pulse::nuclei`. | Each reads `(Z, Q, R)`. The plasma layer's `b = g·E(a)/Q`, the capture chain's face and the burn's product all take the nuclide, not its weight. | One model per quantity, replaced in place. The reads below are unchanged where only one nuclide stood per weight. |
| V11 | The seam carries one nucleus per weight (manifest v3, `Nucleus::of(z, q)`, R = 0). | Manifest v3 carries the whole table `{z, q, r}`, its μ by nuclide. A crossing's checks go by conserved quantity, not kind (78.184 §6: nucleons, charge under the swap). | The manifest is granted on a fresh mesh, and READ-ENDOWMENT reads every nuclide back. |
| V12 | The antipolar face reads netstratum's table with Z ↔ Q swapped by letter only. | Xylarium's table is this table under the pole swap, per plane (V3), with 3Z kept and fusion and fission exchanged (docs/123 §2). The seam compares **exact face pairs**, never rounded nucleon counts: rounding is not an involution. Xylarium's table is keyed by Z with one rest point `Q*/Z` per Z (`circuit::Table`, `TOP_Z = 1700`), its split read on the axis plane (T24). So the swap maps a netstratum nuclide to a Xylarium parcel's face pair, and per-plane pairs join when both sides hold them (xylarium, 2026-11-16). | `swap(swap(T)) = T` exactly on the face pairs. 3Z is conserved across the tube for the whole table. |

### 15.4 What the table forms

| # | Collapsed today | Opened | Witness |
|---|---|---|---|
| V13 | The universal core holds the one-chain table. Its state `universal_state`, its hold, its rim and the first bounce are read on it. | The universal core holds the multi-dimensional table, and its state is read on it. | Read on a fresh founding: the core's state, its rim and the first bounce, before and after. |
| V14 | The heaviest weight is 1508 at 2^40 grains and 1612 at 2^55. | The frontier's heaviest weight falls in 1700–2500, read by the formulas above, never set. | `pulse::tests`: the frontier's heaviest weight is in 1700–2500, with nucleons conserved, on a fresh founding at the working count (2^40). |
| V15 | The valley of stability is missing: every kind is proton-rich (#203). | The neutron excess follows from V4–V7, with no table imported. | Read the rest point `(Z − Q)/w` across the frontier, beside nature (`beside_nature`), never fitted. |

### 15.5 Order and what stays open

1. **Derive.** V1–V3 define the nuclide. V4–V8 are the climb, derived here
   with each channel's share from the master equation (docs/105 §3) and its
   inverse. V9 is the frontier.
2. **Build in place.** Order: `chem::Kind` on `(Z, Q, R)` (V1, V2), the
   frames (V3), the climb (V4–V8), then the readers (V10). One model per
   quantity.
3. **Read.** V13–V15 on a fresh founding.
4. **The seams:** V11 with Mycelium and V12 with Xylarium, agreed before
   they are built.

Nothing here is ruled beyond the operator's range and the shared rulings
cited. If a derivation forks between two readings that the docs do not
decide, the fork is put to the operator in #213.

### 15.6 V3 derived: the nuclide on its three planes

**From the record:**
- **(a) Each nucleon stands once on every axis** (docs/95 §0). So a nuclide
  of weight w holds `3w` dimensions, w on each axis.
- **(b) A fibre is one proton and one neutron bound** (`nuclides::swapped`,
  docs/97 §2). Exchanging the poles of a fibre leaves the fibre, and only
  the lone ends change pole.
- **(c) The master equation's exchange** (docs/105 §3,
  `nuclides::exchange`) moves a held end to open at `g` and an open end to
  held at `u`. Its fixed point is the held share `g/(g + u)`
  (`polarity::equilibrium`).
- **(d) The faces under inverse gravity** at the share δ are
  `(g, u) = (1 − δ, δ)` (`polarity::faces`).
- **(e) The pass per plane:** a body's share on plane k is its own, `s_k`
  (docs/128, `conductor::strip`).
- **(f) Charge moves between the poles only with R carrying the change**
  (docs/95 §0).

**D1 (the state).** A nuclide is its fibres F, its lone ends `L = w − 2F`,
and on each axis k the lone ends held there, `ℓ_k ∈ [0, L]`. Its per-plane
counts are

    z_k = F + ℓ_k,     q_k = F + (L − ℓ_k),     z_k + q_k = w   on every axis.

*Proof.* (a) puts w ends on each axis. (b) puts each fibre's two ends, one
held and one open, on every axis, and leaves only the lone ends free to
change pole. ∎

**D2 (the readings).** Its neutrons and protons are the means over the
axes:

    Z = F + (ℓ₁ + ℓ₂ + ℓ₃)/3,     Q = w − Z

These are exact thirds, rational like Xylarium's `Q*/Z` (its face pairs,
#296). The whole-nucleon nuclide is the case `ℓ₁ = ℓ₂ = ℓ₃`. A nuclide's
pole and strength on plane k are

    σ_k = (z_k − q_k)/w,     o_k = q_k/w      (Mycelium 78.175 F1, with the seam's Z ↔ Q)

**D3 (the reduction).** Today's table is `ℓ_k = 0` on every axis: every
lone end open. The convergence's nuclide is `Q = F + L` and `Z = F`
(`nuclides::table`'s `(fibres + free, fibres)`). This is why every founded
kind is proton-rich (#203): no lone end is ever held.

**D4 (the parting).** The exchange (c) acts on each plane's lone ends at
that plane's own faces (d, e). So `ℓ_k` relaxes toward its fixed point

    ℓ_k*/L = g_k/(g_k + u_k) = 1 − s_k

with the fibres F and the weight w unchanged. Two planes part only where
their passes differ. Where every plane passes alike, the planes coincide,
and the nuclide is a whole-nucleon one. This is Xylarium's E014 ("the planes
part only where a winding is not common") stated on netstratum's pass.

**D5 (R per plane).** By (f), each lone end exchanged on plane k moves one
unit of charge there, carried by the electrons' phase on that plane. At
rest

    R_k* = −σ_k = −(z_k − q_k)/w

which is the rest point Mycelium checks (#357) read per plane. Its mean over
the axes is the kind's `R* = −(Z − Q)/w`.

**D6 (the swap).** The pole swap `(z_k, q_k) ↦ (q_k, z_k)` is `ℓ_k ↦ L − ℓ_k`
with F kept. It is an involution, it keeps `z_k + q_k = w` (3Z) on every
plane, and it maps `σ_k ↦ −σ_k`, `o_k ↦ 1 − o_k` and `R_k ↦ −R_k`
(docs/123 §2). On exact face pairs, `swap(swap(T)) = T` (V12).

**What it opens**
- **Isotopes and isobars** (V1): one F and w with different `(ℓ₁, ℓ₂, ℓ₃)`,
  and one w with different F, are distinct kinds.
- **The climb's pole exchange** (V4): `ℓ_k ± 1` on one plane, at that
  plane's faces.
- **The valley of stability** (V15): as a read, never a target. A lone end
  held where `1 − s_k > ½` gives a neutron excess.

**Witnesses** (with the code, `nuclides` and `chem`):
1. `z_k + q_k = w` on every axis for every nuclide.
2. The exchange keeps F and w.
3. `ℓ_k = 0` on every axis reproduces `nuclides::table` for every w ≤ READ.
4. The swap is an involution keeping 3Z.
5. At the fixed point, `R_k + σ_k = 0`.
6. Planes with equal passes stay equal under the exchange.

**For Xylarium (V3, xylarium #128).** D4 is checked against E014's parting
before any code on either side.

**Fork stated, not decided.** D4 reads the lone ends' rest at the plane's
pass `s_k` (docs/128). The founding reads one share for the whole bounce
(`neutron_share(seal_window(…))`). Here D4 governs the bound nuclide's lone
ends, and the founding's share governs the free pool. If these two must be
one formula, the operator rules it (#213).

### 15.7 The volume opened on the planes

**Operator, 2026-11-16:** "open volume on plane."

§15.6 keeps `ℓ_k`, the lone ends held on axis k. Those are three counts,
each a projection onto one axis. A lone end stands once on every axis
(15.6 (a)), so its pole on the three axes is one **pattern**: the set
`S ⊆ {1, 2, 3}` of axes it is held on. The patterns are the blades of the
axes' algebra:
- `∅`: open on every axis (the scalar);
- `{k}`: held on one axis (a vector);
- `{i, j}`: held on the plane `e_ij` (a bivector);
- `{1, 2, 3}`: held through the volume `e₁₂₃` (the trivector).

**D7 (the state is a multivector of counts).** The lone ends are

    N = Σ_S n_S·e_S,     n_S ≥ 0 whole,     Σ_S n_S = L

The axis counts are its readings:

    ℓ_k = Σ_{S ∋ k} n_S

The plane `e_ij` holds `n_ij + n₁₂₃`, and the volume holds `n₁₂₃`. Each
reading is first degree in N, and no count is squared or summed into a
scalar before it is read. Three axis counts cannot tell the patterns
apart. For example, `ℓ = (1, 1, 1)` is `n₁₂₃ = 1` (one end held through
the volume) or `n₁ = n₂ = n₃ = 1` (three ends held on one axis each).
These are different nuclides. §15.6 collapsed them.

**D8 (whole nucleons are the scalar and the volume).** A lone end held on
every axis is a neutron end. One open on every axis is a proton end. So the
whole-nucleon nuclide is `N = n_∅ + n₁₂₃·e₁₂₃`, and

    Z = F + n₁₂₃,     Q = F + n_∅

Every other pattern (an end held on one axis or on one plane) is a nucleon
end whose pole differs between axes. These are the states the opened
dimensions add. Their Z is the exact mean `F + Σ_k ℓ_k/3` (D2).

**D9 (the exchange moves an end between neighbouring patterns).** One
exchange on plane k flips one lone end's pole on axis k: `S ↔ S ∪ {k}`, at
that axis's faces `(g_k, u_k) = (1 − s_k, s_k)` (15.6 (c)–(e)). Its rest is
the product of each axis's own share:

    n_S*/L = Π_{k ∈ S} (1 − s_k) · Π_{k ∉ S} s_k

Each factor is a different axis's share, first degree in each, and none is
squared. Summed over the patterns containing k, it gives back 15.6's
`ℓ_k*/L = 1 − s_k`.
- Where the three passes are equal, the rest still holds every pattern.
  Ends held through the volume and ends held on one axis stand side by
  side. These are isotopes no axis count separates.

**D10 (the swap is the complement).** The pole swap maps each end's
pattern to its complement, `S ↦ {1, 2, 3} \ S`. Scalar and volume
exchange, and each axis exchanges with the plane it does not lie in. It is
an involution, it keeps L and F, and it keeps 3Z on every axis, since
`ℓ_k ↦ L − ℓ_k` (D6). On exact counts `swap(swap(N)) = N`, so V12's face
pairs carry N whole.

**D11 (R on the volume).** By 15.6 (f), each flip carries one unit of
charge on its axis, held in the electrons' phase there. So R is read per
axis (D5), and the volume's part of N moves charge on all three axes at
once.

**Witnesses** (with the code):
1. `Σ_S n_S = L`, and `ℓ_k = Σ_{S∋k} n_S`.
2. The flip `S ↔ S ∪ {k}` keeps F and w.
3. `N = L` (every end open) reproduces `nuclides::table` (D3).
4. The complement is an involution keeping 3Z per axis.
5. At rest, each `n_S` is its product share, and the axis readings are 15.6's.
6. Two nuclides with one ℓ and different N are distinct kinds.

**For Xylarium (V3, xylarium #128).** Netstratum's nucleon state lives on
three axes: 8 patterns, with 3 planes and the volume. R is the axis that
pairs with none (docs/95 §0). With R as a fourth axis, the orientation's
planes are the six of four axes (`orientation::PLANES`: three spatial and
three R partners). Xylarium holds an axis plane and five isoclinic planes,
also six. Whether N should extend over the four axes (16 patterns, the six
planes, four volumes and the 4-volume) or stay on the three nucleon axes
with R beside it is the mapping to check against E014. If the docs do not
settle it between the two kernels, it goes to the operator.

### 15.8 V3 against Xylarium, and the forks for the operator

**Xylarium's check (2026-11-16, xylarium #128, #296).**
- **The forms agree.** Xylarium's planes rest at `R_p = κ·(1 − 2ψ_p)`, and
  D4 with D5 gives `R_k* = −(L/w)(1 − 2s_k)`. The two match with `κ ↔ L/w`
  and `ψ_p ↔ 1 − s_k`.
- **The parting variables do not.** Xylarium's plane parts by its effective
  pass `ψ_p = frac(w_p·s·Z)` on its face, which depends on the kind's Z.
  Netstratum's `s_k` is the body's pass on axis k at its place, the same for
  every kind in the body.
- **The structures differ.** Netstratum's three axes are the thirds of 3Z.
  Xylarium holds the same three as one undivided dipole on its axis plane.
  Its five isoclinic planes are orthogonal to the Z–Q split (T24) and carry
  phase, not nucleon count.

**Held until ruled (V12 as built):** both sides state their **summed**
split: netstratum `Σ_k` over its axes (and over its patterns, 15.7), and
Xylarium its axis plane's face pairs. D6's swap, summed over k, is
Xylarium's pair swap on the axis plane, so the involution holds on both
sides.

**The forks are physics, derived in §15.9** (operator via Mycelium,
2026-11-16, MU-HOSTING §5a.2a: in-universe physics is netstratum's own
derivation, never a ruling).

### 15.9 F1–F3 derived

**F1: one formula, read at each holder's own face.** The founding's neutron
share is `nuclides::neutron_share(δ) = equilibrium(faces(δ)) = 1 − δ` at the
seal window's δ. D4's lone-end rest is `ℓ_k*/L = equilibrium(faces(s_k)) =
1 − s_k`. These are the same function, the exchange's fixed point
(docs/105 §3), at two arguments:
- the free pool rests at the bounce's face;
- a bound nucleus's lone ends rest at their plane's pass.

There is one formula and no second rule.
*Witness:* `neutron_share(x) = 1 − x = ℓ*/L at s = x` for every x.

**F2: netstratum's split is derived, and the pass sees the kind through its
phase.**
- **The split.** Netstratum's split is the thirds of 3Z, because each
  nucleon stands once on every axis (docs/95 §0). That is a derivation from
  this kernel's own nucleon, not a choice between kernels. Xylarium's axis
  plane is its own derivation. The seam compares the summed face pairs (V12,
  §15.8), which both sides state exactly.
- **The kind sees its own pass.** A plane's pass is read from the plane's
  velocity, the medium's pressure, the phase state and the parcel's boundary
  (docs/128). Each kind's parcel stands at its own capture phase: the bound
  parcels at `φ_kind`, the chain's accumulated face (docs/134 §12, E75). So
  the pass a kind sees on plane k is read at its own phase, `ψ_k + φ_kind`:
  `s_k(kind)`. D4 becomes `ℓ_k*/L = 1 − s_k(kind)`. Kinds in one body part
  their planes by their phases. This is the same form as Xylarium's
  `ψ_p = frac(w_p·s·Z)`, where the phase also carries the kind.
- *Witness:* two kinds at one place with equal φ rest alike, and with
  different φ they part.

**F3: four axes, R the fourth. The node is Spin(4) = S³ × S³.**
- **The nucleons stand on three axes.** R is the axis that pairs with none
  (docs/95 §0, §2), so the nucleon patterns stand on the three nucleon
  axes, 8 patterns (§15.7).
- **R stands on the R-partner planes.** The electrons' phase on axis k is
  carried on the plane that joins axis k with R: `e_k∧e_R`, the
  orientation's R partners (`orientation::PARTNER`, docs/117 §11.6). D5's
  `R_k*` is that plane's rest. The node is the four axes' unit motions,
  Spin(4) = S³ × S³, the two isoclinic halves `orientation::Motion` holds.
  The five-unit reading (S⁷ → S⁴) would add the conformal `e₋`, the dilator
  (#204), which no nucleon formula reads, so it is not the nucleus's node.
- **The order, and the holonomy.** The three R-partner planes share `e_R`,
  so they do not commute. They compose in the capture order, φ ascending
  (E75), and their composite carries the holonomy on the spatial planes.
  *Fixture (exact, Cl(4,0)):*
  - `(1 + ½e₂R)(1 + ⅓e₁R) ∝ 1 + ⅓e₁R + ½e₂R + ⅙e₁₂`;
  - the other order gives `−⅙e₁₂`.

  The electrons' phases, composed in their capture order, turn the
  nucleus's spatial plane between them. That is the coupling of R to the
  orientation, and it falls out of the composition.
- *Witness:* the composed R equals the per-plane rest motions composed in φ
  order. The reverse order gives the reverse holonomy (forms claim, checked
  in Cl(3,0) and here).

**What the manifest entry must carry (to Mycelium, v4, mycelium #359).**
- Keys `(3Z, 3Q)` are not enough, because §15.7 separates nuclides with
  equal thirds and different patterns. An entry carries the lone-end
  pattern counts `n_S`, 8 whole counts, from which 3Z and 3Q read.
- R is carried as the three R-partner planes' (winding, phase), composed in
  φ order.

V3's code proceeds on these. Nothing in §15 waits on a ruling.


### 15.10 The reach derived: held lone ends survive, at the founding's own share

**D12 (survival reads the held ends).** §10's survival is "bound by its
fibres, or lone and held at the electrons' equilibrium",
`σ = κ + (1 − κ)·g` with `κ = 2F/w`. Both ends of a fibre count as bound.
On the opened planes (15.6), a lone end held on an axis is a held
dimension, the glue (docs/95 §0), so it is bound as a fibre's held end is.
With `h` the lone ends' held share (the mean over the axes of `ℓ_k/L`):

    σ(w; h) = κ′ + (1 − κ′)·g,     κ′ = (2F + h·L)/w

At `h = 0` this is §10's survival exactly (the reduction D3).

**D13 (the founding's held share is its own exchange's).** By F1 (15.9),
the lone ends rest at the exchange's fixed point at their holder's face.
The founding's captures happen in the bounce at the seal window, so their
face is the window's:
- `grip(2, ½) = κ₂·held(2, ½) = 1·½`, so `δ_w = (1 + ½)/2 = ¾`
  (`nuclides::seal_window` at the source clock 2);
- `h = neutron_share(δ_w) = 1 − ¾ = ¼`.

Nothing is set: it is the same share the founding already reads for its
free nucleons.

**D14 (the reach, read; withdrawn in §15.11: the scan ignored the free-nucleon budget).** The climb with `σ(w; ¼)` at the throat's
`g = ½` ends where less than one whole nucleus arrives. The scan below was
calibrated so that one chain stops at 1508 at 2^40, as read; it is a
scratch scan, and the exact read lands with the code.

| Founding | One chain (`h = 0`) | Held ends (`h = ¼`) |
|---|---|---|
| 2^30 grains | — | 2347 |
| 2^40 grains | 1508 | **2448** |
| 2^55 grains | 1614 | 2573 |

So the frontier falls in the operator's 1700–2500 at the working counts,
and every number above is derived. The climb no longer thins along one
chain. Each weight's survival is lifted by the share of its lone ends the
founding's own exchange holds.

**D15 (the representation, next).** The population at a weight is many
nuclei whose lone ends rest at the product shares (D9 at `s_k = ¾`):
`n_∅ : n_k : n_ij : n₁₂₃ = 27 : 9 : 3 : 1` over 64 per lone end, each
member a whole nuclide. Its readings come out in thirds, `Z = F + Σℓ_k/3`.
The electrons follow whole protons (`atom(Q)`). So the table needs one of
two forms:
- the population per weight, held as exact pattern shares, read as mean
  thirds (Xylarium's form, `Q*/Z` rational);
- its members, keyed by whole nuclide, with the share of each.

D15 derives which, from where each reading is used (chemistry by member,
gravity and the manifest by population), before the kinds change.

**Witnesses** (with the code):
1. `σ(w; 0)` equals §10's survival for every w.
2. `h = neutron_share(seal_window(source_clock())) = ¼` exactly.
3. On a fresh founding at 2^40 the frontier is in 1700–2500 (D14 read
   exactly), with nucleons conserved.

### 15.11 Correction: the reach is bound by the founding's nucleons

§15.10's D14 was wrong. Its scan thinned the arriving nuclei by survival
but never spent the free nucleons each climb captures. Built and tested
(2026-11-16), D12 at `h = ¼` stops the founding at **weight 12**: raising
survival at light weights, where `L/w` is large, drains the free pool
there. Nucleon conservation is the bound D14 left out.

**The budget.** At the seal window's neutron share ¼:
- every neutron pairs into a seed: `paired = min(p, n) = N/4`;
- the seeds are `N/8`;
- the free nucleons are `p − n = N/2`, all protons.

So each seed can make at most 4 captures on average. One chain reaching
1508 already spends 96.6% of the free pool (scan with conservation). The
reach is fuel-bound, not survival-bound.

**What the budget shows.**
1. **The free pool is protons, but each climb "captures a neutron".** No
   free neutron remains (`n − paired = 0`). The climb's own channel (V4),
   applied honestly, is a proton capture from that pool. Its survival and
   its product differ from a neutron's. This is the first thing to derive.
2. **Reaching 1700–2500 within the budget** needs survival low where `L/w`
   is large (light weights, where most seeds should stay) and near 1 where
   a nucleus is held (heavy weights). D12 does the reverse, so it is
   withdrawn.
3. **Next derivations, in order:**
   - V4: the founding's channels against its real pool, proton capture
     included;
   - V8: the free nucleons placed by the exchange, not paired away at
     once;
   - V6: the bounce's places, where seeds and fuel stand in different
     ratios.

   The reach is read only on a founding that conserves nucleons. No scan
   without the budget is quoted again.

D12, D13 and D14 are withdrawn as the reach mechanism. D13's share
(`h = ¼`, the seal window's exchange) stands as a reading. The code is
unchanged: the founding still climbs one chain to 1508.


### 15.12 Where the reach is set: the budget and the tail meet

This scan conserves nucleons. Seeds hold 4 nucleons each, and the rest is
fuel. It uses one-chain survival at the throat (`g = ½`), calibrated to
1508:

| Seed share of N | Frontier |
|---|---|
| 1/8 (today) | 1508 |
| 1/16 | 1499 |
| 1/256 | 1461 |
| 10⁻⁵ | 1329 |
| 10⁻⁶ | 1179 |

Fewer seeds lower the frontier, so the founding is not seed-starved. At
today's share the fuel runs out at the weight where the survival tail
(`σ ≈ 0.89`) leaves less than one nucleus: the two bounds meet near 1508.
Raising survival everywhere drains the fuel at light weights (§15.11).

So the reach needs **survival raised at heavy weights only**. In
`σ = κ_w + (1 − κ_w)·g`, the only term that is not the nuclide's own is the
place's held share g. The founding reads it once, at the throat. V6, the
bounce's places along `τ(r) = r + H(<r)`, gives each part of the bounce its
own g:
- the climb at light weights runs where the bounce first seals;
- the heavy tail runs where its part is held more deeply.

V6 is the next derivation. Its reach is read only on a founding that
conserves nucleons (§15.11).

### 15.13 V6 examined: what the held share must do, and the one candidate derived

These are scratch scans (exploratory, not state). They conserve nucleons,
use one-chain κ, keep today's seed share of 1/8, and are calibrated to 1508
at 2^40.

**A constant held share sits on a cliff.**

| g | Frontier | Bound |
|---|---|---|
| 0.30 | 752 | tail |
| 0.45 | 986 | tail |
| **0.50** | **1508** | tail, with the fuel nearly spent |
| 0.55 | 19 | fuel |
| 0.70 | 10 | fuel |

The founding's g = ½ (the throat) is the largest constant share that does
not drain the fuel at light weights.

**The shape the range needs.** Hold g = ½ through light weights and raise
it at heavy ones:
- from weight 200: 0.6 gives 2102, and 0.7 gives 2687;
- from weight 500: 0.6 gives 1837, and 0.7 gives 2596.

So the held share must rise with weight, slowly through the light weights.

**The candidate the record derives: the nucleus's own enclosure.** A
nucleus is a body. Its fibres are its hold (the glue) and its lone ends its
reach, so its own held share is `F/(F + L)`. That is 0.25 at weight 10,
about 0.85–0.92 at 50–200, and about 0.74 at 1500.
- Read alone it gives **1610 at 2^40, 2134 at 2^55** and 1487 at 2^30.
- It is tail-bound, with 10.6% of the fuel left. The fuel is no longer the
  limit; the heavy tail's survival, about 0.94, is.
- Composed with the throat's ½ as "held by either", it drains the fuel by
  weight 10. As "held by both", it stops at about 700.

**What this settles, and what is next.** V6 cannot be a constant place:
the throat is already the best constant. The held share has to read the
nucleus's own depth, and `F/(F + L)` is the first reading of it from the
record. It does not reach 1700 at the working count on its own.

The next derivation is the electrons' held share at a nucleus's own place,
from the master equation at the nucleus's own deficit (docs/105 §3), not
from its fibre count alone. Then V4, the real pool (protons), where a fibre
step needs a neutron the exchange makes.

Each read conserves nucleons, and no candidate is fitted to the range.
