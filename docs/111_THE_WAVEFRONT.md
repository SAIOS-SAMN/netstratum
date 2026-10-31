# 111 — The wavefront: one expanding sphere, echoed at surfaces

This document replaces the relayed ring with a single wavefront and sets the
order in which the universe assembles behind it.

- Each bounce of the universal core emits one geodesic sphere that grows from
  S^0 at the core towards S^360.
- Bodies the front passes receive it. A body echoes it only to the degree it
  is a surface.
- The firmament is the first surface, so the echo stays contained until
  collections of its matter, bonded inside it, escape.
- No galaxy forms before the universal core is established. The first is the
  zero galaxy that surrounds the core.

**Status:** scoped; not built. Reviewed with xylarium-kernel, which holds the
same law. The two kernels compare readings before either builds (§6).

**Prerequisites:** docs/78 (S^360, the breath), docs/83 (the ring-density
law), docs/101 §2 (the firmament), docs/106 (the substrate), docs/110 (the
wavefront, the equator, shells and cells).

## 1. What the code holds now

| Part | Code | Reading |
|---|---|---|
| The law's statement | `ring.rs` header | "Every bell … holds each ring that reaches it for its own period, then passes it on as its echo." A relay. |
| The universal period | `pulse::cascade`, `Universe::t_u` | `T_u = 2(R_out + H)`, a ring out to the outermost galaxy shell and back. |
| The founding | `pulse::deploy` | Every bead of the cascade's shells that holds its spread stands as a galaxy at `t = 0`. Only beads that cannot hold it join the home. |
| Arrivals at a galaxy | `breath::breathe` | A fixed schedule `k·T_u + ⌊R_i⌋`, read from each galaxy's radius once per call. |
| The firmament | `pulse::firmament_of`, `flows::repel` | The home's motes that its throat holds wholly. Each echo they repel one another (order only). They never bond. |
| Escape | `flows::exits` | A single mote crosses one tier up by its jet (`ℓ_⊥/H ≥ H`) or radially (`R ≥ H`). |
| A star's bell | `breath` (`rang`) | It rings on the star's own whole turns. It is not driven by any front. |

## 2. The front

Let the universal core bounce at bell `b_0`. Its front at light-time `t` is
the geodesic sphere `W(t) = {x ∈ S^360 : d(core, x) = t}`.

1. **Delay through the held medium.** The front meets a body at reach `r`
   when
   
       t = a(r) = r + H(<r)

   with `r` in throat radii of the universal core and `H` in units of the
   core's state (1). `a` is counted in light-crossings of the throat radius,
   and the equator `δ = ½` stands at `r = H = 1`. With no holding
   (`H(<r) → 0`) the speed is constant, which is Xylarium's present
   `(r/rim)·bell/2`.
2. **Arrivals are meetings, read live.** A body moves while the front
   travels, so the meeting is the first bell `b` at which
   
       b − b_0 ≥ a(R(b))

   with `R(b)` and `H(<R(b))` the body's reach and its enclosed holding at
   `b` (`pulse::orbit_galaxies` keeps them live). There is no schedule
   table. A body whose reach grows as fast as the front never meets it.
3. **Reception.** A body the front meets has its clock advanced and turns by
   its deficit `δ = H/(r + H)`, as now.
4. **Surfaces.** Every body the front meets is a surface to the degree its
   share `σ` says (§3). At a surface the front parts. The reflected share
   `σ` converges back on the body's own source and makes the body's bounce
   (its echo). The transmitted share `1 − σ` travels on. Shares multiply
   along the path, so the front's strength at a body is
   
       S = Π (1 − σ_j)

   taken over the surfaces it has crossed on the way. That product is
   computed within the body's own cell on each plane (docs/110 §2). A surface
   shadows only its own cell of the shell, never the whole shell.

## 3. Which bodies are surfaces

| Surface | σ | Reading |
|---|---|---|
| The firmament | `1 − τ` | It transmits the escaped share `τ` (§4). |
| A core's throat | 1 | A galaxy's core bounces into its own enclosure and passes nothing outward. |
| The S^360 rim | 1 | It reflects wholly. |
| A star | `1 − (1 − ρ_s)(1 − α_s)` | Its **density** and its **activity** each count. |

For a star:
- **Density** is the share of its matter its throat holds, `ρ_s = H_kept/H_total`.
  The `breath` code already reads this as `h_s`.
- **Activity** is the share of its holding that its burn transduces into
  light this echo, `α_s = light made / H_kept`, from the light ledger
  (`breath`, `light.made` per star). In Xylarium this is `burn_per_bell`.

The union form is a share in `[0, 1]` and needs no table:
- a dense remnant is a surface by its density;
- a burning star is a surface by its activity;
- a cold, diffuse body is barely one at all.

A star's echo is its own bounce. Its bell rings when a front meets it, with
strength `σ_s·S`, and the bell rings independently of the star's spin count.
Stellar remnants are cores by the same reading (`ρ_s → 1`). No separate rule
is needed for them.

## 4. The firmament: collections, bonded by perturbation

The firmament's matter moves as **collections**, not as single motes.

1. **The collective.** The firmament's motes are one quantum collective. Each
   contained bounce does two things to it:
   - torques it, by the existing firmament torque;
   - orders it, by `flows::repel`.
   
   A mote's **perturbation** on plane `k` this bounce is the displacement the
   bounce gave it, `Δφ_k`, read once all the moves are known (as `repel`
   reads them).
2. **Bonding.** Two motes bond where all three hold:
   - they stand on one held hemisphere, `step_with ≥ ½` (docs/99 §2.2);
   - the bounce moved them together: the tilt-weighted in-step share of
     their perturbations is `C_Δ = Σ_k t_k · in_step(Δφ_k^i, Δφ_k^j) ≥ ½`;
   - both carry open frames (`chem::unpaired > 0`).
   
   A **collection** is a connected set of bonds. Being moved together is the
   bond. It is read from the bounce and never assigned.
3. **A collection moves as one.** It takes:
   - one specific spin, the holding-weighted mean `ℓ̄ = Σ m·ℓ / Σ m`, so
     `Σ m·ℓ` is unchanged;
   - one phase displacement per plane, the members' holding-weighted mean.
   
   Its reach is `R = ℓ̄/H` about the enclosed holding.
4. **Escape is a collection's crossing.** A collection leaves whole when its
   jet or its radial crossing (`flows::exits`, `R ≥ H`) holds for the
   collection. The order of exits is read: lighter collections reach further,
   and bonding mixes kinds.
5. **The transmission.** The firmament passes the share of its founded
   holding that has left:

       τ = left / founded,   founded = held + left

   `τ` is 0 until the first collection leaves. It tends to the share that
   ever leaves, never 1. It is one amplitude, and the matter that leaves
   lands per kind.

## 5. The order of assembly

1. **Founding.** The cascade (`pulse::cascade`) stands the firmament inside
   the throat, as now. Every shell's matter outside it belongs to the **zero
   galaxy**, the home (`pulse::is_home`), which surrounds the universal core.
   No other galaxy stands at the founding. This is the case `deploy` already
   takes for a bead that cannot hold its spread; under this law it holds for
   every bead.
2. **Contained.** While `τ = 0`, the bounce runs core → firmament → core, so
   its period is

       T_f = 2 · a(r_f)

   with `r_f` the firmament's outer held spread (the greatest `R(1 + p)` in
   `firmament_of`). Each bounce perturbs the collective (§4). The zero
   galaxy's matter outside the firmament receives no front. It breathes no
   stars.
3. **Established.** The universal core is **established** at the first
   collection's exit (`τ > 0`). The transmitted front then reaches:
   - the zero galaxy's matter, at strength `τ`, which begins to breathe;
   - the rim, from which it returns. The full period is `T_u = 2 · a(R_rim)`.
4. **Galaxies form.** A galaxy forms where escaped collections and the zero
   galaxy's outer medium hold their own reach about the universal core. This
   is the pinch test `deploy` uses now (`d = R·H_b/(H_b + H_inside)`, spread
   within `d`), read live in the ocean's frame and not at the founding. Its
   core then becomes a surface (σ = 1), and the front it receives is its
   bounce.

## 6. Build order

- **W1. Statement.** Rewrite the `ring.rs` header to §2 and §3. Add
  `ring::arrival(r, held) = r + held` and the star surface share
  `ring::surface(density, activity)`. No behaviour change.
- **W2. Read the firmament's collective.** Add perturbations and bonds (§4.1,
  §4.2) as a reading only, and report the collections in `breathe`. Witness:
  - an evenly ordered collective that the bounce moves rigidly is one
    collection;
  - motes moved apart are not bonded.
- **W3. Collections move and leave.** Add §4.3–§4.5, plus `founded` as a new
  tag on the universe. Witnesses:
  - `held + left = founded`;
  - `Σ m·ℓ + reservoir` is exact.
- **W4. The founding and containment.** Add §5.1–§5.3 with the period `T_f`,
  and arrivals as live meetings (§2.2) at strength `S`. Witness: a sealed
  firmament breathes nothing outside itself and conserves.
- **W5. Galaxy formation** (§5.4) and **star surfaces** (§3), including each
  cell's shadow product.
- **Read before W4.** A bounded read of a fresh 2^40 universe with W2–W3 in
  place reports:
  - the bell of the first collection's exit, or none within N bells;
  - its composition;
  - `τ` over time.
  
  If nothing leaves within a reachable horizon, that is a finding about the
  collective's torque. It is not a reason to set `τ`. Xylarium reports its
  first-exit order on its own founding (`circuit::onset_echoes`), and the
  two lists are compared.

## 7. Open

1. **The rim's inbound front.** After establishment, does the rim's
   reflection meet each body a second time, at `T_u − a(R)`, as a second
   arrival per period?
2. **Pairing.** Should bonded open frames pair, transducing magnetism into
   holding (3Z conserved), or is the bond a joint motion only? This document
   builds the joint motion only.
3. **Xylarium's side.** Xylarium has every galaxy hear the core from
   `t = 0`, galaxies standing at the founding, and escape by pole state per
   kind. Adopting §4 and §5 is a law change there, for its operator.
