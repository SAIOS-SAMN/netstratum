# Hubs — bench · intel · trade · diplomatic

**Status:** living map (update with every hub land).  
**Law:** hubs are presentation + deed routing over existing world verbs. No second physics.

---

## 0. Dual law (required)

Every **standalone activity root** is also a **bench craft twin**:

```text
activity root S  ──run──►  operate the game
bench S          ──craft─►  build the components that S runs
```

| Activity root | Bench craft twin | Components |
|---------------|------------------|------------|
| `intel` | **`bench intel`** | forge chain · instruments · chem · zones · doctrine prep |
| `trade` | **`bench trade`** | econ · fleet · offices · assay (umbrella) |
| **`diplomatic`** | **`bench diplomatic`** | councils · bonds · forwards · seats · routes · escorts |

**Naming:** preferred root is **`diplomatic`** (unambiguous). Short alias **`diplo`** still accepted everywhere (`diplo local` = `diplomatic local`, etc.).

Aliases (same craft surfaces):
- `bench intel-craft` · `bench knowledge-craft` → `bench intel`
- `bench trade-craft` · `bench circulate` → `bench trade`
- `bench diplo` · `bench diplomacy` · `bench diplomatics` · `bench pact` → `bench diplomatic`

Cross-links from activity roots: `intel craft` / `trade craft` / `diplomatic craft` → matching `bench S`.

---

## 1. Science craft — **seated labs** (paid unlocks · dual ± · explosion)

Not activity roots — **capability seats** under bench. No free science.

### Cost classes (not all monetary)

Bench craft is **not** paid in STAMP unit-of-account by default. Three classes:

| Class | What | Examples |
|-------|------|----------|
| **Local resource** | Colony banks — iron, biomass, alloy, diamond | zones · chem/bio/physics seats · hull scrap |
| **Resonance** | Crystal (scanner/knowledge grain — **not STAMP**) | math/research seats · catalog grain · instrument path |
| **Off-planet** | Requires *resourcing path*, not a price tag | synth / clone / skin **R2** + their create paths |

Off-planet evidence (any one): `has_crossed` · non-empty element routes · non-empty ROUTE-1 deals.  
Advanced tech is **gated** until that evidence exists — synth R2 cannot be bought with local STAMP alone.

### Seating law

| Rank | Meaning |
|------|---------|
| 0 | unseated — read-only craft surface |
| 1 | basic acts (catalog / iso / domain craft) — local/resonance bill |
| 2 | **creation** paths; advanced disciplines need **off-planet** first |

Bill: resource-specific × discipline × rank × taxon affinity (exact rational, no dial).

```text
bench science seat <chem|biology|physics|math|research|synthetic|clone|skin>
```

### Creation paths (need primary seat R2 + required peers)

| Path | Seats | Known + | Known − | Hidden + | Hidden − (explosion) |
|------|-------|---------|---------|----------|----------------------|
| `chem` | Chem | catalog progress | order entropy / fee | dark-well map | runaway reaction |
| `biochem` | Bio+Chem | hybrid yield | contamination | metabolic fit | cascade famine seed |
| `synth` | Synthetic+Math | cyber integrity | identity shear | mesh control | **rogue faction (matrix)** |
| `clone` | Clone+Biology | population seat | identity dilution | escort pool | clone revolt |
| `skin` | Skin+Biology | disguise grade | unmask debt | face-dance license | unmask catastrophe |

Hidden + revealed with Math seat or empiricist perception.  
Hidden − previewed when dark-led / envy-wrath, or after explode.

### Explosion

```text
risk = path.base + max(0, dark−survival) + unseated/8 + (order_thin?1/8:0)
explodes when risk ≥ 1/2
```

No RNG — pure derived comparison. Tags 159–162 · SCHEMA_V16 · kernel 0.37.

### Commands

| Command | Surface |
|---------|---------|
| `bench science` | seats board · clones · synth · skin · rogue |
| `bench science seat …` | pay crystal unlock |
| `bench science preview <path>` | poles + risk before create |
| `bench science create <path>` | dual ± attempt (may explode) |
| `bench chem|biology|physics|math|research` | discipline craft |
| `bench science synth\|clone\|skin` | mod path boards |

Top-level shortcuts: `chem` · `biology` · `physics` · `math` · `science` → matching `bench …`.

---

## 2. Roots

| Root | Role |
|------|------|
| `bench` | **Build** components (includes dual-law craft twins + science) |
| `intel` | **Knowledge activity** |
| `trade` | **Circulate / run** components |
| `diplomatic` | **Diplomacy activity** (local · stellar · mesh); alias `diplo` |
| **`polity`** | **Line identity** (optional 5th · vent open) — genesis · taxon · mint · AP |

### Domain map

```text
bench          BUILD COMPONENTS
  planetary    bio · chem · terraform · medium · thermal · balance
  build        zones · yard · refinery · regen
  star         commission · site · age · rings
  estate       chart · parcels · appraise · ladder
  medium       ATM envelope · reinforce · tamper
  ── science craft ──
  science      index
  chem         bonds · iso · catalog
  biology      life · biomass · regen · habitat
  physics      star · thermal · medium · site
  math         ratios · balances · thresholds
  research     constellation · forge craft
  ── other craft ──
  econ         instruments · routes · rails
  offices      seat exchange|vault|assay|anchor
  fleet        yard · mint hull · scrap
  doctrine     posture · KIN prep
  assay        T7 court craft
  ── dual law craft twins ──
  intel        forge chain · instruments → root intel
  trade        econ · fleet · offices → root trade
  diplomatic   council · bond · seats → root diplomatic

intel          KNOWLEDGE ACTIVITY
  research     constellation · forge · practice (run)
  knowledge    tech · chem · stash · lit · books
  ops          probe · exfil · sabotage · scan
  opsec        veil · ward · detection
  link         council · license · bond · forward
  culture      literature aspire/claim
  craft        → bench intel

trade          CIRCULATE / RUN
  books · stamp · element · fleet · rails · vault · aperture · desk
  craft        → bench trade
  (trade diplo|stellar → redirect to diplomatic root)

diplomatic     DIPLOMACY ACTIVITY  (alias: diplo)
  local        pair treaties · renounce · DEEP DEEDS: seal/renounce/fence fire from the leaf
  stellar      crossed · war walls · aperture · license
  mesh         councils · bonds · forwards standing
  craft        → bench diplomatic

polity         LINE IDENTITY (5th root · dual law)
  genesis      sealed axes · disposition · metabolism · perception
  taxon        class · clade · economy mode · planet fit
  mint         discovery mint (warrant · no free mint)
  ap           P8.1 shared AP budget (build·prospect·research·quote)
  labor        P8.3 force from habitats + clones
  craft        → bench polity
```

### Flow law

```text
bench crafts ──► intel operates knowledge tools / bases
             ├► trade runs market / haul / vault
             └► diplomatic runs treaties / stellar / mesh

Dual:  intel       ←activity→  bench intel
       trade       ←activity→  bench trade
       diplomatic  ←activity→  bench diplomatic

Science: bench chem|biology|physics|math|research  →  intel knowledge/research

Polity:  the polity's identity (axes · taxon · mint · AP · labor)
Dual:    polity  ←activity→  bench polity
```

---

## 3. Still open

| ID | Open |
|----|------|
| E3.6–E3.8 | Element market cargo · WANT-1 · observer routes/refineries **landed** |
| E3.9 | Treaty allow-lists for element ids |
| I5.x | SHARE/NOTE/FUT/PERP instrument lifecycle residual |
| Court | Rename Earth-culture line names (operator only) |
| Deep deeds | more domain leaves still status+verb (partial: planetary prospect/assay/refine fire; **diplomatic local seal/renounce/fence fire**, 2026-08-12 — `diplomatic local <treaty grammar>` delegates to the one home) |

### Landed this pass

| ID | Land |
|----|------|
| **C1.7** | Stellar capacity game — `stellar_capacity_board` · `bench star capacity` |
| **C1.6** | NPC `chemistry_step` disposition-routed (seat · catalog · compound · iso) |
| **P8.1** | Shared AP budget — build · prospect · research · quote · year-voice refill |
| **P8.2** | Trajectory · hunger · rail on observer (`arc` view · state uplink) |
| **P8.3** | Labor force from habitats + clones |
| **P8.4** | Closed-ruins history (`observer ruins`) |
| **P8.5** | AP capacity ruled table constants |
| **Pol.1–3** | Polity root · `bench polity` dual law · NPC `identity_step` |
| **Deep** | `bench planetary prospect|assay|refine` fire deeds |

### Landed: H1.3 desk deep-links

Every desk pane carries a **hub** strip and desk» shortcuts into hub domains:

| Pane | Primary (`h`) | Craft twin (`hc`) |
|------|---------------|-------------------|
| Trade | `trade books` | `bench trade` |
| Diplomatic | `diplomatic local` | `bench diplomatic` |
| Routes | `trade rails` | `bench econ` |
| Arbitrage | `trade books` | `bench trade` |
| Fleet | `trade fleet` | `bench fleet` |
| EXCH | `trade books` | `bench offices` |
| Grow | `bench planetary` | `bench build` |
| Defend | `intel opsec` | `bench doctrine` |
| Explore | `intel research` | `bench research` |

`desk» h` / `hub` runs primary; `hc` / `craft` runs the twin. Any hub verb still types through the normal command path.

---

## 4. CLI entry points

| Command | Surface |
|---------|---------|
| `bench` | build + science + dual craft twins |
| `intel` · `intelligence` · `kb` | knowledge activity |
| `trade` · `trades` | circulate activity |
| **`diplomatic`** · `diplomacy` · `diplomatics` · `diplo` | diplomacy activity |
| **`polity`** · `line` · `identity` | line identity (optional 5th · vent open) |
| `bench diplomatic` · `bench diplo` | diplomacy **craft** |
| `bench chem` · `biology` · `physics` · `math` · `research` | science **craft** |
| `bench star capacity` | C1.7 stellar lines game |
| `polity ap` | P8.1 shared AP board |
| `help bench` · `help intel` · `help trade-hub` · `help diplomatic` · `help polity` | maps |

Barter form preserved: `trade <id> with <world> <id> deal <n>`.
