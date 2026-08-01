# 19 — THE REGISTRY: one authority per axis, and the expansion law

**Status:** MAP + LAW (2026-08-02). The map half is a survey of what
actually stands, verified against the tree rather than quoted from other
documents. The law half answers one question: **how does a vocabulary
grow when the nodes that use it are distributed and may not agree?**

Subordinate to `00_CHARTER.md`. The tag registry (`dop.rs` header) is
this document's older sibling and its precedent — one code, one meaning,
forever.

---

## §1 THE TOOL SURFACE — where C, D and V actually live

Verified by export scan, not by citation. **These names have moved
before and will again; this section is the map, never the authority.**
The authority is always the crate.

| letter | module | the callable surface |
|---|---|---|
| **C** | `rfa-core/src/cage.rs` | `consult`, `Proposal::{offer,withdraw}`, `Lens::{of,show}`, trait `Exact` |
| **V** | `rfa-core/src/boundary.rs` | `boundary_form`, `dual_pole`, `Flow`, `BoundaryForm`, `Vacuity` |
| **D** | `rfa-core/src/capacity.rs` | `capacity`, `circulation`, `divergence`, `egress`, `ext_x`, `ground`, `Degree`, `Extent`, `Egress` |
| the seam | `kernel/src/bridge.rs` | `project`, `project_complex`, `read_entangled`, `verdict`, `degree`, `rank_is_defined`, `observe`, `fold_certified`, `refold` |

**What the judge actually consults today:** `kernel::lawful` calls
`capacity_law` (4 sites), reads `Vacuity` (5), and reaches
`bridge::read_entangled` once. That is the whole of D and V inside the
kernel. **C is consulted from nowhere in the kernel and must never be** —
it has zero authority by construction (`17_CAPACITY` §10ter.c).

**Renames already survived, recorded so a reader does not chase a
ghost.** Each left name is DEAD and appears here only as history: the
boundary read was formerly `potential_form` (renamed when V took no
operator); the arbor was formerly the spine; the ledger was formerly a
name struck whole by Charter §11; and `rank_is_defined` was formerly
`capacity_is_defined`. Live code answers to the right-hand names only.

## §2 THE CLOSED AXES — one authority each

| axis | authority | values |
|---|---|---|
| station | `ledger/src/registry.rs::STATIONS` | progenitor · watcher · observer · sovereign |
| perception | `ledger/src/registry.rs::PERCEPTIONS` | planar · volumetric · harmonic · umbral |
| metabolism | `ledger/src/registry.rs::METABOLISMS` | mineral · biotic · plasmic |
| disposition | `ledger/src/registry.rs::DISPOSITIONS` | expansionist · empiricist · epicurean |
| class | `15_TAXON.md` §2 → `seeds/taxon-map.json` | codes 0–7 |
| clade | `15_TAXON.md` §3 → `seeds/taxon-map.json` | codes 0–20 |
| seat width | `registry.rs::seat_width` | progenitor 8 · watcher/observer 16 · sovereign unbounded |

## §3 THE SECOND-TRUTH REGISTER — axes re-typed away from their authority

An axis re-typed elsewhere is a second readable truth (§11 rule 2), and
it drifts silently because nothing forces the copies to agree.

- **TIER (T0–T3) HAS NO ENGINE AUTHORITY AT ALL.** The station→tier law
  is implemented in `tools/observer.py` (`TIER_OF_STATION`), again in
  `tools/observatory.html`, and described a third time in
  `BUILD_ORDER.md`. `registry.rs` knows `STATIONS` and knows nothing of
  tiers. **Three implementations, no authority.** This is the live
  example of the locality problem and the first thing §5 should fix.

## §4 ELDERS AND ANCIENTS — measured, and they are not stations

The finding, from the tree rather than from intent:

- **`elder` COMPUTES.** `ElderWarded` is a landed deed; WARDEN-1 gives
  the elder world its reverse shield and depth gate (`world.rs`,
  `world_warden_veil.rs`). It is real machinery.
- **`ancient` DOES NOT EXIST.** Two prose mentions, zero code, no deed,
  no vocabulary entry.
- **Neither is a station, and adding them to `STATIONS` would be a
  category error.** Station answers *what office do you hold* —
  progenitor, watcher, observer, sovereign are things a line DOES on the
  board. Elder and ancient answer *how long have you stood*. They are
  ORTHOGONAL: a progenitor may be ancient, an observer may be elder, and
  collapsing two axes into one destroys both readings.

**Therefore they are a TENURE axis, not station values** — and `elder`'s
existing depth-gate semantics already behave that way, which is evidence
the axis is real rather than invented. Seating `ancient` means opening
that axis, not extending `STATIONS`.

## §5 THE EXPANSION LAW — totality, never locality

**The problem, stated plainly.** `STATIONS` is `[&str; 4]` compiled into
one crate. A distributed node that needs a fifth value edits its own
array, and now two nodes hold different vocabularies while both believe
they hold *the* vocabulary. Nothing detects the divergence; a receipt
written under one reads as valid-but-different under the other. **A
locally-extended closed set is the worst of both: it looks authoritative
and is not.**

**The law, in three tiers.** The system already runs this pattern for
free content over closed axes (`SYSTEM_SPEC` §3: *species is free text
over the closed axes*). Made general:

```text
AXIS      CONSTITUTIONAL. An axis is a dimension of identity the kernel
          may judge on. Opening one is a charter amendment and a VERSION
          bump — never a local edit. The set of axes IS the totality.

VALUE     CLAIMED, like a wire tag. One code, one meaning, forever;
          append-only; the claim lands in the authority named in §2
          BEFORE any code uses it. A value may never be removed or
          repurposed, only retired.

CONTENT   FREE. Names, mottos, species prose, world names. No claim, no
          coordination, no ceiling. This is where a node expresses
          itself, and where divergence costs nothing.
```

**Why this answers the distributed case.** Two nodes designing
independently can disagree about CONTENT without consequence, must
coordinate on VALUES exactly as they already coordinate on tags, and
cannot unilaterally invent an AXIS at all. **The blast radius of a
disagreement is bounded by which tier it lives in**, and the tier is
visible from where the thing is written down.

**The standing test for any proposed addition:**

1. Is it an OFFICE, an AGE, a SUBSTRATE, a PERCEPTION — or is it a name?
   A name is content; stop here.
2. If it is a reading a RULE consults, which existing axis does it sit
   on? If none, it needs a new axis, which is constitutional.
3. Does the value already exist under another name on that axis? Two
   names for one reading is the drift `EQUATIONS` §0 exists to prevent.

**Owed, and named rather than assumed:** the tier axis has no authority
(§3); the tenure axis is unopened (§4); and no lint yet checks that a
value claimed in a doc exists in its crate, the way `constitution_lint`
checks that a citation resolves. That lint is the mechanical form of
this whole document and is the obvious next build.
