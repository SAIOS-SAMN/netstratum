//! THE RE-FOUNDING AND THE MAP (docs/67 "After the fleet's recut", docs/72 A4).
//!
//! After the fleet's recut no domain carries over (R5): netstratum's house
//! re-posts its standing, in order, with the SAME keys (a domain's id is its
//! founding verifying key, so `var/uplink/*.key` kept intact keeps 5deab117):
//!
//! 1. the domain (`found`), its tiers named at genesis (78.119/78.120);
//! 2. its origin (`originate`): the Big Bang's seed and the flat L1 torus, which
//!    also states the universe's geometry (78.140) a local throat needs;
//! 3. its reserve (`reserve union`, held for stillcold, docs/68);
//! 4. its first LEDGER entry (78.154 sub-kind 12) at a final point: the
//!    commitment over its checkpoints (Concordance 1.13), no events yet;
//! 5. then, each tend, its gates as LOCAL THROATS (MAP record kind 2), signed
//!    by the reserve, citing a final point: a gate is crossed only once mapped
//!    (Concordance 5.9).
//!
//! The house runs it ITSELF (`House::tend`): when the court refuses its acts as
//! an unknown domain (the recut), it re-founds once for the new origin epoch.
//! `ns-hypha refound` runs the same steps by hand.
//!
//! The house and the conductor meet only through files beside the worlds: the
//! conductor writes the gates to map at their hubs' places (`gates-map.bin`),
//! the house writes back each gate it sealed with the place it sealed it at
//! (`gates-mapped.bin`); a gate whose hub drifts past its freshness is offered
//! again at its new place and mapped again. Keys are LOADED, never minted: a
//! missing key stops the re-founding (a new key is a new identity). The house
//! maps only after the re-founding has run for the present origin (`refounded`
//! in the uplink home): the coordinator ruled the map waits for the recut.
use mycelium::anastome::dialogue::atlas::{decode_map_reply, encode_map, MapAct, MapRecord, MapRefusal, Mapped};
use mycelium::anastome::dialogue::domain::{DomainGenesis, Founded, Origin, Reservation};
use mycelium::anastome::dialogue::seat::Arg;
use mycelium::anastome::dialogue::universe::{Genesis, Geometry, Parent};
use mycelium::chronometry::tuniversal::TPoint;
use mycelium::sdk::Session;
use std::io;
use std::path::Path;

/// A key loaded from the uplink home, never minted here.
pub fn load_key(home: &Path, file: &str) -> io::Result<ed25519_dalek::SigningKey> {
    let bytes = std::fs::read(home.join(file))?;
    let seed: [u8; 32] = bytes
        .get(..32)
        .and_then(|b| b.try_into().ok())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, format!("{file} is not a 32-byte seed")))?;
    Ok(ed25519_dalek::SigningKey::from_bytes(&seed))
}

fn nonce() -> io::Result<[u8; 32]> {
    use std::io::Read;
    let mut n = [0u8; 32];
    std::fs::File::open("/dev/urandom")?.read_exact(&mut n)?;
    Ok(n)
}

/// The checkpoints the first ledger entry commits to: each binary checkpoint
/// beside the worlds, keyed by its coordinate path, valued by its content hash.
fn checkpoint_records(var_root: &Path) -> Vec<(Vec<u8>, Vec<Arg>)> {
    let mut out = Vec::new();
    let mut files: Vec<std::path::PathBuf> = ["ancients.bin", "trust.bin", "council.bin", "commissions.bin", "ascension.bin"]
        .iter()
        .map(|f| var_root.join(f))
        .collect();
    if let Ok(entries) = std::fs::read_dir(var_root.join("substrata")) {
        let mut subs: Vec<_> = entries.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "tube" || x == "bin")).collect();
        subs.sort();
        files.extend(subs);
    }
    for path in files {
        let Ok(bytes) = std::fs::read(&path) else { continue };
        let key = format!("netstratum/{}", path.strip_prefix(var_root).unwrap_or(&path).display());
        out.push((key.into_bytes(), vec![Arg::Key(*blake3::hash(&bytes).as_bytes())]));
    }
    out
}

/// 4. THE FIRST LEDGER ENTRY (78.154 sub-kind 12): at the world's axis
/// coordinate `u`, committing to the checkpoints, no events, parent zero for
/// the first (the head after it is kept in `home/ledger-head`).
/// The ledger entry's frame, built and signed by `reserve` (no session): the
/// commitment over the checkpoints, no events, the given parent and point.
pub fn ledger_frame(
    reserve: &ed25519_dalek::SigningKey,
    domain: [u8; 32],
    var_root: &Path,
    u: u64,
    point: mycelium::chronometry::tuniversal::TPoint,
    parent: [u8; 32],
) -> io::Result<(Vec<u8>, usize)> {
    use mycelium::anastome::dialogue::atlas::{encode_ledger, LedgerEntry};
    use mycelium::anastome::dialogue::commitment::commitment;
    let records = checkpoint_records(var_root);
    let commitment = commitment(&records).ok_or_else(|| io::Error::other("a checkpoint key repeats"))?;
    let entry = LedgerEntry { domain, u, commitment, events: [0; 32], point, parent, applied: Default::default(), nonce: nonce()? };
    Ok((encode_ledger(&entry.sign(reserve)), records.len()))
}

pub fn post_ledger(session: &Session, home: &Path, var_root: &Path, u: u64) -> io::Result<String> {
    use mycelium::anastome::dialogue::atlas::{decode_ledger_reply, Ledgered};
    let reserve = load_key(home, "tier-union.key")?;
    let domain = load_key(home, "tier-domain.key")?.verifying_key().to_bytes();
    let point = session.tuniversal().ok_or_else(|| io::Error::other("no final TUniversal point read yet"))?;
    let parent: [u8; 32] = std::fs::read(home.join("ledger-head")).ok().and_then(|b| b.get(..32).and_then(|s| s.try_into().ok())).unwrap_or([0; 32]);
    let (frame, committed) = ledger_frame(&reserve, domain, var_root, u, point, parent)?;
    let records_len = committed;
    let reply = session.speak(frame)?;
    match decode_ledger_reply(&reply) {
        Some(Ledgered::Sealed { entry, h, .. }) => {
            std::fs::write(home.join("ledger-head"), entry)?;
            Ok(format!("LEDGER sealed at h {h}: u {u}, {records_len} checkpoints committed"))
        }
        Some(Ledgered::Refused(why)) => Ok(format!("LEDGER refused: {why:?}")),
        None => Err(io::Error::other("the ledger reply did not decode")),
    }
}

/// The origin a final point was read under: its genesis and its epoch. A
/// recut draws a new one.
pub fn origin_of(point: &TPoint) -> String {
    let g: String = point.origin.genesis[..8].iter().map(|b| format!("{b:02x}")).collect();
    format!("{g}/{}", point.origin.epoch)
}

/// The origin the re-founding last sealed under (`home/refounded`: `u origin`).
pub fn refounded_under(home: &Path) -> Option<String> {
    std::fs::read_to_string(home.join("refounded")).ok()?.split_whitespace().nth(1).map(str::to_string)
}

/// THE RE-FOUNDING, steps 1 to 4 and the marker, with the keys already in the
/// uplink home (never minted): found, originate (the flat L1 torus, periods
/// 2 Mpc), reserve the union tier, the first ledger entry at the world's u
/// with a zero parent; when the entry seals, `refounded` names the origin it
/// sealed under. A step the court refuses stops the rest. Returns whether it
/// sealed, and what the court said at each step.
pub fn refound(session: &Session, home: &Path, var_root: &Path) -> io::Result<(bool, Vec<String>)> {
    let domain = load_key(home, "tier-domain.key")?;
    let union = load_key(home, "tier-union.key")?;
    let colonies = load_key(home, "tier-colonies.key")?;
    let id = domain.verifying_key().to_bytes();
    let mut said = Vec::new();
    // 1. The domain, its tiers named at genesis (ascending, each once).
    let mut tiers = vec![colonies.verifying_key().to_bytes(), union.verifying_key().to_bytes()];
    tiers.sort_unstable();
    tiers.dedup();
    let founded = session.found_domain(&DomainGenesis { domain: id, tiers }.sign(&domain))?;
    said.push(format!("1 domain: {founded:?}"));
    if !matches!(founded, Founded::Sealed { .. } | Founded::AlreadyFounded) {
        return Ok((false, said));
    }
    // Every door reply carries the door's final point: the domain's answer
    // has brought one, read now (a fresh session has heard none before it).
    let Some(point) = session.tuniversal() else {
        said.push("the re-founding waits: no final point read yet".into());
        return Ok((false, said));
    };
    // 2. The origin: the Big Bang's seed and era 1's closed sphere, its radius
    // the bell's reach (`fold::universe_radius`).
    let ledger = cfe::persist::load(&var_root.join("worlds/convergence.tlv")).map_err(|e| io::Error::other(format!("the universe ledger did not read: {e:?}")))?;
    let seed = ledger
        .receipts()
        .iter()
        .find(|r| matches!(r.event(), cfe::dop::ClosureEvent::UniverseFounded { .. }))
        .map(|r| r.own().0)
        .ok_or_else(|| io::Error::other("no Big Bang on the universe ledger"))?;
    let origin = Origin {
        domain: id,
        kind: crate::fold::ORIGIN_KIND,
        version: 2,
        commitment: seed,
        universe: Some(Genesis { geometry: Geometry::Sphere(crate::fold::universe_radius(var_root)?), parent: Parent::Unwitnessed }),
    };
    let originated = session.originate(&origin.sign(&domain))?;
    said.push(format!("2 origin: {originated:?}"));
    if !matches!(originated, Founded::Originated { .. } | Founded::AlreadyOriginated) {
        return Ok((false, said));
    }
    // 3. The reserve: the union tier, held for stillcold (docs/68).
    let reserved = session.reserve(&Reservation { domain: id, tier: union.verifying_key().to_bytes(), nonce: nonce()? }.sign(&domain))?;
    said.push(format!("3 reserve: {reserved:?}"));
    if !matches!(reserved, Founded::Reserved { .. }) {
        return Ok((false, said));
    }
    // 4. The first ledger entry: its parent is zero, so a head kept from
    // before the recut is set aside.
    let head = home.join("ledger-head");
    if head.exists() {
        std::fs::rename(&head, home.join(format!("ledger-head.before-{}", origin_of(&point).replace('/', "-"))))?;
    }
    let u: u64 = std::fs::read_to_string(var_root.join("conductor.beat")).ok().and_then(|t| t.trim().parse().ok()).unwrap_or(0);
    let line = post_ledger(session, home, var_root, u)?;
    said.push(format!("4 {line}"));
    if !line.starts_with("LEDGER sealed") {
        return Ok((false, said));
    }
    // 5. The marker: the gates are mapped from the next tend.
    std::fs::write(home.join("refounded"), format!("{u} {}\n", origin_of(&point)))?;
    said.push(format!("5 re-founded under {}; the house maps the gates", origin_of(&point)));
    Ok((true, said))
}

/// One gate to map, as the conductor writes it.
pub struct GateEnd {
    /// The gate's name: its galaxy and its core (`from>core`).
    pub id: String,
    pub core: String,
    pub keeper: String,
    /// The galaxy hub's place, the near end.
    pub from: [cfe::ratio::Ratio; 3],
    /// The core's centre, the far end.
    pub to: [cfe::ratio::Ratio; 3],
    /// The cells the hub may drift before the gate is offered again.
    pub freshness: u64,
}

fn text(value: &[u8], at: &mut usize) -> Option<String> {
    let len = u16::from_le_bytes(value.get(*at..*at + 2)?.try_into().ok()?) as usize;
    let s = String::from_utf8(value.get(*at + 2..*at + 2 + len)?.to_vec()).ok()?;
    *at += 2 + len;
    Some(s)
}

fn coords<const N: usize>(value: &[u8], at: &mut usize) -> Option<[cfe::ratio::Ratio; N]> {
    let mut out = Vec::with_capacity(N);
    for _ in 0..N {
        let (r, used) = value.get(*at..).and_then(cfe::ratio::decode)?;
        *at += used;
        out.push(r);
    }
    out.try_into().ok()
}

/// The gates offered (`gates-map.bin`, tag 1: id, core, keeper's world, the
/// hub's place and the core's centre as six ratios, freshness u64).
pub fn read_gate_ends(var_root: &Path) -> Vec<GateEnd> {
    let bytes = std::fs::read(var_root.join("gates-map.bin")).unwrap_or_default();
    cfe::tlv::records(&bytes)
        .filter(|(tag, _)| *tag == 1)
        .filter_map(|(_, value)| {
            let mut at = 0;
            let (id, core, keeper) = (text(value, &mut at)?, text(value, &mut at)?, text(value, &mut at)?);
            let from = coords::<3>(value, &mut at)?;
            let to = coords::<3>(value, &mut at)?;
            let freshness = u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?);
            Some(GateEnd { id, core, keeper, from, to, freshness })
        })
        .collect()
}

/// The gates sealed (`gates-mapped.bin`, tag 2: id, the place sealed at).
fn read_sealed(var_root: &Path) -> Vec<(String, [cfe::ratio::Ratio; 3])> {
    let bytes = std::fs::read(var_root.join("gates-mapped.bin")).unwrap_or_default();
    cfe::tlv::records(&bytes)
        .filter(|(tag, _)| *tag == 2)
        .filter_map(|(_, value)| {
            let mut at = 0;
            Some((text(value, &mut at)?, coords::<3>(value, &mut at)?))
        })
        .collect()
}

/// A gate's LOCAL THROAT map act (MAP record kind 2), built and signed by
/// `reserve` (no session): the throat named by the gate, kept by the line on
/// the keeper's world, its two frames the translators to the hub's place and
/// to the core's centre, fresh for the gate's freshness.
pub fn map_frame(
    reserve: &ed25519_dalek::SigningKey,
    domain: [u8; 32],
    point: TPoint,
    end: &GateEnd,
) -> io::Result<Vec<u8>> {
    let frame = |c: &[cfe::ratio::Ratio; 3]| mycelium::transducer::cga::translator_at(&[c[0].clone(), c[1].clone(), c[2].clone()]);
    let record = MapRecord::LocalThroat {
        throat: *blake3::hash(format!("netstratum gate {}", end.id).as_bytes()).as_bytes(),
        keeper: crate::house::line_key(&end.keeper),
        frames: (frame(&end.from), frame(&end.to)),
        freshness: end.freshness,
    };
    Ok(encode_map(&MapAct { domain, point, record, nonce: nonce()? }.sign(reserve)))
}

/// 5. THE GATES AS LOCAL THROATS (MAP record kind 2), each signed by the
/// reserve and citing a final point: every gate offered at a place it is not
/// sealed at is mapped there, and the seal written back with that place for
/// the conductor (`gates-mapped.bin`). Only after the re-founding under the
/// present origin. Returns what happened, and whether the court refused an act
/// as an unknown domain (the recut).
pub fn map_gates(session: &Session, home: &Path, var_root: &Path) -> io::Result<(Vec<String>, bool)> {
    let Some(point) = session.tuniversal() else { return Ok((Vec::new(), false)) };
    if refounded_under(home).as_deref() != Some(origin_of(&point).as_str()) {
        return Ok((Vec::new(), false));
    }
    let ends = read_gate_ends(var_root);
    let mut sealed = read_sealed(var_root);
    let due: Vec<&GateEnd> = ends.iter().filter(|e| !sealed.iter().any(|(id, at)| *id == e.id && *at == e.from)).collect();
    if due.is_empty() {
        return Ok((Vec::new(), false));
    }
    let reserve = load_key(home, "tier-union.key")?;
    let domain = load_key(home, "tier-domain.key")?.verifying_key().to_bytes();
    let mut said = Vec::new();
    let mut unknown = false;
    for end in due {
        match decode_map_reply(&session.speak(map_frame(&reserve, domain, point, end)?)?) {
            Some(Mapped::Sealed { h, .. }) => {
                sealed.retain(|(id, _)| *id != end.id);
                sealed.push((end.id.clone(), end.from.clone()));
                said.push(format!("GATE {} MAPPED at h {h} (keeper {})", end.id, end.keeper));
            }
            Some(Mapped::Refused(why)) => {
                unknown |= matches!(why, MapRefusal::UnknownDomain);
                said.push(format!("GATE {} not mapped: {why:?}", end.id));
            }
            None => said.push(format!("GATE {}: the reply did not decode", end.id)),
        }
    }
    let mut out = Vec::new();
    for (id, at) in &sealed {
        let mut v = Vec::new();
        v.extend_from_slice(&(id.len() as u16).to_le_bytes());
        v.extend_from_slice(id.as_bytes());
        for c in at {
            cfe::ratio::encode(c, &mut v);
        }
        cfe::tlv::put(2, &v, &mut out);
    }
    let mapped_path = var_root.join("gates-mapped.bin");
    let staged = mapped_path.with_extension("staged");
    std::fs::write(&staged, out)?;
    std::fs::rename(staged, mapped_path)?;
    Ok((said, unknown))
}
