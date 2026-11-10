//! Netstratum's ledger tree on the court (Mycelium 78.119 / 78.120).
//!
//! A tier is a signing key that folds a ledger for the domain; the court
//! admits a tier's fold by continuity — on every lane the identities' nets
//! sum exactly to the declared emission — and seals it, citing the rows by
//! hash. The rows stay with Netstratum: the chronicle is the record, and
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
use mycelium_sdk::house::domain::{DomainGenesis, Founded, Reservation};
use mycelium_sdk::house::uplink::{Child, ChildNet, FoldAt, Net, SignedFold, TierFold, Uplinked};
use std::io::{self, Read};
use std::path::Path;

/// The kind of record Netstratum's origin commits to: its universe ledger's
/// Big Bang (`UniverseFounded`, tag 190), the receipt whose own-hash is the seed.
pub const ORIGIN_KIND: u8 = 190;

/// The tiers Netstratum's domain names at its founding.
pub const TIERS: [&str; 2] = ["colonies", "union"];

/// The lanes of a fold row, by MU-LEXICON v2's fold-lane law: S matter,
/// G value (the only lane a parent may translate), T duration, H
/// information. μ is never summed across lanes, and neither is anything
/// here. Netstratum fills S (matter), G (STAMP) and H (words); T stays zero.
pub mod lane {
    /// Banked matter (iron, biomass, alloy, diamond, ember, sink, crystal),
    /// in units of the resource ledger's 1/1024 grain.
    pub const MATTER: usize = 0;
    /// STAMP, Netstratum's unit of account, on the value lane, in the same
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

/// The universe's radius for its origin (era 1, ruling of 2026-10-22):
/// the reach its bell crosses, half the bell's period `T_u/2` light-rings
/// (`pulse::Universe::t_u`, `<var>/pulse.bin` tag 17). The ring law is already
/// geometrized (mass is holding, in the units of reach; no G, no c), so the
/// court's `2m/R` reads true in Netstratum's own unit; the court's name for
/// that unit is a label only.
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
pub fn fold(
    key: &ed25519_dalek::SigningKey,
    prefix: [u8; 32],
    mut rows: Vec<Net>,
    mut declared: Vec<Net>,
) -> SignedFold {
    rows.sort_by_key(|(who, _)| *who);
    declared.sort_by_key(|(who, _)| *who);
    TierFold {
        tier: key.verifying_key().to_bytes(),
        prefix,
        rows,
        declared,
        children: Vec::new(),
        handoffs: Vec::new(),
        // No crossing of the universal core tube is stated by a census.
        transitions: Vec::new(),
    }
    .sign(key)
}

/// A fold's position as one key: its receipt's stamp, then its path inside
/// that receipt (`u16` LE each; empty for a fold its own receipt sealed, so
/// a sealed fold keys exactly as its stamp).
pub fn at_key(at: &FoldAt) -> Vec<u8> {
    let mut k = at.stamp.key();
    for i in &at.path {
        k.extend_from_slice(&i.to_le_bytes());
    }
    k
}

/// The position [`at_key`] wrote; `None` where it does not read.
pub fn at_of(k: &[u8]) -> Option<FoldAt> {
    let mut p = 0usize;
    let stamp = mycelium_sdk::house::Stamp::take_canonical(k, &mut p)?;
    let rest = k.get(p..)?;
    if rest.len() % 2 != 0 {
        return None;
    }
    Some(FoldAt {
        stamp,
        path: rest
            .chunks(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect(),
    })
}

/// A fold's EXPOSURE per lane: what its rows move, each lane's positive
/// parts summed (a balanced fold moves as much out as in; mycelium #237's
/// candidate `X_l`, against which a bond `B_l ≥ X_l` answers).
pub fn exposure(rows: &[Net]) -> [u64; 4] {
    let mut x = [0u64; 4];
    for (_, lanes) in rows {
        for (l, v) in lanes.iter().enumerate() {
            if *v > 0 {
                x[l] = x[l].saturating_add(u64::try_from(*v).unwrap_or(u64::MAX));
            }
        }
    }
    x
}

impl Uplink {
    /// v0.4.0 (78.138; MU-HOSTING §5a.5) — A TIER'S FOLD CROSSES CARRIED:
    /// a universe's events enter the court only as the act its domain's
    /// sitting reserve signs, a line's own fold carried whole in it under
    /// the line's own signature (never re-signed, never reduced to bare
    /// nets). The reserve first bonds the line by the fold's exposure,
    /// `B = X` per lane (78.129; mycelium #237's candidate), from μ it
    /// holds; then its fresh fold carries the child, including all of it
    /// (the union nets the colonies, 12_UNION) at rate 1 on every lane.
    /// Returns where the carried fold stands (`[0]` inside the reserve's
    /// receipt), or what the court answered instead (the bond's refusal as
    /// `Err(Ok(founded))`, the crossing's as `Err(Err(uplinked))`).
    pub fn carry_fold(
        &self,
        domain: [u8; 32],
        reserve: &ed25519_dalek::SigningKey,
        child: &SignedFold,
    ) -> io::Result<Result<FoldAt, Result<Founded, Uplinked>>> {
        use mycelium_sdk::house::domain::{decode_domain_reply, encode_bond, Bond};
        let x = exposure(&child.fold.rows);
        if x.iter().any(|v| *v > 0) {
            let bond = Bond {
                domain,
                reserve: reserve.verifying_key().to_bytes(),
                tier: child.fold.tier,
                mu: Some(x.into()),
                nonce: crate::refound::fresh()?,
            }
            .sign(reserve);
            let answered = decode_domain_reply(&self.session.speak(encode_bond(&bond))?)
                .ok_or_else(|| io::Error::other("the bond's reply did not decode"))?;
            if !matches!(answered, Founded::Bonded { .. }) {
                return Ok(Err(Ok(answered)));
            }
        }
        let net = ChildNet {
            rows: child.fold.rows.clone(),
            declared: child.fold.declared.clone(),
            child: Child::Carried(Box::new(child.clone())),
            included_rows: child.fold.rows.clone(),
            included_declared: child.fold.declared.clone(),
            rate: [(1, 1); 4],
        };
        let carrier = TierFold {
            tier: reserve.verifying_key().to_bytes(),
            prefix: crate::refound::fresh()?,
            rows: Vec::new(),
            declared: Vec::new(),
            children: vec![net],
            handoffs: Vec::new(),
            transitions: Vec::new(),
        }
        .sign(reserve);
        Ok(match self.session.uplink_fold(&carrier)? {
            Uplinked::Chained { at, .. } => Ok(FoldAt {
                stamp: at,
                path: vec![0],
            }),
            other => Err(Err(other)),
        })
    }

    /// 78.125 — a fold's finality at the court's head, by its receipt's
    /// stamp, and (78.129) whether its tier was bonded. A reading: nothing is
    /// sealed by it.
    pub fn finality(
        &self,
        at: FoldAt,
    ) -> io::Result<(mycelium_sdk::house::challenge::Finality, bool)> {
        self.session.finality(at).map(|r| (r.finality, r.bonded))
    }

    /// 78.125 — the tier's LINE head on the court: where its next fold's span
    /// replays from. A client whose own line head disagrees would be
    /// falsified on demand, so it must not fold until they agree.
    pub fn line_head(&self, tier: [u8; 32]) -> io::Result<[u8; 32]> {
        self.session.line_head(tier).map(|h| h.head)
    }

    /// The startup check (coordinator 2026-10-07): the local line's head must
    /// equal the court's for this tier before any fold is sent. `Err` names
    /// both heads when they disagree; nothing is sent on a disagreement.
    pub fn check_line_head(
        &self,
        key: &ed25519_dalek::SigningKey,
        local: [u8; 32],
    ) -> io::Result<()> {
        let court = self.line_head(key.verifying_key().to_bytes())?;
        if court == local {
            return Ok(());
        }
        let hex = |h: &[u8; 32]| {
            h.iter()
                .take(8)
                .map(|b| format!("{b:02x}"))
                .collect::<String>()
        };
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("the court's line head {}… is not this line's {}…; refusing to fold until they agree", hex(&court), hex(&local)),
        ))
    }

    /// Publish A fold's prefix to the court for replay (78.125 evidence):
    /// opened with the fold's own rows and declared, then its entries in
    /// parts of at most [`PART_BYTES`] encoded, so a fold of any span
    /// streams. Signed by `key` (any filer may publish). Returns the court's
    /// last answer — `Upheld` when the replay reaches the cited prefix.
    pub fn publish(
        &self,
        key: &ed25519_dalek::SigningKey,
        at: FoldAt,
        entries: Vec<mycelium_sdk::house::challenge::Entry>,
    ) -> io::Result<mycelium_sdk::house::challenge::Challenged> {
        use mycelium_sdk::house::challenge::{Challenge, Challenged};
        let (rows, declared) = crate::line::fold_of(&entries);
        let opened = self.session.challenge(
            &Challenge::Open {
                fold: at.clone(),
                rows,
                declared,
                children: Vec::new(),
            }
            .sign(key),
        )?;
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
            last = self.session.challenge(
                &Challenge::Part {
                    fold: at.clone(),
                    entries,
                }
                .sign(key),
            )?;
            if !matches!(
                last,
                Challenged::Streaming { .. } | Challenged::Upheld { .. }
            ) {
                break;
            }
        }
        Ok(last)
    }

    /// Put a signed fold to the court for admission.
    pub fn uplink_fold(&self, f: &SignedFold) -> io::Result<Uplinked> {
        self.session.uplink_fold(f)
    }

    /// Found Netstratum's domain on this court: the domain key names every
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
        let signed = DomainGenesis {
            domain: domain.verifying_key().to_bytes(),
            tiers,
        }
        .sign(&domain);
        self.session.found_domain(&signed)
    }

    /// **The origin** (78.132/78.139/78.140): Netstratum's universe stated
    /// once on this court, signed by the domain key — version 2, a root
    /// (parent face unwitnessed), its space a closed sphere (era 1, docs/78;
    /// `Geometry::Sphere`, Mycelium 8718fede: `Curved` is hyperbolic ℍ³, which
    /// misstated the universe; the era-0 L1 torus is retired) of radius `radius`
    /// (`universe_radius`). The commitment is the universe's Big Bang: the
    /// own-hash of its `UniverseFounded` receipt (its seed). Never edited.
    pub fn originate(
        &self,
        home: &Path,
        seed: [u8; 32],
        radius: (u64, u64),
    ) -> io::Result<Founded> {
        use mycelium_sdk::house::domain::BigBangRecord;
        use mycelium_sdk::house::universe::{Genesis, Geometry, Parent};
        let domain = tier_key(home, "domain")?;
        let origin = BigBangRecord {
            domain: domain.verifying_key().to_bytes(),
            kind: ORIGIN_KIND,
            version: 2,
            commitment: seed,
            universe: Some(Genesis {
                geometry: Geometry::Sphere(radius),
                parent: Parent::Unwitnessed,
            }),
        };
        self.session.originate(&origin.sign(&domain))
    }

    /// Designate `tier` (one of [`TIERS`]) as the domain's reserve on this
    /// court, signed by the domain key — held in trust for the ancient not yet
    /// arisen (docs/58 §6 ruling 7). No basket: that is the sitting reserve's
    /// own act, once the ancients take it up.
    pub fn designate_reserve(&self, home: &Path, tier: &str) -> io::Result<Founded> {
        if !TIERS.contains(&tier) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{tier} is not one of the domain's tiers {TIERS:?}"),
            ));
        }
        let domain = tier_key(home, "domain")?;
        let tier = tier_key(home, tier)?.verifying_key().to_bytes();
        let mut nonce = [0u8; 32];
        std::fs::File::open("/dev/urandom")?.read_exact(&mut nonce)?;
        let signed = Reservation {
            domain: domain.verifying_key().to_bytes(),
            tier,
            nonce,
        }
        .sign(&domain);
        self.session.reserve(&signed)
    }
}

#[cfg(test)]
mod tests {
    use super::{at_key, at_of, exposure};
    use mycelium_sdk::house::uplink::FoldAt;
    use mycelium_sdk::house::Stamp;

    fn stamp() -> Stamp {
        Stamp {
            h: 1376,
            vk: [7; 32],
            k: 48,
            i: 2,
        }
    }

    #[test]
    fn a_folds_position_reads_back_and_a_sealed_one_keys_as_its_stamp() {
        let sealed = FoldAt::sealed(stamp());
        assert_eq!(at_key(&sealed), stamp().key());
        assert_eq!(at_of(&at_key(&sealed)), Some(sealed));
        let carried = FoldAt {
            stamp: stamp(),
            path: vec![0, 3],
        };
        assert_eq!(at_of(&at_key(&carried)), Some(carried));
        let mut odd = stamp().key();
        odd.push(1);
        assert_eq!(at_of(&odd), None, "a path of half a step does not read");
    }

    #[test]
    fn a_balanced_folds_exposure_is_what_it_moves_out() {
        // 5 moved from one holder to another on S, 3 on G: the exposure is
        // what moved, never the sum of both sides.
        let rows = vec![([1; 32], [5, -3, 0, 0]), ([2; 32], [-5, 3, 0, 0])];
        assert_eq!(exposure(&rows), [5, 3, 0, 0]);
        assert_eq!(exposure(&[]), [0, 0, 0, 0]);
    }
}
