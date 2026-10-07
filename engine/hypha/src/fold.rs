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

use crate::Uplink;
use mycelium::anastome::dialogue::domain::{DomainGenesis, Founded};
use mycelium::anastome::dialogue::uplink::{Net, SignedFold, TierFold, Uplinked};
use std::io::{self, Read};
use std::path::Path;

/// The tiers netstratum's domain names at its founding.
pub const TIERS: [&str; 2] = ["colonies", "union"];

/// The lanes of a fold row, by MU-LEXICON v2's fold-lane law: S matter,
/// G value (the only lane a parent may translate), T duration, H
/// information. μ is never summed across lanes, and neither is anything
/// here. Netstratum fills S and G; T and H stay zero until it folds them.
pub mod lane {
    /// Banked matter (iron, biomass, alloy, diamond, joule, sink, crystal),
    /// in units of the resource ledger's 1/1024 grain.
    pub const MATTER: usize = 0;
    /// STAMP, netstratum's unit of account, on the value lane, in the same
    /// grain.
    pub const MONEY: usize = 1;
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
    TierFold { tier: key.verifying_key().to_bytes(), prefix, rows, declared, children: Vec::new() }.sign(key)
}

impl Uplink {
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
}
