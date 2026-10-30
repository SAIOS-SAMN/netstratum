# 28 — The minted vocabulary: how a class of entity comes into being

**Status:** **law — landed 2026-08-05** on the operator's order *execute
on V0–V4*, which is the amendment ruling §7 was waiting for. Drafted the
same day from the order *open this up to an end user with permissions;
eliminate the hardcoding of these classes while still letting the engine
create NPCs.*

All five slices are built and witnessed in
`engine/cfe/tests/vocabulary_pins.rs` (14 pins):

| slice | what landed |
|---|---|
| V0 | the class carries a mechanism — the found vacuity paid |
| V1 | the ceilings are readings of the fold, not constants |
| V2 | `VocabularyMinted` **tag 96**, four gates, wire round-trip |
| V3 | the cleave draws from the fold at its own receipt |
| V4 | `t0 vocab` and `t0 mint` in the progenitor console |

Subordinate to `00_CHARTER.md`, `26_ARTICLES.md` and `27_EVIDENCE.md`.
Governs `15_TAXON` §2/§3 (the class and clade vocabularies),
`04_CIVILIZATION_GENESIS` §1 (the axis vocabularies) and PRACTICE-1.

---

## §1 The law

> **A definition is compiled at genesis or minted by receipt.
> It is never filed.**

This extends the definition law (`conductor::polities`, 2026-08-05)
rather than reversing it. That ruling struck `polities.json` on three
measured grounds, and it is worth being precise about them, because the
extension lives exactly where all three fail to bite:

| the ground | does it bite a chain-minted definition? |
|---|---|
| two truths drift | **No.** There is one truth: the chain. |
| a file is not consensus-safe — two nodes, two universes | **No.** Every node replays the same receipts and folds the same vocabulary. |
| the parser failed silently | **No.** A receipt is decoded by the codec, and a malformed one refuses at the door. |

The chain is the one place where *what exists* and *what happened*
legitimately coincide, because a minting is something that happened. A
file could never say that; a receipt says nothing else.

So the shape is the roster's shape, generalized: **compiled seed ∪
chain-minted growth.** Genesis has no chain to read yet, so the founding
vocabulary compiles; everything after genesis is minted.

**This is also the motion clause 1 on a second surface.** *Continuous
novel work; no finite wells.* The class vocabulary is a well of 8 and
the clade vocabulary a well of 20 — bounded, exhaustible, and raisable
only by someone with commit access. That is the same defect
`FORGE_BUDGET`'s finite word-vocabulary has, and it wants the same cure.
The two are one problem wearing two hats.

---

## §2 What is closed today — measured, not estimated

| vocabulary | size | gate | who can raise it |
|---|---|---|---|
| disposition | 3 | `RegistryRefused::UnknownAxis` | a coder |
| metabolism | 3 | `RegistryRefused::UnknownAxis` | a coder |
| perception | 4 | `RegistryRefused::UnknownAxis` | a coder |
| class (archetype) | 8 | `dop::class_ceiling()` | a coder |
| clade | 20 | `dop::clade_ceiling()` | a coder |
| practice | 5 | in-value codes, append-only | a coder (cheaply) |
| station (office) | 10 | `registry::STATIONS` (T0–T2 band + T4–T9 economic; Wave-1) | **see §10 — not mintable** |

Every row says "a coder". That is the hardcoding the order names, and it
means the board's imaginative surface is bounded by whoever holds the
repository rather than by whoever holds a progenitor seat.

---

## §3 The vacuity found — nothing reads the class

**Measured 2026-08-05, and it changes the design.** Three sites in the
engine match on a `class`: `dop::archetype_sigil`, `dop::archetype_word`
(both added the same day, both renderings), and `dop::spine_bands` —
which matches `thermal::StarClass`, an unrelated type.

**No mechanic branches on the sealed taxon class.** It is sealed by
receipt (`TaxonSealed`, tag 85), it rides the snapshot, it replays — and
nothing in the world reads it to decide anything. Compare the axes,
which are load-bearing: metabolism drives 8 branch sites, disposition 7,
perception 6.

By `27_EVIDENCE` law 8's own test — *a term that computes is kept; a
term that only resonates is excised* — the class partition today is a
term that only resonates.

**Two consequences, and the second is the whole design.**

1. The vacuity is pre-existing and is recorded here rather than quietly
   fixed, per the house rule.
2. **An editor that adds a word would be adding decoration to
   decoration.** A minted class must arrive carrying a mechanism, or the
   feature manufactures vacuity at user speed — which is precisely what
   `26_ARTICLES` Article II and the motion clause 6 forbid.

---

## §4 The binding — an archetype mints with its mechanism, or not at all

A minted vocabulary entry carries a **binding**: a vector of exact
rational modifiers over a closed set of channels.

```text
    Binding = [ (channel, factor) ... ]        factor ∈ ℚ, factor > 0
```

**Why this shape and no other.** It is data, not code — so a mint needs
no sandbox, no interpreter, and no new capability class. It is exact
rationals — so it obeys the no-floats law and replays bit-identically.
It ranges over a closed channel set — so a mint changes reachability,
never what the engine can express. That is the nursery design's first
law (`[[nursery-vocabulary-minting-idea]]`) applied to identity instead
of to solver operators: *a mint changes reachability, never capability
class.*

**The channels are the ones that already exist.** No mint may name a
channel the engine does not already compute:

| code | channel | where it computes |
|---|---|---|
| 1 | zone yield | `world/tick.rs` `yield_factor` |
| 2 | zone upkeep | `world/tick.rs` `upkeep_factor` |
| 3 | survey draw | `world/survey.rs` `scan_draw` |

**As built the table is the three above and no more.** Research rate and
route friction were drafted here and are not declared, because neither
is wired — and a channel a binding may name while nothing reads it is
the very vacuity §3 exists to pay off. The table grows by landing a
channel, and this is the first place that rule was applied to itself.

### 4a. One benefit, one friction — PRACTICE-1's law, generalized

PRACTICE-1 already rules that a discipline carries *one benefit, one
friction, exact, per discipline*. Every one of the five practices obeys
it, measured: the Austerity trades habitat yield for extractor upkeep,
the Showpiece buys yield at double upkeep, the Overdrive raises yield
and upkeep together.

**A binding with no factor below 1 is refused.** A binding with no
factor above 1 is refused too — that is not a class, it is a penalty. So
the editor cannot mint a strictly-dominant archetype, and the refusal is
structural rather than a balance patch applied later.

### 4b. Bounded

Each factor lies in `[1/4, 4]` (placeholder, operator-tunable). No mint
can be a hundredfold. The bound is on the factor, never on a directional
quantity, so `24_FORM` §2b is untouched.

---

## §5 The standing gate — already chartered, not invented

Minting is a **tier 0** power. This needs no new permission concept:
`12_UNION` ruling 4 lists the progenitor powers as *create a new planet,
modify quantum states, issue a technology leap, create new spaces,
create a new galaxy*, and §2a rules that *the progenitors are the top —
they contract and expand the board, they create it. Everything else
plays on it.*

**An archetype editor is that power made concrete.** The gate is
`ledger::registry::tier_of(station) == Some(Tier::T0)`, which is already
the engine authority for tiers rather than a tools-side mapping.

Two properties come free from the existing law:

- `tier_of` returns `None` for an unrecognised station and `None` is a
  refusal, never a fall-through to the common band — so an unreadable
  seat cannot mint.
- The progenitor band has a seat width (8 per domain), so the number of
  minters is bounded by the seat law and not by this document.

**The delegation law applies unchanged** (`12_UNION` §7): the power to
create is sovereign; handing what was created to another line is
council-gated.

---

## §6 The novelty gate — measured, never asserted

A mint is refused unless its binding **differs from every binding
already in the vocabulary**. Word and sigil differences do not count.

```text
    mints  ⟺  ∀ e ∈ vocabulary(axis) :  binding(candidate) ≠ binding(e)
```

Exact rational vectors compare exactly, so this is a total, cheap,
deterministic test with no tolerance parameter to argue about.

**Why it is the load-bearing gate.** Without it the editor is a rename
tool: a hundred archetypes, one mechanism, and a partition that
distinguishes nothing. The motion clause 6 — *nothing vacuous* — is the
law, and this is its arithmetic form.

**Sigil uniqueness is separate and also required.** `every_archetype_-
carries_a_sigil_and_a_word` already pins that no two classes share a
glyph; a mint must extend that, not break it. A mark that names two
things defines neither.

---

## §7 The deed — `VocabularyMinted`, tag 96

```text
    VocabularyMinted {
        axis:    u8,               1 disposition · 2 metabolism
                                   3 perception  · 4 class
                                   5 clade       · 6 practice
        code:    u8,               the entry's own code
        sigil:   String,           the mark (required for axis 4)
        word:    String,           the vocabulary word
        binding: Vec<(u8, Ratio)>, the mechanism (§4)
    }
```

**Tag 96**, above every tag in use. This draft said "tag 90, the next
free tag outside reservations, the highest in use is 85" and both halves
were wrong: 90 was already `SovereignUnionSealed`, and the count came
from a regex that matched only `N => Some(Self::…)` arms and missed every
block-form arm. Drafted onto 90, the mint's decoder shadowed the
sovereign union — every such receipt would have become undecodable, a
chain-breaking defect. It was caught by clippy's unreachable-pattern
warning rather than by the tag register, whose own "next free tag" line
had gone stale the day tag 90 was taken. That line now states the free
set and is not a single number anyone can outrun.

**`code` is carried and verified, never chosen.** The minter does not
pick a number — the lawful code is `ceiling(axis) + 1` derived from the
fold, and a receipt naming any other code refuses at the door. Carrying
it keeps the receipt self-describing for a reader who has not folded the
chain; verifying it keeps two nodes from minting colliding codes.

**This is a wire amendment** — a new tag, additive, minor version bump
by `27_EVIDENCE`'s own amendment tier. It is the whole cost of the
feature and it is cheapest now, before the mesh is live.

---

## §8 The fold — the vocabulary is a replay

```text
    vocabulary(axis)  =  seed(axis)  ∪  { every VocabularyMinted on axis
                                          in chronicle order }
```

The compiled ceilings stop being constants and become readings:

```text
    class_ceiling()   ⟶   vocabulary(CLASS).len() - 1
    clade_ceiling()   ⟶   vocabulary(CLADE).len() - 1
    axis_valid(a, c)  ⟶   c ≤ ceiling(a)
```

Same chain, same vocabulary, on every node, forever. This is the
property `polities.json` could never have and the reason the extension
is safe.

**The seed still compiles.** Genesis reads no chain — there is none yet
— so the founding vocabulary is a compiled definition exactly as the
founding roster is. `27_EVIDENCE` law 8 is satisfied on both halves:
the seed computes, and a minted entry computes because §4 requires it
to arrive with a mechanism.

---

## §9 NPC generation — it comes free

The engine already draws NPC axes from the closed arrays at genesis and
at every cleave (`divergence_step` mutates a nonempty subset of
disposition × metabolism × perception). Point those draws at
`vocabulary(axis)` instead of at the compiled constant and **every
minted archetype automatically enters NPC generation** with no further
wiring.

This is the answer to *how the engine can still populate the
universe*: it populates it from the same vocabulary the operator edits,
so a progenitor who mints a class sees NPCs wearing it in the next wave
— and their bindings are live, because §4 made bindings mandatory.

**One law holds the line.** A cleave draws from the vocabulary as it
stood at the drawing receipt, never from today's — otherwise replay of
an old era would produce lines wearing classes that did not exist yet.
The fold is ordered, so this is a property of reading it correctly, not
a new rule.

---

## §10 What is **not** mintable

**Stations.** The four offices — progenitor, watcher, observer,
sovereign — are chartered by `12_UNION` §3 as *stations of function, not
steps of rank*. Minting a new office would be a change to the
constitution of the union, not to its vocabulary, and it would let a T0
seat manufacture the seat that judges it. Offices change by amendment,
by operator ruling, and by nothing else.

**Channels.** A mint selects from the channel table; it cannot add a
row. A new channel is engine work.

**The kill switch.** A `--no-mint` boot flag freezes the door without
touching what is already minted (the nursery design's law 5). A minted
entry is never deleted — the chain does not forget — but the operator
may close an entry by receipt, which stops new lines drawing it while
leaving every line that wears it standing. Retirement is not erasure,
here as everywhere in this tree.

---

## §11 Where "covenant" lands

The operator asks whether *covenant* is a civilization sub-type and
which family it belongs to. Measured against the partition:

**The eight classes are all substrate** — biotic-swarm, geodic-
lithovore, metashifter, telepathic-resonant, pelagic-aerial,
plasma-field, apex-terrestrial. A covenant-people is a social contract
structure, which is not a substrate, so it does not fit the class
partition at all. That is a real finding about the partition rather than
a problem with the word.

Two homes do fit, and they differ in kind:

| reading | home | cost |
|---|---|---|
| *a covenant people* — an identity | a **clade** (`covenantal`) | one ceiling raise today; free under §8 |
| *taking up covenant* — a chosen discipline | a **practice** (code 6) | free today: the practice codes are in-value and append-only |

The mechanism already exists and is measured: `Covenant { obligor,
beneficiary, resource, quota, delivered, collateral, sealed, window }` —
a collateralized forward delivery obligation. A covenant practice would
bind a line to obligations with one benefit and one friction (§4a); a
covenant clade would say the line is constituted by them.

**The tier ruling (operator, 2026-08-05): *drop covenant to T3 and
below*.** The covenant is a **common-band instrument** and `seal_forward`
now refuses any line holding a chartered office.

The reason is in the mechanism rather than in the hierarchy. A covenant
posts collateral against a deadline — that is what the struct is: an
obligor, a beneficiary, a quota, a stake, a window. Collateral exists
because the parties hold no authority over each other and need something
better than a promise. A seated office already has chartered authority:
it decrees, it delegates, it petitions through the council (`12_UNION`
§7). Asking it to post a stake against its own word is a category error,
and letting it do so would let an office buy obligations it could simply
have ordered.

`tier_of(None) = T3` is the only road to the common band (`12_UNION`
§2a), so the gate reads the world's own line and refuses any seat at all
— T0, T1 and T2 alike. An unsealed world holds no line and therefore no
seat, so it is common by construction, which is the same reading
`tier_of` gives. Witnessed in `a_seated_line_may_not_covenant`.

**Recommendation for the name: the practice.** A practice is adopted and released by
receipt, which is exactly the shape of an obligation people, and it
costs no wire change at all. The clade reading is available if the
operator wants covenant to be inherited rather than chosen — and note
the two are not exclusive.

*(Unrelated to the term's other use: `24_FORM` §2m.3 rules that as a
kernel primitive the plain word is **Obligation**, and the code rename
stays a deferred wire amendment. A civilization sub-type named covenant
is game content and is lawful under the archetype/law distinction the
operator drew on 2026-08-05.)*

---

## §12 What this amends

- `15_TAXON` §2/§3 — the class and clade tables become the seed of a
  folded vocabulary, not the whole of it.
- `04_CIVILIZATION_GENESIS` §1 — the axis vocabularies likewise.
- `dop.rs` — `class_ceiling`, `clade_ceiling` and `axis_valid` become
  readings of the fold. **Not yet landed — corrected 2026-08-13 (doc
  audit): all three are still hardcoded constants**
  (`almanac::taxon_table.rs:15-22` returns a fixed 13 / a formula over
  constants 20+25×13; `axis_valid` at `dop.rs:1556` is a plain hardcoded
  match). The mint deed itself (tag 96, encode/decode) is real and
  landed; this fold-wiring is the owed payoff, not yet built.
- `12_UNION` ruling 4 — the progenitor power table gains one row:
  *mint a vocabulary entry — `VocabularyMinted` (tag 96, corrected from
  a stale "tag 90" — see §7)*.
- PRACTICE-1 — its one-benefit-one-friction law is promoted from a
  property of the five landed practices to the binding law of §4a.
