# 125 — The equations at their base: the nucleon inversion, row by row

This document sorts every era-1 equation (docs/EQUATIONS.md E1–E58) by the
part of matter it acts on and by whether it stands at its base. It then lists
what each equation not at its base still needs derived, in the order to build
it.

An equation is at its base when it reads Netstratum's inverse nuclide as the
record defines it (docs/97, docs/95 §0), every face it names is read through
the inversion `I` (docs/104 §0), and no halving, static or chosen constant
stands in for a reading.

**Operator 2026-11-11:** "some of our equations are going to be inverted
because of the nucleon weighting — this should identify the particles and
components that are proton specific and neutron specific in our universe";
"we should be deriving which equations are not at their base and derive them
further in order to produce the correct formulas". The weights are
Netstratum's own; only the form that crosses the seam is shared with Mycelium
(mycelium #239, the joint LAW-COUPLING v2).

**Status:** the map below is a read of the tree at 70124ed8 (after #178).
Each equation's derivation lands in its own change, with its witness and a
founding read before and after.

**Prerequisites:** docs/97 (Q the proton, Z the neutron, R the electron's
phase), docs/104 §0–§1 (the map `I`), docs/95 §0 (the nucleon dipole, held
and open dimensions), register K15.

## 1. The sides of matter

| side | what it is | its readings |
|---|---|---|
| **proton** (open pole, `3Q`) | the charge, the lone ends | the push, charge, the barrier |
| **neutron** (held pole, `3Z`) | the glue, the fibred ends | grip `κ = 2Z/w`, the hold, captures |
| **electron** (R's domain) | unpaired `U`, bound or free, the phase `R` | magnetism, chemistry, bonds, light from bonds |
| **nucleons** (`w = Q + Z`) | mass | gravity, holding, pressure |
| **none** | pole-free | turning, phase, the luminal bound, periods, geometry |

The rule from docs/104 §0 decides which equations invert. A reading that
names a face is read through `I`: the held share is `1 − δ` and the push
reads `δ`. A reading that names no face (turning, the luminal bound, phases,
in-step) is the same on both sides.

## 2. At their base (17)

E1, E2, E3, E5, E10 (bonds), E12, E13, E14, E21, E24, E26, E33, E35, E37,
E52, E53, E57. Not built, specified as derived: E6, E7, E8 (the test), E11.
Retired: E31, E32.

## 3. Not at their base (34), by cause

### 3.1 Collapsed quantities: the nucleus read as protons and electrons

- **E16, the push** (`pulse::push_of`). It is `½·magnetism/mass`, and its
  doc reads magnetism as "Q" and `w − U` as "A".
- **E17, the universe's hold** (`ocean::universe_hold`). The row reads
  "holding + magnetism" and "3Z"; the code reads nucleons.
- **E25, the burn** (`formation::burn`). The row reads `|Δm|/3Z_p`, the
  code `Δm/2w`, and the barrier's comment uses Z for the charge.
- **E30, recycling** (`recycle::recycle`). The load is shared by
  `chem::hold = w − U`, which mixes nucleons with the electrons' open
  dimensions, under a "3Z ledger".
- **One share read two ways.** `flows::open` and `push_of` read `U/w`;
  `chem::Dimensions::open_share` reads `U/3w`.

### 3.2 A face not read through `I`

- **E27, light on bodies** (`breath::shine`). It warms by `1 − δ`; docs/104
  requires `u·e`.
- **E38, the attractor's keep** (`flows::attractor_keep`, `settle_disc`).
  Its drag is `1 − δ` for continuum motes; docs/104 requires `d = u`.
- **E9, the restoring share** (`flows::fluid`). Its column `Σ m·δ` is called
  gravity's held share (forward), beside `Φ = 1 − δ` (inverse) in the same
  law.
- **E58, the nucleon dipole in the shear** (`flows::shear`). It reads every
  mote's pole and open share uninverted, while the magnet inverts the
  vacuum's (`σ ↦ −σ`, `o ↦ 1 − o`).
- **E18, the release** (`breath`). Its comment names the forward `½(1 − δ)`.

### 3.3 A halving or a static standing in for a reading

- **E16:** the push's `½`.
- **E18:** the release `s = ½·c/(c + 1)` and its base phase `(2/3)·d_k`.
- **E20:** the orthogonal return `½·δ·ℓ_⊥`.
- **E51:** the contained front's push `½`, passed as the whole's unit.
- **E55:** the star keeps `½` a ring.
- **E23 and E22:** a fixed rim at `2`, while E52 reads the rim live.
- **E47–E49:** the isotropic third and `/3`, and through E47 E36 and E39.
- **E58:** an empty throat's base pole is a static closed pole.

### 3.4 An underived constant

- **E18:** base `1`.
- **E50:** `1442/1000` and `693/1000` (`life.rs`).
- **E25:** the star core's hold `1` in `tunnelling`.
- **E17:** a fallback `2`.

### 3.5 Charge changing without R carrying it (docs/97 §1)

docs/97 §1: "charge moves between Q and Z only with the electron phase (R)
carrying the change".

- **E28, captures:** a climb makes a hydrogen (a proton) into part of `w +
  1`, which may stand as a neutron.
- **E29, a dying core's release:** the released nucleons all stand as
  hydrogen.
- **E30:** the peels make hydrogen.

In none of these does R carry the change.

### 3.6 The row and the code disagree

- **E15:** the row names `flows::face`; the law is `polarity::faces`.
- **E19:** the row names `flows::band` and `toward_rest`; the band is zeroed
  (register K12).
- **E20:** the row still cites the band.
- **E22:** the code exits at `R ≥ 2·base` from the image `2H − R`, a reach
  map retired by docs/117 §10.
- **E29:** the row says "at the pulse phase"; the code passes `g = 1 − δ`.
- **E41:** the code pairs `unpaired`, not the lone ends `L_e` (E44).
- **E47–E50:** the rows read built; the placeholders stand.
- **E58:** the row names `chem::frames`, now `chem::dimensions`.
- **docs/104's "in code now" column:** stale for `nuclides::held`.

## 4. Where the record disagrees with itself

Three points cannot be derived until one reading is chosen. Each is stated
with its two sides.

1. **The electrons' held share** (E28, E29, E43).
   - docs/104 §1.1 counts it, `h = (F_e + L_e·in_step(p, ½))/Q`, with the
     phase.
   - docs/105 §3 reads it as `g` and takes the phase out of survival, grip
     and the bound share.
   - The code follows docs/105 §3.
2. **The push's open dimensions and its whole** (E16).
   - docs/97 §3 reads the open dimensions from the electrons, `U`.
   - docs/104 §1.2 says the inverse nucleus makes the protons the open ends,
     `3Q`.
   - The whole is `w` (`push_of`, `flows::open`) or `3w` (`Dimensions`).
3. **Emission's open share** (E54).
   - docs/104 keeps `(1 − e)` and a counterweight (§2).
   - docs/122 law 3, which E54 follows, drops `e`.

### 4.1 Decided (operator 2026-11-11)

The operator on item 2: "regardless of the orientation, this is your
orientation which needs to be defined and needs the antipole/inversion
applied as well". On item 1: "your call".

**Item 1, the electrons' held share: docs/105 §3 governs** (revised the
same day, before any build). docs/105 is the later derivation from the
polarity master equation, built and read with Xylarium. It removed the
halving docs/104 §1.1 objected to: electrons bind at the held face `g` and
free at the unlocked face `u`, so they stand bound at `g/(g + u) = g`, with
no phase and no halving.

The count with the phase was first chosen here, and is refused for two
reasons:
- At the founding's phase (`frac(p) = ½`, docs/97 §13) it reads
  `in_step(½, ½) = 1` on the inverted face. Deuterium's held share becomes 1,
  the seal window moves from `¾` to `1` at `ν = 2`, and the neutron share
  goes to 0.
- It puts R in a share, which Xylarium #183 rules out: "the Z–Q stretch is
  the axis plane's **share**, not its phase. The phase φ₀ joins the common
  turn."

So **R is the axis plane's phase in the turning** (the orientation, docs/117
§11.7), never a factor in a share.

    h = g        (the held share at the body's own place)

**Item 2, the push: its orientation, its antipole, and the inversion.**
- **The orientation is Netstratum's own, the open pole.** The protons' open
  dimensions over the whole, `3Q/3w = Q/w`, the open stratum's share `d₀` of
  the join `S¹¹ = S¹_axis * S⁹_iso` (Xylarium #183). This is docs/104 §1.2's
  "open in both frames: the inverse nucleus already makes the protons the
  open ends".
- **The bound electrons close it.** The electrons bound at their place,
  `h·Q = g·Q` of them, carry their protons' charge onto the held side. So the
  push is the protons' charge left open, at the unlocked face:

      p = Q·(1 − g) / w = Q·u / w

  A body where nothing holds (`g = 0`) pushes by all its protons; one held
  whole (`g = 1`) pushes not at all.
- **The antipole is derived by the same inversion, never left out.** It is
  the held stratum: the neutrons, with the charge the electrons carried
  there:

      p̄ = (Z + Q·g) / w = 1 − p

  `p` and `p̄` are a pair share (docs/104 §1.2), with no `½`.
- **The inversion `I` exchanges them across the faces.** The continuum leg
  reads `p` and the vacuum leg reads `p̄` (pole ↦ the other pole), which is
  the join's `t₀ ↦ 1/t₀`, `c₀ ↔ d₀`.
- **What stays the electrons'.** Their open dimensions `U` (unpaired) remain
  the coupling's strength and the bonds' ends (E58, E40–E44). They no longer
  set the push.
- **Replaces** E16's `½·U/w`. A body's push now depends on its phase, so
  `pulse::push_of` takes the body's pulse phase.

**Item 3, emission's open share** (superseded 2026-11-11 by docs/127 §4:
the counterweight is the manifold's recycled share): docs/122 law 3 governs (operator
2026-11-05: heat and light from bonds and friction). docs/104's `(1 − e)`
row and its "in code now" column are corrected with it.

## 5. The order of derivation

Each step is one change: its formula here or in its own document first, its
witness, and a founding read before and after (2^40, 580 bells).

1. **The three disagreements of §4.** Each is derived from the nucleon
   dipole (docs/95 §0) and `I`, or put to the operator where the record
   alone cannot decide. Everything below them reads them.
2. **E58 in the shear:** the vacuum read inverted, as in the magnet.
3. **One open share** (§3.1): `U/w` or `U/3w`, decided by §4 item 2, read
   everywhere.
4. **The faces not inverted** (§3.2): E27 warmth `u·e`, E38 drag `d = u`,
   E9's column.
5. **The halvings and statics** (§3.3): each replaced by the reading it
   stands for, starting with the push (E16) and the release (E18), which
   every core reads.
6. **Charge carried by R** (§3.5): the climbs, the release and the peels move
   charge between Q and Z only with R (docs/97 §1).
7. **3Q derived as 3Z's antipole by dimensional inversion** (mycelium #239):
   the joint LAW-COUPLING v2. Netstratum's input is the restated form,
   translated Z ↔ Q at the seam.
8. **The rows' text** (§3.6) corrected with each change, and the constants of
   §3.4 derived or listed in docs/108.

## 6. Open

- E4 and E34: whether `ring::surface`, `ring::strength` and the passage at
  `½` are on the live path, not yet traced.
- Whether the star's `δ = ½` (E55, docs/122 §4) is a derived fixed point.
