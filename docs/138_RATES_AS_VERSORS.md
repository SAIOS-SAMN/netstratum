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
   by W2.

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
3. The shear restated (#199), then the release and the drift.
4. Read on a laid checkpoint: where W2's turning moves spin between planes,
   against the per-plane run.
