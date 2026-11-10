# 124 — States and phases: where each form of energy lives

This document places each form of energy on one level of the coordinate
(docs/120), names each by the term nature already uses, and from that
derives the star core's heat, which until now was a constant. It answers
the open question "where heat lives" (docs/120 §9 item 5, docs/118 Part I
item 5) and is the semantic floor beneath docs/122 (heat and light).

**Status:** derived, not built (2026-11-10). The operator said on
2026-11-10: "heat needs to be brought up a meta layer: isn't this
radiation?" and, on the alignment below, "we're starting to split
dimensions by states and phases — let's align on this and build our
semantics in accordance". The naming rule is register H12: nature's term
where one exists, a new name only where none does.

**Prerequisites:** docs/120 (the coordinate and its lanes), docs/121 §11
(the three levels), docs/106 (the substrate's energy book), docs/122 (heat
and light), docs/97 (the windings and their tightness).

## 0. What the code holds today

- **Heat is a share with no lane:** `flows::Mote::agitation`, "the share of
  its turning absorbed and not re-locked" (GLOSSARY), and `Body::heat`.
  docs/120 §9 leaves open whether heat is a lane or a reading.
- **The star core's heat is two constants:** ignition cuts the binding at
  heat 1 (`formation.rs`, `ionizing = 1`, the ruling of 2026-10-18), and the
  outburst re-cuts the burning star core at heat 0 (`breath.rs`), which
  throws its hydrogen out before the first burn (issue #119). The branch
  `hot-heart` set the re-cut to 1 as well; this document retires both.
- **Ionization is already a comparison of two levels:**
  `cfe::almanac::atom::open_at(z, heat)` strips every winding whose
  tightness is at or below the heat. Only the heat's source is missing.

## 1. The three levels, by nature's names

docs/121 §11 names three levels of the tower; each reads its own lanes of
the coordinate.

| level | lanes | what it is in nature | its quantities |
|---|---|---|---|
| **phase** | `H` (with `K`, the closed turns) | the phase of a rotation or an orbit | turning, spin, orbital phase |
| **quantum** | `G` | the quantum state of matter: its kind, its windings (energy levels), how ionized it is | kind, tightness, bonds |
| **state** | `S`, `T` | the thermodynamic state: extent (volume), pressure, the potential where a body stands | reach, span, pressure, potential |

Lanes are never summed into one another (docs/120 §1); an energy is booked
on the level that holds it, and an exchange between levels is a law with
its own witness.

## 2. The forms of energy, one per level

| form | level | nature's term | the book (docs/106, 122) |
|---|---|---|---|
| **motion** | phase, in step | kinetic and rotational energy, bulk motion | `m·ℓ` |
| **heat** | phase, out of step | thermal energy | `a·m` |
| **bond** | quantum | binding energy: nuclear and chemical | `B` |
| **potential** | state | gravitational potential energy | `m·Φ` |
| **light** | none: in transit | radiation | made, received, drained |

- **Light is energy between bodies, on no body's lane.** It is made by
  **emission** where heat leaves a body by its unlocked face (docs/122 law
  3), taken by **absorption** where it falls on a body (that body's
  **irradiance**, law 4), and the rest is **drained** to the **sink**.
- **Darkness is not a quantity.** It is the absence of irradiance: a body
  that receives nothing back cools by its emission alone. In the model
  that is the sink, the vacuum's face (docs/122 §4).

## 3. Heat is motion out of phase

Nature's thermal energy is the motion of a body's parts that does not move
with the body: its velocities' spread about their mean. The mean is bulk
motion and does work; the spread is heat. The record already reads heat
this way (agitation: turning "absorbed and not re-locked"); this section
states it as the law and so answers docs/120 §9 item 5: **heat is a reading
of the coordinate, the spread of a body's turning about its own step,
never a lane of its own.**

- **In step:** a body's turning `ℓ̄`, the holding-weighted mean of its
  parts' turnings on `H`, is its motion.
- **Out of step:** its heat is the holding-weighted spread about that mean,
  first degree, `a = Σ m_i·|ℓ_i − ℓ̄| / Σ m_i` (the record's rulings refuse
  squares, so the spread is the mean absolute deviation, not nature's
  second-degree variance; both vanish together and order bodies alike).
- **Friction is decoherence:** a share of motion that leaves step becomes
  heat (docs/122 law 2). Motion in step is booked as `Σ m·ℓ`, which friction
  never changes; what it spends is the relative motion, the spread about
  the mean: two bodies meeting lose `γ·S` of `S = Σ m_k·|ℓ_k − ℓ̄|` to heat
  (EQUATIONS E56), and a merged body's heat is its members' whole spread. The reverse, heat
  ordered into step, is work done by a pressure contrast (docs/106 §3).
- **Temperature** is heat per unit of holding, the share `a`. A body of one
  part has no spread and so no heat of its own; it carries the heat it was
  given until it emits it.

## 4. States of matter: a reading across two levels

A **state of matter** compares a body's heat (phase level) with the bonds
of its kind (quantum level). Nothing new is stored.

| state | where | its transition |
|---|---|---|
| **molecular** | every chemical grip (`chem::closing`) is tighter than the heat | **dissociation** above it (`formation::dissociate`) |
| **atomic gas** | molecules broken; every electron winding tighter than the heat | **ionization** above it |
| **plasma** | windings at or below the heat stripped (`open_at`) | **recombination** as the heat falls |

The reverse transitions (**association**, **recombination**) release their
bond energy into heat (docs/122 law 1). The word "phase" is kept for a
position on a turn; a state of matter is never called a phase.

## 5. The star core's heat, derived

A real star core is plasma because contraction heated it before any fusion
ran (the Kelvin–Helmholtz mechanism): the binding falls inward, the
potential it gives up becomes motion, and the motion that is not kept in
step becomes heat. In the model:

1. **The fall's potential is its members' motion.** The state level carries
   no radial law (operator 2026-11-06): a member's reach, and so its
   potential, is a reading of its spin in its enclosure, `R = ℓ/H(<ℓ)`. In
   the binding's own frame a member turns `ℓ′ = d·H_c(<d)` (its phase place
   and the hold of every member more in step, docs/99 §2.2).
2. **What stays in step is the star's spin.** The star's own spin is their
   holding-weighted mean, `ℓ̄′`; `Σ w·ℓ′` is kept.
3. **The rest is heat.** The members' motion out of step, their spread about
   that mean, first degree, `S′ = Σ w·|ℓ′ − ℓ̄′|`, ends as they join: per unit
   of what fell it is the contraction's heat. It is the same join law as two
   stars sticking (`formation::spread`, EQUATIONS E56), one law at every
   tier; a separate potential term would count it twice. The star core's
   heat is it plus the heat the members carried in (`formation::core_heat`).
4. **The heat stands where its sources meet its emission** (docs/122 law
   5): sources contraction, the burn's bond energy and absorbed light; the
   loss is emission by the face, ½ a ring at the system's throat
   (`formation::star_glow`).
5. **Its state of matter is read, never set:** the cut at ignition, the
   re-cut at each outburst and the wind read `open_at(z, a)` at the star
   core's own heat. A star core whose heat strips hydrogen keeps it to burn;
   one that does not is not yet a star. What an outburst throws leaves cool
   (operator 2026-10-18: "heat dissipates into the void/vacuum… only the
   deep star core is hot").

This retires `ionizing = 1` at ignition, the re-cut's heat 0, and the
`hot-heart` branch's constant: one heat, from one law.

## 6. Terms (GLOSSARY)

| term | meaning here | replaces |
|---|---|---|
| **heat** | motion out of step: the spread of a body's turning (§3) | "agitation" as a separate idea; frame-count cooling |
| **temperature** | heat per unit of holding, the share `a` | — |
| **state of matter** | molecular, atomic gas or plasma, read from heat against bonds (§4) | — |
| **transition** | dissociation, ionization, recombination, association | "phase" for a change of matter |
| **emission** | heat leaving as light by the unlocked face | `radiates`, frame-count cooling |
| **absorption / irradiance** | light taken in by a body / the light falling on it | — |
| **sink** | where light drains: the vacuum face, the absence of irradiance | "darkness" |
| **contraction heating** | potential released by collapse, less the motion kept in step, into heat (§5) | the constant heat at ignition |

## 7. Witnesses

- **Heat is spread:** a body whose parts turn alike has heat 0; the spread
  of a body's parts is its heat, and adding the same turning to every part
  leaves it unchanged.
- **Friction:** each decoherence leaves `motion + heat` unchanged.
- **Contraction:** across a collapse, `potential + motion + heat` is
  unchanged and `Σ m·ℓ` is unchanged.
- **The star core keeps its fuel by its own heat:** a collapse deep enough
  to strip hydrogen ignites and keeps it; one too shallow does not ignite;
  no constant enters either reading.
- **One founding read before and after:** hydrogen burned above 0 where the
  #119 trace read 0, with the work counters (N against 4N) unchanged in kind.

## 8. Open

1. **The scale on which heat meets a winding.** `open_at` compares heat
   with tightness (enclosed charge over its period, hydrogen's 1); the
   ruling of 2026-10-18 read heat as turns of the bell. §5's heat is a
   share of holding. The bridge between the two units is to be derived,
   never set; until it is, the build reads it as the ruling did (one turn of
   the bell against tightness 1) and says so where it does.
2. **The zero of potential** (docs/106 §7), which §5's release reads as a
   difference and so does not need, but the ledger's totals do.
3. **Whether light rides the `T` lane** when a passage law needs it; it is
   booked on no lane until then (§2).
