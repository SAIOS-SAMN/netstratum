//! Netstratum's ledger tree on the court (mycelium 78.119 / 78.120).
//!
//! A TIER is a signing key that folds a ledger for the domain; the court
//! admits a tier's fold by continuity — on every lane the identities' nets
//! sum exactly to the declared emission — and seals it, citing the rows by
//! hash. The rows stay with netstratum: the chronicle is the record, and
//! anyone holding it can replay the cited prefix.
//!
//! Netstratum's tiers, named once at the domain's founding:
//!
//!   * `colonies` — folds each people's own chronicle: every colony orb's
//!     banked matter and STAMP, the sources and sinks its own receipts
//!     declare ([`crate::census`]).
//!   * `union` — folds what crosses between peoples (apertures, routes,
//!     the knowledge market), where one people's sink is another's source
//!     and the union nets them to zero (12_UNION). Named now because a
//!     domain's tiers are fixed at its genesis; no union fold is sent yet.
//!
//! A tier's key lives in the uplink home as `tier-<name>.key`, a 32-byte
//! seed minted on first use; the domain's own key is `tier-domain.key`.

use crate::wire::HouseActs;
use crate::Uplink;
use mycelium::anastome::dialogue::domain::{DomainGenesis, Founded, Reservation};
use mycelium::anastome::dialogue::uplink::{FoldAt, Net, SignedFold, TierFold, Uplinked};
use std::io::{self, Read};
use std::path::Path;

/// The kind of record netstratum's origin commits to: its universe ledger's
/// Big Bang (`UniverseFounded`, tag 190), the receipt whose own-hash is the seed.
pub const ORIGIN_KIND: u8 = 190;

/// The tiers netstratum's domain names at its founding.
pub const TIERS: [&str; 2] = ["colonies", "union"];

/// The lanes of a fold row, by MU-LEXICON v2's fold-lane law: S matter,
/// G value (the only lane a parent may translate), T duration, H
/// information. μ is never summed across lanes, and neither is anything
/// here. Netstratum fills S (matter), G (STAMP) and H (words); T stays zero.
pub mod lane {
    /// Banked matter (iron, biomass, alloy, diamond, joule, sink, crystal),
    /// in units of the resource ledger's 1/1024 grain.
    pub const MATTER: usize = 0;
    /// STAMP, netstratum's unit of account, on the value lane, in the same
    /// grain.
    pub const MONEY: usize = 1;
    /// Words a people mints — each one issued is a declared source on the
    /// information lane, delivered to the people's lexicon as a row
    /// (MU-LEXICON v2). Counted whole: one word, one unit.
    pub const INFORMATION: usize = 3;
}

/// A streamed publication's part bound, under the court's 786,432-byte
/// encoded limit with room for the frame around it.
pub const PART_BYTES: usize = 700_000;

/// THE UNIVERSE'S RADIUS for its origin (era 1, operator 2026-10-22: "proceed"):
/// the reach its bell crosses, half the bell's period `T_u/2` light-rings
/// (`pulse::Universe::t_u`, `<var>/pulse.bin` tag 17). The ring law is already
/// geometrized (mass is holding, in the units of reach; no G, no c), so the
/// court's `2m/R` reads true in netstratum's own unit; the court names that
/// unit a megaparsec, a label only.
pub fn universe_radius(var_root: &Path) -> io::Result<(u64, u64)> {
    let bytes = std::fs::read(var_root.join("pulse.bin"))?;
    let t_u = cfe::tlv::records(&bytes)
        .find(|(t, _)| *t == 17)
        .and_then(|(_, v)| v.try_into().ok().map(u64::from_le_bytes))
        .filter(|t| *t > 0)
        .ok_or_else(|| io::Error::other("the universe has no bell (pulse.bin tag 17)"))?;
    Ok((t_u, 2))
}

/// The tier's signing key, from `home/tier-<name>.key` (minted on first use).
pub fn tier_key(home: &Path, name: &str) -> io::Result<ed25519_dalek::SigningKey> {
    std::fs::create_dir_all(home)?;
    let path = home.join(format!("tier-{name}.key"));
    if let Ok(b) = std::fs::read(&path) {
        if let Ok(seed) = <[u8; 32]>::try_from(b.as_slice()) {
            return Ok(ed25519_dalek::SigningKey::from_bytes(&seed));
        }
    }
    let mut seed = [0u8; 32];
    std::fs::File::open("/dev/urandom")?.read_exact(&mut seed)?;
    std::fs::write(&path, seed)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
    }
    Ok(ed25519_dalek::SigningKey::from_bytes(&seed))
}

/// An identity on the ledger tree: the digest of what names it in the game
/// (a colony's holding, a declared source), so no registry is needed.
pub fn identity(kind: &str, name: &str) -> [u8; 32] {
    *blake3::Hasher::new_derive_key("netstratum 2026-10-07 ledger identity")
        .update(kind.as_bytes())
        .update(&[0])
        .update(name.as_bytes())
        .finalize()
        .as_bytes()
}

/// A single-level fold of `rows` and `declared` over the tier line whose
/// head is `prefix` ([`crate::line`]), signed by `key`. Both lists are
/// sorted by identity, as continuity requires; the caller hands each
/// identity once.
pub fn fold(key: &ed25519_dalek::SigningKey, prefix: [u8; 32], mut rows: Vec<Net>, mut declared: Vec<Net>) -> SignedFold {
    rows.sort_by_key(|(who, _)| *who);
    declared.sort_by_key(|(who, _)| *who);
    TierFold { tier: key.verifying_key().to_bytes(), prefix, rows, declared, children: Vec::new(), handoffs: Vec::new() }.sign(key)
}

impl Uplink {
    /// 78.125 — a fold's finality at the court's head, by its receipt's
    /// stamp, and (78.129) whether its tier was bonded. A reading: nothing is
    /// sealed by it.
    pub fn finality(&self, at: mycelium::chronometry::Stamp) -> io::Result<(mycelium::anastome::dialogue::challenge::Finality, bool)> {
        self.session.finality(FoldAt::sealed(at)).map(|r| (r.finality, r.bonded))
    }

    /// 78.125 — the tier's LINE HEAD on the court: where its next fold's span
    /// replays from. A client whose own line head disagrees would be
    /// falsified on demand, so it must not fold until they agree.
    pub fn line_head(&self, tier: [u8; 32]) -> io::Result<[u8; 32]> {
        self.session.line_head(tier).map(|h| h.head)
    }

    /// THE STARTUP CHECK (coordinator 2026-10-07): the local line's head must
    /// equal the court's for this tier before any fold is sent. `Err` names
    /// both heads when they disagree; nothing is sent on a disagreement.
    pub fn check_line_head(&self, key: &ed25519_dalek::SigningKey, local: [u8; 32]) -> io::Result<()> {
        let court = self.line_head(key.verifying_key().to_bytes())?;
        if court == local {
            return Ok(());
        }
        let hex = |h: &[u8; 32]| h.iter().take(8).map(|b| format!("{b:02x}")).collect::<String>();
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("the court's line head {}… is not this line's {}…; refusing to fold until they agree", hex(&court), hex(&local)),
        ))
    }

    /// PUBLISH A FOLD'S PREFIX to the court for replay (78.125 evidence):
    /// opened with the fold's own rows and declared, then its entries in
    /// parts of at most [`PART_BYTES`] encoded, so a fold of any span
    /// streams. Signed by `key` (any filer may publish). Returns the court's
    /// last answer — `Upheld` when the replay reaches the cited prefix.
    pub fn publish(
        &self,
        key: &ed25519_dalek::SigningKey,
        at: mycelium::chronometry::Stamp,
        entries: Vec<mycelium::anastome::dialogue::challenge::Entry>,
    ) -> io::Result<mycelium::anastome::dialogue::challenge::Challenged> {
        use mycelium::anastome::dialogue::challenge::{Challenge, Challenged};
        let (rows, declared) = crate::line::fold_of(&entries);
        let opened = self.session.challenge(&Challenge::Open { fold: FoldAt::sealed(at), rows, declared, children: Vec::new() }.sign(key))?;
        if !matches!(opened, Challenged::Streaming { .. }) {
            return Ok(opened);
        }
        let mut last = opened;
        let mut part = Vec::new();
        let mut bytes = 0usize;
        let mut parts: Vec<Vec<_>> = Vec::new();
        for e in entries {
            let size = 2 + (e.rows.len() + e.declared.len()) * (32 + 64) + 4;
            if !part.is_empty() && bytes + size > PART_BYTES {
                parts.push(std::mem::take(&mut part));
                bytes = 0;
            }
            bytes += size;
            part.push(e);
        }
        if !part.is_empty() {
            parts.push(part);
        }
        for entries in parts {
            last = self.session.challenge(&Challenge::Part { fold: FoldAt::sealed(at), entries }.sign(key))?;
            if !matches!(last, Challenged::Streaming { .. } | Challenged::Upheld { .. }) {
                break;
            }
        }
        Ok(last)
    }

    /// Put a signed fold to the court for admission.
    pub fn uplink_fold(&self, f: &SignedFold) -> io::Result<Uplinked> {
        self.session.uplink_fold(f)
    }

    /// Found netstratum's domain on this court: the domain key names every
    /// tier in [`TIERS`], and from then on the court admits folds only from
    /// those tiers. Once per court: `AlreadyFounded` on a second call is the
    /// court remembering, not a fault.
    pub fn found_domain(&self, home: &Path) -> io::Result<Founded> {
        let domain = tier_key(home, "domain")?;
        let mut tiers = TIERS
            .iter()
            .map(|t| tier_key(home, t).map(|k| k.verifying_key().to_bytes()))
            .collect::<io::Result<Vec<_>>>()?;
        tiers.sort_unstable();
        tiers.dedup();
        let signed = DomainGenesis { domain: domain.verifying_key().to_bytes(), tiers }.sign(&domain);
        self.session.found_domain(&signed)
    }

    /// **THE ORIGIN** (78.132/78.139/78.140): netstratum's universe stated
    /// ONCE on this court, signed by the domain key — version 2, a ROOT
    /// (parent face unwitnessed), its space a closed SPHERE (era 1, docs/78;
    /// `Geometry::Sphere`, mycelium 8718fede: `Curved` is hyperbolic ℍ³, which
    /// misstated the universe; the era-0 L1 torus is retired) of radius `radius`
    /// (`universe_radius`). The commitment is the universe's Big Bang: the
    /// own-hash of its `UniverseFounded` receipt (its seed). Never edited.
    pub fn originate(&self, home: &Path, seed: [u8; 32], radius: (u64, u64)) -> io::Result<Founded> {
        use mycelium::anastome::dialogue::domain::Origin;
        use mycelium::anastome::dialogue::universe::{Genesis, Geometry, Parent};
        let domain = tier_key(home, "domain")?;
        let origin = Origin {
            domain: domain.verifying_key().to_bytes(),
            kind: ORIGIN_KIND,
            version: 2,
            commitment: seed,
            universe: Some(Genesis { geometry: Geometry::Sphere(radius), parent: Parent::Unwitnessed }),
        };
        self.session.originate(&origin.sign(&domain))
    }

    /// Designate `tier` (one of [`TIERS`]) as the domain's RESERVE on this
    /// court, signed by the domain key — held in trust for the ancient not yet
    /// arisen (docs/58 §6 ruling 7). No basket: that is the sitting reserve's
    /// own act, once the ancients take it up.
    pub fn designate_reserve(&self, home: &Path, tier: &str) -> io::Result<Founded> {
        if !TIERS.contains(&tier) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("{tier} is not one of the domain's tiers {TIERS:?}")));
        }
        let domain = tier_key(home, "domain")?;
        let tier = tier_key(home, tier)?.verifying_key().to_bytes();
        let mut nonce = [0u8; 32];
        std::fs::File::open("/dev/urandom")?.read_exact(&mut nonce)?;
        let signed = Reservation { domain: domain.verifying_key().to_bytes(), tier, nonce }.sign(&domain);
        self.session.reserve(&signed)
    }
}
