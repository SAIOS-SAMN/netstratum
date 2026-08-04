# THE TAXON VOCABULARIES — class and clade enter the kernel (KERNEL-TAXON, draft for ratification)

**Status:** RATIFIED 2026-07-31 (operator — all three of §6 in one
ruling: the boundary, the class partition with its seventh class and
every seating, the clade table). The vocabularies below are LAW at the
register tier; they enter the constitution at coder-03's amendment.
The ratified machine mapping for the migration wave stands at
`seeds/taxon-map.json` (33 lines, branches inheriting the root class
through the lineage DAG, zero unclassed). Awaiting, in order: the
coder-04 codec freeze window (committed fields), the coder-03
amendment (tag + invariant coverage sealed, minor version bump), then
the migration wave executes from the map.

**The lane contract (KERNEL_RELAY §8.3):** coder-02 authors the
vocabularies and the seal semantics; coder-03 ratifies the invariant
coverage and seals it into the constitution digest (additive tier);
coder-04's codec freeze window gates the committed fields; ruleset
couplings land inside `admits()` after the I-invariants seal.

## 1. The boundary — what moves, what stays

| Category | Disposition | Why |
|---|---|---|
| **class** | MOVES into the kernel (closed u8 vocabulary) | identity fact; mechanics may lawfully key on it |
| **clade** | MOVES into the kernel (closed u8 vocabulary) | identity fact; the taxon tier between species and axes |
| **creed** | STAYS derived | operator-ruled generative (14 §2.6) — enumerating belief is barred; its kernel hook is the practice deed, already landed |
| **function** | STAYS derived | a niche reading over state (12 §4), never stored |
| **names / species / motto** | STAY register-side forever | names are the line's own tongue; the kernel sees only codes |

## 2. The class vocabulary (proposed, code 0 lawful-unclassed)

Append-only; one code, one meaning, forever. The six archetype classes
(14 §1) plus a seventh the spectrum lacked — the solitary apex lines a
partition needs:

| code | class | seated today |
|---|---|---|
| 0 | unclassed (lawful — a class arrives by amendment, never by default) | revenants, unratified lines |
| 1 | biotic-swarm (collective/eusocial/colonial) | Hivelight, Klethaxu, Mycelial Weft, Coralline, Deepmat |
| 2 | geodic-lithovore (mineral metabolisms, stone-eaters) | Slagborn, Tunborn, Cairnborn, Agarthan |
| 3 | metashifter (veil-native) | Darlok — the seat is FILLED (2026-08-01) |
| 4 | telepathic-resonant (alignment/coherence lines) | Syzygy, Archon, **Draco** (sub-archetype of Archon — same class, clade 19) |
| 5 | pelagic-aerial (volumetric biotic) | Echowright, Eightfold, Hadal, Aerie, Frostwright |
| 6 | etherial-field (plasmic/energy) | Coronal Choir, Borealis |
| 7 | apex-terrestrial (solitary apex) | Lyran, Anunnaki |

## 3. The clade vocabulary (proposed, from the live register)

Code 0 = unrecorded (lawful). Append-only; future seats reserve
forward (18 gastropod — the register awaits its snail):

1 felid · 2 humanoid · 3 cryo-coherent · 4 lithic · 5 tardigrade ·
6 cetacean · 7 cephalopod · 8 hymenopteran · 9 amphibian · 10 fungal ·
11 cnidarian · 12 plasmoid · 13 syzygial · 14 endolithic · 15 avian ·
16 teleost · 17 revenant · 18 gastropod (reserved) · **19 draconic** ·
**20 metashifting**.

**19 draconic — a SUB-ARCHETYPE, not a peer line** (operator ruling
2026-08-01). The draconic clade seats inside class 4
(telepathic-resonant), the SAME class the Archon line holds. That is
what "entangled" means here and it is the structural claim, not a
flavour note: **one archetype, two expressions.** Class is the shared
identity; clade is how it is expressed. A draconic line and an Archon
line are not two peoples who happen to cooperate — they are the same
coherence-archetype read at two clades, and any mechanic keyed on class
sees them as one.

**20 metashifting — the Darlok seat, FILLED.** Class 3 (metashifter,
veil-native) stood reserved and empty since the partition was drafted.
The clade that seats there is now named, and the class has an occupant.

## 4. The seal — a NEW deed, deliberately (the amendment-tier argument)

`TaxonSealed { class: u8, clade: u8 }` — once per star, beside the
G1 seal, never inside it: **extending GenesisSealed's fields would
change the meaning of an existing record — a BREAKING amendment (major
bump). A new deed with its own tag is ADDITIVE (minor bump)** — the
tier the charter built for exactly this cadence. The tag number is
claimed at the landing amendment, not here (the registry moves too
fast to pre-claim; four live collisions taught the sequence).

**Invariant coverage (drafted for coder-03's ratification — a tag
lands only WITH its coverage):**
- **I2 containment:** the sealing actor holds Build scope or stands
  sovereign; the deed refuses otherwise.
- **Vocabulary validity (DOP schema law):** `class` and `clade` must
  stand in their registries at the constitution's version — an
  out-of-vocabulary code is an unlawful step, kernel-rejected.
- **The once-law:** one TaxonSealed per star; a second is an unlawful
  step (the GenesisSealed precedent).
- **I1 conservation:** the deed carries no amounts — zero-sum holds
  unconditionally.
- **Projection:** state gains `(class, clade)`; replay reads the
  receipt, never re-derives; resume carries it.

## 5. Sequencing (nothing lands out of order)

1. NOW — this draft; the three ratifications (§6).
2. Codec freeze window (coder-04) — the committed `(class, clade)`
   fields enter the snapshot schema.
3. The amendment (coder-03 seals; minor version bump; tag claimed).
4. The migration wave — the 33 living lines seal from the ratified
   table (one receipted deed each, operator provenance).
5. Re-anchoring — function/creed derivations read sealed codes; the
   scanner gains the taxon facet by NS-2 §4.2 (one appended row).
6. Couplings — taxon-keyed ruleset rows land inside `admits()`, one
   per landing with vectors (the G2 pattern).
7. #19 unblocks — the archetype expansion seeds with codes at genesis.

## 6. The three ratifications that close this draft

1. **The boundary** (§1) — class+clade move; creed/function stay.
2. **The class partition** (§2) — including the seventh class and
   every seating; any line the operator re-seats moves by table edit
   before the migration, by amendment after.
3. **The clade table** (§3) — codes 1–17 as assigned, 18 reserved.
