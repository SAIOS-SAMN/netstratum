# engine/ — the CFE (Canonical Field Engine)

Opened at landing N2. `cfe` is the runtime: `ratio` (the RFA carrier and its
canonical wire encoding), `tlv` (the additive wire, SEAL A), `vrp` (NS-3:
receipt chain + attested world clock), `dop` (NS-1: the colony orb), `world`
(the tick loop; the committed/workspace boundary; replay).

Witnesses: `tests/` (NS-3 conformance against the inscribed vectors, NS-1
wire round trips, world replay). Vector extraction: `examples/ns1_vectors.rs`.
Before any new `fn` or `type`, grep this crate and consult
`/opt/ipsal-autic-v0.4/docs/GROUND.md` — compose, don't re-derive.
