# 138 — Rates as versors: curvature, and closure as a force

**Operator 2026-11-15:** "derive it: compose rates as versors, file the
issue. We're constantly being drawn back to straight lines moving on a piece
of paper, and curvature without closure as a dynamic opposing force."

**Status:** derived 2026-11-15. Step 1 is built (`conductor::turning`, E76): the
fixtures below are its witnesses. Steps 2–4 are open.

## 0. What stands

The flow law `x ⊕ y = (x + y)/(1 + xy)` (LAW-DILATION, `ring::compose`) is
Einstein's collinear velocity addition: rapidities add, `x = tanh θ`. The
engine applies it **plane by plane**:
- the release's `x_k ⊕ P_k` on each plane;
- the shear's pull on each plane (`coupling::pulled`);
- the scale on its one plane `e₊e₋`.

That law is exact only for parallel boosts. Two boosts on different planes do
not compose that way. Holding the planes apart flattens the rate space onto
paper.

## 1. Axioms

- **V1 (a rate is a boost).** A rate on a plane is a boost in the algebra
  (docs/136, Cl(13,1); the hypha's conformal law in Cl(4,1)). It is held in
  its rational Cayley form, the form the engine's turnings already use
  (`orientation::Motion`): `V ∝ 1 + B`, with `B` the rate's bivector, held
  whole on every plane it moves, never as a scalar times a plane. On one
  plane, `B = β·e`. A boost plane joins `e₋` (`e₀`, `e₀² = −1`) with a spatial axis,
  so `B² = +1`. Here `β = tanh(θ/2)` is the half-rate, and the rate is
  `x = 2β/(1 + β²)`.
- **V1′ (each plane at its own angle).** A rate on several planes is
  `B = Σ B_k`, its parts on orthogonal commuting planes, each with its own
  angle `μ_k = B_k·B_k`. Its versor is `V = Π(1 + B_k) = Σ_m W_m`, the sum of
  its wedge powers `W_m = B^{∧m}/m!`. This is exact, and no plane is read
  apart: `1 + B` is the case of one plane. The angles are the roots of
  `Σ (−1)^m e_m λ^{K−m}`, where `e_m` is the scalar part of `W_m·W_m`. Each
  plane reads back exactly:
  `B_k = (Σ_m μ_k^{K−1−m} W_{2m+1})(Σ_m μ_k^{K−1−m} W_{2m})⁻¹`.

  *Proof.* `W_m(B) = W_m(C) + B_k·W_{m−1}(C)`, with C the other planes, and
  `B_k·B_k = μ_k`. Multiplying the second sum by `B_k` gives the first. Two
  planes at one angle (isoclinic) leave the second sum without an inverse,
  and their parts are not unique. *Fixtures:* three planes at ½, ⅓, ⅕ in
  Cl(6,0), turned off the axes, read back each plane and the angles −¼,
  −1/9, −1/25. In Cl(3,1), `⅓e₀₁ + ½e₂₃` (a boost and a turning at once)
  keeps the volume `⅙e₀₁₂₃`. ∎
- **V2 (composition is the geometric product).** Two motions compose as
  `V₂·V₁`, exact over ℚ, up to a scalar. Like `kⁿ` and `Dⁿ`, this is a law
  composed with itself, not a length squared (docs/132 §1).

## 2. Theorems (fixtures exact, in Cl(2,1) on `e₀, e₁, e₂`)

**W1 (parallel boosts are the flow law).** On one plane,
`(1 + β₁B)(1 + β₂B) = (1 + β₁β₂) + (β₁ + β₂)·B`, so the half-rate is
`(β₁ + β₂)/(1 + β₁β₂)`, the flow law. The per-plane law is the collinear
case of V2.

*Proof.* `B² = 1`. *Fixture:* β = ⅓, ½ gives `7/6 + (5/6)·e₀₁`, half-rate 5/7. ∎

**W2 (non-parallel boosts turn the plane between them).** On planes `e₀₁`
and `e₀₂`:

    (1 + β₁e₀₁)(1 + β₂e₀₂) = 1 + β₁e₀₁ + β₂e₀₂ + β₁β₂·e₁₂

This is a boost `B` times a turning `R` on `e₁₂`, the plane of the two
spatial directions (the Thomas–Wigner rotation). The turning's half-rate is
exactly

    γ = β₁β₂

and `B = 1 + a·e₀₁ + b·e₀₂`, with
`a = β₁(1 + β₂²)/(1 + β₁²β₂²)` and `b = β₂(1 − β₁²)/(1 + β₁²β₂²)`.

*Proof.* `e₀₁e₀₂ = e₀e₁e₀e₂ = −e₀²·e₁e₂ = e₁₂`. Expanding `B·R` and matching
coefficients gives `a`, `b` and `γ`. *Fixture:* β = ⅓, ½ gives
`1 + ⅓e₀₁ + ½e₀₂ + ⅙e₁₂`, and `B·R` reproduces it exactly. ∎

**W2′ (orthogonality is the observer's).** Which planes boost and which
turn is read against an observer's own time `t`, not against the base axes:
- the rate parts as `½(B ∓ tBt⁻¹)`, a boost that anticommutes with `t` and a
  turning that commutes with it;
- `V` carries `t` to `t′ = V·t·Ṽ/(V·Ṽ)`, the boost is `B ∝ 1 + t′t⁻¹`, and
  the turning `R ∝ B̃·V` keeps `t`.

*Proof.* `1 + t′t⁻¹` is the motion in the plane of `t` and `t′` taking one
to the other, so `B̃V` fixes `t`. *Fixture:* at `t = e₀` this is W2. Against
`t = W·e₀·W̃` with `W = 1 + ¼e₀₁`, so `t = (17/16)e₀ + ½e₁`, the same V
splits into `B = 1 + (2399/8149)e₀₁ + (4624/8149)e₀₂ + (2176/8149)e₁₂` and
`R = 1 − (28/675)e₀₂ − (119/1350)e₁₂`. Each mixes blades the base axes keep
apart, and `V ∝ B·R`. ∎

**W3 (the turning is real motion the per-plane law drops).** Composing per
plane keeps `a` and `b` (approximately, each by the flow law alone) and loses
`γ`. Whenever two boost planes move at once, the turning `γ` on the plane
between them is motion that no per-plane ledger books. *Proof.* W1 and W2. ∎

**W4 (curvature without closure).** A path that closes on paper,
`V₁ → V₂ → V₁⁻¹ → V₂⁻¹`, does not close in the algebra:

    V₂⁻¹V₁⁻¹V₂V₁ ∝ 1 + b′·e₀₁ + a′·e₀₂ + γ′·e₁₂

It leaves both a boost (the path's failure to close) and a turning (its
holonomy). Both vanish only where `β₁β₂ = 0`.

*Fixture:* β = ⅓, ½ gives `11/18 − (1/6)·e₀₁ + (1/9)·e₀₂ − (1/3)·e₁₂`, that
is, half-rates −3/11 and 2/11 left on the boost planes and a turning of −6/11
on `e₁₂`. (The first draft had +⅓ on `e₁₂`; the build's product gives −⅓.) ∎

This is the curvature of rate space: it is hyperbolic, not the paper the
per-plane law draws on. The residue is a motion. In this sense **closure is
a dynamic force:**
- a body driven around a cycle of rates on two planes is turned (Thomas
  precession) and displaced;
- the turning opposes the drive's plane, turning its direction toward the
  third plane;
- nothing has to add this force: it is what the composition is.

## 3. What it changes

1. **The ledger is the bivector, not the planes.** Spin conservation per
   plane (R9's `Σ m·ℓ_k` on each plane) is the projection of one
   conservation: the motion's bivector, all planes together. W2's `γ` moves
   spin from the boost planes into the plane between them. A per-plane book
   reads that as a leak or a remainder.
2. **The shear (#199).** The pull composed as a boost of the mote's motion
   against the field's, `V_field⁻¹·V_mote`, is a one-parameter motion where
   the field stands. It composes over n bells as a power of a versor, its
   Wigner part included. This is the restatement #199 asks for.
3. **The release (`x_k ⊕ P_k`).** A kind leaving with rates on several
   planes leaves with their composed versor, its turning between the planes
   included.
4. **The drift (A9) across the inversion.** "Drag into velocity across the
   inversion" is a coupling between faces. In this form it is a boost on a
   plane through `e₋`, and its composition with the continuum's planes turns
   by W2. *Read in the code (2026-11-15):* `plasma::drift` moves one carried
   spin `c` per parcel toward the pool's by the keep `δ` (`1 − δ` on the
   vacuum). That is a relaxation of one scalar, with no plane beside it and
   no flow-law composition, so W2 has nothing to turn. It is restated only
   when the parcel carries its spin per plane.

## 4. Build order (#206)

1. **Built.** In `turning` (E76): `versor(B) = 1 + B` of a whole rate
   bivector; `compose_versors` by the geometric product; `power` by
   squarings; `split` into `B·R` and `parts`, both against an observer's
   time (W2′), never the blade masks; `holonomy` (W4).
   Each plane is held at its own angle (V1′): `wedge_powers`, `invariants`,
   `plane_of`. Witnesses (`turning::{versor_tests, angle_tests}`):
   - the fixtures above;
   - the flow law at n = 7 and 28;
   - `V^{4N} = (V^N)⁴` at N = 7, and equal to 4N single products;
   - the Cayley reading `2b/(1 + b·b)`;
   - a rate on two planes at once has no turning;
   - the moving observer's split (W2′);
   - three angles in Cl(6,0) and a boost with a turning in Cl(3,1) (V1′).
2. **Built.** The ledger is one bivector:
   - `flows::Mote::spin` holds a mote's spin on its six planes as one
     bivector;
   - `flows::spin_ledger` is `Σ m·L`, and the per-plane books `Σ m·ℓ_k` are
     its components (witnessed on the shear's book);
   - `turning::carry` gives `V·L·Ṽ/(V·Ṽ)`, a spin carried by a motion. It
     moves between the planes the motion touches and keeps its angles.

   No law in the engine carries spin yet; that is step 3.
3. **Built.** The shear restated (#199, §5). The release (`breath`, a parcel
   leaving with `x_k` and the core's pressures `P_k`) composes whole by
   `compose_whole`: the boost gives each plane's rate, and the turning turns
   the parcel's motion as it leaves. The drift has no planes (§3.4).
   Read l109 → 222 against the shear-only run:
   - tilt ties 295 against 318;
   - 2917 of 2969 motes apart in phase, against 2918;
   - light given by recycling `51;20,03·60⁻⁹` against `17;26,56·60⁻⁹`;
   - the shear moved the same `2;05,25·60⁻³`;
   - the 3Z conservation true; fuel and bodies unchanged.
4. Read on a laid checkpoint: where W2's turning moves spin between planes,
   against the per-plane run.

## 5. The shear restated (#199): derivation, and where it stops

The shear moves a mote's rate `x_k = 2δ·ℓ_k/ℓ_o` on each spatial plane by
the pull `y_k = −σ·F_k·o·(x̄_k ⊖ x_k)` (`coupling::pulled`), one plane at a
time. Restated:
- the rates form one boost, `X = Σ x_k·e₀∧n_k`, with `n_k` the axis dual to
  plane k, and so does the pull, `Y`;
- they compose whole: `V′ = V(Y)·V(X)`. With the rate itself as the
  coefficient, the parallel case is the engine's `x ⊕ y` exactly (W1);
- `split(V′, e₀)` gives the boost, whose components are the new rates (the
  spin follows them as now, `ℓ′_k = ℓ_k·x′_k/x_k`), and the turning
  `R = 1 + c·e`, whose part on each spatial plane is `x∧y/(1 + x·y)` to first
  order: the Thomas–Wigner turning of the mote;
- the bound is the versor's own, `V·Ṽ ≠ 0`, the whole rate inside the
  luminal bound, not each plane apart.

**Where it stops.** The turning R is exact over ℚ only as a versor: its
coefficient `c` is a rate. A mote's orientation holds its motion as the
Cayley generator `g` with `motion(g) ∝ (1 + g·e)²`, and the generator of
`1 + c·e` is `g` with `2g/(1 − g·g) = c`. That `g` is irrational unless
`1 + c·c` is a rational square. So the Wigner turning cannot be composed into
an orientation held as its generator. The two exact ways out:
1. Hold each orientation as its versor (rational, closed under composition
   and under `split`), with the generator as a reading. The phase readings
   then move onto `c`.
2. Keep the generator, and book the turning as a versor beside it until it
   is read.

This is not a scale lost in the split. Composing rational unit motions
keeps a rational unit, but its polar factors are rational only up to a
scalar. *Fixture:* the half-rates ⅓ on `e₀₁` and ½ on `e₀₂` give the unit
motions `5/4 + ¾e₀₁` and `5/3 + (4/3)e₀₂`. Their product is
`25/12 + (5/4)e₀₁ + (5/3)e₀₂ + e₁₂`, a unit (`V·Ṽ = 1`). It carries `e₀`
with `γ = 697/72`. The unit boost needs `√(2(1 + γ)) = √(769/36)`, and the
unit turning `1 + (12/25)e₁₂` needs `√(769/625)`. So what is held must be the
whole motion, or versors up to a scalar, never a unit boost and a unit
turning apart. Way 1 then holds projective versors, and Way 2's generator
also cannot hold the half turn, the pole (Mycelium, 2026-11-15). A held
versor still carries K per plane beside it (law 11).

**Ruled (operator, 2026-11-15, for every kernel; Mycelium 78.173 §8.5,
PR #345):**
- the shared form is the whole unit motion R, with K per plane beside it;
- the generator is a reading where it exists;
- boost and turning parts split off a motion are read as projective
  versors;
- composition is `R₁R₂` on whole motions.

Netstratum holds each orientation as its motion (`Orientation::motion`). On
the grain the motion is `cayley(⌊generator(R)⌋)`, so it stays an exact unit.

**Built (#199):**
- `coupling::compose_whole` composes the rates and the pulls whole, gives
  the boost's part on each plane as the new rate, and reads the turning
  projectively. No root is needed for the rates.
- `flows::shear_booked` moves each spin by its new rate as before, and the
  range pays per plane as before, so `Σ m·ℓ_k` is kept.
- The turning advances the mote's motion after the bell
  (`Orientation::turn_by`). Its generator `C/(1 + √(R·R̃))` takes the root
  on the grain, as its floor. This is the one root in the law.

Witnesses:
- `coupling::tests::rates_compose_whole_and_turn_the_plane_between`: one
  plane is the flow law; two planes give W2 in the order `V(Y)·V(X)`
  (boost 9/37, 20/37, turning ⅙, norm 37/36);
- `orientation::motion_tests::a_projective_turning_enters_by_its_generator`;
- `flows::coupling_tests::a_component_past_zero_flips_its_plane_and_the_composition_turns_the_rest`:
  the motion is `H·W`, with W a small turning on the spatial planes.

Read on `.testtmp/plasma/l109` to bell 222, against the proper-time run:
- the shear moved `2;05,25·60⁻³` outward, against `2;17,12·60⁻³`, about 9%
  less: the whole composition's denominator `1 + x·y` runs over every plane;
- 2918 of 2969 motes apart in phase, against 2904 of 2968; one merge fewer;
- light given by recycling `17;26,56·60⁻⁹` against `53;54,07·60⁻⁹`;
- the 3Z conservation true; fuel, bodies and echoes unchanged; the widest
  carrier held at 38 bits for spins and reaches.

