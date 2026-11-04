# 117 — The axes and the tower: S³ → S⁷ → S⁴, and S¹¹ = S⁷ ∗ S³

This document states Netstratum's base axes and their signature, how degrees
are counted on many axes, and the tower of spheres the machine and its
quantum state stand on. It reads docs/95 (the axes), docs/97 (the nucleus on
S¹¹) and Xylarium's `circuit/axes.rs` and `atoms.rs`, and it is the ground
docs/114 §9–§10 are restated on, in degrees, without angles.

**Status:** derived 2026-11-05 (operator: "S^7 is the base and S^11 is the
quantum state that rides on S^7 - so it should be nested"; "everything
should be degrees on multi-axial machinery"). The degree split of §4 is a
witness in the code
(`nuclides::tests::the_degrees_on_s11_split_over_the_base_s7_and_the_state_s3`);
the algebra of §2 and §5 was checked in an exact Clifford algebra over the
seven axes. Nothing here changes the live physics yet. Open points are in §7.

## 1. The axes

docs/95 §1: a nucleus weighs `3Z`, one frame of each proton on each of three
axes, and each axis is a plane of its closed (Z) and open (Q) frames. With
R, the axis paired with none, that is seven base axes:

| | axes | |
|---|---|---|
| closed frames | `Z₁, Z₂, Z₃` | the hold |
| open frames | `Q₁, Q₂, Q₃` | magnetism, the open ends |
| reach | `R` | paired with none; joined to origin and infinity, it is scale |

Xylarium's machine has the same grammar, larger: eleven base axes, one pole
axis and five isoclinic planes (`11 = 1 + 2 × 5`). Netstratum's is
`7 = 1 + 2 × 3`.

## 2. The signature

Each frame plane `(Z_k, Q_k)` is hyperbolic: one of its poles squares to
+1 and the other to −1, and R squares to +1. Which pole is negative is not
fixed. It is state, plane by plane (operator 2026-11-05: "Q or Z could be
negative - the point is to have dynamic capabilities"):

    in plane k, either  Z_k² = +1, Q_k² = −1   or   Z_k² = −1, Q_k² = +1

Every state has three positive and three negative frame axes, so the base
algebra is always Cl(4, 3), and with Mycelium's conformal pair
(`e₊² = +1`, `e₋² = −1`) Cl(5, 4). §3 reads what the state changes.

The evidence is already in the record:

1. **Origin and infinity.** Xylarium's pole coordinate is `t = U/W` with Z
   at the origin, Q at infinity and the throat at `t = 1`: a null pair.
2. **The whole and the span** (docs/95 §8): the weight is `A + Q` and R is
   their span `A − Q`. With the signature above, `(e_Z ± e_Q)² = 1 − 1 = 0`:
   the whole and the span are the frame plane's two null directions.
3. **LAW-DILATION is the plane's boost** (docs/114 T3). With
   `(e_Z·e_Q)² = +1`, the unnormalised boosts compose as
   `(1 + p·e)(1 + q·e) = (1 + pq) + (p + q)·e`, the flow law
   `p ⊕ q = (p + q)/(1 + pq)`; one-sided on the null directions a boost
   scales `e_Z + e_Q` by `1 − p` and `e_Z − e_Q` by `1 + p`, their ratio
   `D(p) = (1 + p)/(1 − p)`. T3 found the boost algebra without naming it.
4. **The face inversion** `t ↦ 1/t` exchanges origin and infinity and fixes
   the throat, as exchanging the two null directions does.

## 3. Degrees on many axes

An element of the algebra stands on a set of axes; its **degree** is how
many. Multiplication is set arithmetic, with no angle anywhere:

    e_A · e_B = ± e_(A Δ B),   degree(A Δ B) = |A| + |B| − 2·|A ∩ B|

`A Δ B` the axes in exactly one of the two; the sign is the count of swaps
that reorder the axes, times the square of each shared axis. On the seven
axes the degrees hold 1, 7, 21, 35, 35, 21, 7, 1 elements. A plane is any
two axes, not only a frame plane: `Z₁Q₂` and `Z₂Z₃` are planes too.

A degree-2 element `e_a·e_b` squares to `−a²·b²`, so it behaves by the
signs of the two axes it joins, not by their names:

| it joins | squares to | it is |
|---|---|---|
| two positive axes | −1 | a turning |
| two negative axes | −1 | a turning |
| a positive and a negative axis | +1 | a boost: LAW-DILATION and the flow law |

In every state of the six frame axes there are three positive and three
negative, so there are always six turnings and nine boosts. The three
positive axes form one quaternion triple (`i, j, k` with `i·j = k = −j·i`,
Q₈), the three negative axes another, and the two triples commute: the left
and the right of an isoclinic turn, the kind the core's twist already is
(docs/100 §2). The boosts are the nine pairs between the triples.

What the state changes is the triples' members. With every plane alike, the
closed frames are one triple and the open the other. Flipping plane k's
signature moves `Z_k` into the other triple and `Q_k` the other way: with
plane 3 alone flipped, the positive triple is `Z₁, Z₂, Q₃`. Read in all
eight states of the three planes: six turnings and nine boosts in each.

## 4. The tower

Three spheres, nested:

| sphere | in | what it is |
|---|---|---|
| **S⁴** | ℍ ∪ {∞} | the machine's base directions |
| **S⁷** | ℝ⁸ = ℍ² | the machine: the R axis paired into a plane, and the three frame planes |
| **S¹¹** | ℝ¹² = ℝ⁸ ⊕ ℝ⁴ | the quantum state riding on the machine |

**S⁷ over S⁴.** S⁷ is the quaternionic Hopf fibration `S³ → S⁷ → S⁴`: over
each point of S⁴ stands an S³ of turning. The point of S⁴ is the ratio of
the two quaternions a point of ℍ² is (the quaternionic projective line), as
Xylarium's pole coordinate is the ratio `t = U/W` on one axis.

**S¹¹ = S⁷ ∗ S³.** S¹¹ is the join of the machine S⁷ with a second S³, the
quantum state `q` (docs/114 §9). Joining adds the dimensions and one:
`7 + 3 + 1 = 11`. The one is the share between the two: a point of S¹¹ is a
point of the machine, a state, and how much of its size stands on each.
At one end it stands wholly on the machine, at the other wholly in the
state.

Not `S⁷ ∗ S⁴`: that is `S¹²` in ℝ¹³, whose degrees count 1, 13, 90, … and
not the 1, 12, 77, … every nuclide is built on. Not `S⁷ × S⁴` either: that
has dimension 11 but holes in dimensions 4 and 7 a sphere does not, and its
harmonics do not group into S¹¹'s. S⁴ belongs inside the machine, as the
base of S⁷'s own fibration.

**The degrees nest exactly.** A degree-l harmonic on S¹¹ splits over the
machine's degree a and the state's degree b, with j paired:

    d₁₁(l) = Σ_{a + b + 2j = l} d₇(a) · d₃(b)

with `d₇` = 1, 8, 35, 112, … (S⁷ in ℝ⁸) and `d₃` = 1, 4, 9, 16, … (S³ in
ℝ⁴). For example `77 = 35 + 8·4 + 9 + 1` and `352 = 112 + 35·4 + 8·9 + 16 +
8 + 4`. Witnessed for every degree below 40. So the table already counted on
S¹¹ (docs/97 §2) stands unchanged: it is the machine's degrees and the
state's degrees taken together.

## 5. A nuclide's place as a bidegree

A kind's open degree l on S¹¹ (`nuclides::place`) reads as `(a, b, j)`:

- `a`, its degree on the machine (S⁷): where on the axes its ends stand;
- `b`, its degree in the quantum state (S³): its turning;
- `j`, the ends paired across the two: the fibres that bind them.

The two kinds of degree meet in the state. The harmonic degree (how deep on
the sphere, symmetric in the axes) and the degree on the axes (which axes,
antisymmetric) are stepped together by the first-degree operator
`∂ = Σ e_a·∂_a`, which moves the harmonic degree by one and the degree on
the axes by one (Clifford analysis: each harmonic degree is
`M_l ⊕ x·M_(l−1)`, `x·` a degree-one product, no square).

## 6. Where the passage stands

The passage (docs/114 §9) acts on the state, the S³ riding on the machine,
never on the machine's point: a crossing turns the quantum state at a place.
Its group Q₈ is the degree-2 elements of one same-signed triple of §3
(the closed frames when every plane is alike), acting one-sidedly, exact
over ℚ. A loop's obstruction (docs/114 §10) is the class
of its product of units in that Q₈, over the points of the base.

**A crossing in plane k is plane k's signature flipping.** It moves the
plane's two poles between the positive and the negative triple (§3): the
face swap of that plane, with no flag beside the state. Flipping two planes
in either order reaches the same signature, but the turnings carried
through the flips do not commute, so the order is held in the state `q`'s
units (docs/114 §10). Since S³ appears twice in the tower (as S⁷'s Hopf
fibre and as the riding state), the conjecture is that a flip moves the
turning between the machine's fibre and the riding state. Not derived
(§7).

## 7. What is open

1. **What turns a plane over:** derived in §8 (its direction passing
   null). Still open: how "forward" on R reads in a mixed state, and §8.6.
2. **The eighth dimension of ℝ⁸.** Seven base axes stand under S⁷'s eight:
   R is paired into a plane with something, as Xylarium pairs its pole
   axis. docs/95 §2 joins R to origin and infinity (`e₊`, `e₋`); whether
   R's partner is the conformal pair's direction is not derived.
3. **The join's share and the deficit.** The join's share is the machine's
   against the state's; the deficit `δ = H/|q|₁` is the hold's share inside
   the state. Whether they are one reading is not derived.
4. **The L1 chart of the tower.** The Hopf map's ratio `q₁·q₂⁻¹` and the
   join are usually written with Euclidean norms, which square. Each needs
   its first-degree, L1 form before it is built; until then they stand as
   counts and algebra, not as charts.
5. **The face swap's conjecture** (§6).
6. **Mycelium's signature.** Mycelium's algebra is Cl(n + 1, 1), every base
   axis positive; §2 needs half of Netstratum's frame axes negative. A
   cross-project question, raised with Mycelium before anything the court
   reads depends on it.
7. **docs/114 §9–§10 restated in degrees.** "Quarter turn", "half", the
   arc chart and every angle are to be replaced by the degree-2 elements of
   §3 and the degrees they exchange.

## 8. Velocity as the generator, drag as its contraction, the horizon

Derived 2026-11-05 (operator: the per-plane flip "is just palindrome
construction and deconstruction ... on a single dimensional traversing
linear ley lines"; "increase to degrees as velocity with inverse as drag";
"the wedge is flat - you're missing the 2nd and 4th dimension"; "you're
still losing density on B²"). Every reading below was checked against an
exact Clifford algebra over the seven axes, and none of them squares.

### 8.1 A plane's state is a direction

Plane k's state is one direction in its hyperbolic plane, the hold on its
closed axis and its part of `q` (docs/114 §9.1) on its open axis:

    v_k = H·e_Zk + R·t_k·e_Qk

The three directions with R are the body's point on the machine (S⁷).
Nothing here is new state.

### 8.2 Velocity is the generator; drag is its contraction

A motion is a degree-2 generator `B`: its components stand on pairs of
axes, 21 on the seven, each with its own size. Velocity is `B` itself,
the planes a motion turns or boosts in, never a vector wedged with one axis
(that gives a single flat plane). Drag is the contraction, the part of a
product that lowers the degree back toward the hold.

On one plane's own axes the split gives the shares the code already carries:
read from the open axis `Q_k`, velocity `δ = H/(H + R_k)` and drag
`1 − δ = R_k/(H + R_k)`, the turning share and `flows::settle_disc`'s drag,
derived rather than assigned; read from the closed axis `Z_k`, the two
exchange, which is the other face. A face is the axis a plane's state is
read from. Toward an axis outside the plane the product is all velocity,
with no drag.

### 8.3 B is read in first degree, never by its square

`B²` folds 21 components into a rate and a degree-4 coupling, products of
sizes that no longer say which planes move, at what size each, or how they
meet: it loses density. So `B` is read component by component, and
everything the square summarised is read from the components directly:

- **Each component's character, by its axes' signs:** a component on two
  axes of one sign is a turning, on two of opposite signs a boost (§3).
  No size enters.
- **How two components meet, by set arithmetic on their axes:**
  - **disjoint** (no axis shared): they commute, and together they are a
    screw pair, two planes moving at once. This is the degree-4 coupling
    the square would have summed away; here it stays a named pair with each
    size kept.
  - **sharing one axis X:** they compose into a coupling of their two other
    axes. Where those two axes are of opposite sign and the two sizes are
    equal in magnitude, the pair is **null: a horizon**.
- **Size** stays L1: the sum of the components' magnitudes.

Checked against the exact algebra:

| B | read in first degree | exact `B²` |
|---|---|---|
| boost `Z₁Q₁` 1 + turning `Z₂Z₃` 1 | disjoint: a screw pair | degree 4 only |
| turning `Z₁Z₂` 1 + boost `Q₁Z₂` 1 | share `Z₂`, `Z₁`/`Q₁` opposite, sizes equal: null | `0` |
| turning `Z₁Z₂` 1 + boost `Q₁Z₂` 2 | share `Z₂`, sizes unequal: not null | a degree-0 part |

A screw at equal sizes (the first row) is where a degree-0 reading of the
square would have called a horizon falsely: its rate vanishes but its two
planes still move.

### 8.4 The horizon

A plane's horizon is where its direction `v_k` is null, `|R·t_k| = H`: then
any motion carrying it, `v_k ∧ e_X`, is a null pair (§8.3's second row),
and its coupling to every other axis turns over at once, turnings to
boosts and boosts to turnings, five axes at the same instant, R among them.
The axes keep their squares; the character of the state's motion changes,
so the dynamic signature of §2 is the state's own, reached by the motion.

The built luminal bound (`flows::luminal`, whole mote) is the case of a
disc-settled tilt, `t_k = 1`. With a spread tilt each plane reaches its
horizon at its own reach (isotropic: all three at `R = 3H`, the whole still
outside, `δ = ¼`): a change in how the universe moves, read before and after
on a fresh founding before it is built.

### 8.5 The riding state on even degrees

The state `q` (docs/114 §9) stood on degrees 0 and 2 of three axes, the
quaternions. On the seven axes the even degrees are 0, 2, 4 and 6
(1 + 21 + 35 + 7 = 64 components): a state that can hold screw pairs
(degree 4) and the six-axis couplings (degree 6) as its own, not only one
plane's turning. A motion `B` acts on it one-sidedly, exact over ℚ, and the
state is read as `B` is, component by component, never by a square.

### 8.6 What is open

1. **The three-plane event.** A component of degree 3 among the planes'
   directions (`v₁ ∧ v₂ ∧ v₃`) changes sign only through a crossing of all
   three; whether that is the whole body's passage is not derived.
2. **Obstructions by motion type.** The obstructions of docs/114 §10 are to
   be read by §8.3's first-degree types (turning, boost, screw pair, null),
   not only by Q₈'s units.
3. **The tower's levels.** Whether a horizon on the machine (S⁷) is a degree
   step in the riding state, through `∂` (§5), is not derived.
4. **docs/114 §9's `i·q`** turns two planes into each other; against §8.4's
   crossing as a plane's direction passing null, which is the passage is not
   decided.
5. **Ties** of two planes at their horizons in one bell; a rule is owed.
6. **Which hold** a plane reads: at the whole spin or its own part's.
