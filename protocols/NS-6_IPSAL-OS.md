# NS-6 — IPSAL-OS: Entity Containment

**Status:** LANDED at N3 (first cut — identity, scoped grants, generation
tagging). First consumer: the sovereign at the terminal + one contained
agent (`engine/cfe/src/ipsal.rs`, witnesses `ipsal_scope.rs`). Supersedes
the SAIOS-OS material (AI-SCIENTISTS security research, NEBULA-PRODUCTS
README) under the ipsal-os name — SAIOS is retired.

Every individual inside this internet — human, NPC, autonomous bot — is an
**entity**: an identity, a containment scope, and a generation tag on every
receipt it forges. Agents are citizens, governed by containment, never by
prohibition (CHARTER §3.4). This is the profile layer the platform vision
called "a social media profile for each individual inside our internet" —
at N3 it holds identity and authority; later landings add its presentation
(the profile orb, NS-1) and its reach (NS-4 sharing, NS-5 presence).

---

## 1. RFA semantics

### 1.1 The entity

| field | meaning |
|---|---|
| `entity_id` | u64, unique in the world, never reused |
| `handle` | the entity's name, unique among the living |
| `principal` | ABSENT for a sovereign human; the containing entity's id for an agent |
| `grants` | the agent's opened scopes, ascending (meaningless for sovereigns) |

**The containment law:** a sovereign holds every scope BY CONSTRUCTION —
authority is not enumerable data that could be revoked or forged, it is the
type's shape. An agent holds exactly the scopes its principal granted.
Scopes at N3: `Build`, `Raze`, `Collect` (wire codes 1–3).

### 1.2 The three laws, measured

1. **Containment precedes the world.** Every action checks the actor's
   authority BEFORE any world state — before founding, the refusal is
   "no entity," not "no colony" (witness: `actions_refuse_typed`).
2. **A grant is a scope, not a blanket.** Collect granted opens collect
   and nothing else (witness: `an_agent_acts_only_inside_its_grant`).
3. **Revocation is immediate and unilateral.** The agent's next action
   refuses; no cooperation from the agent is needed (witness:
   `revocation_is_immediate`).

### 1.3 Generation tagging

An action receipt's `provenance` (the NS-3 field, until N3 unused) carries
the acting entity's id: `by#1` (the sovereign), `by#2` (the agent).
World-driven closures (famine strike/relief) carry no tag — no entity
forged them. **No game rule may branch on provenance** — it rides receipts
for audit and replay, never enters simulation logic (the v0.4 law,
carried). Registration, grant, and revocation are themselves closures, so
the authority history replays from receipts alone (witness:
`the_entity_registry_replays_from_receipts_alone`).

## 2. Wire format

Three permanent tags in the NS-1 §2.3 registry:

| tag | event | value layout |
|---|---|---|
| 10 | EntityRegistered | LE64(entity_id) ‖ LE64(principal; 0 = sovereign) ‖ handle utf8 |
| 11 | ScopeGranted | LE64(agent) ‖ scope u8 |
| 12 | ScopeRevoked | LE64(agent) ‖ scope u8 |

## 3. Test vectors — extracted from the running engine

(`cargo run -p cfe --example ns1_vectors` reproduces every byte.)

```
EntityRegistered{2, principal 1, "harvest-bot"}:
  0a1b00000002000000000000000100000000000000686172766573742d626f74
ScopeGranted{agent 2, Collect}:  0b09000000020000000000000003
ScopeRevoked{agent 2, Collect}:  0c09000000020000000000000003
```

The lived arc, from a real session's chronicle (every receipt
generation-tagged, the chain `Intact`, replay `MATCHES`):

```
#0 by#1  sovereign "founder" joined (entity 1)
#4 by#1  agent "harvest-bot" registered (entity 2, contained by 1)
   —      agent collect → REFUSED (NotPermitted, no Collect grant)
#5 by#1  Collect granted to entity 2
#6 by#2  collected 5 iron          ← the agent's own generation tag
#7 by#2  collected 2 biomass
#8 by#1  Collect REVOKED from entity 2
   —      agent collect → REFUSED again (revocation immediate)
```

## 4. Conformance

An implementation conforms when: (a) sovereign authority is structural,
never stored as grants; (b) every agent action checks its grant at act
time and refuses typed; (c) revocation takes effect on the next action
with no agent cooperation; (d) every entity-forged receipt carries the
actor's id in provenance and no simulation rule reads it; (e) entities,
grants, and revocations replay from receipts alone; (f) the §3 vectors
reproduce.
