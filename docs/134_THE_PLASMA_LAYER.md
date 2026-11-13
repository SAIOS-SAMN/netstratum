# 134 — The plasma layer: the electrons' state carries the flow

**Operator 2026-11-13:** "there's a quantum and higher dimensional flow that
the atomic state isn't taking into consideration by moving them at different
levels"; "this needs derivation and an additional layer of physics".

**Status:** derived 2026-11-13; §2–§3 built (`conductor::plasma`). Read
against docs/105 §3 (the electrons' pair), `cfe::almanac::atom::open_at`
(ionization by heat), `pulse::push_cut`, `pulse::cascade`,
`breath::breathe` (the bounce's release), `flows::founding` and
`formation::carried`.

## 0. What stands

- **Every release carries each kind alone.** The bounce gives kind z the spin
  `ℓ_z = p·w̄/w_z` (`push_cut`, `disperse`, the release in `breathe`,
  `flows::founding`, `formation::shells`), the founding `(K + s_i)/w_z`
  (`cascade`), a star core `½·(e_z/ē)·(w̄/w_z)` (`formation::carried`). Each
  kind is a gas of its own: hydrogen rides outermost, the heavy kinds in.
  On g557 the core's first star (bell 558) holds no hydrogen; 56% of the
  hydrogen nuclei stand in the ocean.
- **The electrons have a pair and a heat, never joined.** docs/105 §3: the
  electrons' free ↔ bound pair stands at the bound share `g`, the held face
  at the place. `open_at`: a winding held no tighter than the heat (both in
  hydrogen's unit) is stripped. Neither enters the motion.
- **Nature:** star-forming gas is one mix everywhere (hydrogen 0.70–0.75 by
  mass, centre to rim); a plasma's kinds move as one fluid, held by the
  field their charge separation would raise (the ambipolar field), and part
  only by a slow drift (settling goes by weight against charge).

The missing layer is the **plasma**: each kind's charge state at its place
and heat, and the impulse shared through that charge.

## 1. The electrons' bound share

A kind's electron is bound where it is both held at the place and not
stripped by the heat. The heat strips a winding of tightness `t ≤ a`
(`open_at`); of the electrons left, the pair binds the held share `g`
(docs/105 §3). So kind z with `Q_z` protons keeps `E_z(a)` electrons in
windings tighter than `a`, and its **bound share** is

    b_z = g · E_z(a) / Q_z        (a kind with no protons: b = 1)

- cold (`a = 0`, `E = Q`): `b = g`, docs/105's pair;
- every winding stripped (`E = 0`): `b = 0`, the kind wholly ionized;
- at the throat (`g = ½`, docs/104 §1.2), cold: `b = ½`.

A cluster keeps every electron its bonds hold (`E = Q`) unless dissociated
(`formation::dissociate`). The kind's **charge state** is `Q_z − g·E_z(a)`.

## 2. The impulse shared through the charge

The bound share answers by its own inertia; the ionized share answers as the
plasma does, as one: its electrons and ions held together by the field a
separation would raise. The field is not a parameter; it is the reading that
keeps the plasma's charge whole, so the ionized share of every kind takes
one spin. A release giving kind z (amount `n_z`, weight `w_z`) the impulse
`F_z` a nucleus carries it

    ℓ_z = b_z·F_z/w_z + (1 − b_z)·S,     S = Σ n(1 − b)F / Σ n(1 − b)w

`S` is the ionized pool's own spin: its impulse over its holding.

- **Conserved:** `Σ n·w·ℓ = Σ n·F` exact for any `b`, the same as the release
  before (proof: `Σ n·b·F + S·Σ n(1 − b)w`).
- **Limits:** `b = 1` everywhere is the release as it stood (`F_z/w_z`); `b =
  0` everywhere carries every kind at one spin `S`, the mix moving whole.
- **The bounce** (`F = p·w̄` for every kind) at one bound share `b`: `S = p`,
  so `ℓ_z = b·p·w̄/w_z + (1 − b)·p`. Cold at the throat, `b = ½`: half the
  sorting.

The released matter is cold (`breathe` releases at agitation 0) and the
galaxy's throat holds no heat of its own, so the bounce releases at `b = ½`.
A star core is hot (`formation::core_heat`): there `E_z(a)` falls as the
heat strips, and the binding moves toward one mix.

## 3. Witnesses

1. `Σ n·w·ℓ = Σ n·F` exact, for mixed `b_z` (`plasma::tests`).
2. `b = 1` returns `F_z/w_z`; `b = 0` returns `S` for every kind.
3. Cold `b = g`; hydrogen at heat 1 has `b = 0`; a kind with no protons `b = 1`.
4. The bounce at one `b`: `S = p`.

## 4. Open

1. **The drift.** After release each kind's departure from the parcel's spin
   relaxes by the attractor's keep, `ℓ̄ + k_z(ℓ_z − ℓ̄)`, `k_z` read from the
   meetings a bell of charged kinds (docs/132 §1.1's exchange form, `k_zⁿ`
   over a stride). Not built: the release's coupling is.
2. **The throat's heat.** The galaxy core holds no heat; the release is cold
   by construction. Docs/105's heat pair (`a* = u·e/(u·e + g + e − g·e)`) at
   the throat would make it a reading.
3. **The magnet on the charged share.** docs/121's coupling reads `Σ` charge;
   the neutral share slips through the field (ambipolar diffusion).
4. **The electron fluid on R.** The bound share is a reading; the electrons'
   phase on `e_R` (docs/131) is carried by the ions, not yet a motion of its own.
