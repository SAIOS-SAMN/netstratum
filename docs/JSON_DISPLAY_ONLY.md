# JSON is display-only — never authority

**Status:** LAW (landed 2026-08-09).  
**Supersedes:** any practice that treated `state.json` / registry scrapes as colony truth.

---

## 0. One sentence

**Editing a JSON file must not change the world.** The world is the verified receipt chain.

---

## 1. Authority vs display

| Layer | Carrier | Trusted for |
|-------|---------|-------------|
| **Authority** | `chronicle.tlv` | All deeds, SEAL B resume, cross-world load |
| **Machine mirror** | `state.tlv` | Committed orb wire (banks, zones, famine, elements, STAMP) |
| **Display export** | `state.json`, `chronicle.jsonl`, journals, wall HTML | Humans and boards **only** — deletable, regenerable |
| **Session config** | `var/operators/<name>` (plain text) | Operator seat *claim* — not a receipt; engine re-checks chain |
| **Topology cache** | `starchart.json` lines | Chart coordinates only — never balances |

```text
chronicle.tlv  ──replay──►  World  ──scribe──►  state.tlv + state.json (write-only)
     ▲                                              │
     │                                              ▼
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
| Operators | `var/operators/<name>` | plain text lines |

**No longer written:** `state.json`, `world.json`, `chronicle.jsonl`, registry `.json`, `starchart.json`, `universe.json`, `galaxies.json`, inbox `.json`, solve `.json`.

One-shot **migrate-on-read** still accepts legacy JSON files and rewrites TLV, then deletes the JSON.

---

## 3. Engine rules (implemented)

1. **CLI** loads far worlds only via `chronicle.tlv` + verify.  
2. **CLI stress** reads famine & vent from resumed World.  
3. **Registry / chart / universe / directives / solves** are binary cards.  
4. **Operator profiles** are plain text (not JSON).  
5. **Observer** material columns from **`state.tlv` only**.  
6. **Scribe** writes `state.tlv` + `world.card` only; no colony JSON.  
7. **Steward** queues `*.dir` TLV directives.

---

## 4. What an attacker can still do

| Attack | Effect | Mitigated by |
|--------|--------|--------------|
| Edit `state.tlv` alone | Local disk forgery of mirror | Resume from `chronicle.tlv` is law |
| Edit `chronicle.tlv` digests | Breaks SEAL B | `persist::load` / `verify` refuse |
| Hex-edit `.civ` / `.card` | Corrupt identity cache | Not colony banks; re-create via engine |
| Edit operator profile | Local UI claim only | Mint/station re-check chain |

JSON is not “enabled” for these surfaces. Casual edit-as-text is gone.

---

## 5. Zero JSON on the write path

| Was | Now |
|-----|-----|
| journal.jsonl | **journal.jrn** (framed TLV) |
| fiber/*.json | **fiber/*.fiber** |
| agent response JSON | **line wire** (`ok`/`actor`/`deed`/…) |
| /data/index.json | **/data/index.txt** |

`wall.html` remains HTML presentation of journals (not a law file).

`pen::JsonWriter` remains only for unit tests of the pen crate — no production call site.

## 6. Smoke

```bash
test -f var/worlds/<world>/chronicle.tlv
test -f var/worlds/<world>/planets/*/state.tlv
test ! -f var/worlds/<world>/planets/*/state.json
test ! -f var/worlds/<world>/journal.jsonl

python3 tools/observer.py arc --plain | head
python3 tools/steward.py list
```
