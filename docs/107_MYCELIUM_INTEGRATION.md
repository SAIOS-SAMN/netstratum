# 107 — Integration with Mycelium: how Netstratum keeps time on the network

Prepared by the Mycelium project (2026-10-30) from a read-only survey of
Netstratum at `c916737`, against Mycelium `v0.2.0` and its specification
`docs/MU-CHRONOMETRY.md`; adopted here. Status: open items stand as listed;
item 5 is closed (Netstratum pins Mycelium `v0.2.0` from GitHub, `5c725ed`).

## How Netstratum keeps time on the network

- **Free on its own axis.** The world runs on its conductor's beat `u`
  and meets TUniversal only through ledger entries
  (`engine/conductor/src/main.rs:679-684`; docs/67 N8).
- **Reads final points** through `mycelium::sdk::Session::tuniversal()`,
  wrapped by `Seat::tuniversal` (`engine/hypha/src/seat.rs:158-161`), and
  asks the court with `encode_clock_request()` when it has none
  (`refound.rs:153`). `ns-hypha` relays the greatest point any seat read
  to `<var>/tuniversal` (`engine/hypha/src/main.rs:569-620`).
- **Stamps percepts** with the house's final point and `local_time = u`,
  so seated minds are located at `(TPoint, u)` (`house.rs:371-375`,
  `wire.rs:90`).
- **Internal clocks.** A keeper hierarchy (epicenter, core, mantle, rim),
  each at rate `r / (r + M(<r))` with a light delay, journalled in a
  hash-chained `chrono.jrn` every `t_u_bells` beats. Only the epicenter
  entry records the network height (`engine/conductor/src/chronometry.rs`).
  This is the bridge-keeper pattern of MU-chronometry §2.1.
- **Anchors.** One ledger entry (78.154 sub-kind 12) with the domain,
  `u`, a Merkle commitment over its checkpoints, and the final point,
  posted at re-founding (`refound.rs:56-115, 193`).
- **Map.** Gates are mapped as `MapRecord::LocalThroat` citing a final
  point, only when the point's origin matches the last re-founding
  (`refound.rs:117-127, 268-334`).

## Open items against MU-chronometry §7

1. **Anchor on every committed save, not only at re-founding.** With one
   ledger entry the court has a position for the universe but no dilation
   and no history (§3.4 rule 1).
2. **Freshness in final-point advances.** Mouths are mapped with
   `freshness = FRESHNESS_CELLS = 1`, in cells of hub drift
   (`engine/conductor/src/ascension.rs:54-56`). The court counts a
   mouth's `N` in final-point advances (78.156 §4), so a crossing at that
   mouth is refused as not yet at h unless the latest anchor is within one
   advance.
3. **Crossings' reconciliation edges.** In-game crossings are stepped on
   `u` alone. Crossings between universes should be applied in an anchor
   (`applied`), so their edges close and residuals can be read.
4. **Keep the whole point.** The conductor parses only `h` from the
   printed point (`main.rs:686, 695, 928`). Storing the full point keeps
   the four lanes and `K` on the record.
5. **Re-vendor Mycelium at a tagged release (closed, `5c725ed`).** It was a
   worktree of a Mycelium checkout that no longer existed; Netstratum now
   takes `mycelium` and `forms` from the published release. The clock API it uses (`TPoint`, `Origin`,
   `advances`, `print`, `LedgerEntry`, `LocalThroat`,
   `encode_clock_request`) is unchanged in `v0.2.0`:
   `https://github.com/SAIOS-SAMN/mycelium`, tag `v0.2.0`.
6. **Name the in-game odometer apart.** The base-60 counter
   `TUniversal{S,G,T}` (`engine/ledger/src/lib.rs:46-60, 150-224`) shares
   the network clock's name and lane letters (MU-lexicon §4).

## What Netstratum adds that Mycelium may adopt

- The keeper hierarchy with slip residuals, bridged at one keeper, is the
  pattern MU-chronometry §2.1 recommends and plans as a named form in
  `forms` (Roadmap track 20, K3).
