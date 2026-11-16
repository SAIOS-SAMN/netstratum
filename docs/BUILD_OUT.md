# Build-out guidelines

How Netstratum's design record turns into GitHub work, and the gates every
piece of work passes. Mycelium and Xylarium keep the same shape, so a seam
between the three reads alike on every side (agreed 2026-11-16).

## 1. From a doc section to a merge

    design doc §  →  milestone  →  issue  →  branch  →  pull request  →  main

1. **The doc section.** Every law, refactor or seam is first written in
   `docs/`, as a derivation with its witnesses (formulas before code), and
   gets a row in [EQUATIONS.md](EQUATIONS.md). Nothing underived is built as
   law; an underived value is open physics in [108](108_OWN_RULERS.md).
2. **The milestone** is a build-out phase, titled by version and by what it
   delivers (`v0.3.0: time as coordinates: …`). Its description names:
   - the goal;
   - the governing docs;
   - the done-when, as a read or an identity;
   - the order of its issues.

   A release is the annotated tag cut at the milestone's exit
   ([CONTRIBUTING.md](../CONTRIBUTING.md#workflow)).
3. **The issue** is one change, with:
   - **Doc:** the section that governs it.
   - **What stands / what is wrong:** read in the code, with functions
     named.
   - **Do:** the change, as steps.
   - **Done-when:** the witness (an identity, a conservation, a fixed point
     or a refusal, never a number the model happened to produce), plus, for
     a change of how the universe moves, a read before and after on a laid
     checkpoint.
   - **Links:** depends-on, related, and the other repository's issue for
     a seam.
4. **The branch** is one arc, named by its subject (`rates-as-versors`). It
   takes main by merge, never by rebase, and every commit is pushed.
5. **The pull request** is one per arc. It names the issues it closes, its
   doc sections and its reads. It is merged with
   `gh pr merge --merge --delete-branch`.
   - Before deleting a branch that another PR is stacked on, retarget that
     PR to `main` first. GitHub closes a PR whose base branch is deleted.
6. **Closing.** An issue closes on its merge, with a comment naming the
   commit or PR and its read. A superseded issue closes as "not planned" and
   names its successor. A partly done issue stays open, with what remains.

## 2. Labels

| Label | Meaning |
|---|---|
| `law` | changes or derives a law of the universe |
| `track` | a planned step of a milestone |
| `defect` | the code contradicts its doc or nature |
| `needs-decision` | only the operator can rule it; the question is stated in the issue |
| `cross-project` | touches a contract with Mycelium or Xylarium; linked on both sides |
| `seam` | a crossing between kernels or with the court |
| `time` | time as coordinates, strides, events |
| `consolidation` | one model per quantity, no behaviour change |
| `docs` | the record only |
| `area/*` | the crate it lands in |

## 3. The project

The GitHub project [**Netstratum**](https://github.com/orgs/SAIOS-SAMN/projects/3) (SAIOS-SAMN) holds every open issue and PR,
with these fields:

| Field | Values |
|---|---|
| Status | Todo, In progress, Blocked, Done |
| Milestone | the repository's milestones |
| Area | `area/*` |
| Doc | the governing doc section |
| Seam | none, mycelium, xylarium (each repository leaves out its own name) |

A blocked item names its blocker in the issue: an operator decision, or
another repository's issue.

## 4. The gates

- **Exact.** No float in state. Carriers sit on the grain
  (`ring::grain`). No square or squared norm is used as a law, and no
  constant is imported.
- **One model per quantity.** Replace in place and delete the old code;
  never add a parallel version.
- **Witnesses** land with the code.
- **Reads, not bytes.** A change of how the universe moves is read before
  and after on a laid checkpoint copy, by re-derivation identities and work
  counters (N against 4N). It is never read by byte identity or wall-clock
  time, and never replayed from genesis.
- **Documentation in the same PR.** That means:
  - the module header;
  - its EQUATIONS row;
  - the design doc's status;
  - ROADMAP where a status moves;
  - GLOSSARY for each new term;
  - CHANGELOG's Unreleased section.
- **Seams agreed before they are built.** What crosses to the court or to
  Xylarium is agreed with that node first and recorded in a `cross-project`
  issue on both sides, never only in messages.

## 5. The milestones now

| Milestone | Delivers | Seams |
|---|---|---|
| v0.1.0 | stars from released matter (docs/134, 137); the domain live on the mesh | Mycelium: manifest R (#205), T delivery |
| v0.2.0 | motion whole: the unit motion as state, rates as versors (docs/138, 117 §11) | Mycelium: the turning form (78.173 §8.5) |
| v0.3.0 | time as coordinates: strides, events, the echo at each mote's time (docs/132) | Xylarium: the time seam (#210 ↔ xylarium #280) |
| v0.4.0 | energy closes: one ledger, the manifold's dissipation, starlight (docs/122, 124, 127) | — |
| v0.5.0 | one model per quantity: consolidation, the eras, standards (docs/119, PLAN §4–6) | — |
| v0.6.0 | the core tube: crossings with Xylarium (docs/123) | Xylarium: #109 ↔ xylarium #281 |
