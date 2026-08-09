# HUBS — bench · intel · trade · diplomatic

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
Advanced tech is **gated** until that evidence exists — you cannot buy synth R2 with local STAMP alone.

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
| `bench` | **BUILD** components (includes dual-law craft twins + science) |
| `intel` | **KNOWLEDGE activity** |
| `trade` | **CIRCULATE / run** components |
| `diplomatic` | **DIPLOMACY activity** (local · stellar · mesh); alias `diplo` |

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
  local        pair treaties · renounce
  stellar      crossed · war walls · aperture · license
  mesh         councils · bonds · forwards standing
  craft        → bench diplomatic
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
```

---

## 3. Still open

| ID | Open |
|----|------|
| C1.7 | Stellar capacity **game** (lines_of as play) |
| P8.x | AP budget / anti click-to-win |
| Polity | genesis/taxon/mint as optional 5th root |
| Deep deeds | more domain leaves still status+verb (not all fire deeds) |

### Landed: H1.3 desk deep-links

Every desk pane carries a **HUB** strip and desk» shortcuts into hub domains:

| Pane | Primary (`h`) | Craft twin (`hc`) |
|------|---------------|-------------------|
| TRADE | `trade books` | `bench trade` |
| DIPLOMATIC | `diplomatic local` | `bench diplomatic` |
| ROUTES | `trade rails` | `bench econ` |
| ARBITRAGE | `trade books` | `bench trade` |
| FLEET | `trade fleet` | `bench fleet` |
| EXCH | `trade books` | `bench offices` |
| GROW | `bench planetary` | `bench build` |
| DEFEND | `intel opsec` | `bench doctrine` |
| EXPLORE | `intel research` | `bench research` |

`desk» h` / `hub` runs primary; `hc` / `craft` runs the twin. Any hub verb still types through the normal command path.

---

## 4. CLI entry points

| Command | Surface |
|---------|---------|
| `bench` | build + science + dual craft twins |
| `intel` · `intelligence` · `kb` | knowledge activity |
| `trade` · `trades` | circulate activity |
| **`diplomatic`** · `diplomacy` · `diplomatics` · `diplo` | diplomacy activity |
| `bench diplomatic` · `bench diplo` | diplomacy **craft** |
| `bench chem` · `biology` · `physics` · `math` · `research` | science **craft** |
| `help bench` · `help intel` · `help trade-hub` · `help diplomatic` | maps |

Barter form preserved: `trade <id> with <world> <id> deal <n>`.
