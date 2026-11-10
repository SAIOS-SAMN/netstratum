//! The RE-founding and the map (docs/67 "After the fleet's recut", docs/72 A4).
//!
//! After the fleet's recut no domain carries over (R5): Netstratum's house
//! re-posts its standing, in order, with the same keys (a domain's id is its
//! founding verifying key, so `var/uplink/*.key` kept intact keeps 5deab117):
//!
//! 0. the contract (Mycelium's KERNEL profile): the statement the hypha
//!    serves and its Concordance's hash, each read against the pinned SDK's,
//!    and the domain's signature over the hash; a hypha on another contract
//!    stops the re-founding before anything is posted;
//! 1. the domain (`found`), its tiers named at genesis (78.119/78.120);
//! 2. its origin (`originate`): the Big Bang's seed and era 1's closed
//!    sphere, which also states the universe's geometry (78.140) a local
//!    throat needs; then the universe read back as originated;
//! 3. its reserve (`reserve union`, held for stillcold, docs/68); then the
//!    universe's manifest, its nuclei per Z, signed by that reserve: granted,
//!    or refused by the court's standing by name;
//! 4. its first ledger entry (78.154 sub-kind 12) at a final point: the
//!    commitment over its checkpoints (Concordance 1.13), no events yet;
//! 5. then, each tend, its gates as LOCAL throats (map record kind 2), signed
//!    by the reserve, citing a final point: a gate is crossed only once mapped
//!    (Concordance 5.9).
//!
//! The house runs it itself (`House::tend`): when the court refuses its acts as
//! an unknown domain (the recut), it re-founds once for the new origin epoch.
//! `ns-hypha refound` runs the same steps by hand.
//!
//! The house and the conductor meet only through files beside the worlds: the
//! conductor writes the gates to map at their hubs' places (`gates-map.bin`),
//! the house writes back each gate it sealed with the place it sealed it at
//! (`gates-mapped.bin`); a gate whose hub drifts past its freshness is offered
//! again at its new place and mapped again. Keys are loaded, never minted: a
//! missing key stops the re-founding (a new key is a new identity). The house
//! maps only after the re-founding has run for the present origin (`refounded`
//! in the uplink home): the coordinator ruled the map waits for the recut.
use crate::wire::HouseActs;
use mycelium_sdk::house::atlas::{
    decode_map_reply, encode_map, MapAct, MapRecord, MapRefusal, Mapped,
};
use mycelium_sdk::house::domain::{BigBangRecord, DomainGenesis, Founded, Reservation};
use mycelium_sdk::house::seat::Arg;
use mycelium_sdk::house::tuniversal::TPoint;
use mycelium_sdk::house::universe::{Genesis, Geometry, Parent};
use mycelium_sdk::house::{
    decode_grant_reply, decode_universe_reading, encode_manifest, encode_universe_read,
    GrantRefusal, Granted, Manifest,
};
use mycelium_sdk::session::Session;
use std::io;
use std::path::Path;

/// A key loaded from the uplink home, never minted here.
pub fn load_key(home: &Path, file: &str) -> io::Result<ed25519_dalek::SigningKey> {
    let bytes = std::fs::read(home.join(file))?;
    let seed: [u8; 32] = bytes
        .get(..32)
        .and_then(|b| b.try_into().ok())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{file} is not a 32-byte seed"),
            )
        })?;
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
    let mut files: Vec<std::path::PathBuf> = [
        "ancients.bin",
        "trust.bin",
        "council.bin",
        "commissions.bin",
        "ascension.bin",
    ]
    .iter()
    .map(|f| var_root.join(f))
    .collect();
    if let Ok(entries) = std::fs::read_dir(var_root.join("substrata")) {
        let mut subs: Vec<_> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "tube" || x == "bin"))
            .collect();
        subs.sort();
        files.extend(subs);
    }
    for path in files {
        let Ok(bytes) = std::fs::read(&path) else {
            continue;
        };
        let key = format!(
            "netstratum/{}",
            path.strip_prefix(var_root).unwrap_or(&path).display()
        );
        out.push((
            key.into_bytes(),
            vec![Arg::Key(*blake3::hash(&bytes).as_bytes())],
        ));
    }
    out
}

/// 4. The first ledger entry (78.154 sub-kind 12): at the world's axis
///    coordinate `u`, committing to the checkpoints, no events, parent zero for
///    the first (the head after it is kept in `home/ledger-head`).
///    The ledger entry's frame, built and signed by `reserve` (no session): the
///    commitment over the checkpoints, no events, the given parent and point.
pub fn ledger_frame(
    reserve: &ed25519_dalek::SigningKey,
    domain: [u8; 32],
    var_root: &Path,
    u: u64,
    point: mycelium_sdk::house::tuniversal::TPoint,
    parent: [u8; 32],
) -> io::Result<(Vec<u8>, usize)> {
    use mycelium_sdk::house::atlas::{encode_ledger, LedgerEntry};
    use mycelium_sdk::house::commitment::commitment;
    let records = checkpoint_records(var_root);
    let commitment =
        commitment(&records).ok_or_else(|| io::Error::other("a checkpoint key repeats"))?;
    let entry = LedgerEntry {
        domain,
        u,
        commitment,
        events: [0; 32],
        point,
        parent,
        applied: Default::default(),
        nonce: nonce()?,
    };
    Ok((encode_ledger(&entry.sign(reserve)), records.len()))
}

pub fn post_ledger(session: &Session, home: &Path, var_root: &Path, u: u64) -> io::Result<String> {
    use mycelium_sdk::house::atlas::{decode_ledger_reply, Ledgered};
    let reserve = load_key(home, "tier-union.key")?;
    let domain = load_key(home, "tier-domain.key")?
        .verifying_key()
        .to_bytes();
    let point = session
        .tuniversal()
        .ok_or_else(|| io::Error::other("no final TUniversal point read yet"))?;
    let parent: [u8; 32] = std::fs::read(home.join("ledger-head"))
        .ok()
        .and_then(|b| b.get(..32).and_then(|s| s.try_into().ok()))
        .unwrap_or([0; 32]);
    let (frame, committed) = ledger_frame(&reserve, domain, var_root, u, point, parent)?;
    let records_len = committed;
    let reply = session.speak(frame)?;
    match decode_ledger_reply(&reply) {
        Some(Ledgered::Sealed { entry, h, .. }) => {
            std::fs::write(home.join("ledger-head"), entry)?;
            Ok(format!(
                "LEDGER sealed at h {h}: u {u}, {records_len} checkpoints committed"
            ))
        }
        Some(Ledgered::Refused(why)) => Ok(format!("LEDGER refused: {why:?}")),
        // Mycelium v0.4.0: refused at the court's boundary (the act not
        // through its core, not its signatory), named.
        Some(Ledgered::Boundary(why)) => Ok(format!("LEDGER refused at the boundary: {why:?}")),
        None => Err(io::Error::other("the ledger reply did not decode")),
    }
}

/// The origin a final point was read under: its genesis and its epoch. A
/// recut draws a new one.
pub fn origin_of(point: &TPoint) -> String {
    let g: String = point.origin.genesis[..8]
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    format!("{g}/{}", point.origin.epoch)
}

/// The origin the re-founding last sealed under (`home/refounded`: `u origin`).
pub fn refounded_under(home: &Path) -> Option<String> {
    std::fs::read_to_string(home.join("refounded"))
        .ok()?
        .split_whitespace()
        .nth(1)
        .map(str::to_string)
}

/// The RE-founding, steps 0 to 4 and the marker, with the keys already in the
/// uplink home (never minted): found, originate (the flat L1 torus, periods
/// 64 cells), reserve the union tier, the first ledger entry at the world's u
/// with a zero parent; when the entry seals, `refounded` names the origin it
/// sealed under. A step the court refuses stops the rest. Returns whether it
/// sealed, and what the court said at each step.
pub fn refound(session: &Session, home: &Path, var_root: &Path) -> io::Result<(bool, Vec<String>)> {
    let domain = load_key(home, "tier-domain.key")?;
    let union = load_key(home, "tier-union.key")?;
    let colonies = load_key(home, "tier-colonies.key")?;
    let id = domain.verifying_key().to_bytes();
    // 0. The contract.
    let (agreed, mut said) = concord(session, &[&domain, &union, &colonies])?;
    if !agreed {
        return Ok((false, said));
    }
    // 1. The domain, its tiers named at genesis (ascending, each once).
    let mut tiers = vec![
        colonies.verifying_key().to_bytes(),
        union.verifying_key().to_bytes(),
    ];
    tiers.sort_unstable();
    tiers.dedup();
    let founded = session.found_domain(&DomainGenesis { domain: id, tiers }.sign(&domain))?;
    said.push(format!("1 domain: {founded:?}"));
    if !matches!(founded, Founded::Sealed { .. } | Founded::AlreadyFounded) {
        return Ok((false, said));
    }
    // A pointed door reply carries the door's final point: if the domain's
    // answer brought none, read the court's clock, whose reply does.
    if session.tuniversal().is_none() {
        let _ = session.speak(mycelium_sdk::house::encode_clock_request());
    }
    let Some(point) = session.tuniversal() else {
        said.push("the re-founding waits: no final point read yet".into());
        return Ok((false, said));
    };
    // 2. The origin: the Big Bang's seed and era 1's closed sphere, its radius
    // the bell's reach (`fold::universe_radius`).
    let ledger = cfe::persist::load(&var_root.join("worlds/convergence.tlv"))
        .map_err(|e| io::Error::other(format!("the universe ledger did not read: {e:?}")))?;
    let seed = ledger
        .receipts()
        .iter()
        .find(|r| matches!(r.event(), cfe::dop::ClosureEvent::UniverseFounded { .. }))
        .map(|r| r.own().0)
        .ok_or_else(|| io::Error::other("no Big Bang on the universe ledger"))?;
    let genesis = Genesis {
        geometry: Geometry::Sphere(crate::fold::universe_radius(var_root)?),
        parent: Parent::Unwitnessed,
    };
    let origin = BigBangRecord {
        domain: id,
        kind: crate::fold::ORIGIN_KIND,
        version: 2,
        commitment: seed,
        universe: Some(genesis.clone()),
    };
    let originated = session.originate(&origin.sign(&domain))?;
    said.push(format!("2 origin: {originated:?}"));
    if !matches!(
        originated,
        Founded::Originated { .. } | Founded::AlreadyOriginated
    ) {
        return Ok((false, said));
    }
    // The universe read back: originated, in the geometry and parent face the
    // origin stated.
    let read = decode_universe_reading(&session.speak(encode_universe_read(id))?);
    let stands = read.as_ref().is_some_and(|r| {
        r.originated
            && r.geometry.as_ref() == Some(&genesis.geometry)
            && r.parent.as_ref() == Some(&genesis.parent)
    });
    said.push(format!(
        "2 universe: {}",
        if stands {
            "originated as stated"
        } else {
            "not the universe originated"
        }
    ));
    if !stands {
        return Ok((false, said));
    }
    // 3. The reserve: the union tier, held for stillcold (docs/68). A domain
    // designates once (Mycelium v0.4.0): where the court already reads the
    // union as the sitting reserve, it stands and is not asked again.
    let tier = union.verifying_key().to_bytes();
    let sitting = session.reserve_reading(id)?;
    if sitting.current == Some(tier) {
        said.push("3 reserve: the union sits already".into());
    } else {
        let reserved = session.reserve(
            &Reservation {
                domain: id,
                tier,
                nonce: nonce()?,
            }
            .sign(&domain),
        )?;
        said.push(format!("3 reserve: {reserved:?}"));
        if !matches!(reserved, Founded::Reserved { .. }) {
            return Ok((false, said));
        }
    }
    // The manifest: the universe's nuclei per Z, signed by the sitting
    // reserve. Granted, or refused by the court's standing (short, granted
    // already, one in flight, no new closures), nothing moved; any other
    // refusal names a fault on this side and stops the rest.
    let granted = post_manifest(session, &union, id, var_root)?;
    said.push(format!("3 manifest: {granted:?}"));
    if !matches!(
        granted,
        Granted::Sealed { .. }
            | Granted::Refused(
                GrantRefusal::Short
                    | GrantRefusal::Granted
                    | GrantRefusal::InFlight
                    | GrantRefusal::NoNewClosures
            )
    ) {
        return Ok((false, said));
    }
    // 4. The first ledger entry: its parent is zero, so a head kept from
    // before the recut is set aside.
    let head = home.join("ledger-head");
    if head.exists() {
        std::fs::rename(
            &head,
            home.join(format!(
                "ledger-head.before-{}",
                origin_of(&point).replace('/', "-")
            )),
        )?;
    }
    let u: u64 = std::fs::read_to_string(var_root.join("conductor.beat"))
        .ok()
        .and_then(|t| t.trim().parse().ok())
        .unwrap_or(0);
    let line = post_ledger(session, home, var_root, u)?;
    said.push(format!("4 {line}"));
    if !line.starts_with("LEDGER sealed") {
        return Ok((false, said));
    }
    // 5. The marker: the gates are mapped from the next tend.
    std::fs::write(
        home.join("refounded"),
        format!("{u} {}\n", origin_of(&point)),
    )?;
    said.push(format!(
        "5 re-founded under {}; the house maps the gates",
        origin_of(&point)
    ));
    Ok((true, said))
}

/// THE CONTRACT, the re-founding's step 0 (Mycelium's KERNEL profile): the
/// statement the hypha serves against the pinned SDK's, then the clause
/// register it serves (ACT_LAWS mode 7) signed by reconstruction (mode 8) by
/// every key the domain acts with (the court admits an act only from a
/// signatory of the register, Mycelium v0.4.0 `Boundary::NotSignatory`):
/// each clause enabled, carrying this kernel's reconstruction of each of its
/// fixtures (`sign_concordance`), the verdict read back, and each key's
/// alignment read (mode 9). Agreed only where the statement is the pinned
/// one and every served clause is admitted for every key; what each read
/// said.
pub fn concord(
    session: &Session,
    keys: &[&ed25519_dalek::SigningKey],
) -> io::Result<(bool, Vec<String>)> {
    use mycelium_sdk::conformance::{
        decode_statement_reply, encode_statement_request, StatementRead, STATEMENT,
    };
    use mycelium_sdk::house::{
        decode_alignment_reply, decode_concordance_reply, decode_signature_reply,
        encode_alignment_request, encode_concordance_request, encode_signature_request,
        sign_concordance, Assent,
    };
    let mut said = Vec::new();
    let stated = decode_statement_reply(&session.speak(encode_statement_request())?);
    if stated != Some(StatementRead::of(&STATEMENT)) {
        said.push("0 statement: not the pinned contract".into());
        return Ok((false, said));
    }
    said.push("0 statement: the pinned contract".into());
    let served = match decode_concordance_reply(&session.speak(encode_concordance_request())?) {
        Some(Ok(served)) => served,
        Some(Err(r)) => {
            said.push(format!(
                "0 concordance: the hypha cannot read clause {:?} of its register",
                r.id
            ));
            return Ok((false, said));
        }
        None => {
            said.push("0 concordance: no clause register".into());
            return Ok((false, said));
        }
    };
    said.push(format!("0 concordance: {} clauses served", served.len()));
    let mut all = true;
    for key in keys {
        let kernel = key.verifying_key().to_bytes();
        let named: String = kernel.iter().take(4).map(|b| format!("{b:02x}")).collect();
        let Some(signed) = sign_concordance(key, &served, |_| Assent::Enabled) else {
            said.push("0 signature: a served fixture does not reconstruct here".into());
            return Ok((false, said));
        };
        let judged = decode_signature_reply(&session.speak(encode_signature_request(&signed))?);
        all &= matches!(&judged, Some(Ok(a)) if a.len() == served.len());
        said.push(format!("0 signature {named}: {judged:?}"));
        // The key's alignment read back from the hypha's ledger (mode 9):
        // each clause as this key's signing left it.
        match decode_alignment_reply(&session.speak(encode_alignment_request(Some(&kernel)))?) {
            Some(Ok(read)) => {
                let ours = read.kernels.iter().find(|k| k.kernel == kernel);
                said.push(format!(
                    "0 alignment {named}: {}",
                    ours.map_or("unread".to_string(), |k| format!(
                        "{} clauses read, {:?}",
                        k.clauses.len(),
                        k.clauses
                    ))
                ));
            }
            Some(Err(why)) => said.push(format!("0 alignment {named}: refused {why:?}")),
            None => said.push(format!("0 alignment {named}: no reading")),
        }
    }
    Ok((all, said))
}

/// THE MANIFEST: the universe's nuclei per Z (`conductor::pulse::nuclei_by_z`
/// over what it holds), signed by the sitting reserve, and the court's grant
/// or its refusal by name.
pub fn post_manifest(
    session: &Session,
    reserve: &ed25519_dalek::SigningKey,
    domain: [u8; 32],
    var_root: &Path,
) -> io::Result<Granted> {
    let (_, u) = conductor::pulse::read(var_root)
        .ok_or_else(|| io::Error::other("the universe did not read (pulse.bin)"))?;
    let nuclei = conductor::pulse::nuclei_by_z(&u.total())
        .into_iter()
        .map(|(z, c)| Some((u16::try_from(z).ok()?, c.to_biguint()?)))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| io::Error::other("a Z or a count passes the manifest's carrier"))?;
    let signed = Manifest {
        domain,
        nuclei,
        nonce: nonce()?,
    }
    .sign(reserve)
    .ok_or_else(unwritten_manifest)?;
    let act = encode_manifest(&signed).ok_or_else(unwritten_manifest)?;
    decode_grant_reply(&session.speak(act)?)
        .ok_or_else(|| io::Error::other("the grant did not decode"))
}

fn unwritten_manifest() -> io::Error {
    io::Error::other("the manifest passes its carrier")
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
            let (id, core, keeper) = (
                text(value, &mut at)?,
                text(value, &mut at)?,
                text(value, &mut at)?,
            );
            let from = coords::<3>(value, &mut at)?;
            let to = coords::<3>(value, &mut at)?;
            let freshness = u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?);
            Some(GateEnd {
                id,
                core,
                keeper,
                from,
                to,
                freshness,
            })
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

/// A gate's LOCAL throat map act (map record kind 2), built and signed by
/// `reserve` (no session): the throat named by the gate, kept by the line on
/// the keeper's world, its two frames the translators to the hub's place and
/// to the core's centre, fresh for the gate's freshness.
pub fn map_frame(
    reserve: &ed25519_dalek::SigningKey,
    domain: [u8; 32],
    point: TPoint,
    end: &GateEnd,
) -> io::Result<Vec<u8>> {
    let frame = |c: &[cfe::ratio::Ratio; 3]| {
        mycelium_sdk::house::translator_at(&[c[0].clone(), c[1].clone(), c[2].clone()])
    };
    let record = MapRecord::LocalThroat {
        throat: *blake3::hash(format!("netstratum gate {}", end.id).as_bytes()).as_bytes(),
        keeper: crate::house::line_key(&end.keeper),
        frames: (frame(&end.from), frame(&end.to)),
        freshness: end.freshness,
    };
    Ok(encode_map(
        &MapAct {
            domain,
            point,
            record,
            nonce: nonce()?,
        }
        .sign(reserve),
    ))
}

/// 5. The gates as LOCAL throats (map record kind 2), each signed by the
///    reserve and citing a final point: every gate offered at a place it is not
///    sealed at is mapped there, and the seal written back with that place for
///    the conductor (`gates-mapped.bin`). Only after the re-founding under the
///    present origin. Returns what happened, and whether the court refused an act
///    as an unknown domain (the recut).
pub fn map_gates(
    session: &Session,
    home: &Path,
    var_root: &Path,
) -> io::Result<(Vec<String>, bool)> {
    let Some(point) = session.tuniversal() else {
        return Ok((Vec::new(), false));
    };
    if refounded_under(home).as_deref() != Some(origin_of(&point).as_str()) {
        return Ok((Vec::new(), false));
    }
    let ends = read_gate_ends(var_root);
    let mut sealed = read_sealed(var_root);
    let due: Vec<&GateEnd> = ends
        .iter()
        .filter(|e| !sealed.iter().any(|(id, at)| *id == e.id && *at == e.from))
        .collect();
    if due.is_empty() {
        return Ok((Vec::new(), false));
    }
    let reserve = load_key(home, "tier-union.key")?;
    let domain = load_key(home, "tier-domain.key")?
        .verifying_key()
        .to_bytes();
    let mut said = Vec::new();
    let mut unknown = false;
    for end in due {
        match decode_map_reply(&session.speak(map_frame(&reserve, domain, point.clone(), end)?)?) {
            Some(Mapped::Sealed { h, .. }) => {
                sealed.retain(|(id, _)| *id != end.id);
                sealed.push((end.id.clone(), end.from.clone()));
                said.push(format!(
                    "GATE {} MAPPED at h {h} (keeper {})",
                    end.id, end.keeper
                ));
            }
            Some(Mapped::Refused(why)) => {
                unknown |= matches!(why, MapRefusal::UnknownDomain);
                said.push(format!("GATE {} not mapped: {why:?}", end.id));
            }
            Some(Mapped::Boundary(why)) => {
                said.push(format!("GATE {} refused at the boundary: {why:?}", end.id));
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
