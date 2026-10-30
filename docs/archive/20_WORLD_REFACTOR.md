# 20 — The `world.rs` refactor: what the kernel pulls, what stays

> **Archived 2026-08-13.** Superseded/dead weight per the doc audit — kept for history, not live spec. See docs/BUILD_ORDER.md, docs/archive/HANDOFF_2026-08-13.md, or the doc it names as its successor for current status.

**Status:** ~~spec~~ **landed** (kernel lane, 2026-08-02; confirmed by
measurement 2026-08-04). The split is done: `engine/cfe/src/world.rs`
no longer exists and `engine/cfe/src/world/` holds **19 files, 6,860
lines, largest `mod.rs` at 903**. Everything below is kept as the
inventory and reasoning that produced the split — read it in the past
tense.

~~`engine/cfe/src/world.rs` is 5001 lines holding three unrelated
things.~~ This document is the inventory: what the kernel and the miner
must pull out to function standalone, what stays with the game, and the
seam between them.

~~**Any developer modifying `world.rs` should read this first.** The
file carries a banner pointing here.~~ There is no such file to modify;
the modules carry their own headers.

---

## §1 The measurement

`engine/kernel` (1003 + 566 lines) touches `cfe::world` for exactly
three items:

    cfe::world::Committed        the state type it judges
    cfe::world::demurrage_due    the sealed decay rule (I5 calls it)
    cfe::world::ease_of          the tech modifier that rule reads

Three, out of 105 public functions. The judge is already almost free of
this file. The miner needs more, but not much more.

## §2 What must be pulled — the kernel/miner core

### 2a. The state vocabulary (12 definitions)

`Committed` is the type every transition carries, and it is not
self-contained — it holds nine other definitions from this file:

    Committed        the committed projection itself
    ColonyOrb        (defined in dop.rs — already outside)
    Entity           (defined in ipsal.rs — already outside)
    AbsorptionDebt · Bond · Covenant · Council · Offer
    Genesis · Prospected
    TickOutcome      (proposer-side; see §3)
    Leap             (proposer-side; see §3)

Of these, **seven are pure state** and belong with the kernel's
vocabulary: `Committed`, `AbsorptionDebt`, `Bond`, `Covenant`,
`Council`, `Offer`, `Genesis`, `Prospected`. They are data the judge
reads and the codec encodes. They contain no game logic.

### 2b. The sealed rules — called, never copied

Four rule-functions the judge invokes so that one computation serves
two judges (`CHARTER` §0b):

    demurrage_due     I5's decay bound
    ease_of           the tech modifier it reads
    founding_reserves (in dop.rs — already outside)
    ZoneKind::cost / raze_refund (in dop.rs — already outside)

Only the first two live here.

### 2c. The interpreter — `replay`

113 match arms defining what each deed does to state. **This is the
transition**, and per `46_KERNEL` §3 it is currently a second
definition standing beside `lawful`. It must move out of the proposer's
file regardless of what happens to the duplication, because a reader
cannot presently tell the transition from the game around it.

### 2d. The miner's own emitters

A standalone miner — take work, verify, mint, commit — needs a small
set of the 105:

    World::new / resume    open or continue a world from receipts
    committed()            the state to judge and encode
    chronicle()            the receipts the arbor threads
    mint_discovery_as      emit tag 86 against verified work
    seat_station_as        the vertical axis (tag 89)

Five. Everything else in the 105 is the colony game.

## §3 What stays — the proposer

The remaining ~100 public functions are the game, and they stay
together: founding and chartering, zoning and razing, collecting and
upkeep, the thermal loop, trade and transfers, bonds, covenants,
councils, offers and clearing, the tech tree, practices, prospecting
and assay, wards and veils, gauge shifts, leaps and terraforming,
succession, galaxies, routes.

`TickOutcome` and `Leap` are proposer-side returns and stay with them.

This half has a different reader, a different rate of change and a
different blast radius from §2. That difference is the entire argument
for the seam.

## §4 The proposed shape

    cfe::state       the 7 pure state definitions + Committed
    cfe::replay      the interpreter (113 arms), alone and readable
    cfe::world       the proposer (~100 emitters), unchanged in behaviour
    cfe::rules       demurrage_due, ease_of — beside the other sealed
                     rules already in dop.rs

Mechanical. No semantic change, no behaviour change, no schema change.
Every move is a `pub use` away from breaking any caller.

## §5 Why this is not cosmetic

Three reasons, in order of weight:

1. **The transition is invisible.** `replay` is the definition of what
   deeds mean, and it is buried in the middle of the game's emitters.
   `KERNEL_RELAY` §1.4: *"their meaning lives in `replay()` — an
   interpreter three coders rewrite weekly."* What cannot be found
   cannot be audited.
2. **The kernel's surface is unreadable.** To know what the judge
   depends on, a reader today must scan 5001 lines to discover it is
   three items.
3. **Amendment blast radius.** A kernel amendment must reason about
   §2a and §2c; a game landing touches §3. Sharing a file means every
   change re-reads all of it, and that is the mechanism by which
   unrelated abstractions accumulate in one place.

## §6 Sequence and ownership

Order: §4's split first (mechanical, unblocks reading), then
`46_KERNEL` §3's cure — derive apply from judge where the judge already
knows, pin the remainder red-first.

**`engine/cfe/` is unowned on the lane map.** `world.rs` is the largest
shared file in the tree and every lane touches it. This refactor needs
an owner assigned before it lands, not after.

---

## §7 The split is done — on `capacity-lane`, at `666be81` (assessed 2026-08-02)

**This document's §4 is superseded by work that already exists.**
`capacity-lane` landed *"world.rs becomes an organ: 5,034 lines into
fifteen files"* with the public surface unchanged and all 481 workspace
tests passing. Their shape is finer than §4's four-module proposal:

    mod.rs 889 · tick.rs 641 · replay.rs 621 · economy.rs 485
    accord.rs 409 · core.rs 378 · survey.rs 364 · genesis.rs 342
    state.rs 296 · tech.rs 287 · refusal.rs 151 · gauge.rs 131
    thermal.rs 104 · rules.rs 74 · zones.rs 47

`rules.rs` is the cut §4 asked for and could not name: `ease_of` and
`demurrage_due` in 74 lines, apart from the game. `replay.rs` labels
the second definition as one, which is `46_KERNEL` §3 made structural.

### 7a. The merge, measured

    merge base          b3bf038 (the vertical axis)
    kernel-lane ahead   27 commits
    capacity-lane ahead 12 commits
    textual conflicts   one — docs/26_ARTICLES.md

**The single conflict understates the work.** `capacity-lane` deleted
`world.rs`; `kernel-lane` modified it heavily. Git sees add/delete, not
a conflict, so the merge would silently drop every kernel-lane edit to
that file. What must be transplanted by hand:

    rules.rs      ease_of → epoch dispatch (ease_cliff + ease_gradient)
                  demurrage_due → gains the `supply` table
                  standing_supply → new, sealed, serves both judges
    call sites    13 × ease_of(.., stamp)
                  12 × demurrage_due(.., &supply) / touch(.., &supply)
                  6 × the standing_supply snapshot at function head
                  now spread over economy · tick · genesis · survey · core

**The split makes this easier, not harder** — `rules.rs` is 74 lines
where the monolith was 5,034.

**Direction: kernel-lane moves onto capacity-lane's structure.**
capacity-lane deleted the file; rebasing kernel-lane's edits onto the
organ is the only merge that preserves both. The reverse re-creates the monolith.

### 7b. A discrepancy to reconcile before merging

`capacity-lane` records a measurement that contradicts this lane's
framing of #30:

> *Replay's body recomputes exactly one shared rule —
> `dop::commensurate_weight(resource)` in `world/replay.rs`. **It never
> calls `ease_of` or `demurrage_due`** — those arms apply the receipted
> amount. So the double-definition exposure is one call site rather
> than 113 arms.*

If replay only replays receipted amounts, live and replay agree by
construction and changing `ease_of` could not have diverged them. **But
it did, empirically:** `conductor/tests/survey_pins.rs` failed on
`d52f3fd` with live `Crystal 127/16` against replay `15/16`, and
bisection tied it to `ease_of`. Both cannot be right as stated.

Most likely both hold and describe different things — replay may not
call the rule directly while the divergence travels through a
tick-computed consequence. **Unresolved, and it must be resolved before
the merge**, because it decides whether #30 is one call site or 113
arms, and therefore how much of `46_KERNEL` §3 is real work.

The epoch mechanism (`65963b4`) closed the observed divergence either
way, so nothing is blocked on the answer — only the size of #30 is.
