# JSON is display-only — never authority

**Status:** LAW (completed 2026-08-09).  
**Supersedes:** any practice that treated `state.json` / registry scrapes as colony truth.

---

## 0. One sentence

**Editing a JSON file must not change the world.** The world is the verified receipt chain.  
**Production write path emits no JSON.** Residual disk JSON is migrate-once then delete.

---

## 1. Authority vs display

| Layer | Carrier | Trusted for |
|-------|---------|-------------|
| **Authority** | `chronicle.tlv` | All deeds, SEAL B resume, cross-world load |
| **Machine mirror** | `state.tlv` | Committed orb wire (banks, zones, famine, elements, STAMP) |
| **Topology** | `universe.card`, `starchart.card`, `galaxies.card` | Chart / clock — never balances |
| **Identity** | `var/registry/<sigil>.civ` | Registry cards (not colony banks) |
| **Session config** | `var/operators/<name>` (plain text) | Operator seat *claim* — not a receipt |
| **~~JSON display~~** | ~~state.json / chronicle.jsonl~~ | **STRUCK** — no longer written |

```text
chronicle.tlv  ──replay──►  World  ──scribe──►  state.tlv + world.card
     ▲
     └──────── nothing lawfully reads JSON back ────┘
```

---

## 2. Binary / non-JSON carriers (landed)

| Surface | Path | Format |
|---------|------|--------|
| Registry | `var/registry/<sigil>.civ` | TLV (`ledger::card::civ`) |
| Universe | `var/worlds/universe.card` | TLV |
| Starchart | `var/worlds/starchart.card` | TLV entries |
| Galaxies | `var/worlds/galaxies.card` | TLV (render of ledger) |
| World card | `var/worlds/<w>/world.card` | TLV |
| Orb mirror | `var/worlds/<w>/planets/<p>/state.tlv` | NS-1 wire |
| Chronicle | `var/worlds/<w>/chronicle.tlv` | SEAL B authority |
| Directives | `var/worlds/<w>/inbox/*.dir` | TLV (`card::dir`) |
| Solves | `var/worlds/<w>/solves/*.sol` | TLV |
| Journals | `var/worlds/<w>/journal.jrn` | framed TLV |
| Fiber | `var/worlds/fiber/*.fiber` | TLV |
| Operators | `var/operators/<name>` | plain text lines |
| Agent wire | CLI stdout | line protocol (`ok`/`actor`/…) |
| Web index | `/data/index.txt` | plain text |

**Not written:** any `*.json` / `*.jsonl` colony or registry file.

One-shot **migrate-on-read** in the engine still accepts legacy JSON and rewrites TLV, then deletes the JSON. Disk bulk cleanup:

```bash
python3 tools/migrate_off_json.py --root var
```

---

## 3. Engine rules (implemented)

1. **CLI** loads far worlds only via `chronicle.tlv` + verify.  
2. **CLI stress** reads famine & vent from resumed World.  
3. **Registry / chart / universe / directives / solves** are binary cards.  
4. **Operator profiles** are plain text (legacy `.json` migrates on login).  
5. **Observer** material columns from **`state.tlv` only** — zero `json.loads`.  
6. **Scribe** writes `state.tlv` + `world.card` only; strips leftover JSON.  
7. **Steward** queues `*.dir` TLV directives only.  
8. **Webserve** serves binary suffixes + HTML; index is plain text.  
9. **Observatory HTML** decodes TLV in the browser — no `JSON.parse` of colony data.  
10. **`pen::JsonWriter`** retired (crate is an empty placeholder).  
11. **`lens::render_state` / `render_receipt`** struck — helpers only (`universal_clock`, `hex`).

---

## 4. What an attacker can still do

| Attack | Effect | Mitigated by |
|--------|--------|--------------|
| Edit `state.tlv` alone | Local disk forgery of mirror | Resume from `chronicle.tlv` is law |
| Edit `chronicle.tlv` digests | Breaks SEAL B | `persist::load` / `verify` refuse |
| Hex-edit `.civ` / `.card` | Corrupt identity cache | Not colony banks; re-create via engine |
| Edit operator profile | Local UI claim only | Mint/station re-check chain |
| Drop a fake `.json` file | Ignored by production readers | Not on the read path |

---

## 5. Zero JSON on the write path

| Was | Now |
|-----|-----|
| state.json / world.json / chronicle.jsonl | **state.tlv** · **world.card** · **chronicle.tlv** |
| registry `*.json` | **`*.civ`** |
| starchart.json / universe.json | **`*.card`** |
| inbox `*.json` | **`*.dir`** |
| solves `*.json` | **`*.sol`** |
| journal.jsonl | **journal.jrn** (framed TLV) |
| fiber/*.json | **fiber/*.fiber** |
| agent response JSON | **line wire** |
| /data/index.json | **/data/index.txt** |
| pen::JsonWriter / lens JSON render | **retired / struck** |

`wall.html` remains HTML presentation of journals (not a law file).

Engine `ledger::extract` keeps flat-JSON **migrate helpers only** — not a write path.

---

## 6. Smoke

```bash
python3 tools/migrate_off_json.py --root var

test -f var/worlds/<world>/chronicle.tlv
test -f var/worlds/<world>/planets/*/state.tlv
test ! -f var/worlds/<world>/planets/*/state.json
test ! -f var/worlds/<world>/journal.jsonl
test -f var/worlds/starchart.card
test -f var/registry/*.civ

python3 tools/observer.py arc --plain | head
python3 tools/steward.py list

# production readers: no json.loads of colony data
! grep -R 'json.loads' tools/observer.py
```
