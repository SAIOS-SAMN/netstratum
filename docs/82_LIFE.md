# 82 — Life: the ladder each world climbs

Operator, 2026-10-14: a universe cannot be empty. Era 1 deploys galaxies,
star systems and planets (docs/78), and life must arise in it from its own
physics. Netstratum had dropped every species when it stopped carrying era 0's
founding lines. xylarium-kernel's practice, adopted here: **laws carry,
records do not.** Named species are records; life is drawn per world from the
world's own physics, and the hierarchy is a reading of who wakes where and
when. This document states the ladder in netstratum's laws (docs/78's design
laws: exact, square-free, a construction, no tables of Earth's world).

Status: designed 2026-10-14, reviewed by xylarium-kernel the same day (its
changes taken in below); not built.

## What a world gives (built)

- **Its matter:** grains per Z (the pulse's split and the breath).
- **Its star:** the mass it orbits, `M(<r)`, and its radius r.
- **Its clocks:** its local year (one turn, `P = r / M(<r)` bells), its
  nested rate against the universal echo (`r / (r + M(<r))`, multiplied up
  the layers), its coherence and its rungs (docs/78, "The breath").
- **Its chemistry:** each Z's atom (depth, unpaired, tightness τ), which
  sets of atoms close as compounds, and a compound's grip `τ_min`
  (docs/78, "The atom").

## The ladder

1. **Temperature.** A world's heat is its star's agitation at its radius,
   the same enclosure law that turns it: `T = K · M(<r) / r`. It is linear
   (flux through a fluid-dimension shell, design law 2), so a world's heat
   and its turning come from one source. `K` is the orb tier's one anchor,
   declared from one measured world: Earth, 1 AU about one Sun, about 255 K
   in equilibrium. Before this can spread worlds across climates, the orbs'
   SPACING must be physical: neighbours stand ten reaches apart, not with
   reaches just touching (xylarium: Gyr-stable systems sit 7 to 10 mutual
   reaches apart, below about 3.5 they are unstable on short times; touching
   reaches put every world in one climate). The next world stands at
   `r · (1 + 10s) / (1 − 10s)`.
2. **The solvent.** Of the compounds the world's own elements close (docs/78),
   the most abundant whose grip holds at its temperature
   (`τ_min · 157,800 K > T`, the hydrogen anchor). No compound holds: no
   solvent, no life (a plasma world, a world of bare metal).
3. **Fidelity: Boltzmann, held as halvings** (xylarium's review: a linear
   ratio never falls below about 1e-3, since τ runs 1/P to 2, and minds
   would almost never wake). A copy fails where heat beats the solvent's
   cohesion, `ε = e^(−x)` with `x = τ_min · 157,800 K / T`. The exponential
   is no power and no square: it is a count of HALVINGS, `ε = 2^(−x/ln 2)`.
   It is held exactly as a certified enclosure: the halvings bounded below
   and above by rational bounds of `1/ln 2`, and the ceiling read from the
   bound that errs toward fewer minds. Water at 300 K gives about 1e-4.
4. **The ceiling.** A lineage holds `b / ε` bits; each rung of error
   correction multiplies what it holds by `1 / ε`. A MIND needs the rung that
   holds a mind's blueprint, `B` bits (an anchor, of order a genome). The
   rungs a world needs are the least s with `b / ε^(s+1) ≥ B`: a count, by
   repeated multiplication, read from the chemistry.
5. **The climb.** Each rung is climbed by selection writing at most one bit
   a generation, and fewer in a small population (`N · ε · L` mutations a
   generation). A generation is one of the world's own PIVOTS, on its own
   proper clock (xylarium's review: copying is chemistry, not orbits):
   - one local DAY (a turn of its spin) while the genome fits one copier;
   - one local YEAR (a turn of its orbit) once a body carries it.
   A world's spin is not yet physics (docs/78, S^60 bodies): until it is,
   its day is a declared share of its year (an anchor). A slower, deeper
   clock climbs slower against the universe.
   - **Decay:** stored information breaks while it waits to be copied. Hot
     media copy fast and forget fast; cold media keep. Without it, hot
     worlds win too easily.
6. **The window and erasure.** A world can climb while its star holds,
   until the star dissolves (docs/78, the breath sheds it).
   - **An impact sends a lineage back to the START of its rung, and that
     rung's need is kept, not redrawn** (xylarium: a fresh draw forgets
     impacts and undercounted their minds about sevenfold). Large impacts
     end complex life only; life itself is never undone.
   - **Sterilisation:** a dying star's collapse sterilises open surfaces,
     more often where dying stars crowd; seas under ice or deep are
     shielded.
7. **Transfer.** Life (never a mind) may carry between neighbouring worlds of
   one system, never between stars.
8. **A mind wakes** where the ladder is climbed within the window and the
   world can feed a viable people (its biomass, an anchor of order Franklin's
   500 individuals). It is an EVENT, written to the annals at the bell it
   happens: a perpendicular write up, from the world to its galaxy. The run
   is the draw: each world climbs once, by its own seed (xylarium estimates
   by independent replays; we need no estimate, we live the one history).

## A people is read from its world

- **Its solvent and medium** define it: the compound it lives in and the
  thermal band it works in (docs/78's workability). Names are spoken from
  the seed's syllables, flavour only.
- **No catalogue, no types.** Era 0's 22 named species stay in era 0's
  record. What carries is the LAW of how a lineage forms, splits and
  specialises (the conductor's cleave, lineage and caste machinery), applied
  to peoples that arise here.

## The hierarchy is a reading of who wakes where and when

- **The first minds wake near the galactic cores,** where systems are
  oldest (the first generation) and their worlds' ladders are climbed first.
- **Ancients by birth:** the minds waking at a galaxy's core within one core
  assembly time of that core's first mind. **By deed:** keeping a core's
  throat for a full rung (docs/78, "The clocks": only where time layers).
- **A progenitor** seat beside the first minds; every other people a
  **sovereign**; **stewards** where realms emerge (docs/75); **captains**
  as ships are taken. The conductor's existing hierarchy laws read these
  births instead of a compiled roster.

## Seating a people in the conductor

A mind that wakes becomes a line in the conductor's existing machinery
(population, minds, research, markets, lineage). Its place is a stand-in
until the sphere (A1): what the machinery reads as a box position is derived
from its galaxy and system and marked in docs/78's flat-site table for
replacement. Its star class is read from its star's mass, its orbit from its
radius.

## Anchors (declared placeholders, design law 1)

| anchor | value | stands for |
|---|---|---|
| `K`, the orb tier's heat | Earth: 1 AU, 1 Sun, ~255 K | a radiation law not yet stated |
| spacing between worlds | ~10 reaches | orbital stability (dynamics not yet stated) |
| `B`, a mind's blueprint | of order a genome (~6e9 bits) | what a mind must hold |
| `b`, a bare lineage | of order a first replicator | the ladder's foot |
| a generation | a local day (one copier), a local year (a body) | the world's own pivots |
| a world's day | a declared share of its year | its spin, until S^60 bodies spin |
| `1/ln 2` | certified rational bounds | the halvings of Boltzmann |
| impacts, sterilisations | rates per local year | a dynamics not yet stated |
| a viable people | of order 500 | Franklin's minimum |

## Done-when (the live log, before → after)

1. Worlds with a solvent: 0 → read, by climate.
2. Minds woken: 0 → greater than 0, each an annal event with its world, bell,
   solvent and rungs climbed.
3. The first ancients, progenitors and sovereigns read from the wakings.
4. Lines in the conductor from wakings: 0 → greater than 0, and the per-beat
   machinery acting on them (HUB, research, markets reading non-zero).
