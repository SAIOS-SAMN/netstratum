# NS-7 — anchor: WWW3 Settlement Bridge

**Status:** stub — opens at landing N7 (after N6: anchoring needs a converged
mesh to compute the root it anchors). First consumer: minting a rare world /
settling a macro-trade whose ownership anchors to an external chain.

The glue between WWW4 and WWW3. The mesh periodically computes one consensus
root of the NS-3 chronicle and posts that single root to a WWW3 chain as an
immutable checkpoint. The chain settles; it never simulates (Charter §7 scope
rule — no game logic in contract bytecode). WWW3-side contracts are thin
adapters (ERC-721/1155/account-abstraction) mapping wallet addresses to
ipsal-os entity IDs. The three safety seals (Charter §7) bind every section
of this spec. Working ancestor: the two-chain architecture in
`/opt/repos/convergence-engine/DIMENSIONAL_BLOCKCHAIN.md` (temporal anchors +
dimensional receipts).

## 1. RFA semantics
*(unwritten — consensus-root derivation from the chronicle; anchoring
intervals and high-value trigger conditions)*

## 2. Wire format
*(unwritten — the root's on-chain encoding + the adapter ABI boundary)*

## 3. Test vectors
*(unwritten — a real chronicle segment → its computed root → a verified
round-trip through a local testnet adapter)*
