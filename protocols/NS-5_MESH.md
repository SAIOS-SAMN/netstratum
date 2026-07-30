# NS-5 — MESH: two nodes, one world

**Status:** SPECCED (N6a, 2026-07-30) — written FROM the landed
machinery, the spec-from-working-code law (N1's precedent). The
implementation (N6b) is its own cadence. Supersedes the stub.

The mesh is how independent engine instances hold ONE universe: not
by trusting each other, and not by mirroring each other — by
MEASURING each other. Convergence is the proof; opposition is the
check; the fold is the merge that never merges.

## 1. The three laws

1. **Proof-of-convergence, not proof-of-authority.** A node's claim
   about a world IS its chronicle. Two nodes agree when replay of the
   same receipts reaches the same committed projection — the landed
   CONVERGE-1 machinery run across a wire instead of a directory. No
   node is root; the four C's place every claim.
2. **The anti-mirror check (12_UNION §2c).** Cross-node verification
   is by OPPOSITION, never byte-mirroring: a two-legged deed verifies
   when the far leg is the exact inverse of the near one (the
   aperture pair summing zero; the council reckoning summing to the
   locks; the Delta antisymmetry). Byte-canonical encoding exists
   ONLY so digests agree; the question a node asks is never "same
   bytes as mine" — it is "is your leg my inverse."
3. **One writer per clock.** A world's chronicle has ONE authoring
   node at a time (the two-writer guard, made protocol). Authority
   moves by RECEIPTED HANDOFF, never by race: the yielding node seals
   a handoff receipt naming the successor; the taking node's first
   append cites it. A fork (two chains sharing a prefix) is not a
   catastrophe — it is a DIVERGENCE: both branches live, the mirror's
   rotation law names them, and convergence annexes both (annex,
   never merge — 07 §1).

## 2. The wire

NS-1 TLV frames over the socket — the house wire, never a third
format (CBOR/BSON refused on record, 2026-07-30). Frame = one TLV
record; the tags are the NS-1 registry's own; a sync session is:

```
  hello        node id ‖ universe span ‖ epoch base (data frame)
  have         per world: terminal digest ‖ receipt count
  want         the worlds whose terminals differ
  receipts     the missing suffix, verbatim NS-3 receipts
  converge     each side runs CONVERGE-1 locally; annex/quarantine
  (no ack)     agreement is MEASURED (same terminal after annex),
               never declared
```

A received suffix verifies by the chain law before anything reads it
(broken → quarantine, named, kept — 07 §1.2). Unknown event tags
sync and verify without decoding (SEAL A carries across nodes).

## 3. What the mesh retires, and when

- The CLI two-writer guard relaxes into the handoff receipt (N6b).
- The SSE forerunner (webserve /stream) yields to TLV frames pushed
  on the same socket the sync speaks.
- The epistemological want (kind 1) becomes fully real: broken
  chains IN REACH now includes the far node's quarantines.

## 4. Test vectors

*(N6b — extracted from the first two-node session, the N1 discipline:
the spec's vectors must verify from the document alone.)*

## 5. Deferred, recorded

Transport security (this is a prototype: the mesh trusts its LAN);
node identity beyond a name (ties to NS-6 operator profiles); the
handoff under partition (the yielding node unreachable — ruled at
N6b with the vectors in hand).
