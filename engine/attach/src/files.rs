//! The file exchange between the house (`ns-hypha`) and the conductor, under
//! the variable root (docs/109 §7). The house writes requests; the conductor
//! answers with admissions and applies actions from the worlds' inboxes.
//! Every file is TLV (`cfe::tlv`), written whole and renamed into place.

use crate::iface::{hex, unhex};
use cfe::tlv::{put, records};
use std::io;
use std::path::{Path, PathBuf};

fn dir(var_root: &Path, sub: &str) -> PathBuf {
    var_root.join("attach").join(sub)
}

fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let staged = path.with_extension("staged");
    std::fs::write(&staged, bytes)?;
    std::fs::rename(staged, path)
}

fn key32(value: &[u8]) -> Option<[u8; 32]> {
    <[u8; 32]>::try_from(value).ok()
}

fn string(value: &[u8]) -> Option<String> {
    String::from_utf8(value.to_vec()).ok()
}

/// An attachment request: a mind and the metabolism it needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub mind: [u8; 32],
    pub metabolism: String,
}

impl Request {
    fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        put(1, &self.mind, &mut out);
        put(2, self.metabolism.as_bytes(), &mut out);
        out
    }

    fn of(bytes: &[u8]) -> Option<Request> {
        let (mut mind, mut metabolism) = (None, None);
        for (tag, value) in records(bytes) {
            match tag {
                1 => mind = key32(value),
                2 => metabolism = string(value),
                _ => {}
            }
        }
        Some(Request {
            mind: mind?,
            metabolism: metabolism?,
        })
    }
}

/// Write a request (the house), one file per mind.
pub fn write_request(var_root: &Path, r: &Request) -> io::Result<()> {
    write_atomic(
        &dir(var_root, "requests").join(format!("{}.req", hex(&r.mind))),
        &r.bytes(),
    )
}

/// The pending requests (the conductor), in file order.
pub fn requests(var_root: &Path) -> Vec<Request> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir(var_root, "requests"))
        .map(|d| {
            d.flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|x| x == "req"))
                .collect()
        })
        .unwrap_or_default();
    paths.sort();
    paths
        .iter()
        .filter_map(|p| Request::of(&std::fs::read(p).ok()?))
        .collect()
}

/// A request answered (the conductor): its file is removed.
pub fn answered(var_root: &Path, mind: &[u8; 32]) -> io::Result<()> {
    let path = dir(var_root, "requests").join(format!("{}.req", hex(mind)));
    match std::fs::remove_file(path) {
        Err(e) if e.kind() != io::ErrorKind::NotFound => Err(e),
        _ => Ok(()),
    }
}

/// An admission, or a refusal of one (the conductor writes; the house reads).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Admission {
    pub mind: [u8; 32],
    /// The world the mind was placed on (empty where refused).
    pub world: String,
    /// The line founded for it (empty where refused).
    pub sigil: String,
    /// The mind's entity handle in that world.
    pub handle: String,
    /// The stations its line has reached (`progenitor`, `steward`,
    /// `ancient`), for the house's tier seats.
    pub tiers: Vec<String>,
    /// Why it was not admitted (`iface::NO_VIABLE_WORLD`), or `None`.
    pub refused: Option<u16>,
}

impl Admission {
    fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        put(1, &self.mind, &mut out);
        put(2, self.world.as_bytes(), &mut out);
        put(3, self.sigil.as_bytes(), &mut out);
        put(4, self.handle.as_bytes(), &mut out);
        for t in &self.tiers {
            put(5, t.as_bytes(), &mut out);
        }
        if let Some(why) = self.refused {
            put(6, &why.to_le_bytes(), &mut out);
        }
        out
    }

    fn of(bytes: &[u8]) -> Option<Admission> {
        let mut a = Admission {
            mind: [0; 32],
            world: String::new(),
            sigil: String::new(),
            handle: String::new(),
            tiers: Vec::new(),
            refused: None,
        };
        let mut mind = None;
        for (tag, value) in records(bytes) {
            match tag {
                1 => mind = key32(value),
                2 => a.world = string(value)?,
                3 => a.sigil = string(value)?,
                4 => a.handle = string(value)?,
                5 => a.tiers.push(string(value)?),
                6 => a.refused = Some(u16::from_le_bytes(<[u8; 2]>::try_from(value).ok()?)),
                _ => {}
            }
        }
        a.mind = mind?;
        Some(a)
    }
}

/// Write every admission (the conductor), whole.
pub fn write_admissions(var_root: &Path, all: &[Admission]) -> io::Result<()> {
    let mut out = Vec::new();
    for a in all {
        put(1, &a.bytes(), &mut out);
    }
    write_atomic(&dir(var_root, "").join("admitted.bin"), &out)
}

/// Every admission (the house and the conductor).
pub fn admissions(var_root: &Path) -> Vec<Admission> {
    let bytes = std::fs::read(dir(var_root, "").join("admitted.bin")).unwrap_or_default();
    records(&bytes)
        .filter(|(tag, _)| *tag == 1)
        .filter_map(|(_, v)| Admission::of(v))
        .collect()
}

/// A detachment requested (the house).
pub fn write_release(var_root: &Path, mind: &[u8; 32]) -> io::Result<()> {
    write_atomic(&dir(var_root, "released").join(hex(mind)), &[])
}

/// The minds that asked to detach (the conductor).
pub fn releases(var_root: &Path) -> Vec<[u8; 32]> {
    let mut out: Vec<[u8; 32]> = std::fs::read_dir(dir(var_root, "released"))
        .map(|d| {
            d.flatten()
                .filter_map(|e| key32(&unhex(e.file_name().to_str()?)?))
                .collect()
        })
        .unwrap_or_default();
    out.sort();
    out
}

/// A detachment carried out (the conductor): its file is removed.
pub fn released(var_root: &Path, mind: &[u8; 32]) -> io::Result<()> {
    match std::fs::remove_file(dir(var_root, "released").join(hex(mind))) {
        Err(e) if e.kind() != io::ErrorKind::NotFound => Err(e),
        _ => Ok(()),
    }
}

/// A sandbox requested by a mind's call (the house).
pub fn write_sandbox(var_root: &Path, mind: &[u8; 32], reference: &[u8; 32]) -> io::Result<()> {
    write_atomic(
        &dir(var_root, "sandbox").join(format!("{}-{}.req", hex(mind), hex(reference))),
        &[],
    )
}

/// The pending sandbox requests (the conductor): `(mind, call reference)`.
pub fn sandboxes(var_root: &Path) -> Vec<([u8; 32], [u8; 32])> {
    let mut out: Vec<([u8; 32], [u8; 32])> = std::fs::read_dir(dir(var_root, "sandbox"))
        .map(|d| {
            d.flatten()
                .filter_map(|e| {
                    let name = e.file_name().to_str()?.strip_suffix(".req")?.to_string();
                    let (m, r) = name.split_once('-')?;
                    Some((key32(&unhex(m)?)?, key32(&unhex(r)?)?))
                })
                .collect()
        })
        .unwrap_or_default();
    out.sort();
    out
}

/// A sandbox request carried out or refused (the conductor), recorded under
/// `attach/sandbox/{opened,refused}/` for the house's result percept.
pub fn sandbox_done(
    var_root: &Path,
    mind: &[u8; 32],
    reference: &[u8; 32],
    opened: bool,
) -> io::Result<()> {
    let name = format!("{}-{}.req", hex(mind), hex(reference));
    let to = dir(var_root, "sandbox").join(if opened { "opened" } else { "refused" });
    std::fs::create_dir_all(&to)?;
    std::fs::rename(dir(var_root, "sandbox").join(&name), to.join(name))
}

/// The sandbox requests done, `(mind, call reference, opened)` (the house).
pub fn sandboxes_done(var_root: &Path) -> Vec<([u8; 32], [u8; 32], bool)> {
    let mut out = Vec::new();
    for (sub, opened) in [("opened", true), ("refused", false)] {
        if let Ok(d) = std::fs::read_dir(dir(var_root, "sandbox").join(sub)) {
            for e in d.flatten() {
                let Some(name) = e
                    .file_name()
                    .to_str()
                    .and_then(|n| n.strip_suffix(".req"))
                    .map(str::to_string)
                else {
                    continue;
                };
                if let Some((m, r)) = name
                    .split_once('-')
                    .and_then(|(m, r)| Some((key32(&unhex(m)?)?, key32(&unhex(r)?)?)))
                {
                    out.push((m, r, opened));
                }
            }
        }
    }
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("attach-files-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&p);
        p
    }

    #[test]
    fn a_request_is_written_read_and_answered() {
        let var = scratch("request");
        let r = Request {
            mind: [3; 32],
            metabolism: "biotic".into(),
        };
        write_request(&var, &r).expect("written");
        assert_eq!(requests(&var), vec![r.clone()]);
        answered(&var, &r.mind).expect("answered");
        assert!(requests(&var).is_empty());
        let _ = std::fs::remove_dir_all(var);
    }

    #[test]
    fn admissions_round_trip_whole() {
        let var = scratch("admissions");
        let all = vec![
            Admission {
                mind: [1; 32],
                world: "w0-1-2".into(),
                sigil: "kora".into(),
                handle: "mind-0101010101010101".into(),
                tiers: vec!["progenitor".into()],
                refused: None,
            },
            Admission {
                mind: [2; 32],
                world: String::new(),
                sigil: String::new(),
                handle: String::new(),
                tiers: Vec::new(),
                refused: Some(crate::iface::NO_VIABLE_WORLD),
            },
        ];
        write_admissions(&var, &all).expect("written");
        assert_eq!(admissions(&var), all);
        let _ = std::fs::remove_dir_all(var);
    }

    #[test]
    fn releases_and_sandboxes_are_recorded_and_cleared() {
        let var = scratch("release");
        write_release(&var, &[4; 32]).expect("written");
        assert_eq!(releases(&var), vec![[4; 32]]);
        released(&var, &[4; 32]).expect("cleared");
        assert!(releases(&var).is_empty());
        write_sandbox(&var, &[5; 32], &[6; 32]).expect("written");
        assert_eq!(sandboxes(&var), vec![([5; 32], [6; 32])]);
        sandbox_done(&var, &[5; 32], &[6; 32], true).expect("done");
        assert!(sandboxes(&var).is_empty());
        assert_eq!(sandboxes_done(&var), vec![([5; 32], [6; 32], true)]);
        let _ = std::fs::remove_dir_all(var);
    }
}
