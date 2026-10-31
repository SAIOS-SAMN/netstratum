# 104 — The inverse duals: heating and cooling, push and pull, twist and release

This document maps each dual pair of the inverse universe (heating and
cooling, push and pull, twist and release) onto the inverse nucleus and the
inverse faces, and requires every pair to conserve across the two faces, so
that heating is balanced by a counterweight. Because the nucleus is
inverted, the state beneath it is inverted with it, and each formula is read
on the inverse face.

**Status:** building.

**Prerequisites:** docs/97 (the inverse nucleus), docs/98 (the faces),
docs/100 (twist, torsion), docs/103 (the layers; §5 the whole frame
inverted, its enclosure read by share from the outside in).

## 0. A single map

Netstratum is Xylarium's universe inverted at both ends:

- **Its nucleus (built, docs/97):** the open lone end is the proton (Q), the
  fibred end the neutron (Z); the electrons (R) follow the protons.
- **Its gravity (decided, docs/103 §5):** the frame inverted whole; the hold at
  a place counts what stands beyond it (by share, from the outside in).

The inversion is one first-degree map on every face-reading:

    I: δ ↦ 1 − δ     (the share)
       face ↦ the other face, pole ↦ the other pole (the Möbius pass)
       the enclosure's order: deeper ↦ farther out

Forward, gravity's held share is the deficit δ and the push reads the
unlocked face 1 − δ. Inverted, the held share is `1 − δ` (gravity holds from
outside in) and the push reads `δ`. Every reading that names a face is read
through I; every reading that does not (turning on its own leg, the luminal
bound, phases, in-step) is unchanged. The quantum state beneath follows the
same map: its held hemisphere is the face inverse gravity holds.

## 1. The dual pairs

Notation: `δ = H/(R + H)` the forward deficit at a reach; `g = 1 − δ` the
inverse held share (inverse gravity); `u = δ` the inverse unlocked share;
`e` a kind's open share (its open frames over its weight).

| pair | forward (Xylarium) | inverse (Netstratum) | in code now | change |
|---|---|---|---|---|
| **pull** (gravity's hold) | `δ` | `g = 1 − δ` | `Enclosed::reach`, heart cut `H/(R + H) ≥ p` | the enclosure by share from the outside in (docs/103 §5); the cut held where `g ≥ p` |
| **push** (the bounce's carriage) | on the unlocked face `1 − δ` | on the inverse unlocked face `u = δ` | release `s = ½·c/(c + base)` (`breath`) | `s = p·c/(c + base)`, `p = S/(S + T)` the core's pair share (§1.2) |
| **heating** (the face not locked heats) | `½(1 − δ)` | `u·e` (§1.2) | the light warms bodies by `1 − δ` (`breath::shine`); torque heat `p·δ·(1 − |k|)` | warmth by `u·e`; torque heat by the face pushed, `p·u·(1 − |k|)` |
| **cooling** (heat leaves) | light `e` and the vacuum's pull back `δ`, keep `(1 − e)(1 − δ)` a ring | light `e` and the inverse pull back `g`, keep `(1 − e)(1 − g) = (1 − e)·δ` | keep `(1 − e)(1 − δ)` (`flows::spin_torque`) | keep `(1 − e)·u`, and the counterweight (§2) |
| **twist** (winding per turn) | the core's deficit at its ring, `½·t_k` | the same: the fixed point where it holds (§1.2) | `quantum::twist`, a body through its own planes | stands |
| **release** (untwisting; the tide letting go) | by the unlocked face | by the inverse unlocked face `u` | torsion relaxes by the attractor `turned/(d + turned)`, `d = 1 − δ` | `d = u` |
| **bound / free** (the electrons) | the held share counted, `h = (F_e + L_e·in_step(p, 0))/Q` | the same count against the face inverse gravity holds, `h = (F_e + L_e·in_step(p, ½))/Q` | `nuclides::held` `(1 + R)/2` (a halving) | the count (§1.1): no halving |
| **grip / part** (the fibres) | grip `κ·h` | grip `κ·h` with the inverse `h` | `nuclides::grip` | follows `h` |
| **capture / release** (nucleons) | survival `κ + (1 − κ)·h` | the same with the inverse `h` | `nuclides::survival` | follows `h` |

Fixed points: at the ring `δ = ½`, so `g = u = ½` and every pair balances
there (both faces equal).

### 1.1 The held share is a count, never a halving

`(1 ± R)/2` maps R's range onto a share by halving: a normalisation, not a
measure. The held share is measured from the state itself. A nuclide's Q
electrons fill S^11 by the nucleus's own convergence: `F_e = Z(Q)` fibres
(two counter-turning ends each, so exactly one end of each stands on the
held hemisphere whatever the phase) and `L_e = Q(Q) − Z(Q)` lone ends (each
on one hemisphere: its share on the held one the S¹ linear measure,
`in_step`, at the pulse phase p against the held face). Over all
`Q = 2F_e + L_e` ends:

    h(w, p) = (F_e + L_e·in_step(p, φ_held)) / Q

forward `φ_held = 0` (the face in step with the echo), inverse `φ_held = ½`
(the face inverse gravity holds). Its mean over a turn is `(F_e + L_e/2)/Q`,
½ as an average, never as a factor. Hydrogen (F = 0, L = 1) runs from 1 to 0
across a turn where the halving read a flat ½.

**Consequence, read before build:** the founding's seal window reads
deuterium's grip at the source's phase; with the count it no longer reads ¾,
and the neutron share, the seeds and the densest element move with it; the
founding's phase (`frac(p) = ½`) stands on the hemisphere's boundary, where
`in_step` reads 0 forward and 1 inverted. The founding is re-derived on the
count, never carried over.

### 1.2 Every share is a pair share of read quantities

½ is not a constant: it is the inversion's fixed point (δ = 1 − δ only at
δ = ½: the throat, R = H, both faces equal). Every true ½ is that reading
taken where it holds. Every share is the pair share

    share = a / (a + b)

of the two quantities actually present; ½ is its value only where `a = b`.
A ½ written as a factor anywhere else carries the throat's value to a place
that is not the throat: a halving, a static.

| law | the ½ assumed | the pair, read |
|---|---|---|
| the electrons' held share | half their ends held | the count, `(F_e + L_e·in_step(p, φ_held))/Q` (§1.1) |
| a core's push ("regenerated ½") | its state equal to what its throat holds (true at the bounce only) | `p = S/(S + T)`: its state S against what its throat still holds T; ½ at the founding, rising as the throat empties |
| the release `s = ½·c/(c + base)` | that push | `s = p·c/(c + base)` |
| a body's push `½·Q/(A + Q)` | half its open share | `Q/(A + Q)`, its open frames over its whole (open in both frames: the inverse nucleus already makes the protons the open ends) |
| the heating face `½(1 − δ)` | half the unlocked face reaches the body | `u·e`: the inverse unlocked share times the body's own open share (the frames that take heat); heating is the cooling's light channel run backward |
| the core's twist at its own ring | — | `½·t_k` stands: the fixed point read where it holds (a core winds at its ring, δ = ½); a body takes it through its own planes (`quantum::twist`) |
| the luminal bound (share > ½), the founding's phase | — | Stand: the fixed point read where it holds |

## 2. The counterweight: every pair conserves across the faces

A pair is one transfer read from its two faces (I maps each side onto the
other), so nothing leaves the books:

- **Heat:** what a mote's cooling gives up, `a·(1 − keep)`, splits into its
  two channels. The light channel (`e`) is made light, booked on the light
  ledger (received or drained by the existing law). The pull channel (the
  rest) is deposited as heat on the other face: the motes standing there take
  it by their holding at their own share. Cooling here is heating there.
  Witness: the heat ledger closes each echo (heat before + heat in = heat after
  + light out).
- **Push and pull:** the bounce's push given on the inverse unlocked face is the
  hold inverse gravity takes back on the held face: the spin ledger already
  books it (the torque's `given` against the core's reservoir); unchanged.
- **Twist and release:** the winding a core gives (`g·t_k` a ring) is what its
  bodies' torsion holds; their release (the attractor's loosening) returns it
  to the core's own turning, booked on the spin ledger (the tilt's change is
  spin moved between planes, `Σ m·ℓ_k` exact).
- **Bound and free:** the electrons freed on one hemisphere are bound on the
  other; the light of the crossing (docs/100 §9.1) is the counterweight's
  line.

## 3. What this changes, read

- **Heat runs the other way.** Matter is heated by the inverse unlocked face,
  `u = δ`:
  deep matter (large δ) is pushed and heated most, far matter cooled; the
  cooling's pull channel deposits heat on the other face. With the luminal
  bound keeping most matter on the vacuum face, the two faces' heats meet at
  the ring (docs/103 §4's open line), and the thermal pressure is a conserved
  flow, never a residue.
- **The release slows where the throat is deep.** `s = ½·base/(c + base)`:
  kinds carried far (large c) are released less a bounce; heavy kinds (small c)
  released more, then held from outside in.
- **The electrons are free in step.** A body in step with its echo has its
  lone electrons on the face inverse gravity does not hold (the count, §1.1): its captures survive less and
  its fibres grip less in step, more off step. The founding is re-derived on
  the count (§1.1): its phase stands on the hemisphere's boundary.
- **Twist grows outward.** A core winds its far bodies by `g` (near 1 far out)
  and its near ones less: the outer disc is wound, the inner relaxed.

## 4. Build order (each with its witness)

1. **The map in one place** (`flows::face`: `held(δ) = 1 − δ`, `unlocked(δ) =
   δ`), every face-reading routed through it. Witness: at the ring every pair
   reads ½.
2. **Heating, cooling and the counterweight** (§1, §2) on a heat ledger.
   Witness: the ledger closes each echo exactly.
3. **Push and release** on the inverse unlocked face (the bounce's release, the
   torque's band). Witness: nucleons and spin conserved per bounce.
4. **Twist and its release** by the inverse held share at the body's reach. Witness:
   the twist at the ring ½ a turn a ring on both faces; the spin ledger exact.
5. **The quantum state counted and inverted** (`nuclides::held` the count of
   §1.1 against the held face ½). Witness: the count's mean over a turn
   `(F_e + L_e/2)/Q`; the founding re-derived on it (its seal window, neutron
   share and densest element read anew).
6. **The enclosure from the outside in** (docs/103 §5, L5).
7. **Read:** re-found, breathe to bell 1700 and past 7000.

## 5. For confirmation

The whole table rests on one statement: under inverse gravity the held share
is `1 − δ` and the unlocked share is `δ`, and the quantum state's held
share is the count of §1.1 against the face inverse gravity holds. Confirmed,
steps 1–7 follow by the map alone.

## 6. The band of a core's held collective

The band is a core's push on the kinds it holds and its pull back on them,
in spin a bell (`flows::spin_torque`). It sets whether a core's held matter
stands as a container (the firmament) or drains.

### 6.1 Read at the founding

A fresh 2^40 universe:

- The firmament is 48 kinds of weight 160–208, at reaches 0.40–0.52 throat
  radii.
- The live torque passes no band (`heart_push = None`; the galaxies' band is
  held back as static). The bounce's in-step push acts, and nothing pulls the
  held matter back.
- The band as written is forward on both faces: push `q = ½·w̄/w` with no face
  share, and pull `½·δ·R·H`, with `w̄` the mean weight of all the home's
  matter (6.27, the zero galaxy's light gas).
- Its terms are therefore not aligned with the core's own bonds. The push is
  scaled by matter the core does not hold, and the pull is read on the
  forward face.
- Read: the wholly held firmament falls from 48 to 17 to 0 motes over three
  bounces.

### 6.2 The band on the inverse faces

The pair is read through `I` (§0). With `(g, u) = (1 − δ, δ)` at a mote's
reach `R` about the hold `H`:

    band(R) = q·u − ½·g·ℓ,     ℓ = R·H,     q = ½·w̄_c / w

- **Push:** the bounce carries a kind on the inverse unlocked face `u`, in
  inverse proportion to its weight `w`.
- **Pull:** the core holds it back on the inverse held face `g`, by its spin
  `ℓ`.
- **Scale:** `w̄_c` is the mean weight of the collective the core holds: the
  totality of the weighted bonds inside the core over its grains,

      w̄_c = Σ a·w·h / Σ a·h

  with `a` a mote's grains, `w` its weight (its bonds, `3Z − unpaired`) and
  `h` its held share (`flows::held`).

### 6.3 The rest point

`band(R) = 0` gives `q·H/(R + H) = ½·R/(R + H)·R·H`. The hold cancels:

    q = ½·R·R     ⇔     R* · R* = w̄_c / w

The rest point does not depend on the order in which the enclosure is
counted (L5). It is stable:
- inside `R*` the push wins (`u → 1` as `R → 0`);
- outside it the pull wins (`g → 1` and `ℓ` grows).

The equator `R = H = 1` separates the two outcomes:
- a kind heavier than the collective's mean rests inside the throat
  (`R* < 1`), held;
- a kind lighter than the mean rests beyond it and leaves by the radial
  exit (`R ≥ H`, `flows::exits`).

Escape is lightest first. Each departure raises `w̄_c`, which brings the next
kinds below the mean in turn. The collective tends to its heaviest kind
alone, which rests on the throat (`w = w̄_c`, `R* = 1`): the sealed residue.
`τ` then tends to the unsealed share, never 1 (docs/111 §4.5).

### 6.4 Build

1. `flows::face(δ) = (1 − δ, δ)`.
2. The band in `spin_torque` on the inverse faces (§6.2).
3. The universal core's torque passes the band with `w̄_c` read from its
   collective (`pulse::collective_mean`). The galaxies' band stays held back
   until it is read on the same law.
4. Read: the firmament's wholly held count and `τ` over bounces. Expected:
   - kinds heavier than the mean are held;
   - kinds lighter than it leave, lightest first.
