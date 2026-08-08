# HUBS — bench · intel · trade · diplo

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
| `diplo` | **`bench diplo`** | councils · bonds · forwards · seats · routes · escorts |

Aliases (same craft surfaces):
- `bench intel-craft` · `bench knowledge-craft` → `bench intel`
- `bench trade-craft` · `bench circulate` → `bench trade`
- `bench diplomacy` · `bench diplomatics` · `bench pact` → `bench diplo`

Cross-links from activity roots: `intel craft` / `trade craft` / `diplo craft` → matching `bench S`.

---

## 1. Roots

| Root | Role |
|------|------|
| `bench` | **BUILD** components (includes dual-law craft twins) |
| `intel` | **KNOWLEDGE activity** |
| `trade` | **CIRCULATE / run** components |
| `diplo` | **DIPLOMACY activity** (local · stellar · mesh) |

### Domain map

```text
bench          BUILD COMPONENTS
  planetary    bio · chem · terraform · medium · thermal · balance
  build        zones · yard · refinery · regen
  star         commission · site · age · rings
  estate       chart · parcels · appraise · ladder
  chem         bonds · iso · catalog (craft)
  econ         seats · instruments · routes · rails
  offices      seat exchange|vault|assay|anchor · assay challenge
  fleet        yard craft · mint hull · scrap
  doctrine     posture · KIN prep ranks
  medium       ATM envelope · reinforce · tamper
  assay        T7 court craft
  ── dual law craft twins ──
  intel        forge chain · instruments · chem → root intel
  trade        econ · fleet · offices umbrella → root trade
  diplo        council · bond · seats · routes → root diplo

intel          KNOWLEDGE ACTIVITY
  research     constellation · forge · practice · wants
  knowledge    tech · chem catalog · stash · practices · instruments
               · literature · books (orb accounts)
  ops          ladder · probe · exfil · sabotage · scan
  opsec        veil · ward · detection toll
  link         council · license · bond · forward · absorb
  culture      literature aspire/claim
  craft        → bench intel

trade          CIRCULATE / RUN (market · haul — not treaties)
  books        offers · instruments · market war · suite
  stamp        mint · aperture (unit of account activity)
  element      element aperture · element charter · refine path run
  fleet        capacity · assign · escort · risk (run)
  rails        promote · anchor import/export
  vault        deposit · withdraw · margin
  aperture     resource · element · stamp haul
  desk         interactive suite
  craft        → bench trade
  (trade diplo|stellar → redirect to diplo root)

diplo          DIPLOMACY ACTIVITY
  local        pair treaties · renounce
  stellar      crossed · war walls · aperture · license
  mesh         councils · bonds · forwards standing
  craft        → bench diplo
```

### Flow law

```text
bench crafts ──► intel operates knowledge tools / bases
             ├► trade runs market / haul / vault
             └► diplo runs treaties / stellar / mesh

Dual:  intel  ←activity→  bench intel  (craft)
       trade  ←activity→  bench trade  (craft)
       diplo  ←activity→  bench diplo  (craft)
```

---

## 2. Landed expansion

Wired into four roots (no stone from prior gap table left hub-silent):

| Domain | Root |
|--------|------|
| medium · thermal · balance | bench planetary / medium |
| offices · assay court · fleet craft | bench |
| culture · books · opsec · wants | intel |
| stamp · element | trade |
| diplo local · stellar · mesh | **diplo** root |
| dual law craft twins | **bench intel · bench trade · bench diplo** |

**Diplomacy split:** `diplo local` = pair treaties; `diplo stellar` =
crossed/shielded, war walls, aperture haul, license leap.  
`trade diplo` / `trade stellar` redirect to diplo root.

---

## 3. Still open

| ID | Open |
|----|------|
| C1.7 | Stellar capacity **game** (lines_of as play) |
| H1.3 | Desk panes deep-link into hubs |
| P8.x | AP budget / anti click-to-win |
| Polity | genesis/taxon/mint as optional 5th root |
| Deep deeds | more domain leaves still status+verb (not all fire deeds) |

---

## 4. CLI entry points

| Command | Root / surface |
|---------|----------------|
| `bench` | build + dual craft twins |
| `intel` · `intelligence` · `kb` | knowledge activity |
| `trade` · `trades` | circulate activity |
| `diplo` · `diplomacy` · `diplomatics` | diplomacy activity |
| `bench intel` | knowledge **craft** |
| `bench trade` | market/haul **craft** |
| `bench diplo` | diplomacy **craft** |
| `help bench` · `help intel` · `help trade-hub` · `help diplo` | maps |

Barter form preserved: `trade <id> with <world> <id> deal <n>`.
