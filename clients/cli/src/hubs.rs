//! BENCH + TRADE hubs — main workbench menus.
//!
//! **BENCH** = make / modify the world (planetary · build · star · chem · economics craft).
//! **TRADE** = circulate / settle what the bench produced (books · fleet · rails · vault · aperture).
//!
//! Hubs are presentation: they surface status and route language to existing
//! verbs. They do not invent a second physics.

use cfe::dop::{Resource, ZoneKind};
use cfe::ratio::{ratio, Ratio};
use cfe::world::World;

use crate::style::Style;

fn amount(value: &Ratio) -> String {
    format!("{}/{}", value.numer(), value.denom())
}

fn hex_prefix(bytes: &[u8; 32]) -> String {
    bytes
        .iter()
        .take(4)
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn banner(style: &Style, title: &str) {
    println!(
        "{}",
        style.border("════════════════════════════════════════════════════════════")
    );
    println!("  {}", style.title(title));
    println!(
        "{}",
        style.border("════════════════════════════════════════════════════════════")
    );
}

fn row(style: &Style, verb: &str, desc: &str) {
    let plain_w = 36usize;
    let pad = if verb.len() < plain_w {
        " ".repeat(plain_w - verb.len())
    } else {
        String::new()
    };
    println!("  {}{}{}", style.verb(verb), pad, style.dim(desc));
}

fn leaf(style: &Style, verb: &str, desc: &str) {
    println!("    {}  {}", style.key(verb), style.dim(desc));
}

// ── BENCH ────────────────────────────────────────────────────────────

/// `bench [domain [leaf…]]` — make / modify hub.
pub fn bench(world: &World, mut words: impl Iterator<Item = String>, style: &Style, current: u64) {
    let domain = words.next();
    match domain.as_deref() {
        None | Some("help") | Some("?") | Some("menu") | Some("index") => bench_index(style),
        Some("planetary") | Some("planet") | Some("world") => {
            planetary(world, words, style, current)
        }
        Some("build") | Some("infra") | Some("zone") => build_bench(world, words, style, current),
        Some("star") | Some("sky") | Some("system") => star_bench(world, words, style, current),
        Some("chem") | Some("chemistry") | Some("science") | Some("bond") => {
            chem_bench(world, words, style, current)
        }
        Some("econ") | Some("economics") | Some("craft") => {
            econ_bench(world, words, style, current)
        }
        Some(other) => {
            println!(
                "{} unknown bench domain {} — try {}",
                style.bad("✗"),
                style.verb(other),
                style.key("bench")
            );
            bench_index(style);
        }
    }
}

fn bench_index(style: &Style) {
    banner(style, "BENCH — MAKE / MODIFY");
    println!(
        "  {}",
        style.dim("Craft and reshape the world. Components flow to TRADE for circulation.")
    );
    println!();
    println!("  {}", style.section("DOMAINS"));
    row(
        style,
        "bench planetary",
        "resource mod → biologic · chem · terraform",
    );
    row(style, "bench build", "buildings / infra (zones, yard, regen…)");
    row(style, "bench star", "sun · site · rings · thermal · terraform crown");
    row(
        style,
        "bench chem",
        "alchemy science · bonds · iso · catalog · dark-7",
    );
    row(
        style,
        "bench econ",
        "craft trade components (forge · instruments · routes · rails)",
    );
    println!();
    println!("  {}", style.section("SHORTCUTS (same deeds)"));
    leaf(style, "chemistry · compound · iso · catalog", "chem leaves");
    leaf(style, "zone · star · site · terraform · forge", "direct build verbs");
    println!();
    println!(
        "  {}",
        style.hint("Peer hub: `trade` (or `trades`) opens circulation / settlement.")
    );
}

fn planetary(
    world: &World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    match words.next().as_deref() {
        None | Some("help") | Some("?") => {
            banner(style, "BENCH › PLANETARY — resource modification");
            row(style, "bench planetary biologic", "regen · biomass · living loop");
            row(style, "bench planetary chem", "→ chem science (bonds / catalog)");
            row(style, "bench planetary terraform", "TERRA crown · site rework");
            println!();
            leaf(style, "prospect · assay · refine", "open veins · draw · refine");
            leaf(style, "zone … regen", "Regenerator (syntropy)");
            leaf(style, "terraform", "gifted crown only");
        }
        Some("bio") | Some("biologic") | Some("living") => {
            banner(style, "BENCH › PLANETARY › BIOLOGIC");
            if let Some(o) = world.orb_by_id(current).or_else(|| world.orb()) {
                let regen = o
                    .zones
                    .iter()
                    .filter(|z| z.kind == ZoneKind::Regenerator)
                    .count();
                println!(
                    "  biomass {} · regenerators {} · balance path: entropy∥syntropy",
                    amount(&o.banked_amount(Resource::Biomass)),
                    regen
                );
            }
            leaf(style, "zone <tile> regen", "build Regenerator");
            leaf(style, "zone <tile> hab", "Habitat (biotic labour later)");
            leaf(style, "collect", "settle workspace after growth");
            println!(
                "  {}",
                style.hint("Order-work (refine/compound) may refuse when entropy dominates (34).")
            );
        }
        Some("chem") | Some("chemistry") => {
            chem_bench(world, words, style, current);
        }
        Some("terra") | Some("terraform") | Some("tf") => {
            banner(style, "BENCH › PLANETARY › TERRAFORM");
            println!(
                "  can_terraform={} · dual-use ATM tax when TERRA held",
                world.can_terraform()
            );
            leaf(style, "terraform …", "crown deed (base-7 stack)");
            leaf(style, "site <A> <r>", "place orbit before heat work");
            leaf(style, "medium · tamper", "ATM envelope / dual-use residual");
            leaf(style, "thermal", "band + balance reading");
        }
        Some(other) => {
            println!(
                "{} planetary leaf {} unknown — biologic|chem|terraform",
                style.bad("✗"),
                other
            );
        }
    }
}

fn build_bench(
    world: &World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    match words.next().as_deref() {
        None | Some("help") | Some("?") | Some("list") => {
            banner(style, "BENCH › BUILD — buildings / infra");
            if let Some(o) = world.orb_by_id(current).or_else(|| world.orb()) {
                println!("  zones standing: {}", o.zones.len());
                for z in o.zones.iter().take(12) {
                    println!(
                        "    tile {} · {:?}",
                        z.tile,
                        z.kind
                    );
                }
            }
            println!();
            println!("  {}", style.section("ZONE KINDS"));
            leaf(style, "zone <tile> ext|hab|smelt|synth", "core colony");
            leaf(style, "zone <tile> enc|cryo|res|regen", "thermal / science");
            leaf(style, "zone <tile> yard", "ShipYard → fleet");
            leaf(style, "zone <tile> refinery", "ElementRefinery (when deepen)");
            println!();
            leaf(style, "raze <tile>", "remove zone");
            leaf(style, "forge <grade>", "instrument (MANU-1)");
            println!(
                "  {}",
                style.hint("Built pieces enable TRADE (haul capacity, refine path, market evidence).")
            );
        }
        Some(other) => {
            println!(
                "{} build leaf {} — use `zone <tile> <kind>` for the deed; this hub lists standing infra",
                style.bad("✗"),
                other
            );
        }
    }
}

fn star_bench(
    world: &World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    match words.next().as_deref() {
        None | Some("help") | Some("?") | Some("status") => {
            banner(style, "BENCH › STAR — system capacity");
            match world.star() {
                Some(c) => println!("  sun commissioned · class {:?}", c),
                None => println!("  {}", style.warn("no sun — `star <class>` first")),
            }
            if let Some(o) = world.orb_by_id(current).or_else(|| world.orb()) {
                if let Some(t) = o.thermal.as_ref() {
                    println!(
                        "  site albedo {} · orbit_r {}",
                        amount(t.albedo.value()),
                        amount(t.orbit_r.value())
                    );
                } else {
                    println!("  {}", style.dim("unsited — `site <A> <r>`"));
                }
            }
            println!();
            leaf(style, "star <M|K|G|F|A|B|O|…>", "commission sun once");
            leaf(style, "site <albedo> <orbit_r>", "place genesis orb");
            leaf(style, "thermal · rings · sky", "readings");
            leaf(style, "terraform", "crown rework (earned)");
            println!(
                "  {}",
                style.hint("Open game: stellar capacity project (C1.7) — lines_of as play, not dial.")
            );
        }
        Some(other) => {
            println!(
                "{} star leaf {} — use star|site|thermal verbs; hub is status + map",
                style.bad("✗"),
                other
            );
        }
    }
}

fn chem_bench(
    world: &World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let mode = words.next();
    // bare chem / science → ranked science view; bonds → standing list
    let science = !matches!(
        mode.as_deref(),
        Some("bonds") | Some("list") | Some("standing")
    );

    if science {
        banner(style, "BENCH › CHEM — science path");
        if let Some(ax) = world.civ_axes() {
            println!(
                "  axes dis={} met={} per={}",
                ax.disposition, ax.metabolism, ax.perception
            );
        } else {
            println!("  {}", style.dim("no genesis seal — utility uses neutral axes"));
        }
        let pulls = world.dark_pulls();
        if pulls.is_empty() {
            println!("  {}", style.dim("dark-7 silent"));
        } else {
            for (p, w) in &pulls {
                println!(
                    "  {} {} · {} — {}",
                    style.warn("DARK"),
                    p.word(),
                    amount(w),
                    p.gloss()
                );
            }
        }
        let cat = world.chem_catalog();
        if !cat.is_empty() {
            println!("  catalog {} known", cat.len());
            for g in cat.iter().take(8) {
                println!("    · {}", &hex_prefix(g)[..8.min(hex_prefix(g).len())]);
            }
        }
        let rows = world.bench_science(current);
        if rows.is_empty() {
            println!(
                "  {}",
                style.dim("nothing bonds — prospect/assay more or trade for matter")
            );
        }
        for (members, gem, knowledge, standing, survival, dark, net) in &rows {
            let named: Vec<String> = members
                .iter()
                .map(|m| hex_prefix(m)[..6.min(hex_prefix(m).len())].to_string())
                .collect();
            println!(
                "  {} → {} · k{} · net {} (surv {} / dark {})",
                style.strong(&named.join(" + ")),
                style.strong(&hex_prefix(gem)[..8.min(hex_prefix(gem).len())]),
                knowledge,
                amount(net),
                amount(survival),
                amount(dark)
            );
            match standing {
                None => println!("      {}", style.ok("✓ workable here")),
                Some(_) => println!("      {}", style.warn("⚠ place refuses — see compound error")),
            }
        }
        println!();
        leaf(style, "compound <p> <p> …", "bond set");
        leaf(style, "iso <a> <b>", "isomorphic union");
        leaf(style, "catalog <gem>", "chemical creation (known recipe)");
        leaf(style, "bench chem bonds", "unranked standing list");
    } else {
        banner(style, "BENCH › CHEM — bond standing");
        let bench = world.bench_standing(current);
        if bench.is_empty() {
            println!(
                "  {}",
                style.dim("nothing bonds — one geometry, one phase, saturating binds")
            );
        }
        for (members, gem, knowledge, standing) in &bench {
            let named: Vec<String> = members
                .iter()
                .map(|m| hex_prefix(m)[..6.min(hex_prefix(m).len())].to_string())
                .collect();
            println!(
                "  {} → {} · k{}",
                style.strong(&named.join(" + ")),
                style.strong(&hex_prefix(gem)[..8.min(hex_prefix(gem).len())]),
                knowledge
            );
            match standing {
                None => println!("      {}", style.ok("✓ workable")),
                Some(_) => println!("      {}", style.warn("⚠ place gate")),
            }
        }
        leaf(style, "bench chem", "science scores + dark-7");
    }
}

fn econ_bench(
    world: &World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    _current: u64,
) {
    match words.next().as_deref() {
        None | Some("help") | Some("?") | Some("status") => {
            banner(style, "BENCH › ECONOMICS — craft trade components");
            let deep = world.deep_rail_status();
            println!(
                "  Exchange seated={} · Vault={} · Anchor={}",
                world.exchange_seated(),
                world.vault_seated(),
                deep.anchor_seated
            );
            println!(
                "  notes open {} · futures {} · shares {} · perps {}",
                world.notes().iter().filter(|n| n.open).count(),
                world.futures().iter().filter(|f| f.open).count(),
                world.shares().iter().filter(|s| s.open).count(),
                world.perps().iter().filter(|p| p.open).count(),
            );
            println!();
            println!("  {}", style.section("CRAFT (then run under TRADE)"));
            leaf(style, "forge <grade>", "instrument → detection / books");
            leaf(style, "seat exchange|vault|anchor", "open offices");
            leaf(style, "note issue · future open · share · perp", "list under MM");
            leaf(style, "charter · element charter", "route components");
            leaf(style, "rail promote · stamp mint", "altitude · STAMP unit");
            println!();
            println!(
                "  {}",
                style.hint("TRADE hub runs these: `trade books|fleet|rails|vault|aperture`.")
            );
        }
        Some(other) => {
            println!(
                "{} econ leaf {} — forge|note|charter|seat verbs craft the component",
                style.bad("✗"),
                other
            );
        }
    }
}

// ── TRADE ────────────────────────────────────────────────────────────

/// `trade [domain]` hub when not in barter form `trade <id> with …`.
pub fn trade(world: &World, mut words: impl Iterator<Item = String>, style: &Style, current: u64) {
    let domain = words.next();
    match domain.as_deref() {
        None | Some("help") | Some("?") | Some("menu") | Some("index") => trade_index(style),
        Some("books") | Some("market") | Some("mkt") | Some("instruments") => {
            trade_books(world, style)
        }
        Some("fleet") | Some("hull") | Some("ships") => trade_fleet(world, style, current),
        Some("rails") | Some("rail") | Some("anchor") => trade_rails(world, style),
        Some("vault") | Some("custody") => trade_vault(world, style, current),
        Some("aperture") | Some("haul") | Some("cross") => trade_aperture(style),
        Some("desk") => {
            println!(
                "  {}",
                style.hint("interactive desk: `desk enter` · panes t|y|r|a|f …")
            );
        }
        Some("barter") | Some("clear") => {
            println!(
                "{} barter form: {}",
                style.dim("↳"),
                style.strong("trade <my_offer> with <world> <their_offer> deal <n>")
            );
        }
        Some(other) => {
            println!(
                "{} unknown trade domain {} — try {}",
                style.bad("✗"),
                style.verb(other),
                style.key("trade")
            );
            trade_index(style);
        }
    }
}

fn trade_index(style: &Style) {
    banner(style, "TRADE — CIRCULATE / SETTLE");
    println!(
        "  {}",
        style.dim("Run components the BENCH crafted. Make stays on `bench`.")
    );
    println!();
    println!("  {}", style.section("DOMAINS"));
    row(style, "trade books", "offers · clears · STAMP instruments · market war");
    row(style, "trade fleet", "scrap · hulls · capacity · escort");
    row(style, "trade rails", "R1–R4 · promote · Anchor import/export");
    row(style, "trade vault", "custody deposit/withdraw · margin reserve");
    row(style, "trade aperture", "cross-world haul · element/stamp aperture");
    row(style, "trade desk", "interactive suite (desk enter)");
    println!();
    println!("  {}", style.section("BARTER (direct)"));
    leaf(
        style,
        "trade <my_id> with <world> <their_id> deal <n>",
        "clear crossing",
    );
    leaf(style, "offer · stamp · element aperture", "post / move value");
    println!();
    println!(
        "  {}",
        style.hint("Peer hub: `bench` opens make/modify (planetary · build · star · chem · econ).")
    );
}

fn trade_books(world: &World, style: &Style) {
    banner(style, "TRADE › BOOKS — instruments & market");
    let mi = world.market_integrity();
    println!(
        "  sealed={} · Exchange={} · notes/fut/share/perp open={}/{}/{}/{}",
        mi.sealed,
        world.exchange_seated(),
        mi.notes_open,
        mi.futures_open,
        mi.shares_open,
        mi.perps_open
    );
    println!(
        "  clear depth crystal {}/{} · offers {}",
        mi.crystal_depth,
        mi.seal_need,
        world.offers().len()
    );
    println!();
    leaf(style, "offer … · trade <id> with …", "post / clear");
    leaf(style, "note|future|share|perp …", "instrument lifecycle (MM)");
    leaf(style, "market status|seal|clear|raid|cascade", "MKT war");
    leaf(style, "stamp mint|aperture", "unit of account");
    leaf(style, "suite market", "war UI over same deeds");
}

fn trade_fleet(world: &World, style: &Style, _current: u64) {
    banner(style, "TRADE › FLEET — capacity in motion");
    // Idle pool capacity for deal 0 (unassigned free).
    let free = world.free_capacity(0);
    println!(
        "  hulls {} · free capacity (idle pool) {}",
        world.hulls().len(),
        amount(&free)
    );
    leaf(style, "fleet · fleet mint hauler|escort [g] finance", "mint");
    leaf(style, "fleet assign …", "route assignment");
    leaf(style, "defend escort|risk", "KIN convoy");
    leaf(style, "zone … yard · salvage", "bench-built yard feeds fleet");
}

fn trade_rails(world: &World, style: &Style) {
    banner(style, "TRADE › RAILS — settlement altitude");
    let d = world.deep_rail_status();
    println!(
        "  Anchor seated={} · strand_feat={} · court_feat={}",
        d.anchor_seated, d.strand_enabled, d.court_enabled
    );
    println!("  element routes: {}", world.element_routes().len());
    leaf(style, "rail status|promote", "R1→R2; R3/R4 gated");
    leaf(style, "anchor export-stamp|import-*", "X7 exit");
    leaf(style, "element charter … [rail]", "lane component (bench econ crafts)");
}

fn trade_vault(world: &World, style: &Style, current: u64) {
    banner(style, "TRADE › VAULT — custody & margin");
    let bal = world.vault_balance_of(current);
    let res = world.vault_reserved(current);
    let free = if bal > res {
        bal.clone() - res.clone()
    } else {
        ratio(0, 1)
    };
    println!(
        "  seated={} · custody {} · reserved {} · free {}",
        world.vault_seated(),
        amount(&bal),
        amount(&res),
        amount(&free)
    );
    leaf(style, "vault deposit|withdraw <n>", "custody move");
    leaf(style, "seat vault", "open T8 (bench/office)");
    leaf(style, "note issue …", "margin earmark when vault sits");
}

fn trade_aperture(style: &Style) {
    banner(style, "TRADE › APERTURE — cross-world legs");
    leaf(style, "aperture …", "resource haul + melt");
    leaf(style, "element aperture …", "open-matter haul");
    leaf(style, "stamp aperture …", "STAMP cross-book");
    leaf(style, "treaty · war wall", "diplo gates the door");
    println!(
        "  {}",
        style.hint("Melt from D_eff; hull capacity required when melt > 0.")
    );
}
