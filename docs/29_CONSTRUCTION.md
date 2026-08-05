# 29 — THE CONSTRUCTIVE LAW: how the judge and the constants both leave

**Status:** DESIGN, drafted 2026-08-05 on the operator's question — *how
do we remove "judge" and constants from our codebase* — following the
ruling that the tree carries too much static content.

Subordinate to `00_CHARTER.md`, `26_ARTICLES.md`, `27_EVIDENCE.md`.

---

## §1 ONE DEFECT, TWO COSTUMES

A judge and a constant are the same thing wearing two costumes: **a
decision made outside the structure and imposed on it.**

- A **judge** is a decision made *after* construction: the state was
  representable, so an authority must examine it and refuse.
- A **constant** is a decision made *before* construction: the mechanism
  was missing, so a number was frozen where it should have been derived.

The removal is the same move in both directions: **push the decision
into the structure.** And the tree's own kernel header already states
the creed, verbatim:

> *an unlawful transition is not caught after the fact, it cannot be
> constructed … you do not reconcile what cannot diverge.*

The judge is not removed by deleting the function. It is removed by
making it **vacuous** — every arm dies the day the shape it refuses
becomes unrepresentable. What remains is not an authority; it is a door.

---

## §2 THE JUDGE SURFACE, MEASURED

| surface | count |
|---|---|
| `ActionRefused` variants (cfe) | 81 |
| kernel `Refused` variants | 24 |
| `WorkBroken` variants (verifier) | 23 |

Classified, the 81 split into two kinds — and the distinction is the
whole design:

**SHAPE refusals (~20)** guard states the types can spell but the law
forbids: `BadTransfer`, `BadSiting`, `TaxonUnknown`, `OffSlot`,
`BindingUnlawful`. Every one is a debt. The type let an unlawful value
exist, so a judge had to catch it. **Each is removable by
construction** — private fields, one door, the door returns the proof
(`ShapeReading`, `DivisionProof` — the pattern is already landed).

**FACT refusals (~23)** guard contingent state: `CannotAfford`,
`NoSuchOrb`, `SeatFull`, `MintNotNovel`. These are not judgments of
form — they are readings of what the chain currently holds, and they
belong at the door forever. Refusing to sell what you do not have is not
an authority; it is arithmetic.

**The end state is therefore not zero refusals. It is zero SHAPE
refusals.** A door that reports facts is a door. A judge that catches
representable-but-unlawful shapes is a debt, and the debt count is ~20.

Precedents already landed, each one a dead judge arm:

- `Degree` — private field, one constructor; below-ground is
  unrepresentable, so no rule checks for it.
- the siting triple — one `Option<(star, albedo, orbit)>`; half-sited is
  unrepresentable.
- `Delta` — antisymmetry enforced by its only writer `set_antisym`.
- `Field::new` / `Shell::new` — the one door; a malformed carrier never
  exists past it.
- `refinement_tier` — no catch-all arm; a new resource breaks the build
  until ruled, instead of defaulting into an answer.

---

## §3 THE CONSTANT'S THREE LAWFUL DESTINATIONS

A constant is a frozen decision. Unfreezing it means finding where the
decision actually lives. There are exactly three lawful homes, and which
one applies is a per-cluster ruling, not a refactor:

1. **DERIVATION** — the law was already in the numbers.
   The refinement ladder (`2^(t−1)` / `4^(2−t)`), the ring
   (`lcm` of the live vocabulary arities), the stack (counted from the
   condition namespace). Proof obligation: the derivation must
   REPRODUCE the landed table exactly, so nothing is repriced.

2. **MEASUREMENT** — the number is a reading of the live board, taken
   fresh instead of frozen. The difficulty ladder's own header already
   promises this: *network-rate adjustment lands with the mesh (N6)*.
   `hunger()` already does it (derived from the line's own rhythm, no
   constants). Prices, pacing and reserves belong here: a reserve target
   should be read off measured volatility, not written as `8`.

3. **THE CHAIN** — the number is operator CONTENT, and content is minted
   by receipt (`28_VOCABULARY`'s door, THE DEFINITION LAW's shape).
   The roster's 76 siting literals are data about 19 lines, not law;
   `placement()` already derives positions for registered lines, and the
   founding lines can walk the same road. Data on the chain is not a
   constant in the codebase — it is a deed with provenance.

What remains after the three sweeps is **physics with a named ground**
(T⁴ bands cite thermodynamics) and **placeholder parameters explicitly
marked for a balance ruling** — and the census pin keeps the count
honest so "placeholder" cannot quietly become "permanent."

---

## §4 THE ORDER TO DO IT IN

1. **The shape refusals** (~20 arms) — each becomes a constructor;
   highest value per edit, no wire impact, and every kill shrinks the
   judge.
2. **The conductor's pacing constants** — flagged in `PICKUP.md` §8c as
   *"no cited law"* already; convert to measurements of the board.
3. **The roster siting** — derive from the sigil unless
   operator-pinned; the pinned seats (Furnace, Frost) are deliberate
   physics fixtures and stay.
4. **The judge's bounds** (`mint_bound`, the ladder) — last, because
   they price the mint and the mesh's measurement layer (N6) is their
   ruled destination; moving them before the mesh exists would be
   inventing the measurement.

The census is the progress meter: `const` declarations and `ratio`
literals in `src/`, counted by pin, expected to fall monotonically.

---

## §5 THE SEMANTICS RULING — machinery speaks machine words

**Operator, 2026-08-05:** *why are we judging/lawful — that is not a
mechanical state of machinery; that's esotericism with bias bleeding
over into our semantics.*

Correct, and the defect ran deeper than the functions: the VOCABULARY
itself imported an authority frame. A judge has discretion; a verdict is
pronounced; "lawful" appeals to an order outside the machine. A machine
has none of those — a transition is DEFINED or UNDEFINED, a shape
CONSTRUCTS or FAULTS, a gate ADMITS what the relation contains. Naming
the mechanism after a courtroom is the same bias LAW 9 bars from
premises, bleeding in through the identifiers.

**Retired from machinery (renamed 2026-08-05):**

| was | now | why |
|---|---|---|
| `FoldVerdict`, `GateVerdict`, `ShapeVerdict`, `RungVerdict` | `…Reading` / `…Fault` | a machine measures and faults; it does not pronounce |
| `judged_fold` | `gated_fold` | the fold gate is a mechanism |
| `binding_lawful` | `binding_trades` | named for what it computes |
| `is_lawful`, `lawful_deltas` | `is_admitted`, `admitted_deltas` | membership in the relation |
| Difficulty::lawful, VolDifficulty::lawful (struck names, plain-text so the citation lint reads history as history) | `well_formed` | the formal-language term |
| "judge/judged/judges" in src comments | gate vocabulary | 71 files swept |

**Kept, deliberately:** `admits` (a turnstile admits — membership, no
discretion), `Refused` (a lock refuses a key), `Reading`/`Read` (the
tree's own measurement vocabulary, already everywhere).

**The boundary:** the DOCUMENT layer keeps "law" as a genre word — a
charter is a document, and the operator's own workflow names charter-law
docs. What is retired is the courtroom as a description of MACHINE
STATE. The doc layer's own prose sweep ("the judge" as a character in
chapter prose) is the doc lane's follow-on, not this ruling's scope.
