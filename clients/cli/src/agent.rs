//! **THE AGENT DOOR (#39) — where something that is not the operator
//! plays.**
//!
//! The conductor's automaton has been the only AI player and it is
//! COMPILED IN: it calls `World` directly, so "an AI plays netstratum"
//! has meant "an AI we shipped plays netstratum". This is the inbound
//! half of the uplink — outbound is the line wire (no JSON),
//! which has always been able to tell an outsider what is happening
//! without any way for one to answer.
//!
//! **THE DOOR ADDS NO AUTHORITY.** That is its whole design. Every
//! request runs through the same `*_as` verbs the operator's own
//! commands do, so an agent is exactly an `Entity` with scoped grants
//! (`ipsal::Entity::permits`) and the kernel gate judges its deeds
//! identically. There is no agent-only path into the world, and a
//! reader looking for one should find this comment and then find
//! nothing.
//!
//! **TRANSPORT-FREE, deliberately.** The door is a function from a
//! request LINE to a response, so it can be driven by the CLI, a
//! socket, a pipe or a test with no law living in the transport. What
//! ships here is the door and its dispatch; a listener is a thin
//! driver over it and carries no rules of its own.

use cfe::dop::{Resource, ZoneKind};
use cfe::world::{ActionRefused, World};

/// What the door answers. Every response names the actor and the deed
/// so a client that pipelines requests can match them up.
pub struct Response {
    pub ok: bool,
    pub deed: String,
    /// The digest of the receipt this deed sealed, where it sealed one.
    /// A refusal carries none — a refused deed leaves no receipt, which
    /// is the law the whole engine is built on.
    pub receipt: Option<String>,
    pub detail: String,
}

impl Response {
    fn refused(deed: &str, detail: String) -> Response {
        Response { ok: false, deed: deed.to_string(), receipt: None, detail }
    }
    /// Render as plain key-lines (not JSON) — uplink shape for agents.
    ///
    /// ```text
    /// ok true
    /// actor rover
    /// deed zone
    /// receipt <hex>
    /// detail ...
    /// ```
    pub fn wire(&self, actor: &str) -> String {
        let mut out = String::new();
        out.push_str(&format!("ok {}\n", self.ok));
        out.push_str(&format!("actor {}\n", one_line(actor)));
        out.push_str(&format!("deed {}\n", one_line(&self.deed)));
        if let Some(receipt) = &self.receipt {
            out.push_str(&format!("receipt {receipt}\n"));
        }
        out.push_str(&format!("detail {}\n", one_line(&self.detail)));
        out
    }
}

fn one_line(text: &str) -> String {
    text.chars()
        .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
        .collect()
}

/// **THE DOOR.** One request line from `actor`, judged and answered.
///
/// The actor is named by HANDLE and resolved against the world's own
/// entity register: an unregistered name is refused before any deed is
/// parsed, because identity precedes the world (the kernel's own
/// ordering). Nothing here grants a scope — `register` and `grant` are
/// the sovereign's deeds and stay the sovereign's.
pub fn handle(world: &mut World, actor_handle: &str, line: &str) -> Response {
    let before = world.chronicle().receipts().len();
    let Some(actor) = world
        .entities()
        .iter()
        .find(|held| held.handle == actor_handle)
        .map(|held| held.entity_id)
    else {
        return Response::refused(
            "",
            format!("no entity answers to {actor_handle} — the sovereign registers agents"),
        );
    };
    let mut words = line.split_whitespace();
    let deed = words.next().unwrap_or("").to_string();
    let orb = world.orb().map(|held| held.orb_id).unwrap_or(1);

    let outcome: Result<(), ActionRefused> = match deed.as_str() {
        // BUILD scope.
        "zone" => match (
            words.next().and_then(|w| w.parse::<u16>().ok()),
            words.next().and_then(parse_kind),
        ) {
            (Some(tile), Some(kind)) => world.zone_as(actor, orb, tile, kind),
            _ => return Response::refused(&deed, "usage: zone <tile> <kind>".into()),
        },
        "prospect" => match words.next().and_then(|w| w.parse::<u8>().ok()) {
            Some(stratum) => world.prospect_as(actor, orb, stratum).map(|_| ()),
            None => return Response::refused(&deed, "usage: prospect <stratum>".into()),
        },
        // RAZE scope.
        "raze" => match words.next().and_then(|w| w.parse::<u16>().ok()) {
            Some(tile) => world.raze_as(actor, orb, tile),
            None => return Response::refused(&deed, "usage: raze <tile>".into()),
        },
        // COLLECT scope.
        "collect" => world.collect_as(actor, orb).map(|_| ()),
        "assay" => match words.next().and_then(|w| resolve_element(world, orb, w)) {
            Some(element) => world.assay_as(actor, orb, element).map(|_| ()),
            None => return Response::refused(&deed, "usage: assay <element-prefix>".into()),
        },
        "compound" => {
            let prefixes: Vec<&str> = words.collect();
            let members: Option<Vec<[u8; 32]>> =
                prefixes.iter().map(|held| resolve_element(world, orb, held)).collect();
            match members {
                Some(members) if members.len() >= 2 => {
                    world.compound_as(actor, orb, &members).map(|_| ())
                }
                _ => {
                    return Response::refused(
                        &deed,
                        "usage: compound <prefix> <prefix> …".into(),
                    )
                }
            }
        }
        // A READING, not a deed: the door answers what the world looks
        // like without sealing anything, so an agent can decide before
        // it acts. Deliberately the ONLY non-sealing verb here.
        "look" => {
            return Response {
                ok: true,
                deed,
                receipt: None,
                detail: format!(
                    "clock {} · receipts {} · orbs {}",
                    world.now(),
                    world.chronicle().receipts().len(),
                    world.orbs().len()
                ),
            }
        }
        other => {
            return Response::refused(
                other,
                "unknown deed — zone · prospect · raze · collect · assay · compound · look"
                    .into(),
            )
        }
    };

    match outcome {
        Ok(()) => {
            // The receipt this deed sealed, if it sealed one. Read from
            // the chronicle rather than reported by the verb, so the
            // answer is what the CHAIN holds and not what the door
            // believes it did.
            let receipt = match world.chronicle().receipts().len() > before {
                true => world
                    .chronicle()
                    .receipts()
                    .last()
                    .map(|held| crate::hex(&held.own().0)),
                false => None,
            };
            Response { ok: true, deed, receipt, detail: String::from("sealed") }
        }
        Err(refusal) => Response::refused(&deed, format!("{refusal:?}")),
    }
}

fn parse_kind(word: &str) -> Option<ZoneKind> {
    match word {
        "ext" | "extractor" => Some(ZoneKind::Extractor),
        "hab" | "habitat" => Some(ZoneKind::Habitat),
        "smelt" | "smelter" => Some(ZoneKind::Smelter),
        "synth" | "synthesizer" => Some(ZoneKind::Synthesizer),
        "enc" | "encasement" => Some(ZoneKind::Encasement),
        "cryo" => Some(ZoneKind::CryoArray),
        "res" | "resonator" => Some(ZoneKind::Resonator),
        "regen" | "regenerator" => Some(ZoneKind::Regenerator),
        _ => None,
    }
}

/// Resolve an element by hex prefix against what this orb actually
/// holds or has opened — the same prefix idiom the CLI uses, so an
/// agent and a person name matter the same way.
fn resolve_element(world: &World, orb: u64, prefix: &str) -> Option<[u8; 32]> {
    let committed = world.committed();
    let banked = committed
        .orbs
        .iter()
        .find(|held| held.orb_id == orb)
        .map(|held| held.elements.iter().map(|(id, _)| *id).collect::<Vec<_>>())
        .unwrap_or_default();
    banked
        .iter()
        .chain(
            committed
                .prospects
                .iter()
                .filter(|vein| vein.orb == orb)
                .map(|vein| &vein.element),
        )
        .find(|element| crate::hex(*element).starts_with(prefix))
        .copied()
}

/// The unused import guard: `Resource` is named so the door's deed set
/// can grow into the value-moving verbs without re-deriving what a
/// resource is.
#[allow(dead_code)]
fn _resource_is_named(_: Resource) {}

#[cfg(test)]
mod tests {
    use super::*;
    use cfe::ipsal::Scope;
    use cfe::world::FIRST_ORB;

    fn played() -> World {
        let mut world = World::new();
        world.found("tunborn").expect("founds");
        world
    }

    #[test]
    fn the_door_adds_no_authority() {
        // THE WHOLE DESIGN, as a witness. A registered agent with NO
        // grant is refused by the same containment that refuses it at
        // the operator's own commands — the door is not a way in.
        let mut world = played();
        let agent = world.register_agent("rover").expect("registers");
        let answer = handle(&mut world, "rover", "zone 0 ext");
        assert!(!answer.ok, "an ungranted agent builds nothing");
        assert!(answer.receipt.is_none(), "and a refusal seals no receipt");
        assert!(answer.detail.contains("NotPermitted"), "{}", answer.detail);

        // Granted, the SAME request goes through and seals a receipt.
        world.grant(agent, Scope::Build).expect("grants");
        let answer = handle(&mut world, "rover", "zone 0 ext");
        assert!(answer.ok, "{}", answer.detail);
        let receipt = answer.receipt.expect("a sealed deed names its receipt");
        assert_eq!(
            receipt,
            crate::hex(&world.chronicle().receipts().last().expect("sealed").own().0),
            "the answer names what the CHAIN holds"
        );
        // And the receipt carries the AGENT's tag, not the sovereign's.
        assert!(
            world
                .chronicle()
                .receipts()
                .last()
                .expect("sealed")
                .provenance()
                .contains(&agent),
            "the deed is tagged to whoever did it"
        );
    }

    #[test]
    fn an_unregistered_name_is_refused_before_any_deed() {
        // Identity precedes the world — the kernel's own ordering, and
        // the door keeps it: nothing is parsed for a stranger.
        let mut world = played();
        let answer = handle(&mut world, "nobody", "zone 0 ext");
        assert!(!answer.ok);
        assert!(answer.detail.contains("no entity answers"), "{}", answer.detail);
        assert_eq!(world.chronicle().receipts().len(), 2, "nothing was sealed");
    }

    #[test]
    fn a_reading_seals_nothing_and_a_malformed_deed_is_named() {
        let mut world = played();
        world.register_agent("rover").expect("registers");
        let before = world.chronicle().receipts().len();
        let answer = handle(&mut world, "rover", "look");
        assert!(answer.ok);
        assert!(answer.receipt.is_none(), "a reading is not a deed");
        assert_eq!(world.chronicle().receipts().len(), before);

        let answer = handle(&mut world, "rover", "wiggle 3");
        assert!(!answer.ok);
        assert!(answer.detail.contains("unknown deed"), "{}", answer.detail);
        let answer = handle(&mut world, "rover", "zone");
        assert!(!answer.ok);
        assert!(answer.detail.contains("usage"), "{}", answer.detail);
    }

    #[test]
    fn the_answer_is_line_wire_not_json() {
        let mut world = played();
        let agent = world.register_agent("rover").expect("registers");
        world.grant(agent, Scope::Collect).expect("grants");
        let answer = handle(&mut world, "rover", "look");
        let wire = answer.wire("rover");
        assert!(wire.contains("ok true"), "{wire}");
        assert!(wire.contains("actor rover"), "{wire}");
        assert!(!wire.contains('{'), "no JSON: {wire}");
        let odd = Response {
            ok: false,
            deed: "zone".into(),
            receipt: None,
            detail: "he said \"no\"\nline".into(),
        };
        let w = odd.wire("rover");
        assert!(!w.contains('{'));
        assert!(!w.lines().any(|l| l.contains('\n') && l.starts_with("detail")));
    }

    #[test]
    fn every_scope_gates_its_own_deeds() {
        // The door's deed set spans all three scopes, and each is
        // gated by ITS own — a Build grant does not buy a Collect.
        let mut world = played();
        let agent = world.register_agent("rover").expect("registers");
        world.grant(agent, Scope::Build).expect("grants build");
        assert!(handle(&mut world, "rover", "prospect 1").detail.contains("NoStar")
            || !handle(&mut world, "rover", "prospect 1").ok);
        let collect = handle(&mut world, "rover", "collect");
        assert!(!collect.ok, "build does not buy collect");
        assert!(collect.detail.contains("NotPermitted"), "{}", collect.detail);
        world.grant(agent, Scope::Collect).expect("grants collect");
        // Now permitted — it may still refuse for its OWN law (nothing
        // accrued), which is a JUDGED answer rather than a barred one.
        let collect = handle(&mut world, "rover", "collect");
        assert!(
            !collect.detail.contains("NotPermitted"),
            "the scope no longer bars it: {}",
            collect.detail
        );
        let _ = FIRST_ORB;
    }
}
