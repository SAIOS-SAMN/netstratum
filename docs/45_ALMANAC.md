# The almanac — the ruled grammar of elements and systems

**Status:** element-GRAMMAR-1 first slice landed (2026-07-31,
coder-03; ratified by standing operator order). Landed: the mint deed
(ProspectStruck, tag 70 — the genesis root is the founding receipt's
digest, measured: receipt 0 is the sovereign's registration and
identical across worlds), the assay closure (AssayStruck, tag 71 —
richness × elapsed in one receipt, the bond-settle shape; a scope
correction recorded: extraction landed as a closure, not a zone —
no workspace surgery, no new zone kind), banked element stores (orb
entry tag 72), the abundance spine, hash-derived properties and
names, the prospect/assay CLI verbs. Measured: two worlds mint
different elements from one grammar; the spine binds the draw to the
sky; replay reads, never rolls. Deferred to element-GRAMMAR-2,
recorded: deeper family fantasy rates (§5.2) and G2 element rations (§7.4).
**Play board 2026-08-07:** `31_ELEMENT_COMMERCE.md`. **REFINE-1 landed
Wave-1 `2109bf0`:** `ZoneKind::ElementRefinery`, tag 107 `ElementRefined`,
pure-NPC build/refine — rates from the shared refinement ladder on element
tier. **Still open:** E-ROUTE-1 · E-APT-1 · E-MKT-1. STAMP: `32_STAMP_MARKET.md`.
Composes with: D3's
seeded chart (the draw's door), G2's rations (elements made edible),
SCIENCE-1's relevance (elements made visible), MARKET-1 (elements made
tradeable), WANT-1 (elements made wanted), G4 at the mesh (elements
made confirmable), Ground step (conductor prospect/assay).

**The one law over this whole chapter:** content enters through a
deed, never through a commit. Laws, seeds, and small exact tables are
authored; the chronicle writes the periodic table. Every mechanic below
is discrete, exact-rational, chronicle-seeded, and replayable — the
almanac's "intelligence" is a reading of history (Charter §9), never
an oracle.

---

## 1. What stands today (the closed carriers)

Seven resources, wire codes 1–7, a closed vocabulary: iron · biomass ·
alloy · diamond · joule · sink · crystal. They stay closed — they are
the laws' own carriers (rations, fuel, the lens's power, the thermal
dual ledger). The periodic table does not extend this enum.

## 2. The element registry (the open vocabulary)

An **element** is play-minted identity, the tech-node pattern applied
to matter:

| field | law |
|---|---|
| `element_id` | the seeded synthesis: BLAKE3(chronicle root at the mint ‖ orb ‖ stratum) — replay re-derives it at the identical tick |
| `name` | word-table composition in the discovering line's own voice (the naming law, 10 §2 law 4: the hash is the identity, the name is the tongue) |
| `family` | drawn at the mint from the world's composition vector (§4): one of the small ruled families — light · heavy · noble · volatile |
| `tier` | drawn 1–3 within family (the refinery depth it admits) |
| `richness` | an exact rational drawn from the family's table — the extraction rate carrier |

The registry is **replay-derived state** (the techs precedent):
committed, rebuilt from receipts alone, never a stored authority.
Banked element stores ride one new additive orb TLV entry —
`element_id ‖ ratio` — under SEAL A (old readers skip it); the
`Resource` enum is untouched.

## 3. The mint deed — prospecting

`ProspectStruck` (tag claimed from the header authority on the landing
day): a closure on an orb, actor-scoped, refusal-pure. The deed draws
against the world's composition vector (§4) through `physics::beacon`
with a named domain tag (contract §6 — every draw routes through the
beacon). What the world holds is what the seed always held; the
prospect reads it into the registry. A stratum prospected twice
refuses typed (the once-per-stratum law — nothing mints twice); a
barren draw is a lawful typed verdict carrying its stratum, never a
silent nothing.

**CIV-REACH-1 (landed):** dig depth is capped by civilization growth,
not only by the star. Bronze / pre-industry (`civ_reach = 2`) may open
only strata 1..=2. Each crystallized base condition **1..=5** raises
the ceiling by one; never above `lines_of(star)`. **Assay and refine**
also refuse when the vein's periodic line exceeds `civ_reach` — a
people may chart a deep identity and still be unable to mine it until
sky tech grows. Solar leap (condition 0) does not deepen the mine.

**Star-ENDOW-1 / orbit-ENDOW-1 (landed):** closed-carrier kit scales
with the sky. `ColonyFounded` banks the bronze packet (20 iron / 10
biomass). `StarCommissioned` top-ups the genesis orb to a class kit
(G matches founding; O/B metal-rich + crystal seed; M biomass-rich;
brown dwarf seeds sink). Never strips spent banks. `PlanetSited`
adds a small band gift (cryo→sink, hyperthermal→iron; mesic home
none). Chart transfers still move the founding packet (conservation).

Conservation: an element enters banked stores only by extraction
against `richness` per tick (the extractor pattern, terrain-multiplied
by D3's chart) — the mint creates identity, never matter.

## 4. The composition vector — the almanac of systems

At charting/siting, a world's **composition vector** derives from its
receipted seed (the D3 seeded-chart door, tag 8): per-family exact-
rational weights, drawn once, replayable forever.

The draw's priors are the **abundance spine** — one small ruled table,
operator-owned, keyed by `StarClass` (the thirteen classes are the
almanac's spine; metallicity is real astrophysics and the table wears
it): hot young classes weight heavy families up; dwarfs weight noble
and volatile; the exotic classes (NeutronStar, WolfRayet,
LuminalWhiteHole) carry the rare tails. All weights exact rationals;
placeholder values at the landing, tuned by ruling at balance time.

The **galactic gradient**: per-region multipliers over the spine, read
from a lookup table by the region's wrapped-L1 index (13_GALAXY's
frontier distance) — **never computed powers** (the 2^1100 denominator
lesson, inscribed: exponent-flavored, table-implemented; every
denominator bounded by the table's own).

## 4a. The dispersal law — star death as allocation (ENRICH-1, specced)

**What is already emergent (landed, zero new code):** the mint reads
the star's class at mint time, and `StarAged` (tag 45) moves stars
through their regions — so a star's death already rewrites its own
system's periodic table: every prospect struck after a G-star falls
to WhiteDwarf draws the exotic spine (heavy and noble open, light
closes). Home-system enrichment is a property of composition, not a
feature.

**The dispersal (the blast's gift to the sector), to land:**

1. **The receipted band carrier (the SEAL B constraint made law):** a
   world's chronicle replays from itself alone, and galaxy membership
   lives in the universe ledger world replay is blind to — so sector
   flavor may never be read live from `galaxy_of` at mint time. It
   enters each world as a **gradient band byte**, receipted into the
   world's own chain (fixed at charting; shifted only by enrichment
   deeds), which `element_properties` reads forever after. Charting
   the sky and drawing the resource map are the same act.
2. **The enrichment deed:** on a death transition (StarAged into
   WhiteDwarf ‖ NeutronStar — the operator may widen the trigger
   set), the conductor's year-voice sweep issues a receipted
   `EnrichmentStruck` into every world within the blast band —
   wrapped-L1 distance against a radius lookup table (never computed
   powers; every denominator the table's own). The deed shifts the
   receiving world's band byte additively; future prospects there
   draw heavier. Replay of each world reads its own deeds — the
   sweep composes cross-world, the receipts stay per-world (the
   aperture two-book precedent).
3. **The allocation reading:** destruction is creation — the dying
   star mints its sector's heavy tail. Scarcity gradients follow
   stellar demography; sectors specialize by their dead as much as
   their living. Distribution then rides the landed rails: element
   apertures (§5a) move the veins' yields down the gradients, and
   MARKET-1 prices what the sky made rare.

Lanes: the deed, the band carrier, and the spine modulation are
engine-side (coder-03); the death sweep rides the conductor's beat
(coder-01, the observation-sweep precedent — a relay seam, named).

## 5. The refinery derivation — no hand-written converters

One new zone kind, the **elemental refinery**, tuned at build to a
registered element (the tuning rides the ZoneBuilt receipt). Its
economics derive from the element's drawn properties by the §5.2
converter law as a function:

    input:  the element, at a rate derived from (family, tier)
    output: the element's refined form (tier + 1 identity in the
            registry, minted at first refinement — the same seeded
            synthesis, predecessor-linked), at richness-scaled yield
    heat:   converter-hot (the thermal board prices it)

No per-element table is ever written. Two worlds with different seeds
grow different refinery chains from the same function — measured, not
asserted, at the landing.

## 5a. Element apertures — distribution (specced)

Banked element stores are their own ledger and the aperture speaks
only the seven closed carriers today — **elements are not yet
tradeable cross-world**. Element-GRAMMAR-2 opens the element leg: the
aperture pair keyed by element id (the 47/48 shape with the id in the
value), conservation exact across the pair, demurrage-free (elements
are matter, not thermodynamic ledger quantities). With the gradient
live, comparative advantage derives from astrophysics: sectors
specialize by spine and by their dead; trade flows down the
gradients; the market prices what the sky made rare.

## 6. What stays ruled forever (never generated)

Conservation and the three seals; refusal semantics; the closed
carrier vocabulary (§1); the axis vocabularies (identity words are
ruled, not rolled); the abundance spine's values and every placeholder
constant (operator rulings at balance); the beacon-domain tags. The
grammar generates content; it never generates law.

## 7. The element-GRAMMAR-1 landing checklist (opens on ratification)

1. Tags claimed in the dop.rs header first commit of the cadence
   (the collision convention): the ProspectStruck deed + the orb
   element entry (+ the refinery tuning rider if the ZoneBuilt value
   cannot carry it additively).
2. Red witnesses: two seeds mint different elements with different
   derived refinery chains from one grammar; the same seed re-derives
   byte-identically; the once-per-stratum refusal is pure; a barren
   draw is a typed verdict; abundance respects the spine exactly
   (a class with zero weight in a family never yields it); replay
   matches through mint → extract → refine.
3. The CLI `prospect` verb, refusals naming cures; the registry read
   on `status`.
4. G2 hook recorded (not landed): `metabolic_ration` extended to
   element ids — a line that eats lithium is one row, when ruled.
5. PICKUP/BUILD_ORDER inscriptions; the register note here flipped to
   landed with evidence.

---

## 8. The periodic law — the reading layer (landed, `cfe::almanac`)

**Status:** landed 2026-08-04 (d3-scope), 19 witnesses in
`engine/cfe/tests/almanac_pins.rs`. **Everything in this section is a
derivation** — pure, total, exact-rational, no I/O and no deed.
`dop::element_properties`, `thermal::band_of` and `site_planet` are
untouched, so nothing that has replayed replays differently. It adds
the reading a refinement deed must be written against; it does not
add the deed.

### 8.1 Broken apart by concern, so each axis widens alone

    almanac/periodic.rs   the five axes — where an element sits
    almanac/locality.rs   what a star makes reachable; what pressure
                          refines
    almanac/compound.rs   what elements make together; the gem's identity
    almanac/thermal.rs    the temperature line closed; the orbit grain

The split is the point: widening a lane touches `periodic.rs`, adding
a star class touches `locality.rs`, and neither disturbs the gem
digest or the thermal closure.

### 8.2 The five axes, and each decides something no other can

An axis that changes no outcome is decoration. Every one below is
pinned to an outcome no other axis can move:

| axis | reads | decides |
|---|---|---|
| Line | period, `1..=7` | the pressure it demands to refine |
| Lane | group, `1..=10` | how much it binds — recurs at every line |
| Shell | geometry, `1..=4` | the arity: how many bind at once (`1+shell`) |
| Phase | state, `1..=4` | which thermal region it is workable in |
| Spin | chirality, `0..=1` | which gem a composition names |

Read from bytes 3..=7 of the element id, one per axis, so no two are
correlated by construction. Bytes 1 and 2 belong to
`element_properties` and are untouched — this table is independent of
that draw, not a re-slicing of it.

**Recurrence along the lane is load-bearing.** A lane's character
depends on the lane alone, so a civilization that has refined one
element can predict the next. That is what makes discovery cumulative
rather than a lottery, and it is the pin that would fail first if
anyone made the table a list.

### 8.3 Why the lanes are ten — a derived width, not a preferred one

Saturation for an n-body compound is `arity · (LANES/2 + 1) / 2`, so
it is whole for every arity exactly when `LANES/2 + 1` is even — that
is, when **`LANES ≡ 2 (mod 4)`**:

| Lanes | a=2 | a=3 | a=4 | a=5 | |
|---|---|---|---|---|---|
| 8 | 5 | **15/2** | 10 | **25/2** | odd arities impossible |
| **10** | 6 | 9 | 12 | 15 | every arity reachable |
| 12 | 7 | **21/2** | 14 | **35/2** | the same hole returns |

The first landing stood at 8 and had to record that no three-body
compound could saturate, rather than tune the hole away. Widening to
10 — the smallest width that opens the odd arities — closes it. The
three-body case is the one that matters: it is the irreducibly
multi-party compound a pairwise projection cannot express, and the
witness now checks that none of its three pairs compound on their own.

The binds fold at the half (`1..=5` rising, then back down), which is
what makes the lane periodic rather than a ramp.

### 8.4 Locality — two independent readings

A resource is a place, not a number, and it takes both readings:

- **The star says how deep.** `lines_of(class)` — a heavy, bright
  class forges down to line 7; an M dwarf reaches 3; a brown dwarf 2.
  The same element id is present in one system and absent from
  another. Locality dominates pressure: no stratum releases what the
  system does not carry.
- **The orbit says how hot** (§8.6). Neither implies the other — a
  shallow element under a heavy star can still be unworkable in the
  wrong region.

`pressure(stratum, class)` is the stratum an orb has opened against
the deepest line its star offers; `refines` is met at exact
comparison, no tolerance.

### 8.5 Composition — the lanes' rule, not the members'

Three conditions, one per participating axis: every member shares the
shell (and the count entering equals its arity), every member shares
the phase, and the binds saturate across the set. Any set drawn from
the same lanes compounds alike whatever their lines, so one recipe
names a family of gems — the periodicity paying out.

`gem_of` is content-addressed, so a "new" gem is new by identity
rather than by anyone's say-so. Order-free (members sorted before
hashing — a compound is not a sequence) but **not spin-blind**: the
set's parity enters the digest, so the same composition in opposite
handedness is a different gem. `knowledge_of` sums the members'
lines, so both depth and width are won.

### 8.6 The thermal closure — naming the four silences

`thermal::band_of` returns `Option<SubstrateBand>` and answers none in
four places, which the tree's own witnesses already recorded: below
the cryogenic floor, between cryogenic and mesic, between mesic and
hyperthermal, and above hyperthermal. A `None` is not a temperature —
every orb has one — so a reading that declines to name it is a hole in
the vocabulary rather than a refusal.

`ThermalRegion` closes the line: Quenched · Cryogenic · **Sublime** ·
Mesic · **Torrid** · HyperThermal · **Incandescent** · Beyond, and
`region_of` is total. **It changes no admission** — `SubstrateBand`
still rules siting, and every temperature that had a band keeps it.

### 8.7 The orbit grain — heat amplified by closeness, without irrationality

Heat is amplified by closeness (`t_eq4 = L(1−a)k / r²`), and the
division is where irrationality enters: `r → 0` is unbounded in value,
and `r = 1/10^k` is unbounded in denominator (~`10^(2k)` — the same
exact-rational blowup that killed proportional demurrage at ~2^1100
and forced the flat law; witnessed here at over 30 digits by k = 16).

The engine already answers this one layer down: the melt law posts in
whole lots on a `1/64` grid *"so exact arithmetic stays cheap."* An
orbit is the same kind of quantity and takes the same discipline.
`ORBIT_GRAIN = 64` is that grid, not a new number.

`orbit_on_grain` floors rather than rounds — no orbit is moved closer
to its star than it was asked to be — and lifts everything inside the
first step to the first step: an orbit inside the grain is not a
nearer orbit, it is not an orbit.

Two things fall out and both are pinned:

1. **The denominator cannot run away.** Every grained orbit's
   denominator divides the grain, so the heat stays holdable however
   small the orbit asked to be — under 30 digits at every k tested,
   where the ungrained form had already passed it.
2. **The ceiling is derived**, not chosen: brightest class, zero
   albedo, one grain out. Nothing grained exceeds it, so `Beyond`
   becomes unreachable — the arm exists in order to be checked
   unreachable. Amplification survives intact: quartering the orbit
   still multiplies the heat by more than a hundred.

**The open hole, recorded:** `site_planet` refuses `orbit_r <= 0` and
admits everything above, so an ungrained lawful siting can still
outrun the ceiling. Making the grain law is a `site_planet` amendment
and belongs to that lane; the closure names the region either way.
