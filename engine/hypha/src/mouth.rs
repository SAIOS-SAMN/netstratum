//! A wormhole mouth between two domains (Mycelium 78.128, #327; netstratum
//! #109, xylarium #281). Netstratum's reserve answers a mouth offered to it
//! (`Handoff::Accept`) and maps its side of the mouth (`MapRecord::Mouth`).
//! Each act is signed by the union reserve (`tier-union.key`) and sent on
//! the uplink's session. Nothing here runs unless a command names it.
use crate::refound::{fresh, load_key};
use mycelium_sdk::house::{
    decode_handoff_reply, decode_map_reply, encode_handoff, encode_map, Handed, Handoff, MapAct,
    MapRecord, Mapped,
};
use mycelium_sdk::session::Session;
use std::io;
use std::path::Path;

/// The 32 bytes a 64-hex string names, refused by name otherwise.
pub fn id_of(hex: &str) -> io::Result<[u8; 32]> {
    let bytes: Option<Vec<u8>> = (0..hex.len())
        .step_by(2)
        .map(|i| {
            hex.get(i..i + 2)
                .and_then(|b| u8::from_str_radix(b, 16).ok())
        })
        .collect();
    bytes
        .and_then(|b| <[u8; 32]>::try_from(b).ok())
        .ok_or_else(|| io::Error::other(format!("{hex} is not 32 bytes of hex")))
}

/// Accept the mouth `id` offered to this domain's reserve.
pub fn accept(session: &Session, home: &Path, id: [u8; 32]) -> io::Result<Handed> {
    let reserve = load_key(home, "tier-union.key")?;
    let signed = Handoff::Accept { id }.sign(&reserve);
    let act =
        encode_handoff(&signed).ok_or_else(|| io::Error::other("the accept cannot be written"))?;
    decode_handoff_reply(&session.speak(act)?)
        .ok_or_else(|| io::Error::other("the handoff reply did not decode"))
}

/// Map this domain's side of the mouth `id`, fresh for `freshness`, signed by
/// the reserve at the session's final point (its crossing still).
pub fn map(
    session: &Session,
    home: &Path,
    id: [u8; 32],
    side: u8,
    freshness: u64,
) -> io::Result<Mapped> {
    let reserve = load_key(home, "tier-union.key")?;
    let domain = load_key(home, "tier-domain.key")?
        .verifying_key()
        .to_bytes();
    let point = session
        .tuniversal()
        .ok_or_else(|| io::Error::other("no final TUniversal point read yet"))?;
    let act = MapAct {
        domain,
        point: point.clone(),
        record: MapRecord::Mouth {
            place: (id, side),
            freshness,
        },
        nonce: fresh()?,
    }
    .sign(crate::refound::still(&point), &reserve)
    .ok_or_else(|| io::Error::other("the map act's record cannot be written"))?;
    let frame =
        encode_map(&act).ok_or_else(|| io::Error::other("the map act cannot be written"))?;
    decode_map_reply(&session.speak(frame)?)
        .ok_or_else(|| io::Error::other("the map reply did not decode"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_mouth_id_reads_from_64_hex_and_refuses_anything_else() {
        let hex = "57ff54ac79b450f043f632eca5df37add052c65ebcb14ae568acd1e887849f8b";
        let id = super::id_of(hex).expect("32 bytes");
        assert_eq!(id[0], 0x57);
        assert_eq!(id[31], 0x8b);
        assert!(super::id_of(&hex[..62]).is_err());
        assert!(super::id_of("zz").is_err());
    }
}
