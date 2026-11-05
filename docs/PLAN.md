# Plan

The one order of work. It replaces the plan's earlier steps (v0.1.0 and
v0.2.0 below, mapped into this order), docs/102 §4's order, and docs/118
Part IV, which this plan carries forward. Every remediation is a row of the
register, [docs/119](119_REMEDIATION_REGISTER.md); every step names its rows.

## Principles

- **The coordinate is the state** ([docs/120](120_THE_COORDINATE.md)): a
  body stands at its TUniversal point inside the universe and is addressed
  by it. Laws read ranges on one lane; nothing iterates a catalogue of
  bodies or all pairs; motion is the closed-form advance of the lanes.
- **One law and its inverse.** Expansion and contraction, attraction and
  repulsion, outflow and capture are each one law applied with its sign,
  never two pieces of code.
- **First degree, exact, no scalar collapse** (docs/118 F1): no square, no
  quantity times itself, no number standing for a structured object, no
  float, no import.
- **Derivation before code** for every change of law: its derivation in a
  design document and its row in [EQUATIONS.md](EQUATIONS.md), then the code
  with its witness.
- **Verified by re-derivation** (operator 2026-10-20): a change is proved by
  its witnesses and by identities read on a fresh founding, never by byte
  identity; cost is counted (N against 4N), never timed.
- **Aligned with the other nodes.** What crosses to the court or is shared
  with Xylarium is agreed with Mycelium before it is built
  (LAW-PASSAGE v2, LAW-COORDINATE v1).

## The order

### 1. The record (done; kept current)

- [docs/117](117_THE_AXES_AND_THE_TOWER.md) the axes and the tower;
  [docs/118](118_FOUNDATION_AUDIT_TRAJECTORY.md) the foundation and the
  trajectory; [docs/119](119_REMEDIATION_REGISTER.md) the register (nine
  audits); [docs/120](120_THE_COORDINATE.md) the coordinate;
  [docs/121](121_MAGNETIC_COUPLING.md) the magnetic coupling.
- docs/118 is corrected by the register's last section; docs/102's register
  is re-cut from docs/119 (G11).

### 2. Wrong behaviour (register §B and the S1 rows)

Fixed, PR #58: B6, B10, B12, A11, A13, J2, J3, J5. Next, each a change of
how the universe moves, so each read before and after:

| rows | what | waits on |
|---|---|---|
| B3 | a star system keeps its motes through its settle | — |
| B4, B5 | one outflow book; an identity on every passing, arrival and exit | — |
| B7, B16 | one face law, read from a plane's axis (docs/117 §8.2) | — |
| B8, B9 | heat and light from bonds and friction; one starlight law | docs/122, the operator's read |
| B11, B13, B14, B15 | `toward_rest`'s reach; `Body.load`; the legacy read; a witness from both sides | — |
| B1, B2 | galaxies form as branches; the ocean lands by a range on S | the coordinate (step 3) |

### 3. The coordinate as the state (docs/120)

| step | rows |
|---|---|
| The module, `conductor::coordinate` (PR #57), replaced by Mycelium's `forms::coordinate` when LAW-COORDINATE v1 lands | J1 |
| The state type: each body's point; the existing fields become readings of it | A1, C17 |
| The magnetic coupling on it (docs/121), replacing `repel`, the gas attractor, `shear`'s and the field's one-way exchange and the band's squared rest | C18, C3, A8, A25 |
| The collective's bonds and the captures as ranges on one lane | A28, A29, A12, A32 |
| The phases as H, the face as T, the passage (docs/114 §9, docs/117 §9–§10) | C1, C2, C7, A2–A10 |
| Expansion and contraction as one advance and one re-origin | A17–A27 |
| The schedule on events solved from the lanes; the stored period retired | C14, C15, J4 |

### 4. The physics against the foundation, and one law per quantity

The register's §C and §D. The old consolidation issues (#28 one deficit and
face map, #29 one clock and coherence, #30 one push, heat and release, #31
one burn, rim, winding and motion) are done here, on the coordinate, never
on the collapsed state (G9).

### 5. The eras

Era 0 is demolished and the bridge retires (register §F; docs/89 B4, B6,
B8): positions become origins, distance the route through the throats, the
year the galaxy's terms. The imported rulers go (#13, #32; C8). The live
beat runs era 1 alone.

### 6. Standards and naming

The register's §H and §J: files of one organ and their sizes, names by
function, one meaning per letter (Z, R, p, Q), the MU-STANDARDS rules, the
headers, the tests' names.

### 7. Mycelium and Xylarium

LAW-PASSAGE v2 (Concordance draft clause 2.11) and LAW-COORDINATE v1
(proposal 78.173): their fixtures read as tests here; the court's form for
crossings at the vertex (docs/113). #35 (`tongue`), #36 (`TPoint::parse`),
#11 (attachment, the operator's go-ahead).

## Decisions open

Only what no document decides; each is stated where it arises.

- Heat and light from bonds and friction, out by the face (B8, B9): derived in [docs/122](122_HEAT_AND_LIGHT.md), awaiting the operator's read; it dissolves hydrogen's hold as a question.
- A body's magnetic sense: its turning's sign or the electrons' phase
  (docs/121 §8).
- The signed lanes inside a universe against the court's non-negative ones
  at the vertex (docs/120 §9, with Mycelium).
- Where heat lives (docs/118 Part I item 5, docs/120 §9).
- Whether compounding `(1 − s)ⁿ` stands under first degree (C11).
