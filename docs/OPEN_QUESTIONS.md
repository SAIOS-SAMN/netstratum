# Open questions — unresolved research vectors

**Status:** research / not law. Everything here is an *open* direction,
recorded so a good idea is not lost and a rejected one is not silently
revived as if settled. Nothing in this file is inscribed law; a vector leaves
this file only by landing under the BUILD_ORDER process, or by being formally
closed with its verdict. If a proposal here conflicts with an inscribed law
(Charter, the SEALs, an NS spec), the inscribed law wins until a landing says
otherwise.

---

## Q1. Holographic-GPU field engine & Proof-of-Useful-Work mesh

**The proposal (2026-07-26).** Replace the DAG/chronicle + NS-5/NS-7 stack
with a GPU-accelerated "holonomic holographic field" engine: treat the
spatial-thermal continuum as a continuously differentiable manifold, compute
sector sub-volumes on GPU tensor cores, reach consensus by *boundary
stitching* between neighbouring sectors, and make the physics simulation
itself the Proof-of-Useful-Work. Settlement proofs would be "holonomy /
sheaf-cohomology" checks over sector boundaries instead of succinct proofs
over a trace.

**Verdict: rejected as an architecture replacement; kernels salvaged
elsewhere.** It cannot replace the inscribed stack, for reasons that are
load-bearing, not tunable:

1. **GPU float breaks exact-rational determinism (Charter §3.2) and SEAL B.**
   IEEE-754 addition is not associative; two GPUs — or two schedules on one
   GPU — diverge in rounding. That is fatal to bit-exact cross-node replay.
   "Spectral rational grids (Chebyshev/Fourier over integer fields)" is
   self-contradictory: those are floating-point approximation methods. Exact
   arithmetic on GPU reduces to finite-field bignum work — which is what real
   ZK provers already do.
2. **Holonomy ≠ tamper-evidence.** A flat connection (Ω=0) gives
   path-independence, not a tamper-evident history. Determinism (same replay)
   and tamper-evidence (no undetected rewrite) are different guarantees; the
   second needs commitments — i.e. the chronicle. Discarding block history
   violates SEAL B directly.
3. **Boundary-stitching is a security hole.** Matching neighbours on a shared
   boundary proves *consistency*, not *interior integrity* — a node can
   compute a false interior while keeping its edges clean. The claim that
   the mesh rejects interior cheating is false in general.
4. **AdS/CFT misapplied.** The holographic principle is a specific
   gravity/CFT duality; it does not make arbitrary game-state bookkeeping
   reconstructable at O(1) from a 2D boundary.
5. **Homology proves structure, not history.** Betti numbers can show "a
   route exists" or "a loop is closed" but cannot prove "≥ N joules delivered
   by tick T". Persistent homology is also expensive; the "beats ZK on
   throughput, independent of tick count" claim compares different problems.
6. **PoUW's open problem is verification.** "The simulation is the useful
   work" is an old, attractive idea; it stalls because useful work must be
   cheaply verifiable and hard to fake, which leads back to needing a
   succinct proof. Boundary-stitching does not close this.

**Kernels kept (already inscribed):**

- Domain decomposition — sectors to nodes, exchange boundary data — is
  standard and already the spirit of NS-5. Kept as engineering, not as
  "holography" or free consensus.
- Succinct settlement proofs over exact-rational inequalities (`A/B ≥ C/D`) via a
  real stark / multilinear prover (Lasso, Jolt, HyperPlonk) — inscribed at
  `05_THERMAL_BALANCE.md` §7.3 as the buildable version of the "topological
  proof" idea.

**What could reopen this vector, honestly:** a deterministic exact-rational GPU
arithmetic path (finite-field, reproducible across hardware) *plus* a real
succinct proof of each interior transition (not boundary agreement). At that
point it is no longer "holographic field magic" — it is GPU-accelerated ZK
over the existing chronicle, which is a legitimate performance project that
does not touch the SEALs. That is the form in which it may return.

---

## Q2. Reopening paths for the sink-dual rejections (05 §1.4)

Three rejections from the sink-dual draft carry lawful ways back in;
recorded so a flat "no" doesn't bury a good tuning lever.

1. **Passive sink minting — reopened by distinction.** `radiator_base`
   is already a passive, ungated sink (every orb self-cools; landed in
   THERMAL-1). What stays earned is *bankable/exportable* capacity
   (`S_cool`). Baseline passive, tradeable earned. Reopen trigger:
   balance shows the cold start too punishing without a baseline.
2. **Banked-store decay — reopenable as receipted demurrage.** Banked
   state never mutates silently (SEAL B), but decay may accumulate as
   workspace and settle in one receipt at collect/transfer (the
   VentSettled shape), or be held off by storage upkeep. Reopen trigger:
   balance wants banked joules not to be a free eternal store outside
   Glasskeeper vaults.
3. **A global conservation sheet — reopenable at N6.** Per-side
   invariants (05 §1.4) stay the orb-level ground truth; they compose
   into a universe-level audit (Σ encased + Σ vented = Σ excess;
   Σ absorbed = Σ sink-consumed) checked at mesh consensus (NS-5), not
   imposed as an orb law.

Closed for good: un-receipted anything (SEAL B — spills are workspace
until `VentSettled` seals them, which is already the design); the
terminology rulings (charter §8); section numbering (mechanical).

---

## Q3. Unitary morphodynamics for the ideology axes

**The proposal (operator relay, 2026-07-27).** "Upgrade to unitary
morphodynamics" so that love, compassion, morality, and integrity live as
continuous unitary field transformations over civilization state.

**Verdict: rejected as an engine change; the four concepts landed instead
as the measured dual cohesion ledger** (`09_IDEOLOGY.md`; implementation
on the ideology board, BUILD_ORDER §3). Two load-bearing walls, the Q1
verdict's shape:

1. **The determinism hazard.** Continuous/unitary transforms imply
   floating-point field dynamics; IEEE-754 arithmetic is not associative
   and diverges across CPU/GPU architectures and schedules — fatal to the
   no-float law (Charter §3.2) and to SEAL B byte-exact replay, exactly
   as Q1's GPU holonomy was.
2. **The subjectivity hazard.** There is no deterministic CFE transition
   function for "love" as a continuous field that is not invented,
   arbitrary physics — the scalar/scored provenance refusal returning as a
   field.

**Kernels kept (inscribed at 09):** the four concepts themselves —
dual-entry receipted ledgers with ratios derived at read (09 §1); friction
as real entropy riding the landed VENT-1 machinery (09 §3.2); ideology
export as ordinary covenant state machines (09 §3.3).

**Addendum — the terminology resolution (operator ruling, 2026-07-29).**
The reopening path below was taken, and the vector is resolved without
re-litigating the verdict: "unitary" stays rejected (continuous unitary
transforms imply float operator arithmetic — wall 1 stands), but
**morphodynamics is admitted in discrete form** as **Discrete Gauge
Morphodynamics** — discrete group actions on the gauge fiber of
TUniversal{S, G, T}, exact-rational, wrapped on the AEON twisted torus
(11 §2.0), executed only as receipted `GaugeShifted` deeds (tag 42).
Elevation is a gauge climb gated on three exact chronicle readings
(thermodynamic settlement · four-C loop coherence · the interstellar
crossing) — the full law is `12_UNION.md` §6. The torsion is structural,
never a continuous field equation; no float leakage, SEAL B intact.

**What could reopen this vector, honestly:** an exact-rational, replayable
discrete dynamic — richer covenant state machines, exact-rational policy
evolution rules — at which point it is no longer "morphodynamics", just
more CFE state machinery under the existing laws. For genuinely continuous
fields, the same door as Q1: a deterministic exact-arithmetic path plus
succinct interior proofs. Closed for good: any stored mutable "alignment
score" (the provenance refusal); tokenized live reputation (09 §3.4 — a
token snapshot forks SEAL B's truth; NS-7 may anchor covenant *outcomes*,
never the ratio).

---

## Q4. The ideology board's recorded deferrals (post-landing, 2026-07-27)

The board landed whole (TRADE-1 → IDEO-1, BUILD_ORDER §3). These
deferrals were recorded in the landing entries; gathered here so each
carries its lawful way back in.

1. **The commensuration table beyond weight-1.** Every cohesion-book
   posting weighs all resources at 1 (`commensurate_weight`, the 09 §1
   placeholder). Reopen trigger: balance shows book-gaming by cheap
   resources (a diamond gift and an iron gift posting equal compassion).
   The table stays exact-rational and lands as a ruleset revision, law
   for replay under a version bump — never a silent retune.
2. **Recency weighting on the books.** The eight counters are cumulative
   forever; an ancient breach never fades. Reopen path: per-era or
   windowed readings derived at the observer (the derived-never-stored
   law holds — the counters stay cumulative; only the reading windows).
   Reopen trigger: balance wants redemption arcs.
3. **Civilization-scoped composition and the universe audit.** The books
   are per-orb; a faction's reading is composition over its orbs, and
   the universe-level sheet (Σ books, Σ entropy_vented incl. friction)
   joins Q2.3's mesh audit at N6 — checked at consensus, never imposed
   as an orb law.
4. **The deferred covenant templates (09 §3.3).** The delivery-quota
   pact landed as the template; the Compassionate Covenant (auto-routing
   surplus to crisis-struck neighbours) and the Bond Covenant (severance
   requires notice, else slash — coupling tag 34's severed flag to a
   pact condition) await a covenant-conditions landing. Also deferred
   there: target-side bond severance (v1: the source cuts) and
   absorption counting toward covenant quotas (v1: deliberate
   deliveries only — transfers and bond settlements).
5. **The SINK-1 research/throttle gates — already boarded.** TECH-1
   (10) is the carrier: sink capture's research gate binds to a
   crystallized node and the throttle-α policy action lands beside it
   (10 §5). Nothing to reopen here; the pointer closes the loop opened
   at the VENT-1 landing.
6. **The journal reads the books.** The Chronicle Feed (08) may now
   read the landed cohesion counters and friction receipts lens-side
   under its lens law (09 §6; MIRROR/ANTI-MIRROR deprecated, 12 §2c) —
   a journal follow-on for the coder node, by relay.

---

## Q5. "Converged" carries two opposite meanings across lanes

**Raised by the lens lane, 2026-08-02, as a relay — not a proposal to
rename anything.** Recorded here so the collision is visible before it
produces a real defect, and so the lens lane's own handling of it is not
mistaken for a ruling on anyone else's.

### The three senses standing in the tree today

```text
  1. CONVERGE-1 / era convergence          ~100 uses   LANDED LAW (07)
     ledger::converge, conductor, 07_ERA_CONVERGENCE.md
     "annex the archived timelines" — a world is CONVERGED when it has
     been adopted into the universe timeline. A COMPLETED ACTION.

  2. rfa-core's strike                     1 ruling    LANDED
     "the word `converged` is struck from this crate — zero says the
     relations compose, never that anything was finished or emptied."
     The word was removed because it wrongly implied FINISHED.

  3. lens::propagation::Motion::Converging  12 uses    LANDED (19_LENS §5)
     A DIRECTION: a volume drawing inward, the sink pole against
     Diverging's source pole. ONGOING, with its own positive magnitude.
     Never "negative expansion".
```

**Sense 1 means finished. Sense 3 means moving, and says nothing about
finishing. Sense 2 already struck the word once for exactly this
reason** — which is the strongest evidence the collision is real rather
than pedantic: another lane hit it independently and resolved it by
deletion.

### Where it actually bites

The trap is not the two spellings. It is that the naive reading of
"converged" — *it has settled, it stopped moving* — names the arm of
`Motion` that is explicitly **not** the converging one:

```text
    Motion::Diverging { by }    sources — outward
    Motion::Converging { by }   sinks   — inward, and STILL MOVING
    Motion::Still               nothing moved
```

A volume that stopped is `Still`. A volume that is converging is in
motion. Anyone carrying sense 1's intuition into a lens reading will
read `Still` as "converged" and `Converging` as "finishing", and both
are wrong.

### What the lens lane has already done, and its limit

`Motion::Still` is documented as *"Not converged — a volume that did not
move may be at rest or may be stuck, and this reading refuses to
presume"*, and the rule is pinned:
`propagation_pins::a_still_volume_is_not_called_converged` and
`kinematic_pins::a_still_reading_does_not_claim_convergence`.

**That is the whole of what this lane can enforce.** Those pins bind the
lens's own vocabulary and reach nothing in `ledger` or `conductor`.

### The lens lane's recommendation

**Rename nothing.** Specifically:

- **CONVERGE-1 should not be renamed.** It is landed law with its own
  spec document and ~100 call sites, and the cost of touching it is
  enormously larger than the confusion it causes. The word is also
  correct in its own domain: timelines genuinely do converge onto one
  axis, and that convergence genuinely does complete.
- **`Motion::Converging` should not be renamed.** The Diverging/Converging
  pair was ruled directly by the operator (*"invert to convergence"*,
  2026-08-02) and deliberately mirrors `Flux { sources, sinks }` in
  `rfa-core::capacity`. Renaming it would break the parallel that
  correction was made to create.

Two correct uses of one word in two domains is not a defect. **The
defect would be a third use that conflates them**, and that is what this
entry exists to prevent.

### The open question, addressed to the ledger and conductor lanes

Is there anywhere that a *timeline* convergence result is, or could be,
fed into a *motion* reading — or the reverse? The lens lane has looked
at its own side and found none: `lens` does not import
`ledger::converge` and `Motion` is derived only from two `Degree`
values. **The lanes that own `ledger` and `conductor` are the ones who
can answer for their side**, and this entry is the ask.

If the answer is "nowhere", this closes as a documentation note. If the
answer is "somewhere", that site is a real bug and should be raised
under BUILD_ORDER rather than fixed in passing.

---

## Q-T / Q-R / Q-S — Offices, rails, STAMP (opened 2026-08-07)

Recorded from `30_OFFICES_AND_RAILS`, `31_ELEMENT_COMMERCE`, `32_STAMP_MARKET`.
Not law until ruled and landed — except **coded defaults** below.

| ID | Question | Default / land status |
|----|----------|------------------------|
| **Q-T1** | Seat widths for T4–T9 per domain? | **Coded Wave-1:** 16 for T4–T8; T9 unbounded |
| **Q-T2** | Is T5 Crucible **required** to build ElementRefinery, or only advantaged? | **Coded Wave-1:** advantage only; zone needs deepening |
| **Q-R1** | Must R3 require a live strand process, or sealed strand receipt file offline? | Sealed receipt file OK for offline demo — *open* |
| **Q-R2** | STAMP mint schedule: crystal-only convert vs multi-collateral? | Crystal convert + Vault Notes against elements — *open* |
| **Q-S1** | Crystal→STAMP conversion curve? | Placeholder linear at landing; **ratchet forbids undervied dials** — derive/mint at S4.2 |
| **Q-S2** | Demurrage on unbacked STAMP on/off? | Off in v1 — *open* |
| **Q-S3** | Migrate FORWARD-1 into Note or keep parallel? | **Coded Phase-4:** parallel (Notes landed; Bond/Forward stay) |
| **Q-S4** | Minimum clears before a perp index lists? | N=8 clears in window — *open*; Perp opens on last-clear in v1 |

---

## Q-D — Defense axioms · covert terraform · espionage (opened 2026-08-08)

Recorded from `36_DEFENSE_AXIOMS.md`. Not law until ruled and landed.

| ID | Question | Lean / default |
|----|----------|----------------|
| **Q-D1** | CovertTamper = pressure scalar vs half PlanetSited? | Pressure scalar + threshold event |
| **Q-D2** | Multi-actor tamper blame? | Last unveiled actor + INF witness |
| **Q-D3** | Plasmic ATM medium? | Compose thermal + optional field TLV |
| **Q-D4** | Espionage vs veiled cost? | Compose VeilToll |
| **Q-D5** | Escort risk vs pair_melt? | Separate typed residual (not melt rewrite) |
| **Q-D6** | Elder as combat? | Keep warden puzzle-only (no combat) |
| **Q-D7** | Digital vs cyber split in wire? | One CYB domain, two sub-channels |
| **Q-D8** | Clade as mechanic or voice? | Voice + residual priority only (15_TAXON law) |
| **Q-D9** | MKT as 7th domain vs CYB sub-channel? | **Ruled:** first-class **MKT** domain (`36` §1.3, §10) |
| **Q-D10** | Mini-games separate currency? | **Ruled:** suites only; no shadow score (`36` §10.3–10.4) |
| **Q-D11** | N for clear seal before mark? | Compose Q-S4 (default 8) at P5.32 land |
| **Q-D12** | Circuit breaker duration? | Year-voice (60) or until explicit resume — *open* |

Authority: `36_DEFENSE_AXIOMS.md`. Board: `TASKS_TRADE_FLEET_DESK` P5.*

---

## Q-F — Trade desk · fleets · alignment (opened 2026-08-07)

Recorded from `33_TRADE_FLEET_AND_DESK.md`. Not law until ruled and
landed — defaults below are **implementation defaults** if a phase must
ship before a full ruling.

| ID | Question | Default if forced early |
|----|----------|-------------------------|
| **Q-F1** | θ advance per tick vs year-voice only? | **Coded Phase-2:** year-voice — `ORBIT_PHASE_TICKS = 60` |
| **Q-F2** | Hull η stack with route η? | Product of complements (pin Phase 3) |
| **Q-F3** | Scrap: one pool vs per-resource? | **One scrap pool** from iron/alloy burn v1 |
| **Q-F4** | Can T3 mint hulls without yard zone? | **Yard required**; office optional advantage |
| **Q-F5** | R1 bourse without T6 Exchange? | Yes, **Spot-only**; T6 unlocks full stack |
| **Q-F6** | Lease vs own hull on commission? | **Own** v1; lease Phase 5+ |

Task board: `archive/TASKS_TRADE_FLEET_DESK.md`. Authority: `33_TRADE_FLEET_AND_DESK.md`.

