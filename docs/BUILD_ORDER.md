# BUILD ORDER — the one carrier

**LAW: the current landing is read HERE, never inferred.** One landing at a
time; red-first with recorded output; spec + mechanic + test vectors in the
same landing (CHARTER §1); commit on explicit operator go.

## 0. The landed ledger

- **N0 LANDED** (`bd70ed2`, 2026-07-25): the scaffold — charter (Dark Forest
  constraints, §7 bridge governance, §8 terminology law), this carrier,
  NS-1..NS-7 stubs, the economy at design stage.
- **N1 LANDED** (2026-07-25): NS-3 VRP written FROM the working v0.4
  chronicle organ. Evidence: the sealed build driven read-only (path
  dependency, fixture identical to the landed witness); the tick law measured
  (3-of-3 ticks; 2-of-3, skip both refuse, clock unmoved); three receipts
  extracted byte-exact (genesis parent zero; chain `Intact`); tamper
  divergence measured (one changed u64 → digest `c86c17aa…` → `366aa785…`).
  Vectors are self-contained in NS-3 §3 — any implementation verifies from
  the document alone. Ruling carried: the normative TLV event encoding lands
  WITH the first engine implementation at N2 (the event vocabulary cannot
  precede the actions that close).

## 1. The board

| # | Landing | Builds | Proven by |
|---|---|---|---|
| **N0** | THE SCAFFOLD | the monorepo tree, the charter, this carrier, protocol stubs | the tree exists; the charter read whole |
| **N1** | NS-3 VRP, spec-from-working-code | the receipt spec written FROM the v0.4 chronicle organ (BLAKE3 append-only, parent-linked, quorum-stamped) with test vectors extracted from real chronicle output | the spec's vectors verify against a fresh engine implementation; tampering/reordering refuses typed |
| **N2** | NS-1 DOP + THE FIRST SLICE | one entity, one colony orb, CFE state ticks in the CLI; every action emits an N1 receipt | a playable tick loop; the orb round-trips its wire codec byte-exact; the chronicle replays the session |
| **N3** | NS-6 IPSAL-OS, first cut | the entity/profile container: identity, permissions, generation tagging for one human + one scripted agent | the agent acts only inside its granted scope; every agent action receipted with generation tag |
| **N4** | NS-2 RQP + THE SCANNER | resonance query over the orb population (sigil-address lookup, never a scan) | a structural query finds the planted match and refuses the planted near-miss |
| **N5** | NS-4 BEP + SHARING | detach a bubble (sector map / trade offer) with permissions and multi-depth content | recipient reads the summary, zooms to detail, cannot exceed granted depth; re-share honors the flag |
| **N6** | NS-5 MESH + TWO NODES | proof-of-convergence sync between two independent engine instances | two laptops converge on one world state by exact measurement; a forged state refuses |
| **N7** | NS-7 ANCHOR + WWW3 SETTLEMENT | the mesh computes one consensus root of the chronicle; a thin adapter posts it to a local WWW3 testnet; wallet↔ipsal-os entity mapping (CHARTER §7, the three seals) | a real chronicle segment anchors as one root; the world keeps ticking with the testnet down; a forged root refuses on verification |

Sequencing: N1 before N2 (receipts before the actions that emit them); N3
before N4 (identity before query scope); N6 before N7 (anchoring needs the
converged mesh root it anchors); N7 last. Each landing names its spec section
and its game consumer in the landing record.
