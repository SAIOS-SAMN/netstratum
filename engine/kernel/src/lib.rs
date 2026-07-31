//! THE KERNEL — the one validated update function (CHARTER §11 rule 3).
//!
//! `lawful(prior, next, step)` judges whether ONE step lawfully carries
//! the universe from `prior` to `next`. It is:
//!
//! - **pure** — same inputs, same verdict, forever;
//! - **total** — every input returns `Ok` or a typed `Unlawful`, never
//!   panics, never hangs;
//! - **no I/O** — it touches no file, clock, socket, or thread;
//! - **encoding-blind** — it never produces or reads bytes.
//!   Serialization is the codec's monopoly (the spine), so the kernel
//!   and the chain can never disagree about bytes: only one component
//!   ever produces them. The kernel judges MEANING; the codec fixes
//!   FORM; the two never overlap.
//!
//! Conservation, containment, and exactness are INVARIANTS inside this
//! function — an unlawful transition is not caught after the fact, it
//! cannot be constructed. This is why CHARTER §11 dissolves the
//! anti-mirror: you do not reconcile what cannot diverge.
//!
//! **Versioned like the charter.** [`VERSION`] is bumped only by
//! amendment. Every chain commitment binds the version that judged the
//! step, so old history verifies under the judge it was cut with — no
//! future change reinterprets the past. The kernel never reads the
//! commitment; the spine binds `VERSION` into it.
//!
//! **Coverage grows by amendment, never silently.** Each event's rules
//! land as a witnessed addition (one witness per invariant, red-first);
//! CHARTER §11 clause 2 guards that the growth is amendment-shaped and
//! that no state semantics ever live outside this function.

use cfe::dop::{ClosureEvent, Resource};
use cfe::ratio::{ratio, Ratio};
use cfe::world::Committed;

/// The kernel version — bumped ONLY by charter amendment. The spine
/// binds this into every commitment (`H(schema ‖ bytes ‖ parent ‖
/// coordinate)` carries the judging version alongside the schema), so
/// a chain segment is forever verified under the version that cut it.
pub const VERSION: u64 = 1;

/// One proposed transition: the deed that CLAIMS to carry `prior` to
/// `next`. `event` is what closed, `actor` is who under NS-6
/// containment, `stamp` is the world-clock value it is stamped with.
/// The kernel judges whether the claim is lawful; it never applies it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub event: ClosureEvent,
    pub actor: u64,
    pub stamp: u64,
}

/// A refused transition — typed, carrying its measurement whole (the
/// house law: a refusal names its cure; a break shows its residual,
/// never a scalar). An `Ok(())` from [`lawful`] is the transition
/// admitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unlawful {
    /// A value-conserving step changed the total of a resource across
    /// the world's banks — value was minted or burned where the deed
    /// promised only to MOVE it. Both totals carried (boxed — the
    /// carrier is a big-int rational); the residual is theirs to read
    /// (shown, never summarized away).
    ConservationBroken { resource: Resource, before: Box<Ratio>, after: Box<Ratio> },
    /// THE SEALED DEFAULT — the judge refuses what no invariant covers.
    /// This event has no landed rule; a deed the kernel cannot judge does
    /// NOT enter. Coverage grows by amendment: each landing moves an event
    /// from this default-refuse arm to an explicit judged arm, and the
    /// kernel wires as the live gate (Phase 3) only when `Unjudged` is
    /// unreachable — every event covered. **SEAL A is a READER's law,
    /// never the judge's:** readers skip unknown TAGS so old software
    /// survives new content; a judge that skips unknown DEEDS is a hole
    /// exactly the size of every future tag. Two laws, one word "unknown"
    /// — conflating them silently rebuilds default-admit.
    Unjudged,
}

/// THE JUDGE. Does `step` lawfully carry `prior` to `next`? Pure,
/// total, encoding-blind, AND network-blind (it reads no mesh, no peer,
/// no uplink — `engine/kernel` imports `cfe` only; all arrows point one
/// way). `Ok(())` admits; `Err(Unlawful)` refuses with the break whole.
pub fn lawful(prior: &Committed, next: &Committed, step: &Step) -> Result<(), Unlawful> {
    // The sealed default arm (mesh-operator ruling, airtight-critical):
    // refuse-unknown, not admit-unknown. What no invariant covers does
    // not enter.
    if !covered(&step.event) {
        return Err(Unlawful::Unjudged);
    }
    conservation(prior, next, step)
}

/// The landed coverage of the invariant set. An event is judged only if
/// some invariant governs it; this predicate grows as I2..I6 land (I2
/// containment and I3 exactness apply to nearly every event, so the
/// `Unjudged` arm shrinks toward empty as the kernel seals). Today: only
/// conservation (I1) has landed, and it covers the value-moving deeds.
fn covered(event: &ClosureEvent) -> bool {
    matches!(event, ClosureEvent::ResourceTransferred { .. })
}

/// INVARIANT — conservation. A deed that only MOVES value may not
/// change the total of that value across the world's banks. The
/// kernel needs no game rule to enforce this: it reads the promise
/// from the event's shape (a transfer moves, never mints) and checks
/// the ledgers agree. Mint/burn classes declare their delta in the
/// event and are judged as their coverage lands (each an amendment).
fn conservation(prior: &Committed, next: &Committed, step: &Step) -> Result<(), Unlawful> {
    match &step.event {
        // A pure intra-world move: `from_orb` loses exactly what
        // `to_orb` gains, so the world's total of `resource` is
        // invariant across the step. (The cross-empire ideology books
        // post from this deed; they carry no value and never bear on
        // conservation.)
        ClosureEvent::ResourceTransferred { resource, .. } => {
            let before = total_banked(prior, *resource);
            let after = total_banked(next, *resource);
            match before == after {
                true => Ok(()),
                false => Err(Unlawful::ConservationBroken {
                    resource: *resource,
                    before: Box::new(before),
                    after: Box::new(after),
                }),
            }
        }
        // A covered event on which conservation places no constraint
        // (uncovered events already refused at the `lawful` default arm,
        // so this is reached only by covered events other invariants
        // govern). Coverage grows by witnessed amendment — clause 2.
        _ => Ok(()),
    }
}

/// The world's total banked holding of one resource, summed across
/// every orb. Pure; the canonical ledger drops zero entries, so the
/// sum is over what is actually held.
fn total_banked(state: &Committed, resource: Resource) -> Ratio {
    state
        .orbs
        .iter()
        .map(|orb| orb.banked_amount(resource))
        .fold(ratio(0, 1), |sum, held| sum + held)
}
