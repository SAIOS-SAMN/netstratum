# 98 — The inverse faces: gravity, magnetism and velocity on the other leg

Operator 2026-10-27: "you need to invert magnetism, gravity, velocity -
because your 'continuum' will actually be the vacuum"; "proced".

> **Corrected (2026-10-28, docs/103 §5):** §3's "a relabel" holds for MOTION,
> not for PRODUCTION: the luminal bound keeps most matter on the vacuum face,
> inside the ring, while the strip, crest and disc read the continuum face.

Status (2026-10-28): §3–§5 DERIVED; §4 the symmetric luminal bound BUILT
(fa7378d); §6 read (§7) and carried into docs/99. Open: the leg flag's second
writer (`flows::stream`, docs/101 §1.6).

Builds on docs/94 §2 (the Möbius jetstream), docs/96 §7–§8, docs/97 (Q the
proton, Z the neutron, R the electron's phase; the nucleus as built).

## 1. Xylarium's faces (forward)

From the kernel doc (§ the throat, S^0 faces; the jetstream (11)):

| face | role | reads |
|---|---|---|
| VACUUM (black) | holds the rings it takes: gravity, containment, quantum mass; matter in transit (axis, arc) | the deficit `δ = H/(R + H)` |
| CONTINUUM (white) | outflow, pulse and echoes, push, jets, light; the open frames repel (magnetism); the disc is held on it | the unlocked share `1 − δ` |

## 2. Ours, inverted

Our continuum leg is the vacuum. Each law swaps the face it reads: gravity
and velocity read `δ(H²/R) = 1 − δ(R)` where the forward read `δ(R)`, the
push, light and cooling read `δ(R)` where the forward read `1 − δ(R)`.

## 3. The inversion is already in the grammar, and the luminal bound makes it a relabel

- **Inversion through the throat** `R → H²/R` keeps every angle and maps
  `δ ↦ 1 − δ` (witness: `recycle` tests, fixed at R = H, both ½). A mote
  carries its face (`Mote.vacuum`); on the inverted face it reads its pull
  and turning at `1 − δ`, runs its circulation backward, and reflects its
  band through the throat (`flows::turn`, `spin_torque`).
- **The Möbius pass** `(δ, ψ, s) ↦ (1 − δ, ψ + ½, −s)` turns a mote's face.
- **The luminal bound** (SUBSTRATE_C, one whole turn a K-step): a body turns
  `2·share` a bell, its share δ on one face and `1 − δ` on the other.
  Exactly one face reads at most ½ at any reach; the other would fracture.
  So matter stands on the face whose share is at most ½, whichever we call
  the continuum.

Therefore inverting gravity, magnetism and velocity globally only relabels
the legs: what was read forward on one face is read inverted on the other,
and the luminal bound returns every mote to the face it can stand on. The
physics is the same up to the antipode (`ψ + ½`).

## 4. Where it was not exact: the luminal bound read one face

`flows::luminal` turned only continuum motes inside the ring (`δ > ½`). A
vacuum mote standing outside the ring reads `1 − δ > ½`: it turned more
than one whole turn a bell, and nothing returned it. Measured 2026-10-27
(2^55 grains, bell 1500): 30292 motes took the pass onto the vacuum leg; as
they drift past the ring they break the bound.

**The bound, symmetric:** a mote whose turning share on its own face
exceeds ½ takes the Möbius jetstream, either way:

    share = δ(R)       on the continuum face
    share = 1 − δ(R)   on the vacuum face
    share > ½  ⇒  mobius(x)

Witness: after the pass every mote's share is at most ½; twice is the
identity; the ring (δ = ½) is fixed on both faces.

## 5. Bodies

Bodies (stars, worlds) carry no face: they read `δ` at their reach. They
form from disc motes outside the ring, where `δ < ½`, the face the bound
admits, so their laws already read the admitted face. The nucleus is
unchanged (Q the proton, Z the neutron, pole = leg).

## 6. Open

What refuses the galaxies' bindings (49683 members returned with no heart by
bell 1500) is measured next: `ignition_share` (heat ≥ ½) or
`formation::system` (no heart cut).

## 7. Read (2026-10-27, 2^55 grains)

- The symmetric bound moved 2 motes more than the one-sided (30294 against
  30292) by bell 1500; every other count unchanged. The legs were already
  nearly where the bound puts them.
- No binding was refused at ignition (0 by `ignition_share` or
  `formation::system`). The 49683 "returned" members are bindings in which
  NOTHING FELL (`formation::infall`): by bell 900, 59 bindings and 16876
  members. The heart's extent reads about ½ in the binding's own units; the
  members' distances, the galaxy offsets times `g/m` (the galaxy's holding
  over the binding's), read 10^1–10^5: the nearest member stands a median
  118 extents out. The binding holds over its arc (the crest, `w_max` of
  order the ring) while its infall reads its members as if its extent were
  its share `m`: two frames for one body. Owed: the infall read in the
  binding's own arc.
