# 134 — The plasma layer: the electrons' state carries the flow

**Operator 2026-11-13:** "there's a quantum and higher dimensional flow that
the atomic state isn't taking into consideration by moving them at different
levels"; "this needs derivation and an additional layer of physics".

**Status:** derived 2026-11-13; §1–§2 and §6 built (`conductor::plasma`, `flows::Mote::parts`), read §3.1. Read
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

## 3.1 The read

One 2^40 founding under each law, laid to just past its first echo
(`.testtmp/plasma/{old,u}`), before → after:

| | before (per kind) | after (shared) |
|---|---|---|
| T_u | 186 | 110 |
| stars at the first echo | 106 | 35 |
| stars holding hydrogen | 1 | 3 |
| hydrogen in the ocean (grains) | 18;27·60^5 | none |
| hydrogen in the throats | 30;13·60^5 | 32;58·60^5 |
| hydrogen kept by stars | 14;59·60^5 | 15;42·60^5 |

The shared release no longer throws hydrogen past the galaxy: the founding's
rim draws in (T_u 110). The stars still form nearly one kind each (the three
holding hydrogen hold only hydrogen; 34 of 35 bindings had none fall in),
because each mote carries one kind (`Mote::key`): a binding gathers parcels
that were each sorted at birth. That is the next defect of the same layer
(§4.5).



1. **The drift.** After release each kind's departure from the parcel's spin
   relaxes by the attractor's keep, `ℓ̄ + k_z(ℓ_z − ℓ̄)`, `k_z` read from the
   meetings a bell of charged kinds (docs/132 §1.1's exchange form, `k_zⁿ`
   over a stride). Not built: the release's coupling is.
2. **The throat's heat** (resolved by §7, T1: nothing is stripped there, `b = ½` exactly). The galaxy core holds no heat; the release is cold
   by construction. Docs/105's heat pair (`a* = u·e/(u·e + g + e − g·e)`) at
   the throat would make it a reading.
3. **The magnet on the charged share.** docs/121's coupling reads `Σ` charge;
   the neutral share slips through the field (ambipolar diffusion).
4. **The electron fluid on R.** The bound share is a reading; the electrons'
   phase on `e_R` (docs/131) is carried by the ions, not yet a motion of its own.
5. **A parcel of one kind** (derived in §6). A mote holds one kind, so the medium is an array
   by kind, never a mix: the shared release moves the kinds together, but each
   still travels in its own parcel. A mote as a mix (its manifest, one state)
   is the plasma's parcel.

## 6. The plasma's parcel (§4.5, derived and built 2026-11-13)

A release's ionized share of every kind moves at one spin `S` (§2), so it is
one parcel: a **mix**. Each kind's bound share moves by its own inertia at
`F_z/w_z`, so it is a parcel of that kind alone. A mote is a mix: its parts
`{kind → amount}` under one state (place, spins, orientation, agitation, face).
This is the seam's parcel form agreed with Xylarium (§5).

- **Readings of a parcel** are its parts' sums, never a representative kind:
  - its holding `Σ a·w`;
  - its open frames `Σ a·U`;
  - its charge `Σ a·Q`;
  - its dimensions `Σ a·dims`;
  - its grip, read on its parts' manifest (`formation::grip`).
  A law that read one kind's property per mote reads the parcel's
  holding-weighted share of it. A parcel with no open frames is closed
  (inert).
- **A one-kind parcel reads exactly as the mote of that kind did.** Every
  law's value on a one-kind parcel is unchanged.
- **The release** writes, per bounce:
  - one parcel of the pool, `{z: n_z(1 − b_z)}` at `S`, on the nucleon grid;
  - per kind, the bound share `n_z·b_z` at `F_z/w_z`.

  The pool's phase reads its own carried spin, the same reading a kind's
  release reads.
- **Merging** sums parts: parcels of one bead join, and their parts add by kind.
- **The save record** writes a parcel's parts. A one-kind parcel is written
  as before, so g557 reads unchanged.

**Witnesses:**
1. A one-kind parcel's readings equal the kind's.
2. A release conserves nucleons by kind and `Σ n·w·ℓ = Σ n·F`, with the pool
   as one parcel.
3. A merge keeps each kind's amount exactly.
4. The record of a mixed parcel reads back.

**Read** (one 2^40 founding, `.testtmp/plasma/mix`, laid to bell 112, T_u 110,
against §3.1's shared release with one-kind motes): stars at the first echo
35 → 4, none holding hydrogen (was 3); hydrogen in the galaxy's flows none →
7;51·60^5 grains and in the ocean none → 7;51·60^5; the intergalactic medium
≈ 0 → a third of the whole. The stars still form one or two kinds each. g557
reads unchanged (its motes are one-kind parcels).

## 7. The release, derived: axioms, theorems, proofs (2026-11-13)

**Operator:** "derive the answers with proofs, theorems, and axioms". The
question: why the mixed-parcel read (§6) gave 4 stars and none holding
hydrogen, against 35 and 3 before it.

### 7.1 Axioms (each is a law already standing; none is new)

- **A1 (impulse).** A bounce gives every nucleus in the throat the impulse
  `F = p·w̄`, where `w̄` is the galaxy's mean weight (`pulse::push_cut`,
  docs/96 §7).
- **A2 (the pair).** A kind's electrons bind at the held face `g` of the place
  (docs/105 §3). At the throat `g = u = ½` (docs/104 §1.2).
- **A3 (stripping).** A winding of tightness `t` is stripped at heat `a` iff
  `t ≤ a` (`atom::open_at`; R7's fill on S¹¹).
- **A4 (heat).** Heat stands at the heat pair's equilibrium
  `a* = u·e/(u·e + g + e − g·e)`, where `e ∈ [0, 1]` is the open share
  (docs/105 §3).
- **A5 (parcel).** A parcel has one state: one place, one spin. Matter that
  moves at two spins is two parcels (§6).
- **A6 (coupling).** The ionized share of every kind moves as one; the bound
  share by its own inertia (§2).
- **A7 (passage).** Matter passing out through matter inside its reach meets
  it. The share caught is `contact·gripped·m_i/(m_i + m_j)`, with
  `contact = ½·crossed` and `gripped = min(s·D, T)/T` (`pulse::cascade`'s
  filter, operators 2026-10-18: "every later, lighter stack passes out
  through the heavy stacks that stay in, each passage a meeting"). Design
  law: one grammar at every tier.
- **A8 (exit).** A parcel whose reach passes the rim leaves for the ocean.

### 7.2 Lemma (the loosest winding)

**L1.** Over the founded table (Z 1–1092 on S¹¹), the least tightness of any
occupied winding is `½`, at Z = 3 (lithium's (2,0)).

*Proof.* A winding (s, l) has tightness `(Z − inside)/s` (R7). The minimum
over the table is a finite exact computation, read from `cfe` by exhaustion
(witness `plasma::tests::the_loosest_winding_is_lithiums`). ∎

### 7.3 Theorems

**T1 (the throat strips nothing).** At the throat, every kind with protons
has `b = ½`, cold or at its heat's equilibrium.

*Proof.* At the throat `g = u = ½` (A2), so A4 gives
`a* = (e/2)/(e/2 + ½ + e/2) = e/(1 + 2e)`. This rises in `e`, so
`a* ≤ 1/3` on `e ∈ [0, 1]`. By L1 every occupied winding has `t ≥ ½ > 1/3 ≥ a*`,
so by A3 none is stripped, and `E(a*) = Q`. Then `b = g·E/Q = g = ½` (§1). ∎

*Consequence:* the open item §4.2 (the throat's heat) changes nothing at the
throat. The cold release is exact, not an approximation.

**T2 (the pool's spin).** At one bound share `b < 1`, the pool's spin per
unit push is `S = w̄/w̄_π`, where `w̄_π` is the mean weight of the matter in
the throat.

*Proof.* By A1 and A6,
`S = Σ n(1 − b)·w̄ / Σ n(1 − b)·w = w̄·N/(N·w̄_π)` with `N = Σ n`. The
factor `(1 − b)` cancels because it is uniform by T1. ∎

**T3 (the split is forced).** A kind's take leaves as two parcels: its
ionized share at `p·S` and its bound share at `p·w̄/w`. One parcel per kind
at the mean `ℓ̄_z = b·p·w̄/w + (1 − b)·p·S` contradicts A5 wherever
`w ≠ w̄_π`.

*Proof.* By A6 the two shares of kind z move at `p·S` and `p·w̄/w`. These
are equal iff `w = w̄/S = w̄_π` (T2). Otherwise the kind's matter has two
spins, so by A5 it is two parcels. The mean `ℓ̄_z` is the holding-weighted
mean of the two (the momentum of A1 kept), the spin of no matter. ∎

*So the earlier read (35 stars) was built on per-kind motes at `ℓ̄_z`, the
spin of no matter. The mixed-parcel read is what A1–A6 give.*

**T4 (light bound shares leave).** The bound share of a kind with
`w < span·p·w̄/(rim − 1)` passes the rim at release and, by A8, leaves for
the ocean, unless something stops it before the rim.

*Proof.* Its reach is `1 + span·p·w̄/w` (`flows::founding`, the release's
reach), which exceeds the rim exactly in that range of `w`. Hydrogen
(`w = 1`, the least) is the first such kind. With `b = ½` (T1), half of each
bounce's hydrogen is bound. This is the read's `flows = ocean` for hydrogen,
`7;51·60⁵` each. ∎

**T5 (the passage at release, corrected 2026-11-13).** At release, a kind's
bound part leaves from the throat outward through its bounce's pool. By A7,
read in the cascade's own direction (the lighter matter passing out
catches), it takes the share
`f = ½·crossed·gripped·m_b/(m_b + m_π)` of the pool's mix. The take rides
out with it, at the holding mean of the two carried spins, `Σ m·c` kept.

*Proof.* The two parcels start at the throat, and the bound one moves
outward past the pool's inner edge (T4's ordering). That is A7's passage, so
by the design law of one grammar at every tier A7 applies, in its own
direction: `pulse::cascade`'s pairs are (i catching, j giving), and i is the
lighter one, passing out. The joining is a holding mean
(`flows::merge`). ∎

*An earlier draft of T5 had the direction reversed (the pool catching the
bound share). The cascade's code reads the other way.*

**T6 (what the release lacks).** The bounce's release (`breath::breathe`)
applies A1, A2, A5, A6 and A8, but not A7. The founding's cascade applies
A7; the bounce does not. So the 4-star read is A1–A6 and A8 without A7, and
the missing law is the passage.

*Proof.* By inspection of the release loop (`held_now`): each parcel is
pushed and paid, with no meeting read between the parcels of one bounce.
`pulse::cascade` reads the meetings. ∎

### 7.4 The answer, and the read

1. The throat's bound share is exactly ½ (T1). No heat law changes it.
2. Splitting each kind into its pooled and bound parcels is forced (T3). The
   35-star read rested on a spin no matter has.
3. Light bound shares leave for the ocean unless something holds them (T4).
4. The bounce now applies A7 (T5, T6: `plasma::passage`, built). Read on one
   2^40 founding to its first echo (bell 112), before → after:

| | split, no passage | split, with passage (A7) |
|---|---|---|
| stars | 4 | 0 |
| hydrogen in the flows (grains) | 7;51·60⁵ | 7;43·60⁵ |
| hydrogen in the ocean | 7;51·60⁵ | 7;58·60⁵ |
| intergalactic medium | 19;59·60⁻² | 24;34·60⁻² |

**T7 (A7 cannot hold the light bound share).** A passage moves the take at the
holding mean of the two carried spins, and the passing part is the outer one.
So the take's spin rises (`c_π → c̄ > c_π`) and the part's falls, while the part
still stands outside the pool. Matter is carried outward, never inward. By A8
light bound matter still leaves, and now carries heavy matter with it.

*Proof.* `c̄ = (m_b·c_b + m_t·c_π)/(m_b + m_t)` lies between `c_π` and `c_b`,
and `c_b > c_π` (T4). ∎

**So the retention is not a passage. It is the exchange of momentum between
the bound and the ionized shares,** a relaxation of the bound part's spin
toward the pool's with both parcels kept (§4.1, the drift): docs/105's
two-pole master equation on spin, with the meeting rate a bell as its rate.
That is the next law to derive. The passage stays, as the bounce's reading
of A7.

## 8. The drift: momentum's degradation on the continuum (2026-11-13)

**Operator 2026-11-13:** "momentum is the constant degradation of velocity on
the continuum into the inversion across the vacuum where the physics changes
drag into velocity".

### 8.1 Axiom

- **A9 (drag and its inversion).** A parcel moving against the medium, on
  the continuum (the open axis), keeps the velocity share `δ = H/(R + H)` of
  its departure from the medium's spin each bell and gives the drag share
  `1 − δ` to the medium. Across the inversion (the vacuum, the closed axis)
  the faces exchange (docs/117 §8.2, `polarity::faces_on`): the drag share
  becomes velocity. The keep is `δ` on the continuum and `1 − δ` on the
  vacuum, the one face law read as momentum.

### 8.2 Theorems

**T8 (the drift keeps momentum).** A bound part `b` (holding `m_b`, carried
`c_b`, keep `k_b`) against the pool `π` (`m_π`, `c_π`) gives

    c_b' = c_π + k_b·(c_b − c_π),     c_π' = c_π + Σ_b m_b(1 − k_b)(c_b − c_π)/m_π

and `Σ m·c` is unchanged.

*Proof.* `Σ_b m_b·c_b' + m_π·c_π'
= Σ_b m_b(c_π + k_b(c_b − c_π)) + m_π·c_π + Σ_b m_b(1 − k_b)(c_b − c_π)
= Σ_b m_b·c_b + m_π·c_π`. ∎

What the drag takes is the medium's: it is booked to the pool, the
medium's charged share (§5).

**T9 (n bells in closed form).** While the medium's spin stands, n bells give
`c_b⁽ⁿ⁾ = c_π + k_bⁿ·(c_b − c_π)`, the attractor's keep composed n times
(docs/132 §1).

*Proof.* Induction on n, the keep composed with itself. ∎

**T10 (the light part is held).** On the continuum the keep is `δ_b`, which
falls as the part stands further out (`δ = 1/(c + 1)` at carried `c`). So a
light bound part, carried furthest (T4), keeps the least of its excess and
is pulled most toward the pool. Its excess after the release's bell is
`(c_b − c_π)/(c_b + 1) < 1` for any `c_b`.

*Proof.* `δ_b·(c_b − c_π) = (c_b − c_π)/(c_b + 1) < 1` for `c_π ≥ 0`. ∎

So a bound part's carried spin after its release's bell is at most
`c_π + 1`, whatever its weight. Whether it passes the rim is then read on the
pool's spin plus less than one, not on `p·w̄/w`.

### 8.3 Build

At the bounce's release, after the parcels are split (§6) and before the
passage (§7), one bell of drift (T8) acts on the continuum, where the
released parcels leave (`vacuum = false`): each bound part relaxes toward the
pool by its keep `δ_b = 1/(c_b + 1)`, and the pool takes the drag. The
passage then reads the drifted spins. Between echoes the drift composes
over the stride (T9, docs/132).

### 8.4 The read

One 2^40 founding to its first echo (bell 112), before (split and passage,
§7.4) → after (with the drift, on the grain):

| | before | after |
|---|---|---|
| intergalactic medium | 24;34·60⁻² of the whole | none |
| hydrogen in the ocean (grains) | 7;58·60⁵ | none |
| hydrogen in the galaxy's flows | 7;43·60⁵ | 50;06·60³ |
| stars | 0 | 1 |
| what fell into the star: hydrogen share by mass | — | 6;09·60⁻¹ (0.10) |
| hydrogen the star core kept | — | none: its cut sent it to the star's disc |

The drift holds every bound part in the galaxy: nothing reaches the ocean.
One binding gathers most of the flows (3;03·60⁶ of holding fell, 2;31·60⁶
into the star core). The first star's binding holds hydrogen, a tenth of its
mass, but the star core's cut (`formation::star_core_cut`) keeps none of it.
That cut is the next law to read. Total hydrogen by nuclei is unchanged
(12;57·60⁻¹ of the universe). The breath's conservation holds.

## 9. Throats and cores: where the flux turns, where energy is made (2026-11-13)

**Operator 2026-11-13:** "we have throats and cores: the star produces
energy on continuums and the cores are the starting/ending of the vacuum to
continuum flux".

- **A10 (throats and star cores).**
  - The universal and galactic cores are **throats**: the ends of the flux,
    where the vacuum turns into the continuum and back through the inversion
    (§5). Their push is the bounce, the tide across that turn, the same
    impulse `p·w̄` for every nucleus.
  - A star core has **no throat** (operators 2026-10-17: "no throat, no
    state", `H = held`). It stands wholly on the continuum and produces
    energy there, by the burn. Its push is that energy on the open frames,
    `F_z = ½·(e_z/ē)·w̄` (`formation::binding_spins`): radiation pressure.
    Bound electrons take it; a stripped kind, with no open frame, is not
    pushed.
- **T11 (a star core keeps its hydrogen iff its heat strips it).** With no
  inversion, a star's matter drifts with keep `δ` (A9, the continuum). A
  bound hydrogen has the largest open share (its lone electron) and is
  pushed hardest. At heat `a ≥ 1` (hydrogen's tightness, A3) it is stripped:
  `F_H = 0`, `b_H = 0`, and it rides the hot plasma's pooled spin.

  *Proof.* §1 and §2 with `E_H(a) = 0` for `a ≥ 1`. ∎
- **Read** (the first star, `.testtmp/plasma/drift`, born bell 110): core
  heat `9;58·60⁻¹ ≈ 0.166` (its ignition share's agitation plus the heat its
  contraction made). That is below hydrogen's tightness 1, so by T11 its
  hydrogen is bound, pushed hardest, and sent to the disc: the read's "kept
  none".
- **Open:** the heat a star core reaches at ignition. In nature a stellar
  core ionizes hydrogen wholly. Here the contraction's heat (docs/124 §5)
  stands at a sixth of hydrogen's grip. The law to derive is the core's heat
  from its contraction against hydrogen's tightness, on the model's own
  rulers.

## 5. The seam with Xylarium (agreed 2026-11-13)

- Xylarium reads the bound share at the inverse place: `b'_z = [r/(r + M)]·E_z(a)/Z`
  (its Z is Netstratum's Q), its convergent face `δ` at `r' = M²/r`, the same
  function as Netstratum's `g`; on the far face only.
- Both strip by the tightness threshold (`open_at`); both releases are cold.
- The medium's resistance offsets the pool's spin only, `S' = S ⊕ (−r_medium)`;
  the take `Σ n(1 − b)w(S − S')` is booked to the medium's charged share, so
  `Σ n·w·ℓ' + take = Σ n·F` exactly.
- The mixed parcel shared at the seam is a recipe `{kind → amount}` under one
  state (reach, phase, span), mass read from the counts.
