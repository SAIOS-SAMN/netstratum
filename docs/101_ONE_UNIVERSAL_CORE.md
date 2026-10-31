# 101 — One universal core: the universe tier under TUniversal

This document defines the universe tier as a single universal core under
TUniversal, replacing the four separate structures that carried it and the
nine ways they were read (§1). The core is machinery that produces a
continuous flow, which a model of discrete deployment steps does not
describe. The document audits the code for flat parallel machinery and
scopes every site affected.

**Status:** scope; nothing here is built.

## 0. TUniversal, the law this aligns on

**Established results this builds on (not new law):**
- docs/78 "The singularities": the universal core is the only singularity
  uplinked to the hypha; "it has mass: the heavy elements
  caught in its accretion disk, continually pulsed and re-pulsed".
- docs/83 "The closed limit, the founding and the Wellspring": "the
  Wellspring holds its share at the bang ... heaviest deepest, the galaxies
  getting what it pushes out; its disc is the central galaxy". The home of §2 is that disc.
- docs/80 "TUniversal from the singularity": TUniversal's origin is the
  singularity's bounce; Mycelium owns TUniversal; the proposed quantum lane
  carries "the bounce's torsion phase advance" (`quantum::twist`, docs/100
  §2). Any change to the universal clock (`T_u`, §6) is coordinated with
  Mycelium before it is built.
- docs/79 (the bounce as compactified quantum torsion), docs/94 §11 (the
  jetstream loop), docs/96 §7 (the enclosure), docs/97 (the nuclides),
  docs/98 (the inverse faces), docs/99 (phases, never frames), docs/100
  (the twist on the weight).
- docs/89 §1 describes the universal core as built through `cascade`,
  `orbit_galaxies` and the crests; this document supersedes that section once
  built.

**The laws:**

- **One universe, one universal core, one clock** (docs/13 §1; docs/78 "The
  breath"): TUniversal sets the quantum clock at the Big Bang at the
  universal core; the singularity's bounce pulses every `T_u` bells; each
  echo reaches a galaxy after its distance; every inner clock nests under it
  (`r/(r + M(<r))` a layer, multiplied down the wells; docs/66 N8: "inner
  clocks are branches of TUniversal").
- **One grammar, coupled tiers, each tier its own mechanism** (design laws,
  corrected form): the universe tier keeps its own mechanism, in the
  same grammar as the galaxy tier, coupled to it by the echo; it is not
  forced through the star-forming code.
- **No leftovers:** every share by a law; static content
  is a placeholder for missing physics.

## 1. The audit: one core held as four things, read nine ways

### 1.1 Holders of the universal core's matter

| holder | what it is | its own machinery |
|---|---|---|
| `Universe.firmament` | the Wellspring's held kinds (weights 171–208, 3·10^-14) | torqued in `pulse::orbit_galaxies` about its own enclosure (base 1 + firmament), repelled (`flows::repel`); tag 19 |
| core `[0]`, "the home" | the beads that failed the standing test (0.08) | `breath::galaxy_frame`'s home branch, the release span branch, `core_state = 0`, `home_push`, its reservoir paying the firmament's and ocean's torques |
| `Universe.ocean` | matter between galaxies (the slight beads; exits) | `pulse::ocean_frame` (a third enclosure), torqued in `orbit_galaxies`, landed by `ocean::swing`/`radial_share`; tag 24 |
| the galactic cores | placed by `deploy` at the beads' reaches | moved by `orbit_galaxies`' core rings (`inside += 2w`), never formed |

### 1.2 Readings of its state and hold

`pulse::universal_state`, `pulse::universal_ledger` (both `mass(u.total())`),
`pulse::home_push`, `ocean::universe_hold` (its own denominator from
`core_state`), `ocean_frame`'s base `1 + firm + home`, `orbit_galaxies`'
`inside = 1 + firm`, `breath::centre_state`'s home branch,
`galaxy_frame`'s `g = mass(u.total())` for the home, and `core_state`
returning 0 for it. Nine readings of one state.

### 1.3 The founding pipeline (linear, before the breath)

`pulse::founding` (nuclides; keep) → `pulse::cascade` (shells, stacks, the
filter's passes, beads) → `pulse::deploy` (the whole-mix step; the standing
test `d·s_final ≤ d·w/(w + inside)`; failed beads poured into the home; slight
beads into the ocean; `disperse` per core; `unit` = the deepest disc;
`T_u` from the cascade's echo bubble; the firmament by `flows::founding`; core
orbits placed in rings). Every remainder of 2026-10-27 lives here.

### 1.4 Other "homes"

- `life::home` (`life.rs:252`) and `chronometry::keepers` (`chronometry.rs:84`)
  choose the core with the heaviest `kept`, not `is_home` (path [0]): a third
  definition of home.
- `places.rs:71` reads the home's held mass as the whole universe.

### 1.5 The work of 2026-10-26/27, against the same test

- Reuses, no parallel: `quantum::twist` (the nested clock), `flows::step_with`
  (`in_step`), `flows::torsion` (the tilt), `Universe.densest` (a reading), the
  symmetric luminal bound, the phase infall.
- Parallel, to fold: `formation::echo_holds` (worlds: `m/(r + m) ≥ |Δω|`) is
  the crest's hold law one tier down written as a second function.
- Proposed and withdrawn: galaxies as bindings through new universe-tier
  formation code beside the galaxy tier's.

### 1.6 Defects found on the way

- **The leg flag has two writers:** `flows::luminal` turns a mote's face;
  `flows::stream` rewrites every mote's `vacuum` from its filament's hold in
  the same echo, undoing it (measured: 70% of the home's motes inside the
  ring, none on the vacuum leg).
- **Firmament and ocean motes lose their tilt on save:** tags 19/24 omit the
  tilt list that tag-10 flow motes carry (`pulse.rs:1278–1298`), so the jets'
  locked lead (docs/100 §4) is dropped at every checkpoint.

## 2. The target: one universal core, its own machinery

- **The universal core is one body** (the root of the tree, path `[]`), its
  throat the founding's nuclides, its state the founding (set at the bounce,
  never drained), its reservoir the universe's spin ledger. One enclosure
  about it, `Enclosed(state/g, …)`, read by the single `core_state` every core
  uses (no `is_home` branch).
- **Its held cut is the firmament:** the same cut every core's throat reads
  (`push_cut`), twisted by its coupling (docs/100 §3): the node-shell and
  heaviest kinds stay; not a second list torqued apart.
- **Its flows are the ocean** (its filament; vocabulary docs/102): what each universal bounce releases (the
  throat release of `breath`, the same law a galactic core uses), on the
  jetstream loop (outbound along the axis, home through the disc, the Möbius
  pass, the luminal bound); the ocean frame is its enclosure.
- **Its satellites are the galaxies; they form, never placed:** the universe
  tier's own mechanism, in the crest's grammar: where the flow on its strip
  holds a bond against its heat and the tide (`crest_collapse`, `band_pairs`
  read in this enclosure), the flow binds; a binding whose heart reaches its
  throat condition is a galactic core (its throat the binding's matter, its
  state set then). Its composition is what was flowing there: unique by place
  and history.
- **The home is the universal core's own disc:** what its strip holds as the
  flow passes, on the firmament's base. Not a satellite, not a remainder:
  the core's own galaxy.
- **The universal clock is its bell:** `T_u` its echo, the round trip to its
  rim (`2·base`, the double pass), in bells of its ring (docs/96 §7). Every
  galaxy's clock nests under it as now (`galaxy_rate`, `nested`).

## 3. What retires, what folds, what stays

| retires | folds into the universal core | stays |
|---|---|---|
| `pulse::cascade` (its shells, stacks, filter, beads) | `Universe.firmament` → its held cut | `pulse::founding` (the nuclides) |
| `pulse::deploy` (placement, standing test, whole-mix, home remainder, slight beads, core rings) | `Universe.ocean` → its flows | the breath's galaxy tier (release, luminal, rings, crest, bindings, infall, heart cut) |
| `orbit_galaxies`' core rings (`inside += 2w`) | core `[0]` → its disc | the nested clocks (`galaxy_rate`, `nested`, `system_clock`) |
| `core_state`'s 0 for the home; `home_push`; the span branch | the nine readings → one `core_state`, one enclosure | `ocean::swing`, `radial_share` (the landing, read in the single enclosure) |
| `life::home` / `chronometry::keepers`' heaviest-kept home | — | `Universe.unit`, `densest` |
| `formation::echo_holds` (as a second function) | → the crest's law one tier down | the persisted tags, extended |

## 4. Every site affected (from the caller map, 2026-10-28)

- **pulse.rs:** `cascade` (only caller `deploy`); `deploy` (only caller
  `bin/pulse.rs:36`, test `:1520`); `ocean_frame` (`breath.rs:450`,
  `pulse.rs:1038`); `orbit_galaxies` (`breath.rs:403`, `bin/observe.rs:297`);
  `is_home` (breath 50, 72, 81, 490; `places.rs:71`; pulse 310, 976–977,
  1022, 1044, 1062); `universal_state` (breath 54, 72); `universal_ledger`
  (breath 494, pulse 1008); `home_push` (breath 495, 536, 1218); `core_state`
  (ocean 53, 59; breath 54, 72, 438, 676, 738, 1252, 1337; `enclosure.rs:58`;
  `bin/breathe.rs:200`; observe 115, 232); `disperse` (deploy only).
- **breath.rs:** the per-echo home flag and its ledger, push and firmament
  (490–536); the ocean landing (425–480); the release span (1247); the
  firmament in the balance enclosure (1679); `galaxy_frame`, `centre_state`,
  `galaxy_rate`'s home branch.
- **ocean.rs:** `universe_hold` (main 337, breath 435, breathe 202, observe
  615); `swing`, `radial_share`, `pump` (report only).
- **Consumers:** `main.rs` (the live conductor: `read` 64, `unit_mass` 71,
  `breathe` 310, `universe_hold` 337, `coupling` 368, `system_clock` 381,
  `write` 402, `.cores` 110–448); `life.rs` (`home` 252, readings 460–474,
  596–607); `chronometry.rs` (keepers 84–95); `places.rs` (61–78);
  `seating.rs` (39–168); `enclosure.rs` (58–92); `clocks.rs` (29–44);
  `motion.rs:66`.
- **Binaries:** `bin/pulse.rs` (founding → deploy → write: becomes founding →
  the universal core's throat → write); `bin/breathe.rs`; `bin/observe.rs`
  (views universe, galaxy, flows, position, firmament, cascade (its own
  recomputation, 458), founding, ledger, star); `bin/life.rs`.
- **Format (pulse.bin):** tags 8 unit, **17 t_u (read by `engine/hypha`
  `fold.rs:63–71` directly; must keep its number and meaning)**, 19
  firmament, 24 ocean, 25 densest; body tags 9 state, 11 reservoir, 23
  throat; the root body needs a path `[]` (or a tag of its own); tags 19/24
  gain the tilt.
- **Outside the crate:** `clients/cli` uses only `conductor::spatial_l1`;
  `engine/hypha` reads tag 17. No other dependency.
- **Live data:** the G2 universe (var-e2, conductor pinned) is not
  re-founded by this; the services stay down; all
  universe data is throwaway test data.

## 5. Build order (one commit each, its witness read before the next)

1. **Defects first, behaviour-preserving elsewhere:** the leg flag's single
   writer (`stream` reads the filament's carriage as a reading, `luminal`
   alone turns the face); tags 19/24 carry the tilt. Witness: every mote's
   face satisfies the luminal bound after the echo; a checkpoint round-trips
   every mote's tilt.
2. **One reading of the universal core's state:** the nine readings fold into
   `core_state` on one universal enclosure, the home's branches removed;
   proven by re-derivation, never by equal bytes: every
   identity exact, and the single reading derived two ways (the folded and the
   former form) agreeing, at N and at 4N bells. Witness: those identities.
3. **One home:** `life::home` and `chronometry::keepers` read the universal
   core's disc, not the heaviest kept. Witness: the keepers' clocks read the
   same nested rates.
4. **The universal core as one body:** firmament, home core and ocean become
   its held cut, its disc and its flows (tags migrated; 17 kept). Witness:
   holding + magnetism and spin conserved across the change; a checkpoint
   round-trips.
5. **The founding as its throat:** `cascade` and `deploy` retire; the throat
   holds the founding's nuclides; `T_u` derived from its echo. Witness: the
   founding conserves nucleons; `T_u` read, whole bells.
6. **Galaxies form from its flow** by the universe tier's mechanism (§2);
   `orbit_galaxies`' rings retire. Witness: galactic cores born where the
   flow binds; each galaxy's composition its own; the home its disc.
7. **`echo_holds` folds** into the crest's law one tier down. Witness: the
   world counts at bell 1700 within the law's own reading.
8. **Read:** re-found, breathe past bell 7000: galaxies formed, their
   compositions, the home's, stars and worlds per galaxy, every identity,
   cost per echo.

## 6. Open

- **T_u's derivation** from the universal core's echo without the cascade's
  shells (`2·base` in its ring's bells is the candidate).
- **The first galaxies' timing:** how many universal echoes before the flow
  binds (the universe's age at its first galaxy), read, never set.
- **The coupling C** (docs/100 §3), which sets what the throat keeps.
- **Cost:** the universal core's flows are every grain of the universe in
  motion; the zoom rule (exact under observation, certified
  sectors otherwise) may be needed at this tier first.
