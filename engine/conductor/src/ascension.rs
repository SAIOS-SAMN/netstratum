//! Ascension (docs/72; operator-agreed 2026-10-09): the gates, the climb, the
//! higher order.
//!
//! - **A1, the core's choice and the gates.** A line that has mastered the top
//!   of the forge's ladder weighs every core by its own lineage's prospects
//!   there: G3 read with the master's envoy at that core and the incumbents as
//!   they stand. It goes where its lineage would keep the throat (its own core
//!   included), the most systems first. Where no gate stands from the master's
//!   galaxy hub to that core, the core's parliament (the lineage keeping its
//!   throat) folds one, its span measured across the torus, paying the gate's
//!   cost in STAMP shared equally among ALL the core's ancients (born and
//!   ascended; the remainder to the first): the progenitors build the doorways
//!   to the ancients' cores. A progenitor is not seated at founding: the
//!   builders' line that takes a folded gate's tolls (`builder_payee`) becomes
//!   one at the fold.
//! - **A2, the envoy.** A master's envoy crosses a mapped gate to its core. An
//!   envoy makes its lineage a bearer of that core's wave and enters the
//!   throat's contest (`ancients::keepers`). Each bounce it stays it pays the
//!   passage toll to the gate's builders (`builder_payee`), carried through the
//!   gate; unable to pay, it is recalled. Each bounce after its crossing it
//!   judges: an envoy whose lineage does not keep the throat (it cannot out-hold
//!   the keeper) withdraws and pays no more.
//! - **A3, ascension by deed.** An envoy's lineage that keeps the core's throat
//!   for a full core period ascends: the envoy's line joins that core's
//!   ancients (`AncientArose`, tag 192) and the house lists council seats for it
//!   (Concordance 3.9).
//! - **The roads.** A mapped gate carries its galaxy's trade to the core's hub
//!   (`roads`, read by the hub clearing): the gate's span melt replaces the
//!   open-space distance, and every unit carried pays the road toll to the
//!   gate's builders.
//!
//! Concordance 5.9 (coordinator ruling (a), 2026-10-09): every crossing takes
//! its coordinates from the hypha's map, so an envoy crosses, and trade rides,
//! only a gate mapped at its hub's present place. Each bounce a gate whose hub
//! has drifted from its mapped place by more than its freshness (in cells) goes
//! back to unmapped and is offered again; envoys already through stay put, and
//! owe no toll for a passage they cannot use.
//!
//! Declared (until a law derives them): a gate's cost is the ladder's dearest
//! closure; its span's melt fraction is span x 1/64 (the passage toll each
//! bounce is that fraction of one STAMP; the roads melt by it); its freshness
//! is one cell. State: `<var>/ascension.bin`, binary records (Concordance 1.13).
use crate::ancients::{keeper_at, root_of, Keeper};
use crate::Polity;
use cfe::ratio::{ratio, Ratio};
use ledger::registry::Registry;
use ledger::Ledger;
use solver::Difficulty;
use std::collections::BTreeMap;
use std::path::Path;

type Place = (Ratio, Ratio, Ratio);

/// Declared (docs/59): a gate's freshness, the cells its hub may drift from
/// its mapped place before the map no longer holds it.
pub const FRESHNESS_CELLS: u64 = 1;

/// One gate: from a galaxy's hub to a core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gate {
    /// The galaxy it opens from (its hub is the near end).
    pub from: String,
    /// The core it opens on.
    pub core: String,
    /// The lineage root that built it (the core's parliament then).
    pub builder: String,
    /// Its span in cells (the hub's torus distance to the core, floored).
    pub span: u64,
    /// Mapped on the hypha's map at `place` (A4): only a mapped gate is crossed.
    pub mapped: bool,
    pub built: u64,
    /// The hub's place the gate is offered (and mapped) at.
    pub place: Place,
    /// The cells the hub may drift from `place` before the gate recalibrates.
    pub freshness: u64,
}

impl Gate {
    /// The gate's name on the map: its galaxy and its core.
    pub fn id(&self) -> String {
        format!("{}>{}", self.from, self.core)
    }
}

/// One envoy: a master's line at a core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envoy {
    pub line: String,
    pub core: String,
    pub since: u64,
}

/// A gate being paid for: one load a trip, one trip a bounce (the haul law:
/// a crossing carries no more than the payer's free hulls hold, and a hauler
/// runs the route again on a later beat). It folds when the last load lands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Folding {
    pub from: String,
    pub core: String,
    /// The lineage root of the parliament that pays (its builders).
    pub builder: String,
    /// The master it is folded for.
    pub master: String,
    /// What each ancient of the core is still owed, in order (the first takes
    /// the remainder of the equal shares).
    pub owed: Vec<(String, Ratio)>,
    /// The trips run so far.
    pub trips: u64,
    pub started: u64,
}

/// A mapped gate as a road for the hub clearing: its galaxy, its core, the
/// melt its span takes, and the line its builders take the toll through.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Road {
    pub from: String,
    pub core: String,
    pub melt: Ratio,
    pub payee: Option<usize>,
}

fn put_text(text: &str, out: &mut Vec<u8>) {
    out.extend_from_slice(&(text.len() as u16).to_le_bytes());
    out.extend_from_slice(text.as_bytes());
}

fn get_text(value: &[u8], at: &mut usize) -> Option<String> {
    let len = u16::from_le_bytes(value.get(*at..*at + 2)?.try_into().ok()?) as usize;
    let s = String::from_utf8(value.get(*at + 2..*at + 2 + len)?.to_vec()).ok()?;
    *at += 2 + len;
    Some(s)
}

fn get_u64(value: &[u8], at: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(value.get(*at..*at + 8)?.try_into().ok()?);
    *at += 8;
    Some(v)
}

fn get_place(value: &[u8], at: &mut usize) -> Option<Place> {
    let mut c = Vec::new();
    for _ in 0..3 {
        let (r, used) = value.get(*at..).and_then(cfe::ratio::decode)?;
        *at += used;
        c.push(r);
    }
    let mut c = c.into_iter();
    Some((c.next()?, c.next()?, c.next()?))
}

fn put_place(place: &Place, out: &mut Vec<u8>) {
    for c in [&place.0, &place.1, &place.2] {
        cfe::ratio::encode(c, out);
    }
}

/// The book: tag 1 a gate (from, core, builder, span u64, mapped u8, built
/// u64, freshness u64, its place as three ratios); tag 2 an envoy (line, core,
/// since u64).
pub fn read(var_root: &Path) -> (Vec<Gate>, Vec<Envoy>) {
    let (gates, envoys, _) = read_all(var_root);
    (gates, envoys)
}

/// The whole book, the gates being paid for too (tag 3: from, core, builder,
/// master, trips u64, started u64, count u16, then each (ancient, owed ratio)).
pub fn read_all(var_root: &Path) -> (Vec<Gate>, Vec<Envoy>, Vec<Folding>) {
    let mut foldings = Vec::new();
    let bytes = std::fs::read(var_root.join("ascension.bin")).unwrap_or_default();
    for (tag, value) in cfe::tlv::records(&bytes).filter(|(t, _)| *t == 3) {
        let _ = tag;
        let mut at = 0;
        let folding = (|| {
            let (from, core, builder, master) = (get_text(value, &mut at)?, get_text(value, &mut at)?, get_text(value, &mut at)?, get_text(value, &mut at)?);
            let (trips, started) = (get_u64(value, &mut at)?, get_u64(value, &mut at)?);
            let count = u16::from_le_bytes(value.get(at..at + 2)?.try_into().ok()?);
            at += 2;
            let mut owed = Vec::new();
            for _ in 0..count {
                let sigil = get_text(value, &mut at)?;
                let (r, used) = value.get(at..).and_then(cfe::ratio::decode)?;
                at += used;
                owed.push((sigil, r));
            }
            Some(Folding { from, core, builder, master, owed, trips, started })
        })();
        foldings.extend(folding);
    }
    let (gates, envoys) = read_gates_envoys(&bytes);
    (gates, envoys, foldings)
}

fn read_gates_envoys(bytes: &[u8]) -> (Vec<Gate>, Vec<Envoy>) {
    let (mut gates, mut envoys) = (Vec::new(), Vec::new());
    for (tag, value) in cfe::tlv::records(bytes) {
        let mut at = 0;
        match tag {
            1 => {
                let gate = (|| {
                    let (from, core, builder) = (get_text(value, &mut at)?, get_text(value, &mut at)?, get_text(value, &mut at)?);
                    let span = get_u64(value, &mut at)?;
                    let mapped = *value.get(at)? != 0;
                    at += 1;
                    let (built, freshness) = (get_u64(value, &mut at)?, get_u64(value, &mut at)?);
                    let place = get_place(value, &mut at)?;
                    Some(Gate { from, core, builder, span, mapped, built, place, freshness })
                })();
                gates.extend(gate);
            }
            2 => {
                let (Some(line), Some(core), Some(since)) = (get_text(value, &mut at), get_text(value, &mut at), get_u64(value, &mut at)) else { continue };
                envoys.push(Envoy { line, core, since });
            }
            _ => {}
        }
    }
    (gates, envoys)
}

fn write(var_root: &Path, gates: &[Gate], envoys: &[Envoy], foldings: &[Folding]) -> std::io::Result<()> {
    let mut out = Vec::new();
    for f in foldings {
        let mut v = Vec::new();
        for t in [&f.from, &f.core, &f.builder, &f.master] {
            put_text(t, &mut v);
        }
        v.extend_from_slice(&f.trips.to_le_bytes());
        v.extend_from_slice(&f.started.to_le_bytes());
        v.extend_from_slice(&(f.owed.len() as u16).to_le_bytes());
        for (sigil, r) in &f.owed {
            put_text(sigil, &mut v);
            cfe::ratio::encode(r, &mut v);
        }
        cfe::tlv::put(3, &v, &mut out);
    }
    for g in gates {
        let mut v = Vec::new();
        put_text(&g.from, &mut v);
        put_text(&g.core, &mut v);
        put_text(&g.builder, &mut v);
        v.extend_from_slice(&g.span.to_le_bytes());
        v.push(u8::from(g.mapped));
        v.extend_from_slice(&g.built.to_le_bytes());
        v.extend_from_slice(&g.freshness.to_le_bytes());
        put_place(&g.place, &mut v);
        cfe::tlv::put(1, &v, &mut out);
    }
    for e in envoys {
        let mut v = Vec::new();
        put_text(&e.line, &mut v);
        put_text(&e.core, &mut v);
        v.extend_from_slice(&e.since.to_le_bytes());
        cfe::tlv::put(2, &v, &mut out);
    }
    let path = var_root.join("ascension.bin");
    let staged = path.with_extension("staged");
    std::fs::write(&staged, out)?;
    std::fs::rename(staged, path)
}

/// The envoys as `(line, core)`, for the throat's contest.
pub fn envoys(var_root: &Path) -> Vec<(String, String)> {
    read(var_root).1.into_iter().map(|e| (e.line, e.core)).collect()
}

/// A4: the gates the house sealed on the hypha's map, as `(gate id, the place
/// it was mapped at)` (`gates-mapped.bin`, tag 2, written by ns-hypha).
fn sealed(var_root: &Path) -> Vec<(String, Place)> {
    let bytes = std::fs::read(var_root.join("gates-mapped.bin")).unwrap_or_default();
    cfe::tlv::records(&bytes)
        .filter(|(t, _)| *t == 2)
        .filter_map(|(_, v)| {
            let mut at = 0;
            Some((get_text(v, &mut at)?, get_place(v, &mut at)?))
        })
        .collect()
}

/// Declared: a gate's cost, the ladder's dearest closure.
fn gate_cost() -> Ratio {
    (0..=Difficulty::top()).map(|r| kernel::mint_bound(Some(r))).max().unwrap_or_else(|| ratio(0, 1))
}

/// Declared: a gate span's melt fraction, span x 1/64: what a crossing of the
/// gate melts (the roads), and the passage toll each bounce in STAMP.
pub fn gate_melt(span: u64) -> Ratio {
    ratio(i64::try_from(span).unwrap_or(i64::MAX), 64)
}

fn stamp_of(p: &Polity) -> Ratio {
    p.world.orb().map(|o| o.stamp_amount()).unwrap_or_else(|| ratio(0, 1))
}

fn cells(distance: &Ratio) -> u64 {
    num_traits::ToPrimitive::to_u64(&distance.floor().to_integer()).unwrap_or(0)
}

/// The open-space melt between two lines (the iceberg law over the torus).
fn open_melt(worlds: &[Polity], a: usize, b: usize, span: &Ratio) -> Ratio {
    let distance = crate::spatial_l1(&worlds[a].spec.position, &worlds[b].spec.position, span);
    cfe::world::route_melt(&worlds[a].world, &worlds[b].world, &distance)
}

/// The melt of line `i`'s road through `gate`: its leg to its own galaxy's hub,
/// then the gate's span (what survives is what survives both).
fn through(worlds: &[Polity], i: usize, home_hub: usize, gate: &Gate, span: &Ratio) -> Ratio {
    let one = ratio(1, 1);
    let leg = if i == home_hub { ratio(0, 1) } else { open_melt(worlds, i, home_hub, span) };
    one.clone() - (one.clone() - leg) * (one - gate_melt(gate.span))
}

/// One trip: carry up to `amount` STAMP from line `from` to line `to` on a
/// road melting `melt`, no more than the payer's free hulls hold on a melting
/// road (the haul law; a road that does not melt asks no hull). Returns what
/// was carried and what arrived, or why nothing crossed (walled, short, no
/// free hull, or the transfer's refusal). The aperture needs the stellar leap
/// (THRESHOLD-1), granted to both as the hub clearing grants it.
fn trip(worlds: &mut [Polity], from: usize, to: usize, amount: &Ratio, deal: u64, melt: &Ratio) -> Result<(Ratio, Ratio), String> {
    if from == to {
        return Err("payer and payee are one line".into());
    }
    if *melt >= ratio(1, 1) {
        return Err(format!("walled (melt {melt})"));
    }
    let mut load = cfe::world::smelt_resource(amount);
    if *melt > ratio(0, 1) {
        let hold = cfe::world::smelt_resource(&worlds[from].world.free_capacity(deal));
        if hold <= ratio(0, 1) {
            return Err("no free hull".into());
        }
        load = load.min(hold);
    }
    let have = stamp_of(&worlds[from]);
    if have < load {
        return Err(format!("short ({have} of {load} STAMP)"));
    }
    let carried = cfe::dop::Positive::new(load.clone()).ok_or("nothing to carry")?;
    for i in [from, to] {
        if !worlds[i].world.knows_its_star() {
            let steward = worlds[i].steward;
            let _ = worlds[i].world.grant_tech_as(steward, cfe::world::FIRST_ORB, 0);
        }
    }
    let before = stamp_of(&worlds[to]);
    let (a, b) = crate::two_mut(worlds, from, to);
    cfe::world::stamp_aperture_transfer(&mut a.world, cfe::world::FIRST_ORB, &mut b.world, cfe::world::FIRST_ORB, carried, deal, melt).map_err(|e| format!("{e:?}"))?;
    Ok((load, stamp_of(&worlds[to]) - before))
}

fn parents_of(registry: &Registry) -> BTreeMap<String, Vec<String>> {
    registry.list().into_iter().map(|c| (c.sigil, c.lineage)).collect()
}

/// Every galaxy's hub (`crate::galaxy_hub`, the hub clearing's own).
fn hubs(worlds: &[Polity], book: &Ledger, var_root: &Path, span: &Ratio) -> BTreeMap<String, usize> {
    crate::galaxy_members(worlds, book)
        .into_iter()
        .filter_map(|(core, members)| crate::galaxy_hub(worlds, book, var_root, &core, &members, span).map(|h| (core, h)))
        .collect()
}

/// The gate's builders' LINE, who takes its tolls: the builder lineage's
/// richest living line at the core, or, keeping the throat through an envoy
/// with no line living there, its rider.
pub(crate) fn builder_payee(worlds: &[Polity], cores: &[Option<String>], parents: &BTreeMap<String, Vec<String>>, keepers: &BTreeMap<String, Keeper>, gate: &Gate) -> Option<usize> {
    (0..worlds.len())
        .filter(|&i| {
            !worlds[i].penned
                && cores[i].as_deref() == Some(gate.core.as_str())
                && root_of(&worlds[i].spec.civilization.sigil, parents) == gate.builder
        })
        .max_by(|a, b| stamp_of(&worlds[*a]).cmp(&stamp_of(&worlds[*b])).then(worlds[*b].spec.civilization.sigil.cmp(&worlds[*a].spec.civilization.sigil)))
        .or_else(|| {
            let k = keepers.get(&gate.core).filter(|k| k.line == gate.builder)?;
            worlds.iter().position(|p| !p.penned && p.spec.civilization.sigil == k.rider)
        })
}

/// The roads for the hub clearing: every mapped gate, its span's melt, and its
/// builders' line, read with the beat's one reading of the keepers.
pub fn roads(worlds: &[Polity], book: &Ledger, registry: &Registry, var_root: &Path, keepers: &BTreeMap<String, Keeper>) -> Vec<Road> {
    let mapped: Vec<Gate> = read(var_root).0.into_iter().filter(|g| g.mapped).collect();
    if mapped.is_empty() {
        return Vec::new();
    }
    let parents = parents_of(registry);
    let cores = crate::ancients::cores_of(worlds, book);
    mapped
        .iter()
        .map(|g| Road { from: g.from.clone(), core: g.core.clone(), melt: gate_melt(g.span), payee: builder_payee(worlds, &cores, &parents, keepers, g) })
        .collect()
}

/// A1 the core's choice: the core master `i`'s lineage would keep (G3 read
/// with its envoy there and every incumbent as it stands), the most systems
/// first, then its own galaxy, then by name; and those systems.
fn choose_core(
    worlds: &[Polity],
    book: &Ledger,
    cores: &[Option<String>],
    parents: &BTreeMap<String, Vec<String>>,
    envoys: &[(String, String)],
    galaxies: &[ledger::galaxy::Galaxy],
    i: usize,
    home: &str,
) -> Option<(String, u64)> {
    let sigil = &worlds[i].spec.civilization.sigil;
    let root = root_of(sigil, parents);
    galaxies
        .iter()
        .filter_map(|g| {
            let mut with = envoys.to_vec();
            with.push((sigil.clone(), g.name.clone()));
            let k = keeper_at(worlds, book, cores, parents, &with, &g.name, &g.center)?;
            (k.line == root).then(|| (g.name.clone(), k.systems))
        })
        .max_by(|(a, sa), (b, sb)| sa.cmp(sb).then((a == home).cmp(&(b == home))).then(b.cmp(a)))
}

/// The systems lineage `root` holds at `core`: its lines living there, and
/// its envoy's line `envoy` if that one lives elsewhere.
fn lineage_systems(worlds: &[Polity], cores: &[Option<String>], parents: &BTreeMap<String, Vec<String>>, root: &str, core: &str, envoy: usize) -> u64 {
    let living: u64 = (0..worlds.len())
        .filter(|&i| !worlds[i].penned && cores[i].as_deref() == Some(core) && root_of(&worlds[i].spec.civilization.sigil, parents) == root)
        .map(|i| worlds[i].world.orbs().len() as u64)
        .sum();
    let away = cores[envoy].as_deref() != Some(core);
    living + if away { worlds[envoy].world.orbs().len() as u64 } else { 0 }
}

/// docs/77 — A gate's account: what it cost its parliament and what passage
/// over it earned its builders (road tolls on the trade that rode it, and the
/// envoys' passage tolls), keyed by (galaxy, core). `<var>/gate-accounts.bin`
/// (Concordance 1.13): tag 1 (galaxy, core, cost ratio, earned ratio).
fn read_accounts(var_root: &Path) -> BTreeMap<(String, String), (Ratio, Ratio)> {
    let bytes = std::fs::read(var_root.join("gate-accounts.bin")).unwrap_or_default();
    cfe::tlv::records(&bytes)
        .filter(|(t, _)| *t == 1)
        .filter_map(|(_, v)| {
            let mut at = 0;
            let (from, core) = (get_text(v, &mut at)?, get_text(v, &mut at)?);
            let (cost, used) = v.get(at..).and_then(cfe::ratio::decode)?;
            at += used;
            let (earned, _) = v.get(at..).and_then(cfe::ratio::decode)?;
            Some(((from, core), (cost, earned)))
        })
        .collect()
}

fn write_accounts(var_root: &Path, accounts: &BTreeMap<(String, String), (Ratio, Ratio)>) {
    let mut out = Vec::new();
    for ((from, core), (cost, earned)) in accounts {
        let mut v = Vec::new();
        put_text(from, &mut v);
        put_text(core, &mut v);
        cfe::ratio::encode(cost, &mut v);
        cfe::ratio::encode(earned, &mut v);
        cfe::tlv::put(1, &v, &mut out);
    }
    let path = var_root.join("gate-accounts.bin");
    let staged = path.with_extension("staged");
    if std::fs::write(&staged, out).is_ok() {
        let _ = std::fs::rename(staged, path);
    }
}

/// Add to a gate's account (`cost` at its folding, `earned` as passage pays).
fn credit(var_root: &Path, from: &str, core: &str, cost: &Ratio, earned: &Ratio) {
    let mut accounts = read_accounts(var_root);
    let entry = accounts.entry((from.to_string(), core.to_string())).or_insert_with(|| (ratio(0, 1), ratio(0, 1)));
    entry.0 += cost.clone();
    entry.1 += earned.clone();
    write_accounts(var_root, &accounts);
}

/// docs/77 — the gates' accounts at a term's close: the term's road tolls
/// added to each gate's earnings, and each standing gate's cost, earnings and
/// payback (earned over cost) for the log. A gate folded before its account
/// was kept is booked at the cost every gate paid (`gate_cost`).
pub fn accounts(var_root: &Path, road_tolls: &BTreeMap<(String, String), Ratio>) -> Vec<String> {
    let mut accounts = read_accounts(var_root);
    for ((from, core), arrived) in road_tolls {
        accounts.entry((from.clone(), core.clone())).or_insert_with(|| (ratio(0, 1), ratio(0, 1))).1 += arrived.clone();
    }
    let mut said = Vec::new();
    for g in read(var_root).0 {
        let entry = accounts.entry((g.from.clone(), g.core.clone())).or_insert_with(|| (ratio(0, 1), ratio(0, 1)));
        if entry.0 == ratio(0, 1) {
            entry.0 = cfe::world::smelt_resource(&gate_cost());
        }
        let (cost, earned) = entry.clone();
        let payback = if cost > ratio(0, 1) { earned.clone() / cost.clone() } else { ratio(0, 1) };
        said.push(format!(
            "GATE ACCOUNT {} — cost {cost} STAMP, earned {earned} STAMP by its builders ({}), payback {payback}{}",
            g.id(),
            g.builder,
            if g.mapped { "" } else { "; unmapped" }
        ));
    }
    write_accounts(var_root, &accounts);
    said
}

/// The ascension step, once a bounce, after the ancients' step, with the
/// bounce's one reading of the keepers. Returns what happened, for the log.
pub fn step(worlds: &mut [Polity], book: &Ledger, registry: &Registry, var_root: &Path, keepers: &BTreeMap<String, Keeper>, bounce: u64, stamp: u64) -> Vec<String> {
    let mut said = Vec::new();
    let span = book.universe_span();
    let (mut gates, mut envoys, mut foldings) = read_all(var_root);
    let galaxies = book.galaxies();
    let centre_of = |core: &str| galaxies.iter().find(|g| g.name == core).map(|g| g.center.clone());
    let hubs = hubs(worlds, book, var_root, &span);
    // Each line's core, read once for the step (a line does not change galaxy
    // within it).
    let cores = crate::ancients::cores_of(worlds, book);
    // 5.9 recalibration: a gate whose hub drifted from its place by more than
    // its freshness leaves the map and is offered again at the hub's place
    // now; a gate sealed by the house at its offered place reads mapped.
    let sealed = sealed(var_root);
    for g in gates.iter_mut() {
        let (Some(&hub), Some(centre)) = (hubs.get(&g.from), centre_of(g.core.as_str())) else { continue };
        let now = worlds[hub].spec.position.clone();
        let drift = ledger::torus_distance(&now, &g.place, &span);
        if drift > ratio(i64::try_from(g.freshness).unwrap_or(i64::MAX), 1) {
            if g.mapped {
                g.mapped = false;
                said.push(format!("THE GATE {} RECALIBRATES — its hub drifted {drift} cells (freshness {}); offered for re-mapping", g.id(), g.freshness));
            }
            g.place = now;
            g.span = cells(&ledger::torus_distance(&g.place, &centre, &span));
        }
        if !g.mapped && sealed.iter().any(|(id, place)| *id == g.id() && *place == g.place) {
            g.mapped = true;
            said.push(format!("THE GATE {} IS ON THE MAP — envoys may cross, trade may ride", g.id()));
        }
    }
    let arisen = crate::ancients::arisen(var_root);
    let is_ancient = |sigil: &str| arisen.iter().any(|(_, s)| s == sigil);
    let parents = parents_of(registry);
    let envoys_now: Vec<(String, String)> = envoys.iter().map(|e| (e.line.clone(), e.core.clone())).collect();
    let top = Difficulty::top();
    let mut seq = 0u64;
    let mut deal = || {
        seq += 1;
        stamp.saturating_mul(100_000).saturating_add(95_000 + seq)
    };
    // A1: each master's choice of core, by its lineage's prospects there.
    let masters: Vec<(usize, String, String, u64)> = (0..worlds.len())
        .filter(|&i| !worlds[i].penned && worlds[i].level >= top && !is_ancient(&worlds[i].spec.civilization.sigil))
        .filter(|&i| !envoys.iter().any(|e| e.line == worlds[i].spec.civilization.sigil))
        .filter_map(|i| {
            let home = cores[i].clone()?;
            let (target, systems) = choose_core(worlds, book, &cores, &parents, &envoys_now, &galaxies, i, &home)?;
            Some((i, home, target, systems))
        })
        .collect();
    // A1: a chosen core's parliament begins a gate for its master: the cost
    // in equal shares on the grain among every living ancient of the core,
    // the remainder to the first.
    for (i, home, target, systems) in &masters {
        if gates.iter().any(|g| &g.from == home && &g.core == target) || foldings.iter().any(|f| &f.from == home && &f.core == target) {
            continue;
        }
        let Some(keeper) = keepers.get(target) else { continue };
        let ancients: Vec<String> = arisen
            .iter()
            .filter(|(c, _)| c == target)
            .filter(|(_, s)| worlds.iter().any(|p| !p.penned && &p.spec.civilization.sigil == s))
            .map(|(_, s)| s.clone())
            .collect();
        if ancients.is_empty() {
            continue;
        }
        // A parliament with no line at its core but its ancients has no one
        // to pay: no gate is begun.
        let can_pay = (0..worlds.len()).any(|j| {
            !worlds[j].penned
                && !ancients.contains(&worlds[j].spec.civilization.sigil)
                && cores[j].as_deref() == Some(target.as_str())
                && root_of(&worlds[j].spec.civilization.sigil, &parents) == keeper.line
        });
        if !can_pay {
            continue;
        }
        let cost = cfe::world::smelt_resource(&gate_cost());
        let n = ancients.len() as i64;
        let share = cfe::world::smelt_resource(&(cost.clone() / ratio(n, 1)));
        let first = cost - share.clone() * ratio(n - 1, 1);
        let owed = ancients.into_iter().enumerate().map(|(k, a)| (a, if k == 0 { first.clone() } else { share.clone() })).collect();
        let master = worlds[*i].spec.civilization.sigil.clone();
        said.push(format!(
            "A GATE {home}>{target} IS BEGUN by the parliament of {} for {master} (its lineage would hold {systems} systems there); paid in trips to {n} ancients",
            keeper.line
        ));
        foldings.push(Folding { from: home.clone(), core: target.clone(), builder: keeper.line.clone(), master, owed, trips: 0, started: bounce });
    }
    // A1: one trip a bounce for each gate being paid for, from the builders'
    // richest line at the core that is not an ancient, to the first ancient
    // still owed; the gate folds when the last load lands.
    let mut paying = Vec::new();
    for mut f in std::mem::take(&mut foldings) {
        let label = format!("{}>{}", f.from, f.core);
        // An ancient no longer living is owed nothing more.
        f.owed.retain(|(s, r)| *r > ratio(0, 1) && worlds.iter().any(|p| !p.penned && &p.spec.civilization.sigil == s));
        let total = cfe::world::smelt_resource(&gate_cost());
        if !f.owed.is_empty() {
            let ancients: Vec<usize> = arisen.iter().filter(|(c, _)| *c == f.core).filter_map(|(_, s)| worlds.iter().position(|p| !p.penned && &p.spec.civilization.sigil == s)).collect();
            let to = worlds.iter().position(|p| !p.penned && p.spec.civilization.sigil == f.owed[0].0);
            // The trip's payer: of the builders' lines at the core that are
            // not its ancients, the richest whose hulls are free on the road
            // to the ancient (a road that does not melt asks no hull).
            let id = deal();
            let payer = to.and_then(|to| {
                (0..worlds.len())
                    .filter(|&j| {
                        !worlds[j].penned
                            && !ancients.contains(&j)
                            && cores[j].as_deref() == Some(f.core.as_str())
                            && root_of(&worlds[j].spec.civilization.sigil, &parents) == f.builder
                            && stamp_of(&worlds[j]) > ratio(0, 1)
                            && (open_melt(worlds, j, to, &span) == ratio(0, 1) || worlds[j].world.free_capacity(id) > ratio(0, 1))
                    })
                    .max_by(|a, b| stamp_of(&worlds[*a]).cmp(&stamp_of(&worlds[*b])))
            });
            let (Some(payer), Some(to)) = (payer, to) else {
                said.push(format!("A GATE {label} IS PAYING — trip {} waits: no line of {} at {} with STAMP and a free hull", f.trips + 1, f.builder, f.core));
                paying.push(f);
                continue;
            };
            let melt = open_melt(worlds, payer, to, &span);
            let owed = f.owed[0].1.clone();
            match trip(worlds, payer, to, &owed, id, &melt) {
                Ok((carried, arrived)) => {
                    f.trips += 1;
                    f.owed[0].1 = owed - carried.clone();
                    if f.owed[0].1 <= ratio(0, 1) {
                        f.owed.remove(0);
                    }
                    let left = f.owed.iter().fold(ratio(0, 1), |t, (_, r)| t + r.clone());
                    said.push(format!(
                        "A GATE {label} IS PAYING — trip {}: {carried} STAMP carried by {} to {} ({arrived} arrived); {} of {total} STAMP paid",
                        f.trips,
                        worlds[payer].spec.civilization.sigil,
                        worlds[to].spec.civilization.sigil,
                        total.clone() - left
                    ));
                }
                Err(why) => {
                    said.push(format!("A GATE {label} IS PAYING — trip {} refused: {why}", f.trips + 1));
                }
            }
        }
        if !f.owed.is_empty() {
            paying.push(f);
            continue;
        }
        let (Some(&hub), Some(centre)) = (hubs.get(&f.from), centre_of(f.core.as_str())) else {
            paying.push(f);
            continue;
        };
        let place = worlds[hub].spec.position.clone();
        let cells = cells(&ledger::torus_distance(&place, &centre, &span));
        said.push(format!(
            "A GATE IS FOLDED {label} by the parliament of {} for {} in {} trips (span {cells}); unmapped until the house maps it",
            f.builder, f.master, f.trips
        ));
        credit(var_root, &f.from, &f.core, &total, &ratio(0, 1));
        let gate = Gate { from: f.from, core: f.core, builder: f.builder, span: cells, mapped: false, built: bounce, place, freshness: FRESHNESS_CELLS };
        // The progenitors emerge (operator 2026-10-21): the doorway's builders'
        // line, the one who takes its tolls, is a progenitor from the fold; an
        // ancient is already above the doorways.
        if let Some(b) = builder_payee(worlds, &cores, &parents, keepers, &gate) {
            let ancients: Vec<String> = arisen.iter().map(|(_, s)| s.clone()).collect();
            if crate::tiers::raise(&mut worlds[b], registry, "progenitor", &ancients) {
                said.push(format!("A PROGENITOR EMERGES — {}, whose lineage {} folded the gate {}", worlds[b].spec.civilization.sigil, gate.builder, gate.id()));
            }
        }
        gates.push(gate);
    }
    foldings = paying;
    // A2: the masters' envoys cross mapped gates to their chosen cores (5.9).
    for (i, home, target, _) in &masters {
        if gates.iter().any(|g| &g.from == home && &g.core == target && g.mapped) {
            let line = worlds[*i].spec.civilization.sigil.clone();
            envoys.push(Envoy { line: line.clone(), core: target.clone(), since: bounce });
            said.push(format!("AN ENVOY CROSSES — {line} from {home} to {target}"));
        }
    }
    // Each bounce: the envoys judge, then pay the passage toll to the gate's
    // builders through the gate.
    let mut kept = Vec::new();
    for e in envoys {
        let Some(at) = worlds.iter().position(|p| !p.penned && p.spec.civilization.sigil == e.line) else {
            said.push(format!("AN ENVOY LEAVES — {} from {} (its line is gone)", e.line, e.core));
            continue;
        };
        // An ascended line sits on the council: its passage is over.
        if is_ancient(&e.line) {
            continue;
        }
        // An envoy is a master's: a line fallen below the ladder's top is recalled.
        if worlds[at].level < top {
            said.push(format!("AN ENVOY IS RECALLED — {} from {} (no longer at the ladder's top)", e.line, e.core));
            continue;
        }
        // The judgement, each bounce after its crossing: with its envoy counted
        // (the one reading), does its lineage keep the throat?
        let root = root_of(&e.line, &parents);
        if e.since < bounce && keepers.get(&e.core).is_none_or(|k| k.line != root) {
            let held = lineage_systems(worlds, &cores, &parents, &root, &e.core, at);
            let (line, systems) = keepers.get(&e.core).map(|k| (k.line.clone(), k.systems)).unwrap_or_default();
            said.push(format!("AN ENVOY WITHDRAWS — {} from {}: its lineage {root} holds {held} systems there, the keeper {line} {systems}", e.line, e.core));
            continue;
        }
        let home = cores[at].clone().unwrap_or_default();
        let Some(gate) = gates.iter().find(|g| g.from == home && g.core == e.core).or_else(|| gates.iter().find(|g| g.core == e.core)) else {
            kept.push(e);
            continue;
        };
        // Through an unmapped gate nothing crosses (5.9): the envoy stays put
        // and owes nothing for a passage it cannot use.
        if !gate.mapped {
            kept.push(e);
            continue;
        }
        let Some(payee) = builder_payee(worlds, &cores, &parents, keepers, gate).filter(|&p| p != at) else {
            kept.push(e);
            continue;
        };
        let melt = hubs.get(&gate.from).map(|&h| through(worlds, at, h, gate, &span)).unwrap_or_else(|| gate_melt(gate.span));
        // The toll is one trip: carried whole, or unpaid.
        let toll = cfe::world::smelt_resource(&gate_melt(gate.span));
        // A passage that costs nothing is owed nothing.
        if toll <= ratio(0, 1) {
            kept.push(e);
            continue;
        }
        match trip(worlds, at, payee, &toll, deal(), &melt) {
            Ok((carried, arrived)) if carried == toll => {
                credit(var_root, &gate.from, &gate.core, &ratio(0, 1), &arrived);
                kept.push(e);
            }
            Ok((carried, _)) => said.push(format!("AN ENVOY IS RECALLED — {} from {} (the passage toll unpaid: its hulls carried {carried} of {toll})", e.line, e.core)),
            Err(why) => said.push(format!("AN ENVOY IS RECALLED — {} from {} (the passage toll unpaid: {why})", e.line, e.core)),
        }
    }
    envoys = kept;
    // A3: ascension by deed, an envoy keeping the throat a full core period.
    let mut books = crate::ancients::read_books(var_root);
    let mut ascending: Vec<String> = Vec::new();
    for (core, keeper) in keepers {
        let Some(envoy) = envoys.iter().find(|e| &e.core == core && e.line == keeper.rider) else { continue };
        let Some(entry) = books.get_mut(core) else { continue };
        let Some((root, since)) = &entry.kept else { continue };
        // A full core period kept by the lineage, and served by the envoy itself
        // since its own crossing.
        if root != &keeper.line || bounce < since + 1 || bounce < envoy.since + 1 || entry.ascended.contains(&envoy.line) || entry.arisen.as_ref() == Some(&envoy.line) {
            continue;
        }
        let living = (0..worlds.len()).filter(|&i| !worlds[i].penned && cores[i].as_deref() == Some(core.as_str())).count() as u64;
        let event = cfe::dop::ClosureEvent::AncientArose { sigil: envoy.line.clone(), tree: keeper.systems, living };
        if book.append_universe(vec![event], stamp).is_ok() {
            entry.ascended.push(envoy.line.clone());
            said.push(format!("AN ANCIENT ASCENDS at {core} — {} by deed, through the gate", envoy.line));
            ascending.push(envoy.line.clone());
        }
    }
    // The ascended envoys' passage ends: they sit on the council now.
    envoys.retain(|e| !ascending.contains(&e.line));
    let _ = crate::ancients::write_books(var_root, &books);
    let _ = write(var_root, &gates, &envoys, &foldings);
    // A4: the unmapped gates for the house to map, each from its galaxy's hub
    // place to its core's centre, kept by its builders' line: tag 1 (gate id,
    // core, the keeper's world, six ratios, freshness u64).
    let mut out = Vec::new();
    for g in gates.iter().filter(|g| !g.mapped) {
        let Some(centre) = centre_of(g.core.as_str()) else { continue };
        let Some(keeper) = builder_payee(worlds, &cores, &parents, keepers, g).map(|k| worlds[k].spec.world_name.clone()) else { continue };
        let mut v = Vec::new();
        put_text(&g.id(), &mut v);
        put_text(&g.core, &mut v);
        put_text(&keeper, &mut v);
        put_place(&g.place, &mut v);
        put_place(&centre, &mut v);
        v.extend_from_slice(&g.freshness.to_le_bytes());
        cfe::tlv::put(1, &v, &mut out);
    }
    let path = var_root.join("gates-map.bin");
    let staged = path.with_extension("staged");
    if std::fs::write(&staged, out).is_ok() {
        let _ = std::fs::rename(staged, path);
    }
    said
}
