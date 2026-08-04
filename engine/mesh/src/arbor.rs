//! THE ARBOR — the container of spines (operator ruling 2026-08-01).
//!
//! **Why the name moved.** This type was called `Spine`, and a spine is
//! one line: ordered, load-bearing, one writer per clock. That is
//! exactly right for a BRANCH and exactly wrong for the object holding
//! them. `branch()` mints a new line at a fork position and `head(name)`
//! serves each separately, so one of these has always held several
//! spines at once — the type was named for its part rather than for
//! itself.
//!
//! An ARBOR holds many spines. It forks and it does not retract: every
//! branch persists, which is the bifurcation law in the storage layer —
//! both sides of a cleave become roots and neither is discarded.
//!
//! **"Chain" was refused for the same reason.** A chain means one linear
//! history, one winner at each height, and longest-chain arbitration.
//! Convergence here is additive rather than subtractive, divergence
//! keeps both peers, and disagreement is held rather than voted away.
//! The word would have smuggled all three back in — the same way
//! "capacity" smuggled a ceiling and "converged" smuggled minimisation.
//!
//! **A spine is still a spine.** Within one branch the structure is
//! linear and ordered and the word is accurate; `Coordinate::branch`
//! names which spine a commitment sits on.
//!
//! **KNOWN GAP: an arbor can fork and cannot JOIN.** There is no merge
//! here. Today it is a semilattice — branches only ever fan outward —
//! so convergence, the whole value thesis, has no landing site in the
//! commitment structure. Naming it for branching alone would bless the
//! half that exists; it is named for the whole it must become.
//!
//! **On disk the directory is still `spine/`.** Renaming it would orphan
//! every live world's committed history, so the path is a MIGRATION to
//! schedule, not a naming decision. Recorded rather than silently split.

use cfe::dop::ClosureEvent;
use cfe::ratio::{self, Ratio};
use cfe::tlv;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

/// Tag 84 — the arbor commitment record (the one authority).
pub const SPINE_RECORD_TAG: u8 = 84;
/// Tag 87 — a SEGMENT blob frame (SPINE-UPLINK): `state[32] ‖ bytes`,
/// carried in a replication segment, re-hashed on arrival. Never
/// stored in `arbor.tlv` — the store already holds blobs by hash.
pub const SEGMENT_BLOB_TAG: u8 = 87;
/// Tag 88 — a SEGMENT step frame (SPINE-UPLINK): `commit[32] ‖
/// actor[8] ‖ stamp[8] ‖ event_bytes`, the proof a JUDGED record's
/// transition was lawful, re-judged under the receiver's kernel.
pub const SEGMENT_STEP_TAG: u8 = 88;

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
    /// commit and used whole as the arbor map's key.
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

/// One arbor commitment record (tag 84): one state bound to one
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

/// The typed arbor refusal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArborRefused {
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
    /// A second pass through the adoption door: the mainline already
    /// stands, and adoption happens once (one truth, one birth).
    AlreadyAdopted,
    /// A step offered at genesis: nothing prior exists to judge from.
    GenesisJudged,
    StoreMissing,
    Refused(kernel::Refused),
    /// SPINE-UPLINK: a received record claims `judged` but its step
    /// did not ride the segment — the receiver cannot reproduce the
    /// kernel verdict, so it will not admit the claim. Admission is
    /// re-judging, never trust; a judgment you cannot re-run is a
    /// judgment you do not have.
    JudgedWithoutProof { frame: usize },
    /// SPINE-UPLINK: a received record references a blob absent from
    /// the segment and from the local store — the geometry is unheld,
    /// the record unverifiable.
    BlobUnheld { frame: usize },
}

fn io(error: std::io::Error) -> ArborRefused {
    ArborRefused::Io { message: error.to_string() }
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
    /// the arbor's job on open — the wire only carries.
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

/// The arbor at a root directory: `arbor.tlv` (the record stream) and
/// `store/` (the content-addressed geometry).
#[derive(Debug)]
pub struct Arbor {
    root: PathBuf,
    records: Vec<Commitment>,
}

impl Arbor {
    /// Open (or create) the arbor at `root`. Every record recomputes,
    /// every parent links, the arrow ascends per branch, and every
    /// address holds one value — or the whole file refuses typed.
    pub fn open(root: impl AsRef<Path>) -> Result<Self, ArborRefused> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(root.join("store")).map_err(io)?;
        let stream_path = root.join("arbor.tlv");
        let mut records: Vec<Commitment> = Vec::new();
        if stream_path.exists() {
            let bytes = fs::read(&stream_path).map_err(io)?;
            let mut heads: HashMap<String, usize> = HashMap::new();
            let mut addresses: HashSet<Vec<u8>> = HashSet::new();
            let mut commits: HashMap<[u8; 32], usize> = HashMap::new();
            for (frame, (tag, value)) in tlv::records(&bytes).enumerate() {
                let record = Commitment::from_wire(tag, value)
                    .ok_or(ArborRefused::Malformed { frame })?;
                let resealed = seal(
                    record.coordinate.clone(),
                    record.schema,
                    record.kernel,
                    record.judged,
                    record.state,
                    record.parent,
                );
                if resealed.commit != record.commit {
                    return Err(ArborRefused::Forged { frame });
                }
                let branch = record.coordinate.branch.clone();
                match heads.get(&branch) {
                    Some(&head_at) => {
                        let head = &records[head_at];
                        if record.parent != head.commit {
                            return Err(ArborRefused::Forged { frame });
                        }
                        if record.coordinate.t <= head.coordinate.t {
                            return Err(ArborRefused::ArrowBroken {
                                t: record.coordinate.t,
                                head: head.coordinate.t,
                            });
                        }
                    }
                    None if branch.is_empty() => {
                        if record.parent != [0u8; 32] {
                            return Err(ArborRefused::Forged { frame });
                        }
                    }
                    None => {
                        // A named branch is born as a genesis-copy of
                        // its fork: same position, same state, parent
                        // the fork's commitment (NS-5 §9.2).
                        let fork_at = *commits
                            .get(&record.parent)
                            .ok_or(ArborRefused::Forged { frame })?;
                        let fork = &records[fork_at];
                        let copies = fork.coordinate.s == record.coordinate.s
                            && fork.coordinate.g == record.coordinate.g
                            && fork.coordinate.t == record.coordinate.t
                            && fork.state == record.state;
                        if !copies {
                            return Err(ArborRefused::Forged { frame });
                        }
                    }
                }
                if !addresses.insert(record.coordinate.bytes()) {
                    return Err(ArborRefused::Occupied);
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

    /// The verified records, in stream order — a LENS over the open
    /// arbor (read-only; every record already recomputed at `open`).
    pub fn records(&self) -> &[Commitment] {
        &self.records
    }

    /// The branch's latest commitment.
    pub fn head(&self, branch: &str) -> Option<&Commitment> {
        self.records.iter().rev().find(|record| record.coordinate.branch == branch)
    }

    /// The geometry, verified on arrival: a tampered blob refuses.
    pub fn fetch(&self, state: &[u8; 32]) -> Result<Vec<u8>, ArborRefused> {
        let path = self.blob_path(state);
        if !path.exists() {
            return Err(ArborRefused::StoreMissing);
        }
        let bytes = fs::read(&path).map_err(io)?;
        match blake3::hash(&bytes).as_bytes() == state {
            true => Ok(bytes),
            false => Err(ArborRefused::Forged { frame: 0 }),
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
    ) -> Result<&Commitment, ArborRefused> {
        let next = codec::from_state_bytes(state_bytes)
            .ok_or(ArborRefused::StateUndecodable)?;
        // The record repeats the blob's leading byte — the codec owns
        // the form; the arbor never asserts its own schema constant.
        let schema = *state_bytes.first().ok_or(ArborRefused::StateUndecodable)?;
        if self.resolve(&coordinate).is_some() {
            return Err(ArborRefused::Occupied);
        }
        let parent = match self.head(&coordinate.branch) {
            Some(head) => {
                if coordinate.t <= head.coordinate.t {
                    return Err(ArborRefused::ArrowBroken {
                        t: coordinate.t,
                        head: head.coordinate.t,
                    });
                }
                if let Some(step) = step {
                    let prior_bytes = self.fetch(&head.state)?;
                    let prior = codec::from_state_bytes(&prior_bytes)
                        .ok_or(ArborRefused::StateUndecodable)?;
                    kernel::admits(&prior, &next, step).map_err(ArborRefused::Refused)?;
                }
                head.commit
            }
            None if coordinate.branch.is_empty() => {
                if step.is_some() {
                    return Err(ArborRefused::GenesisJudged);
                }
                [0u8; 32]
            }
            None => {
                return Err(ArborRefused::BranchUnknown { name: coordinate.branch });
            }
        };
        let state = *blake3::hash(state_bytes).as_bytes();
        let blob = self.blob_path(&state);
        if !blob.exists() {
            fs::write(&blob, state_bytes).map_err(io)?;
        }
        let record = seal(
            coordinate,
            schema,
            kernel::VERSION,
            step.is_some(),
            state,
            parent,
        );
        self.append(record)
    }

    /// THE ADOPTION DOOR (#36): a pre-chain world enters the mainline
    /// ONCE — its current state seated as the unjudged genesis, the
    /// debt stamped `judged = false` (history before the chain is
    /// NAMED, never hidden). After adoption every step arrives through
    /// [`Arbor::extend`], judged where covered. A second adoption
    /// refuses: one truth, one birth — re-entry through this door
    /// would be a mirror made of genesis records.
    pub fn adopt(
        &mut self,
        coordinate: Coordinate,
        state_bytes: &[u8],
    ) -> Result<&Commitment, ArborRefused> {
        if !coordinate.branch.is_empty() {
            // Nurseries are born by `branch`, never adopted.
            return Err(ArborRefused::BranchUnknown { name: coordinate.branch });
        }
        if self.head("").is_some() {
            return Err(ArborRefused::AlreadyAdopted);
        }
        self.extend(coordinate, state_bytes, None)
    }

    /// The nursery opens (NS-5 §7.6/§9.2): a genesis-copy record at
    /// the fork position under the new branch name — the same state
    /// entering a new address space, parent-linked to its fork.
    pub fn branch(&mut self, from: &Coordinate, name: &str) -> Result<&Commitment, ArborRefused> {
        if name.is_empty() || self.head(name).is_some() {
            return Err(ArborRefused::BranchTaken { name: name.into() });
        }
        let fork = self.resolve(from).ok_or(ArborRefused::ForkUnknown)?;
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

    /// PRESENT (SPINE-UPLINK, NS-5 §9): this arbor as a replication
    /// SEGMENT — every record (tag 84) followed by each unique
    /// referenced blob (tag 87, `state ‖ bytes`). Only chain-verified
    /// bytes ride (THE UPLINK LAW): records recompute, blobs are
    /// content-addressed. NOTE: a JUDGED record needs its step (tag 88)
    /// to be re-judged on arrival, and the arbor stores NO steps — so a
    /// arbor carrying judged records is not fully presentable through
    /// this door alone (the Phase-3 seam: per-deed threading brings the
    /// step storage that makes judged records re-presentable). Today
    /// every record is `judged = false`, so `present` is complete.
    pub fn present(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for record in &self.records {
            out.extend_from_slice(&record.wire_bytes());
        }
        let mut seen: HashSet<[u8; 32]> = HashSet::new();
        for record in &self.records {
            if seen.insert(record.state) {
                if let Ok(bytes) = self.fetch(&record.state) {
                    let mut frame = record.state.to_vec();
                    frame.extend_from_slice(&bytes);
                    tlv::put(SEGMENT_BLOB_TAG, &frame, &mut out);
                }
            }
        }
        out
    }

    /// RECEIVE (SPINE-UPLINK): admit a replication segment onto THIS
    /// arbor. Admission IS verification — the receiver trusts nothing:
    /// every record must reseal (its commit recomputes), extend this
    /// arbor's own head lawfully (parent links, the arrow ascends, the
    /// address is fresh), carry a hash-matched blob that decodes as
    /// committed state, and — if it claims `judged` — re-run
    /// `kernel::admits` under this node's kernel from the carried step.
    /// The first break refuses typed; nothing partial is admitted past
    /// it. Returns the count admitted.
    pub fn receive(&mut self, segment: &[u8]) -> Result<usize, ArborRefused> {
        let mut incoming: Vec<Commitment> = Vec::new();
        let mut blobs: HashMap<[u8; 32], Vec<u8>> = HashMap::new();
        let mut steps: HashMap<[u8; 32], kernel::Step> = HashMap::new();
        for (frame, (tag, value)) in tlv::records(segment).enumerate() {
            match tag {
                SPINE_RECORD_TAG => {
                    let record = Commitment::from_wire(tag, value)
                        .ok_or(ArborRefused::Malformed { frame })?;
                    incoming.push(record);
                }
                SEGMENT_BLOB_TAG => {
                    let state: [u8; 32] = value
                        .get(..32)
                        .and_then(|s| s.try_into().ok())
                        .ok_or(ArborRefused::Malformed { frame })?;
                    let bytes = value[32..].to_vec();
                    if blake3::hash(&bytes).as_bytes() != &state {
                        return Err(ArborRefused::Forged { frame });
                    }
                    blobs.insert(state, bytes);
                }
                SEGMENT_STEP_TAG => {
                    let commit: [u8; 32] = value
                        .get(..32)
                        .and_then(|s| s.try_into().ok())
                        .ok_or(ArborRefused::Malformed { frame })?;
                    let actor = u64::from_le_bytes(
                        value.get(32..40).and_then(|s| s.try_into().ok())
                            .ok_or(ArborRefused::Malformed { frame })?,
                    );
                    let stamp = u64::from_le_bytes(
                        value.get(40..48).and_then(|s| s.try_into().ok())
                            .ok_or(ArborRefused::Malformed { frame })?,
                    );
                    let (etag, eval) = tlv::records(value.get(48..).unwrap_or_default())
                        .next()
                        .ok_or(ArborRefused::Malformed { frame })?;
                    let event = ClosureEvent::from_wire(etag, eval)
                        .ok_or(ArborRefused::Malformed { frame })?;
                    steps.insert(commit, kernel::Step { event, actor, stamp });
                }
                // Judge-side: an unknown frame in a segment REFUSES
                // (never skip — a skipped frame is an unadmitted truth).
                _ => return Err(ArborRefused::Malformed { frame }),
            }
        }
        let mut admitted = 0usize;
        for (frame, record) in incoming.into_iter().enumerate() {
            if self.admit(record, frame, &blobs, &steps)? {
                admitted += 1;
            }
        }
        Ok(admitted)
    }

    /// Admit ONE received record onto this arbor — the trustless core
    /// of `receive`, in order (reseal → geometry → chain-extension →
    /// re-judge → append).
    fn admit(
        &mut self,
        record: Commitment,
        frame: usize,
        blobs: &HashMap<[u8; 32], Vec<u8>>,
        steps: &HashMap<[u8; 32], kernel::Step>,
    ) -> Result<bool, ArborRefused> {
        // Idempotent replication: a record I already hold is a no-op
        // (a peer may re-present its whole arbor); the SAME commit at
        // its address skips, a DIFFERENT commit is a fork attempt.
        if let Some(existing) = self.resolve(&record.coordinate) {
            return match existing.commit == record.commit {
                true => Ok(false),
                false => Err(ArborRefused::Occupied),
            };
        }
        // 1. Reseal — the commit must recompute from its own fields.
        let resealed = seal(
            record.coordinate.clone(),
            record.schema,
            record.kernel,
            record.judged,
            record.state,
            record.parent,
        );
        if resealed.commit != record.commit {
            return Err(ArborRefused::Forged { frame });
        }
        // 2. Geometry — the blob is held (segment or local store),
        //    hash-matched, decodes, and its leading byte IS the schema.
        let state_bytes = match blobs.get(&record.state) {
            Some(bytes) => bytes.clone(),
            None => self.fetch(&record.state).map_err(|_| ArborRefused::BlobUnheld { frame })?,
        };
        if blake3::hash(&state_bytes).as_bytes() != &record.state {
            return Err(ArborRefused::Forged { frame });
        }
        if state_bytes.first() != Some(&record.schema) {
            return Err(ArborRefused::Forged { frame });
        }
        let next = codec::from_state_bytes(&state_bytes)
            .ok_or(ArborRefused::StateUndecodable)?;
        // 3. Chain-extension — the record extends THIS arbor's own head.
        let branch = record.coordinate.branch.clone();
        let prior_state = self.head(&branch).map(|head| head.state);
        match self.head(&branch) {
            Some(head) => {
                if record.parent != head.commit {
                    return Err(ArborRefused::Forged { frame });
                }
                if record.coordinate.t <= head.coordinate.t {
                    return Err(ArborRefused::ArrowBroken {
                        t: record.coordinate.t,
                        head: head.coordinate.t,
                    });
                }
            }
            None if branch.is_empty() => {
                if record.parent != [0u8; 32] {
                    return Err(ArborRefused::Forged { frame });
                }
            }
            None => {
                // A branch-birth: the parent is a known local commit and
                // the record copies the fork's position and state.
                let fork = self
                    .records
                    .iter()
                    .find(|held| held.commit == record.parent)
                    .ok_or(ArborRefused::Forged { frame })?;
                let copies = fork.coordinate.s == record.coordinate.s
                    && fork.coordinate.g == record.coordinate.g
                    && fork.coordinate.t == record.coordinate.t
                    && fork.state == record.state;
                if !copies {
                    return Err(ArborRefused::Forged { frame });
                }
            }
        }
        // 4. Re-judge — a judged record's transition re-runs `lawful`
        //    under THIS kernel from the carried step, or it is refused:
        //    a judgment the receiver cannot reproduce is not admitted.
        if record.judged {
            let step = steps
                .get(&record.commit)
                .ok_or(ArborRefused::JudgedWithoutProof { frame })?;
            let prior_state = prior_state.ok_or(ArborRefused::GenesisJudged)?;
            let prior_bytes = self.fetch(&prior_state)?;
            let prior = codec::from_state_bytes(&prior_bytes)
                .ok_or(ArborRefused::StateUndecodable)?;
            kernel::admits(&prior, &next, step).map_err(ArborRefused::Refused)?;
        }
        // 5. Admit — write the blob, append the record VERBATIM (its
        //    commit verified to reseal: the two spines agree byte-exact).
        let blob = self.blob_path(&record.state);
        if !blob.exists() {
            fs::write(&blob, &state_bytes).map_err(io)?;
        }
        self.append(record)?;
        Ok(true)
    }

    fn append(&mut self, record: Commitment) -> Result<&Commitment, ArborRefused> {
        let mut stream = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.root.join("arbor.tlv"))
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
