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

## §0 THE IDENTITY KEY — settled first, because every other row depends on it

**`slug` is the identity. `name` is mutable display.** Measured
2026-08-03: every seeds entry carries `slug`, every registry file carries
`slug`, and the registry's `slug` equals its filename in ALL 33 cases.

This was not assumed — it was tested, and the first pass of this register
got it wrong by comparing on `name`. A name-keyed comparison reported
20 unrecorded civilizations and 6 missing ones. A slug-keyed comparison
reports **17 unrecorded and 3 never-seated**, because three of the
"missing" and three of the "new" are the same three lines under their
renames. The corrected figures stand below; the superseded ones are left
visible in this paragraph rather than quietly swapped, because the
mistake is the lesson: **comparing civilizations by display name
manufactures deaths and births that never happened.**

Any claim path keys on `slug` (task #3). Nothing keys on `name` or on
filename-minus-extension without checking `slug` first.

## §1 E1 — SEVENTEEN CIVILIZATIONS HAVE NO DURABLE RECORD

**Severity: highest. This is single-copy data loss exposure, today.**

The durable, git-tracked roster is `seeds/aeon1-polities.json` — 19
lines. The live record is `var/registry/*.json` — 33 lines. `var*/` is
git-ignored and single-copy by standing note (`PICKUP.md` §6).

**Seventeen slugs exist ONLY in the git-ignored directory:**

    aerie · basaltvault · bloomdeep · bloomhollow · bloomvault
    borealis · cairnborn · ferndeep · forgechoir · forgedeep
    hadal · reefhum · reefreach · reefspan · sporechord
    sporehum · veinreach

Four were founded by `operator` (Aerie, Borealis, Cairnborn, Hadal); the
rest by `conductor`. Each carries species, clade, motto and the three
axes — real content, not placeholders.

**If `var/` is lost, seventeen civilizations are gone.** There is no
second copy anywhere in the repository.

Measured 2026-08-03 (note the key — `slug`, never `name`):

    python3 - <<'PY'
    import json,glob,os
    sd={p['slug'] for p in json.load(open('seeds/aeon1-polities.json'))}
    rg={json.load(open(f))['slug'] for f in glob.glob('var/registry/*.json')}
    print(len(rg-sd), sorted(rg-sd))
    PY

**Owed:** a claim path that MERGES live into durable (§4 E4 explains why
the existing one cannot do it). Not yet built — this register is the
first half of the debt.

## §2 E2 — THE SEED ROSTER STILL CARRIES THREE PRE-RENAME NAMES

**Downgraded from "wrong identity" to "stale display name" by §0's
measurement.** The identity was never at risk; the slug held on both
sides. What diverged is the name:

| slug (the identity, agreeing) | seeds `name` | registry `name` |
|---|---|---|
| `eightfold` | Eightfold | **Brine Cortex** |
| `deep-chorus` | Deep Chorus | **Echowright** |
| `hivelight` | Hivelight | **Vitreous Hex** |

This is the sigil rename (`06_CIVILIZATIONS.md` §2a — *slug → sigil, a
recorded migration … applied at the next roster wave, never
retroactively*) landing in the live registry and never reaching the
durable roster. The law said "at the next roster wave"; the wave carried
the live side only.

**What it still costs.** Nothing structural — but any tool that compares
these two files on `name` (which is the obvious thing to write, and what
this register did first) reports three deaths and three births that
never happened. The defect is now a documented trap rather than a
corruption. The cure rides the §4 claim path: on a shared slug, the
registry's `name` is the later authority.

## §3 E3 — THREE SEEDED LINES WERE NEVER SEATED — RESOLVED 2026-08-03

**Darlok · Draco · Pleiadian** stand in the durable roster with no
registry entry and no world directory (`shroudmere`, `eclipse-hold`,
`seven-sisters` are all absent from `var/worlds/`).

**Not folded, not renamed, not lost — never seated.** The dates settle
it:

| event | when |
|---|---|
| live registry written | 2026-07-30 22:57 |
| `var/worlds/` last changed | 2026-07-31 00:33 |
| Darlok + Draco enter the roster (`66b5f8c`) | **2026-08-01 20:52** |
| Pleiadian enters the roster (`64739af`) | **2026-08-01 21:03** |

All three were added to the durable roster a day and a half AFTER the
live universe was founded and last run. The roster is seated at genesis
and `write_default_polities` refuses to overwrite (`PICKUP.md` §1), so
nothing has ever seated them.

**This is the endowment running the OTHER WAY, and it is the one the
charge names most literally.** E1 is value the live universe holds and
the record has never claimed. E3 is standing the record grants and the
live universe has never taken: three chartered lines — an `observer`
seat (Draco), a `watcher` seat (Pleiadian), and an unstationed
`umbral` line (Darlok) — sitting unclaimed because the universe has not
been re-seated since they were written.

**Owed:** seating the three, which is a universe operation and not a
document edit. Booked separately from the claim path; it needs the
operator's word on whether the live universe is re-seated or the three
wait for the next genesis.

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
   read, the seventeen lines of §1 would not ride it.
3. **It is `cp`, not a merge.** A straight overwrite DELETES the three
   §3 lines from the durable record — and those are the three that exist
   ONLY there, so the deletion is total. It also drops the seeds-side
   siting fields (`world`, `planet`, `star`, `x/y/z`, `colony`, `policy`)
   that the registry does not carry, since the two files are not the
   same schema.

**Consequence for `PICKUP.md` §8f:** its `[VERIFIED]` claim that "the
live `var/polities.json` holds 20" cannot be true as written; that file
has not existed since at least 2026-07-30. The §8f row is corrected on
this lane, quoted as history, never silently rewritten.

**THE TWO FILES ARE NOT THE SAME SCHEMA** — measured 2026-08-03, and it
is the constraint that decides the claim path's shape:

    seeds only     world · planet · star · albedo · orbit · x/y/z ·
                   colony · policy · station        (the SITING)
    registry only  clade · founder                  (the LINEAGE)
    both           slug · name · species · motto ·
                   disposition · metabolism · perception
                   (flat in seeds, nested under `axes` in the registry)

So neither file is a superset and neither can be copied over the other
without loss. The registry cannot express where a civilization sits; the
roster cannot express what founded it.

**The shape the claim path must have** (design, settled by the above;
not yet built): a UNION keyed on `slug` — live-only slugs added,
durable-only slugs preserved, and on a shared slug the registry's `name`
taken as the later authority while the seeds' siting fields are kept.
Registry-only lines will land with EMPTY siting, which must be visible
rather than defaulted: a fabricated coordinate is a worse record than an
absent one. A claim that can delete a civilization is not a claim, it is
a wipe wearing an export's name.

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
