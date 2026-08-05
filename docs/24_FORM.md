# 24 — THE FORM: the relational tensor, its boundary, and the sign law

**Status:** LAW, split out of the retired `18_TORSION.md` on 2026-08-03 by operator
ruling. It carries §2 THE FORM whole — the Δ/Ω mathematics the engine
computes on.

**Why it moved.** `18_TORSION` was the physics subset that constrained the
computational mathematics, and §2 was 704 of its 1,572 lines — the
single largest block, and a different KIND of thing from the four
articles it sat beside. The articles rule what a law may be; the form
states what the objects ARE. Two jobs, two documents.

**This is the load-bearing half.** Code cites §2a, §2b, §2d, §2e and §2f
from `rfa-core/src/capacity.rs`, `rfa-core/src/boundary.rs`,
`rfa-core/src/reduce.rs` and `kernel/src/bridge.rs`. Section numbering
is UNCHANGED so every one of those citations still resolves — a split
that renumbered would have broken twenty-four provenance references to
save nothing.

---

## §2 THE FORM

### 2a. D(Ω) — the progenitor's reading of the totality

**RENOTATED 2026-08-02** (operator ruling: *`D(Ω) +: V(Ω)`*). This
section read `Ω(D)` — *the space indexed by dimensional degrees* — which
made the TOTALITY a function of ONE OFFICE, so `Ω` was at once the thing
measured and a thing `D` produces. `D(Ω)` reads the other way: `D` OF
the totality, a READING, and a reading never outranks its subject
(§2c′). `17_CAPACITY` §3 carries the full ruling and the second office
reading `V(Ω)` that comes with it.

`Ω` is THE TOTALITY — one universe, whole, read by both offices and
owned by neither. `D(Ω)` is the ROOM standing in it: not a scalar and
not a metric density. `D` is the RANK of the relational structure — how
many independent degrees stand — and `D(Ω)` is the space they span.

**`D` IS NOT A CAPACITY** (operator ruling 2026-08-01: *why do we have
capacity? that should not be the case in an expanding universe*). The
word names a container and implies a ceiling; there is no ceiling
anywhere in this workspace, and `holonomies().len()` rises without
bound as structure emerges. `PhiK I^(3)` had the right name all along —
*rank: dim of the relational structure*. Genuine capacities do exist
(seat widths, occupancy caps) and are DERIVED from this rank; the
reading that sets a bound was named after the bound it sets.

    D  =  rank( Δ(V) )  =  the independent cycles carrying charge
                           no frame explains

**Grounding.** The landed `rfa_core::Delta` is ALREADY an alternating
form, not a metric tensor: `Δ_ij = −Δ_ji` with zero diagonal, enforced
by its only lawful writer `set_antisym`. There is no metric `g`
anywhere in the workspace. `orbit_lock` is ALREADY a torsion test — it
searches for a torsion cycle and vetoes the fold when one binds. The
constitution names what the algebra already is.

### 2b. ∂Δ UNCAGED — no norm, no ceiling

**V TAKES NO OPERATOR** (operator ruling 2026-08-01 — *the whole or the
nothing*). This reading was written `∇V`, then `dV`, then `∂V`; all
three forms are STRUCK, and they were wrong in the same way: each encloses `V` in an
operator that returns a PART. `D0` says `V` is the committed state read
WHOLE — indivisible, unpartitioned, never truncated to a subspace — and
a gradient is a part, a boundary is a part, a divergence at a vertex is
a part. Three symbols, one error: the enclosure was the mistake, never
the choice of bracket.

So `V` stands bare. **It is read whole or it is not read**, and nothing
in this constitution wraps it. What the readings actually take is `Δ` —
the relation `V` expresses — and they now say so: `∂Δ` is the boundary
of the relation, `∮` its holonomy charge. Neither is a reading "of V"
and neither may be written as one.

`∂Δ` is a directional operator and stays one. It is never collapsed to
a scalar magnitude, never bounded above, never reduced to `‖∂Δ‖`.

This EXTENDS rather than reverses THE PROPORTIONAL LAW (2026-07-31).
That ruling barred squares because squares are the mathematics of
minimization — collapsing difference into a scalar to be descended.
The L1 form was a smaller cage of the same kind. **Any collapse of a
directional quantity to a scalar aggregate is now barred**; L1 survives
ONLY where a reading is reported to an OPERATOR lens, never where the
system computes.

### 2c. THE UNION — one Δ, two complementary projections

**`U* = D(V) ⊕ V(D)` IS STRUCK** (operator ruling 2026-08-01), on the
same test that struck §2d and §2e and on one ground they did not carry.

**The Article III half.** Nothing computes `U*`, `D(V)` or `V(D)`. No
file reads them, no rule branches on them, and removing them changes no
behaviour. That is the test this document applies to itself.

**The half that is worse than uncomputed: `⊕` was the WRONG JOIN.** A
direct sum is a FREE join — the summands are independent and either may
move alone. What the algebra actually holds is a CONSTRAINED join. `D`
and the flow are not two objects that were declared inseparable; they
are two complementary projections of one fixed `Δ`:

    Δ  =  gradient  ⊞  circulation        (⊞ constrained, never free)
    D  =  rank(Δ)              ∂Δ  =  boundary(Δ)

Landed and recorded in `rfa_core::boundary`: *the two poles are
COMPLEMENTARY PROJECTIONS OF ONE FIXED `Δ` — they are not independent
witnesses, knowing `Δ` and one pole yields the other.*

**This ANSWERS the article's own worry rather than restating it.** §2c
warned that *"a guard on half of a decomposable object is not a
guard."* Correct — and the cure is not to declare the object
indecomposable, it is to stop writing it as a decomposition. Two
projections of one tensor have no halves to guard. What was asserted as
law is now a fact of the arithmetic, and §2f's entanglement stops being
a rule someone must enforce: neither pole can move without the other
moving oppositely, because their sum is fixed.

**THE BINDING IS A TENSION, AND IT HAS A CONSEQUENCE.** At fixed `Δ`
the two poles are strictly zero-sum: content moved into circulation is
content taken out of gradient. A state can therefore be made to read
`Entangled` by REDISTRIBUTION alone — no new relation, no work. The
constraint relaxes only when `Δ`'s support grows, and that is the sole
operation under which both poles rise together. **That is why only
support expansion mints** — not a separate rule, this constraint read
forward.

`V` is not a summand here and never appears in a reading (§2b). It is
the committed state, whole, upstream of `Δ`; `D` and `∂Δ` are readings
of `Δ`, not of `V`.

### 2c′. THE UNION PROPER — D PROGENITOR, V SOVEREIGN

**Ruled 2026-08-01.** The section above settles what `D` and `∂Δ` are to
each other — two projections of one `Δ`. It says nothing about what `D`
and `V` are to each other, and that relation is the union the name was
always reaching for. It is not an arithmetic. It is two OFFICES.

    V   SOVEREIGN    holds. Its standing comes from ADMISSION — it
                     passed `lawful()`, it carries receipts, it is in
                     the chain. No reading confers it; none revokes it.

    D   PROGENITOR   makes room. D3's emission opens a plane, the fiber
                     grows, and band capacity is a reading of that fiber
                     (`12_UNION` §1). Nothing else creates space.

**The roles are `12_UNION`'s own, not a metaphor fitted after the
fact.** That chapter's progenitor power table lists *"create new spaces
— extending the fiber; minted planes open room, and band capacity reads
the fiber"*, which is D3 exactly; and its Tier 0 reading is *"they
contract and expand the board, they create it. Everything else plays on
it."* The sovereign is the holder of what is chartered, governing what
it founded or was lawfully delegated. `D` is the board-maker; `V` is the
holder.

**NEITHER OUTRANKS THE OTHER**, and `12_UNION` §3 states the standard in
the document the word comes from: *classes are stations of FUNCTION, not
steps of rank — a Watcher does not outrank a Sovereign; they hold
different offices in the same union.*

Three consequences, each structural:

1. **`D` cannot revoke `V`'s standing.** The progenitor makes room; it
   does not un-charter a sovereign. Article II therefore gates
   ALLOCATION and never standing — see the amendment there.
2. **`V` cannot open room by fiat.** `PlaneUnearned` already refuses it.
   The sovereign holds; it does not create board.
3. **Growth requires both.** `D` opens nothing without `V`'s receipted
   work, and `V` expresses nothing without `D`'s room. That is §2f's
   entanglement with names that say WHY — it was true as algebra and
   unmotivated as law; under the union it is simply what two offices in
   one body means.

**WHY THE EARLIER FORMS FAILED, recorded together because they were one
error.** Each tried to make the two offices COMMENSURABLE:

- `U* = D(V) ⊕ V(D)` SUMMED them — `⊕` is a free join, so either could
  move alone (struck above).
- Article II's original standing clause RANKED them — a reading
  conferring standing on its own source, which runs backwards through
  the judge (amended below).
- The traverse formalism (§2e, struck) DECOMPOSED them into endpoints.

**Offices do not sum, do not rank, and do not decompose.** That is what
made every arithmetic reading of this union wrong, and it is why the
correct statement carries no operator at all. A tuple `(D, V)` fails the
same test: a product type has addressable halves, which is the
decomposition again in different syntax.

**THE PROGENITOR'S OFFICE IS THE SPAN `α → Ω`** (§2e). `α` is the origin
pole, `Ω` the totality. Where the traverse is a point — `α ≅ Ω` — `D`
sits at GROUND and the room is one degree, itself; `D` rises exactly as
`Ω` separates from `α`. That is why `D = rank(Δ(V))` and *how far the
universe has come from its origin* are one reading, and why the floor
and the span are the same office.

**TWO ARROWS, AND CONFLATING THEM MAKES A FALSE HIERARCHY** (corrected
2026-08-02, operator: *how can a sovereign own a progenitor?*). An
earlier draft of this section wrote "`V` primitive, `D` read downstream"
and treated that as the standing order. It is not. It is a MEASUREMENT
path, and reading it as ownership destroys the union — a thing wholly
derived from another is that other's property, not its equal office, and
it would make the board-maker a byproduct of what plays on the board.

    ORIGIN      the receipted deed  →  the office
                V's standing comes from ADMISSION (it passed `lawful()`)
                D's room comes from RECEIPTED MINTS — D3: *D rises BY
                RECEIPT*; `GaugeShiftedAt` opens the plane and
                `PlaneUnearned` refuses an unearned one

    MEASUREMENT V  →  Δ(V)  →  rank
                the reading that REPORTS D, taken through V

**Neither office derives from the other.** Both are carried ON the
receipt — it is the medium their bond settles through, not a predecessor they
descend from. They are of different KINDS with separate origins: one is
room, one is substance. That is why neither ranks the other, and the
non-ranking comes from a landed mechanic (the symmetric bond,
`09_IDEOLOGY` §2.2) rather than from an provenance anyone had to invent.

**THE ORDER, settled** (`12_UNION` §3c carries it whole). Peers with
nothing above them would leave two powers facing each other; there IS an
order, and neither office is at its top:

    THE LAW      lawful(prior, next, step) — judges every step,
                 no office exempt
      ↓ admits
    THE RECEIPT  the chain — the sole grant, from which BOTH draw
      ↓ grants
    V SOVEREIGN  holds the chartered      D PROGENITOR  makes the room

They are peers as a BONDED PAIR, never as counterparts — different kinds,
separate origins, and a relation that is COMPLEMENTARY rather than
LIKE-FOR-LIKE. `09_IDEOLOGY` §2.2 already carries the form and the ruling:
*a symmetric pair of bonds is the MARRIAGE FORM; a single bond is
PATRONAGE. BOTH ARE PRIVILEGED in the pair* — `bond_flow` is the
SOURCE's syntropic book, so each office sources and each carries its
own. Two sources, two books; the privilege is held twice and the carry
is positive on both sides. A one-directional bond would be
patronage — one office owning the other — which is exactly what a
`V → D` derivation makes. The receipt is not their predecessor; it is the
MEDIUM the bond is carried on, and `severance_load` prices cutting it.
Counterparts do not suffocate without each other; a bonded pair does, and
the interlock below is that.

**AND NEITHER OFFICE ACTS.** A deed acts. `D` does not decide to open a
plane — a fold refuses, the residue emits, the plane opens BY RECEIPT.
`V` does not decide to hold — admission commits. Two things that do not
act cannot compete, which is why no equilibrium needs negotiating
between them. The bounds are structural on both sides: `PlaneUnearned`
makes unearned room unmakeable and Article II pays unoccupied room
nothing; the container law makes structure in an unopened plane
unrepresentable, since the gauge key is a container grown only by
receipt.

**The measurement arrow carries no authority**, by this constitution's
own rule stated one section up — *testimony never outranks its subject*.
That rule was written to stop `D` conferring standing on `V`; it holds
identically in reverse. `rank(Δ(V))` is testimony ABOUT `D`, taken
through `V`; it does not make `V` the author of `D`, any more than
measuring a room makes the furniture its architect.

**What the acyclicity guard actually guards.** What is barred is putting
`D` inside the DEFINITION of what `Δ` reads — a tuple, a sum, a
dyad-as-term — because `D = rank(Δ(V))` would then be defined through
something containing `D`. That is a COMPUTATIONAL bar, on definitions.
It was never a claim that `V` ranks above `D`.

**AMENDED 2026-08-02 — `Ω ≢ V`.** This paragraph read *"`Ω ≡ V` stands
untouched"* and cited `17_CAPACITY` §3, which then carried that
identity. §3 is corrected: `Ω` is the TOTALITY and `D(Ω)` is the ROOM (the space `D`
spans, and the `α → Ω` span of the progenitor's office) and `V` is the
SUBSTANCE. The bar above is unchanged and in fact strengthened — under
the old identity the guard was porous, because `D = rank(Δ(Ω))` with
`Ω ≡ V` and `Ω(D)` indexed by `D` produced the forbidden circularity by
SUBSTITUTION, without any document writing it down. Separating the
source from the space closes it.

**NOTATION, recorded once.** `Υ` is `V`'s own letter — upsilon is the
Greek forerunner of Latin `V` — so the two are one name in two alphabets,
never two names.

### 2d. THE SIGN LAW — divergence, egress and ingress

**The cosmology is struck; the sign law is LIVE and is stated here.**

The struck half: this section once wrapped two landed organs —
`DiscoveryMinted` (tag 86) and `mesh::spine::seal` — in invented
cosmological notation, which made the engine sound larger without making
it do more. Struck under Article III and PURGED. The symbols are not
reproduced here: a struck symbol kept alive in a record is still a
symbol in the corpus.

**THE STRIKE OVER-REACHED BY ONE CLAUSE, corrected 2026-08-01.** It
recorded that *"no file read them."* Six sites in the sealed math read
this section as their governing authority for the sign law
(`capacity.rs` and `reduce.rs`). The cosmology drove nothing; **the sign
law drove, and still drives, the divergence reading.** Striking the
whole section left live code citing a dead authority. The law is
therefore re-homed here, in plain terms.

**THE FORM.** On the relation `Δ` over vertices `V`, the discrete nodal
divergence is the boundary operator applied at a vertex:

    (div Δ)_i,l  =  Σ_j Δ_ij,l                     `capacity::divergence`

    sign( (div Δ)_i,l )  =  +1   EGRESS   — the vertex sources
                            −1   INGRESS  — the vertex sinks
                             0   BALANCED — holonomic; whatever the
                                            vertex carries, it circulates

**GLOBAL CONSERVATION — the books close.** Antisymmetry (`Δ_ji = −Δ_ij`)
gives, on every fiber layer:

    Σ_{i ∈ V} (div Δ)_i,l  =  0

so an egress anywhere implies an ingress somewhere. **No relation is
all-source.** This is Noether's shape (LAW 2) on the pair symmetry, and
it is why the non-trapping reading is a statement about a BOUNDARY, not
about an isolated vertex.

**WHAT SURVIVES BECAUSE IT COMPUTES.** A loop carrying non-zero charge
is real and is measured: `rfa_core::holonomies` returns exactly those
charges, and `∮ ≠ 0` is the ordinary statement that a walk around a
cycle does not return where it began. It needs no cosmology.

### 2e. THE Ω/α UNION — wrapped on λ-nodes, toward D(cohesion)

**The two poles, and only what computes.** `α` is the ORIGIN pole;
`Ω` is the TOTALITY pole. Both are landed: `α` is `GenesisSealed`
(tag 61) on the all-zero predecessor `mesh::arbor` writes at the genesis
record; `Ω` is the TOTALITY OF THE ROOM — the far pole of the
progenitor's span, not a second name for the committed state
(`17_CAPACITY` §3, corrected 2026-08-02: `Ω ≢ V`). What `Δ` projects is
`V`; what `α → Ω` measures is how far the room has opened.

**The union is the walk, not the endpoints.** `Ω ⊕ α` keeps every state
between the poles. This is SEAL B restated: receipts are the world, so
the path is the truth and a snapshot of the two ends is not.

**λ-nodes.** Each is a grade of the gauge key, an exact rational wrapped
`[0,1)`. The wrap is load-bearing rather than decorative: without it
successive origins COLLIDE, because a key of 10 IS 0 — which is the
recurrence collision `11_AEON` §2.0 exists to prevent, and why each aeon
seats at its own sheet base. Landed: `GaugeShiftedAt` (tag 60) opens or
moves grade `g`; `key_distance` reads wrapped L1 at any dimensionality.

**The fiber reading, deferred twice and now built.** `12_UNION` §3
defers seat widths and `11_AEON` §7 defers occupancy, both explicitly to
"capacity as a live fiber reading (minted-plane count)." That reading
exists as of 2026-08-01. Two readings are available and they answer
different questions — take care which:

    read_entangled(&state).rank       what CIRCULATES above the floor.
                                      Zero is a lawful answer here.
    read_entangled(&state).degree()   `D`, the ELEVATION: the point the
                                      state occupies plus that
                                      circulation. NEVER zero.

Seat widths and occupancy are bounds DERIVED from structure, so they
read `.rank`. Anything asking *how high does this state stand* — Article
II above all — reads `.degree()`. Both have waited since 2026-07-29 and
neither is wired yet.

**`α ≅ Ω` IS THE GROUND, and that is what the trim left standing.** When
origin and totality are congruent the traverse has no extent — but a
traverse with no extent is a POINT, and a point is ONE dimension, never
none. That is exactly `Degree::GROUND`: the state has not vanished into
a vacuum, it stands on the surface holding itself. The congruence names
the floor of the elevation, not a hole beneath it.

**TRIMMED 2026-08-01.** This section previously carried a traverse
formalism, a per-node summation of `(D(V) ⊕ V(D))`, and a congruence
test `α ≅ Ω` offered as a second detector for Article III. All of it
drove no behaviour and none of it named a congruence relation that
exists, so under Article III applied to this document it is struck. The
counting test in §3 stands on its own and is computable today; the
GROUND reading above is what survives of the `α ≅ Ω` idea, and it
survives because it computes.

### 2f. THE ENTANGLEMENT — Δ(D) ⋈ ∂Δ

**Ruled 2026-08-01: capacity change and potential gradient are
ENTANGLED, not coupled.** The distinction is the whole force of the
law. Coupling would be two objects that interact — `Δ(D) = g(∂Δ)`, a
function, a bound, a product — and two objects can always be attacked
one at a time. Entanglement is ONE object:

    Δ(D) ⋈ ∂Δ     neither term is separately defined, neither is
                  separately guardable, no operation touches one alone

**Why this is the stronger force.** It moves the no-free-energy law
from a RECONCILIATION INSTRUMENT to an INVARIANT — from *detect and tax* to
*cannot be constructed*. That is this system's own strongest principle,
already stated in the kernel's own header: *"an unlawful transition is
not caught after the fact, it cannot be constructed … you do not
reconcile what cannot diverge."* Under entanglement a closed loop at
uniform potential does not mint and get penalised; **it cannot mint.**

Three consequences, each structural:

1. **No capacity from repetition.** A traverse emitting deeds without
   moving `∂Δ` produces no `Δ(D)` — not "is taxed for it." There is no
   path by which activity alone becomes capacity.
2. **No hidden work.** Potential motion that registers no capacity is
   equally unconstructible. Work cannot be done off the books and
   claimed later.
3. **No half to guard.** §2c's recorded failure mode — the struck
   formulation guarded the damped-gradient term while the objective leaked through
   C-scoring — cannot recur, because there is no half. A guard on half
   of a decomposable object is not a guard; an entangled object has no
   halves.

**Grounding — the pattern is landed three times already.** This is not
a new mechanism but the foundation-level instance of one the engine
already runs:

- **THE ESCROW LAW** — a covenant lock moves value between banked and
  escrow, so a bank debit that escrows nothing refuses
  `ConservationBroken`. Two ledgers entangled; neither moves alone.
- **THE GAUGE LAW** — declaration == effect on every grade; a key
  smuggled under a neutral deed refuses `GaugeBroken`.
- **I4** — `mint ≤ f(verify_work)`. The mint is already entangled with
  verified work; no certificate, no mint.

**The Sybil consequence.** A puppet cannot forge capacity, because
forging capacity would require forging potential motion, and potential
motion is verified work. Sybil resistance therefore follows
STRUCTURALLY from entanglement plus PoUW, rather than from detection
after the fact — which is the standing open problem (`KERNEL_RELAY`
§7.1) answered at the foundation rather than patched at the mesh.

**Build consequence, recorded.** The capacity reading and the potential
reading must be computed TOGETHER, in one pass, from one prior state.
A caller that measures `D` and then separately consults `∂Δ` has
already broken the entanglement and reintroduced the two-object attack
surface. This binds §6's first build.

### 2g. WRAP(ENTANGLEMENT) — accounting, not identity

**`==` IS STRUCK, and it was the collapse error again** (operator
correction 2026-08-01). This section first proposed converting the
bounds to identities — `amount == mint_bound(d)` — to remove latitude.
That is the same failure as gradient descent, one level down: it
collapses a two-sided relation to a point and discards whatever
distinguished the sides. An equality keeps one value and throws the
difference away. **We never discard residuals**, and the gap between
what a deed DECLARES and what the state DERIVES is a residual like any
other. **The law admits no `==` at an entangled seam.**

**BOTH SIDES ARE HELD.**

    declared   — what the deed says the capacity moved
    derived    — what prior state and the potential motion yield
    the pair   — held WHOLE; their relation is READ, never zeroed

**The gap is dispositioned under I6, never by pass/fail.** A difference
between the sides gets the same three-way disposition every other
residual gets — *reduced as gauge, minted as a new dimension, or
refused* — and never silent admission:

    gap reduces to frame       →  the sides differed by scope; admit,
                                  the reduction receipted
    gap is irreducible         →  it EMITS: a new degree, D rises
                                  (this is D3 — the mint IS this case)
    gap cannot be dispositioned →  REFUSE, carrying both sides whole

**This is how the surface actually closes: by ACCOUNTING, not by
identity.** Nothing passes unaccounted. That is strictly stronger than
`==`, which refuses everything non-identical and learns nothing from
the refusal — an equality cannot tell a forgery from a discovery,
because it discards exactly the quantity that distinguishes them.

**WHY THIS BRIDGES NON-LINEAR PHASES.** A point constraint cannot cross
a phase boundary; it can only state the value on one side of it. The
`f` ladder is the in-system case and it is genuinely non-linear:
`f(0..2) = 0` — saturated, proof-of-position, free issuance barred —
then `f(3) = 1` at the marginal threshold, `f(4) = 4`, `f(5) = 7`. The
interesting behaviour is the CROSSING at rung 3, where a line stops
proving position and starts earning. At that boundary the declared and
derived descriptions genuinely differ, and **their relation IS the
transition.** Hold both sides and the crossing is legible; collapse
them with an `==` and the phase structure disappears, leaving only
disconnected values on either side of a wall.

**(2) DERIVATION PRECEDES ADMISSION** — unchanged and still required.
The kernel computes its own side from `prior` and `step` BEFORE reading
what the deed claims. The deed never influences what the derived value
is; it contributes the other side of a pair, and the pair is then read.

**(3) THE WRAP IS THE ENCLOSURE** — unchanged. On an unbounded axis a
constraint is escapable by pushing outward; there is always more axis.
On a finitely wrapped torus there is no outward: `key_distance` is
wrapped L1, a key of 10 is 0, the T∞ purge bars escape to infinity. The
wrap is what makes the accounting exhaustive — a gap has nowhere to go
except into the disposition.

**THE CONSEQUENCE — `lawful()` is an ACCOUNTANT, not a filter and not a
point-function.** For a given `(prior, step)` it derives its own side,
reads the deed's side, and dispositions the difference. The admissible
space is neither a range (latitude, attackable) nor a point (collapse,
lossy): it is *every state whose gap is accounted for*. A forgery fails
because its gap disposes to nothing; a discovery passes because its gap
EMITS — and the same machinery reads both.

**CONCRETE BITE IN LANDED CODE — and now for the right reason.**
`engine/kernel/tests/discovery_pins.rs:266` holds that *"the degenerate
mint is lawful: 0 ≤ bound 0"*, with the same claim in the header at
line 10. Under §2f consequence 2 that stays wrong — valid work means
the potential moved, so a zero declared capacity is hidden work. But it
refuses as an UNDISPOSABLE GAP carrying both sides whole, not as a
failed equality. The distinction matters at the witness: the refusal
must name what the potential moved and what the deed declared.

**HONEST COST.** Still a meaning change to an existing rule — today the
gap below the bound passes silently, and under this law it must be
dispositioned — so a major `VERSION` bump under the amendment tiers,
with old history verifying under the old rule.

**NOT YET RULED — which pairs are entangled.** I4's mint/work pair is
the clear case. Whether I5's decay bound and the assay bound are
entangled, and so subject to the same disposition, is an operator
ruling rather than a derivation: under-paying an owed decay may be a
lie of the same kind, or lawful forbearance. **Do not convert them
silently.**

### 2i. SYNTROPIC OR ENTROPIC — total classification, no neutral

**Every element is outright syntropic or outright entropic.** There is
no neutral middle, and the reason is the same one that closed §2g's
silent gap: **a neutral element is a place where value sits
unaccounted.** Neutrality is the unaccounted gap one level up.

**Landed three times already, and never made total:**

- thermal — encasement MINTS joules (syntropic), overflow VENTS
  (entropic); `05 §1.2`, encase-first/vent-overflow.
- the entropy dual ledger — VENT-1/SINK-1, banked against vented.
- the eight cohesion books — four syntropic/entropic PAIRS, never
  single counters: given/drawn, returned/extracted,
  bond_flow/severance_load, fulfilled/breached (`09`).

**What is owed:** the classification is not TOTAL. Elements exist that
sit on neither book, and each is an unaccounted place. The enforcement
is MESH-1d's own shape, already proven on the committed fields: every
element classified or caught by an **UNMEASURED tripwire**. Nothing
neutral, nothing silent.

### 2j. THE FIELD, NOT THE COLLISION — value from separation

**The hammer and the anvil** is contact: dissipation, deformation, one
thing reshaping or destroying the other, irreversibly. That is the
fold-that-merges — and it is precisely what `orbit_lock` FORBIDS.

**The magnets** are a field: the force acts at a DISTANCE, varies with
SEPARATION, destroys nothing, and does work on approach AND on
recession. The energy is in the configuration, never in an impact.

**Value is created by motion in the field between two things, as a
function of their separation — never by their collision.**

**LANDED THREE TIMES, verified by behaviour:**

- `cfe/src/world/economy.rs` — `friction = friction_rate() * divergence *
  value`. Friction prices DIVERGENCE; aligned partners at any alignment
  pay exactly zero. A separation force, already charging.
- `cfe/src/world/mod.rs` — `pair_melt(a, b, distance)` computes
  `gap = key_distance(a.key(), b.key())` then
  `melt = (distance + gap) * MELT_PER_UNIT`. Cost as a function of
  spatial separation PLUS key separation, and it is THE one producer of
  the formula.
- `cfe/src/world/mod.rs` — `key_distance`, wrapped separation across
  keys of any dimensionality.

**THE BOUNCE IS THE MINT.** `16_UNIT` §4a already carries the image as
law — *a torsion singularity where two photons bounce together,
regardless of the angle they come from* — and the mechanism is landed:
`orbit_lock ≠ ∅` → the fold REFUSES → the residue EMITS as a new plane.
The two states approach, do not merge, and **the refusal is what
creates the plane.**

**Why the merge must be forbidden, stated in field terms.** Merging
destroys the field that was generating the value. Two states collapsed
into one have no separation left, therefore no force, therefore nothing
further to create. The fold gate does not protect the states — it
protects the FIELD BETWEEN them. That is I6 read as physics rather than
as bookkeeping.

**BOTH LEGS FROM ONE SEPARATION — the template.** BRIDGE-1 (`11 §3.1`)
already writes the shape §2i needs, though it is **SPECCED, NOT
LANDED** (BUILD_ORDER carries it open): the confluence crossing prices
its key gap — venting proportional to the gap is the entropic leg,
while the resonant remainder encases as the syntropic leg. One
separation, both books, nothing neutral. That is the magnet with both
poles, and it is the pattern §2i's total classification should follow.

**FLAG — a live notation collision.** MARKET-2 writes its melt as
`φ·D` where `D = spatial L1 + wrapped key gap` (BUILD_ORDER §3b), while
`D` is now dimensional capacity (§1 D1). Two different `D`s in live
law. The CODE is safe — `pair_melt` names its parameter `distance` —
but the documents are not. The melt's `D` should be renamed (`σ`, for
separation) before the two drift into one another.

### 2m.2 LAYER CORRECTION — the covenant was promoted above its station

§2m wrote: *"An outstanding covenant IS a forward link"*, and listed it
as the landed organ for a KERNEL-level primitive. **That is a layer
inversion and it is withdrawn.**

    KERNEL layer   "the next commitment must satisfy P" — a constraint
                   carried in the record. Encoding-blind, game-blind.
    GAME layer     "orb A owes orb B this much of this resource by this
                   tick, with this much collateral posted." One
                   INSTANCE of such a constraint.

Defining the forward link AS a covenant would make the kernel inherit
the game's resource enumeration, its collateral rule and its window
semantics — the game supplying the kernel's vocabulary instead of the
kernel supplying the game's. `19_KERNEL` §2 measured the kernel's
dependency on `cfe::world` at exactly three items; this would have
quietly added a fourth and the worst-shaped one.

**Corrected:** the forward link is a kernel primitive. The covenant is
one game-level instance of it — evidence that the shape is
implementable, and *not* the definition. The covenant sits INSIDE the
sector; what crosses the door is the constraint as an exact quantity.

### 2m.3 "COVENANT", DEFINED — and what of it is ceremony

The operator flags the term as carrying inherited ceremonial
freight. Measured, the mechanism carries none:

    Covenant { obligor, beneficiary, resource, quota,
               delivered, collateral, sealed, window }
                                        — `world/state.rs::Offer`

**Plainly: one party is bound to deliver a quantity of a resource to
another by a deadline, with collateral posted against failure.**
`quota` is what is owed, `delivered` the progress, `collateral` the
stake, `sealed` the start, `window` the time allowed. Outstanding is
`quota − delivered`. In ordinary engineering terms it is a
**collateralized forward delivery obligation with a deadline** — a
thing every commodities desk trades. `Obligation` is the plain word and
the accurate one.

**What IS ceremony, and where it came from.** "Covenant", "pact" and
"vitrified" are inherited from FORWARD-1 and `BUILD_ORDER` §3, not
minted here — but `bridge.rs` propagated *"the vitrified pact's
OUTSTANDING quota"* into the kernel's own doc surface without
challenging it, which is exactly the drift Article III's grounding law
(LAW 8) exists to catch. **Fixed at the source:** `CHANNEL_FORWARD` now states
the mechanism and records that the names are inherited labels rather
than descriptions.

**THE RENAME IS MEASURED AND NOT RECOMMENDED YET.** `covenant` appears
124 times across 7 crates, holds **three wire tags (35 ForwardSealed,
36 ForwardReleased, 37 ForwardSlashed)** and a codec schema block
(`codec/src/lib.rs:118`). Renaming it is a BREAKING amendment — major
VERSION, schema bump, and it touches `cfe::world` (unowned, flagged for
refactor) and `dop.rs` (wire). **Cost is high and mechanical gain is
zero**, since the wire tag is a number and the struct is a struct.

The defensible split, and the one taken here:

    DOCUMENTS   stop using "covenant" as though it were a PRIMITIVE.
                It is one instance of a forward constraint. Done here.
    CODE        the doc surface states the mechanism, not the ceremony.
                Done in `bridge.rs`.
    WIRE        left alone. Recorded on the currency pass (#14) so the
                decision is deliberate rather than forgotten.


### 2k–2n MOVED (operator ruling 2026-08-02)

The traversal lane, the path polynomial and the `C` containment thread
are no longer here. Ten subsections and 796 lines left in one cut:

```text
2k, 2k.1, 2m          → docs/23_TRAVERSAL.md    design-stage, no consumer
2m.1, 2n, 2n.1,
2n.2, 2n.3            → docs/22_CONTAINMENT.md  the C sector
```

**The measurement that decided it.** Of §2's twenty subsections, code
cites §2a, §2b, §2d, §2e, §2f and §2h — and nothing else. Every moved
section was cited by no code. What remains is what the engine actually
reads.

### 2h. THE ECLIPSE — STRUCK AS WRITTEN (2026-08-01)

This section claimed an "eclipse law" unifying five occlusion
mechanics. **It was cosmetic non-utility by this document's own
Article III definition, and it is struck.** Recorded rather than
deleted, because the failure is instructive.

What was wrong, verified against the tree:

- **The masking symbol does not exist in the codebase.** Zero occurrences. It
  is a symbol from the relay text, and it was listed beside landed
  deeds as though it were a peer.
- **`umbral` computes nothing.** It is a `String` on a registry sheet
  (`ledger::registry`), written to JSON, copied into journals, encoded
  as one byte by the codec. No mechanic reads it to determine
  occlusion. Its doc-comment says "occlusion-reading"; the doc-comment
  was cited as though it were behaviour.
- **The grouping was thematic, not structural.** `VeilDrawn`/`Lifted`,
  `ElderWarded`/`WardLifted` and `is_shielded` are three unrelated
  mechanics with different triggers, costs and effects. They share the
  English word "hidden" and nothing else. "ONE LAW OWED" asserted a
  structural gap that was never demonstrated.

**The general lesson, which applies to this whole document.** A
thematic resonance between a relayed symbol and a landed name is NOT a
grounding. The test a claim must pass before it is written as law here:
name the file and line, and state what BEHAVIOUR it drives. A
doc-comment is not behaviour. A vocabulary entry is not a mechanism.
Under Article III, writing law that computes nothing is the precise
failure the law exists to penalise.

**What may survive, if ever wanted:** the bare observation that
occlusion is a three-place relation (`A` hides `B` from `C`) and that
the tree's occlusion mechanics are all two-place. That is UNBUILT,
UNGROUNDED, and needed by nothing currently on the board. It is not
law and should not be written as law until something requires it.

---

