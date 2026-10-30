//! The arbor — the container of spines (operator ruling 2026-08-01).
//!
//! **Why the name moved.** This type was called `Spine`, and a spine is
//! one line: ordered, load-bearing, one writer per clock. That is
//! exactly right for a branch and exactly wrong for the object holding
//! them. `branch()` mints a new line at a fork position and `head(name)`
//! serves each separately, so one of these has always held several
//! spines at once — the type was named for its part rather than for
//! itself.
//!
//! An arbor holds many spines. It forks and it does not retract: every
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
//! **KNOWN gap: an arbor can fork and cannot join.** There is no merge
//! here. Today it is a semilattice — branches only ever fan outward —
//! so convergence, the whole value thesis, has no landing site in the
//! commitment structure. Naming it for branching alone would bless the
//! half that exists; it is named for the whole it must become.
//!
//! **On disk the directory is still `spine/`.** Renaming it would orphan
//! every live world's committed history, so the path is a migration to
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
/// Tag 87 — a segment blob frame (SPINE-UPLINK): `state[32] ‖ bytes`,
/// carried in a replication segment, re-hashed on arrival. Never
/// stored in `arbor.tlv` — the store already holds blobs by hash.
pub const SEGMENT_BLOB_TAG: u8 = 87;
/// Tag 88 — a segment step frame (SPINE-UPLINK): `commit[32] ‖
/// actor[8] ‖ stamp[8] ‖ event_bytes`, the proof a judged record's
/// transition was lawful, re-gated under the receiver's kernel.
pub const SEGMENT_STEP_TAG: u8 = 88;

/// A position on the one clock (NS-5 §9.1): the nursery branch (""
/// is the mainline), the wrapped torus triple, the gauge sheet, the
/// attested tick, and the sub-tick deed index.
///
/// **`sub` — The PHASE-3 flip** (`39_PHASE3_FLIP` §3, operator ruling
/// 2026-08-13, option A). Several deeds share one world tick, and
/// judgment is per-deed while the old coordinate addressed per-tick —
/// a second deed's commitment refused `Occupied` on contact (measured).
/// `sub` is the deed index within one tick, hashed into every commit
/// like every other field: one commitment per deed, each carrying its
/// own kernel verdict. Resets to 0 whenever `t` advances; within one
/// `t`, `sub` counts 0, 1, 2… in deed order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coordinate {
    pub branch: String,
    pub s: [Ratio; 3],
    pub g: Ratio,
    pub t: u64,
    pub sub: u32,
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
        out.extend_from_slice(&self.sub.to_le_bytes());
        out
    }
}

/// The arrow, ascending on `(t, sub)` — the Phase-3 addressing law
/// (`39_PHASE3_FLIP` §3). A later commit on one branch must be later in
/// (tick, sub-tick) lexicographic order: `t` strictly ahead, or the
/// same `t` with `sub` strictly ahead — a fresh tick always resets
/// `sub`'s meaning, since it addresses that tick's deeds only.
fn ascends(next: &Coordinate, head: &Coordinate) -> bool {
    (next.t, next.sub) > (head.t, head.sub)
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
    /// A frame that is not a well-formed tag-84 record — gate-side,
    /// unknown refuses.
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
    /// A step offered at genesis: nothing prior exists to gate from.
    GenesisJudged,
    StoreMissing,
    Refused(kernel::Refused),
    /// SPINE-UPLINK: a received record claims `gated` but its step
    /// did not ride the segment — the receiver cannot reproduce the
    /// kernel verdict, so it will not admit the claim. Admission is
    /// re-judging, never trust; a judgment that cannot be re-run is
    /// a judgment not held.
    JudgedWithoutProof { frame: usize },
    /// [`Arbor::extend_judged`]: the judge's prior state is not this
    /// branch's head (its digest differs) — the verdict was reached on a
    /// state the arbor does not hold, so it does not carry here.
    PriorDiverged,
    /// SPINE-UPLINK: a received record references a blob absent from
    /// the segment and from the local store — the geometry is unheld,
    /// the record unverifiable.
    BlobUnheld { frame: usize },
}

fn io(error: std::io::Error) -> ArborRefused {
    ArborRefused::Io { message: error.to_string() }
}

/// Seal a commitment: compute its commit hash from its fields —
/// `BLAKE3(coordinate ‖ schema ‖ kernel ‖ gated ‖ state ‖ parent)`.
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
    /// gated ‖ state[32] ‖ parent[32] ‖ commit[32]`.
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
        let sub = u32::from_le_bytes(value.get(at..at + 4)?.try_into().ok()?);
        at += 4;
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
        Some(Commitment { coordinate: Coordinate { branch, s, g, t, sub }, schema, kernel, judged, state, parent, commit })
    }
}

/// The arbor at a root directory: `arbor.tlv` (the record stream) and
/// `store/` (the content-addressed geometry).
#[derive(Debug)]
pub struct Arbor {
    root: PathBuf,
    records: Vec<Commitment>,
    /// The memory bound (opened with [`Arbor::open_heads`]): `records` holds
    /// only each branch's head, verified whole at open; appends replace the
    /// head. For the live judge, which reads nothing but heads.
    heads_only: bool,
}

impl Arbor {
    /// Open (or create) the arbor at `root`. Every record recomputes,
    /// every parent links, the arrow ascends per branch, and every
    /// address holds one value — or the whole file refuses typed.
    pub fn open(root: impl AsRef<Path>) -> Result<Self, ArborRefused> {
        Self::open_with(root, false)
    }

    /// [`Arbor::open`] verifying the whole file exactly as ever, then keeping
    /// only each branch's head in memory (the memory bound: a live judge reads
    /// heads alone). `resolve` and `records` then see heads only.
    pub fn open_heads(root: impl AsRef<Path>) -> Result<Self, ArborRefused> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(root.join("store")).map_err(io)?;
        let stream_path = root.join("arbor.tlv");
        if !stream_path.exists() {
            return Ok(Self { root, records: Vec::new(), heads_only: true });
        }
        // Streamed, frame by frame: every record recomputes, its parent is
        // its branch's head, and the arrow ascends — the same checks as
        // `open`, holding one head per branch and never the file. (Within a
        // branch the strict arrow makes every address distinct; a branch's
        // name is part of its address.) A named branch's birth needs its
        // fork's record, which a stream no longer holds: such an arbor is
        // opened whole instead.
        use std::io::Read;
        let file = fs::File::open(&stream_path).map_err(io)?;
        let mut remaining = file.metadata().map_err(io)?.len();
        let mut reader = std::io::BufReader::new(file);
        let mut heads: Vec<Commitment> = Vec::new();
        let mut frame = 0usize;
        loop {
            let mut header = [0u8; 5];
            match reader.read_exact(&mut header) {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(io(e)),
            }
            let len = u32::from_le_bytes(header[1..5].try_into().expect("four bytes")) as u64;
            // A length is never trusted past what the file holds.
            remaining = remaining.saturating_sub(5);
            if len > remaining {
                return Err(ArborRefused::Malformed { frame });
            }
            remaining -= len;
            let mut value = vec![0u8; len as usize];
            reader.read_exact(&mut value).map_err(|_| ArborRefused::Malformed { frame })?;
            let record = Commitment::from_wire(header[0], &value).ok_or(ArborRefused::Malformed { frame })?;
            let resealed = seal(record.coordinate.clone(), record.schema, record.kernel, record.judged, record.state, record.parent);
            if resealed.commit != record.commit {
                return Err(ArborRefused::Forged { frame });
            }
            match heads.iter().position(|h| h.coordinate.branch == record.coordinate.branch) {
                Some(at) => {
                    let head = &heads[at];
                    if record.parent != head.commit {
                        return Err(ArborRefused::Forged { frame });
                    }
                    if !ascends(&record.coordinate, &head.coordinate) {
                        return Err(ArborRefused::ArrowBroken { t: record.coordinate.t, head: head.coordinate.t });
                    }
                    heads[at] = record;
                }
                None if record.coordinate.branch.is_empty() => {
                    if record.parent != [0u8; 32] {
                        return Err(ArborRefused::Forged { frame });
                    }
                    heads.push(record);
                }
                None => return Self::open_with(root, true),
            }
            frame += 1;
        }
        Ok(Self { root, records: heads, heads_only: true })
    }

    fn open_with(root: impl AsRef<Path>, heads_only: bool) -> Result<Self, ArborRefused> {
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
                        if !ascends(&record.coordinate, &head.coordinate) {
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
                            && fork.coordinate.sub == record.coordinate.sub
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
        if heads_only {
            let mut kept: Vec<Commitment> = Vec::new();
            for record in records.into_iter().rev() {
                if !kept.iter().any(|h| h.coordinate.branch == record.coordinate.branch) {
                    kept.push(record);
                }
            }
            kept.reverse();
            records = kept;
        }
        Ok(Self { root, records, heads_only })
    }

    /// Where the universe stands at one coordinate.
    pub fn resolve(&self, coordinate: &Coordinate) -> Option<&Commitment> {
        self.records.iter().find(|record| &record.coordinate == coordinate)
    }

    /// The verified records, in stream order — a lens over the open
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
    /// kernel gates across the seam. The gated flag is stamped into
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
                if !ascends(&coordinate, &head.coordinate) {
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

    /// **The judged door** (reconstruct iii, Concordance 4.4 annex): a
    /// step the caller already judged with `kernel::admits` on a prior
    /// state whose digest is `prior` (BLAKE3 of its state bytes). The arbor
    /// carries that verdict instead of re-deriving it: it checks `prior` is
    /// its branch head's own state digest — the one thing it must not take
    /// on trust — and appends `state_bytes` judged, with no blob fetched,
    /// decoded or re-admitted. [`Arbor::extend`] stays the door for steps
    /// nobody here judged (received work re-judges, never trusts).
    pub fn extend_judged(
        &mut self,
        coordinate: Coordinate,
        state_bytes: &[u8],
        prior: [u8; 32],
    ) -> Result<&Commitment, ArborRefused> {
        let schema = *state_bytes.first().ok_or(ArborRefused::StateUndecodable)?;
        if self.resolve(&coordinate).is_some() {
            return Err(ArborRefused::Occupied);
        }
        let head = self.head(&coordinate.branch).ok_or(ArborRefused::GenesisJudged)?;
        if !ascends(&coordinate, &head.coordinate) {
            return Err(ArborRefused::ArrowBroken { t: coordinate.t, head: head.coordinate.t });
        }
        if head.state != prior {
            return Err(ArborRefused::PriorDiverged);
        }
        let parent = head.commit;
        let state = *blake3::hash(state_bytes).as_bytes();
        let blob = self.blob_path(&state);
        if !blob.exists() {
            fs::write(&blob, state_bytes).map_err(io)?;
        }
        let record = seal(coordinate, schema, kernel::VERSION, true, state, parent);
        self.append(record)
    }

    /// The adoption door (#36): a pre-chain world enters the mainline
    /// once — its current state seated as the unjudged genesis, the
    /// debt stamped `gated = false` (history before the chain is
    /// named, never hidden). After adoption every step arrives through
    /// [`Arbor::extend`], gated where covered. A second adoption
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
                sub: fork.coordinate.sub,
            },
            fork.schema,
            kernel::VERSION,
            false,
            fork.state,
            fork.commit,
        );
        self.append(record)
    }

    /// Receive (SPINE-UPLINK): admit a replication segment onto this
    /// arbor. Admission is verification — the receiver trusts nothing:
    /// every record must reseal (its commit recomputes), extend this
    /// arbor's own head lawfully (parent links, the arrow ascends, the
    /// address is fresh), carry a hash-matched blob that decodes as
    /// committed state, and — if it claims `gated` — re-run
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
                // Gate-side: an unknown frame in a segment refuses
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

    /// Admit one received record onto this arbor — the trustless core
    /// of `receive`, in order (reseal → geometry → chain-extension →
    /// re-gate → append).
    fn admit(
        &mut self,
        record: Commitment,
        frame: usize,
        blobs: &HashMap<[u8; 32], Vec<u8>>,
        steps: &HashMap<[u8; 32], kernel::Step>,
    ) -> Result<bool, ArborRefused> {
        // Idempotent replication: a record already held is a no-op
        // (a peer may re-present its whole arbor); the same commit at
        // its address skips, a different commit is a fork attempt.
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
        //    hash-matched, decodes, and its leading byte is the schema.
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
        // 3. Chain-extension — the record extends this arbor's own head.
        let branch = record.coordinate.branch.clone();
        let prior_state = self.head(&branch).map(|head| head.state);
        match self.head(&branch) {
            Some(head) => {
                if record.parent != head.commit {
                    return Err(ArborRefused::Forged { frame });
                }
                if !ascends(&record.coordinate, &head.coordinate) {
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
                    && fork.coordinate.sub == record.coordinate.sub
                    && fork.state == record.state;
                if !copies {
                    return Err(ArborRefused::Forged { frame });
                }
            }
        }
        // 4. Re-gate — a gated record's transition re-runs `lawful`
        //    under this kernel from the carried step, or it is refused:
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
        // 5. Admit — write the blob, append the record verbatim (its
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
        if self.heads_only {
            self.records.retain(|h| h.coordinate.branch != record.coordinate.branch);
        }
        self.records.push(record);
        Ok(self.records.last().expect("just appended"))
    }

    fn blob_path(&self, state: &[u8; 32]) -> PathBuf {
        let hex: String = state.iter().map(|byte| format!("{byte:02x}")).collect();
        self.root.join("store").join(format!("{hex}.bin"))
    }
}
