# THE ALMANAC — the ruled grammar of elements and systems

**Status:** DESIGN (2026-07-30, coder-03; operator direction: build the
dynamics, never the content — "the system autocompletes with its own
intelligence"). Awaits operator ratification; the landing checklist
(§7) opens ELEMENT-GRAMMAR-1 on that ruling. Composes with: D3's
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
