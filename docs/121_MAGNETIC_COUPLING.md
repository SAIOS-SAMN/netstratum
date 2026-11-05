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
