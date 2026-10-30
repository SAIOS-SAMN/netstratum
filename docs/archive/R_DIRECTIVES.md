# R_DIRECTIVES — the remediation lane's work orders

**Lane:** `remediations-lane`. **Opened:** 2026-08-02.
**Authority:** the code and the operator's word. **Not the tests.**

> Operator ruling 2026-08-02: the tests are not the authority; the code and
> the operator's directives are. Where they disagree, the likelier fault is
> a test asserting what the code does not do.

A directive names an owner, a cure, and how the cure is checked. It is
discharged by landing or by being struck — never by a test going green,
because a green test is exactly what a facade produces.

---

## R-D1 · main is clippy-red, and nobody is fixing it

**Status:** open · **Owner:** whoever split `engine/cfe` (kernel-lane
split `world.rs` → `world/`; capacity-lane merged it and added
`stock.rs`) · **Severity:** blocks the workspace's own clippy-zero gate.

`cargo clippy --workspace --all-targets -- -D warnings` is **red on
`main` itself**, independent of any lane. Seven errors, all residue of
the cfe split:

| where | error |
|---|---|
| `cfe/src/world/mod.rs:19` | unused import `TickRead` |
| `cfe/src/world/state.rs:9` | unused imports `ClosureEvent`, `Zone` |
| `cfe/src/world/state.rs:12` | unused import `Attestation` |
| `cfe/src/world/replay.rs:225,228,321,324` | redundant closure `\|held\| in_crisis(held)` ×4 |

**Cure:** delete the four unused imports; replace the four closures with
`in_crisis`. Mechanical, no behaviour change.

**Why this lane did not just fix it.** It is another lane's crate, and
silently repairing someone's in-flight split is how two nodes end up
editing one file. **The directive is the correct instrument; the patch
is theirs.** This lane's own crates (kernel, solver, conductor) are
clippy-green.

**Check:** `cargo clippy --workspace --all-targets -- -D warnings` exits
0 on `main`.

---

## R-D2 · the quorum is a facade — the clock never asks anyone

**Status:** open · **Owner:** operator ruling required (strike or build)
· **Severity:** a constitutional mechanism that does not exist.

**The code, whole** (`cfe/src/vrp.rs:25-31`):

```rust
impl Timekeeper {
    pub fn attest(&self, clock: &WorldClock) -> Attestation {
        Attestation::Attests { next: clock.now() + 1 }
    }
}
```

**`&self` is never read.** `self.id` — the only thing distinguishing one
timekeeper from another — is ignored. Therefore:

1. **Every timekeeper returns the identical answer, always.** The
   "3-of-3 unanimous" quorum (`vrp.rs:36`) is one answer counted three
   times. `genesis.rs:10,55` dutifully seats `Timekeeper { id: 0, 1, 2 }`
   and the ids do nothing.
2. **`Attestation::Refuses` is constructed in exactly one place in the
   whole tree — `cfe/tests/ns3_conformance.rs:93`, a test.** Never in
   production. No code path can produce it.
3. **`TickRead::QuorumOpen` is therefore unreachable in production.**
   `walk`'s match arm at `vrp.rs:81` is dead: the count is always 3.

**This is the facade, precisely.** `ns3_conformance` hand-builds
`Refuses`, feeds it straight into `walk()`, and asserts `QuorumOpen`.
**The test passes and proves nothing about the system**, because it
bypasses the only producer of attestations there is. The types say
*consensus*; the code says `now() + 1`.

**The unused imports of R-D1 are this finding's fingerprint.** The split
carried `Attestation` into `world/state.rs` and `TickRead` into
`world/mod.rs`, and neither is used — the world layer holds the quorum
vocabulary and never exercises it. A dead mechanism leaves its clothes
lying around.

**Recorded before, never cured:** `PICKUP` §8e already names this
(*"`Timekeeper::attest` ignores `self.id` … `QuorumOpen`/`Refuses` are
unreachable"*). It has survived at least one audit because the test is
green.

**The cure is a ruling, and there are exactly two honest ones:**

- **Strike it.** One writer per clock is already the mesh's law
  (NS-5 §7, `HandoffSealed`). If the clock is not a quorum, delete
  `Timekeeper`, `Attestation`, the `QuorumOpen` arm and the fixture that
  dresses them, and let `WorldClock` say plainly that it walks by one
  hand. **Nothing is lost — the mechanism was never running.**
- **Build it.** Give `attest` a reason to consult `self.id` — a keeper
  that can disagree — and the quorum becomes real. This is heavier and
  only worth it if the N7 anchor wants independent keepers.

**Do not "fix the test".** The test is not wrong; it is honest about a
mechanism that is not there. Deleting the assertion would hide the
finding, and strengthening it would pin a fiction harder.

**Check (either ruling):** `Attestation::Refuses` must be constructible
by production code, or must not exist.

---

## R-D3 · the facade detector — a standing test, not vigilance

**Status:** proposed · **Owner:** this lane · **Severity:** the general
cure for R-D2's class.

R-D2 is one instance. The general shape is:

> **A type variant, or an error arm, that only ever comes into existence
> inside `tests/`.**

Such a variant is a claim the production code never makes. It reads as
covered — a test names it, a match arm handles it, a doc comment
explains it — and no live path can reach it.

The house already has the right instrument for exactly this class:
`kernel/tests/recurrence_guard.rs` mechanizes review lints *"at the same
tier as clippy-zero"*, on the stated ground that **vigilance did not
prevent the defect; a standing test does.** `constitution_lint.rs` does the same
for retired vocabulary — and caught this lane twice in one session.

**Proposed:** a lint that enumerates public enum variants and fails any
that appear in `src/` **only** at their declaration and their match
arms, while being constructed only under `tests/`. Seed the allow-list
with anything ruled deliberately-latent, by name, so the list is the
review conversation.

**And it must have teeth before it is trusted** — `recurrence_guard`'s
own rule 7: prove the detector on a synthetic facade first. A guard
that cannot bite is the vacuous check this directive exists to eliminate.

---

## R-D4 · why does iron exist solely — expansion gated on capability

**Status:** open, survey not started · **Owner:** this lane, against the
element frame (PToE / syntropy) · **Severity:** design.

Operator reframe 2026-08-02: rather than plugging specific items such as
iron, ask why iron alone exists, and remediate so that expansion is gated on
capabilities.

This lane's alloy finding (task #19) was resource plumbing — one rung
too low. The target is the same cartesian shape one level up:
**`Resource` is a closed enum** (`cfe/src/dop.rs:209`) matched
exhaustively across the tree, so a periodic table cannot expand without
touching every arm. Identical in shape to the forge vocabulary `V` this
lane already un-caged, and the same cure applies: **a set that grows
with capability, never a fixed enumeration.**

Survey targets, in order:

1. `Resource` / `ZoneKind` closed enums and their exhaustive matches.
2. **Boolean gates** — capabilities flattened to yes/no, which is the
   same collapse as reading a dual through one arm:
   `can_smelt = level >= 3`, `knows_its_star()`, `crossed`, `veiled`,
   `warded: Option<bool>`, `industry_relevant()`, `permits()`.
3. Iterations on dimensional expansion (the standing cartesian hunt).

**Read the landed element work first** — ELEMENT-GRAMMAR-1 (tags 70–72),
`45_ALMANAC` §4a/§5a — before proposing anything against it. This lane
has twice built on one reading without checking the other frame; that is
the mistake this directive is written to avoid making a third time.
