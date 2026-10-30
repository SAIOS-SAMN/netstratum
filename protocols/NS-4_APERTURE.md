# NS-4 — APERTURE: the controlled opening

*(Renamed 2026-07-28. First "BEP / Bubble Exchange Protocol" — dropped: "BEP"
collides with Binance BEP-20/BEP-2 token standards, unacceptable given the
crypto settlement model at NS-7. Then briefly "membrane" — dropped as too
organic. An APERTURE is a controlled opening whose width is the access
granted.)*

**Status:** partial — the **value carrier** (§1–§3) landed 2026-07-29
(APERTURE-1, tags 47–48): banked value crosses between worlds as a
two-legged receipted deed. The **facet machinery** (§4) — sharing bounded
state at a granted read depth — stays ahead at N5; the market board
(BUILD_ORDER §3a) opens on the carrier landed here.

Sharing is opening an **aperture** onto bounded state at a granted **facet**.
A facet is one cut depth of the whole — `summary → detail → full` — and the
aperture's width is the deepest facet the recipient may read: facet granted is
facet readable, nothing more. An offer carries its facets, its permissions,
its expiration, and a re-share flag. Supersedes PROTOCOL_STACK.md §4 and
INFORMATION_BUBBLES.md.

## 1. RFA semantics — the two-legged deed

An aperture transfer moves banked value from an orb in one world to an orb
in another, conservation-exact across the pair. Unlike TRADE-1 (one
chronicle proves conservation), **no single chronicle proves it here**:
each world receipts only its own leg, and the shared **deal id** links the
legs — double entry across two books.

- **The source leg** (`ApertureSent`, tag 47) debits the source orb's bank.
- **The destination leg** (`ApertureReceived`, tag 48) credits the
  destination orb's bank and relieves a famine strike at the landed 02
  §2.4 relief law (the TRADE-1 shape).
- **The touch law (SINK-1):** both orbs settle due demurrage at the deed's
  stamp before value moves. Affordability is judged on the source's
  **post-decay** bank, computed pure — a refusal moves nothing and orphans
  nothing in either world.
- **Replay** is per-world (SEAL B holds within each chronicle): the source
  leg replays as a bank draw, the destination leg as a bank credit; each
  leg re-anchors its orb's `last_touch` at the deed stamp. Pair-level
  conservation (`Σ sent(deal) = Σ received(deal)`) is an N6 mesh-audit
  reading, not an orb law — the same boundary as the cohesion books'
  universe composition (OPEN_QUESTIONS Q2.3).
- **The judgment boundary (09 board law):** the cross-empire cohesion
  books (compassion/morality) do not post on an aperture — cross-world
  reading composes at N5/N6. **An aperture carries value, never
  judgment.**
- Amounts are exact-rational and strictly positive; a zero or negative
  amount, an unknown orb on either side, or an unaffordable draw is a
  typed refusal (`BadTransfer` / `NoSuchOrb` / `CannotAfford` naming the
  exact shortfall).
- **The shield (THRESHOLD-1, 12_UNION §7):** an aperture touches only
  worlds whose peoples have made the solar leap (TECH-1a) — a pre-solar
  world is unreachable in either direction (`Shielded`, naming which
  side stands shielded; a refusal moves nothing anywhere). A world is a
  star system, so every aperture deed is interstellar by construction:
  a world's first lawful aperture is its **crossing** into the union —
  a replay-derived committed reading (`has_crossed`), never a flag.

## 2. Wire format

Both legs ride NS-1 TLV (`tag u8 ‖ LE32(length) ‖ value`; unknown tags
skip, SEAL A). The ratio carrier is NS-1's canonical form (sign u8 ‖
LE32-length magnitude pairs, coprime, positive denominator — equal values
encode equal bytes).

| tag | meaning | value layout |
|---|---|---|
| 47 | event: ApertureSent (APERTURE-1) | LE64(deal) ‖ LE64(from_orb) ‖ resource u8 ‖ ratio (amount) — the source leg: banked value left this world for another; replay sees only the debit |
| 48 | event: ApertureReceived (APERTURE-1) | LE64(deal) ‖ LE64(to_orb) ‖ resource u8 ‖ ratio (amount) — the destination leg: banked value arrived from another world, bound to the same deal; replay sees only the credit |

The deal id is caller-assigned and world-external (the conductor or a
market landing mints it); the engine treats it as an opaque LE64 link.

## 3. Test vectors

Worked leg — `ApertureSent { deal: 7, from_orb: 1, resource: Iron, amount: 5 }`:

```
2f 1c000000                                  tag 47, len 28
0700000000000000                             LE64 deal        = 7
0100000000000000                             LE64 from_orb    = 1
01                                           resource         = Iron (1)
00 01000000 05 01000000 01                   ratio            = 5/1
```

`ApertureReceived` is byte-identical in layout under tag 48 (`30`), with
`to_orb` in the second LE64. Witnesses (`engine/cfe/tests/world_aperture.rs`):

- `an_aperture_conserves_across_two_worlds` — 5 iron crosses; source
  debited to 15, destination credited to 25, the pair minted nothing; one
  leg in each chronicle, none crossed; the legs share the deal id.
- `an_aperture_refuses_typed` — zero amount, unknown orbs, and
  over-draws refuse with the typed refusals; nothing moves in either
  world on any refusal.
- `both_legs_replay_in_their_own_world` — each world's replay reproduces
  its own leg's bank state from receipts alone (SEAL B per chronicle).
- `the_aperture_legs_round_trip_byte_exact` — tags 47/48 encode/decode
  byte-exact, the canonical-ratio law included.

## 4. The facet machinery (ahead — N5)

*(unwritten — the facet ladder as nested exact-rational projections; the
aperture width as the readable-facet bound; permissions, expiration, the
re-share flag. The N5 landing; the market board's OFFER-1 prices what §1
carries.)*
