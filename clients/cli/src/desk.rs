//! Trade desk — Phase 1–2 (`33_TRADE_FLEET_AND_DESK`).
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
            "y" | "diplo" | "diplomacy" | "diplomatic" | "diplomatics" => Some(Pane::Diplo),
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
            Pane::Diplo => "DIPLOMATIC",
            Pane::Routes => "ROUTES",
            Pane::Arb => "ARBITRAGE",
            Pane::Fleet => "FLEET",
            Pane::Exch => "EXCH",
            Pane::Grow => "GROW",
            Pane::Defend => "DEFEND",
            Pane::Explore => "EXPLORE",
        }
    }

    /// Hub deep-links for this pane (H1.3).
    ///
    /// Order: primary activity (or craft when the pane is craft-native), then
    /// craft twin / peer. Desk» `h` runs the first; `hc` runs the second.
    /// Operators can also type any listed verb inside desk» (command path).
    pub fn hub_links(self) -> &'static [&'static str] {
        match self {
            // Circulate: books activity · dual-law craft twin.
            Pane::Trade => &["trade books", "bench trade"],
            // Diplomacy activity (preferred name) · craft twin.
            Pane::Diplo => &["diplomatic local", "bench diplomatic"],
            // Lanes are trade rails; craft instruments/routes under econ.
            Pane::Routes => &["trade rails", "bench econ"],
            // Triangle read sits on books; same craft twin as trade.
            Pane::Arb => &["trade books", "bench trade"],
            // Run hulls · craft hulls.
            Pane::Fleet => &["trade fleet", "bench fleet"],
            // Instruments / STAMP books · office seats (Exchange).
            Pane::Exch => &["trade books", "bench offices"],
            // Colony growth is craft-native (planetary + build).
            Pane::Grow => &["bench planetary", "bench build"],
            // Knowledge opsec · doctrine craft (KIN / posture).
            Pane::Defend => &["intel opsec", "bench doctrine"],
            // Knowledge research activity · research craft twin.
            Pane::Explore => &["intel research", "bench research"],
        }
    }

    /// Primary hub command for desk» `h` / `hub`.
    pub fn hub_primary(self) -> &'static str {
        self.hub_links()[0]
    }

    /// Craft / secondary hub command for desk» `hc` (if present).
    pub fn hub_craft(self) -> Option<&'static str> {
        self.hub_links().get(1).copied()
    }

    /// All panes — pin surface + iteration.
    #[cfg(test)]
    pub fn all() -> &'static [Pane] {
        &[
            Pane::Trade,
            Pane::Diplo,
            Pane::Routes,
            Pane::Arb,
            Pane::Fleet,
            Pane::Exch,
            Pane::Grow,
            Pane::Defend,
            Pane::Explore,
        ]
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
    // TLV starchart.card via ledger API — never starchart.json.
    let book = ledger::Ledger::at("var/worlds");
    match book.charted_position(name) {
        Some(((x, y, z), _)) => (x, y, z),
        None => (zero(), zero(), zero()),
    }
}

/// The universe's carried span — every desk distance wraps on it.
pub fn universe_span() -> Ratio {
    ledger::Ledger::at("var/worlds").universe_span()
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
    let chronicle = cfe::persist::load(&path).map_err(|r| format!("{name}: load refused {r:?}"))?;
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
    if !home.knows_its_star() || !peer.knows_its_star() {
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
        let d_chart = spatial_l1(&home_pos, &charted_position(&name), &universe_span());
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
    // Alignment-sensitive sort: open first by rising melt, then shield, then wall.
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
    let balance = home.thermal_balance_of(orb_id);
    let bal_s = match &balance {
        None => style.dim("balance —"),
        Some(b) if *b >= World::balance_orderly_floor() => {
            style.align_cheap(&format!("balance {}", ratio_str(b)))
        }
        Some(b) => style.align_dear(&format!("balance {}", ratio_str(b))),
    };
    let regen = if home.regenerator_stands(orb_id) {
        style.ok("regen●")
    } else {
        style.dim("regen○")
    };
    let committed = home.committed();
    let Some(orb) = committed.orbs.iter().find(|o| o.orb_id == orb_id) else {
        return format!(
            "  {} (no orb) · {} {} · {bal_s}",
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
        "  {} [{}z] {} {} · {bal_s} {regen} · {banks}{famine}",
        style.label(&orb.name),
        orb.zones.len(),
        style.money("STAMP"),
        style.value(&stamp),
    )
}

/// Render the outer frame + active pane body (full ansi).
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
    lines.push(hub_link_line(style, pane));
    lines.push(
        style.border("╠══ COLONY ══════════════════════════════════════════════════════════╣"),
    );
    lines.push(colony_strip(style, home, orb_id));
    lines.push(
        style.border("╚════════════════════════════════════════════════════════════════════╝"),
    );
    lines.push(format!(
        "  {}",
        style.hint(
            "deeds: offer · trade · aperture · charter · stamp · element · treaty  ·  help desk"
        )
    ));
    lines.push(format!(
        "  {}",
        style.hint("hub: desk» h primary · hc craft twin · type any hub verb (docs/HUBS.md)")
    ));
    lines
}

/// H1.3 — per-pane deep-link strip into hub domains.
fn hub_link_line(style: Style, pane: Pane) -> String {
    let links = pane.hub_links();
    let joined = links.join(" · ");
    format!(
        "  {} {}  {}",
        style.section("HUB"),
        style.key(&joined),
        style.dim(&format!("← {} pane", pane.label()))
    )
}

fn render_pane(style: Style, home: &World, pane: Pane, orb_id: u64) -> Vec<String> {
    match pane {
        Pane::Trade => render_trade(style, home),
        Pane::Diplo => render_diplo(style, home),
        Pane::Routes => render_routes(style, home),
        Pane::Arb => render_arb(style, home),
        Pane::Fleet => render_fleet(style, home),
        Pane::Exch => render_exch(style, home),
        Pane::Grow => render_grow(style, home, orb_id),
        Pane::Defend => render_defend(style, home),
        Pane::Explore => render_explore(style, home),
    }
}

fn render_exch(style: Style, home: &World) -> Vec<String> {
    let mi = home.market_integrity();
    let seal_line = if mi.sealed {
        format!(
            "  {} until t{} · cascade {}",
            style.warn("CIRCUIT OPEN"),
            mi.resume_at.unwrap_or(0),
            mi.cascade_pressure
        )
    } else {
        format!(
            "  clear seal crystal {}/{} iron {}/{} · cascade {}",
            mi.crystal_depth, mi.seal_need, mi.iron_depth, mi.seal_need, mi.cascade_pressure
        )
    };
    let mut lines = vec![
        format!(
            "  {} {}  books: note={} fut={} share={} perp={}  exch={}",
            style.money("STAMP"),
            style.value(
                &home
                    .orb()
                    .map(|o| ratio_str(&o.stamp_amount()))
                    .unwrap_or_else(|| "0/1".into())
            ),
            yn(mi.notes_open),
            yn(mi.futures_open),
            yn(mi.shares_open),
            yn(mi.perps_open),
            yn(home.exchange_seated()),
        ),
        seal_line,
        format!(
            "  {}",
            style.hint(
                "note · future · share · perp · market seal|clear|raid|cascade · suite market"
            )
        ),
    ];
    let notes: Vec<_> = home.notes().iter().filter(|n| n.open).collect();
    if !notes.is_empty() {
        lines.push(format!("  {}", style.label("NOTES")));
        for n in notes.iter().take(6) {
            lines.push(format!(
                "    note {} face {} mat t{}",
                n.note_id,
                ratio_str(&n.notional),
                n.maturity
            ));
        }
    }
    let futs: Vec<_> = home.futures().iter().filter(|f| f.open).collect();
    if !futs.is_empty() {
        lines.push(format!("  {}", style.label("FUTURES")));
        for f in futs.iter().take(6) {
            lines.push(format!(
                "    fut {} {:?}×{} @ {} exp t{}",
                f.fut_id,
                f.resource,
                ratio_str(&f.notional),
                ratio_str(&f.price),
                f.expiry
            ));
        }
    }
    let shares: Vec<_> = home.shares().iter().filter(|s| s.open).collect();
    if !shares.is_empty() {
        lines.push(format!("  {}", style.label("SHARES")));
        for s in shares.iter().take(6) {
            lines.push(format!(
                "    share {} {:?} units {} par {}",
                s.share_id,
                s.stream,
                ratio_str(&s.units),
                ratio_str(&s.par)
            ));
        }
    }
    let perps: Vec<_> = home.perps().iter().filter(|p| p.open).collect();
    if !perps.is_empty() {
        lines.push(format!("  {}", style.label("PERPS")));
        for p in perps.iter().take(6) {
            lines.push(format!(
                "    perp {} {:?}×{} entry {}",
                p.perp_id,
                p.resource,
                ratio_str(&p.notional),
                ratio_str(&p.entry)
            ));
        }
    }
    if notes.is_empty() && futs.is_empty() && shares.is_empty() && perps.is_empty() {
        lines.push(format!(
            "  {}",
            style.dim("(no open instruments — bank STAMP/cargo or seat Exchange)")
        ));
    }
    let clears = home.last_clears();
    if !clears.is_empty() {
        lines.push(format!("  {}", style.label("LAST CLEARS")));
        for (r, p) in clears.iter().rev().take(4) {
            lines.push(format!("    {:?} @ {} STAMP/u", r, ratio_str(p)));
        }
    }
    lines
}

fn yn(b: bool) -> &'static str {
    if b {
        "open"
    } else {
        "—"
    }
}

fn render_defend(style: Style, home: &World) -> Vec<String> {
    use cfe::world::{DefenseDomain, RiskBand};
    let posture = home.defense_posture();
    let mut lines = vec![
        format!(
            "  {} primary {} · secondary {} · {}",
            style.label("POSTURE"),
            style.strong(posture.primary.word()),
            style.accent(posture.secondary.word()),
            style.dim(&posture.voice)
        ),
        format!(
            "  anatomy met={} · doctrine dis={} per={} · reoriented={}  (36)",
            posture.metabolism, posture.disposition, posture.perception, posture.reoriented
        ),
    ];
    let domains = [
        DefenseDomain::Cel,
        DefenseDomain::Atm,
        DefenseDomain::Cog,
        DefenseDomain::Cyb,
        DefenseDomain::Inf,
        DefenseDomain::Kin,
        DefenseDomain::Mkt,
    ];
    let rank_line: String = domains
        .iter()
        .enumerate()
        .map(|(i, d)| format!("{}:{}", d.word(), posture.ranks[i]))
        .collect::<Vec<_>>()
        .join(" ");
    lines.push(format!("  ranks {}", style.value(&rank_line)));
    let mi = home.market_integrity();
    if mi.sealed {
        lines.push(format!(
            "  {} until t{} · cascade {}",
            style.warn("MKT CIRCUIT"),
            mi.resume_at.unwrap_or(0),
            mi.cascade_pressure
        ));
    } else {
        lines.push(format!(
            "  MKT depth crystal {}/{} iron {}/{} · cascade {}",
            mi.crystal_depth, mi.seal_need, mi.iron_depth, mi.seal_need, mi.cascade_pressure
        ));
    }
    let atm = home.medium_integrity(
        home.orb()
            .map(|o| o.orb_id)
            .unwrap_or(cfe::world::FIRST_ORB),
    );
    let atm_paint = if atm.compromised {
        style.warn("COMPROMISED")
    } else {
        style.ok("stable")
    };
    lines.push(format!(
        "  ATM pressure {} / {} · {} · spend {:?}",
        ratio_str(&atm.pressure),
        ratio_str(&atm.threshold),
        atm_paint,
        atm.reinforce_resource
    ));
    lines.push(format!("  {}", style.dim(atm.residual_voice)));
    let inf = home.inf_integrity();
    let veil_w = if inf.veiled {
        style.warn("VEIL")
    } else {
        style.dim("clear")
    };
    let ward_w = if inf.ward_holds {
        style.warn(inf.ward_state.word())
    } else {
        style.dim(inf.ward_state.word())
    };
    lines.push(format!(
        "  INF rank {} · {} · ward {} · detect g{} · instruments {}",
        inf.inf_rank, veil_w, ward_w, inf.detection_grade, inf.instrument_count
    ));
    let cog = home.cog_integrity();
    let cyb = home.cyb_integrity();
    lines.push(format!(
        "  COG rank {} · wants {} · practices {} · {}",
        cog.cog_rank,
        cog.want_load,
        cog.practice_load,
        if cog.stressed {
            style.warn("STRESSED")
        } else {
            style.ok("steady")
        }
    ));
    lines.push(format!(
        "  CYB rank {} · mesh_ready={} · {}",
        cyb.cyb_rank,
        cyb.mesh_ready,
        style.dim(cyb.voice)
    ));
    let esp = home.espionage_integrity();
    lines.push(format!(
        "  SPY marks {} · intel {} · probe={}",
        esp.spy_marks,
        esp.intel_count,
        if esp.can_probe {
            style.ok("ready")
        } else {
            style.dim("need instrument")
        }
    ));
    lines.push(format!(
        "  {}",
        style.hint("spy probe · veil|ward · posture · escort · market · forge · suite spy")
    ));
    // KIN colour (P5-K2)
    let kin_deal = home
        .routes()
        .first()
        .map(|(d, _)| *d)
        .or_else(|| {
            home.hulls()
                .iter()
                .find(|h| h.assigned_route != 0)
                .map(|h| h.assigned_route)
        })
        .unwrap_or(0);
    let ki = home.kin_integrity(kin_deal);
    lines.push(format!(
        "  KIN residual {:?} · {}",
        ki.residual,
        style.dim(ki.residual_voice)
    ));
    // Escorts on duty
    let escorts: Vec<_> = home
        .hulls()
        .iter()
        .filter(|h| {
            h.assigned_route != 0
                && cfe::dop::HullRole::from_wire(h.role) == Some(cfe::dop::HullRole::Escort)
        })
        .collect();
    if escorts.is_empty() {
        lines.push(format!(
            "  {}",
            style.dim("(no escorts on duty — corridors EXPOSED at base risk 1/8)")
        ));
    } else {
        lines.push(format!("  {}", style.label("ESCORTS ON DUTY")));
        for h in escorts.iter().take(8) {
            let k = home.kin_integrity(h.assigned_route);
            let paint = match k.band {
                RiskBand::Secure => style.ok(k.band.word()),
                RiskBand::Exposed => style.warn(k.band.word()),
                RiskBand::Contested => style.bad(k.band.word()),
            };
            lines.push(format!(
                "    hull {} g{} → deal {} · risk {} · {}",
                h.hull_id,
                h.grade,
                h.assigned_route,
                ratio_str(&k.risk),
                paint
            ));
        }
    }
    // Sample live route deals
    let deals: Vec<u64> = home
        .routes()
        .iter()
        .map(|(d, _)| *d)
        .chain(
            home.committed()
                .element_routes
                .iter()
                .map(|(d, _, _, _)| *d),
        )
        .collect();
    if !deals.is_empty() {
        lines.push(format!("  {}", style.label("CORRIDOR RISK")));
        for deal in deals.into_iter().take(6) {
            let risk = home.convoy_risk(deal);
            let cover = home.escort_cover(deal);
            let band = home.convoy_risk_band(deal);
            lines.push(format!(
                "    deal {} · risk {} · cover {} · {}",
                deal,
                ratio_str(&risk),
                ratio_str(&cover),
                band.word()
            ));
        }
    }
    lines
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
        style.hint("renounce: treaty renounce <deal> <world>  ·  fence: treaty elements <deal> <world> <id>...  ·  help diplo")
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
            style.hint("upkeep 1/4 ember / year-voice")
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

    // Projected melt windows vs nearest open peer (Phase 2 P2.6).
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
    let span = universe_span();
    for a in 0..n {
        for b in (a + 1)..n {
            let d_ab = spatial_l1(&worlds[a].2, &worlds[b].2, &span);
            let melt_ab = desk_route_melt(&worlds[a].1, &worlds[b].1, &d_ab);
            if melt_ab < ratio(1, 1) {
                continue;
            }
            for m in 0..n {
                if m == a || m == b {
                    continue;
                }
                let d_am = spatial_l1(&worlds[a].2, &worlds[m].2, &span);
                let d_mb = spatial_l1(&worlds[m].2, &worlds[b].2, &span);
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

fn render_fleet(style: Style, home: &World) -> Vec<String> {
    let scrap = home
        .orb()
        .map(|o| ratio_str(&o.scrap_amount()))
        .unwrap_or_else(|| "0/1".into());
    let mut lines = vec![format!(
        "  {} {}  {} {}  {} {}",
        style.money("scrap"),
        style.cargo(&scrap),
        style.label("free cap"),
        style.value(&ratio_str(&home.free_capacity(0))),
        style.label("hulls"),
        style.strong(&home.hulls().len().to_string())
    )];
    let metab = home.committed().genesis.as_ref().map(|g| g.metabolism);
    for h in home.hulls() {
        let role = cfe::dop::HullRole::from_wire(h.role)
            .map(|r| cfe::world::hull_voice(r, metab))
            .unwrap_or("?");
        let assign = if h.assigned_route == 0 {
            "idle".into()
        } else {
            format!("deal {}", h.assigned_route)
        };
        lines.push(format!(
            "  hull {} · {} g{} · cap {} · {}",
            style.strong(&h.hull_id.to_string()),
            style.accent(role),
            h.grade,
            style.cargo(&ratio_str(&cfe::dop::hull_capacity(h.role, h.grade))),
            style.route(&assign)
        ));
    }
    if home.hulls().is_empty() {
        lines.push(format!(
            "  {}",
            style.hint("zone <tile> yard · fleet salvage <n> · fleet mint hauler")
        ));
    }
    lines.push(format!(
        "  {}",
        style.hint("positive-melt hauls require free hull capacity (Phase 3)")
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
    let zone_line = committed
        .orbs
        .iter()
        .find(|o| o.orb_id == orb_id)
        .map(|orb| {
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
        cfe::dop::Resource::Ember => "ember",
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

#[cfg(test)]
mod h13_hub_deep_link_pins {
    use super::*;

    #[test]
    fn every_pane_has_two_hub_links() {
        for &pane in Pane::all() {
            let links = pane.hub_links();
            assert!(
                links.len() >= 2,
                "{} must deep-link activity + craft (got {links:?})",
                pane.label()
            );
            assert_eq!(pane.hub_primary(), links[0]);
            assert_eq!(pane.hub_craft(), Some(links[1]));
        }
    }

    #[test]
    fn pane_hub_map_matches_h1_3_contract() {
        // Activity panes → activity root domain first; craft twin second.
        assert_eq!(Pane::Trade.hub_links(), &["trade books", "bench trade"]);
        assert_eq!(
            Pane::Diplo.hub_links(),
            &["diplomatic local", "bench diplomatic"]
        );
        assert_eq!(Pane::Routes.hub_links(), &["trade rails", "bench econ"]);
        assert_eq!(Pane::Arb.hub_links(), &["trade books", "bench trade"]);
        assert_eq!(Pane::Fleet.hub_links(), &["trade fleet", "bench fleet"]);
        assert_eq!(Pane::Exch.hub_links(), &["trade books", "bench offices"]);
        // Craft-native panes still expose two bench domains (planetary/build).
        assert_eq!(Pane::Grow.hub_links(), &["bench planetary", "bench build"]);
        assert_eq!(Pane::Defend.hub_links(), &["intel opsec", "bench doctrine"]);
        assert_eq!(
            Pane::Explore.hub_links(),
            &["intel research", "bench research"]
        );
    }

    #[test]
    fn hub_line_renders_for_every_pane() {
        // Pure string surface — no world required.
        let style = Style::new(true);
        for &pane in Pane::all() {
            let line = hub_link_line(style, pane);
            assert!(
                line.contains(pane.hub_primary()),
                "hub line for {} missing primary {}",
                pane.label(),
                pane.hub_primary()
            );
            if let Some(craft) = pane.hub_craft() {
                assert!(
                    line.contains(craft),
                    "hub line for {} missing craft {craft}",
                    pane.label()
                );
            }
        }
    }

    #[test]
    fn diplomatic_pane_key_accepts_preferred_name() {
        assert_eq!(Pane::from_key("diplomatic"), Some(Pane::Diplo));
        assert_eq!(Pane::from_key("diplo"), Some(Pane::Diplo));
        assert_eq!(Pane::from_key("y"), Some(Pane::Diplo));
    }
}
