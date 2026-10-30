# Binary carriers — JSON is not architecture

**Status:** law.  
**One sentence:** Colony truth is the verified receipt chain. Disk carriers
are TLV / binary cards / plain text. **JSON is not a format the system speaks.**

---

## 1. Architecture

```text
chronicle.tlv  ──replay──►  World  ──scribe──►  state.tlv + world.card
     ▲                         │
     │                         ├── registry/*.civ
     │                         ├── universe.card · starchart.card · galaxies.card
     │                         ├── inbox/*.dir · solves/*.sol
     │                         └── journal.jrn · fiber/*.fiber
     └──── nothing reads JSON ─┘
```

| Layer | Carrier | Role |
|-------|---------|------|
| **Authority** | `chronicle.tlv` | Deeds, SEAL B resume, converge |
| **Machine mirror** | `state.tlv` | Committed orb wire (banks, zones, famine, …) |
| **Identity** | `registry/<sigil>.civ` | Civilization cards |
| **Topology** | `universe.card`, `starchart.card`, `galaxies.card` | Clock + chart |
| **Steward** | `inbox/*.dir` | Directives |
| **Proof** | `solves/*.sol` | Certificates |
| **Journal** | `journal.jrn` | Framed year voice |
| **Session** | `operators/<name>` | Plain text seat claim (not a receipt) |
| **Agent** | line wire | `ok` / `actor` / `deed` — not JSON |
| **Web** | `/data/index.txt` + binary files | No JSON API |

**Not carriers:** `*.json`, `*.jsonl`. The engine does not write them, does
not load them, and does not migrate them. A leftover file on disk is
residue — ignored. The scribe may delete known residues when it rewrites
a world so the tree stays clean; that is hygiene on the write path, not
a second truth.

---

## 2. Rules

1. **Resume / far worlds / converge** — `chronicle.tlv` only.  
2. **Material boards** — `state.tlv` only.  
3. **Registry / chart / universe / directives / solves** — binary cards only.  
4. **Operators** — plain text only.  
5. **No `JsonWriter`, no `render_state` JSON, no `json.loads` of colony data.**  
6. **No external migrate scripts** — architecture is the law; cleanup is not a subsystem.

---

## 3. What an attacker can still do

| Attack | Effect | Mitigated by |
|--------|--------|--------------|
| Edit `state.tlv` | Local mirror forgery | Resume from `chronicle.tlv` |
| Edit `chronicle.tlv` | Breaks SEAL B | `persist::load` / `verify` refuse |
| Hex-edit `.civ` / `.card` | Corrupt identity/topology cache | Not colony banks |
| Drop a fake `.json` file | Ignored | Not on any read path |

---

## 4. Smoke

```bash
test -f var/worlds/<w>/chronicle.tlv
test -f var/worlds/<w>/planets/*/state.tlv
test -f var/worlds/starchart.card
test -f var/registry/*.civ

python3 tools/observer.py arc --plain | head
python3 tools/steward.py list
```
