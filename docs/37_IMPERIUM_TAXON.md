# IMPERIUM TAXON — species · clades · economy · planet · labs

**Status:** living law (2026-08-08).  
**Authority:** ports and extends `15_TAXON` (class/clade wire + seal).  
**Pure module:** `cfe::almanac::taxon_economy`.

---

## 0. Layers (kernel sees codes only)

| Layer | Codes | What |
|-------|-------|------|
| **Class** | 0–7 | Archetype mechanism (`15` §2) |
| **Clade** | 0–20 | Body expression (`15` §3 · imperium words) |
| **Metabolism** | 1 mineral · 2 biotic · 3 plasmic | Food (`04` genesis) |
| **Economic mode** | derived | extractive · regenerative · **balanced** |
| **Planet fit** | derived | native · tolerant · hostile |
| **Line / species string** | register | Imperium tongue — never Earth common animals |

---

## 1. Classes (from `15_TAXON` — ported)

| # | Sigil | Word | Seated lines |
|---|-------|------|--------------|
| 0 | ∅ | unclassed | revenants |
| 1 | ⬡ | biotic-swarm | Vitreous Hex, Klethaxu, Mycelial Weft, Coralline, Deepmat |
| 2 | ◈ | geodic-lithovore | Slagborn, Tunborn, Agarthan |
| 3 | ◐ | metashifter | Darlok |
| 4 | ≋ | telepathic-resonant | Archon, Draco, Syzygy |
| 5 | ⊚ | pelagic-aerial | Echowright, Brine Cortex, Frostwright |
| 6 | ∿ | plasma-field | Coronal Choir |
| 7 | ▲ | **apex-solitary** | Lyran, Anunnaki (flatlander seat class) |

---

## 2. Clades — Latin → space sentience (ported + display)

| # | Imperium | Alias (parse) | Role |
|---|----------|---------------|------|
| 1 | ailuroform | felid | solitary apex |
| 2 | **bimaniform** | humanoid, **flatlander**, **human**, humans | **FLATLANDER BASELINE** |
| 3 | cryocoherens | cryo-coherent | superfluid mind |
| 4 | lithophage | lithic | stone-eater |
| 5 | hypobiotid | tardigrade | void-sleeper |
| 6 | soniform | cetacean | pressure-song |
| 7 | polybrachia | cephalopod | multi-arm cortex |
| 8 | hexasocial | hymenopteran | eusocial comb |
| 9 | amphiform | amphibian | dual-medium |
| 10 | mycotiform | fungal | hyphal network |
| 11 | cnidiform | cnidarian | accrete-polyp |
| 12 | plasmiform | plasmoid | plasma intelligence |
| 13 | syzygiform | syzygial | conjunction-form |
| 14 | endolithid | endolithic | pore-rock |
| 15 | volatiform | avian | free-body atmosphere |
| 16 | actinoform | teleost | finned line |
| 17 | revenid | revenant | returners |
| 18 | heliciform | gastropod | reserved spiral |
| 19 | draconiform | draconic | class-4 sub-archetype |
| 20 | metamorphid | metashifting | veil-native |

Wire codes **locked**. Display is imperium; aliases parse forever.

---

## 3. FLATLANDER BASELINE (restored and named)

| Field | Value |
|-------|--------|
| Clade wire | **2** |
| Imperium word | `bimaniform` |
| Knowledge aliases | `flatlander` · `human` · `humans` · `baseline` |
| Species string | `bimaniform-flatlander` |
| Economic mode | **Balanced** (always) |
| Design role | Earth-trained operator knowledge is **this** suite |

**Lines that seat the baseline:** Anunnaki (progenitor), Kemet / Baktun / Dogon
(sovereign Record Keepers), Pleiadian (watcher) — all `bimaniform`.

Unsealed worlds **default** taxon economy to flatlander balanced so play
and tests have a readable baseline before `seal_taxon`.

---

## 4. Economic modes (all generative)

Every mode **generates** play and receipts. They differ in *how* matter
moves — not in whether the line can live.

| Mode | Word | Typical clades / classes | Extract | Regen | Habitat |
|------|------|--------------------------|---------|-------|---------|
| **Extractive** | mine-first | geodic class, lithophage, hypobiotid, endolithid, plasma class, plasmiform | ×5/4 | ×3/4 | ×7/8 |
| **Regenerative** | restore-first | biotic-swarm class, mycotiform, cnidiform, hexasocial, soniform, amphiform | ×3/4 | ×5/4 | ×5/4 |
| **Balanced** | flatlander | **bimaniform**, ailuroform, polybrachia, default | ×1 | ×1 | ×1 |

Scales further multiply **planet fit** (native 1 · tolerant 3/4 · hostile 1/2).

### 4.1 Tech / research / mining gates

| Gate | Extractive | Regenerative | Balanced (flatlander) |
|------|------------|--------------|------------------------|
| Place **Extractor** | soft-cap 6 | soft-cap 2; **hard refuse** if hostile + over cap | soft-cap 4 |
| Place **Regenerator** | tech condition **3** | tech condition **2** | tech condition **3** |
| Place **ElementRefinery** | condition 3 | condition 3 | condition 3 |
| Research seat fee | ×2 (secondary) | ×1 | ×1 (primary knowledge path) |
| Science lab affinity | mineral/plasmic tables | biotic tables | flatlander primary suite |

### 4.2 Planet fit (founded world colours the line)

Preferred region by clade → compare to actual thermal region of the
**sited** planet (star × albedo × orbit):

| Clade family | Prefers |
|--------------|---------|
| cryocoherens · draconiform | Cryogenic |
| plasmiform · syzygiform | HyperThermal |
| flatlander · ailuroform · hexasocial · mycotiform · mesic majority | Mesic |
| hypobiotid · lithophage | Mesic (stone) |
| volatiform | Torrid |

**Same species, different colony world → different fit → different rates.**
That is the planet dependence: generative always, numbers from place.

---

## 5. Technology · research · information · mining (seated)

| Domain | How taxon seats it |
|--------|---------------------|
| **Mining / extraction** | Extractor yield × extract_scale(mode, fit); soft-cap by mode |
| **Regeneration** | Regenerator yield × regen_scale; tech door softens for regenerative |
| **Habitat / farm** | habitat_scale(mode, fit); plasmic farm still metabolism-0 joule |
| **Research / information** | Science seats + research fee mult; flatlander primary |
| **Labs (chem/bio/synth…)** | `science_seat_affinity(metabolism, discipline, clade)` (prior land) |

No second physics — only pure scales and gates over existing zones/tech.

---

## 6. LOCKED line names (court only)

Tunborn · Slagborn · Echowright · Coronal Choir · Agarthan · Archon · Draco ·
Darlok · Lyran · Anunnaki — seat structure. Species/clade words move;
**line names** wait for operator court rename.

---

## 7. Pins

`engine/cfe/tests/taxon_economy_pins.rs`:

- flatlander baseline aliases + balanced mode  
- extractive / regenerative clade derivation  
- yield scale inequalities  
- planet fit native/hostile  
- regenerator tech soften  
- world defaults + sealed extractive  
- extractor builds · regen tech gate  
- every class×clade has a mode  
- clade word round-trip  

---

## 8. CLI / play

```text
bench science          # metabolism · clade · primary/secondary/alien fees
status / dashboard     # taxon economy surfaces via residual + board
zone <tile> ext        # extract — may soft-cap under regenerative+hostile
zone <tile> regen      # regenerator — tech condition by mode
```

Refusal: `EconomicModeBlocked` when regenerative line over-extracts on a
hostile world.
