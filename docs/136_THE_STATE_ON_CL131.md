# 136 — The state on Cl(13,1): the place on the base, the S¹¹ fibre over it

**Operator ruling 2026-11-13** (relayed by Mycelium, MU-HOSTING §5a.2a,
mycelium #305; "one operator is amongst you all"): one body-state signature
across every kernel, **Cl(13,1)** = Cl(n+1,1) at n = 12, the Hopf carrier
S¹ → S¹¹ → CP⁵. It supersedes docs/131's Cl(5,4).

**Status:** derived 2026-11-13; §6.2 built (`orientation.rs`: the base rotor, the scale; the Cl(5,4) axes and vacuum factor retired). The plane assignment is
Xylarium's carrier (xyl_core::hopf, rotor, cga; xylarium #183), read
jointly; Netstratum's fixtures go to Xylarium before the build.

## 0. What Cl(5,4) held, and where each part goes

| Cl(5,4) (docs/131) | Cl(13,1) |
|---|---|
| 3 spatial axes `e₊ₖ` with the orbit spins `ℓ_k = ℓ_o·t_k` | **the base**: places on the enclosure's S^360 (S^60 for a system); the orbit is motion on the base, not planes of the fibre |
| 3 spans `e₋ₖ` (negative): the vacuum as axes | **retired as axes.** The vacuum is a face: each plane's **share split** between its two faces, read by δ (§2) |
| R's axis `e_R`, the circulation on `e_kR` (`spins_orth`) | **R is a phase on every plane** of the fibre, `R_p = κ(1 − 2·frac(ψ_p))` (§3). The circulation's orbits stay as the base rotor's partner planes with the orbit's axial leg (§6.2) |
| the universe pair `e₊, e₋`, the scale dilator on `e₊e₋` | **kept**: `D(ρ) = (ρ + 1) + (ρ − 1)E`, `E = e_o ∧ e_∞`, composed by the flow law (§4) |
| six built planes of R⁴ on isoclinic SU(2) halves | the **base rotor** on four of the base's directions (three spatial and the axial leg), the same six planes and halves (§6.2); and the fibre's **six complex planes of C⁶**, each a commuting U(1) (§1) |
| 16 turnings, 20 boosts | 91 bivectors, 78 turnings (the base axes' pairs), 13 boosts (each with `e₋`); only the per-plane turnings, the tilt and the dilator are motions of a body |
| pole swap (negate the spans) | the conformal inversion `t → 1/t`, the reflection through the unit sphere (§5) |

## 1. The fibre: six planes in the common phase

A body carries an S¹¹ fibre over its site: C⁶ = R¹², plane p on base axes
`(2p, 2p + 1)`.

- **Plane 0 is the axis plane.** Its first axis is the pole axis, where the
  proton–neutron stretch is read (Netstratum's `t = Q/Z`, Xylarium's `Z/Q`;
  docs/129 §0).
- **Planes 1–5 are isoclinic,** interchangeable in multiplicity and fill.
  None is a spatial direction.
- **The spin turns all six in the common phase** (the Hopf action):
  `z_p → e^{i·w_p·θ} z_p`, the weighted Seifert action. Turning only five
  would fix the circle `z_iso = 0`, so the action would not be free.
- **The motion is per plane:** each plane is one rotor (a U(1)), composed by
  the geometric product and applied by the sandwich. There is no general
  2¹⁴-blade product and no matrix (docs/131's cost finding, now structural).
  The only coupling between planes is the **tilt**: one generator turning
  the pole axis toward the isoclinic plane of the open charge.

**The state a body holds:** its place on the base; on the fibre, the common
phase θ with its winding, each plane's weight `w_p`, each plane's share
split `c_p` (§2), the tilt, and the dilation ρ. That is a fixed count of
exact values (1 + 6 + 6 + 1 + 1, plus the winding), held as base and formula
(docs/117 §11.4).

## 2. The vacuum as each plane's share split

S¹¹ is a join: `S¹¹ = S¹_axis * S⁹_iso`. Each point splits its share
between the axis circle and the isoclinic sphere, `c₀ = 1/(1 + t₀)`,
`d₀ = 1 − c₀`. The vacuum is read the same way on every plane: **δ is the
plane's share split between its continuum face and its vacuum face**, never
a phase between them (a phase would break the free action):

    c_p = δ_p,   d_p = 1 − δ_p,   δ = H/(R + H)

So docs/104's two faces, docs/105's pairs and docs/134's bound share read
unchanged (they read `g`, `u` from δ). The nine whole×span boosts of
Cl(5,4) become these share splits. A body's passage through a throat
(docs/131 §4) is its splits moving through `½`, not a span crossing zero.

## 3. R is a phase on every plane

The electron's phase is not an axis. On each plane it is read from that
plane's capture phase, `ψ_p = (w_p·s·Z) mod 1`:

    R_p = κ·(1 − 2·frac(ψ_p))

In the common phase the six `R_p` are equal. This is the sawtooth Netstratum
already reads as the echo's response (`flows::mean_response`, docs/132 §5),
now the electron's phase itself. The circulation on `e_kR` (docs/128 §9.1,
`Mote::spins_orth`) is that phase's motion, not a spin on a plane of its own.

## 4. The scale

    D(ρ) = (ρ + 1) + (ρ − 1)·E,   E = e_o ∧ e_∞ (the e₊e₋ pair)

Dilations compose as the one generator by the geometric product, so the flow
law composes their rates (`ring::compose`, `compose_n` over a stride). A
change of calibre `D(60^p)` fixes both poles and keeps the faces.
`Orientation::advance_scale` is unchanged in law.

## 5. The inversion

docs/131's pole swap (negating the blades that hold a span) becomes the
conformal inversion `t → 1/t`. It exchanges the held and open faces and
the two nucleon letters (docs/104; docs/134 §5's inverse place `r' = M²/r`).
It is read, never stored: the antipolar face is the same state read through it.

## 6. What moves in the code

1. `orientation.rs`:
   - Six complex planes, one rotor each, with the common phase and its
     winding, the weights, the share splits, the tilt and the dilator.
   - Retired: the Cl(5,4) constants (`WHOLE`, `SPAN`, `R_AXIS`, `NEGATIVES`),
     the vacuum factor (`advance_vacuum`) and the so(4) halves.
   - Kept: `advance_scale`.
2. `flows::Mote`:
   - `spins` (per spatial plane) read as the orbit on the base.
   - `spins_orth` retires into R's phase (§3).
   - The `vacuum` flag reads off the share splits (docs/131 step 5's aim,
     now structural).
3. The save record changes (`values()`), a save-format change: one founding
   is laid anew (the lay-once ruling) and g557 retires.
4. docs/131 is marked superseded; EQUATIONS E71 is rewritten; GLOSSARY gains
   Cl(13,1), the common phase and the share split.

## 6.1 The uses, mapped (audit 2026-11-13)

About 70 uses of the retiring items, sorted by what replaces each:

| category | uses | replaced by |
|---|---|---|
| (a) read R's phase (the loop phase, `circulation`, `r_side`, `plane_forms`, `from_phases`, the throat passage's R crossing ½) | 8 | `R_p = κ(1 − 2·frac(ψ_p))` (§3) |
| (b) read the vacuum face (`stream_delta` and the 16 laws that read it: conform, shear, beads, strands, bands, fluid, emit, dipole, magnet, torsion sense, luminal, merge keys, shine, arrival frame, counterflow) | 17 | the plane's share split `(c, d)` (§2): which face a mote reads is a reading of its split, not a flag |
| (c) move the circulation spin or the face | ~38 | §6.2 |
| (d) the save record and display | 8 | the new record (§6 step 3) |

## 6.2 The circulation is on the base (revised after the joint read)

The fibre's weights are whole windings per plane (Xylarium, xyl_core::hopf).
They change only at an event, such as a pinch, never as a rate a shear moves.
So the R side's orbits (docs/128 §9–§14; operator 2026-11-11: "the R side is
orbits, spun by the manifold's shear") cannot be the fibre's weights. They
are motions on the base, as Xylarium holds a body's orbital plane:

- **The base rotor** is held on four of the base's directions: the three
  spatial axes and the orbit's **axial leg** (axis 4, where docs/131 held
  `e_R`). Its six planes are the three spatial planes and each one's partner
  with the axial leg. These are `Orientation`'s six planes, now on the base's
  Euclidean signature.
- **No spatial direction is assigned a fibre plane** (T18: the isoclinic
  planes are interchangeable, and ranking them would choose members by
  position).
- The partner spins `ℓ_kR` (`Mote::spins_orth`), the halves
  `ℓ± = ℓ_k ∓ ℓ_kR`, the conformation to the shear and the chiral split
  stand as built: they are the base rotor's laws.
- **The face flag** (`Mote::vacuum`) is the parity of the mote's passes
  through the inversion (§5): a topological reading kept as a bit, set
  where the inversion is passed (the Möbius pass, the throat, the exits).
- **Retired:** the Cl(5,4) axes and constants (`WHOLE`, `SPAN`, `R_AXIS`,
  `NEGATIVES`, `pairs`, `is_turning`, `pole_swap`, `carried`) and the
  vacuum factor (`advance_vacuum`, `factors`). No law read them; only
  tests did.
- **The save record** is 12 values, or 13 when the scale moves. Saved g557
  states read unchanged, since the vacuum factor never moved there.

## 7. Witnesses (fixtures, read jointly with Xylarium)

1. **Free action:** turning the common phase by one whole turn (each plane by
   `w_p` turns) returns every point; a turn of five planes alone fixes
   `z_iso = 0`, and is refused.
2. **Share split:** `c_p + d_p = 1`; the throat (`R = H`) reads `c = d = ½`;
   the inversion exchanges `c` and `d`.
3. **R's phase:** at `ψ = 0, ¼, ½, ¾` it reads `κ, κ/2, 0, −κ/2`; it is odd
   about ½; the six `R_p` are equal in the common phase.
4. **Dilator:** `D(ρ₁)·D(ρ₂)` is `D` of the flow-law composition of their
   rates, exactly, at N and 4N.
5. **Conservation:** the books, `Σ m·ℓ` on the base and the turning's degrees
   stand through a bell, as they do now.

## 8. Open

1. **The orbit's plane is a rotor** (Xylarium, xyl_core::rotor): an even
   versor of turns on named planes, carrying the anchor's reference plane to
   the orbit's. The tilt is one generator inside it. Two bodies' planes are
   compared by the relative versor `P_b⁻¹·P_a` per plane, never as one angle.
   Netstratum's shares `t_k` of `ℓ_o` over three spatial planes become that
   rotor; its law on the base is to be derived.
2. **The capture phase** (confirmed): `ψ_p = (w_p·s·Z) mod 1`, where `s` is
   the share the jet carries through the throat and `Z` is the protons
   (Netstratum's Q). In the common phase `w_p = 1`, so `ψ = frac(s·Q)`.

## 9. The joint read (2026-11-13)

Xylarium reproduced all four of §7's fixtures in xyl_core, as tests on its
branch `vacuum-yield`:
1. The common turn by 360° returns every point, and weights (0,1,1,1,1,1) fix
   the axis circle. A body state built on a frozen turn is refused by its
   builder; the type only shows the fixed circle.
2. `faces(1,1) = (½, ½)`, `faces(3,5) = (3/8, 5/8)`, and the antipole reads 5/8.
3. `electron_phase` is `κ(1 − 2·frac(ψ))`, odd about ½.
4. `dilator(2)·dilator(3)` is projectively `dilator(6)`, and
   `flow(1/3, 1/2) = 5/7`.
