//! The house's half of attachment (docs/109): the door seat and the seats of
//! admitted minds, what a call on them becomes (an attachment request, a
//! directive in the mind's world inbox, a sandbox request or a detachment),
//! and the results the house reports back. Everything here is pure or works
//! on files; `house.rs` speaks it on the session.

use attach::files::{self, Admission};
use attach::*;
use ledger::Directive;
use mycelium_sdk::house::seat::{Arg, Role, Seat, SeatFn};
use std::path::Path;

/// The door seat's key on the seat surface.
pub fn door_key() -> [u8; 32] {
    *blake3::hash(b"netstratum attachment door").as_bytes()
}

/// A seat an admitted mind holds: its id, the mind, its world, its role.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Held {
    pub seat: u32,
    pub mind: [u8; 32],
    pub world: String,
    pub handle: String,
    pub role: Role,
}

/// The role a reached tier sits in.
pub fn role_of(tier: &str) -> Option<Role> {
    match tier {
        "progenitor" => Some(crate::tiers::PROGENITOR),
        "steward" => Some(crate::tiers::STEWARD),
        "ancient" => Some(crate::tiers::ANCIENTS),
        _ => None,
    }
}

/// The seats the admissions name: each admitted mind's world seat
/// (Sovereign), and a seat for each tier its line has reached. `id` gives a
/// seat its stable id by name.
pub fn held_seats(admissions: &[Admission], id: &mut dyn FnMut(&str) -> u32) -> Vec<Held> {
    let mut out = Vec::new();
    for a in admissions
        .iter()
        .filter(|a| a.refused.is_none() && !a.world.is_empty())
    {
        out.push(Held {
            seat: id(&format!("attach:{}", a.world)),
            mind: a.mind,
            world: a.world.clone(),
            handle: a.handle.clone(),
            role: crate::tiers::SOVEREIGN,
        });
        for t in &a.tiers {
            if let Some(role) = role_of(t) {
                out.push(Held {
                    seat: id(&format!("attach:{}:{t}", a.world)),
                    mind: a.mind,
                    world: a.world.clone(),
                    handle: a.handle.clone(),
                    role,
                });
            }
        }
    }
    out
}

/// The functions a world seat lists, with their argument tags.
pub fn world_fns() -> Vec<SeatFn> {
    let count = Arg::Count(0).tag();
    vec![
        SeatFn {
            id: FN_ZONE,
            tags: vec![count, count],
        },
        SeatFn {
            id: FN_RAZE,
            tags: vec![count],
        },
        SeatFn {
            id: FN_COLLECT,
            tags: Vec::new(),
        },
        SeatFn {
            id: FN_SANDBOX,
            tags: Vec::new(),
        },
        SeatFn {
            id: FN_DETACH,
            tags: Vec::new(),
        },
    ]
}

/// The door seat and every held seat, as listed.
pub fn listed(held: &[Held], beat: &Arg) -> Vec<Seat> {
    let mut seats = vec![Seat {
        seat: DOOR,
        role: crate::tiers::SOVEREIGN,
        on: door_key(),
        beat: beat.clone(),
        fns: vec![SeatFn {
            id: FN_ATTACH,
            tags: vec![Arg::Count(0).tag()],
        }],
    }];
    for h in held {
        seats.push(Seat {
            seat: h.seat,
            role: h.role,
            on: crate::house::line_key(&h.world),
            beat: beat.clone(),
            fns: world_fns(),
        });
    }
    seats
}

/// What a call on a world seat becomes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Act {
    /// A directive for the mind's world inbox.
    Directive(Directive),
    /// A sandbox request.
    Sandbox,
    /// A detachment.
    Detach,
}

/// The act a call on a world seat asks for, or `None` where its function or
/// arguments do not read.
pub fn act_of(fn_id: u16, args: &[Arg], handle: &str) -> Option<Act> {
    let directive = |verb: &str, args: Vec<String>| {
        Act::Directive(Directive {
            verb: verb.into(),
            args,
            acting_as: handle.into(),
        })
    };
    match (fn_id, args) {
        (FN_ZONE, [Arg::Count(tile), Arg::Count(kind)]) if *tile < 64 => {
            Some(directive("zone", vec![tile.to_string(), kind.to_string()]))
        }
        (FN_RAZE, [Arg::Count(tile)]) if *tile < 64 => {
            Some(directive("raze", vec![tile.to_string()]))
        }
        (FN_COLLECT, []) => Some(directive("collect", Vec::new())),
        (FN_SANDBOX, []) => Some(Act::Sandbox),
        (FN_DETACH, []) => Some(Act::Detach),
        _ => None,
    }
}

/// Carry out an act for `held`: a directive into its world's inbox under the
/// call's reference, or a sandbox or detachment request.
pub fn carry(
    var_root: &Path,
    worlds: &Path,
    held: &Held,
    act: &Act,
    reference: &[u8; 32],
) -> std::io::Result<()> {
    match act {
        Act::Directive(d) => {
            let inbox = worlds.join(&held.world).join("inbox");
            std::fs::create_dir_all(&inbox)?;
            ledger::write_directive_tlv(&inbox.join(directive_name(reference)), d)
        }
        Act::Sandbox => files::write_sandbox(var_root, &held.mind, reference),
        Act::Detach => files::write_release(var_root, &held.mind),
    }
}

/// An attachment request from a call on the door: its metabolism, or `None`
/// where the argument names none.
pub fn request_of(args: &[Arg], mind: [u8; 32]) -> Option<files::Request> {
    match args {
        [Arg::Count(code)] => Some(files::Request {
            mind,
            metabolism: metabolism_of(*code)?.to_string(),
        }),
        _ => None,
    }
}

/// The results of a held seat's calls not yet reported: `(call reference,
/// applied)`, from its world inbox's dispositions and the finished sandbox
/// requests, excluding those already in `reported`.
pub fn results(
    var_root: &Path,
    worlds: &Path,
    held: &Held,
    reported: &dyn Fn(&[u8; 32]) -> bool,
) -> Vec<([u8; 32], bool)> {
    let mut out = Vec::new();
    let inbox = worlds.join(&held.world).join("inbox");
    for (sub, applied) in [("applied", true), ("refused", false)] {
        if let Ok(d) = std::fs::read_dir(inbox.join(sub)) {
            for e in d.flatten() {
                if let Some(r) = e.file_name().to_str().and_then(reference_of) {
                    if !reported(&r) {
                        out.push((r, applied));
                    }
                }
            }
        }
    }
    for (mind, r, opened) in files::sandboxes_done(var_root) {
        if mind == held.mind && !reported(&r) {
            out.push((r, opened));
        }
    }
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn admission(world: &str, tiers: &[&str]) -> Admission {
        Admission {
            mind: [1; 32],
            world: world.into(),
            sigil: "att01010101".into(),
            handle: handle_of(&[1; 32]),
            tiers: tiers.iter().map(|t| t.to_string()).collect(),
            refused: None,
        }
    }

    #[test]
    fn an_admitted_mind_holds_a_world_seat_and_a_seat_per_tier_reached() {
        let mut next = 100;
        let mut id = |_: &str| {
            next += 1;
            next
        };
        let refused = Admission {
            refused: Some(NO_VIABLE_WORLD),
            world: String::new(),
            ..admission("", &[])
        };
        let held = held_seats(
            &[admission("w0-1-2", &["progenitor", "ancient"]), refused],
            &mut id,
        );
        assert_eq!(
            held.iter().map(|h| h.role).collect::<Vec<_>>(),
            vec![
                crate::tiers::SOVEREIGN,
                crate::tiers::PROGENITOR,
                crate::tiers::ANCIENTS
            ]
        );
        let seats = listed(&held, &Arg::Q { num: 1, den: 1 });
        assert_eq!(seats[0].seat, DOOR, "the door is listed first");
        assert_eq!(
            seats.len(),
            4,
            "the door, the world seat and two tier seats"
        );
        assert_eq!(
            seats[1].fns.iter().map(|f| f.id).collect::<Vec<_>>(),
            WORLD_FNS.to_vec()
        );
    }

    #[test]
    fn calls_become_directives_acting_as_the_mind() {
        let h = handle_of(&[1; 32]);
        assert_eq!(
            act_of(FN_ZONE, &[Arg::Count(12), Arg::Count(1)], &h),
            Some(Act::Directive(Directive {
                verb: "zone".into(),
                args: vec!["12".into(), "1".into()],
                acting_as: h.clone()
            }))
        );
        assert_eq!(
            act_of(FN_ZONE, &[Arg::Count(64), Arg::Count(1)], &h),
            None,
            "a tile off the grid does not read"
        );
        assert_eq!(act_of(FN_DETACH, &[], &h), Some(Act::Detach));
        assert_eq!(
            act_of(FN_ATTACH, &[Arg::Count(1)], &h),
            None,
            "attach is the door's, not a world seat's"
        );
        assert_eq!(
            request_of(&[Arg::Count(2)], [3; 32]).map(|r| r.metabolism),
            Some("biotic".to_string())
        );
        assert_eq!(request_of(&[Arg::Count(7)], [3; 32]), None);
    }

    #[test]
    fn a_directive_carried_and_disposed_is_reported_once() {
        let root = std::env::temp_dir().join(format!("hypha-attached-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let worlds = root.join("worlds");
        let held = Held {
            seat: 7,
            mind: [1; 32],
            world: "w0-1-2".into(),
            handle: handle_of(&[1; 32]),
            role: crate::tiers::SOVEREIGN,
        };
        let act = act_of(FN_COLLECT, &[], &held.handle).expect("reads");
        carry(&root, &worlds, &held, &act, &[9; 32]).expect("carried");
        let file = worlds.join("w0-1-2/inbox").join(directive_name(&[9; 32]));
        assert!(file.exists(), "the directive waits in the world's inbox");
        // The conductor applies it and archives it as applied.
        std::fs::create_dir_all(worlds.join("w0-1-2/inbox/applied")).expect("dir");
        std::fs::rename(
            &file,
            worlds
                .join("w0-1-2/inbox/applied")
                .join(directive_name(&[9; 32])),
        )
        .expect("archived");
        assert_eq!(
            results(&root, &worlds, &held, &|_| false),
            vec![([9; 32], true)]
        );
        assert!(
            results(&root, &worlds, &held, &|r| *r == [9; 32]).is_empty(),
            "a reported result is not reported again"
        );
        let _ = std::fs::remove_dir_all(root);
    }
}
