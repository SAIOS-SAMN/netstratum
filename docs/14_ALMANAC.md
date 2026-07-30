# THE ALMANAC — the ruled grammar of elements and systems

**Status:** ELEMENT-GRAMMAR-1 FIRST SLICE LANDED (2026-07-31,
coder-03; ratified by standing operator order). Landed: the mint deed
(ProspectStruck, tag 70 — the genesis root is the FOUNDING receipt's
digest, measured: receipt 0 is the sovereign's registration and
identical across worlds), the assay closure (AssayStruck, tag 71 —
richness × elapsed in ONE receipt, the bond-settle shape; a scope
CORRECTION recorded: extraction landed as a closure, not a zone —
no workspace surgery, no new zone kind), banked element stores (orb
entry tag 72), the abundance spine, hash-derived properties and
names, the prospect/assay CLI verbs. Measured: two worlds mint
DIFFERENT elements from one grammar; the spine binds the draw to the
sky; replay reads, never rolls. DEFERRED to ELEMENT-GRAMMAR-2,
recorded: the refinery derivation (§5) and G2 element rations (§7.4). Composes with: D3's
seeded chart (the draw's door), G2's rations (elements made edible),
SCIENCE-1's relevance (elements made visible), MARKET-1 (elements made
tradeable), WANT-1 (elements made wanted), G4 at the mesh (elements
made confirmable).

**The one law over this whole chapter:** content enters through a
DEED, never through a commit. We author laws, seeds, and small exact
tables; the chronicle writes the periodic table. Every mechanic below
is discrete, exact-rational, chronicle-seeded, and replayable — the
almanac's "intelligence" is a reading of history (CHARTER §9), never
an oracle.

---

## 1. What stands today (the closed carriers)

Seven resources, wire codes 1–7, a CLOSED vocabulary: iron · biomass ·
alloy · diamond · joule · sink · crystal. They stay closed — they are
the LAWS' own carriers (rations, fuel, the lens's power, the thermal
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
Banked element stores ride ONE new additive orb TLV entry —
`element_id ‖ ratio` — under SEAL A (old readers skip it); the
`Resource` enum is untouched.

## 3. The mint deed — prospecting

`ProspectStruck` (tag claimed from the header authority on the landing
day): a closure on an orb, actor-scoped, refusal-pure. The deed draws
against the world's composition vector (§4) through `physics::beacon`
with a named domain tag (CONTRACT §6 — every draw routes through the
beacon). What the world holds is what the seed always held; the
prospect READS it into the registry. A stratum prospected twice
refuses typed (the once-per-stratum law — nothing mints twice); a
barren draw is a lawful typed verdict carrying its stratum, never a
silent nothing.

Conservation: an element enters banked stores only by extraction
against `richness` per tick (the extractor pattern, terrain-multiplied
by D3's chart) — the mint creates IDENTITY, never matter.

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
from a LOOKUP table by the region's wrapped-L1 index (13_GALAXY's
frontier distance) — **never computed powers** (the 2^1100 denominator
lesson, inscribed: exponent-flavored, table-implemented; every
denominator bounded by the table's own).

## 4a. The dispersal law — star death as allocation (ENRICH-1, specced)

**What is already emergent (landed, zero new code):** the mint reads
the star's class AT MINT TIME, and `StarAged` (tag 45) moves stars
through their regions — so a star's death already rewrites its OWN
system's periodic table: every prospect struck after a G-star falls
to WhiteDwarf draws the exotic spine (heavy and noble open, light
closes). Home-system enrichment is a property of composition, not a
feature.

**The dispersal (the blast's gift to the sector), to land:**

1. **The receipted band carrier (the SEAL B constraint made law):** a
   world's chronicle replays from itself alone, and galaxy membership
   lives in the universe ledger world replay is blind to — so sector
   flavor may NEVER be read live from `galaxy_of` at mint time. It
   enters each world as a **gradient band byte**, receipted into the
   world's own chain (fixed at charting; shifted only by enrichment
   deeds), which `element_properties` reads forever after. Charting
   the sky and drawing the resource map are the same act.
2. **The enrichment deed:** on a death transition (StarAged into
   WhiteDwarf ‖ NeutronStar — the operator may widen the trigger
   set), the conductor's year-voice sweep issues a receipted
   `EnrichmentStruck` into every world within the blast band —
   wrapped-L1 distance against a RADIUS LOOKUP table (never computed
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

One new zone kind, the **elemental refinery**, TUNED at build to a
registered element (the tuning rides the ZoneBuilt receipt). Its
economics DERIVE from the element's drawn properties by the §5.2
converter law as a FUNCTION:

    input:  the element, at a rate derived from (family, tier)
    output: the element's refined form (tier + 1 identity in the
            registry, minted at first refinement — the same seeded
            synthesis, parent-linked), at richness-scaled yield
    heat:   converter-hot (the thermal board prices it)

No per-element table is ever written. Two worlds with different seeds
grow DIFFERENT refinery chains from the same function — measured, not
asserted, at the landing.

## 5a. Element apertures — distribution (specced)

Banked element stores are their own ledger and the aperture speaks
only the seven closed carriers today — **elements are not yet
tradeable cross-world**. ELEMENT-GRAMMAR-2 opens the element leg: the
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
grammar generates CONTENT; it never generates LAW.

## 7. The ELEMENT-GRAMMAR-1 landing checklist (opens on ratification)

1. Tags claimed in the dop.rs header FIRST commit of the cadence
   (the collision convention): the ProspectStruck deed + the orb
   element entry (+ the refinery tuning rider if the ZoneBuilt value
   cannot carry it additively).
2. Red witnesses: two seeds mint DIFFERENT elements with different
   derived refinery chains from ONE grammar; the same seed re-derives
   byte-identically; the once-per-stratum refusal is pure; a barren
   draw is a typed verdict; abundance respects the spine exactly
   (a class with zero weight in a family never yields it); replay
   MATCHES through mint → extract → refine.
3. The CLI `prospect` verb, refusals naming cures; the registry read
   on `status`.
4. G2 hook recorded (not landed): `metabolic_ration` extended to
   element ids — a line that eats lithium is one row, when ruled.
5. PICKUP/BUILD_ORDER inscriptions; the register note here flipped to
   LANDED with evidence.
