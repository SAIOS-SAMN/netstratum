# 131 — The state on Cl(5,4): uniform phase states

**Operator 2026-11-12:** "what happened to Cl(5,4)(Q) or Cl(n+1,1)? — we
have to have uniform phase states as defined by fluid hydromagnetics"; then
"yes - Cl(5,4) over Q, proceed".

**Status:** derived 2026-11-12. Build order in §9, witnesses in §8. Read
against docs/95 §0, docs/97, docs/117 §2–§3 and §10, docs/128 §9–§14 and
`conductor::turning` (exact in any signature).

## 0. What was lost, and why it matters

docs/117 §2 counted Netstratum's base axes:
- three planes of held (`Z_k`, neutron) and open (`Q_k`, proton)
  dimensions;
- R, the electron's phase, the axis that pairs with none;
- Mycelium's conformal pair `e₊, e₋` beside them.

With each plane hyperbolic, that is **Cl(5,4)**: nine axes, one pole axis
and four hyperbolic planes (`9 = 1 + 2·4`). It is the grammar of Xylarium's
`11 = 1 + 2·5`.

Two reductions followed, neither ruled:
1. **9 → 8.** docs/117 §10.2 read R as the conformal pair's own span. That
   merged R's axis into the pair and gave (4,4); R lost its own dimension.
2. **8 → 4.** `conductor::orientation` holds a body's turning on the four
   continuum axes `e₊₁, e₊₂, e₊₃, e_R` alone, Cl(4,0): six turnings, no
   negative axis, no boost. As a result:
   - the vacuum is a flag beside the state (`Mote::vacuum`, "never
     reassigned"), against docs/117 §6 ("no flag beside the state");
   - the lift is a number beside the state;
   - the bounce lays a nucleus's spin on 3 of the 16 turnings and on none of
     the boosts (`breath.rs`, the release).

This is the home galaxy's missing disc (docs/128 §14.1). Its matter stands
inside the core's ring because its spin is read on too few axes.

## 1. The axes

Nine axes over ℚ, numbered as `turning::base_axis` numbers Mycelium's
(axes 1–3 on bits 0–2, the conformal pair on bits 3 and 4, axes from 4 on
bits from 5):

| axis | bit | squares to | it is |
|---|---|---|---|
| `e₊₁, e₊₂, e₊₃` | 0, 1, 2 | +1 | each plane's **whole** (`Z_k + Q_k`) |
| `e₊` | 3 | +1 | the universe's whole: origin and infinity together |
| `e₋` | 4 | −1 | the universe's span: origin against infinity |
| `e_R` | 5 | +1 | R, the electron's phase: the pole axis |
| `e₋₁, e₋₂, e₋₃` | 6, 7, 8 | −1 | each plane's **span** (`Z_k − Q_k`) |

The negative mask is bits 4, 6, 7 and 8. A plane's state stands on its two
poles, `Z_k·n_Z + Q_k·n_Q = (Z_k + Q_k)·e₊ₖ + (Z_k − Q_k)·e₋ₖ` (docs/117
§10.1), the whole on the positive axis and the span on the negative.

## 2. The motions: 16 turnings, 20 boosts

A motion's rate is a generator, a bivector on the 36 pairs of axes. Its
character is read from its two axes' signs, never from a table (docs/117
§3): two of one sign is a turning (the component squares to −1); one of each
is a boost (it squares to +1).

| | pairs | count | what they carry |
|---|---|---|---|
| turnings, positive | `e₊ₗe₊ₘ` | 3 | the continuum's orbit about axis k (the built spatial planes) |
| | `e₊ₖe_R` | 3 | the R side: the orbit's circulation (the built partners) |
| | `e₊ₖe₊` | 3 | each plane turned against the universe's whole |
| | `e_R e₊` | 1 | R turned against the universe's whole |
| turnings, negative | `e₋ₗe₋ₘ` | 3 | the vacuum's orbit about axis k (docs/117 §10.3) |
| | `e₋ₖe₋` | 3 | each plane's span against the universe's span |
| boosts | `e₊ₖe₋ₖ` | 3 | each plane's own dilation: its pole ratio `Q_k/Z_k` |
| | `e₊ₖe₋ₗ` (k ≠ l) | 6 | one plane's whole against another's span |
| | `e_R e₋ₖ`, `e_R e₋` | 4 | R against each span and the universe's |
| | `e₊ₖe₋`, `e₊e₋ₖ` | 6 | each plane against the universe |
| | `e₊e₋` | 1 | the conformal dilator: the universe's scale |

Sixteen turnings, twenty boosts, 36 in all. The built orientation is the
first six rows' first two lines (six turnings).

## 3. Uniform phase states

In magnetohydrodynamics every parcel of the fluid carries the same state —
density, velocity, pressure, field — whatever region it is in. A region is
told by the values, never by a kind of parcel. Here a mote's state is, in
every stream alike:

| MHD | on Cl(5,4) | held as |
|---|---|---|
| density | its frames, `X = Σ_k (Z_k + Q_k)·e₊ₖ + (Z_k − Q_k)·e₋ₖ` | a reading of its kind (not stored) |
| velocity | its motion, held as three commuting factors (§9 step 2), with windings `K` on their turnings | stored, on the grain |
| angular momentum | its spin `L`, a bivector, 36 components | stored, on the grain |
| pressure | the boost share of its motion: `Σ_boosts |G| / Σ_all |G|` | a reading |
| field | its open poles carried by its motion, `M·n_Q·M̃` | a reading (frozen-in flux) |

**The stream is a reading.** A mote's continuum share is its spin on the
positive turnings, its vacuum share its spin on the negative turnings, each
L1:

    c = Σ_{++} |L| / Σ_turnings |L|,   v = Σ_{−−} |L| / Σ_turnings |L|

`Mote::vacuum` retires. A law that read the flag reads `v ≥ ½` where it
needs one answer, and the shares where it can weigh both.

## 4. The passage is the planes' dilation

A boost on plane k, `e₊ₖe₋ₖ` at rate `b`, composes by the flow law
(`turning::composed`, Hyperbolic) and moves the pole ratio by LAW-DILATION
(docs/117 §10.1). Carried by boosts on planes l and m, a continuum turning
`e₊ₗe₊ₘ` gains a part on `e₋ₗe₋ₘ`: in the motion's action
`L ↦ M·L·M̃`, every `e₊ₗ` gains a part on `e₋ₗ`. So spin passes from the
continuum to the vacuum continuously, by the dilation of the planes it turns
in, with no flip and no flag.

**The face** of plane k is the sign of its carried span, `⟨M·X·M̃⟩` on
`e₋ₖ` (docs/117 §10.1: the sense of a crossing is the sign of the span). The
throat is the span at zero (`Z_k = Q_k` as carried), the fixed sphere of the
pole swap, not a null state. The poles (wholly held, wholly open) are null.

**The throat test** (docs/128 §14.2) becomes plane k's carried span passing
zero, read signed.

## 5. The lift is the conformal dilator

The lift `u` (docs/121 §14.6) displaces a body's reach from its spin
balance, `R = R_ℓ·D(u)`. The universe's scale is the conformal pair's boost,
`e₊e₋`, and LAW-DILATION is how it acts on origin and infinity (docs/117
§10.1). So the lift is the motion's component on `e₊e₋`, composed by the
flow law, and `Mote::lift` retires into `G`. The magnet's boost (docs/121
§14) acts on that component.

## 6. Reach reads every turning

A body's orbit spin in plane k is its spin about axis k on both faces and on
the R side, first degree:

    ℓ_k = |L_{e₊ₗe₊ₘ}| + |L_{e₋ₗe₋ₘ}| + |L_{e₊ₖe_R}|

Its reach `R = Σ_k ℓ_k / H(<ℓ)`, dilated by the lift (§5). The ring of each
isoclinic plane (docs/128 §12) reads its plane's own `ℓ_k`. Spin on the
universe's turnings (`e₊ₖe₊`, `e₋ₖe₋`, `e_R e₊`) is the body's turning
against the whole, not orbit: it adds to no reach (open: §10).

## 7. The bounce lays the spin on the nucleus's own frames

A nucleus of `Z` neutrons and `Q` protons stands, on each plane, at the pole
ratio `Q/Z` (docs/95 §0: each nucleon in each of the three axes). The
throat's state is the ratio 1 (span zero). The plane's boost carrying the
throat's state to the nucleus's has the rate whose dilation is that ratio,
`D(b) = (1 + b)/(1 − b) = Q/Z`:

    b = (Q − Z)/(Q + Z)          (on each frame plane, the dipole's pole negated)

The bounce releases the nucleus with its carried spin `ℓ = p·w̄/w` on the
continuum turnings (as built) and its frames' boost `b` on each `e₊ₖe₋ₖ`.
The release's spin is then `L = M_b·L₀·M̃_b`: a neutron-rich nucleus keeps
its spin on the continuum; a proton-rich one carries part of it onto the
vacuum's turnings; the spin's size is kept (a motion keeps `L`'s grade).

- **Hydrogen** (`Z = 0`) stands at the pole, `b = −1`: wholly open, a null
  state, where the Cayley form has no motion (`TurningRefusal::NoMotion`).
  Hydrogen's frames stand at infinity. What its spin does there is open
  (§10); it is laid on the continuum as built until derived.

## 8. Witnesses

1. **Character from signature:** each of the 36 pairs reads elliptic or
   hyperbolic by `turning::Regime::of`, 16 and 20, in agreement with §2.
2. **The built motion embeds:** a generator on the six built planes composes
   on Cl(5,4) to the same components the isoclinic halves give
   (`orientation`'s tests' values, unchanged).
3. **Spin passes by dilation, kept in size:** a continuum turning carried by
   equal boosts on its two planes gains a vacuum part; the carried bivector's
   reverse product is unchanged (`(M·L·M̃)·(M·L·M̃)~ = L·L̃`).
4. **The face swap is an involution:** the pole swap on plane k (reflection
   along `e₋ₖ`) applied twice is the identity on `X`, `G` and `L`.
5. **The lift composes by the flow law** on `e₊e₋`, and
   `R = R_ℓ·D(u)` reads the same reach as the built lift.
6. **The bounce conserves:** the reservoir pays the spin the release leaves
   with, summed over every turning (§7), and the holding released equals
   the holding taken.
7. **No flag:** a mote's continuum and vacuum shares sum to one wherever it
   turns, and the streams' census reads from them.

## 9. Build order

Each step replaces in place (one model per quantity) and lands with its
witnesses and a read on a copy of g557.

1. **The algebra:** the nine axes, the 36 planes and their characters, the
   pole swap; witnesses 1 and 4.
2. **The orientation on Cl(5,4)** (built 2026-11-12). The motion is held
   as three factors on disjoint axes, which commute, so the motion is their
   product, each composed in closed form:
   - **the continuum's**, on `e₊₁e₊₂e₊₃e_R`: the six built planes, on their
     isoclinic halves, as before (witness 2: the general Cayley form's
     components on that subspace, on the grain);
   - **the vacuum's**, on the spans' three turnings `e₋ₗe₋ₘ`: one quaternion
     half of the opposite hand (`(e₋₂e₋₃)(e₋₃e₋₁) = +e₋₁e₋₂`, the inverted
     face's mirror), witnessed against the general form;
   - **the scale's**, on the conformal dilator `e₊e₋`: one rate, composed by
     the flow law (§5).

   **Measured, and why not the general form.** `turning`'s general Cayley
   form on Cl(5,4) gave the same components as the halves on the six built
   planes, at 43 ms an advance against 0.19 ms (release). With the three
   plane dilations and the dilator added, one advance did not finish in ten
   minutes: the inverse is taken by elimination over the subalgebra the
   value's blades generate, which the boosts open toward all 256 blades.
   So the 19 other boosts and the 7 universe turnings are not held in the
   orientation. The boosts act on a body's spin and frames by the carry
   `M·x·M̃` (§4, `orientation::carried`, exact in the general form, where
   only a few products are taken); the universe turnings stay open (§10).
   This is a reduction of the motion, stated here; the nine axes and the 36
   planes stand in the state's algebra.
3. **The spin as a bivector:** `spins` and `spins_orth` become `L` on the 36
   planes; reach by §6 (witness 3).
4. **The lift into `G`** (§5, witness 5).
5. **The stream as a reading** (§3–§4, witness 7); `Mote::vacuum` retires.
6. **The bounce on the frames** (§7, witness 6); read `observe streams` for
   matter between the ring and the rim, and bands.

## 10. Open

1. The universe's turnings (`e₊ₖe₊`, `e₋ₖe₋`, `e_R e₊`): what drives them
   and whether they bear on reach.
2. Hydrogen at the pole (§7).
3. The seam with Mycelium's Cl(n+1,1): Cl(5,4) and Cl(8,1) are not
   isomorphic (`p − q` is 1 against 7, mod 8). The court reads Netstratum's
   state through a translation at the seam, raised with Mycelium as a
   cross-project matter.
4. Cost (measured, §9 step 2): a motion held whole on Cl(5,4) needs a
   closed form for the even subalgebra (`Cl⁰(5,4) ≅ M(16, ℝ)`: a motion is
   a 16×16 matrix, composed by integer products over one denominator, as
   the halves are) before the boosts can be held in the motion rather than
   carried.
