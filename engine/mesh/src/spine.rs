//! THE SPINE (NS-5 §9) — the chain bound to the real clock. State is
//! position, the kernel is motion, the spine is the recorded path.
//!
//! Two maps over one append-only record stream plus a
//! content-addressed store:
//!
//! - **the spine map** — coordinate → commitment, single-valued
//!   structurally: T strictly ascends per branch (the clock only
//!   walks forward), so a second value at an occupied address cannot
//!   be appended, and a forged one refuses the whole file on open;
//! - **the store** — state digest → canonical bytes; a blob cannot be
//!   wrong, only absent, and `fetch` re-hashes on every read.
//!
//! The four verbs: `resolve` (where the universe stands), `fetch`
//! (the geometry, verified on arrival), `extend` (the one write —
//! the kernel judges across the seam when a step is offered, and the
//! judged flag is STAMPED into the commitment either way), `branch`
//! (the nursery opens: the same state entering a new address space,
//! parent-linked to the mainline — NS-5 §7.6).
//!
//! This file is judge-side: an unknown tag or kind in `spine.tlv`
//! REFUSES — SEAL A is a reader's law, never the judge's. The kernel
//! is consumed, never owned: `kernel::VERSION` binds into every
//! commitment; the judge never reads the spine.

use cfe::ratio::{self, Ratio};
use cfe::tlv;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

/// Tag 84 — the spine commitment record (the one authority).
pub const SPINE_RECORD_TAG: u8 = 84;

/// A position on the one clock (NS-5 §9.1): the nursery branch (""
/// is the mainline), the wrapped torus triple, the gauge sheet, the
/// attested tick.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coordinate {
    pub branch: String,
    pub s: [Ratio; 3],
    pub g: Ratio,
    pub t: u64,
}

impl Coordinate {
    /// The canonical coordinate bytes — the address, hashed into the
    /// commit and used whole as the spine map's key.
    fn bytes(&self) -> Vec<u8> {
        let mut out = (self.branch.len() as u16).to_le_bytes().to_vec();
        out.extend_from_slice(self.branch.as_bytes());
        for axis in &self.s {
            ratio::encode(axis, &mut out);
        }
        ratio::encode(&self.g, &mut out);
        out.extend_from_slice(&self.t.to_le_bytes());
        out
    }
}

/// One spine commitment record (tag 84): one state bound to one
/// position, forever.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commitment {
    pub coordinate: Coordinate,
    pub schema: u8,
    pub kernel: u64,
    pub judged: bool,
    pub state: [u8; 32],
    pub parent: [u8; 32],
    pub commit: [u8; 32],
}

/// The typed spine refusal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpineRefused {
    Io { message: String },
    /// A frame that is not a well-formed tag-84 record — judge-side,
    /// unknown REFUSES.
    Malformed { frame: usize },
    /// A record whose commit does not recompute, a broken parent
    /// link, or a tampered store blob.
    Forged { frame: usize },
    Occupied,
    ArrowBroken { t: u64, head: u64 },
    BranchUnknown { name: String },
    BranchTaken { name: String },
    ForkUnknown,
    StateUndecodable,
    /// A step offered at genesis: nothing prior exists to judge from.
    GenesisJudged,
    StoreMissing,
    Unlawful(kernel::Unlawful),
}

fn io(error: std::io::Error) -> SpineRefused {
    SpineRefused::Io { message: error.to_string() }
}

/// Seal a commitment: compute its commit hash from its fields —
/// `BLAKE3(coordinate ‖ schema ‖ kernel ‖ judged ‖ state ‖ parent)`.
pub fn seal(
    coordinate: Coordinate,
    schema: u8,
    kernel: u64,
    judged: bool,
    state: [u8; 32],
    parent: [u8; 32],
) -> Commitment {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&coordinate.bytes());
    hasher.update(&[schema]);
    hasher.update(&kernel.to_le_bytes());
    hasher.update(&[judged as u8]);
    hasher.update(&state);
    hasher.update(&parent);
    let commit = *hasher.finalize().as_bytes();
    Commitment { coordinate, schema, kernel, judged, state, parent, commit }
}

impl Commitment {
    /// One TLV record, tag 84: `coordinate ‖ schema ‖ LE64(kernel) ‖
    /// judged ‖ state[32] ‖ parent[32] ‖ commit[32]`.
    pub fn wire_bytes(&self) -> Vec<u8> {
        let mut value = self.coordinate.bytes();
        value.push(self.schema);
        value.extend_from_slice(&self.kernel.to_le_bytes());
        value.push(self.judged as u8);
        value.extend_from_slice(&self.state);
        value.extend_from_slice(&self.parent);
        value.extend_from_slice(&self.commit);
        let mut out = Vec::new();
        tlv::put(SPINE_RECORD_TAG, &value, &mut out);
        out
    }

    /// Parse one record; verification (commit recompute, linkage) is
    /// the spine's job on open — the wire only carries.
    pub fn from_wire(tag: u8, value: &[u8]) -> Option<Self> {
        if tag != SPINE_RECORD_TAG {
            return None;
        }
        let branch_len = u16::from_le_bytes(value.get(..2)?.try_into().ok()?) as usize;
        let branch = String::from_utf8(value.get(2..2 + branch_len)?.to_vec()).ok()?;
        let mut at = 2 + branch_len;
        let take_ratio = |at: &mut usize| -> Option<Ratio> {
            let (held, used) = ratio::decode(value.get(*at..)?)?;
            *at += used;
            Some(held)
        };
        let s = [take_ratio(&mut at)?, take_ratio(&mut at)?, take_ratio(&mut at)?];
        let g = take_ratio(&mut at)?;
        let t = u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?);
        at += 8;
        let schema = *value.get(at)?;
        at += 1;
        let kernel = u64::from_le_bytes(value.get(at..at + 8)?.try_into().ok()?);
        at += 8;
        let judged = match *value.get(at)? {
            0 => false,
            1 => true,
            _ => return None,
        };
        at += 1;
        let state: [u8; 32] = value.get(at..at + 32)?.try_into().ok()?;
        at += 32;
        let parent: [u8; 32] = value.get(at..at + 32)?.try_into().ok()?;
        at += 32;
        let commit: [u8; 32] = value.get(at..at + 32)?.try_into().ok()?;
        at += 32;
        if at != value.len() {
            return None;
        }
        Some(Commitment { coordinate: Coordinate { branch, s, g, t }, schema, kernel, judged, state, parent, commit })
    }
}

/// The spine at a root directory: `spine.tlv` (the record stream) and
/// `store/` (the content-addressed geometry).
#[derive(Debug)]
pub struct Spine {
    root: PathBuf,
    records: Vec<Commitment>,
}

impl Spine {
    /// Open (or create) the spine at `root`. Every record recomputes,
    /// every parent links, the arrow ascends per branch, and every
    /// address holds one value — or the whole file refuses typed.
    pub fn open(root: impl AsRef<Path>) -> Result<Self, SpineRefused> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(root.join("store")).map_err(io)?;
        let stream_path = root.join("spine.tlv");
        let mut records: Vec<Commitment> = Vec::new();
        if stream_path.exists() {
            let bytes = fs::read(&stream_path).map_err(io)?;
            let mut heads: HashMap<String, usize> = HashMap::new();
            let mut addresses: HashSet<Vec<u8>> = HashSet::new();
            let mut commits: HashMap<[u8; 32], usize> = HashMap::new();
            for (frame, (tag, value)) in tlv::records(&bytes).enumerate() {
                let record = Commitment::from_wire(tag, value)
                    .ok_or(SpineRefused::Malformed { frame })?;
                let resealed = seal(
                    record.coordinate.clone(),
                    record.schema,
                    record.kernel,
                    record.judged,
                    record.state,
                    record.parent,
                );
                if resealed.commit != record.commit {
                    return Err(SpineRefused::Forged { frame });
                }
                let branch = record.coordinate.branch.clone();
                match heads.get(&branch) {
                    Some(&head_at) => {
                        let head = &records[head_at];
                        if record.parent != head.commit {
                            return Err(SpineRefused::Forged { frame });
                        }
                        if record.coordinate.t <= head.coordinate.t {
                            return Err(SpineRefused::ArrowBroken {
                                t: record.coordinate.t,
                                head: head.coordinate.t,
                            });
                        }
                    }
                    None if branch.is_empty() => {
                        if record.parent != [0u8; 32] {
                            return Err(SpineRefused::Forged { frame });
                        }
                    }
                    None => {
                        // A named branch is born as a genesis-copy of
                        // its fork: same position, same state, parent
                        // the fork's commitment (NS-5 §9.2).
                        let fork_at = *commits
                            .get(&record.parent)
                            .ok_or(SpineRefused::Forged { frame })?;
                        let fork = &records[fork_at];
                        let copies = fork.coordinate.s == record.coordinate.s
                            && fork.coordinate.g == record.coordinate.g
                            && fork.coordinate.t == record.coordinate.t
                            && fork.state == record.state;
                        if !copies {
                            return Err(SpineRefused::Forged { frame });
                        }
                    }
                }
                if !addresses.insert(record.coordinate.bytes()) {
                    return Err(SpineRefused::Occupied);
                }
                commits.insert(record.commit, records.len());
                heads.insert(branch, records.len());
                records.push(record);
            }
        }
        Ok(Self { root, records })
    }

    /// Where the universe stands at one coordinate.
    pub fn resolve(&self, coordinate: &Coordinate) -> Option<&Commitment> {
        self.records.iter().find(|record| &record.coordinate == coordinate)
    }

    /// The branch's latest commitment.
    pub fn head(&self, branch: &str) -> Option<&Commitment> {
        self.records.iter().rev().find(|record| record.coordinate.branch == branch)
    }

    /// The geometry, verified on arrival: a tampered blob refuses.
    pub fn fetch(&self, state: &[u8; 32]) -> Result<Vec<u8>, SpineRefused> {
        let path = self.blob_path(state);
        if !path.exists() {
            return Err(SpineRefused::StoreMissing);
        }
        let bytes = fs::read(&path).map_err(io)?;
        match blake3::hash(&bytes).as_bytes() == state {
            true => Ok(bytes),
            false => Err(SpineRefused::Forged { frame: 0 }),
        }
    }

    /// The one write (NS-5 §9.3): the arrow holds, the address is
    /// fresh, the state decodes, and — when a step is offered — the
    /// kernel judges across the seam. The judged flag is stamped into
    /// the commitment either way; the coverage hole is visible, never
    /// hidden.
    pub fn extend(
        &mut self,
        coordinate: Coordinate,
        state_bytes: &[u8],
        step: Option<&kernel::Step>,
    ) -> Result<&Commitment, SpineRefused> {
        let next = codec::from_state_bytes(state_bytes)
            .ok_or(SpineRefused::StateUndecodable)?;
        if self.resolve(&coordinate).is_some() {
            return Err(SpineRefused::Occupied);
        }
        let parent = match self.head(&coordinate.branch) {
            Some(head) => {
                if coordinate.t <= head.coordinate.t {
                    return Err(SpineRefused::ArrowBroken {
                        t: coordinate.t,
                        head: head.coordinate.t,
                    });
                }
                if let Some(step) = step {
                    let prior_bytes = self.fetch(&head.state)?;
                    let prior = codec::from_state_bytes(&prior_bytes)
                        .ok_or(SpineRefused::StateUndecodable)?;
                    kernel::lawful(&prior, &next, step).map_err(SpineRefused::Unlawful)?;
                }
                head.commit
            }
            None if coordinate.branch.is_empty() => {
                if step.is_some() {
                    return Err(SpineRefused::GenesisJudged);
                }
                [0u8; 32]
            }
            None => {
                return Err(SpineRefused::BranchUnknown { name: coordinate.branch });
            }
        };
        let state = *blake3::hash(state_bytes).as_bytes();
        let blob = self.blob_path(&state);
        if !blob.exists() {
            fs::write(&blob, state_bytes).map_err(io)?;
        }
        let record = seal(
            coordinate,
            codec::SCHEMA_V1,
            kernel::VERSION,
            step.is_some(),
            state,
            parent,
        );
        self.append(record)
    }

    /// The nursery opens (NS-5 §7.6/§9.2): a genesis-copy record at
    /// the fork position under the new branch name — the same state
    /// entering a new address space, parent-linked to its fork.
    pub fn branch(&mut self, from: &Coordinate, name: &str) -> Result<&Commitment, SpineRefused> {
        if name.is_empty() || self.head(name).is_some() {
            return Err(SpineRefused::BranchTaken { name: name.into() });
        }
        let fork = self.resolve(from).ok_or(SpineRefused::ForkUnknown)?;
        let record = seal(
            Coordinate {
                branch: name.into(),
                s: fork.coordinate.s.clone(),
                g: fork.coordinate.g.clone(),
                t: fork.coordinate.t,
            },
            fork.schema,
            kernel::VERSION,
            false,
            fork.state,
            fork.commit,
        );
        self.append(record)
    }

    fn append(&mut self, record: Commitment) -> Result<&Commitment, SpineRefused> {
        let mut stream = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.root.join("spine.tlv"))
            .map_err(io)?;
        stream.write_all(&record.wire_bytes()).map_err(io)?;
        stream.flush().map_err(io)?;
        self.records.push(record);
        Ok(self.records.last().expect("just appended"))
    }

    fn blob_path(&self, state: &[u8; 32]) -> PathBuf {
        let hex: String = state.iter().map(|byte| format!("{byte:02x}")).collect();
        self.root.join("store").join(format!("{hex}.bin"))
    }
}
