# U directives — the unit ladder, the I-remnants woven in

**Status:** live work orders (ratified by execution order 2026-07-31;
U0/PROPORTION-1/U1a/U1/U1f landed — see §0). The continuation
document for the unit lane: every open U-rung with its code
specifics, and every kernel-invariant remnant woven into the rung
that pays it (I4's bound → U1 landed, I5's bound → U1f landed, I6
whole → U2 open, the I1–I3 coverage extension → the W-waves open).
Cold-start here, then `16_UNIT.md` (the law), `EQUATIONS.md` (the
forms), `engine/kernel/CHARTER.md` (the constitution). The working
discipline is unchanged: one landing at a time, red-first with
recorded output, spec + mechanic + vectors together, commit on
explicit operator go.

## §0 State (2026-07-31, arch-01 — committed, board 297/297 green)

Landed and committed this session, in order (I1–I5 whole; the kernel
judges 10 of 64 deed types at that landing; 59 of 64 on main after the W-waves):
- **U0** — the spine judge fixture seats its actors (I2 honored); the
  true board count surfaced (240 was a fail-fast undercount).
- **PROPORTION-1** — the proportional law: `Σ R²` → `Σ |R|` in
  `rfa_core::{coherence_functional, conservation_measurement}`,
  `norm_squared` → `magnitude` (‖Δ‖₁), the `cfe::query` scalar;
  witnesses 24→12 and 29→7; the tag-58 wire vector RE-extracted; law
  amended (kernel Charter §3, SYSTEM_SPEC §0/§4, NS-2, 03 §5). Squares
  survive only as physics (05 T⁴). The name linear is dead.
- **U1a** — the prover/verifier split (`engine/solver-verify`); the
  opaque-payload law (cfe carries proof bytes, never parses).
- **U1 (I4)** — the PoUW anchor: tag 86 `DiscoveryMinted`,
  `verify_work` the replay-complete judge, `mint_bound` sealed shut.
  Holes A/B (audit §4) closed at landing — witnesses 7/8, the
  degenerate-difficulty decode guard.
- **U1f (I5)** — the flat decay bound calling the sealed demurrage
  rule; `last_touch` already rode tag 28 (no schema spent).
- **Docs:** `16_UNIT.md` (the unit law + §4a the singularity law),
  `EQUATIONS.md` (the one ledger), this file, and
  `AUDIT_2026-07-31.md` (the mesh-second correction + the hole audit).

Open: U2 (I6), the W-waves, #36 spine adoption (mesh-second — §7),
Phase 3 flip, U3, U4 (operator rulings). Secondary (§10): S1 the
volumetric verifier decision, S2 the worktree prune. The mesh half
rides directly behind the kernel rungs (audit §1, §7 here).

## §1 The board — where each I-remnant lands

| rung | pays | status |
|---|---|---|
| U0 | the seated hand (I2 honored in the spine fixture) | Landed |
| U1 | **I4's bound** — `mint ≤ f(verify)`, the PoUW anchor, tag 86 | Landed; **unsealed by Ruling Box 1 (2026-08-01)** — f ladder-pinned 0·0·0·1·4·7, kernel version 2 |
| U1f | **I5's bound** — flat decay ≤ rate·ease·elapsed | Landed (no schema spent — `last_touch` already rode tag 28) |
| U2 | **I6 whole** — orbit_lock, the fold gate, the singularity witness | Open — the live rung (∥ #36, mesh-second) |
| W-A..W-D | **I1/I2/I3 coverage extension** — the then-54 unjudged deeds, in four waves | parallel, any lane |
| U3 | UNIT-1 — the deployable boundary + constitution digest | after #36 |
| U4 | the Ruling Boxes come home (operator words) | **Box 1 ruled 2026-08-01** (the mint opens — §8); Box 2 open |
| S1 | the volumetric verifier decision (§10 — secondary) | boards with the volumetric tier |
| S2 | the worktree prune (§10 — housekeeping) | operator go |

Kernel coverage today: 59 of 64 deed types. `scope_of()` is already
exhaustive over all 63 (the compiler forces the scope decision); an
amendment = move the event into `covered()` + declare its lawful
conservation delta + red-first witness against `cfe::world`.

## §2 U1 — kernel-verify (the PoUW anchor; I4 completes)

**The split (prover free, verifier sealed) — landed (U1a).** New
crate `engine/solver-verify` holding exactly what a replay needs and
nothing a search needs, moved from `engine/solver`: `grid` (Grid, Op,
Program and the apply path), `loader` (Task), `forge` (`forge_task`,
`Difficulty` — deterministic re-derivation from a digest),
`certificate` (`SolveCertificate`, `seal`, `verify_certificate`,
`CertificateRead`), and `objects` — discovered at the split: the `Op`
vocabulary carries `Op::Object(ObjOp)`, so the object ops are apply
vocabulary, verifier-side; only the search is prover-side. `solver`
keeps `search` + `volume`, depends on `solver-verify`, re-exports
whole so no caller breaks. The kernel imports `cfe` + `solver-verify`
only (one-way; pure, total, no-I/O).

**The opaque-payload law (derived at the split — the arrow forced
it).** `forge` needs `physics::Beacon` and `physics` imports `cfe`,
so `cfe` can never import `solver-verify` (a cycle). The deed
therefore carries the proof as an opaque byte payload at the cfe
layer, and the work-proof encoding is `solver-verify`'s monopoly —
the codec-seam pattern restated: one component owns the form, the
kernel calls `verify_work(&bytes)` and never parses (it stays
encoding-blind; the sealed math owns both form and meaning of work
proofs). Refuse never guess: truncation, trailing bytes, unknown
codes all refuse typed.

**The replay-complete law (derived this session — do not weaken).**
`verify_certificate` alone re-derives the hash chain; it does not
prove the program solves the task. A dishonest prover can seal an intact chain
over a wrong output. The kernel-level verification is therefore the
full replay, all three or refuse:

    solver_verify::verify_work(work) -> Result<Difficulty, WorkBroken>
      1. re-forge:  task = forge_task(work.forge_digest, work.difficulty)
                    (pure, deterministic — no external data ever)
      2. re-apply:  work.program maps every training pair exactly and
                    the test input to work.output (the honest-withhold
                    law: one miss = refuse, never a guess)
      3. re-chain:  verify_certificate(work.certificate, task,
                    program, output) == Intact

**The deed — tag 86, claimed in the `dop.rs` header in the landing's
first commit (the tag-claim convention; 86 verified next-free).**

    ClosureEvent::DiscoveryMinted {
        orb_id, resource, amount,          // the mint itself (cfe-typed)
        work: Vec<u8>,                     // the proof, riding whole —
    }                                      // solver-verify's sealed encoding of
                                           // (forge_digest, difficulty, program,
                                           //  output, certificate); opaque to cfe
                                           // per the opaque-payload law above

The receipt carries its proof whole — replay under SEAL B re-verifies
the work forever. No change to `Step` (the deed is the carrier), no
schema change (event-only; `banked` is already committed) — U1 touches
no codec bytes. **v2 corpus work landed 2026-08-04** — see §2a below.
**v1 work-shape: forge tasks only** (re-derivable from
the digest, chronicle-seeded). Corpus classes (ARC-2 files) need a
task-registry ruling — parked in Ruling Box 1.

**Kernel arms.** `covered()` gains `DiscoveryMinted`; `scope_of` →
`Collect`; `conservation` lawful_delta = `+amount`; and I4 completes:

    creation: amount ≥ 0                                (landed sign law)
              && verify_work(work) = Ok(difficulty)     (else WorkBroken{at_step})
              && amount ≤ mint_bound(difficulty)        (else MintExceedsWork{bound, claimed})

    mint_bound(_: &Difficulty) -> Ratio { ratio(0, 1) } // sealed shut —
    // opens only by Ruling Box 1 amendment (rate, shapes, split),
    // bumping kernel::version. Mechanism first, economics by ruling.

**Unsealed (Ruling Box 1, operator 2026-08-01 — the version 1→2
amendment; evidence: the #17 calibration, 5898351).** `mint_bound`
now pays the ruled `f` in exact integers on the ladder only —
`f(0..=2) = 0` (saturated), `f(3) = 1`, `f(4) = 4`, `f(5) = 7`; an
off-ladder difficulty earns zero (a payload cannot name its own
price). Red-first: `the_ruled_rate_pays_the_ladder_exactly` (red
recorded against the sealed judge), `saturated_and_off_ladder_work_
earns_zero`; the sealed-shut pin truth-updated (level 1's bound is
still 0 — ruled, not defaulted); the version pin moved 1→2.

New `Unlawful` arms: `WorkBroken { at_step: usize }` and
`MintExceedsWork { bound: Box<Ratio>, claimed: Box<Ratio> }` — both
carry the measurement whole.

**Witnesses (red-first, in order) — landed, all green
(`engine/kernel/tests/discovery_pins.rs`, 9 pins):**
1. valid forged work + any positive amount refuses `MintExceedsWork`
   with bound 0 carried — the sealed-shut law is itself witnessed;
2. a tampered chain refuses `WorkBroken` naming its exact `at_step`;
3. an intact chain over a program that misses one training pair
   refuses — the fabricated-render attack, dead;
4. a wrong forge_digest refuses (the task is not the one claimed);
5. a malformed payload refuses as form (trailing bytes, empty proof);
6. zero-amount with valid work admits (the degenerate mint is lawful);
7. **degenerate difficulty refuses as form** — `train_pairs: 0` (an
   empty training set makes the replay vacuous — hole A, audit §4);
8. **zero-dimension difficulty refuses, never panics** — `rows/cols:
   0` drove the forge's `Grid::new().expect()` to panic in the
   kernel's call path (hole B, audit §4; the totality law). Both
   close at one guard: `decode_work` refuses degenerate difficulty
   (the refuse-never-guess law extended to degenerate fields);
9. the deed round-trips tag 86 byte-exact; the work payload
   round-trips its own encoding.

**Equations row-claim:** I4's row claimed landed whole in the landing.

## §2a Work-shapes v2 — the corpus registry (landed 2026-08-04, cognition lane)

Operator ruling 2026-08-04, executing `16_UNIT` §7a's corpus clause.
Verification landed whole in `solver-verify`; the economics are relayed
to the kernel lane and corpus work earns zero until they land — the U1
precedent exactly (mechanism first, price by amendment).

**The problem v1 never had.** A forge task re-derives from its digest,
so the judge needs nothing but the digest to know the task is real. A
corpus task is observed content — it cannot be re-derived, so it must
ride in the payload, and the judge must then answer a question v1 never
asked: *is this a real task, or one the miner wrote?*

**Content-addressing does not answer it.** `task_id = H(content)`
proves which task was solved, never that the task was admissible. A
miner may author a trivial task, hash it honestly, solve it honestly,
seal an intact chain, and mint. The ruling's rationale — "the task's
identity is its content, so no forged corpus entry can mint" — holds
only once an admissible set exists; identity alone is not admission.

**The mechanism — struck 2026-08-04, the same day it landed**
(operator ruling: the engine must be able to forge new corpus, and no
merkle trees). Recorded here rather than deleted, because the reasoning
that produced it was half right and the half that survives is
load-bearing.

*What it was.* A static Merkle tree over the admissible corpus, its root
pinned as an invariant constant, derived over `arc_real_pairs` (960
files, 960 distinct task_ids, every inclusion path verified at
derivation). A payload carried its `task_id` and an inclusion path; the
judge recomputed the root from leaf + path. Four legs — re-hash, admit,
re-apply, re-chain.

*Why it went.* A pinned root is a static enumeration — a finite well,
where the ruling asks for an open one — and it is an authority, a
constant deciding what counts, which is a second machine-readable truth
of the kind `00_CHARTER` §11 excised the mirror for. It also cannot move
at mesh speed: every node must carry the same root, and changing it is a
kernel version amendment.

*What survives, and it is the finding not the answer.* `task_id =
H(content)` proves which task was solved and never that the task was
admissible — **identity is not admission**. That stands. So does
`task_digest`, the v2 payload carrying task content (observed content
still cannot be re-derived), the BLAKE3 decision, `Grid::new`'s hue
refusal, and `Difficulty::well_formed`.

*What replaces it: a predicate, not a list.* A claim is admissible when
the base vocabulary provably fails the task at depth `d` and the
candidate closes it — both halves replayable with machinery
`verify_work` already has. No root, no constant, no authority, and it
moves as fast as the mesh. Until it lands the corpus judge does not
admit at all, which is lawful: corpus work has no rate, and the motion
clause 4 makes imported content a benchmark and never a mint source.

*Nothing in `engine/kernel` changed.* The constant was never hoisted
into the judge — `verify_corpus_work_against(bytes, &root)` kept it a
parameter — and that seam is why the strike stayed inside one crate.

The judge is now three legs:

| leg | check | refusal |
|---|---|---|
| 1 re-hash | carried task hashes to the claimed `task_id` | `TaskIdMismatch` |
| 2 re-apply | every training pair exactly, then test input → claimed output | `TrainingMiss{pair}` / `OutputMismatch` |
| 3 re-chain | the certificate re-derives Intact | `ChainDiverged{at_step}` |

The class an admitted corpus solve used to report was renamed to
`WorkClass::Corpus` in the strike (the old name carried the word
"admissible"): it claimed an admission the judge no longer performs, and
a name that overclaims is the vacuity the motion clause 6 forbids.

**What the judge cannot check, stated rather than hidden.** A corpus
task's held-out answer is not carried and not derivable, so leg 3
proves the program closes the training pairs and renders the claimed
output — never that the output is correct. That is not a defect in the
judge; it is the actual ARC problem. Measured on this corpus: of 34
programs that closed training, 33 matched the held-out answer and one
did not, so **~3% of closures are confidently wrong**. That is the
fabrication surface a corpus rate would buy, and it is why the rate is
an economics ruling rather than a mechanical one.

**Wire law.** Schema bytes are append-only. v1 is byte-identical and
untouched: `WORK_SCHEMA_V1 = 1`, `WORK_SCHEMA_V2_CORPUS = 2`, dispatch
on the leading byte, an unknown byte refuses. The op table and the
certificate encoding were extracted to one shared definition rather
than copied — two copies of a wire table is how they drift. The
`Difficulty::well_formed` vacuity guard has a v2 equivalent: `train_len == 0`
refuses as form, so `98ccbbe`'s hole cannot reopen in the new shape.

**Names and identity.** `task_digest` covers the observable task only —
every training pair, then the test input. The filename is excluded (it
is provenance, not content) and `test_expected` is excluded (the
held-out answer never rides in a proof, and for real corpus work may
not be known). The certificate needs a name, so both halves derive the
same canonical one from the task_id — a miner cannot vary it.

**Relayed to the kernel lane (its file, not this lane's):**
1. `mint_bound` gains an `AdmissibleCorpus` arm — the operator ruled a
   fixed premium rational reflecting the 3.4% solve rate. Until it
   lands, corpus work verifies and earns nothing.
2. The kernel should call `verify_work_class`, not `verify_work`: only
   the former can express corpus work, whose identity is a task_id and
   not a ladder rung. `verify_work` is unchanged and refuses a corpus
   payload `UnsupportedShape` rather than mispricing it.
3. **Root placement.** The ruling puts `CORPUS_ROOT` in the kernel; it
   is implemented in the sealed tier because the opaque-payload law
   forbids the kernel parsing a payload, and the inclusion check needs
   the parsed path. `verify_corpus_work_against(bytes, &root)` exists
   as the seam: hoisting the constant into `engine/kernel` and passing
   it down is a one-parameter change that keeps the kernel
   encoding-blind.
4. **The hash is BLAKE3; the inscribed text says SHA256.** `16_UNIT`
   §7a reads `task_id = SHA256(corpus_input_digest)`. Every other
   digest in this system is BLAKE3 and `solver-verify`'s only hash
   dependency is `blake3`; adding SHA-2 puts a second hash primitive in
   the sealed tier, against §0b's one-definition law. Implemented in
   BLAKE3 and flagged — nothing mints corpus work until the rate lands,
   so this is the moment to correct it if SHA-256 was meant literally.

Witnesses: 13 pins, `solver-verify/tests/corpus_pins.rs`, load-bearing
one `a_self_authored_task_refuses` (honest hash + honest solve + intact
chain + no membership = nothing earned, including the borrowed-path
variant). Board 642/642, clippy zero in both solver crates.

## §3 U1f — the decay bound (I5 completes) — landed 2026-07-31

**The premise died at execution:** `last_touch` was already committed
— it rides the orb wire (tag 28, the vent entry, VENT-1/SINK-1) and
the codec commits orbs as wire bytes. No SCHEMA_V3, no codec-lane
coordination, no schema spent. (Recorded so the next reader learns
the lesson: check the wire before pricing an amendment.)

The landed form: the kernel calls the sealed rule —
`cfe::world::demurrage_due` and `ease_of` made pub (call-not-copy,
Charter §0b; one computation, two judges): `decay ≤ min(rate · ease ·
elapsed, banked)`, flat, thermally gated; an orb the rule owes
nothing (no leak, no bank, no elapsed time) bounds at zero. New
refusal `DecayExceedsBound{bound, claimed}`, both sides whole.
Witnessed in `engine/kernel/tests/decay_pins.rs` (red recorded: the
sign-only kernel admitted the excess) with one witness truth-update
in `conservation_pins.rs` (the sign-era "decay of 2 admits" pin sat
on a starless world — it now refuses with bound 0 carried, and says
so). `DemurrageSettled`'s wire form untouched (one tag, one meaning,
forever).

## §4 U2 — kernel-fold (I6 whole; the singularity witnessed)

**The verifier-only boundary (operator ruling 2026-08-01; amends this
rung).** The kernel's `rfa-core` dependency is the verifier half only.
`orbit_lock` runs a search over the operator space (the tunable bound is struck — directive 01);
the kernel is pure, total and cheap and **must never call it**. The
split is U1a's, applied to the fold gate:

    prover  (free, may be stochastic, chain-seeded so it stays replayable)
            orbit_lock(a, b, operators) -> Option<OrbitWitness>
    judge   (pure, total, O(step), no search, no tunable bound)
            verify_orbit_witness(a, b, operators, &witness) -> bool

`verify_orbit_witness` landed 2026-08-01 (`835d03b`, `rfa-core` 9/9
green). A kernel arm may call it and nothing else from `fold`. Pinned by
`a_presented_witness_verifies_without_searching`,
`a_forged_witness_refuses`, and `absence_cannot_be_witnessed` — the last
being why the minting branch must be the lock branch: only a lock can
carry proof a judge can re-check.

`rfa_core::orbit_lock(a, b, operators) -> Option<OrbitWitness>`
re-derived from the v0.1 predecessor's `fold.rs` (`/opt/repos` — consult,
never port blindly). **Two translation laws bind the derivation:** the
proportional law (any squared aggregator in the predecessor lands as L1) and
translate-never-transcribe (the RCF canonical form `≅_RCF` — PhiK
I^(4)'s hash — is derived as part of this landing). The kernel gains
its `rfa-core` dependency here (arrow still one-way).

Fold-class deeds (`Confluent`, `EraAnnexed`, `EraFolded`) enter
`covered()` under I6: a fold is lawful iff `orbit_lock = ∅`; a refused
fold's residue is the emission — and the U1×U2 seam is the law that
convergence-scale `DiscoveryMinted` pays only the emitted residue
(16_UNIT §4: PoWC decides genuine, PoUW bounds the pay).

**Witnesses:** gauge-only difference folds; torsion-locked refuses with
the lock named; the refused fold's residue is the one mintable plane;
**the angle-free witness** (16_UNIT §4a) — the same pair under all
eight D4 frames and a gauge-coboundary shift yields the identical
verdict and canonical residue; the C(Δ)=0 pair is the replication
witness, not consilience.

## §5 W-waves — the coverage campaign (I1/I2/I3 extension, 55 deeds)

The completeness rule stands: an event enters `covered()` only when
every invariant that should govern it has landed. Each amendment is
ratified against `cfe::world` red-first (the `conservation_pins.rs`
model). The waves are disjoint deed families — parallel across lanes:

- **W-A, the authority deeds (~24, {I2, I3}):** ColonyFounded,
  ZoneBuilt, WorldCharted, ColonyCharted, StarCommissioned,
  PlanetSited, BondSealed, ForwardSealed, CouncilSealed,
  GaugeShiftedAt, TechLicensed, GenesisSealed, PracticeAdopted/
  Released, ProspectStruck, ElderWarded, VeilDrawn, InstrumentForged,
  RouteChartered, OfferPosted, LineSucceeded, ZoneRazed, WardLifted,
  VeilLifted. **Caution (derived):** several of these move or create
  value — `ColonyFounded` seeds the founding bank, `ColonyCharted`
  moves the 20-iron/10-biomass charted reserves — so each amendment
  derives its lawful_delta from `cfe::world`, and the founding-reserve
  mints need a declared I4 arm (a chartered genesis mint, named, never
  a hole). `TaxonSealed` is coder-03's standing #42 — leave it theirs.
- **W-B, the value movers (~8, {I1, I2, I3}):** UpkeepSettled,
  OverflowAbsorbed, BondSettled, ForwardReleased, FrictionSettled,
  CrystalDrawn, AssayStruck, OfferCleared.
- **W-C, the world-driven consequences (~14, {I1 + sign/bound}):**
  FamineStruck/Relieved, VentSettled, OverheatStruck/Cleared,
  AbsorptionLapsed, ForwardSlashed, WantStruck/Relieved, StarAged,
  TechDiscovered, RouteLapsed, VoidShare, IncursionStruck.
- **W-D, the mesh/universe deeds (~8, protocol-law + I6-gated):**
  EraAnnexed, GalaxyObserved, HandoffSealed, GaugeShifted, Diverged,
  Confluent, EraFolded, CouncilClosed — the fold-class trio waits on
  U2; the rest ride their protocol law.

Phase 3 (the kernel as the live gate — `cfe::world` demoted to proposer,
refuse-unknown live) flips only when `Unjudged` is unreachable.

## §6 U3 — UNIT-1 (the deployable boundary)

One binary = cli + kernel + spine + codec + cfe + physics + rfa-core +
solver-verify + the mesh door. Outside: conductor (a client of units),
the ledger's display halves (the physical ledger/lens split is #37's
opening move — U3 sequences after it or feature-gates around it),
tools. Surface `kernel::constitution_digest()` — `H(version ‖
vocabularies ‖ derivation_rules ‖ schema)` (kernel Charter §3), the
Equations ledger among the hashed derivation rules (16_UNIT §5) — in
the mesh handshake: cross-version units refuse typed. **Acceptance:**
the unit binary founds, plays, solves (one forge grind → certificate),
and uplinks against a second copy of itself (extend the
`ns5_two_nodes` pattern); a forged chain refuses at its break.

## §7 Sequencing (mesh-second — operator ruling 2026-07-31, audit §1)

The mesh is secondary to the kernel, not last: its door rises directly
behind the kernel rungs, ahead of the W-wave tail. Landed this session:
U0 · PROPORTION-1 · U1a · U1 · U1f (I1–I5 whole).

    [Done] U0 · PROPORTION-1 · U1a · U1 (I4) · U1f (I5)
      → U2 (orbit_lock/I6 + the angle-free witness — the one
         research-shaped rung; consult /opt/repos)
      ∥ #36 (spine at TUniversal + adoption — the MESH's promoted
         rung, riding directly behind U1, not behind the W-waves;
         kernel-OBSERVED play, holes stamped judged=0)
      → W-A..W-D  ∥  (parallel, any lane, disjoint families; W-D's
         fold trio lands with/after U2)
      ∥ the mesh half — Σ_mesh, re-sync, resonance, the verified-work
         anchor (the rate rides the kernel's mint_bound once ruled)
      → Phase 3 flip (Unjudged unreachable — all waves + U2 complete
         → the kernel is the door) — **landed 2026-08-13**
         (`39_PHASE3_FLIP.md`, operator ruling: sub-coordinate
         addressing + sign-off). `mesh::arbor::Coordinate` gained
         `sub: u32`; `conductor::chain_beat` commits one arbor record
         per deed, gated live; a refused deed gets no commitment.
         U2/W-waves were a different precondition this doc names —
         39_PHASE3_FLIP's own §2 audit found coverage already total
         and the live board already re-judging clean, which is the
         precondition this flip actually landed against.
      → U3 (after #36/#37) → U4 (the rulings open the mint)

Calibration note: after #36 the whole single-player + NPC economy runs
chained with certificate-bounded minting refusing until ruled — measure
the synthetic economics there first, then rule `f` against observation,
not assumption.

## §8 The Ruling Boxes (operator words — nothing else blocks)

1. **Box 1 — ruled (operator, 2026-08-01; landed as the kernel
   version 1→2 amendment).** Evidence: the #17 calibration
   (`5898351` — hit-rates 100·100·100·97.5·75·42.5% at rungs 0–5).
   - **The rate `f`, ladder-pinned in exact integers (L1/ladder
     terms — no norms, no roots, the proportional law):**
     `f(0..=2) = 0` (saturated rungs are proof-of-position; free
     issuance barred) · `f(3) = 1` (the marginal threshold) ·
     `f(4) = 4` · `f(5) = 7` (the ~1.75× inverse-hit-rate step).
     Defined on the ladder only — off-ladder difficulty earns zero.
   - **Work-shapes v1: planar forge tasks only** (structural —
     `verify_work` accepts no other form). ARC corpus classes await
     the task-registry word; the volumetric tier stays decorative
     (S1) until then.
   - **Reward split v1: 100% to the solving world** (the deed banks
     on its own orb; no mesh counterparty exists yet). **v2
     reserved, inscribed for the mesh era:** 40% prover · 40% mesh
     carriers · 20% protocol reserve — wiring unbuilt, opens with
     the mesh-era ruling, never before.
2. **Box 2:** what verified work earns beyond the mint — pen weight,
   crystallization coupling (kernel-amendment questions now, per
   16_UNIT §1).
3. **The verifier's home:** resolved by execution — the `solver-verify`
   split crate landed with U1a and stands; absorption into `rfa-core`
   is now a refactor question for a future landing, not a gate.

## §9 The laws that bind every U landing

The proportional law (no squared aggregators — L1 everywhere; squares only
as physics, 05's T⁴) · red-first with the red output recorded ·
tag-claim in the `dop.rs` header first commit (next free: 87) + the
Equations row-claim in the same landing · one-way arrows (kernel
imports sealed math + cfe; nothing imports upward into the kernel) ·
purity (the proof rides the deed; the kernel reads only prior/next/
step) · translate-never-transcribe from v0.1 (consult `/opt/repos`,
port nothing blindly) · the terminology law · exact rationals, no
floats, roots never taken · read whole files before any edit (operator
discipline 2026-07-31 — grep locates, never grounds an amendment) ·
commit on explicit operator go.

## §10 The secondary board (audit-sourced; deliberate, none forgotten)

- **S1 — the volumetric verifier** (audit §4/§5.4). The volume path
  (`engine/solver/src/volume.rs`) is half a component: the prover is
  real (Volume, VOp, forge_volume, solve_volume) but `seal_volume` is
  private and its "certificate" is a bare digest nobody can re-derive
  — tamper-evidence for no one, decorative. A hand-built VolumeTask
  with an empty train is vacuously "Solved" by the empty program.
  Non-constitutional today: ARC-AGI-2/3 are planar, volumetric tasks
  are forge-only synthetics, and no volumetric deed can mint (tag 86
  carries planar work only). **The decision (operator's, rides Ruling
  Box 1's work-shape word):** if higher-dimensional work becomes a
  real economic axis, land `verify_volume` mirroring `verify_work`
  (re-forge ∧ re-apply ∧ re-chain + the degenerate-difficulty guard)
  with its own deed; if not, mark `seal_volume` decorative in its
  header and refuse the empty-train solve. Boards with the volumetric
  tier, never before.
  **The deferral is lifted — `verify_volume` landed 2026-08-04**
  (operator ruling: volumetric work is a real economic axis; the
  motion's clause 6, *nothing vacuous*, says a decorative seal does not
  stand — it earns a real verifier or it goes). C8's reading below is
  kept as the record of what was true for three days.
  Work-shapes **v3**, `WORK_SCHEMA_V3_VOLUME = 3`. The prover/verifier
  split mirrors U1a: the replay half moved to
  `solver-verify/src/volume.rs` (carrier, apply path, forge, degeneracy
  law, certificate); `solver` keeps only `solve_volume`, the search.
  - **`seal_volume` is gone, not re-homed.** C8 was right that a bare
    digest nobody can re-derive is tamper-evidence for no one. Its
    replacement, `VolumeCertificate`, is parent-linked over
    observation → word → output and names its diverged link. Keeping a
    decorative seal beside a real one is how a verifier gets called
    with the wrong one.
  - **Strictly simpler than corpus v2**, for a structural reason:
    volumetric tasks are forge-only and re-derive from their digest, so
    the judge needs no admissible set at all — one re-forge leg does
    what corpus work needed a whole admission mechanism to attempt.
  - **The vacuity is unreachable, not merely guarded.**
    `VolDifficulty::well_formed` refuses a degenerate rung at decode, before
    the forge runs: `train_pairs: 0` is the exploitable one (a free
    mint, since "maps every pair" is vacuously true over none), and
    zero dimensions are the totality half (they panic the forge).
    `word_len: 0` stays lawful — the empty word is the identity task.
  - **Its own ladder.** `WorkClass::Volumetric(VolDifficulty)` is
    reported separately from `Forge(Difficulty)`: a volumetric rung is
    not a planar rung and pricing 3-D work at 2-D rates would be a
    category error the type now prevents.
  - Relayed: the volumetric deed needs **tag 91** (cfe's registry), a
    `mint_bound` arm for the volumetric ladder (kernel lane), and the
    conductor's volumetric arm must build and carry a `VolumeProof`
    instead of passing `None` (harness lane). One mechanical call-site
    adaptation was made in `conductor/src/lib.rs` — the journal takes
    `certificate.output` now that the seal is a chain — and relayed.
  - 10 pins, `solver-verify/tests/volume_verify_pins.rs`. Board
    660/660, clippy zero in both solver crates.

  **Ruled (operator C8, 2026-08-01): decorative — superseded above.**
  v1 is forge-only,
  ARC-AGI-2/3 are planar, no volumetric deed mints (tag 86 = planar
  work). `seal_volume` now carries a decorative header
  (`engine/solver/src/volume.rs`) stating it is not a verifier and its
  digest carries no economic weight. `verify_volume` is deferred until
  higher-dimensional work becomes a real economic axis (rides Ruling
  Box 1's work-shape word).
  **Closed whole 2026-08-03 (cognition lane) — the ruling's second
  clause, which had not landed.** C8 said two things: mark the seal
  decorative *and* refuse the empty-train solve. Only the header
  landed; the vacuous solve stood for two days, and the header had
  reframed it ("the empty-train solve is not a mint") rather than
  obeyed it. Measured red-first, the hole was wider than the audit
  recorded — two failure modes, not one:
  - `solve_volume` returned `Solved` on an empty train (the vacuity
    the audit named): `train.iter().all(..)` is true over no pairs,
    so the empty program closes nothing and claims everything.
  - `solve` (Planar) did not vacuously solve — it **panicked**,
    `derived_tiles` indexing `task.train[0]` unguarded
    (`search.rs`). A totality violation of the same class as audit
    hole B, in the prover half, never recorded. Non-constitutional
    (the kernel's `decode_work` refuses `train_pairs: 0`, and
    `load_task` refuses an empty train at the file door) — but
    `Task`'s fields are `pub`, so any in-process caller constructing
    a task reached it.
  Both close at the guard shape hole A/B used: an empty training set
  reads `Unsolved` — the honest withhold, nothing to close means
  nothing proved. The guard is repeated at the sector door
  (`closes_every_training_pair`) on purpose: the door's law is
  vacuously true over no pairs, and it is what "only exactness may
  dispose" rests on, so it refuses the shape whoever proposes it.
  Witnesses (red proven live, 3 pins): `an_empty_training_set_never_
  solves` planar (`harness_pins`) and volumetric (`volume_pins`),
  `an_empty_training_set_never_crosses_the_door` (`consulted_pins`).
  Board 625/625, clippy zero in both solver crates.
- **S3 — the invariant that lived in its callers** (cognition lane,
  measured 2026-08-04 sweeping the class S1 exposed). `Grid` documents
  "hues 0–9" in its own type comment and `Grid::new` never enforced it.
  The loader enforced it separately (`loader.rs`), which is the tell:
  the invariant lived in whichever caller remembered, so every door
  that was not the loader stood open. Two panics follow an
  out-of-range hue, both in sealed math the kernel calls through
  `verify_work`: `Op::Recolor`'s `map[h as usize]` on a `[u8; 10]`
  (`grid.rs`) and `objects::background`'s `counts[cell as usize]` on a
  `[usize; 10]` (`objects.rs`).
  **Not reachable under v1, and that is a property of the shape, not
  of the code.** Work-shapes v1 is forge-only, so every grid
  `verify_work` feeds to `run` comes from the forge (hues sampled
  `uniform_index(.., 10)`), and the wire-carried output grid is only
  compared and hashed, never applied. **Work-shapes v2 ends this**: a
  corpus task cannot be re-derived from a digest, so its content must
  ride in the payload — the moment wire-borne grids reach `run`, both
  panics are reachable inside the judge. Closed ahead of v2 at the one
  door: `Grid::new` refuses any hue ≥ `HUES` (10). Refuse never guess —
  not clamped, not a grid. 3 pins, `solver-verify/tests/hue_pins.rs`.
  Same sweep, second finding: `forge_task` panics on a degenerate
  `Difficulty` and the fields are `pub`, so the guard can only live at
  the doors — and it lived as an inline condition inside `decode_work`
  alone. Now named `Difficulty::well_formed()` so v2's corpus decode path
  calls the law instead of restating it (restating is how hole B
  happened). `word_len: 0` is deliberately admitted: the empty secret
  word is the identity task, and a copy task is a task.
  Board 629/629, clippy zero in both solver crates.
- **S2 — the worktree prune** (audit §3/§5.6). ~13 stale worktrees
  under `.claude/worktrees/` still carry old `engine/mirror` code on
  disk — the only place the dead name survives. Outside the
  workspace. Check each for uncommitted work first; prune on operator
  go.
