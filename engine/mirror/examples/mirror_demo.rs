//! Play two short sessions and mirror them — the observer/steward
//! smoke fixture. Usage: cargo run -p mirror --example mirror_demo -- <root>

use cfe::dop::ZoneKind;
use cfe::world::World;
use mirror::Mirror;

fn main() {
    let root = std::env::args().nth(1).expect("usage: mirror_demo <root>");
    let mirror = Mirror::at(&root);

    // aurora: the aggressive two-extractor opening, played 6 ticks.
    let mut aurora = World::new();
    aurora.found("aurora").unwrap();
    aurora.zone(0, ZoneKind::Extractor).unwrap();
    aurora.zone(1, ZoneKind::Extractor).unwrap();
    aurora.zone(8, ZoneKind::Habitat).unwrap();
    for _ in 0..6 {
        aurora.tick();
    }
    aurora.collect().unwrap();
    mirror.record("aurora", &aurora).unwrap();
    mirror
        .record_solve("aurora", "arc_6150a2bd", "rotate180", "deadbeef")
        .unwrap();

    // whalesong: safe and slow, an agent on a leash, 3 ticks.
    let mut whalesong = World::new();
    whalesong.found("whalesong").unwrap();
    whalesong.zone(0, ZoneKind::Extractor).unwrap();
    whalesong.zone(8, ZoneKind::Habitat).unwrap();
    let rover = whalesong.register_agent("rover").unwrap();
    whalesong.grant(rover, cfe::ipsal::Scope::Collect).unwrap();
    for _ in 0..3 {
        whalesong.tick();
    }
    whalesong.collect_as(rover).unwrap();
    mirror.record("whalesong", &whalesong).unwrap();

    println!("mirrored 2 civilizations under {root}");
}
