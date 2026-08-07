//! TRADE DESK — Phase 1 sightlines (`33_TRADE_FLEET_AND_DESK`,
//! `TASKS_TRADE_FLEET_DESK` P1.*).
//!
//! Pure readings over landed physics. Every cost number is produced by
//! calling [`cfe::world::pair_melt`] (one producer). The desk holds no
//! ledger state — panes render committed books and dispatch is the
//! CLI's ordinary verbs (P1.9).

use cfe::ratio::{ratio, Ratio};
use cfe::treaty::TreatyKind;
use cfe::world::{pair_melt, World};
use conductor::spatial_l1;
use std::path::PathBuf;

/// Active desk pane (key hints match 33 §6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Trade,
    Diplo,
    Routes,
    Arb,
    /// Placeholders until Phase 3–5 — still switchable so the frame is stable.
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

/// Peer status for the TRADE pane (33 §3.4).
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

/// One peer row — pure reading.
#[derive(Debug, Clone)]
pub struct PeerRow {
    pub name: String,
    pub d_chart: Ratio,
    pub melt: Ratio,
    pub door: PeerDoor,
    pub treaty: String,
}

/// Charted position from starchart.json (same law as CLI `charted_position`).
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

/// Saved world names under `var/worlds/` (directories with chronicle.tlv).
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

/// Resume a peer for **reading only** (no pen required for melt/treaty views).
/// Writing still goes through CLI `far_world` pen law.
pub fn load_peer_read(name: &str) -> Result<World, String> {
    let path = PathBuf::from("var/worlds").join(name).join("chronicle.tlv");
    if !path.exists() {
        return Err(format!("{name}: no chronicle.tlv"));
    }
    let chronicle = cfe::persist::load(&path)
        .map_err(|r| format!("{name}: load refused {r:?}"))?;
    Ok(World::resume(chronicle))
}

/// **P1.11** — the desk's only melt producer: call-not-copy of
/// [`pair_melt`]. Alignment (Phase 2) will feed `D_eff` into this same call.
pub fn route_melt(home: &World, peer: &World, d_chart: &Ratio) -> Ratio {
    pair_melt(home, peer, d_chart)
}

/// Door + treaty label for a peer.
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

/// Build TRADE pane peer rows for every saved world (except self).
pub fn trade_peers(home: &World) -> Vec<PeerRow> {
    let home_name = home.name().to_string();
    let home_pos = charted_position(&home_name);
    let mut rows = Vec::new();
    for name in peer_world_names(Some(&home_name)) {
        let Ok(peer) = load_peer_read(&name) else {
            continue;
        };
        let d_chart = spatial_l1(&home_pos, &charted_position(&name));
        let melt = route_melt(home, &peer, &d_chart);
        let door = if melt >= ratio(1, 1) {
            // Economically walled by decay — show as WALL with melt note.
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
            melt,
            door: door.0,
            treaty: door.1,
        });
    }
    rows
}

fn ratio_str(value: &Ratio) -> String {
    format!("{}/{}", value.numer(), value.denom())
}

/// Colony strip — always visible under the desk frame.
pub fn colony_strip(home: &World, orb_id: u64) -> String {
    let stamp = home
        .orb()
        .map(|o| ratio_str(&o.stamp_amount()))
        .unwrap_or_else(|| "0/1".into());
    let committed = home.committed();
    let Some(orb) = committed.orbs.iter().find(|o| o.orb_id == orb_id) else {
        return format!("  colony: (no orb) · STAMP {stamp}");
    };
    let banks: String = orb
        .banked
        .iter()
        .map(|(r, a)| format!("{} {}", ratio_str(a), resource_word(*r)))
        .collect::<Vec<_>>()
        .join(" · ");
    let famine = if orb.famine.is_empty() {
        String::new()
    } else {
        format!(
            " · FAMINE {}",
            orb.famine
                .iter()
                .map(|r| resource_word(*r))
                .collect::<Vec<_>>()
                .join(",")
        )
    };
    let zones = orb.zones.len();
    format!(
        "  {} [{}z] STAMP {stamp} · {banks}{famine}",
        orb.name, zones
    )
}

/// Render the outer frame + active pane body.
pub fn render_frame(home: &World, pane: Pane, orb_id: u64) -> Vec<String> {
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
    lines.push(format!(
        "╔══ DESK · {name} · {station} · routes {routes_n} · offers {offers_n} · STAMP {stamp} ══╗"
    ));
    lines.push(
        "║ [t]rade [y]diplo [r]outes [a]rb  [f]leet [x]exch [g]row [d]efend [e]xplore  [q]uit ║"
        .into(),
    );
    lines.push(format!("╠══ {} ══════════════════════════════════════════════════════════════╣", pane.label()));
    lines.extend(render_pane(home, pane, orb_id));
    lines.push("╠══ COLONY ══════════════════════════════════════════════════════════╣".into());
    lines.push(colony_strip(home, orb_id));
    lines.push("╚════════════════════════════════════════════════════════════════════╝".into());
    lines.push(
        "  deeds: offer · trade · aperture · charter · stamp · element · treaty  (no desk ledger)"
            .into(),
    );
    lines
}

fn render_pane(home: &World, pane: Pane, orb_id: u64) -> Vec<String> {
    match pane {
        Pane::Trade => render_trade(home),
        Pane::Diplo => render_diplo(home),
        Pane::Routes => render_routes(home),
        Pane::Arb => render_arb(home),
        Pane::Fleet => vec![
            "  (Phase 3) hulls · scrap · yard — not landed".into(),
            "  until then: capacity is unlimited aperture (engine)".into(),
        ],
        Pane::Exch => vec![
            "  (Phase 4) local instrument books — not landed".into(),
            format!(
                "  STAMP balance now: {}",
                home.orb()
                    .map(|o| ratio_str(&o.stamp_amount()))
                    .unwrap_or_else(|| "0/1".into())
            ),
            "  use: stamp balance · stamp mint <amt> · stamp aperture …".into(),
        ],
        Pane::Grow => render_grow(home, orb_id),
        Pane::Defend => vec!["  (Phase 5) convoy / escort — not landed".into()],
        Pane::Explore => render_explore(home),
    }
}

fn render_trade(home: &World) -> Vec<String> {
    let mut lines = vec![format!(
        "  {:<18} {:>8} {:>10} {:<8} {}",
        "PEER", "D_chart", "melt", "door", "treaty"
    )];
    let rows = trade_peers(home);
    if rows.is_empty() {
        lines.push("  (no other saved worlds with chronicle.tlv under var/worlds/)".into());
        lines.push("  save peers, or run conductor polities, then reopen the desk".into());
        return lines;
    }
    for row in rows {
        lines.push(format!(
            "  {:<18} {:>8} {:>10} {:<8} {}",
            trunc(&row.name, 18),
            ratio_str(&row.d_chart),
            ratio_str(&row.melt),
            row.door.word(),
            row.treaty
        ));
    }
    lines.push("  melt = pair_melt (chart L1 + key gap, route η, caravan, treaty)".into());
    lines
}

fn render_diplo(home: &World) -> Vec<String> {
    let mut lines = vec!["  standing treaties (this book):".into()];
    let now = home.now();
    let treaties = home.treaties();
    if treaties.is_empty() {
        lines.push("  (none) — treaty <trade|defense|peace|war> with <world> …".into());
    } else {
        for t in treaties {
            if t.expired(now) {
                continue;
            }
            lines.push(format!(
                "  deal {} · {} with {} · term {:?}",
                t.deal,
                t.kind.word(),
                t.partner,
                t.term
            ));
        }
    }
    lines.push("  renounce: treaty renounce <deal> <world>".into());
    lines.push("  kinds: trade · defense · peace · war  (war ⇒ WALL on TRADE)".into());
    // Silence unused warning if TreatyKind only used via word().
    let _ = TreatyKind::Trade;
    lines
}

fn render_routes(home: &World) -> Vec<String> {
    let mut lines = Vec::new();
    let committed = home.committed();
    lines.push(format!("  ROUTE-1 lanes: {}", committed.routes.len()));
    for (deal, eta) in &committed.routes {
        lines.push(format!(
            "  deal {}  η {}  (melt × (1−η); upkeep 1/4 joule at year voice)",
            deal,
            ratio_str(eta.value())
        ));
    }
    if committed.routes.is_empty() {
        lines.push("  no carrier lanes — charter <world> deal <n>".into());
    }
    lines.push(format!(
        "  element lanes: {}",
        committed.element_routes.len()
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
            "  deal {}  η {}  rail {rail_word}  element {}",
            deal,
            ratio_str(eta.value()),
            hex_prefix(element)
        ));
    }
    if committed.element_routes.is_empty() {
        lines.push("  no element lanes — element charter <world> <prefix> deal <n>".into());
    }
    lines
}

/// Read-only arbitrage map: every unordered pair of peers that is
/// economically walled direct while a third peer bridges both with
/// melt < 1 (merchant_step geometry).
fn render_arb(home: &World) -> Vec<String> {
    let home_name = home.name().to_string();
    let mut names = peer_world_names(Some(&home_name));
    names.insert(0, home_name.clone());

    // Load what we can.
    let mut worlds: Vec<(String, World, (Ratio, Ratio, Ratio))> = Vec::new();
    for name in &names {
        if name == &home_name {
            worlds.push((
                name.clone(),
                // Clone via committed resume is heavy; use home by re-load
                // for uniform typing — load home from memory via name.
                {
                    // We cannot move home; re-read for arb map if saved.
                    if let Ok(w) = load_peer_read(name) {
                        w
                    } else {
                        // Unsaved session world: skip triangles that need disk home.
                        continue;
                    }
                },
                charted_position(name),
            ));
            continue;
        }
        if let Ok(w) = load_peer_read(name) {
            let pos = charted_position(name);
            worlds.push((name.clone(), w, pos));
        }
    }

    let mut lines = vec![
        "  triangles (read-only): direct WALL (melt≥1) bridged by M with both legs open"
            .into(),
    ];
    let mut found = 0usize;
    let n = worlds.len();
    for a in 0..n {
        for b in (a + 1)..n {
            let d_ab = spatial_l1(&worlds[a].2, &worlds[b].2);
            let melt_ab = route_melt(&worlds[a].1, &worlds[b].1, &d_ab);
            if melt_ab < ratio(1, 1) {
                continue; // direct open — not merchant territory
            }
            for m in 0..n {
                if m == a || m == b {
                    continue;
                }
                let d_am = spatial_l1(&worlds[a].2, &worlds[m].2);
                let d_mb = spatial_l1(&worlds[m].2, &worlds[b].2);
                let melt_am = route_melt(&worlds[a].1, &worlds[m].1, &d_am);
                let melt_mb = route_melt(&worlds[m].1, &worlds[b].1, &d_mb);
                if melt_am < ratio(1, 1) && melt_mb < ratio(1, 1) {
                    found += 1;
                    if found <= 24 {
                        lines.push(format!(
                            "  {} --X--> {}  via {}  (legs melt {} · {})",
                            trunc(&worlds[a].0, 14),
                            trunc(&worlds[b].0, 14),
                            trunc(&worlds[m].0, 14),
                            ratio_str(&melt_am),
                            ratio_str(&melt_mb)
                        ));
                    }
                }
            }
        }
    }
    if found == 0 {
        lines.push("  (no walled pairs with a live bridge among saved worlds)".into());
    } else if found > 24 {
        lines.push(format!("  … {found} triangles total (showing 24)"));
    }
    lines.push("  NPC merchant_step bridges these; player: multi-hop aperture/trade".into());
    lines
}

fn render_grow(home: &World, orb_id: u64) -> Vec<String> {
    let committed = home.committed();
    let mut lines = vec![format!(
        "  star {:?} · orbs {} · techs {} · instruments {}",
        committed.star,
        committed.orbs.len(),
        committed.techs.len(),
        committed.instruments.len()
    )];
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
        lines.push(line);
    }
    lines.push("  deeds: zone · tick · collect · prospect · refine path via zones".into());
    lines
}

fn render_explore(home: &World) -> Vec<String> {
    let mut lines = vec![format!(
        "  prospects: {}  elements banked kinds: {}",
        home.committed().prospects.len(),
        home.orb().map(|o| o.elements.len()).unwrap_or(0)
    )];
    if let Some(orb) = home.orb() {
        for (el, amt) in orb.elements.iter().take(12) {
            lines.push(format!("  {}  {}", hex_prefix(el), ratio_str(amt)));
        }
        if orb.elements.len() > 12 {
            lines.push(format!("  … {} more", orb.elements.len() - 12));
        }
    }
    lines.push("  deeds: prospect · assay · elements · element aperture/charter".into());
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
