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
(the profile orb, NS-1) and its reach (NS-4 aperture sharing, NS-5 presence).

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

---

## 5. Lineage — the CCL Entity Hierarchy (ancestry, recorded)

The sovereign/agent containment above descends directly from the **Entity
Hierarchy** of the Convergence Coding Language (CCL) —
`/opt/ipsal-autic-v0.1/repos/convergence-engine/CCL.md`, Part IIb — carried
here so the ancestry is not lost. CCL's tiers and its governing principle:

| CCL tier | CCL role | netstratum NS-6 |
|---|---|---|
| **Founder** (was "Witness") | the generating element; *"it does not command, it seeds"* | the **sovereign** — authority is structural, not granted |
| **Derived** (was "Elder") | first-order derivation, composed from the Founder | a **contained agent** (`principal` = the sovereign) |
| **Emergent** (was "Child") | second-order emergence, the frontier | a further-contained agent (agents may contain agents) |

The load-bearing inheritance is CCL's rule **"no entity governs another;
self-consistency is the design principle."** NS-6 keeps it exactly: a
sovereign contains, it does not command; an agent acts only inside the scopes
its principal opened (§1.2). netstratum **collapses** CCL's three creation
tiers into one recursive relation (sovereign ▷ agent ▷ agent…) — the tier
*count* is not inscribed, but the seeds-not-commands principle and the
upward-authority / downward-grant flow are. CCL's naming-discipline half lives
separately, as the CHARTER §8 Terminology Law.

### 5.1 "Sovereign" — the classifier (kept, defined)

"Sovereign" is retained deliberately. Its loaded reading — the "sovereign
citizen" movement — exists and is acknowledged, but in this domain the term's
dominant meaning is **self-sovereign identity** (a mainstream identity concept:
an entity that controls its own identity under no central authority), and CCL
originally chose "sovereign" as the NEUTRAL replacement for the genuinely
loaded `master` / `root` / `admin`. In netstratum it is not a political claim;
it is a structural classifier.

**Definition.** A **sovereign** is a self-governing entity that answers to no
external principal — it holds its standing by its own establishment, never by
grant, and sits at the top of its containment tree. Sovereignty is **earned by
establishment, never assigned as a rank.** The classifier applies at every
altitude of the §6 hierarchy:

- **Individual (landed):** a founding entity with no `principal` (§1.1).
  Authority is structural; a contained agent is never sovereign. (Unchanged —
  the wire format and witnesses hold.)
- **Species / civilization:** a **sovereign civilization** is one established
  enough to govern itself — the classifier for extremely sophisticated,
  deeply-established groupings, not merely any founded one.
- **Star / star-power:** a sovereign power holding a star system on its own
  standing (a higher landing).

The unifying thread: sovereignty always means **self-governing
establishment** — at the individual level the threshold is "no principal"; at
the species/star level it is genuine, demonstrated establishment. An entity
that answers to a principal is a contained **agent**, never sovereign.

### 5.2 "Polity" and "automaton" — the autonomous-civilization vocabulary

"Bot" is retired (operator ruling, 2026-07-27): it imports *mechanical
servant of a scripter*, false under the citizenship doctrine ("agents are
citizens; automation on a leash"). The lawful pair, each importing only true
invariants:

- **POLITY** — an autonomous collective will that ARISES FROM a group's
  record and acts on its own. The conductor's civilization-players are
  polities literally: temperament from the registry axes, voice from the
  journal fold, character from the cohesion books — constituted by the
  chronicle, scripted by no one. A guild of players acting as one will (N6)
  is an polity forming in the original sense.
- **AUTOMATON** — Greek `αὐτόματος`, "self-willed" (Homer's self-moving
  gates; Talos): crafted operative WITH its own motion, bounded by granted
  scopes. Exactly an NS-6 contained agent — no entity commands it (§5), it
  acts of its own will inside its grants.

The two-tier mapping: the polity holds the will (sovereign-side); its
automaton does the receipted labor in-world (`{slug}-conductor`, scoped
grants). Lore file: `<var>/polities.json` (legacy `bots.json` adopted once
on boot).

## 6. The place hierarchy — star ▷ planet ▷ colony

Distinct from the authority hierarchy (§1, §5) is the **place** hierarchy: the
containment of *locations*, normative here and repeated in
`03_SUBSTRATE_CONTRACT.md` §8.

- A **STAR** (a star system: one sun and the bodies bound to it) is the
  simulation container — ONE clock, ONE chronicle, ONE receipt chain (the
  replay unit today, the mesh/consensus unit at N6). This tier was formerly
  called "world"; it is renamed **star** because the sun is what the planets
  orbit and what fixes their thermal regime (`05_THERMAL_BALANCE.md` §1). The
  chronicle stays **per-star**, never per-planet.
- A **PLANET** is a settled body orbiting the star. **A star contains MANY
  planets** (cardinality 1★ : N planets).
- A **COLONY** is one civilization's settlement on a planet. **A planet hosts
  MANY colonies** (cardinality 1 planet : N colonies), including colonies of
  different civilizations sharing one body.
- A **CIVILIZATION** (species-level identity) spans **many stars**.

**Cardinality law (target):** `civilization 1:N star 1:N planet 1:N colony`.
Today the runtime cardinality is still degenerate (1★ = 1 planet = 1 colony);
the tiers are named now so multi-colony (D3 terrain) and multi-star (N6 mesh)
grow without renaming. "World" is retired as a container term; where "world"
still appears colloquially it means *a planet* (e.g. rare-world deeds = a
scarce planet), never the container.

**Naming debt (extends §8):** the engine struct `World` and the
`worlds/<slug>/` path still carry the old container name; they rename to
`Star` / `stars/<slug>/` at the multi-planet landing, before N6. Code
identifiers are untouched until that landing.

## 7. StarClass — the enumerator of known stars (suns)

Every star orb carries a `StarClass`. The class fixes the sun's luminosity
`L`, which drives every bound planet's `T_eq⁴` and therefore its thermal band
(`05_THERMAL_BALANCE.md` §1–§3). The main-sequence spectrum (Morgan–Keenan) is
the core enumerator; remnants, giants, and the lore exotic follow.

**Main sequence (the common suns):**

| code | class | colour | ~surface T | luminosity vs Sol | thermal consequence (05) |
|---|---|---|---|---|---|
| 1 | **O** | blue | >30,000 K | ~10⁴–10⁶ L☉ | Furnace dominates; equilibrium band pushed far outward; rare |
| 2 | **B** | blue-white | 10–30,000 K | ~25–30,000 L☉ | vast Furnace; hyper energy yield |
| 3 | **A** | white | 7.5–10,000 K | ~5–25 L☉ | wide Furnace, mesic band far out |
| 4 | **F** | yellow-white | 6–7.5,000 K | ~1.5–5 L☉ | mesic band beyond 1 unit |
| 5 | **G** | yellow (Sol) | 5.2–6,000 K | ~0.6–1.5 L☉ | balanced; mesic band near 1 unit |
| 6 | **K** | orange | 3.7–5.2,000 K | ~0.08–0.6 L☉ | mesic band pulled inward |
| 7 | **M** | red dwarf | 2.4–3.7,000 K | ~10⁻⁴–0.08 L☉ | most common star; band hugs the star; deep Frost everywhere else |

**Remnants, giants, and exotic:**

| code | class | archetype | thermal consequence (05) |
|---|---|---|---|
| 8 | **L/T/Y** | brown dwarf (sub-stellar) | near-Frost at every radius; compute-favourable |
| 9 | **D** | white dwarf (remnant) | low total output, fading; tight, shrinking bands |
| 10 | **RG/SG** | red giant / supergiant (evolved) | enormous luminous shell; Furnace swallows inner orbits |
| 11 | **N** | neutron star / pulsar (remnant) | beamed, extreme, hazardous; narrow usable shells |
| 12 | **WR** | Wolf–Rayet (hot, mass-losing) | violent Furnace; short-lived, resource-rich |
| 13 | **✦** | **Luminal White Hole** (lore exotic) | the ultimate energy anchor (`05_THERMAL_BALANCE.md` §1.3) |

Codes are the wire enumerator (a permanent NS-1 tag at the star-orb landing);
values here are placeholder parameters that tune at balance time. `StarClass`
is descriptive substrate that feeds the exact-rational thermal math — it is
never itself a game rule in bytecode (SEAL C).

## 8. The lived arc since N3 (recorded 2026-07-30, coder-02)

What grew on this protocol without changing it — the first cut held:

- **Scopes stand at the three inscribed** (`Build`/`Raze`/`Collect`),
  and TEN mutating verbs now pass their gates (chart, zone, raze,
  collect, transfer, seal_bond, sever_bond, seal_covenant, absorb,
  scan — audited 2026-07-30: ten verbs, ten `permitted` checks, zero
  ungated). The scanner rides Collect; a dedicated scan scope awaits
  an operator ruling (the N4 deferral).
- **The sovereign's face at the terminal:** profiles under
  `var/operators/` — `login <name>` mints or switches, bare `login`
  lists with the active marked, `whoami`, `continue` resumes the
  active profile's last saved star. No passwords by design: the
  containment boundary is the machine, not a credential.
- **The lived containment:** every polity star runs its
  `{slug}-conductor` automaton (three scopes granted at genesis) and a
  steward draining the inbox — §5.2's two-tier mapping operating on
  every live star, every beat.
- **Cross-star identity stays deferred** to N5/N6; PLANNER-1's
  authorship-attribution deferral rides it.
- **Open, awaiting operator ruling** (recorded 2026-07-30): whether
  IPSAL additionally names the translation bridge — today translation
  is G3's territory (frames as coboundaries, measured loss) and
  presence is NS-5's; nothing in this protocol claims them. If ruled,
  the binding lands here and in those chapters together.

