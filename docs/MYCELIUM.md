# Mycelium

This document states how Netstratum uses [Mycelium](https://github.com/SAIOS-SAMN/mycelium):
what it takes from Mycelium, where each use runs, how complete each one is,
and where the line falls between Netstratum's physics and Mycelium's. It is
read from the code; file references are given so each statement can be
checked. How Netstratum keeps time on the network is detailed in
[107_MYCELIUM_INTEGRATION.md](107_MYCELIUM_INTEGRATION.md); how the physics
is used is in [PHYSICS.md](PHYSICS.md).

The pin is in the root `Cargo.toml`. Moving every import to `mycelium-sdk`
is pull request #24, held for Mycelium v0.3.0-rc.3; the paths below are
those on `main`.

## 1. The line between Netstratum and Mycelium

| | Mycelium | Netstratum |
|---|---|---|
| **Owns** | The network: the court (hyphae), the network clock (TUniversal), seats and their mail, domains, origins and reservations, the atlas, folds and their finality, μ and its lanes; the shared law forms (`forms`); the verification of a transition against its laws in Cl(4,1)(ℚ); rule elements; the client conformance read | The universe (era 1, `conductor`) and the game's worlds (era 0, `cfe`): every law of matter, motion, heat and formation; its own clocks; the house that offers seats; the attachment of external agents |
| **Contract between them** | The SDK surface at a tagged release; the law ids and versions each kernel declares ([EQUATIONS.md](EQUATIONS.md), e.g. LAW-WAVEFRONT v1); each universe's own unit, stated at founding (Concordance 1.8/8.5, proposed); the wire version and act table | |

Netstratum's universe physics is its own: no era-1 module except
`compound.rs` imports from Mycelium (§3). Mycelium's physics enters
Netstratum in one place on the live path, the share-compounding law (§2.2).

## 2. What Netstratum takes from Mycelium

| Use | Code | State |
|---|---|---|
| Census folds to the court | `ns-hypha follow`: `census.rs`, `fold.rs`, `follow.rs` | **Live** (the follower unit; currently stopped, no court named). Folds net holdings per identity and lane from era-0 colony chronicles; the court checks their continuity, not the physics |
| The share-compounding law | `conductor::compound` re-exports `forms::…::share_compounds::Remaining`, the run product `(1 − x)^n` | **Live**: fuel burned per echo and the spent-star test (`breath.rs`), shell cooling (`formation::cooled`), stellar lifetimes (`main.rs`) |
| Seats and the house | `ns-hypha seat`: `house.rs`, `attached.rs`, `wire.rs`; `engine/attach` | **Built, not running** (docs/109 A8). Lists seats, hands out tasks, files solves with their rule tapes, re-founds the domain, origin and reservation, maps gates as throats |
| The network clock | `ns-hypha seat` keeps the greatest final point and writes `<var>/tuniversal`; the conductor reads it (`main.rs:908-934`) | **Live only while the seat runs**. The conductor parses the point by hand, not with the SDK's `TPoint::parse` |
| Verification of transitions | `ns-hypha` `verify_local`, `verify_both` (`lib.rs:82, 180`) | **Diagnostic only**: called from the `probe` subcommand, on one fixed transition of two unit bodies (`probe.rs:49-70`). No Netstratum state, era 0 or era 1, is ever turned into a transition |
| Rule elements | `engine/tongue::translate`, called at `conductor/src/lib.rs:4416-4418` | **Half-wired**: up to two words a beat are translated; only whether a translation exists is kept, for a log count. The seat does not speak through it, although `tongue`'s header says it does |
| Origination geometry | `fold.rs:63, 268-280` reads `pulse.bin` tag 17 (`T_u`) for `Geometry::Sphere` | Once, at origination: the only era-1 value the court receives |

## 3. Defects in the integration

1. **Verification never touches real state.** The only transitions built
   are the probe's (`probe.rs`). The universe's motes, bodies and steps are
   never offered to Mycelium's verifier, so nothing on the network
   attests that Netstratum's physics was followed.
2. **Agreement counts unmeasured laws as agreeing.** `Reading::agree`
   compares only the laws a verdict breached (`lib.rs:56, 68`); a law read
   as `Undecomposed(Unbounded)` on one side and breached on the other is
   dropped from the comparison.
3. **docs/109 A6 overstates.** "Verification of attached transitions —
   built" is the census fold: receipts folded for continuity, not
   transitions verified against laws.
4. **`tongue` is half-wired** (§2): its result is discarded; `hypha` lists
   it as a dependency and never uses it; `seat.rs:29`'s `not_a_rule` counter
   is never incremented.
5. **Duplicates of what Mycelium provides:** the conductor's hand parse of
   the final point; `engine/substrate`'s Clifford multivector beside the
   SDK's `Mv` (only the probe and `tongue` use `Mv`).
6. **A false comment:** `engine/conductor/Cargo.toml` says the flows read
   Mycelium's `physics::source`; no conductor code does.
7. **Imported and unused:** `Flow`, `Flux`, `verify` (house), `forms::frame`,
   `forms::shares::closeness`; `rule::Physics` only in an example.

## 4. What a correct integration needs

These are the decisions and the work the defects call for, in the order
of [PLAN.md](PLAN.md); each is an issue on the project ([GitHub issues](https://github.com/SAIOS-SAMN/netstratum/issues),
label `cross-project` where Mycelium is involved).

1. **Decide what the court verifies.** Either the era-1 universe states its
   steps as transitions under Mycelium's laws (its bodies as `Body`, a
   bell's motion as the transition's rotor), or the court verifies
   Netstratum's own laws by their declared ids and versions, and the probe
   is retired. Until one is chosen, the verification code is a diagnostic
   and is documented as one.
2. **Fix the agreement rule** so an unmeasured law on one side is a
   disagreement, not a match.
3. **Wire `tongue` or remove it:** the seat speaks its answers through it, or
   the translation and its dependency leave the beat.
4. **Use the SDK's own parts:** `TPoint::parse`; one multivector (the SDK's)
   where Netstratum needs Mycelium's geometry.
5. **Correct the record:** docs/109 A6, the Cargo comment, and the unused
   imports.
