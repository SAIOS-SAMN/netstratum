# U DIRECTIVES — the unit ladder, the I-remnants woven in

**Status:** DRAFT work orders (2026-07-31). The continuation document
for the unit lane: every open U-rung with its code specifics, and every
kernel-invariant remnant (I4's bound, I5's bound, I6 whole, the I1–I3
coverage extension) woven into the rung that pays it. Cold-start here,
then `16_UNIT.md` (the law), `EQUATIONS.md` (the forms),
`engine/kernel/CHARTER.md` (the constitution). The working discipline
is unchanged: one landing at a time, red-first with recorded output,
spec + mechanic + vectors together, commit on explicit operator go.

## §0 State at writing (2026-07-31, session end)

Working tree holds, UNCOMMITTED, awaiting operator go:
- **U0 LANDED in-tree** — the spine judge fixture seats its actors
  (sovereign + contained hand 100 with Collect); the board's true count
  surfaced: **285/285 workspace green, clippy zero** (the old 240 was a
  fail-fast undercount).
- **PROPORTION-1 LANDED in-tree** — THE PROPORTIONAL LAW: `Σ R²` → `Σ |R|` in
  `rfa_core::{coherence_functional, conservation_measurement}`,
  `norm_squared` → `magnitude` (‖Δ‖₁), `cfe::query` scalar; witnesses
  moved 24→12 (RFA vectors, elasticity) and 29→7 (scanner); the tag-58
  wire vector RE-EXTRACTED from the running engine (`ns2_vectors`
  example) and inscribed in NS-2 §3; law docs amended (kernel CHARTER
  §3, SYSTEM_SPEC §0/§4, NS-2, 03_SUBSTRATE_CONTRACT §5).
- **Docs drafted:** `16_UNIT.md` (the unit law + §4a the singularity
  law), `EQUATIONS.md` (the one ledger), this file.

## §1 The board — where each I-remnant lands

| rung | pays | status |
|---|---|---|
| U0 | the seated hand (I2 honored in the spine fixture) | LANDED in-tree |
| U1 | **I4's bound** — `mint ≤ f(verify)`, the PoUW anchor, tag 86 | NEXT |
| U1b | **I5's bound** — flat decay ≤ rate·ease·elapsed, `last_touch` (SCHEMA_V3) | after U1 |
| U2 | **I6 whole** — orbit_lock, the fold gate, the singularity witness | after U1 |
| W-A..W-D | **I1/I2/I3 coverage extension** — the 55 unjudged deeds, in four waves | parallel, any lane |
| U3 | UNIT-1 — the deployable boundary + constitution digest | after #36 |
| U4 | the Ruling Boxes come home (operator words) | gated on operator |

Kernel coverage today: 8 of 63 deed types. `scope_of()` is already
exhaustive over all 63 (the compiler forces the scope decision); an
amendment = move the event into `covered()` + declare its lawful
conservation delta + red-first witness against `world.rs`.

## §2 U1 — KERNEL-VERIFY (the PoUW anchor; I4 completes)

**The split (prover free, verifier sealed).** New crate
`engine/solver-verify` holding exactly what a REPLAY needs and nothing
a SEARCH needs, moved from `engine/solver`: `grid` (Grid, Op, Program
and the apply path), `loader` (Task), `forge` (`forge_task`,
`Difficulty` — deterministic re-derivation from a digest), and
`certificate` (`SolveCertificate`, `seal`, `verify_certificate`,
`CertificateRead`). `solver` keeps `search`, `objects`, `volume`
enumeration, depends on `solver-verify`, and re-exports so no caller
breaks. The kernel imports `cfe` + `solver-verify` ONLY (the one-way
arrow holds; the kernel stays pure, total, no-I/O).

**THE REPLAY-COMPLETE LAW (derived this session — do not weaken).**
`verify_certificate` alone re-derives the HASH CHAIN; it does NOT
prove the program solves the task. A cheater can seal an intact chain
over a wrong output. The kernel-level verification is therefore the
full replay, all three or refuse:

    solver_verify::verify_work(work) -> Result<Difficulty, WorkBroken>
      1. re-forge:  task = forge_task(work.forge_digest, work.difficulty)
                    (pure, deterministic — no external data ever)
      2. re-apply:  work.program maps EVERY training pair exactly AND
                    the test input to work.output (the honest-withhold
                    law: one miss = refuse, never a guess)
      3. re-chain:  verify_certificate(work.certificate, task,
                    program, output) == Intact

**The deed — tag 86, claimed in the `dop.rs` header in the landing's
FIRST commit (the tag-claim convention; 86 verified next-free).**

    ClosureEvent::DiscoveryMinted {
        orb_id, resource, amount,          // the mint itself
        forge_digest: [u8; 32],            // the task re-derives from this
        difficulty,                        // the forge key
        program, output, certificate,      // the proof, riding WHOLE
    }

The receipt carries its proof whole — replay under SEAL B re-verifies
the work forever. NO change to `Step` (the deed is the carrier), NO
schema change (event-only; `banked` is already committed) — U1 touches
no codec bytes. **v1 work-shape: FORGE TASKS ONLY** (re-derivable from
the digest, chronicle-seeded). Corpus classes (ARC-2 files) need a
task-registry ruling — parked in Ruling Box 1.

**Kernel arms.** `covered()` gains `DiscoveryMinted`; `scope_of` →
`Collect`; `conservation` lawful_delta = `+amount`; and I4 completes:

    creation: amount ≥ 0                                (landed sign law)
              && verify_work(work) = Ok(difficulty)     (else WorkBroken{at_step})
              && amount ≤ mint_bound(difficulty)        (else MintExceedsWork{bound, claimed})

    mint_bound(_: &Difficulty) -> Ratio { ratio(0, 1) } // SEALED SHUT —
    // opens ONLY by Ruling Box 1 amendment (rate, shapes, split),
    // bumping kernel::VERSION. Mechanism first, economics by ruling.

New `Unlawful` arms: `WorkBroken { at_step: usize }` and
`MintExceedsWork { bound: Box<Ratio>, claimed: Box<Ratio> }` — both
carry the measurement whole.

**Witnesses (red-first, in order):**
1. valid forged work + ANY positive amount refuses `MintExceedsWork`
   with bound 0 carried — the sealed-shut law is itself witnessed;
2. a tampered chain refuses `WorkBroken` naming its exact `at_step`;
3. an intact chain over a program that MISSES one training pair
   refuses — the fabricated-render attack, dead;
4. a wrong forge_digest refuses (the task is not the one claimed);
5. zero-amount with valid work admits (the degenerate mint is lawful);
6. the deed round-trips tag 86 byte-exact; replay MATCHES.

**EQUATIONS row-claim:** update I4's row status in the same landing.

## §3 U1b — THE DECAY BOUND (I5 completes)

The bound `decay ≤ rate · ease · elapsed` needs `elapsed = stamp −
last_touch(orb, resource)`, and the kernel reads only (prior, next,
step) — so **`last_touch` must enter COMMITTED state**: a trailing
per-orb map, **SCHEMA_V3** (additive; v1/v2 still decode — the CODEC-1
convention). This is a schema amendment: coordinate on the codec lane
per §11 before landing. The existing `DemurrageSettled` wire form is
UNTOUCHED (one tag, one meaning, forever).

The sealed vocabulary — flat per-resource `rate` table and the `ease`
read — is RATIFIED FROM `world.rs`'s landed flat-demurrage behavior
(the regression-pinned ruling: never proportional, the 2^1100 lesson),
red-first, never invented. Witnesses: exact-at-bound admits; over-bound
refuses `DecayExceedsBound{bound, claimed}`; elapsed 0 admits only zero
decay; `last_touch` advances on settle (ratified against `world.rs`);
v2-state replay still reads whole.

## §4 U2 — KERNEL-FOLD (I6 whole; the singularity witnessed)

`rfa_core::orbit_lock(a, b, operators, k_max) -> Option<OrbitWitness>`
re-derived from the v0.1 ancestor's `fold.rs` (`/opt/repos` — consult,
NEVER port blindly). **TWO translation laws bind the derivation:** the
proportional law (any squared aggregator in the ancestor lands as L1) and
translate-never-transcribe (the RCF canonical form `≅_RCF` — PhiK
I^(4)'s hash — is derived as part of this landing). The kernel gains
its `rfa-core` dependency here (arrow still one-way).

Fold-class deeds (`Confluent`, `EraAnnexed`, `EraFolded`) enter
`covered()` under I6: a fold is lawful iff `orbit_lock = ∅`; a refused
fold's residue is THE EMISSION — and the U1×U2 seam is the law that
convergence-scale `DiscoveryMinted` pays ONLY the emitted residue
(16_UNIT §4: PoWC decides genuine, PoUW bounds the pay).

**Witnesses:** gauge-only difference folds; torsion-locked refuses with
the lock named; the refused fold's residue is the one mintable plane;
**the angle-free witness** (16_UNIT §4a) — the same pair under all
eight D4 frames and a gauge-coboundary shift yields the IDENTICAL
verdict and canonical residue; the C(Δ)=0 pair is the replication
witness, not consilience.

## §5 W-waves — the coverage campaign (I1/I2/I3 extension, 55 deeds)

The completeness rule stands: an event enters `covered()` only when
EVERY invariant that should govern it has landed. Each amendment is
RATIFIED against `world.rs` red-first (the `conservation_pins.rs`
model). The waves are disjoint deed families — parallel across lanes:

- **W-A, the authority deeds (~24, {I2, I3}):** ColonyFounded,
  ZoneBuilt, WorldCharted, ColonyCharted, StarCommissioned,
  PlanetSited, BondSealed, CovenantSealed, CouncilSealed,
  GaugeShiftedAt, TechLicensed, GenesisSealed, PracticeAdopted/
  Released, ProspectStruck, ElderWarded, VeilDrawn, InstrumentForged,
  RouteChartered, OfferPosted, LineSucceeded, ZoneRazed, WardLifted,
  VeilLifted. **CAUTION (derived):** several of these MOVE or CREATE
  value — `ColonyFounded` seeds the founding bank, `ColonyCharted`
  moves the 20-iron/10-biomass daughter reserves — so each amendment
  DERIVES its lawful_delta from `world.rs`, and the founding-reserve
  mints need a declared I4 arm (a chartered genesis mint, named, never
  a hole). `TaxonSealed` is coder-03's standing #42 — leave it theirs.
- **W-B, the value movers (~8, {I1, I2, I3}):** UpkeepSettled,
  OverflowAbsorbed, BondSettled, CovenantReleased, FrictionSettled,
  CrystalDrawn, AssayStruck, OfferCleared.
- **W-C, the world-driven consequences (~14, {I1 + sign/bound}):**
  FamineStruck/Relieved, VentSettled, OverheatStruck/Cleared,
  AbsorptionLapsed, CovenantSlashed, WantStruck/Relieved, StarAged,
  TechDiscovered, RouteLapsed, VoidShare, IncursionStruck.
- **W-D, the mesh/universe deeds (~8, protocol-law + I6-gated):**
  EraAnnexed, GalaxyObserved, HandoffSealed, GaugeShifted, Diverged,
  Confluent, EraFolded, CouncilClosed — the fold-class trio waits on
  U2; the rest ride their protocol law.

Phase 3 (the kernel as the LIVE gate — `world.rs` demoted to proposer,
refuse-unknown live) flips ONLY when `Unjudged` is unreachable.

## §6 U3 — UNIT-1 (the deployable boundary)

One binary = cli + kernel + spine + codec + cfe + physics + rfa-core +
solver-verify + the mesh door. OUTSIDE: conductor (a client of units),
the ledger's display halves (the physical ledger/lens split is #37's
opening move — U3 sequences after it or feature-gates around it),
tools. Surface `kernel::constitution_digest()` — `H(VERSION ‖
vocabularies ‖ derivation_rules ‖ schema)` (kernel CHARTER §3), the
EQUATIONS ledger among the hashed derivation rules (16_UNIT §5) — in
the mesh handshake: cross-version units refuse typed. **Acceptance:**
the unit binary founds, plays, SOLVES (one forge grind → certificate),
and uplinks against a second copy of itself (extend the
`ns5_two_nodes` pattern); a forged chain refuses at its break.

## §7 Sequencing (with the #36 seam)

    #36 (spine at TUniversal + adoption — coder-04's boarded rung;
         kernel-OBSERVED play: every deed chained, judged where
         covered, holes stamped judged=0, never hidden)
      → U1 (independent of #36 at the kernel level; lands anytime)
      → U1b (codec-lane coordination for SCHEMA_V3)  ∥  U2 (the fold
         gate — W-D's fold trio waits on it)
      → W-A..W-D  ∥  (parallel, any lane, disjoint families; W-D's
         fold trio lands with/after U2)
      → Phase 3 flip (Unjudged unreachable — ALL waves + U1b + U2
         complete → the kernel is the door)
      → U3 (after #36/#37) → U4 (the rulings open the mint)

Calibration note: after #36 + U1 the whole single-player + NPC economy
runs CHAINED with certificate-bounded minting refusing until ruled —
measure the synthetic economics there FIRST, then rule `f` against
observation, not assumption.

## §8 The Ruling Boxes (operator words — nothing else blocks)

1. **Box 1 (opens `mint_bound`):** the rate `f` per difficulty; the
   admissible work-shapes (v1 drafted = forge-only; corpus classes
   need the task-registry word); the reward split.
2. **Box 2:** what verified work EARNS beyond the mint — pen weight,
   crystallization coupling (kernel-amendment questions now, per
   16_UNIT §1).
3. **The verifier's home:** `solver-verify` split crate is DRAFTED
   here as the default; absorption into `rfa-core` is the alternative;
   one word decides before U1's first commit.

## §9 The laws that bind every U landing

THE PROPORTIONAL LAW (no squared aggregators — L1 everywhere; squares only
as physics, 05's T⁴) · red-first with the red output recorded ·
tag-claim in the `dop.rs` header FIRST commit (next free: 86) + the
EQUATIONS row-claim in the same landing · one-way arrows (kernel
imports sealed math + cfe; nothing imports upward into the kernel) ·
purity (the proof rides the DEED; the kernel reads only prior/next/
step) · translate-never-transcribe from v0.1 (consult `/opt/repos`,
port nothing blindly) · the terminology law · exact rationals, no
floats, roots never taken · commit on explicit operator go.
