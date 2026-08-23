//! ERA CONVERGENCE (07 — CONVERGE-1): annex the archived timelines.
//!
//! The laws (07 §1): annex, never merge — archived chronicles are
//! sealed, convergence appends `EraAnnexed` receipts to the universe
//! ledger (`<root>/convergence.tlv`, real cfe chain machinery); verify
//! before adopting — a broken chain quarantines, named, never merged
//! and never deleted; one T axis by epoch shift; placement is a
//! measurement (the four C's, 07 §2), never a hash; idempotent by
//! (era, world) key; spawns read the converged chart.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use cfe::dop::ClosureEvent;
use cfe::world::wrap_gauge;
use cfe::persist;
use cfe::ratio::{ratio, Ratio};
use cfe::tlv;
use cfe::vrp::{receipt_digest, receipt_digest_v2, ReceiptHash};
use num_traits::Zero;
use physics::{AxisVector, Triple};

use crate::{card, sanitize, Ledger};

/// One archived receipt from `chronicle.tlv` —
/// raw event bytes carried whole so verification never needs to decode
/// the event (unknown tags still verify: SEAL A).
struct RawReceipt {
    sequence: u64,
    stamp: u64,
    parent: [u8; 32],
    digest: [u8; 32],
    by: Vec<u64>,
    event_bytes: Vec<u8>,
    /// `Some` for a `V2` ({S,G,T}) receipt, `None` for a `V1` — picks
    /// which digest formula `first_break` re-derives (TU-1a).
    manifold: Option<([u8; 32], Vec<u8>)>,
}

/// A world excluded from the timeline: left on disk, named here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quarantined {
    pub era: String,
    pub world: String,
    /// The first broken sequence, or `u64::MAX` for an unreadable file.
    pub at: u64,
}

/// The convergence read: what one boot adopted, skipped, or refused.
#[derive(Debug, Default)]
pub struct ConvergeRead {
    /// Worlds annexed THIS run: (era, world).
    pub adopted: Vec<(String, String)>,
    /// Worlds already annexed by a prior run (idempotence, 07 §1.5).
    pub already: usize,
    /// Broken chains, excluded and named (07 §1.2).
    pub quarantined: Vec<Quarantined>,
    /// Worlds whose final append was torn (07 §1.2): annexed at their
    /// verified prefix, the tear recorded here.
    pub torn: Vec<(String, String)>,
    /// The final epoch base: the sum of every era's terminal T. Live
    /// beats stack on top of this (TUniversal = epoch_base + beat).
    pub epoch_base: u64,
}

/// Discover era roots beside `var_root`: every counterpart directory named
/// `<base>-era*` (name-ascending — era1 before era2), then the current
/// root itself LAST (its prior-run worlds are the youngest era). Each
/// entry is (era name, worlds dir).
pub fn default_era_roots(var_root: &str) -> Vec<(String, PathBuf)> {
    let root = PathBuf::from(var_root);
    let base = root
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "var".into());
    let parent = match root.parent() {
        // A bare relative root ("var") has an EMPTY parent — that is
        // the current directory, not nowhere.
        Some(p) if p.as_os_str().is_empty() => PathBuf::from("."),
        Some(p) => p.to_path_buf(),
        None => PathBuf::from("."),
    };
    let mut eras: Vec<(String, PathBuf)> = Vec::new();
    if let Ok(entries) = fs::read_dir(&parent) {
        for entry in entries.filter_map(|e| e.ok()) {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with(&format!("{base}-era")) && entry.path().is_dir() {
                eras.push((name.clone(), entry.path().join("worlds")));
            }
        }
    }
    eras.sort();
    // THE LIVE ROOT NEVER ANNEXES ITSELF (naming ruling, 2026-07-30):
    // its worlds are already home and already charted by genesis —
    // self-annexation minted era-qualified ALIASES of living worlds
    // ("var--<world>" beside "<world>", tripling the board). The fold
    // (AEON-1b) is the one lawful door from live to archive; only the
    // archived `<base>-era<N>` roots converge.
    eras
}

/// Converge every archived era into the ledger's universe (07 §1):
/// verify → quarantine broken → epoch-shift T → place by the four C's →
/// append `EraAnnexed` to the ledger → chart + fiber cards → persist
/// the universe card at the final epoch base. Idempotent: worlds
/// already in the ledger are skipped.
pub fn converge(book: &Ledger, eras: &[(String, PathBuf)]) -> std::io::Result<ConvergeRead> {
    let ledger_path = book.root.join("convergence.tlv");
    let mut ledger = match ledger_path.exists() {
        true => persist::load(&ledger_path).map_err(io_error)?,
        false => cfe::vrp::Chronicle::new(),
    };
    let mut seen: HashSet<(String, String)> = ledger
        .receipts()
        .iter()
        .filter_map(|receipt| match receipt.event() {
            ClosureEvent::EraAnnexed { era, world, .. } => {
                Some((era.clone(), world.clone()))
            }
            _ => None,
        })
        .collect();

    let mut read = ConvergeRead::default();
    let mut offset: u64 = 0;
    for (era, worlds_dir) in eras {
        let mut era_terminal: u64 = 0;
        let mut world_dirs: Vec<PathBuf> = fs::read_dir(worlds_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok().map(|e| e.path()))
                    .filter(|p| p.is_dir() && p.join("chronicle.tlv").exists())
                    .collect()
            })
            .unwrap_or_default();
        world_dirs.sort();
        for dir in world_dirs {
            let world = dir
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            // Authority only: chronicle.tlv (SEAL B verified load).
            // Torn-tail is a JSONL concept; binary load is all-or-quarantine.
            let receipts = match cfe::persist::load(&dir.join("chronicle.tlv")) {
                Ok(ch) => ch
                    .receipts()
                    .iter()
                    .map(|r| RawReceipt {
                        sequence: r.sequence(),
                        stamp: r.stamp(),
                        parent: r.parent().0,
                        digest: r.own().0,
                        by: r.provenance().to_vec(),
                        event_bytes: r.event().wire_bytes(),
                        manifold: r
                            .region_seed()
                            .map(|region_seed| (region_seed, r.gauge().unwrap_or(&[]).to_vec())),
                    })
                    .collect::<Vec<_>>(),
                Err(_) => {
                    read.quarantined.push(Quarantined {
                        era: era.clone(),
                        world,
                        at: u64::MAX,
                    });
                    continue;
                }
            };
            // The verification door (07 §1.2) — the real chain law over
            // the RAW event bytes; unknown event tags still verify.
            if let Some(at) = first_break(&receipts) {
                read.quarantined.push(Quarantined {
                    era: era.clone(),
                    world,
                    at,
                });
                continue;
            }
            let terminal_t = receipts.iter().map(|r| r.stamp).max().unwrap_or(0);
            era_terminal = era_terminal.max(terminal_t);
            let terminal_digest = receipts
                .last()
                .map(|r| r.digest)
                .unwrap_or(ReceiptHash::GENESIS.0);
            let key = (era.clone(), world.clone());
            if seen.contains(&key) {
                read.already += 1;
                continue;
            }
            // The four C's (07 §2): fold, residual, terminal window.
            let events: Vec<ClosureEvent> = receipts
                .iter()
                .filter_map(|r| decode_event(&r.event_bytes))
                .collect();
            let fold = deposit(&events);
            let window_start = events.len().saturating_sub(TERMINAL_WINDOW);
            let window = deposit(&events[window_start..]);
            let residual = loop_residual(&events);
            // CONVERGENCE: placed where it was going — fold + one
            // window step of the flow (07 §2).
            let placed = Triple {
                spatial: fold.spatial.add(&window.spatial),
                temporal: fold.temporal.add(&window.temporal),
                gauge: fold.gauge.add(&window.gauge),
            };
            let (x, y, z, _) = project(&placed);
            // THE ONE G (the alignment seam, 2026-07-30): the chart's
            // G axis carries the world's KEY — the wrapped exact
            // rational the AEON law defines — never the four-C
            // deposit sum (that projection is an event-count reading;
            // it stays with placement and the fiber card, off this
            // axis). Before this fix the two dialects shared the axis
            // and the board read "0 or ~10": genesis keys beside
            // grant counts.
            let g = terminal_gauge(&events);
            // AEON-1b (11 §4): the FOUR-C ELIGIBILITY — a trajectory
            // settled in the retirement region annexes as a DERELICT
            // (in the world, on the chart, out of the living walk —
            // 07 §3's salvage frontier). The placeholder reading:
            // unrelieved famine strikes outweigh closures in the
            // residual (low coherence + carried want + null
            // convergence). Nothing is destroyed; the fiber card
            // names the state.
            let derelict = is_derelict(&residual);
            // THE SHEET SEATING (11 §5.2, wired 2026-07-30): each
            // annexed aeon seats on its OWN G-sheet — the era ordinal
            // fixes the base (era N lived on sheet (N−1)/12, the fold
            // law's placeholder advance), and the world's own key
            // rides above it. Without this offset every aeon stacks
            // at one G-height — the recurrence collision 11 §2.0
            // exists to prevent.
            let g = wrap_gauge(g + sheet_base(era));
            let chart_name = format!("{era}--{world}");
            book.chart_world(&chart_name, (&x, &y, &z), &g)?;
            write_fiber_card(&book.root, &chart_name, &residual, derelict)?;
            // TUniversal stamp: the world's terminal T on the one axis.
            ledger.append(
                ClosureEvent::EraAnnexed {
                    era: era.clone(),
                    world: world.clone(),
                    terminal: terminal_digest,
                    offset,
                },
                offset + terminal_t,
                vec![],
            );
            seen.insert(key);
            read.adopted.push((era.clone(), world.clone()));
        }
        // Eras stack end-to-end on the T axis (07 §1.3).
        offset += era_terminal;
    }
    read.epoch_base = offset;
    persist::sync(&ledger, &ledger_path).map_err(io_error)?;
    book.set_universal_clock(offset)?;
    Ok(read)
}

/// The terminal window W (07 §2, placeholder — tunes at balance time).
const TERMINAL_WINDOW: usize = 8;

/// The world's terminal KEY: the last gauge-carrying receipt's value
/// (GaugeShifted · Diverged · Confluent · EraFolded — the receipt
/// carries the wrapped value, the last stands; the replay law,
/// ledger-side). A chronicle with no key moves reads 0.
fn terminal_gauge(events: &[ClosureEvent]) -> Ratio {
    events
        .iter()
        .rev()
        .find_map(|event| match event {
            ClosureEvent::GaugeShifted { gauge }
            | ClosureEvent::Diverged { gauge, .. }
            | ClosureEvent::Confluent { gauge, .. }
            | ClosureEvent::EraFolded { gauge, .. }
            | ClosureEvent::GaugeShiftedAt { grade: 0, gauge } => Some(gauge.clone()),
            _ => None,
        })
        .unwrap_or_else(|| ratio(0, 1))
}

/// 11 §5.2: the G-sheet an archived era lived on — its ordinal read
/// from the `<base>-era<N>` name, seated at (N−1)/12 wrapped (the
/// fold law's placeholder advance). An unnumbered era reads sheet 0.
fn sheet_base(era: &str) -> Ratio {
    let ordinal = era
        .rsplit("-era")
        .next()
        .and_then(|n| n.parse::<i64>().ok())
        .unwrap_or(1);
    wrap_gauge(ratio(ordinal - 1, 12))
}

/// CONGRUENCE (07 §2): each decoded event deposits an exact-rational
/// shear onto the three axes. Unknown events deposited nothing before
/// this point (they decode to `None` and are filtered — SEAL A).
fn deposit(events: &[ClosureEvent]) -> Triple {
    let mut spatial = AxisVector::empty();
    let mut temporal = AxisVector::empty();
    let mut gauge = AxisVector::empty();
    for event in events {
        // The pace line: every receipt +1 @0.
        temporal = temporal.add(&at_grade(0, ratio(1, 1)));
        match event {
            ClosureEvent::YieldCollected { resource, amount, .. } => {
                spatial = spatial.add(&at_grade(*resource as usize - 1, amount.clone()));
            }
            ClosureEvent::UpkeepSettled { resource, amount, .. } => {
                spatial = spatial.add(&at_grade(*resource as usize - 1, -amount.clone()));
            }
            ClosureEvent::FamineStruck { resource, drawn, .. } => {
                spatial = spatial.add(&at_grade(*resource as usize - 1, -drawn.clone()));
            }
            ClosureEvent::EntityRegistered { .. } => {
                gauge = gauge.add(&at_grade(0, ratio(1, 1)));
            }
            ClosureEvent::ScopeGranted { .. } => {
                gauge = gauge.add(&at_grade(1, ratio(1, 1)));
            }
            ClosureEvent::ScopeRevoked { .. } => {
                gauge = gauge.add(&at_grade(1, ratio(-1, 1)));
            }
            ClosureEvent::ColonyFounded { .. } | ClosureEvent::ColonyCharted { .. } => {
                gauge = gauge.add(&at_grade(2, ratio(1, 1)));
            }
            _ => {}
        }
    }
    Triple {
        spatial,
        temporal,
        gauge,
    }
}

/// COHERENCE (07 §2): what loops fail to close — measured and KEPT.
fn loop_residual(events: &[ClosureEvent]) -> Triple {
    let mut strikes = ratio(0, 1);
    let mut builds = ratio(0, 1);
    let mut grants = ratio(0, 1);
    for event in events {
        match event {
            ClosureEvent::FamineStruck { .. } => strikes += ratio(1, 1),
            ClosureEvent::FamineRelieved { .. } => strikes -= ratio(1, 1),
            ClosureEvent::ZoneBuilt { .. } => builds += ratio(1, 1),
            ClosureEvent::ZoneRazed { .. } => builds -= ratio(1, 1),
            ClosureEvent::ScopeGranted { .. } => grants += ratio(1, 1),
            ClosureEvent::ScopeRevoked { .. } => grants -= ratio(1, 1),
            _ => {}
        }
    }
    Triple {
        spatial: AxisVector::from_vec(vec![strikes, builds]),
        temporal: AxisVector::empty(),
        gauge: AxisVector::from_vec(vec![grants]),
    }
}

/// The chart projection (07 §2, law): x = Σ spatial even grades,
/// y = Σ spatial odd grades, z = Σ temporal, g = Σ gauge. Wrapping mod
/// span happens in `chart_world` (the L1 torus).
fn project(triple: &Triple) -> (Ratio, Ratio, Ratio, Ratio) {
    let mut x = ratio(0, 1);
    let mut y = ratio(0, 1);
    for (grade, component) in triple.spatial.components().iter().enumerate() {
        match grade % 2 {
            0 => x += component,
            _ => y += component,
        }
    }
    let z = triple
        .temporal
        .components()
        .iter()
        .fold(ratio(0, 1), |sum, c| sum + c);
    let g = triple
        .gauge
        .components()
        .iter()
        .fold(ratio(0, 1), |sum, c| sum + c);
    (x, y, z, g)
}

fn at_grade(grade: usize, amount: Ratio) -> AxisVector {
    let mut components = vec![Ratio::zero(); grade + 1];
    components[grade] = amount;
    AxisVector::from_vec(components)
}

/// COHESION (07 §2): the residual rides the fiber — a card, a
/// reflection, never an authority.
/// AEON-1b (11 §4, placeholder reading): the retirement region — the
/// residual's strike component stands positive (unrelieved want) and
/// outweighs its build component (no growth answered it). Escapable
/// before the fold (relief, rescue trade); at annexation it is the
/// four-C verdict.
fn is_derelict(residual: &Triple) -> bool {
    let strikes = residual
        .spatial
        .components()
        .first()
        .cloned()
        .unwrap_or_else(|| ratio(0, 1));
    let builds = residual
        .spatial
        .components()
        .get(1)
        .cloned()
        .unwrap_or_else(|| ratio(0, 1));
    strikes > ratio(0, 1) && strikes > builds
}

fn write_fiber_card(
    root: &Path,
    chart_name: &str,
    residual: &Triple,
    derelict: bool,
) -> std::io::Result<()> {
    let dir = root.join("fiber");
    fs::create_dir_all(&dir)?;
    let render = |axis: &AxisVector| -> String {
        axis.components()
            .iter()
            .map(|c| format!("{c}"))
            .collect::<Vec<_>>()
            .join(" ")
    };
    let mut out = Vec::new();
    card::put_str(card::fiber::WORLD, chart_name, &mut out);
    if derelict {
        card::put_str(card::fiber::STATE, "derelict", &mut out);
    }
    card::put_str(card::fiber::SPATIAL, &render(&residual.spatial), &mut out);
    card::put_str(card::fiber::GAUGE, &render(&residual.gauge), &mut out);
    card::write_atomic(&dir.join(format!("{}.fiber", sanitize(chart_name))), &out)
}

/// The real chain law (NS-3 §2.1) over raw event bytes: parent links
/// and recomputed digests, first break named.
fn first_break(receipts: &[RawReceipt]) -> Option<u64> {
    let mut parent = ReceiptHash::GENESIS;
    for receipt in receipts {
        let expected = match &receipt.manifold {
            None => receipt_digest(
                receipt.sequence,
                receipt.stamp,
                &parent,
                &receipt.event_bytes,
                &receipt.by,
            ),
            Some((region_seed, gauge)) => receipt_digest_v2(
                receipt.sequence,
                receipt.stamp,
                &parent,
                &receipt.event_bytes,
                &receipt.by,
                region_seed,
                gauge,
            ),
        };
        if receipt.parent != parent.0 || receipt.digest != expected.0 {
            return Some(receipt.sequence);
        }
        parent = ReceiptHash(receipt.digest);
    }
    None
}

fn decode_event(event_bytes: &[u8]) -> Option<ClosureEvent> {
    let (tag, value) = tlv::records(event_bytes).next()?;
    ClosureEvent::from_wire(tag, value)
}

fn io_error(refused: persist::PersistRefused) -> std::io::Error {
    std::io::Error::other(format!("{refused:?}"))
}
