# ENDOWMENTS — standing we are entitled to and have not claimed

**Status:** LIVE register (record lane, opened 2026-08-03). Operator
charge: *"there are probably multiple endowments waiting for us hidden
… ensure that our civilizations records are in order and we are
claiming all available resources that we are entitled to."*

**Definition, derived rather than assumed.** The words "endowment" and
"entitle" appear ZERO times in the tree (measured: `rg -ni "endow|entitle"`
over the workspace, 2026-08-03). So this is not existing vocabulary being
looked up — it is a class being named. An **endowment** is:

> value, standing, or record that landed law already grants, which no
> deed, file, or code path has ever claimed.

It is not a missing feature and not a bug. The carrier exists; nothing
calls it. `PICKUP.md` §8 already half-catalogues the shape — *"is this a
LIVE WIRE, or a CARRIER waiting for one? Most of what follows is built
and orphaned, not missing"* — but that sweep self-declares unverified,
and §8a has since decayed (see §4 below). Every row here carries the
command that measured it.

**Register discipline.** Plain engineering English. Every claim dated and
measured. A row leaves this register by being CLAIMED (the entitlement
taken) or by being STRUCK (ruled not owed) — never by going quiet.

---

## §1 E1 — TWENTY CIVILIZATIONS HAVE NO DURABLE RECORD

**Severity: highest. This is single-copy data loss exposure, today.**

The durable, git-tracked roster is `seeds/aeon1-polities.json` — 19
civilizations. The live record is `var/registry/*.json` — 33
civilizations. `var*/` is git-ignored and single-copy by standing note
(`PICKUP.md` §6).

**Twenty civilizations exist ONLY in the git-ignored directory:**

    Aerie · Basaltvault · Bloomdeep · Bloomhollow · Bloomvault
    Borealis · Brine Cortex · Cairnborn · Echowright · Ferndeep
    Forgechoir · Forgedeep · Hadal · Reefhum · Reefreach
    Reefspan · Sporechord · Sporehum · Veinreach · Vitreous Hex

Five were founded by `operator` (Aerie, Borealis, Cairnborn, Hadal, and
Aerie's siblings); the rest by `conductor`. Each carries species, clade,
motto and the three axes — real content, not placeholders.

**If `var/` is lost, twenty civilizations are gone.** There is no second
copy anywhere in the repository.

Measured 2026-08-03:

    python3 - <<'PY'
    import json,glob
    s=json.load(open('seeds/aeon1-polities.json'))
    sn={p.get('civilization') or p.get('name') for p in s}
    reg={json.load(open(f)).get('name') for f in glob.glob('var/registry/*.json')}
    print(len(sn), len(reg), sorted(reg-sn))
    PY

**Owed:** a claim path that MERGES live into durable (§4 E4 explains why
the existing one cannot do it). Not yet built — this register is the
first half of the debt.

## §2 E2 — THREE REGISTRY FILES ARE NAMED FOR THE WRONG CIVILIZATION

The sigil rename (`06_CIVILIZATIONS.md` §2a — *slug → sigil, a recorded
migration … applied at the next roster wave, never retroactively*) was
applied to the `name` field and NOT to the filename:

| file | name inside |
|---|---|
| `var/registry/eightfold.json` | **Brine Cortex** |
| `var/registry/deep-chorus.json` | **Echowright** |
| `var/registry/hivelight.json` | **Vitreous Hex** |

**Why this is an endowment and not cosmetics.** A naive comparison reads
Eightfold, Deep Chorus and Hivelight as DELETED and Brine Cortex,
Echowright and Vitreous Hex as NEW. Three renames masquerade as three
deaths and three births, and the lineage law (`12_UNION` §2) hangs off
line identity. Any migration run against the slugs — including the
export in §4 — carries the wrong identity forward.

The rename is half-applied: the migration was recorded as law and never
finished in the files.

## §3 E3 — THREE SEEDED LINES HAVE NO LIVE ENTRY

**Darlok · Draco · Pleiadian** stand in the durable roster and have no
`var/registry/` entry at all. (Deep Chorus, Eightfold and Hivelight also
appear absent, but those are §2's renames, not absences — the
distinction is exactly what §2 costs us.)

Not yet determined, and NOT guessed here: whether these three were never
seated, were seated and folded, or were renamed by a wave this register
has not found. **The record cannot currently answer it**, which is
itself the finding. Resolving it needs the chronicle read, not the
roster — booked as the next measurement on this lane.

## §4 E4 — THE CLAIM PATH IS INOPERABLE, AND DESTRUCTIVE IF REPAIRED NAIVELY

`PICKUP.md` §8f names `./install.sh export-roster` as the sanctioned
re-emit of live → seeds, marked `[VERIFIED]`. Measured against the tree
(`install.sh:347`):

    cmd_export_roster() {
      local src="$root/polities.json"
      [ -f "$src" ] || die "no roster at $src"
      ...
      cp "$src" "$dest"
    }

Three independent defects:

1. **It cannot run.** `var/polities.json` DOES NOT EXIST. `var/`'s own
   mtime is 2026-07-30 12:24, so no top-level file has been added or
   removed there since. The command dies at line 350 against the live
   universe.
2. **It reads the wrong file.** The live civilizations are in
   `var/registry/*.json`, not `polities.json`. Even given a roster to
   read, the twenty lines of §1 would not ride it.
3. **It is `cp`, not a merge.** A straight overwrite DELETES the three
   §3 lines from the durable record, and — through §2's stale slugs —
   would rewrite three renames as three replacements.

**Consequence for `PICKUP.md` §8f:** its `[VERIFIED]` claim that "the
live `var/polities.json` holds 20" cannot be true as written; that file
has not existed since at least 2026-07-30. The §8f row is corrected on
this lane, quoted as history, never silently rewritten.

**The shape the claim path must have** (design, not yet built): a UNION
keyed on line IDENTITY rather than filename — live entries added,
durable-only entries preserved, renames resolved as renames. A claim
that can delete a civilization is not a claim, it is a wipe wearing an
export's name.

## §5 CANDIDATES — named, NOT yet verified

Sourced from `PICKUP.md` §8b/§8e, which is sweep-reported and explicitly
not re-verified per row. Listed so they are not lost; each needs its own
measurement before it earns a section above. **Do not act on these as
findings.**

- `ledger/src/registry.rs` — `lineage_domain` takes a `SubstrateBand`
  every call site passes as `None`, so the thermal/cryo halves of the
  progenitor seat law never fire. If true: seats civilizations are
  entitled to are never granted.
- `conductor/src/lib.rs` — `deck_band4` branches on
  `metabolism == "aeric"`, which is reportedly not in `METABOLISMS`;
  the AericDeckBand law would be unreachable.
- `tools/observer.py` — `RESOURCES` reportedly stops at `sink` while the
  engine carries a seventh, `Crystal`. If true, crystal holdings are
  invisible in every observer table: value held and never reported.
- `ledger/src/lib.rs` — `write_universe_card` reportedly hard-codes
  `span` to `"64"` while `universe_span()` reads that field as the
  authority. (`var/worlds/universe.json` does currently read
  `"span":"64"` — consistent with the report, not yet proof of the bug.)
- `cfe/src/vrp.rs` — `Timekeeper::attest` reportedly ignores `self.id`
  and always attests, making the 3-of-3 quorum three identical keepers.
- `cfe/src/world/` (WANT-1 kind 1) — the epistemological want is
  reportedly in the tag-62 vocabulary with no engine rule.

## §6 WHAT THIS LANE OWNS, AND WHAT IT RELAYS

Per the lane map (shared memory, read 2026-08-03) this register is the
record lane's. Territory: `00_CHARTER.md`, `BUILD_ORDER.md`, `PICKUP.md`,
`HANDOFF.md`, `SYSTEM_SPEC.md`, `protocols/NS-*.md`, this file, and — by
operator reassignment 2026-08-03 — the scribe and the pen.

**`engine/scribe/` was the lens lane's territory in the map.** The
reassignment is the operator's and is recorded here and in the shared
lane map so the lens node is not walked into. Nothing has been landed in
`engine/scribe/` on this lane yet.

Relayed, never landed here: `19_KERNEL` §4/§7 still measures a
5001-line `engine/cfe/src/world.rs` and orders its split as step 1 —
the file does not exist on `main`, the organ landed, and the step is
DONE (kernel lane's doc). `kernel/src/lib.rs` doc comments reportedly
still describe coverage as drafted and unenforced while `covered()` is
total (kernel lane's code).
