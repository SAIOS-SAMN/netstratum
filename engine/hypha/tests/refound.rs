//! docs/72 A4 — the re-founding's acts as the court reads them: netstratum's
//! ledger entry and gate map act, built by `ns_hypha::refound`, decoded by
//! Mycelium's own court decoder (`decode_domain_request`) and verified against
//! the reserve's key. Each test also breaks the act and must see it refused.

use ed25519_dalek::{Signature, SigningKey, Verifier};
use mycelium::anastome::dialogue::atlas::MapRecord;
use mycelium::anastome::dialogue::domain::{decode_domain_request, DomainRequest};
use mycelium::chronometry::tuniversal::{Origin, TPoint};
use ns_hypha::refound::{ledger_frame, map_frame, GateEnd};

fn point() -> TPoint {
    TPoint { origin: Origin { genesis: [7; 32], epoch: 0x92 }, h: 0xb708, lanes: [1, 2, 3, 4], k: 5 }
}

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("ns-refound-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("substrata")).unwrap();
    dir
}

#[test]
fn the_gate_map_act_is_read_by_the_court_as_netstratums_local_throat() {
    let reserve = SigningKey::from_bytes(&[11; 32]);
    let domain = SigningKey::from_bytes(&[12; 32]).verifying_key().to_bytes();
    let r = |n: i64, d: i64| cfe::ratio::ratio(n, d);
    let end = GateEnd {
        id: "stillcold-sky-3".into(),
        core: "stillcold-sky-3".into(),
        keeper: "cryo-vault".into(),
        from: [r(13, 2), r(-3, 1), r(1, 4)],
        to: [r(0, 1), r(0, 1), r(0, 1)],
        freshness: 1,
    };
    let frame = map_frame(&reserve, domain, point(), &end).unwrap();
    let Some(DomainRequest::Map(s)) = decode_domain_request(&frame) else { panic!("the court does not read it as a MAP act") };
    assert_eq!(s.act.domain, domain);
    assert_eq!(s.act.point, point());
    let MapRecord::LocalThroat { throat, keeper, frames, .. } = &s.act.record else { panic!("not a local throat") };
    assert_eq!(*throat, *blake3::hash(b"netstratum gate stillcold-sky-3").as_bytes(), "the throat is named by its gate");
    assert_eq!(*keeper, ns_hypha::house::line_key("cryo-vault"), "kept by the keeper's line");
    let to = |c: &[cfe::ratio::Ratio; 3]| mycelium::transducer::cga::translator_at(&[c[0].clone(), c[1].clone(), c[2].clone()]);
    assert_eq!(frames.0, to(&end.from), "side 0 stands at the hub");
    assert_eq!(frames.1, to(&end.to), "side 1 stands at the core");
    reserve
        .verifying_key()
        .verify(&s.act.signed_bytes(), &Signature::from_bytes(&s.sig))
        .expect("signed by the reserve");
    // Broken: a different key's signature does not verify as the reserve's.
    let other = SigningKey::from_bytes(&[13; 32]);
    let forged = map_frame(&other, domain, point(), &end).unwrap();
    let Some(DomainRequest::Map(f)) = decode_domain_request(&forged) else { panic!("decodes") };
    assert!(reserve.verifying_key().verify(&f.act.signed_bytes(), &Signature::from_bytes(&f.sig)).is_err());
    // Broken: a flipped byte in the record is refused by the decoder or the signature.
    let mut torn = frame.clone();
    let at = 1 + 32 + 2 + 1 + 84 + 3;
    torn[at] ^= 0xff;
    let survives = match decode_domain_request(&torn) {
        Some(DomainRequest::Map(t)) => reserve.verifying_key().verify(&t.act.signed_bytes(), &Signature::from_bytes(&t.sig)).is_ok(),
        _ => false,
    };
    assert!(!survives, "a torn act must not verify");
}

#[test]
fn the_ledger_entry_commits_to_netstratums_checkpoints() {
    let reserve = SigningKey::from_bytes(&[21; 32]);
    let domain = SigningKey::from_bytes(&[22; 32]).verifying_key().to_bytes();
    let var = scratch("ledger");
    // No checkpoints: Mycelium's law, the empty commitment is zero.
    let (empty, n) = ledger_frame(&reserve, domain, &var, 100, point(), [0; 32]).unwrap();
    assert_eq!(n, 0);
    let Some(DomainRequest::Ledger(e)) = decode_domain_request(&empty) else { panic!("the court does not read it as a LEDGER entry") };
    assert_eq!(e.entry.commitment, [0; 32]);
    // With checkpoints: committed, signed, parent and u as given.
    std::fs::write(var.join("ancients.bin"), b"arose").unwrap();
    std::fs::write(var.join("substrata").join("ab.tube"), b"tube").unwrap();
    let (frame, n) = ledger_frame(&reserve, domain, &var, 101, point(), [9; 32]).unwrap();
    assert_eq!(n, 2, "both checkpoints committed");
    let Some(DomainRequest::Ledger(s)) = decode_domain_request(&frame) else { panic!("decodes") };
    assert_eq!((s.entry.domain, s.entry.u, s.entry.parent, s.entry.events), (domain, 101, [9; 32], [0; 32]));
    assert_ne!(s.entry.commitment, [0; 32]);
    reserve.verifying_key().verify(&s.entry.signed_bytes(), &Signature::from_bytes(&s.sig)).expect("signed by the reserve");
    // Broken: a changed checkpoint changes the commitment.
    std::fs::write(var.join("ancients.bin"), b"arose twice").unwrap();
    let (changed, _) = ledger_frame(&reserve, domain, &var, 101, point(), [9; 32]).unwrap();
    let Some(DomainRequest::Ledger(c)) = decode_domain_request(&changed) else { panic!("decodes") };
    assert_ne!(c.entry.commitment, s.entry.commitment, "the commitment must see the checkpoint change");
    let _ = std::fs::remove_dir_all(&var);
}
