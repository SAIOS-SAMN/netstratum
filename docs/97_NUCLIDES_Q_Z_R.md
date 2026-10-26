# 97 — Nuclides: Q the proton, Z the neutron, R the electron's phase

Operator 2026-10-26: "you're still stuck on singular Z numerics : we need
weight up to 1200 to have well formed galaxies with enough resources to build
planets - i want this built as the inverse of xylarium where Q is the proton
and Z is the neutron and R is the phase state of the electron through p as
the pulse reverberation of the echo from the core".

Status: §1–§2 and §4 BUILT (41b2a6b); §5 BUILT in nature's form (the neutron share for ruling); §3 (R) not yet built.

Read against: xylarium spec 11.0 (53) (Z the proton, Q the neutron, R the
electron's phase; ruled there) and xyl-cosmos `fibres.rs` (ends on S^11,
fibres, seals). Netstratum is the inverse: Q the proton, Z the neutron.

## The seam with xylarium

The letters are swapped deliberately (operator, relayed by xylarium
2026-10-26: "i want netstratum to run the opposite of yours - Z/Q letter swap
for the nucleons"; xylarium spec 11.0 (53), 5e4db24): netstratum Q = proton,
Z = neutron; xylarium Z = proton, Q = neutron; both R = the electron's phase.
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

## 2. The nucleus converges on S^11 (xylarium's fibres, inverted)

Nucleon ends arrive one at a time on S^11, whose degree-l harmonics give
`d(l) = C(l + 11, 11) − C(l + 9, 11)` independent directions (1, 12, 77,
352, …; the same count as our atom's orientations).

- An end first claims a free direction ALONE. Lone ends are open: in
  netstratum they are PROTONS (Q).
- When every direction of the open degree is claimed, the next end BINDS into
  a lone end: a FIBRE, one proton and one neutron counter-turning.
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

## 5. OPEN — the abundance law (for ruling)

The founding must spread the bounce's nucleons over weights to ≥ 1200 while
conserving them and leaving the hydrogen stars burn. Read on 2^55 grains:

| law | reaches | conserves nucleons | hydrogen |
|---|---|---|---|
| today's: `count_w = count_{w−1}·½·held` | Z 210 (w ≈ 600) | yes | 27% of the weight |
| `count_w = count_{w−1}·½(1 + κ_w)` (a capture survives bound, or lone on the held hemisphere) | w 1644 | no (counts sum past the nucleons) | — |
| the same as a capture chain, normalized to the nucleons | w ≫ 1200 | yes | none left (w = 2 seals, so every proton is captured to deuterium) |

The survival share `½(1 + κ)` derives from the hemisphere law and reaches
weight 1200. What is missing is how a capture CONSUMES a free nucleon:
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

## 7. Built: the founding in nature's form (operator 2026-10-26: "align on nature and build it")

As nature's: the bounce leaves free protons and neutrons; captures run only on
the neutrons and stop when they are spent; the heavy weight is the stars' to
build over generations.

- **The neutron share:** a quarter of the nucleons, the unlocked face of the
  bounce's ring, `½(1 − δ)` at δ = ½. FOR RULING: nature's freeze-out leaves
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

## 8. Bounce nucleosynthesis: three components (operator 2026-10-26: "is this isomorphic nucleosynthesis or unitary?"; "should it be automorphic?"; "land all three so we have the individual components correct")

Each component is its own, with its own witness (`nuclides.rs`,
`the_three_components_stand_apart_and_each_holds`):

- **AUTOMORPHIC, the structure.** A nucleon's pole IS its leg: on the
  continuum leg (open, R forward) a proton, on the vacuum leg (held, R
  backward) a neutron. Conversion is the Möbius pass (`(δ, ψ, s) ↦ (1 − δ,
  ψ + ½, −s)`), an involution: `M∘M = id`. The swap Q ↔ Z maps the nucleus's
  convergence onto itself (every fibre one of each; every seal its own
  image). Nothing is imported from nature.
- **UNITARY, the books.** One exchange a pass: protons cross to the vacuum leg
  by the held face, `p·δ`; neutrons to the continuum by the unlocked face,
  `n·½(1 − δ)`; whole nucleons, both sides booked, `p + n` unchanged.
- **ISOMORPHIC, a reading only.** The founding's outcome set beside nature's
  (neutrons per proton, the weight-4 mass share, hydrogen's) and reported at
  the founding, never imposed.

OPEN, to land before the founding runs on them: THE SEAL WINDOW, the pass at
which deuterium's seal first holds against the bounce's heat and the neutrons
standing on the vacuum leg are captured. Our founding is cold (the bounce
carries no heat of its own today), so the window has no heat to close against;
the bounce's heat must be stated first. Until then the founding keeps its
static quarter, flagged.
