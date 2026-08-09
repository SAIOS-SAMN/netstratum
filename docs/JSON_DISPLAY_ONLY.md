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

## 2. What may still be JSON (and why)

| File | Why JSON is OK | Must not |
|------|----------------|----------|
| `state.json` | Human snapshot from lens | Be read by engine for banks/famine/deeds |
| `chronicle.jsonl` | Append-only **display** of receipts (each line carries `tlv` hex) | Be the sole resume carrier |
| Registry cards | Identity **cache** until G1 fully binds every field to genesis receipts | Override sealed genesis axes |
| Steward inbox `*.json` | Directive files (become real only as NS-6 scope-checked deeds) | Edit state directly |
| Solve certificates | Proof blobs for observer depth | Replace DiscoveryMinted receipts |

---

## 3. Engine rules (implemented)

1. **CLI** loads far worlds only via `chronicle.tlv` + verify.  
2. **CLI stress / inbox** reads famine & vent from resumed World, never `state.json`.  
3. **CLI `conductor_founded`** uses MESH pen / `spine/` — never registry JSON via state scrape.  
4. **Operator profiles** are plain `var/operators/<name>` (`station` / `world` lines). Legacy `.json` is migrated on login/save.  
5. **Observer** material columns (banks, famine, zones, elements, STAMP, labor) decode **`state.tlv` only**. No row without TLV.  
6. **Scribe** diverges on **`chronicle.tlv` digests** when present; `chronicle.jsonl` is write-only rebuild/append of display.

---

## 4. What an attacker can still do

| Attack | Effect | Mitigated by |
|--------|--------|--------------|
| Edit `state.json` banks | Fool a tool that still scrapes JSON | Engine ignores it; observer prefers TLV |
| Edit `state.tlv` alone | Local disk forgery of mirror | Resume/`verify` from `chronicle.tlv` refuses mismatch |
| Edit `chronicle.tlv` digests | Breaks SEAL B parent chain | `persist::load` / `verify` refuse |
| Edit operator profile station | Local UI claim only | Mint/station gates re-check chain |

---

## 5. Still open (honest residual)

- **Registry JSON** remains the civilization identity cache; G1 sealed axes on the chronicle are the binding copy where sealed. Full registry→chain is a later land.  
- **Starchart.json** remains topology export until a TLV chart lands.  
- **Steward directives** remain JSON files (become receipts when applied).  
- Optional: export integrity hash (terminal receipt digest over display) for board anti-fooling — not authority.

---

## 6. Smoke

```bash
# Engine must not need state.json for resume
test -f var/worlds/<world>/chronicle.tlv

# Observer material from TLV
python3 tools/observer.py arc --plain | head
# Rows present only when state.tlv exists

# Login writes plain profile, not JSON
# var/operators/<name> contains: station … / world …
```
