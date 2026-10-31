//! NS-6 ipsal-os — the entity layer, first cut. Every individual
//! inside this internet — the operator at the terminal, a scripted agent
//! — is an entity: an identity, a containment scope, and a generation
//! tag on every receipt it forges.
//!
//! The containment law: a SOVEREIGN (no principal) holds every
//! scope by construction. An agent holds exactly the scopes its
//! principal granted, checked at every action, revocable at any time.
//! Agents are citizens, governed by containment, never by prohibition
//! (Charter §3.4).
//!
//! Generation tagging: an action receipt's provenance is the acting
//! entity's id. No game rule may branch on provenance — it rides
//! receipts for audit, never enters cognition (the v0.4 law, carried).

/// An action class an agent can be granted (wire codes permanent).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Scope {
    Build = 1,
    Raze = 2,
    Collect = 3,
}

impl Scope {
    pub fn from_wire(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Build),
            2 => Some(Self::Raze),
            3 => Some(Self::Collect),
            _ => None,
        }
    }
}

/// One entity — committed state, grounded by EntityRegistered /
/// ScopeGranted / ScopeRevoked receipts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    pub entity_id: u64,
    pub handle: String,
    /// None = sovereign operator; Some(id) = an agent contained by that
    /// principal.
    pub principal: Option<u64>,
    /// Granted scopes, ascending (canonical). Meaningful for agents;
    /// a sovereign's authority is by construction, not enumeration.
    pub grants: Vec<Scope>,
}

impl Entity {
    /// May this entity take an action of the given scope, by its own
    /// grants alone? (`permits_in` also checks its principals.)
    pub fn permits(&self, scope: Scope) -> bool {
        match self.principal {
            None => true,
            Some(_) => self.grants.contains(&scope),
        }
    }
}

/// May `actor` take an action of `scope` among `entities`? An agent's
/// authority is bounded by every principal above it: the actor and each
/// principal up to the sovereign must permit the scope. A chain that names an
/// unregistered entity, or that does not reach a sovereign within the number
/// of entities, permits nothing.
pub fn permits_in(entities: &[Entity], actor: u64, scope: Scope) -> bool {
    let mut at = actor;
    for _ in 0..=entities.len() {
        let Some(entity) = entities.iter().find(|held| held.entity_id == at) else {
            return false;
        };
        if !entity.permits(scope) {
            return false;
        }
        match entity.principal {
            None => return true,
            Some(principal) => at = principal,
        }
    }
    false
}
