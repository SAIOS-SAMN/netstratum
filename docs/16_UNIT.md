# 16 — THE UNIT (the individual kernel as the deployable whole)

**Status:** DRAFT for operator ratification (2026-07-31). Translates the
operator ruling: *the baseline mathematics, the physics, and the control
plane are governed in the INDIVIDUAL kernel; the unit is what an end
user downloads and then uplinks to the mesh; Proof of Useful Work
(PoUW) and Proof of Work-Convergence (PoWC) are judged at that level.*
Subordinate to `00_CHARTER.md` §11 and `engine/kernel/CHARTER.md`;
reasoning lineage in `docs/KERNEL_RELAY.md`; component equations in
`docs/SYSTEM_SPEC.md`.

## §1 The ruling, reconciled with standing law

The law already says the kernel is individual (KERNEL_RELAY §3.1: every
running binary carries its own copy of the one small judge) and already
splits each POW++ axis into a kernel half and a mesh half (kernel
CHARTER §5a). This ruling moves the center of gravity: **both proof
axes live WHOLE in the unit.** The mesh half of each axis is not a
second law — it is the SAME unit law re-executed on received bytes.
Admission = re-judging (SYSTEM_SPEC §5). Consequences:

1. PoUW and PoWC are KERNEL AMENDMENTS, not mesh-era protocol. They
   land in the kernel-sealing phase, witnessed red-first, and bump
   `kernel::VERSION` like any amendment.
2. The N7 Ruling Boxes (what verified work earns; where the wiring
   lands) become kernel-law questions before the operator, no longer
   parked behind the anchor.
3. The mesh keeps exactly what it holds after the excision: transport
   (the file carrier), the integrity door, the head announcement, and
   the authority walk. NO mesh-only state law, ever — a mesh rule the
   unit cannot judge alone is a SECOND TRUTH made of protocol (the
   figure formerly read "a mirror made of protocol"; MIRROR is
   deprecated as a framing — 12 §2c — and the plain name is the point:
   a second machine-readable authority, banned by 00_CHARTER §11).

## §2 The unit's organs (one binary, six organs)

    ┌────────────────────────── THE UNIT ──────────────────────────┐
    │ sealed math core   rfa-core (Δ, C(Δ), Π_G, orbit_lock) ·     │
    │                    cfe::ratio (exactness) · verify_certificate│
    │ physics            reversible kinematics · beacon ξ_k        │
    │ the judge          kernel::lawful — I1..I6 + the two proofs  │
    │ the spine + codec  the chain (tag 84) · the one encoding     │
    │ state semantics    cfe (deeds, orbs, the world)              │
    │ control plane      steward seam (one input door) ·           │
    │                    authority walk (one writer per clock) ·   │
    │                    the uplink door (chain-verified bytes only)│
    └──────────────────────────────────────────────────────────────┘

The kernel CALLS the sealed math, never copies it (CHARTER §0b). The
arrow direction stays one-way: kernel imports sealed math and cfe;
spine, mesh, and clients import the kernel; nothing imports upward.

**Outside the unit, by design:** the conductor (the polity board is a
CLIENT of units, not an organ), the ledger's display halves (lenses,
deletable), tools. The solver's SEARCH half is also outside — see §3.

## §3 PoUW at the unit level — the prover/verifier split

The classic asymmetry, made constitutional:

- **The prover is free.** `engine/solver`'s search (enumerate, propose)
  is client-side, expensive, and NON-constitutional — anyone may build
  a better solver; the law does not care how a certificate was found.
- **The verifier is sealed.** `verify_certificate(task, program,
  output)` — replay the operator chain, recompute the BLAKE3 links,
  confirm the output byte-exact — moves INTO the sealed math core.
  Cheap, pure, total. The kernel calls it inside `lawful`.

**The mechanism (I4 completes).** The kernel is pure and does no I/O,
so everything judged must ride in `(prior, next, step)`: **the
certificate rides IN the step.** The discovery mint deed (the
convergence-scale mint of 04 §6/G4 — NOT the colony's `YieldCollected`,
which stays under the I1/I4 sign law) carries its work, and I4 gains
its bound arm:

    mint ≤ f( verify_certificate(work) )        // f = the ruled rate
    no certificate  → no discovery mint          // sealed default
    tampered chain  → refuse, the break named
    over the bound  → refuse, both sides carried whole

New deed, new tag: **claim 86** in the `dop.rs` header in the landing's
FIRST commit (the tag-claim convention). `f` refuses-by-default until
Ruling Box 1 fills (rate, work-shape, split) — the mechanism lands
sealed shut, the operator opens it by amendment.

## §4 PoWC at the unit level — I6 lands

Proof of Work-Convergence is the fold gate judged locally:

    orbit_lock(a, b) = ∅   →  the difference was gauge: fold lawful
    orbit_lock(a, b) ≠ ∅   →  the difference is GENUINE: the collapse
                              refuses; the residue emerges as a NEW
                              plane — and ONLY that residue is what
                              the §3 discovery mint may pay.

`orbit_lock` is re-derived RED-FIRST from the v0.1 ancestor's `fold.rs`
(consult, never port blindly) into the sealed core, beside the landed
`coherence_functional` and `coboundary_reduce`. The merge/fold deed
enters `covered()`; two units converging is both kernels judging the
same pair of chains — agreement is measured, never voted. What
`C(Δ) = 0` measures there is CONSISTENCY (the differences compose),
never sameness: two units can compose perfectly and still hold genuine
irreducible difference between them, which is precisely the difference
`orbit_lock` protects and the mint pays for. Reading that zero as
"converged" would refuse the mint exactly where the value is.
This is where the two proofs meet: **PoWC decides whether difference
is genuine; PoUW bounds what minting it earns.**

### §4a The singularity law (the angle-free collision)

Operator ruling (2026-07-31, relayed register: *a torsion singularity
where two photons bounce together, regardless of the angle they come
from*), translated: **the convergence merge is ONE point — the judge —
and its verdict is invariant to the frame of approach.** The master
equation already carries this shape as its inner sandwich
(SYSTEM_SPEC §0):

    B_S⁻¹( Π_G( F_fold*( … B_S(Δ) … ) ) )

Everything entering the center is stripped on the way in and
re-dressed on the way out:

1. **B_S kills the angle** — the finite frame group (D4⋉Z², the eight
   grid symmetries and the wrap) canonicalizes both states; the frame
   of approach quotients out (G3's recorded correction: frames join T
   OUTSIDE the key).
2. **Π_G kills the gauge** — `coboundary_reduce` strips coordinate
   distortion to the H¹ core; what a relabeling could remove, is
   removed.
3. **Torsion is the only survivor** — the sole frame-invariant
   remainder. The collision at the center (`F_fold*`, I6) then has
   exactly two lawful outcomes:

    orbit_lock = ∅   the difference was angle all the way down —
                     the fold closes to zero; the two paths were one.
    orbit_lock ≠ ∅   TORSION — the collapse refuses; the encounter
                     EMITS: the residue rises as a NEW plane.

**The bounce is emission, never annihilation.** Nothing is voted away
and nothing merges by force (the standing law: annex never merge;
disagreement preserved as exact structure). Two genuinely different
states meet the singularity and leave it as three things: the fold's
common core and the residue-plane their difference minted.

**The invariance witness (joins U2, red-first):** the same pair of
states presented in ANY frame of the finite group and any gauge
presentation yields the IDENTICAL verdict — fold or emit — and the
identical residue up to canonical form. That is the "regardless of
which angle" clause made a test, at the same tier as clippy-zero and
replay-MATCHES.

**The discreteness correction (translation, never transcription).** The
quantum register enters ONLY in its lawful discrete form. Q3's ruling
stands (discrete gauge morphodynamics; the unitary/continuous form
rejected) and 14 §2.5 bars the float form: the frame group is FINITE,
the orbit test is exact and UNBOUNDED (the tunable bound is struck — DIRECTIVE 01), and all stochasticity is
the chronicle-seeded beacon ξ_k — replayable, never an oracle. No
continuous angle exists in the law; "angle" means an element of the
finite frame group joined with a gauge coboundary, nothing more. And
the singularity JUDGES the collision — it never optimizes the
equation globally (KERNEL_RELAY §9: the kernel is the equation's
judge, not its driver).

## §5 The control plane

The unit's control plane is what already stands, drawn inside the
boundary: the steward seam (§11 rule 4 — the ONE input door), the
authority walk with its receipted handoff (tag 67 — one writer per
clock), and the spine's four verbs. The uplink law binds the door:
nothing file-borne rides the mesh; only chain-verified bytes ascend.
Units refuse each other typed across kernel versions — `VERSION` is
bound into every commitment; the constitution digest (vocabularies +
derivation rules + the equations ledger, `docs/EQUATIONS.md`) surfaces
as the unit's handshake: two units agree on the LAW before they
compare a single state.

**The end-user story:** download the unit → play and solve under the
FULL law locally (no local-vs-network mode, SYSTEM_SPEC §1) → uplink:
present the chain; every peer re-judges every step with its own kernel
copy. Trustless because admission is verification, not reputation.

## §6 The landing ladder (one at a time, red-first)

| # | landing | what | proven by |
|---|---|---|---|
| U0 | THE SEATED HAND | the spine judge fixture registers its actor (I2 honored) | 285/285 workspace green, clippy zero |
| U1 | KERNEL-VERIFY | the verifier into sealed math; the discovery-mint deed (tag 86); I4's bound arm, sealed shut | honest certificate admits at its exact bound; tampered chain refuses at its break; over-mint refuses both sides whole; certificate-less mint refuses |
| U2 | KERNEL-FOLD (I6) | `orbit_lock` re-derived; the fold deed judged; the singularity law (§4a) | gauge difference folds; torsion-locked fold refuses; the refused fold's residue emerges as the one mintable plane; **the angle-free witness** — any frame/gauge presentation of the same pair yields the identical verdict and residue |
| U3 | UNIT-1 | the deployable boundary: one binary builds without conductor/lenses; constitution digest in the handshake | the binary founds, plays, solves, and uplinks against a second copy of itself; cross-version refuses typed |
| U4 | THE RULING BOXES COME HOME | mint rate `f`, work-shape, reward split, what work earns (pen weight, mint rights) — operator rulings inscribed as kernel amendments | each ruling lands as an amendment with its witness; the sealed `f` opens |

Sequencing unchanged elsewhere: #36 spine adoption and Phase 3 (the
kernel as the live gate) proceed as chartered; this ladder relocates
the two proofs from the mesh era into the kernel era, nothing more.

## §7 Open rulings (before the operator)

1. **Ruling Box 1 (PoUW economics):** the rate `f`, the admissible
   work-shapes (planar ARC corpus, forge volumetric, future classes),
   the reward split. U1 lands with `f` refusing; these open it.
2. **Ruling Box 2 (what work earns):** pen weight, minting rights,
   crystallization coupling — now a kernel-amendment question.
3. **The verifier's home:** a split `solver-verify` crate (smallest
   surface) vs. absorption into `rfa-core` (one sealed-core crate).
   Drafted here as the split crate; one word decides.

## §7a Rulings recorded (operator, 2026-08-01)

The operator's words on the boxes above, reconciled with THE EMISSION
LAW as it LANDED (kernel VERSION 1→2, commit 9866781; authority:
EQUATIONS I4, U_DIRECTIVES §8). Ruling Box 1 is no longer open —
`mint_bound` is unsealed and `f` is ruled. Recorded here so this ladder
stays true to the landed kernel, not an earlier assumption.

- **Rate `f` — RULED, ladder-pinned (LANDED).** `f` was ruled against
  the #17 calibration (evidence commit 5898351), NOT left to assumption:
  exact ladder integers `f(0..=2)=0` (saturated rungs — free issuance
  barred), `f(3)=1`, `f(4)=4`, `f(5)=7` (the ≈1.75× inverse-hit-rate
  step). Defined ON THE LADDER ONLY — an off-ladder difficulty earns
  zero (a payload never names its own price). L1/ladder terms, no norms,
  no roots. (Supersedes the earlier "observe first / sealed at zero"
  reading: the observation happened — the calibration IS the evidence —
  and the rate is now pinned.)
- **Reward split — PHASED (LANDED v1, INSCRIBED v2).** **v1: 100% to the
  solving world** (single-node era — no mesh legs to pay yet). **v2:
  40/40/20 dual-key** — 40% Prover/Worker, 40% Mesh Verifiers/Router,
  20% Protocol Reserve — INSCRIBED for the mesh era, lands when the
  spine carries prover and mesh legs. The split does not open the mint;
  the mint is already open at v1's 100%.
- **Work-shapes — forge-only v1, corpus registry v2 (RULED).** v1
  admits FORGE-ONLY planar synthetic work (deterministic, self-checking;
  the external scoreboard untouched). v2 admits the ARC corpus through a
  registry keyed `task_id = SHA256(corpus_input_digest)` — the task's
  identity is its content, so no forged corpus entry can mint. Higher
  dimensions (volumetric) stay forge-only until ruled an economic axis
  (§10 S1, ruling C8).
- **Beyond the mint (Ruling Box 2) — consensus weight + crystallization
  (INSCRIBED).** Verified work earns MORE than banked value: it earns
  **consensus weight** (the pen — heavier judgement in the mesh) and
  **faster crystallization** (the §1 coupling — solved work sediments
  the world sooner). Kernel-amendment questions inscribed alongside the
  v2 split.

**Ruling 3 (the verifier's home) — RESOLVED:** the split `solver-verify`
crate landed (U1a). The smallest-surface option was taken; `verify_work`
lives there, importable by the kernel without a cycle.
