//! TRADE DESK — Phase 1–2 (`33_TRADE_FLEET_AND_DESK`).
//!
//! Pure readings over landed physics. Melt is always
//! [`cfe::world::route_melt`] (pair_melt over D_eff = D_chart + D_orbit).
//! No desk ledger — panes render and deeds go through the CLI command path.

use crate::style::Style;
use cfe::ratio::{ratio, Ratio};
use cfe::treaty::TreatyKind;
use cfe::world::{
    alignment_of, d_orbit, effective_distance, route_melt, route_melt_at, Alignment, World,
    ORBIT_PHASE_TICKS,
};
use conductor::spatial_l1;
use std::path::PathBuf;

/// Active desk pane (key hints match 33 §6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Trade,
    Diplo,
    Routes,
    Arb,
    Fleet,
    Exch,
    Grow,
    Defend,
    Explore,
}

impl Pane {
    pub fn from_key(key: &str) -> Option<Pane> {
        match key.to_ascii_lowercase().as_str() {
            "t" | "trade" => Some(Pane::Trade),
            "y" | "diplo" | "diplomacy" => Some(Pane::Diplo),
            "r" | "routes" | "route" => Some(Pane::Routes),
            "a" | "arb" | "arbitrage" => Some(Pane::Arb),
            "f" | "fleet" => Some(Pane::Fleet),
            "x" | "exch" | "exchange" => Some(Pane::Exch),
            "g" | "grow" => Some(Pane::Grow),
            "d" | "defend" => Some(Pane::Defend),
            "e" | "explore" => Some(Pane::Explore),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Pane::Trade => "TRADE",
            Pane::Diplo => "DIPLO",
            Pane::Routes => "ROUTES",
            Pane::Arb => "ARBITRAGE",
            Pane::Fleet => "FLEET",
            Pane::Exch => "EXCH",
            Pane::Grow => "GROW",
            Pane::Defend => "DEFEND",
            Pane::Explore => "EXPLORE",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerDoor {
    Wall,
    Shield,
    Open,
}

impl PeerDoor {
    pub fn word(self) -> &'static str {
        match self {
            PeerDoor::Wall => "WALL",
            PeerDoor::Shield => "SHIELD",
            PeerDoor::Open => "OPEN",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PeerRow {
    pub name: String,
    pub d_chart: Ratio,
    pub d_orbit: Ratio,
    pub d_eff: Ratio,
    pub melt: Ratio,
    pub door: PeerDoor,
    pub treaty: String,
    pub align: Alignment,
}

pub fn charted_position(name: &str) -> (Ratio, Ratio, Ratio) {
    let zero = || ratio(0, 1);
    let Ok(text) = std::fs::read_to_string("var/worlds/starchart.json") else {
        return (zero(), zero(), zero());
    };
    for line in text.lines() {
        if field(line, "world").as_deref() == Some(name) {
            let axis = |key: &str| {
                field(line, key)
                    .and_then(|v| parse_ratio_text(&v))
                    .unwrap_or_else(zero)
            };
            return (axis("x"), axis("y"), axis("z"));
        }
    }
    (zero(), zero(), zero())
}

fn field(text: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let at = text.find(&needle)?;
    let rest = &text[at + needle.len()..];
    let colon = rest.find(':')?;
    let mut val = rest[colon + 1..].trim();
    if val.starts_with('"') {
        val = &val[1..];
        let end = val.find('"')?;
        return Some(val[..end].to_string());
    }
    let end = val.find([',', '}', '\n']).unwrap_or(val.len());
    Some(val[..end].trim().to_string())
}

fn parse_ratio_text(text: &str) -> Option<Ratio> {
    use std::str::FromStr;
    let (numer, denom) = match text.split_once('/') {
        Some((n, d)) => (n, d),
        None => (text, "1"),
    };
    Some(Ratio::new(
        num_bigint::BigInt::from_str(numer.trim()).ok()?,
        num_bigint::BigInt::from_str(denom.trim()).ok()?,
    ))
}

pub fn peer_world_names(exclude: Option<&str>) -> Vec<String> {
    let root = PathBuf::from("var/worlds");
    let Ok(entries) = std::fs::read_dir(&root) else {
        return Vec::new();
    };
    let mut names = Vec::new();
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(name) = entry.file_name().to_str().map(str::to_string) else {
            continue;
        };
        if exclude == Some(name.as_str()) {
            continue;
        }
        if path.join("chronicle.tlv").exists() {
            names.push(name);
        }
    }
    names.sort();
    names
}

pub fn load_peer_read(name: &str) -> Result<World, String> {
    let path = PathBuf::from("var/worlds").join(name).join("chronicle.tlv");
    if !path.exists() {
        return Err(format!("{name}: no chronicle.tlv"));
    }
    let chronicle =
        cfe::persist::load(&path).map_err(|r| format!("{name}: load refused {r:?}"))?;
    Ok(World::resume(chronicle))
}

/// Desk melt producer — call-not-copy of engine `route_melt`.
pub fn desk_route_melt(home: &World, peer: &World, d_chart: &Ratio) -> Ratio {
    route_melt(home, peer, d_chart)
}

pub fn peer_door(home: &World, peer: &World, peer_name: &str) -> (PeerDoor, String) {
    if home.at_war_with(peer_name) || peer.at_war_with(home.name()) {
        return (PeerDoor::Wall, "war".into());
    }
    if !home.knows_its_sun() || !peer.knows_its_sun() {
        return (PeerDoor::Shield, "shield".into());
    }
    let now = home.now();
    let treaties = home.treaties();
    let label = treaties
        .iter()
        .filter(|t| t.partner == peer_name && !t.expired(now))
        .map(|t| t.kind.word())
        .collect::<Vec<_>>()
        .join("+");
    let label = if label.is_empty() {
        "—".into()
    } else {
        label
    };
    (PeerDoor::Open, label)
}

pub fn trade_peers(home: &World) -> Vec<PeerRow> {
    let home_name = home.name().to_string();
    let home_pos = charted_position(&home_name);
    let mut rows = Vec::new();
    for name in peer_world_names(Some(&home_name)) {
        let Ok(peer) = load_peer_read(&name) else {
            continue;
        };
        let d_chart = spatial_l1(&home_pos, &charted_position(&name));
        let orbit = d_orbit(home, &peer);
        let d_eff = effective_distance(home, &peer, &d_chart);
        let melt = desk_route_melt(home, &peer, &d_chart);
        let align = alignment_of(home, &peer);
        let (door, treaty) = if melt >= ratio(1, 1) {
            let (d, treaty) = peer_door(home, &peer, &name);
            if d == PeerDoor::Open {
                (PeerDoor::Wall, format!("{treaty}|melt≥1"))
            } else {
                (d, treaty)
            }
        } else {
            peer_door(home, &peer, &name)
        };
        rows.push(PeerRow {
            name,
            d_chart,
            d_orbit: orbit,
            d_eff,
            melt,
            door,
            treaty,
            align,
        });
    }
    // Alignment-sensitive sort: OPEN first by rising melt, then SHIELD, then WALL.
    rows.sort_by(|a, b| {
        let rank = |d: PeerDoor| match d {
            PeerDoor::Open => 0,
            PeerDoor::Shield => 1,
            PeerDoor::Wall => 2,
        };
        rank(a.door)
            .cmp(&rank(b.door))
            .then_with(|| a.melt.cmp(&b.melt))
    });
    rows
}

fn ratio_str(value: &Ratio) -> String {
    format!("{}/{}", value.numer(), value.denom())
}

fn paint_door(style: Style, door: PeerDoor) -> String {
    match door {
        PeerDoor::Open => style.door_open(door.word()),
        PeerDoor::Wall => style.door_wall(door.word()),
        PeerDoor::Shield => style.door_shield(door.word()),
    }
}

fn paint_align(style: Style, align: Alignment) -> String {
    match align {
        Alignment::Cheap => style.align_cheap(align.word()),
        Alignment::Fair => style.align_fair(align.word()),
        Alignment::Dear => style.align_dear(align.word()),
        Alignment::Unplaced => style.dim(align.word()),
    }
}

fn colony_strip(style: Style, home: &World, orb_id: u64) -> String {
    let stamp = home
        .orb()
        .map(|o| ratio_str(&o.stamp_amount()))
        .unwrap_or_else(|| "0/1".into());
    let committed = home.committed();
    let Some(orb) = committed.orbs.iter().find(|o| o.orb_id == orb_id) else {
        return format!(
            "  {} (no orb) · {} {}",
            style.dim("colony:"),
            style.money("STAMP"),
            style.value(&stamp)
        );
    };
    let banks: String = orb
        .banked
        .iter()
        .map(|(r, a)| {
            format!(
                "{} {}",
                style.cargo(&ratio_str(a)),
                style.dim(resource_word(*r))
            )
        })
        .collect::<Vec<_>>()
        .join(" · ");
    let famine = if orb.famine.is_empty() {
        String::new()
    } else {
        format!(
            " · {}",
            style.bad(&format!(
                "FAMINE {}",
                orb.famine
                    .iter()
                    .map(|r| resource_word(*r))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
        )
    };
    format!(
        "  {} [{}z] {} {} · {banks}{famine}",
        style.label(&orb.name),
        orb.zones.len(),
        style.money("STAMP"),
        style.value(&stamp),
    )
}

/// Render the outer frame + active pane body (full ANSI).
pub fn render_frame(style: Style, home: &World, pane: Pane, orb_id: u64) -> Vec<String> {
    let name = home.name();
    let station = home
        .committed()
        .stations
        .last()
        .map(|(_, s)| {
            cfe::dop::Station::from_wire(*s)
                .map(|st| st.word().to_string())
                .unwrap_or_else(|| format!("#{s}"))
        })
        .unwrap_or_else(|| "T3".into());
    let routes_n = home.routes().len();
    let offers_n = home.offers().len();
    let stamp = home
        .orb()
        .map(|o| ratio_str(&o.stamp_amount()))
        .unwrap_or_else(|| "0/1".into());

    let mut lines = Vec::new();
    lines.push(style.border(&format!(
        "╔══ DESK · {name} · {station} · routes {routes_n} · offers {offers_n} · STAMP {stamp} ══╗"
    )));
    lines.push(format!(
        "{} {} {} {} {} {} {} {} {}  {} {}",
        style.border("║"),
        style.key("[t]rade"),
        style.key("[y]diplo"),
        style.key("[r]outes"),
        style.key("[a]rb"),
        style.key("[f]leet"),
        style.key("[x]exch"),
        style.key("[g]row"),
        style.key("[d]efend"),
        style.key("[e]xplore"),
        style.key("[q]uit"),
    ));
    lines.push(style.border(&format!(
        "╠══ {} ══════════════════════════════════════════════════════════════╣",
        pane.label()
    )));
    lines.extend(render_pane(style, home, pane, orb_id));
    lines.push(style.border(
        "╠══ COLONY ══════════════════════════════════════════════════════════╣",
    ));
    lines.push(colony_strip(style, home, orb_id));
    lines.push(style.border(
        "╚════════════════════════════════════════════════════════════════════╝",
    ));
    lines.push(format!(
        "  {}",
        style.hint(
            "deeds: offer · trade · aperture · charter · stamp · element · treaty  ·  help desk"
        )
    ));
    lines
}

fn render_pane(style: Style, home: &World, pane: Pane, orb_id: u64) -> Vec<String> {
    match pane {
        Pane::Trade => render_trade(style, home),
        Pane::Diplo => render_diplo(style, home),
        Pane::Routes => render_routes(style, home),
        Pane::Arb => render_arb(style, home),
        Pane::Fleet => vec![
            format!("  {}", style.warn("(Phase 3) hulls · scrap · yard — not landed")),
            format!(
                "  {}",
                style.dim("until then: capacity is unlimited aperture (engine)")
            ),
        ],
        Pane::Exch => vec![
            format!(
                "  {}",
                style.warn("(Phase 4) local instrument books — not landed")
            ),
            format!(
                "  {} {}",
                style.money("STAMP"),
                style.value(
                    &home
                        .orb()
                        .map(|o| ratio_str(&o.stamp_amount()))
                        .unwrap_or_else(|| "0/1".into())
                )
            ),
            format!(
                "  {}",
                style.hint("use: stamp balance · stamp mint <amt> · stamp aperture …")
            ),
        ],
        Pane::Grow => render_grow(style, home, orb_id),
        Pane::Defend => {
            vec![format!(
                "  {}",
                style.warn("(Phase 5) convoy / escort — not landed")
            )]
        }
        Pane::Explore => render_explore(style, home),
    }
}

fn render_trade(style: Style, home: &World) -> Vec<String> {
    let mut lines = vec![format!(
        "  {} {} {} {} {} {} {}",
        style.label(&format!("{:<16}", "PEER")),
        style.label(&format!("{:>7}", "D_ch")),
        style.label(&format!("{:>7}", "D_orb")),
        style.label(&format!("{:>7}", "D_eff")),
        style.label(&format!("{:>8}", "melt")),
        style.label(&format!("{:<6}", "align")),
        style.label("door/treaty"),
    )];
    let rows = trade_peers(home);
    if rows.is_empty() {
        lines.push(format!(
            "  {}",
            style.dim("(no other saved worlds with chronicle.tlv under var/worlds/)")
        ));
        lines.push(format!(
            "  {}",
            style.hint("save peers, or run conductor polities, then reopen the desk")
        ));
        return lines;
    }
    for row in rows {
        lines.push(format!(
            "  {} {} {} {} {} {} {} {}",
            style.accent(&format!("{:<16}", trunc(&row.name, 16))),
            style.value(&format!("{:>7}", ratio_str(&row.d_chart))),
            style.info(&format!("{:>7}", ratio_str(&row.d_orbit))),
            style.strong(&format!("{:>7}", ratio_str(&row.d_eff))),
            style.cargo(&format!("{:>8}", ratio_str(&row.melt))),
            paint_align(style, row.align),
            paint_door(style, row.door),
            style.dim(&row.treaty),
        ));
    }
    lines.push(format!(
        "  {}",
        style.hint("D_eff = D_chart + D_orbit · melt = route_melt(D_eff) · sorted OPEN by melt")
    ));
    lines
}

fn render_diplo(style: Style, home: &World) -> Vec<String> {
    let mut lines = vec![format!(
        "  {}",
        style.section("standing treaties (this book)")
    )];
    let now = home.now();
    let treaties = home.treaties();
    if treaties.is_empty() {
        lines.push(format!(
            "  {}",
            style.dim("(none) — treaty <trade|defense|peace|war> with <world> …")
        ));
    } else {
        for t in treaties {
            if t.expired(now) {
                continue;
            }
            lines.push(format!(
                "  {} {} {} {} {} {}",
                style.label("deal"),
                style.value(&t.deal.to_string()),
                style.verb(t.kind.word()),
                style.dim("with"),
                style.accent(&t.partner),
                style.dim(&format!("term {:?}", t.term)),
            ));
        }
    }
    lines.push(format!(
        "  {}",
        style.hint("renounce: treaty renounce <deal> <world>  ·  help diplo")
    ));
    let _ = TreatyKind::Trade;
    lines
}

fn render_routes(style: Style, home: &World) -> Vec<String> {
    let mut lines = Vec::new();
    let committed = home.committed();
    lines.push(format!(
        "  {} {}",
        style.route("ROUTE-1 lanes:"),
        style.value(&committed.routes.len().to_string())
    ));
    for (deal, eta) in &committed.routes {
        lines.push(format!(
            "  {} {}  {} {}  {}",
            style.label("deal"),
            style.value(&deal.to_string()),
            style.dim("η"),
            style.strong(&ratio_str(eta.value())),
            style.hint("upkeep 1/4 joule / year-voice")
        ));
    }
    if committed.routes.is_empty() {
        lines.push(format!(
            "  {}",
            style.dim("no carrier lanes — charter <world> deal <n>")
        ));
    }
    lines.push(format!(
        "  {} {}",
        style.route("element lanes:"),
        style.value(&committed.element_routes.len().to_string())
    ));
    for (deal, eta, element, rail) in &committed.element_routes {
        let rail_word = cfe::dop::Rail::from_wire(*rail)
            .map(|r| match r {
                cfe::dop::Rail::Hearth => "R1",
                cfe::dop::Rail::Aperture => "R2",
                cfe::dop::Rail::Strand => "R3",
                cfe::dop::Rail::Court => "R4",
            })
            .unwrap_or("R?");
        lines.push(format!(
            "  {} {}  {} {}  {}  {}",
            style.label("deal"),
            style.value(&deal.to_string()),
            style.dim("η"),
            style.strong(&ratio_str(eta.value())),
            style.key(rail_word),
            style.cargo(&hex_prefix(element))
        ));
    }
    if committed.element_routes.is_empty() {
        lines.push(format!(
            "  {}",
            style.dim("no element lanes — element charter <world> <prefix> deal <n>")
        ));
    }

    // Projected melt windows vs nearest OPEN peer (Phase 2 P2.6).
    let peers = trade_peers(home);
    if let Some(best) = peers.iter().find(|r| r.door == PeerDoor::Open) {
        if let Ok(peer) = load_peer_read(&best.name) {
            lines.push(format!(
                "  {} {}  (phase step every {} ticks)",
                style.section("melt window →"),
                style.accent(&best.name),
                ORBIT_PHASE_TICKS
            ));
            let now = home.now();
            let d_chart = best.d_chart.clone();
            for step in 0u64..6 {
                let at = now + step * ORBIT_PHASE_TICKS;
                let m = route_melt_at(home, &peer, &d_chart, at);
                let band = if step == 0 {
                    paint_align(style, best.align)
                } else {
                    style.dim(&format!("t+{}", step * ORBIT_PHASE_TICKS))
                };
                lines.push(format!(
                    "    {}  melt {}",
                    band,
                    style.cargo(&ratio_str(&m))
                ));
            }
            lines.push(format!(
                "  {}",
                style.hint("wait a CHEAP window before hauling to shed less to the void")
            ));
        }
    }
    lines
}

fn render_arb(style: Style, home: &World) -> Vec<String> {
    let home_name = home.name().to_string();
    let mut names = peer_world_names(Some(&home_name));
    names.insert(0, home_name.clone());

    let mut worlds: Vec<(String, World, (Ratio, Ratio, Ratio))> = Vec::new();
    for name in &names {
        if let Ok(w) = load_peer_read(name) {
            let pos = charted_position(name);
            worlds.push((name.clone(), w, pos));
        }
    }

    let mut lines = vec![format!(
        "  {}",
        style.section("triangles: direct WALL (melt≥1) bridged by M with both legs open")
    )];
    let mut found = 0usize;
    let n = worlds.len();
    for a in 0..n {
        for b in (a + 1)..n {
            let d_ab = spatial_l1(&worlds[a].2, &worlds[b].2);
            let melt_ab = desk_route_melt(&worlds[a].1, &worlds[b].1, &d_ab);
            if melt_ab < ratio(1, 1) {
                continue;
            }
            for m in 0..n {
                if m == a || m == b {
                    continue;
                }
                let d_am = spatial_l1(&worlds[a].2, &worlds[m].2);
                let d_mb = spatial_l1(&worlds[m].2, &worlds[b].2);
                let melt_am = desk_route_melt(&worlds[a].1, &worlds[m].1, &d_am);
                let melt_mb = desk_route_melt(&worlds[m].1, &worlds[b].1, &d_mb);
                if melt_am < ratio(1, 1) && melt_mb < ratio(1, 1) {
                    found += 1;
                    if found <= 24 {
                        lines.push(format!(
                            "  {} {} {}  {} {}  (legs {} · {})",
                            style.accent(&trunc(&worlds[a].0, 12)),
                            style.door_wall("--X-->"),
                            style.accent(&trunc(&worlds[b].0, 12)),
                            style.dim("via"),
                            style.key(&trunc(&worlds[m].0, 12)),
                            style.cargo(&ratio_str(&melt_am)),
                            style.cargo(&ratio_str(&melt_mb)),
                        ));
                    }
                }
            }
        }
    }
    if found == 0 {
        lines.push(format!(
            "  {}",
            style.dim("(no walled pairs with a live bridge among saved worlds)")
        ));
    } else if found > 24 {
        lines.push(format!(
            "  {}",
            style.dim(&format!("… {found} triangles total (showing 24)"))
        ));
    }
    lines.push(format!(
        "  {}",
        style.hint("NPC merchant_step bridges these; player: multi-hop aperture/trade")
    ));
    lines
}

fn render_grow(style: Style, home: &World, orb_id: u64) -> Vec<String> {
    let committed = home.committed();
    let mut lines = vec![format!(
        "  star {:?} · orbs {} · techs {} · instruments {}",
        committed.star,
        committed.orbs.len(),
        committed.techs.len(),
        committed.instruments.len()
    )
    .pipe(|s| style.info(&s))];
    let zone_line = committed.orbs.iter().find(|o| o.orb_id == orb_id).map(|orb| {
        format!(
            "  zones: {}",
            orb.zones
                .iter()
                .map(|z| cfe::dop::zone_word(z.kind))
                .collect::<Vec<_>>()
                .join(", ")
        )
    });
    if let Some(line) = zone_line {
        lines.push(style.dim(&line));
    }
    lines.push(format!(
        "  {}",
        style.hint("deeds: zone · tick · collect · prospect · help colony")
    ));
    lines
}

fn render_explore(style: Style, home: &World) -> Vec<String> {
    let mut lines = vec![format!(
        "  prospects: {}  element kinds: {}",
        home.committed().prospects.len(),
        home.orb().map(|o| o.elements.len()).unwrap_or(0)
    )
    .pipe(|s| style.info(&s))];
    if let Some(orb) = home.orb() {
        for (el, amt) in orb.elements.iter().take(12) {
            lines.push(format!(
                "  {}  {}",
                style.cargo(&hex_prefix(el)),
                style.value(&ratio_str(amt))
            ));
        }
        if orb.elements.len() > 12 {
            lines.push(format!(
                "  {}",
                style.dim(&format!("… {} more", orb.elements.len() - 12))
            ));
        }
    }
    lines.push(format!(
        "  {}",
        style.hint("deeds: prospect · assay · elements · element aperture/charter")
    ));
    lines
}

fn hex_prefix(element: &[u8; 32]) -> String {
    element
        .iter()
        .take(4)
        .map(|b| format!("{b:02x}"))
        .collect::<Vec<_>>()
        .join("")
}

fn trunc(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
    }
}

fn resource_word(resource: cfe::dop::Resource) -> &'static str {
    match resource {
        cfe::dop::Resource::Iron => "iron",
        cfe::dop::Resource::Biomass => "biomass",
        cfe::dop::Resource::Alloy => "alloy",
        cfe::dop::Resource::Diamond => "diamond",
        cfe::dop::Resource::Joule => "joule",
        cfe::dop::Resource::Sink => "sink",
        cfe::dop::Resource::Crystal => "crystal",
    }
}

/// Tiny pipe helper for format → paint.
trait Pipe: Sized {
    fn pipe<R>(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }
}
impl<T> Pipe for T {}
