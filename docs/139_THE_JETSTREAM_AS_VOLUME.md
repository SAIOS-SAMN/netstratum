# 139 — The jetstream as volume: the Möbius current, home to the cores

**Operator, 2026-11-16:** "sheet implied 'two dimensions' — we need to add
volume to that, either as a 'mote' in an 'ocean' or on a 'jetstream with
volume' that is the mobius strip that turns the jetstream's polarity
inverse: all roads lead back to the cores."

**Status:** scoped and derived 2026-11-16, including §6. Not built; #158
and #220 build it.

## 0. What the record holds

- **The faces are one strip's two sides** (docs/126 §2, kept as the record
  of the flat reading). A Möbius strip's centre line is the ring,
  `δ = ½` (`R = H`). One pass turns the face over (`δ ↦ 1 − δ`), reverses
  the helix and carries the half twist. Two passes return the face, with
  the winding kept.
- **The face per plane is continuous** (docs/128, superseding docs/126 §2:
  "this is a flat plane"). The pass is read on each plane from that plane's
  own velocity.
- **The jetstream is the orthogonal orbit** (`flows`, operators
  2026-10-16/17): a mote's phase in the circulation's plane (`LOOP`) is its
  place on the orbit.
  - The axial leg, `ψ ∈ [½, 1)`, runs out from the core along the torsion
    axis.
  - The disc leg, `ψ ∈ [0, ½)`, runs home through the disc.
  - "A continuum jetstream carries matter out of the singularity along the
    axis and home through the disc; a vacuum jetstream draws it in along the
    axis and out through the disc."
- **The inversion runs through the ocean** (operator 2026-10-25): "a mobius
  inversion through the ocean as a jetstream current".

## 1. Where it is flat today

| Held today | Its dimension |
|---|---|
| `Mote::vacuum: bool` | the face: two values |
| `flows::mobius` flips the bool and turns each spatial plane `½·t_k` | the twist, applied as a jump |
| the orthogonal orbit's phase `ψ` | along the stream: one dimension |
| `flows::luminal`: a share past ½ takes the Möbius jetstream | crossing to the other face with no path |

So the jetstream is a strip with two dimensions: along it (`ψ`) and across
it (two faces). It has no cross-section and no depth. My "two sheets"
(e₊ and e₋ per axis) was the same collapse in another form.

## 2. Axioms

- **J1, the current has volume.** The jetstream is a solid tube about its
  centre line, the ring `δ = ½`. A mote stands in it at three coordinates:
  - `ψ` along the orbit;
  - `ρ`, its depth from the centre line, its departure from the ring
    (`|δ − ½|` read on its own share);
  - `α`, its angle round the centre line, in turns.

  The flat strip is one diameter of the tube, the line through `α = 0` and
  `α = ½`.
- **J2, the twist turns the cross-section.** Going once round the orbit
  turns the tube's cross-section by half a turn, `α ↦ α + ½` (the Möbius
  twist, kept as turns with the winding, #241).
- **J3, the face is the angle.** A mote's polarity is read from `α`, never
  stored: continuum at `α = 0`, vacuum at `α = ½`, and every face between
  at its own angle by the hemisphere law's opening (`coupling::cosine`, first
  degree). The two faces of the strip are the two ends of one diameter. The
  orthogonal directions, `α = ¼` and `¾`, are the faces neither sheet holds:
  "the many orthogonals of traversal".
- **J4, the centre line runs into the cores.** The axial leg leaves the core
  along the centre line, and the disc leg returns to it. Every path in the
  tube is bounded by the centre line's two ends, both at the core's throat.
- **J5, the ocean is the medium the current runs in.** A mote outside every
  current is "a mote in the ocean": the medium's own reading at its place,
  with no `ψ`, `ρ` or `α`. A mote in a current carries all three. These are
  not two models. The ocean is the medium, and the jetstream is a current
  with volume inside it.

## 3. Derived

**L1 (two ways to the other face).** A mote reaches the inverse polarity
either by going round, one circuit (J2), or by going through: crossing the
centre line, `ρ → 0`, onto the antipodal angle, `α ↦ α + ½` (J1, J3).
- Going round turns the cross-section and adds one winding on the
  orthogonal orbit, `K_⊥`. Time stands; this is a winding.
- Going through crosses the ring, `R = H`, which is a throat. It is a
  crossing and writes the T lane, `T ↦ T − 2(H − R)` (xylarium E032, docs/132
  §4.1).

*Proof.* J2 reaches `α + ½` along the orbit. J1 and J3 reach it across the
cross-section through the centre line, which is the ring. ∎

**L2 (the bool is a reading).** At `ρ ≠ 0`, `vacuum` is the parity of the
half-turns the mote's cross-section has made: its circuits on the orbit plus
its crossings through the centre line, mod 2. Where `α` is not a diameter
end, the face is continuous (J3), and no bool can hold it.

**L3 (the luminal bound becomes a path).** Today `luminal` flips the face
where a mote's share passes ½. In the volume, a share past ½ is a mote past
the centre line, so the bound is the crossing of L1's second way: a throat
crossing with its T flux and its half-turn of `α`. The twist is the path,
not a jump.

**L4 (all roads lead back to the cores).**
- Every path in the current is a word in the two ways (around and through).
- Its ends lie on the centre line, whose ends lie at the core (J4).
- So every traversal returns to a core, and its homecoming is read there:
  - the composite motion closes (docs/138 W4);
  - the windings `K_⊥` resonate;
  - the light ledger balances on both faces (#109).

A traversal never closes in open water, only at a core's throat.

## 4. What it replaces

| Today | Becomes |
|---|---|
| `Mote::vacuum: bool` | a reading of `α` (J3, L2) |
| `flows::mobius` (flip plus `½·t_k`) | the cross-section's half turn per circuit (J2) |
| `flows::luminal` (flip at share ½) | the throat crossing through the centre line (L1, L3), with its T flux |
| the faces as two sheets | the face as a continuous angle round the centre line |
| a mote in the ocean | the medium's reading, outside every current (J5) |

## 5. Witnesses (with the code)

1. Two circuits return `α`, with `K_⊥` raised by 2 (the winding kept).
2. A crossing through the centre line writes `T ↦ T − 2(H − R)` and turns
   `α` by ½. Two crossings return the face, and their T fluxes cancel only
   if `H − R` is the same at both (a single throat). Through two different
   throats they do not cancel.
3. At `α ∈ {0, ½}` the face reads today's `vacuum` exactly, and `flows`'
   reads stay unchanged where every mote stands on a diameter end.
4. Every path's ends lie at a core: the current's centre line meets the
   core's throat at both ends.
5. `Σ m·ℓ` is unchanged by the cross-section's turn, which moves no spin.

## 6. The depth, the angle and where the volume lives: derived

1. **The depth `ρ` is the mote's own departure from the ring,
   `|δ − ½|`.** The jetstream is its core's orthogonal orbit (§0), not a
   body with its own hold. So the tube's centre line is the core's ring and
   its radius is read on the enclosure's own share. A current with its own
   H would be a second body, and nothing in the record founds one.
2. **The angle is per plane, `α_k`.** The face per plane is already
   continuous (docs/128), so the cross-section's angle is read per spatial
   plane, and docs/128's face per plane is its reading.
3. **The volume lives on the orientation.** The angle round the centre
   line on plane k is the turning on the plane that joins axis k to the
   orbit's axial leg, the R partner `e_k∧e_R` (`orientation::PARTNER`). The
   motion already holds that plane. The circuit's half turn of the
   cross-section (J2) is a half turn on the R-partner planes per circuit of
   the orbit, with the winding kept. It is the same planes that carry the
   electrons' phase (docs/97 §15.9 F3), so the face and R stand on one
   structure.

   No new state is added. `Mote::vacuum` becomes the reading of the
   R-partner planes' angles (L2), and #158 (the circulation per pair) is
   its build.

## 7. Issues

- Tracked: #220.
- Related:
  - #160, the lift as a boost;
  - #204, the scale dilator;
  - #158, the Möbius leg's half turn;
  - #109, the homecoming test;
  - #213, the table's held and open faces per axis (F3).
