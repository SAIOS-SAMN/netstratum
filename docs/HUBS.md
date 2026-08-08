# HUBS — bench · intel · trade

**Status:** living map (update with every hub land).  
**Law:** hubs are presentation + deed routing over existing world verbs. No second physics.

---

## 0. BEFORE this expansion (state of the board)

### Roots (already correct)

| Root | Role |
|------|------|
| `bench` | **BUILD** components |
| `intel` | **KNOWLEDGE activity** |
| `trade` | **CIRCULATE / run** components |

### What was wired (pre-expansion)

| Root | Domains present | Deep? |
|------|-----------------|-------|
| bench | planetary · build · star · estate · chem · econ · doctrine | chem deep; rest thin |
| intel | research · knowledge · ops · link | research/ops/link boards |
| trade | books · fleet · rails · vault · aperture · diplo · desk | status + verb lists |

### Stones left unturned (code existed, hub silent)

| Code | Gap |
|------|-----|
| `offices` seat · assay court | no bench offices |
| `medium_defense` | no medium domain |
| `fleet` mint as craft | only trade fleet run |
| `thermal` / age_star | buried |
| `literature` · `books` | not in intel knowledge |
| veil · ward · scan | not intel opsec |
| WANT-1 | not surfaced |
| stamp / element commerce | thin under trade |
| diplomacy local vs stellar | one flat diplo leaf |
| KIN convoy run | partial under fleet |
| balance / regen | thin under planetary |

### Decision

- **No fourth root** in this land (polity/identity deferred).
- Wire **all** of the above into the **three roots**.
- Diplomacy split: **local** (same-system pair play) vs **stellar** (crossed, aperture, war wall, license haul).

---

## 1. AFTER expansion — complete domain map

```text
bench          BUILD COMPONENTS
  planetary    bio · chem · terraform · medium · thermal · balance
  build        zones · yard · refinery · regen
  star         commission · site · age · rings
  estate       chart · parcels · appraise · ladder
  chem         bonds · iso · catalog (craft)
  econ         seats · instruments list craft · routes · rails
  offices      seat exchange|vault|assay|anchor · assay challenge
  fleet        yard craft · mint hull · scrap (component)
  doctrine     posture · KIN prep ranks

intel          KNOWLEDGE ACTIVITY
  research     constellation · forge · practice · wants
  knowledge    tech · chem catalog · stash · practices · instruments
               · literature · books (orb accounts)
  ops          ladder · probe · exfil · sabotage · scan
  opsec        veil · ward · detection toll
  link         council · license · bond · forward · absorb
  culture      literature aspire/claim (alias of knowledge culture)

trade          CIRCULATE / RUN
  books        offers · instruments · market war · suite
  stamp        mint · aperture (unit of account activity)
  element      element aperture · element charter · refine path run
  fleet        capacity · assign · escort · risk (run)
  rails        promote · anchor import/export
  vault        deposit · withdraw · margin
  aperture     resource · element · stamp haul
  diplo        local treaties · renounce · pair factors
  stellar      interstellar: crossed · war walls · aperture · license haul
  desk         interactive suite
```

### Flow law

```text
bench crafts ──► intel operates knowledge tools / bases
             └► trade runs market / haul / vault / diplo
```

---

## 2. Landed expansion (this commit)

Wired into three roots (no stone from §0 table left hub-silent):

| Domain | Root |
|--------|------|
| medium · thermal · balance | bench planetary / medium |
| offices · assay court · fleet craft | bench |
| culture · books · opsec · wants | intel |
| stamp · element · diplo local · **stellar** | trade |

**Diplomacy split:** `trade diplo` = local pair treaties; `trade stellar` =
crossed/shielded, war walls, aperture haul, license leap path.

## 3. Still open after expansion

| ID | Open |
|----|------|
| C1.7 | Stellar capacity **game** (lines_of as play) |
| H1.3 | Desk panes deep-link into hubs |
| P8.x | AP budget / anti click-to-win |
| Polity | genesis/taxon/mint as optional 4th root |
| Deep deeds | more domain leaves still status+verb (not all fire deeds) |

---

## 3. CLI entry points

| Command | Root |
|---------|------|
| `bench` | build |
| `intel` · `intelligence` · `kb` | knowledge |
| `trade` · `trades` | circulate |
| `help bench` · `help intel` · `help trade-hub` | maps |

Barter form preserved: `trade <id> with <world> <id> deal <n>`.
