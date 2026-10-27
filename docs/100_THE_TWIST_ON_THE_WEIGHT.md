# 100 — The twist on the weight: the quantum state the elements flow by

Operator 2026-10-27: "make sure you have the proper quantum state formulations
in order to provide the filament (ocean) that the elements are flowing through
with the proper twists on the weight". Status: SCOPE. Builds on docs/94 (the
Möbius jetstream), docs/96 (founding as flow), docs/97 (Q, Z, R; §14 the
densest element), docs/99 (phases, never frames); xylarium kernel 11.0 (8)
twist and torsion on the jets, (13) the harmonic founding, and its built
`circuit::axes::antinode`.

## 0. What stands now

- **The carriage is one number, the weight.** The founding cascade carries
  each kind `ℓ_z = (K + s_i)/w_z` (the bounces' one force over its inertia):
  heavy kinds stay deep, light ride out, by mass alone. Xylarium's reading
  (13): "the carriage sees each kind as ONE NUMBER, its weight".
- **The twist is drawn.** `quantum::torsion_phase` reads a galaxy's torsion
  phase from the seed, "a spectrum field until the vacuum's integers define
  it" (docs/79): a draw, against design law "no draws".
- **The jets carry no winding.** A released mote leaves the throat with no
  lead between its planes (its tilt isotropic or its members' blend).
- **The ocean carries no twist.** Its motes stand at rest phases (`rest(½)`),
  untilted.

## 1. The quantum state of a kind (its place, its harmonic weight)

Xylarium 11.0 (13): "a kind's HARMONIC WEIGHT is its place, never its mass
alone": its phases on its own sphere, where its bands stand, how its frames
are turned, whether its outer frame is paired.

Ours, read off the nucleus's convergence on S^11 (docs/97 §2), exact, per
weight w:

- `n(w)` its turning (it holds the degrees `0..n − 1` closed);
- `l(w)` its open degree, the harmonic its outer ends stand on;
- `f(w) = (lone + bound)/d(l)` its outer degree's claimed share, `b(w) =
  bound/d(l)` the paired share (`b = 1` at every seal).

Its R (the electrons' phase, docs/97 §13) is the same convergence over its Q
ends.

## 2. The bounce's twist, derived (retiring the drawn torsion phase)

- **Twist per turn is the deficit** (xylarium 11.4: "twist per turn is the
  deficit"): a core winds each plane, each ring, by its deficit at its own
  ring, `δ = ½`, shared over its planes by its tilt, `φ_k = ½·t_k` (isoclinic:
  one share of a turn in every plane at once).
- **It relaxes with its history** (xylarium (13): "the torsion has a
  history: it relaxes as it carries matter out"): at the founding each
  capture event sees the cascade's own face `s = ½·R/(R + H)` (held so far),
  already the cascade's per-event share; the twist a shell sees is that
  face, not a constant.
- **A galaxy's torsion phase** is its core's winding accumulated, `Σ φ` over
  its bells, read, never drawn (`quantum::torsion_phase` retires).

## 3. Carriage by harmonic (the twist on the weight)

Xylarium (13): "each kind couples to [the twist] by its in-step share with
that twist in every plane of its place - the product over its planes - so
the kinds in step ride the jets out far, the out of step stay deep. The
superheavy kinds (complex high-l shells) are the most out of step."

**The form:** keep the force and the inertia (operator 2026-10-18: "the push
must be agnostic to the element by anything other than the force ...
inertia is fine") and add the coupling:

    ℓ_z = (K + s_i) · C_z / w_z

`C_z ∈ [0, 1]` the kind's harmonic coupling with the twist. Heavy kinds stay
deep by inertia AND by coupling; light, simple kinds ride out; chemical
variety from the place, never a draw.

**The coupling (OPEN, for ruling):** xylarium's built antinode law gives a
harmonic of order q the share `1/q` for odd q and 0 for even q (a node: its
two faces cancel). Which order a kind presents is the open ruling (xylarium:
"how many frames a shell holds ... is open and sets every kind's phases"):

- (a) `q = l(w) + 1`, its outer degree's order: s-shells (`l = 0`) couple
  whole, every even-q shell is a node and is HELD at the core whatever its
  weight; variety strong, not monotone in weight.
- (b) the in-step product over its planes of its outer phase `l·φ_k` with the
  twist `φ_k`, `C = Π_k in_step(l·φ_k, φ_k)`, weighted by the paired share
  (a sealed nuclide is whole, `b = 1`): monotone where the twist is small,
  periodic in l.

Xylarium's answer (2026-10-27, a reading of its spec, not a ruling): its
atom-band question is closed as the PIVOT DEGREE on S^11 with its Hopf
fibres (its 11.0 (14), our convergence); it has no ruled C (its circuit never
computes one); of (a) and (b), (a) is the one consistent with the pivot-degree
ruling, reading the order straight from the open degree, and non-monotone in
weight is acceptable ("shells, not weights, set the order"). The choice is
the operator's.

## 4. The jets' locked lead (the helix)

Xylarium 11.0 (8): "a mote EXPELLED from a throat carries the core's winding
as a LOCKED LEAD between its planes - the jet a HELIX, its pitch the core's
... along the stream the planes advance together at the core's torsion ratio
and loosen as the stream's own turning takes over - torsion relaxing
outward, read, never set."

Ours, by machinery already built:

- **At release** a mote's tilt is the core's twist shares, `t_k = φ_k/Σφ`
  (its lead between planes), not isotropic.
- **Along the stream** its tilt returns toward its galaxy's disc plane by the
  attractor law already built (`flows::settle_disc`: keeps `turned/(d +
  turned)` of its displacement): the lead loosens as its own turning takes
  over. No new state.
- **On the vacuum leg** the route runs backward (docs/98), so the helix's
  handedness turns with the Möbius pass: one law.

## 5. Twist is tension in the fibre

Xylarium (8): "TWIST IS TENSION IN THE FIBRE: it adds to the tautness of every
filament it threads ... a wound jet binds more readily where it slows and
snaps where over-wound."

**Proposed:** a binding's bond counts the lead its members carry in step:
`λ = 2(m + μ + τ)/R`, `τ` the members' in-step lead (holding-weighted
`Σ m·in_step(lead, lead̄)`), so a wound filament binds more readily; over-wound
it parts. The over-winding bound is the nuclei's own grip (§9.2): where
`a + τ > κ_w·h` the fibres snap.

## 6. The ocean is the filament

The ocean's motes are the jetstream's return leg between galaxies (docs/94
§11). They carry what they were expelled with:

- **its lead** (§4), relaxing along the stream;
- **its phases** from its expelling core's winding at the bell it left, never
  `rest(½)`;
- **its landing** by the pull as now (`ocean::land`), the twist entering the
  binding it joins (§5).

## 7. The inverse

The twist acts on phases, and phases carry no face: the Möbius pass turns a
mote's route and so the helix's sense (§4). The harmonic coupling reads the
nucleus as built (Q the open ends, Z the fibres, R the electrons' phase); in
the inverse universe the open ends are the protons, so a kind whose outer
degree is unpaired presents its PROTONS to the twist (open, coupling) and a
sealed kind presents only fibres (whole).

## 8. Quantum torsion (operator 2026-10-27: "add quantum torsion and sub-atomic phase transitions")

The bounce is Einstein–Cartan: torsion is real and couples to spin
(xylarium (8)). In our grammar torsion is the WINDING a quantum state takes
from the twist, per plane, and keeps until its own turning relaxes it.

- **Taken:** each ring a body receives, it is wound in plane k by the twist
  it couples to, `φ_k·C` (§2, §3): a sealed or in-step state is wound fully,
  a node not at all.
- **Kept as its lead:** the winding is the body's displacement between its
  planes (its tilt against its enclosure's plane, §4), state already
  carried (`Mote.tilt`, `Body.tilt`); no new scalar.
- **Relaxed by its own turning:** each ring it keeps `turned/(d + turned)`
  of it (`turned = 2δ`, `d = 1 − δ` at its reach; the attractor law of
  `settle_disc`): torsion relaxes outward, read, never set.
- **The torsion a body carries,** `τ = Σ_k |t_k − t̄_k|/2` (L1, half the
  displacement of its plane shares, in `[0, 1]`): 0 settled, 1 wound wholly
  into a plane its enclosure does not turn in.
- **Spin couples to it:** the winding's sense per plane is the twist's
  (forward on the continuum leg, backward on the vacuum leg, docs/98), so a
  Möbius pass reverses the helix and the torsion's sign with it.

## 9. Sub-atomic phase transitions

A phase transition is a state crossing a hemisphere boundary (the one
measure, `1 − 2|ψ|`, crossing ½): never a threshold set. Three, one per
sub-atomic state, each read at the body's own pulse phase p (docs/97 §13).

### 9.1 The electrons: bound ↔ free (R)

`R = κ_e·(1 − 2·frac(p))` crosses 0 at the half turn of p. On the held side
(`R > 0`) a nuclide's open frames are bound (its chemistry closes, its
clusters hold); past it (`R < 0`) they are freed (ionized, its clusters
part). The transition's LINE is the frames it moves: `κ_e·Q` of them, each a
quantum of light given (bound) or taken (freed), booked on the light ledger.
Built in part (R reads survival and grip); owed: chemistry and the light
ledger reading the crossing.

### 9.2 The fibres: bound ↔ parted (the nucleus)

A nucleus grips by `κ_w·h` (docs/97 §13). Against it stand its heat `a` and
its torsion `τ` (§8): TWIST IS TENSION IN THE FIBRE. It PARTS where

    a + τ > κ_w·h

each fibre breaking into its nucleons (they stand as hydrogen, the rapid
capture's fuel, docs/97 §11). This is §5's over-winding bound, derived: a
wound filament binds more readily (its lead in step adds to its bond) until
its winding passes its own nuclei's grip, where it snaps. Below the bound
the capture runs (`σ = κ + (1 − κ)·h`); past it the release. One crossing,
both directions.

### 9.3 The poles: proton ↔ neutron (the Möbius pass)

A nucleon's pole is its leg (docs/97 §8, `nuclides::mobius`). A lone end
wound through half a turn (its torsion's phase crossing ½) passes the ring
and turns its pole: a proton becomes a neutron and binds as a fibre's
partner, `(Q, Z) → (Q − 1, Z + 1)` at the same weight. The convergence's
`(Q(w), Z(w))` is the exchange's FIXED POINT (each weight one nuclide), so a
state off it is a transient: the unitary exchange (p→n by the held face, n→p
by the unlocked, docs/97 §8) returns it to the line within its ring, and the
excursion's yield is its electron's phase change (§9.1): a quantum of R
given or taken. Nucleons conserved; weight unchanged; the table unchanged.

### 9.4 Where they act

| state | crossing | in | yields |
|---|---|---|---|
| electrons | `R = 0` at `frac(p) = ½` | every body, its own pulse phase | light (the line), chemistry opening or closing |
| fibres | `a + τ = κ_w·h` | stars' hearts, the jets, bindings | fission to hydrogen / capture |
| poles | torsion phase through ½ | wound lone ends | an R quantum; the nuclide returns to its line |

## 10. Build order (each with its witness)

1. **The kind's place** (§1, BUILT): `nuclides::place(w)`. Witness:
   the seals read `b = 1`; `l` and `n` match the convergence.
2. **The twist derived** (§2, BUILT: `quantum::twist`, the hidden phase
   `frac(b·φ + b/P)`; the release reads the core's winding per plane; the
   breath now draws nothing): the core's winding `φ_k = ½·t_k`; the galaxy's
   torsion phase accumulated; `quantum::torsion_phase`'s draw retired.
   Witness: the hidden phase reads the same at N and 4N on the closed form.
3. **Carriage by harmonic** (§3), after xylarium's ruling on C. Witness: the
   heaviest kinds held at the core; the cascade still conserves nucleons and
   spin.
4. **The locked lead** (§4) and **the ocean's twist** (§6) (BUILT: a
   released mote's tilt is its core's twist; motes leaving for the ocean keep
   their lead and phases; the founding's slight beads still stand at rest,
   owed). Witness: a
   released mote's tilt is the core's; it relaxes by the attractor law; the
   Möbius pass reverses its sense.
5. **Quantum torsion** (§8, BUILT: `flows::torsion`, `torsion_signed`): `τ` read from the lead; spin's sense by leg.
   Witness: a settled mote reads 0; a Möbius pass reverses the winding's
   sign; relaxation by the attractor law, exact.
6. **Twist in the bond and the fibres' transition** (§5, §9.2): bond with
   the in-step lead; nuclei part where `a + τ > κ_w·h`. Witness: nucleons
   conserved; an unwound sealed nucleus never parts at `a < h`.
7. **The electrons' transition on the ledgers** (§9.1): chemistry and light
   read the R crossing. Witness: the light ledger balances across a
   crossing.
8. **The poles' transition** (§9.3): the excursion read as an R quantum; the
   table unchanged. Witness: nucleons and weight conserved; every nuclide
   back on its line within the ring.
9. **Read:** re-found and breathe past bell 7000.
