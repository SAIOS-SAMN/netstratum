# 20 — THE `world.rs` REFACTOR: what the kernel pulls, what stays

**Status:** SPEC (kernel lane, 2026-08-02). `engine/cfe/src/world.rs`
is 5001 lines holding three unrelated things. This document is the
inventory: what the KERNEL and the MINER must pull out to function
standalone, what stays with the game, and the seam between them.

**Any developer modifying `world.rs` should read this first.** The file
carries a banner pointing here.

---

## §1 THE MEASUREMENT

`engine/kernel` (1003 + 566 lines) touches `cfe::world` for exactly
three items:

    cfe::world::Committed        the state type it judges
    cfe::world::demurrage_due    the sealed decay rule (I5 CALLS it)
    cfe::world::ease_of          the tech modifier that rule reads

Three, out of 105 public functions. The judge is already almost free of
this file. The MINER needs more, but not much more.

## §2 WHAT MUST BE PULLED — the kernel/miner core

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

113 match arms defining what each deed DOES to state. **This is the
transition**, and per `19_KERNEL` §3 it is currently a SECOND
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

## §3 WHAT STAYS — the proposer

The remaining ~100 public functions are the GAME, and they stay
together: founding and chartering, zoning and razing, collecting and
upkeep, the thermal loop, trade and transfers, bonds, covenants,
councils, offers and clearing, the tech tree, practices, prospecting
and assay, wards and veils, gauge shifts, leaps and terraforming,
succession, galaxies, routes.

`TickOutcome` and `Leap` are proposer-side returns and stay with them.

This half has a different reader, a different rate of change and a
different blast radius from §2. That difference is the entire argument
for the seam.

## §4 THE PROPOSED SHAPE

    cfe::state       the 7 pure state definitions + Committed
    cfe::replay      the interpreter (113 arms), alone and readable
    cfe::world       the proposer (~100 emitters), unchanged in behaviour
    cfe::rules       demurrage_due, ease_of — beside the other sealed
                     rules already in dop.rs

Mechanical. No semantic change, no behaviour change, no schema change.
Every move is a `pub use` away from breaking any caller.

## §5 WHY THIS IS NOT COSMETIC

Three reasons, in order of weight:

1. **The transition is invisible.** `replay` is the definition of what
   deeds mean, and it is buried in the middle of the game's emitters.
   `KERNEL_RELAY` §1.4: *"their MEANING lives in `replay()` — an
   interpreter three coders rewrite weekly."* You cannot audit what you
   cannot find.
2. **The kernel's surface is unreadable.** To know what the judge
   depends on, a reader today must scan 5001 lines to discover it is
   three items.
3. **Amendment blast radius.** A kernel amendment must reason about
   §2a and §2c; a game landing touches §3. Sharing a file means every
   change re-reads all of it, and that is the mechanism by which
   unrelated abstractions accumulate in one place.

## §6 SEQUENCE AND OWNERSHIP

Order: §4's split first (mechanical, unblocks reading), then
`19_KERNEL` §3's cure — derive apply from judge where the judge already
knows, pin the remainder red-first.

**`engine/cfe/` is UNOWNED on the lane map.** `world.rs` is the largest
shared file in the tree and every lane touches it. This refactor needs
an owner assigned before it lands, not after.
