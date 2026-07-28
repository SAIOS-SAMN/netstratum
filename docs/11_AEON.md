# AEON — the polytonic clockwork and the fold (AEON-1 spec, design stage)

**Status:** DESIGN — replaces the retracted linear "LIFECYCLE/LIFE-1" draft
(operator critique, 2026-07-28: terrestrial-linear, destroy-not-fold,
shell-script slop, unqualified naming). Re-founded on the engine's own
n-dimensional machinery — the three sovereign axes, the edgeless torus, the
**growing** polynomial fiber, and the four C's — as a **polytonic** design.
Named for Penrose conformal cyclic cosmology: each universe-cycle is an
**aeon**, and the universe *folds* between them. Constants are placeholders.

The board "grew out of control" (184 worlds) because progression was a
LINE: found without bound, climb a rung ladder, decline down a chain, bud
one axis at a time. AEON makes it a *walk on a manifold*.

## 1. Polytonic time — voices, not a line

`universal_clock` renders T as six base-60 fields. They are not one clock
counting up; they are six independent **voices** (polyrhythm). AEON hangs
triggers on their **interference** — an event fires where voices ALIGN, not
on a lone rollover:

- **tick** (voice 5) — the colony beat (flow, thermal, famine). Landed.
- **year** = 60t (voice 4) — the planet speaks (journal), growth is read.
- **cycle** = 3 600t (voice 3) — a world's divergence/confluence is weighed.
- **age** = 216 000t (voice 2) — a star moves in its region (§6).
- **aeon** = 12.96 Mt (voice 1) — the fold (§5).

Interference example: a *fold* is not merely "the aeon voice rolled" — it is
weighed where the aeon voice aligns with a world's own low-coherence phase
(§4). Deterministic, tick-exact, SEAL B intact.

## 2. The gauge axis G — the played key (the unused dimension)

TUniversal is **{S, G, T}**. S (space) and T (time) have been played; **G —
the gauge/fiber phase — has been zero at genesis everywhere** ("G moves with
the fiber landings," never landed). G is the **polytonic key**: a
civilization's tone, an exact-rational phase wrapped on the torus. AEON
plays it.

- **Resonance & clash.** Two worlds at the same S,T but different G stand in
  different keys. Small `|ΔG|` (wrapped) = **resonance** (trade friction
  low, confluence possible, bonds cheap); near the antipode = **clash**
  (friction high, absorption refused). This is a mechanic, not flavour — it
  rides the landed L1-wrapped distance, now on the G axis.
- **Progression MOVES in G.** Divergence, confluence, and crystallized tech
  (10) shift a world's key. `GaugeShifted` (tag 42) receipts the move;
  replay reconstructs it; the star-chart's `g` is the mirror rendering.
- Because the fiber GROWS (Mint adds planes), G is not a fixed dial — new
  tonal dimensions can open at high-value landings. The line becomes a
  surface, then a volume.

## 3. Lineage as a manifold walk — divergence + confluence

"Daughter" is retired: it imported terrestrial unary budding (one parent,
mutate one axis) — half a model with no second half. Two named events, both
*positions on the {disposition × metabolism × perception × G} manifold*,
never a family-tree line:

- **DIVERGENCE (fission, `Diverged` tag 43).** A civilization under sustained
  stress CLEAVES into two peer branches — no parent, no child. The two land
  at divergent points: the cleave IS a key change, the branches taking
  **opposite gauge phases** (`G ± Δ`, wrapped). A dialect splitting, not a
  birth.
- **CONFLUENCE (recombination, `Confluent` tag 44).** Two civilizations held
  by a landed **BOND** (BOND-1, tags 33–34) COMBINE their chronicle-vectors
  into a hybrid at a NEW lattice point, its key the **interference of both**
  (`G = midpoint of the bonded pair on the torus`). Reproduction with both
  halves present — and it is *earned*, gated on a sustained bond, not free.

Neither mints matter from nothing (conservation holds, the P1 charter law);
each is a walk to a new point, seeded deterministically from the acting
chronicle digests (CHARTER §9).

## 4. A world's arc as trajectory — the four C's, not a decline chain

A world does not march FOUNDED → … → RETIRED. It MOVES through a phase space
the four C's already read (this is exactly convergence placement, 07 §2):

- **CONGRUENCE** — its per-tick transitions (the flow).
- **COHERENCE** — its loop residual (does it close its cycles, or leak?).
- **COHESION** — its fiber (the residual it carries, incl. entropy_vented).
- **CONVERGENCE** — where its trajectory is HEADING, not where it sits.

"Stagnation / dormancy / retirement" are not stages on a line — they are a
**region** of that space (low coherence + high carried entropy + null
convergence), reachable from many directions and **escapable** (a rescue —
inbound trade, a key-shift into resonance, a re-siting — moves the trajectory
out). A world becomes **fold-eligible** only when its trajectory has *settled*
in that region across an aeon-voice alignment (§1) — measured, never a timer
counting to a hardcoded death.

## 5. The fold — the universe folds onto itself

The "clean start" is not deletion — it is the **aeon fold**, the mechanic the
convergence engine was already built for, run forward:

1. On an aeon alignment (or an operator `fold`), the live era is **sealed**
   and **rotated into the archive sequence** (`var/` → the next `var-eraN/`).
   `EraFolded` (tag 46) receipts it to the universe ledger.
2. The next boot **CONVERGES** the sealed era: the four C's read which
   trajectories CONTINUE — thriving, coherent lineages re-annex onto the new
   aeon's chart; those settled in the retirement region (§4) annex as
   **derelicts**; broken chains **quarantine**. "Some things continue, some
   won't" is convergence's verdict, not an erasure.
3. **Nothing is destroyed.** The fold is archival by construction — the same
   annex-never-merge law (07 §1). The graveyard is frontier (07 §3 salvage).

This subsumes the old "reset": a fresh universe is an aeon that folded an
empty predecessor.

## 6. Stars as regions, not rails

A star is a point in **mass × composition × age**, not a rung on a track.
Each age it MOVES in that region (class-scaled: O/B fast, K/M nearly still),
`StarAged` (tag 45) recomputing `L` → every bound planet's `T_eq⁴` and band
shift. The dynamic lever stands — a G-star's mesic halls cook as it reddens,
a frozen K world thaws when a neighbour goes giant — but framed as movement
in a region, so two same-class stars can age differently by composition.

## 7. Bounded occupancy — a cap without a monotone line

The manifold has finite **occupancy** (`MAX_LIVE_WORLDS`, placeholder 24).
At density, a new lineage does not queue behind a "death" on a line — it
requires **room in phase space**: a fold clears a settled region, or a
divergence/confluence lands in a *sparse* neighbourhood (low local
occupancy on the torus). Population is bounded by the geometry, not by a
births-equal-deaths counter.

## 8. Process control — the conductor, not scripts

Retracted the bash slop. Control is first-class conductor subcommands,
deterministic and witnessable:

- `conductor run [--cadence]` — beat (default).
- `conductor status` — read the heartbeat (`var/state.json`: live worlds,
  aeon voice, occupancy) without attaching.
- `conductor halt` — **graceful**: an in-process signal handler finishes the
  current beat, flushes the mirror, seals the tail — no torn tails from
  stops.
- `conductor fold` — trigger the aeon fold (§5) on demand.

## 9. The AEON-1 landing + tags

Lands (conductor + a little cfe): polytonic trigger dispatch (§1), the gauge
axis as played state + resonance/clash (§2), divergence & confluence (§3,
confluence gated on landed bonds), the four-C trajectory read replacing the
decline chain (§4), the fold (§5), star-region evolution (§6), occupancy
(§7), and the control subcommands with graceful shutdown (§8). **Tags
reserved 42–46:** 42 `GaugeShifted` · 43 `Diverged` · 44 `Confluent` ·
45 `StarAged` · 46 `EraFolded` — next free **47**. Seasonal band wobble is
DERIVED per year (a formula on the tick), no receipt. Coordinate: 32–38
ideology (landed), 39–40 tech, 42–46 aeon.
