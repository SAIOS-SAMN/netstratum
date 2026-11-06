# 121 — The magnetic coupling: the attractor and its inverse, and the shear that makes an orbit

This document derives one magnetic coupling between bodies, signed by their
senses, whose attraction and repulsion are one law and its inverse, and
whose difference in turning across reach is the shear that closes an orbit.
It is stated on the coordinate (docs/120): a body's lanes, read in its
enclosure, never a catalogue of pairs.

**Status:** derived 2026-11-05, proposed for the operator's reading
(operator: "do you have orbits defined by shear according to the magnetic
attractor with its inverse the in repulsion"; "derive the magnetic
coupling"). The audit found the pieces apart (docs/119 C18): an orbit is
the turning law alone; attraction (opposite spins pairing in gas) and
repulsion (open frames pushing, in the firmament and the heart) stand in
separate laws; the shear that reaches the orbit is gravity's and runs one
way. Its conservation and its rest were checked by exact arithmetic. Open
points are in §8.

## 1. What the record already holds

- **Magnetism is the open frames** (operator 2026-10-15, "transduction via
  magnetism"; `pulse::magnetism`): an atom's unpaired electrons are its
  magnetic moments; a manifest's magnetism is its kinds' open frames; holding
  and magnetism together are its 3Z (docs/83, docs/95 §8).
- **Its share** is the open pole's share of the whole, `o = Q/(A + Q)`
  (docs/95 §8, `flows::open`), first degree.
- **Opposite spins pair** in free gas: "full spin magnetism attractors in the
  gases", the antipole of the heart's push (`formation::Stretch::mu`).
- **Open frames repel**: "matter pushing matter apart by its open frames"
  (ruling of 2026-10-19, `flows::repel`); a heart pushes by "the
  electromagnetic repulsion gripping open frames" (`formation`).
- **The shear makes the orbit**: "the kinematic pressure is sheared as an
  attractor on opposing hemispheres to provide the shear on the orbit, which
  turns the 'orthogonal orbit' into an 'orbit'" (ruling of 2026-10-18,
  `flows::shear`).
- **The ring law** reads a hold against a reach as one share,
  `M/(d + M)` (docs/83), first degree: the form every pull in the universe
  takes.
- **Both senses** (docs/78 design law 6): a turning runs either way, and its
  sense is a sign, never a magnitude.

## 2. A body's moment

A body's **moment** is its open frames with its sense:

    μ = σ · m

`m` its magnetism (its kinds' open frames, in its enclosure's holding
units), `σ` its sense (±1). Its open share `o = m/(hold + m)` is how much of
it the field grips.

## 3. The coupling: one law, its sign the senses

Two bodies a and b of one origin, at reach-lane values `S_a`, `S_b`
(docs/120 §1), their gap `g = S_b − S_a`:

    κ = o_a · o_b                        (how much of each the field grips)
    n = M / (|g| + M),  M = m_a + m_b    (how near: the ring law's share)
    c = −σ_a·σ_b · κ · n                 (the pair's closing rate a bell)

- **The sign is the senses**, and that is the whole difference between the
  attractor and its inverse: opposite senses (`σ_a·σ_b = −1`) give `c > 0`,
  the pair closes; matching senses give `c < 0`, the pair opens. One law,
  applied with its inverse; never two codes (docs/119 §A17–A27).
- **First degree throughout:** `κ` is a product of two different shares,
  `n` a share of the ring law's form; nothing is squared, nothing falls as
  an inverse square of a length.
- **Exchanged, never made:** of the pair's closing `c`, each moves by the
  other's holding share toward (or from) the other:

      ΔS_a = sign(g) · c · w_b/(w_a + w_b)
      ΔS_b = −sign(g) · c · w_a/(w_a + w_b)

  so the holding-weighted reach is unchanged, `w_a·ΔS_a + w_b·ΔS_b = 0`, and
  the gap changes by exactly `−c`. Checked: senses opposite, the gap closes
  by `1/12` and the weighted change is 0; senses matching, it opens by
  `1/12`, the weighted change 0.

## 4. The shear: the coupling on the turning lane

On the holonomic lane the same coupling acts on the pair's turning rates
`ω_a`, `ω_b` (their H-lane rates): it moves them toward each other where the
pair attracts (the field locks the pair into one turning) and apart where
it repels.

    Δω_a = c · (ω_b − ω_a) · w_b/(w_a + w_b)
    Δω_b = c · (ω_a − ω_b) · w_a/(w_a + w_b)

conserving the holding-weighted turning (the spin ledger, docs/118 F5).
Across reach, bodies at different `S` turn at different rates (the turning
law, `ω = δ(S)`); the coupling's exchange of turning between neighbours at
different reach is the **shear**. It is signed: attraction draws the outer
inward in turning and the inner outward (they lock), repulsion drives them
apart. The built `flows::shear` is its one-way case (the outer always
gains, docs/119 §A25).

## 5. The orbit: where the coupling and gravity balance

A body's reach is pulled back by gravity's held share (`δ` at its reach)
and moved by the couplings of its neighbours. Its orbit stands where the
two balance: for one neighbouring pair, where the magnetic share equals the
gravitational return `c_g` it works against,

    M / (|g| + M) = c_g   ⇒   |g| = M·(1 − c_g)/c_g

a first-degree solve, never the square the band's rest takes now
(`R·R = 2q`, docs/119 §C3). Checked: `M = 3`, `c_g = ¼` gives `|g| = 9`,
and the share there is `¼` exactly. Inside it the pair is drawn in (or
pushed out, repelling); at it the orbit is closed: the "orthogonal orbit"
the ruling of 2026-10-18 says the shear turns into an orbit.

## 6. Its neighbours are a range, never all pairs

The coupling's nearness `n = M/(|g| + M)` falls with the gap. Below the
spin grain it moves nothing a reading can tell, so a body couples only to
the bodies within the gap where `n` stands at or above the grain:

    |g| ≤ M·(1/grain − 1)

a range on the S lane of the enclosure's `LaneIndex` (docs/120 §3), read by
two bisections, never a test of every pair (docs/119 §A28–A29). Where two
lanes matter (in reach and in step), the range on S is read first and the
sense and step filtered within it.

## 7. What it replaces

| now | becomes |
|---|---|
| `flows::repel` (open frames push, firmament only, its result discarded, a box clamp per axis) | the coupling with matching senses |
| `formation::Stretch::mu` (the gas attractor, in binding only) | the coupling with opposite senses, read in the binding's own enclosure |
| `flows::shear`, the field exchange in `flows::fluid` (outward only) | the coupling on the turning lane, signed |
| the heart's push as "electromagnetic repulsion" | the coupling between the heart and its open frames, with its sense |
| the band's rest `R·R = 2q` | the balance of §5, first degree |

## 8. What is open

1. **The sense σ.** A body's sense is the sign of its turning on the H lane
   (docs/120), or the electrons' phase state R (bound or freed, docs/97 §3),
   which is where magnetism lives. Which one is the moment's sign is not
   derived; the coupling's form does not depend on it.
2. **The units of M against S.** `M` is open frames in holding units and `g`
   a gap on the reach lane; that the ring law's share reads them on one
   ruler is assumed from the ring law's own form (hold against reach) and
   is to be confirmed on the coordinate's units.
3. **Many neighbours.** A body's couplings are summed over its range. Whether
   they add (each pair its own exchange) or the field of the range acts as
   one (its neighbours' moments summed first) is not derived; the first
   conserves pair by pair.
4. **The gravitational return `c_g`** in §5 is gravity's held share at the
   body's reach; its exact form against the coupling, for a body with many
   neighbours, follows from 3.
5. **A read.** Replacing `repel`, `shear`, the field exchange and the band's
   rest changes how the universe moves; it is read before and after on a
   fresh founding, conservation and every identity exact, before it is
   built.

## 9. Rulings of 2026-11-05: dipoles, and the range's field as one

The operator answered §8's first and third points:
- the sense is "both - as dipoles";
- many neighbours act through "the range's field as one".

1. **A body's moment is a dipole on each plane.** Its orientation on plane
   k is its turning there, the component of H, in degrees. Its pole is its
   electrons' phase state (bound or freed, docs/97 §3), so magnetism stays
   where its open frames are. Its strength is its open frames, `m`. A dipole
   is never a bare sign: two dipoles couple by how they stand to each other
   on each plane, their separation in degrees, read as a share on the
   manifold. It is not a product of two signs. Attraction and repulsion
   remain one law: dipoles standing opposed draw together, and aligned
   dipoles push apart.
2. **The range's field acts as one.** The dipoles of the bodies in a body's
   range (a range on S, §6) sum into one field, each weighted by its
   nearness `M/(|g| + M)`. The body's dipole answers that one field, and
   its exchange is against the range's whole holding, which conserves the
   holding-weighted reach and turning across the range.
3. **What remains open.** First, the dipole's coupling as a share of its
   separation on each plane, which must be read on the manifold (the
   turning composed by Cayley, docs/117 §11), never as a flat
   `1 − off/180°`. Second, the electrons' pole in the coordinate's lanes.
   Both are derived before the coupling replaces `repel`, `shear`, the
   field exchange and the band (with its `½`, its halved step and its
   squared rest).

## 10. The coupling derived on the manifold (resolving §9.3)

1. **The pole is the span's sign.** A body's frame state is `(A, Q)`, its
   closed and open frames (docs/95 §8, docs/117 §10.1). Its dipole's pole is
   `σ = sign(A − Q)`, closed against open, the same sign a passage flips
   (docs/117 §12). It is read from the state: the body's holding against
   its open frames. The electrons' phase state (B6, test-only) is not
   needed for it.
2. **Orientation and alignment are the hemisphere law.** The dipole's
   orientation on plane k is its turning there, `H`'s component, in
   degrees. Two dipoles are aligned on plane k where their separation is
   under 90°, opposed past 90°, and uncoupled at 90°: the hemisphere law
   (ruling of 2026-10-16), a sign `h_k ∈ {+1, 0, −1}`. Their coupling takes
   no magnitude from a flat `1 − off/180°` or a cosine.
3. **The range's field as one** (§9.2): for body `i`,
   `F_i,k = Σ_j w_j·σ_j·o_j·n_ij·h_ij,k / Σ_j w_j`, against the range's
   whole holding, its neighbours in its range on `S` (§6), with `o` the open
   share and `n = M/(|g| + M)` the ring law's nearness. Each term stands in
   `[−1, 1]`, so `|F| ≤ 1`, and with `o ≤ 1` and `|x̄ ⊖ x| < 1` the pull keeps
   `|y| < 1`: the flow law then holds the rate inside the luminal bound by
   construction. (A first draft summed the field without the holding; its
   witness carried a rate past the bound.) The pair `σ_i·F_i,k` is the field's pull on `i` on plane k,
   attracting where negative and repelling where positive, by one law.
4. **Bending: rates compose by the flow law.** A rate on plane k is a share
   of the luminal bound (`SUBSTRATE_C`, "never faster than a turn a
   bell"), `x ∈ (−1, 1)`. The field moves it by composition,
   `x ⊕ y = (x + y)/(1 + x·y)` (LAW-DILATION's flow law, built), where
   `y = −σ_i·F_i,k·o_i·(x̄ ⊖ x)` is the pull toward the range's
   holding-weighted rate `x̄`, and `⊖` is the flow law's inverse. The
   denominator is the curvature: no step is halved, and no rate passes the
   bound.
5. **The rest is first degree:** `|g| = M(1 − c_g)/c_g` (§5). It replaces
   the band's `½`, its halved step and its squared rest `R·R = 2q`
   (register K9).
6. **Witnesses before it is built:**
   - the flow-law composition never passes ±1;
   - aligned dipoles push their rates apart and opposed ones lock them;
   - the range's holding-weighted rate is kept where the field exchanges
     between neighbours;
   - a body alone in its range is untouched.

## 11. Shear is the degree of modulation at every level (operator, 2026-11-05)

The operator ruled: "shear needs to be the degree of modulation at any
dimensional level (phase, quantum, state)".

- **Shear is the degree of modulation:** the difference, in degrees a bell,
  between a body's rate and its range's field's rate, composed by the flow
  law (§10.4). It is not a spin exchange between neighbouring rings
  (`flows::shear`'s reading, one level, one way).
- **The same law at every level of the tower** (docs/117):
  - **phase:** the turning on each plane, `H`'s components;
  - **quantum:** the riding state's modulation on `S³ ⊂ S¹¹`, the change of
    kind on `G`;
  - **state:** the span's dilation on `S`, and the passages on `T`.
- **Each level's rate is a share of its own luminal bound,** and each change
  is booked on that level's own ledger: spin `Σ m·ℓ` at the phase level,
  the span at the state level, the hue moves at the quantum level.
  `coupling::pulled` is that one law; the level is which lane it reads.
- **What the engine holds today:** the phase level, as spins and phases, and
  the state level, as reach. `G` holds no rate yet (docs/120 §9 item 1), so
  the quantum level is read where the code gains it.

## 12. The state level: the reach's rest and its boost (replacing the band)

The band (docs/104 §6: the push `q·u` against the pull back `½·g·R·H`, its
rest `R·R = 2q`, `flows::toward_rest`'s halved step) is restated on the
coupling at the state level, where the rate is the reach's dilation (`S`, a
boost).

1. **The two shares:**
   - Gravity's return at reach `R` is the held share `δ = H/(R + H)`.
   - The heart's push is the coupling between the mote's open frames and the
     core's (ruling: "the electromagnetic repulsion gripping open frames"),
     `k·n`, with `k = o_m·o_c` the two open shares and `n = M/(R + M)` the ring
     law's nearness of their moments `M`.
2. **The rest is where they balance:** `k·M/(R + M) = H/(R + H)`, first degree
   in `R`:

       R* = H·M·(1 − k)/(k·M − H)

   A rest exists where the repulsion can outweigh the hold (`k·M > H`). It
   differs by kind through the kind's open share and its moment, with no
   square and no `½`.
3. **Motion toward the rest is a boost:**
   - The net pull `y = k·n − δ` lies in `(−1, 1)`.
   - The reach's rate composes by the flow law, `x′ = x ⊕ y`, and the reach
     dilates by LAW-DILATION, `R′ = R·(1 + x′)/(1 − x′)`.
   - At the rest `y = 0`. A boost never carries the reach through zero or past
     the horizon, so no step is halved.
4. **The vacuum is the same law with reach and hold exchanged,** `(A, Q) =
   (H, R)` (docs/117 §12.6).
5. **What it replaces:** the band's value and rest (`flows::band`,
   `toward_rest`) and `through` at the vacuum's band. The core's release
   follows: its parcel is carried by the boost from the throat, never
   mirrored.
6. **Witnesses before it is built:**
   - the rest solves the balance exactly;
   - the boost composes by the flow law and stays inside the horizon;
   - a mote at its rest stays;
   - inside the rest it is driven out, past it drawn in;
   - the vacuum's rest is the continuum's with reach and hold exchanged.
