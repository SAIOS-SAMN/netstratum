//! BENCH + TRADE hubs — main workbench menus.
//!
//! **BENCH** = make / modify / know / link the world.
//! **TRADE** = circulate / settle what the bench produced.
//!
//! Navigation: every nested view ends with a **back-out bar**.
//! `back` · `..` · `up` · `home` · `return` always climb toward hub root.
//! Bare `bench` / `trade` always open the peer hub root.

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
    let plain_w = 40usize;
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

fn is_back(word: Option<&str>) -> bool {
    matches!(
        word,
        Some("back")
            | Some("..")
            | Some("up")
            | Some("home")
            | Some("return")
            | Some("root")
            | Some("exit")
            | Some("q")
    )
}

/// Footer on every nested bench view — always a path home.
fn foot_bench(style: &Style, parent: Option<&str>) {
    println!();
    println!("  {}", style.section("↩ BACK-OUT"));
    leaf(style, "bench", "← BENCH root (all domains)");
    if let Some(p) = parent {
        leaf(style, &format!("bench {p}"), "← domain menu");
    }
    leaf(style, "back · .. · up · home", "same as bench root from any leaf");
    leaf(style, "trade", "→ CIRCULATE hub");
}

fn foot_trade(style: &Style, parent: Option<&str>) {
    println!();
    println!("  {}", style.section("↩ BACK-OUT"));
    leaf(style, "trade", "← TRADE root (all domains)");
    if let Some(p) = parent {
        leaf(style, &format!("trade {p}"), "← domain menu");
    }
    leaf(style, "back · .. · up · home", "same as trade root");
    leaf(style, "bench", "→ MAKE hub");
}

// ── BENCH ────────────────────────────────────────────────────────────

/// `bench [domain [leaf…]]` — make / modify / know / link hub.
/// Takes `&mut World` so deep leaves can fire deeds (forge, probe, …).
pub fn bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let domain = words.next();
    if is_back(domain.as_deref()) {
        bench_index(style);
        return;
    }
    match domain.as_deref() {
        None | Some("help") | Some("?") | Some("menu") | Some("index") => bench_index(style),
        Some("planetary") | Some("planet") | Some("world") => {
            planetary(world, words, style, current)
        }
        Some("build") | Some("infra") | Some("zones") => {
            build_bench(world, words, style, current)
        }
        Some("star") | Some("sky") | Some("system") => star_bench(world, words, style, current),
        Some("chem") | Some("chemistry") | Some("science") | Some("bond") => {
            chem_bench(world, words, style, current)
        }
        Some("econ") | Some("economics") | Some("craft") => {
            econ_bench(world, words, style, current)
        }
        Some("research") | Some("tech") | Some("practice") | Some("lab") => {
            research_bench(world, words, style, current)
        }
        Some("intel") | Some("intelligence") | Some("spy") | Some("inf") => {
            intel_bench(world, words, style, current)
        }
        Some("link") | Some("cross") | Some("pollinate") | Some("pollination") | Some("union")
        | Some("coop") => link_bench(world, words, style, current),
        Some("doctrine") | Some("posture") | Some("defense") | Some("kin") => {
            doctrine_bench(world, words, style, current)
        }
        Some("estate") | Some("survey") | Some("land") => {
            estate_bench(world, words, style, current)
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
    banner(style, "BENCH — MAKE / MODIFY / KNOW / LINK");
    println!(
        "  {}",
        style.dim(
            "Primary workbench. Nested views always offer `bench` or `back` to return here."
        )
    );
    println!();
    println!("  {}", style.section("MATTER & PLACE"));
    row(
        style,
        "bench planetary",
        "resource mod → biologic · chem · terraform",
    );
    row(style, "bench build", "buildings / infra (zones, yard, regen…)");
    row(style, "bench star", "sun · site · thermal · capacity (open game)");
    row(style, "bench estate", "terrain chart · parcels · appraisal");
    println!();
    println!("  {}", style.section("KNOWLEDGE"));
    row(
        style,
        "bench chem",
        "alchemy science · bonds · iso · catalog · dark-7",
    );
    row(
        style,
        "bench research",
        "tech tree · forge · practices · constellation",
    );
    row(
        style,
        "bench intel",
        "detection · spy · veil/ward · instrument grade",
    );
    println!();
    println!("  {}", style.section("PEOPLE & POWER"));
    row(
        style,
        "bench link",
        "cross-pollination · license · council · absorb · leap",
    );
    row(
        style,
        "bench doctrine",
        "posture · defend · domains · reorient",
    );
    row(
        style,
        "bench econ",
        "craft trade components (instruments · routes · rails)",
    );
    println!();
    println!("  {}", style.section("NAV"));
    leaf(style, "back · .. · home", "this root from any leaf");
    leaf(style, "trade", "→ CIRCULATE hub (run what you built)");
    leaf(style, "help bench", "full map");
    println!();
    println!(
        "  {}",
        style.hint("From chem: type `bench` or `back` — never stuck in a leaf.")
    );
}

fn planetary(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let leaf_w = words.next();
    if is_back(leaf_w.as_deref()) {
        bench_index(style);
        return;
    }
    match leaf_w.as_deref() {
        None | Some("help") | Some("?") => {
            banner(style, "BENCH › PLANETARY — resource modification");
            row(style, "bench planetary biologic", "regen · biomass · living loop");
            row(style, "bench planetary chem", "→ chem science");
            row(style, "bench planetary terraform", "TERRA crown · site rework");
            println!();
            leaf(style, "prospect · assay · refine", "open veins · draw · refine");
            leaf(style, "zone … regen", "Regenerator (syntropy)");
            foot_bench(style, Some("planetary"));
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
                    "  biomass {} · regenerators {}",
                    amount(&o.banked_amount(Resource::Biomass)),
                    regen
                );
            }
            leaf(style, "zone <tile> regen|hab", "living infra");
            leaf(style, "collect", "settle growth");
            foot_bench(style, Some("planetary"));
        }
        Some("chem") | Some("chemistry") => {
            chem_bench(world, words, style, current);
        }
        Some("terra") | Some("terraform") | Some("tf") => {
            banner(style, "BENCH › PLANETARY › TERRAFORM");
            println!("  can_terraform={}", world.can_terraform());
            leaf(style, "terraform …", "crown deed");
            leaf(style, "site · medium · thermal", "place + ATM + heat");
            foot_bench(style, Some("planetary"));
        }
        Some(other) => {
            println!(
                "{} planetary leaf {} — biologic|chem|terraform · or `back`",
                style.bad("✗"),
                other
            );
            foot_bench(style, Some("planetary"));
        }
    }
}

fn build_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let leaf_w = words.next();
    if is_back(leaf_w.as_deref()) {
        bench_index(style);
        return;
    }
    match leaf_w.as_deref() {
        None | Some("help") | Some("?") | Some("list") => {
            banner(style, "BENCH › BUILD — buildings / infra");
            if let Some(o) = world.orb_by_id(current).or_else(|| world.orb()) {
                println!("  zones standing: {}", o.zones.len());
                for z in o.zones.iter().take(12) {
                    println!("    tile {} · {:?}", z.tile, z.kind);
                }
            }
            println!();
            leaf(style, "zone <tile> ext|hab|smelt|synth|enc|cryo|res|regen|yard", "");
            leaf(style, "raze <tile>", "remove");
            leaf(style, "forge <grade>", "instrument (also research)");
            foot_bench(style, Some("build"));
        }
        Some(other) => {
            println!(
                "{} use `zone <tile> <kind>` for the deed · `back` for bench root",
                style.bad("✗")
            );
            let _ = other;
            foot_bench(style, Some("build"));
        }
    }
}

fn star_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let leaf_w = words.next();
    if is_back(leaf_w.as_deref()) {
        bench_index(style);
        return;
    }
    match leaf_w.as_deref() {
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
            leaf(style, "star <class> · site · thermal · rings", "deeds");
            leaf(style, "terraform", "earned crown");
            println!(
                "  {}",
                style.hint("Open game C1.7: stellar capacity as play (not a free dial).")
            );
            foot_bench(style, Some("star"));
        }
        Some(other) => {
            println!("{} star leaf {} · `back` → bench", style.bad("✗"), other);
            foot_bench(style, Some("star"));
        }
    }
}

fn chem_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let mode = words.next();
    if is_back(mode.as_deref()) {
        bench_index(style);
        return;
    }
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
            println!("  {}", style.dim("no genesis seal — neutral axes"));
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
                let h = hex_prefix(g);
                println!("    · {}", &h[..8.min(h.len())]);
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
                .map(|m| {
                    let h = hex_prefix(m);
                    h[..6.min(h.len())].to_string()
                })
                .collect();
            let gh = hex_prefix(gem);
            println!(
                "  {} → {} · k{} · net {} (surv {} / dark {})",
                style.strong(&named.join(" + ")),
                style.strong(&gh[..8.min(gh.len())]),
                knowledge,
                amount(net),
                amount(survival),
                amount(dark)
            );
            match standing {
                None => println!("      {}", style.ok("✓ workable here")),
                Some(_) => println!("      {}", style.warn("⚠ place refuses")),
            }
        }
        println!();
        if let Some((_, _, _, standing, survival, dark, net)) = rows.first() {
            let pull = if *dark > *survival {
                "dark gravity leading — immediate need"
            } else {
                "survival utility leading — long-term civ need"
            };
            println!(
                "  {} top net {} · {}",
                style.warn("NEXT"),
                amount(net),
                pull
            );
            if standing.is_some() {
                println!(
                    "  {}",
                    style.dim("top set place-refuses — site heat / pressure / locality")
                );
            } else {
                println!(
                    "  {}",
                    style.dim("compound the top set · or iso/catalog to deepen creation")
                );
            }
        }
        leaf(style, "compound <p> <p> …", "bond set");
        leaf(style, "iso <a> <b>", "isomorphic union");
        leaf(style, "catalog <gem>", "chemical creation");
        leaf(style, "bench chem bonds", "unranked standing list");
        leaf(style, "bench intel", "ENVY pull from spy/intel stash");
        foot_bench(style, Some("chem"));
    } else {
        banner(style, "BENCH › CHEM — bond standing");
        let bench = world.bench_standing(current);
        if bench.is_empty() {
            println!("  {}", style.dim("nothing bonds"));
        }
        for (members, gem, knowledge, standing) in &bench {
            let named: Vec<String> = members
                .iter()
                .map(|m| {
                    let h = hex_prefix(m);
                    h[..6.min(h.len())].to_string()
                })
                .collect();
            let gh = hex_prefix(gem);
            println!(
                "  {} → {} · k{}",
                style.strong(&named.join(" + ")),
                style.strong(&gh[..8.min(gh.len())]),
                knowledge
            );
            match standing {
                None => println!("      {}", style.ok("✓ workable")),
                Some(_) => println!("      {}", style.warn("⚠ place gate")),
            }
        }
        leaf(style, "bench chem", "science scores + dark-7");
        foot_bench(style, Some("chem"));
    }
}

fn econ_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    _current: u64,
) {
    let leaf_w = words.next();
    if is_back(leaf_w.as_deref()) {
        bench_index(style);
        return;
    }
    match leaf_w.as_deref() {
        None | Some("help") | Some("?") | Some("status") => {
            banner(style, "BENCH › ECONOMICS — craft trade components");
            let deep = world.deep_rail_status();
            println!(
                "  Exchange={} · Vault={} · Anchor={}",
                world.exchange_seated(),
                world.vault_seated(),
                deep.anchor_seated
            );
            println!(
                "  notes/fut/share/perp open {}/{}/{}/{}",
                world.notes().iter().filter(|n| n.open).count(),
                world.futures().iter().filter(|f| f.open).count(),
                world.shares().iter().filter(|s| s.open).count(),
                world.perps().iter().filter(|p| p.open).count(),
            );
            println!();
            leaf(style, "forge · seat exchange|vault|anchor", "open craft doors");
            leaf(style, "note|future|share|perp issue/open", "list under MM");
            leaf(style, "charter · element charter · rail promote", "lanes");
            leaf(style, "stamp mint", "unit of account");
            println!(
                "  {}",
                style.hint("Run under TRADE: `trade books|fleet|rails|vault`.")
            );
            foot_bench(style, Some("econ"));
        }
        Some(other) => {
            println!("{} econ leaf {} · `back` → bench", style.bad("✗"), other);
            foot_bench(style, Some("econ"));
        }
    }
}

fn research_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let leaf_w = words.next();
    if is_back(leaf_w.as_deref()) {
        bench_index(style);
        return;
    }
    // Deep deed: bench research forge <condition>
    if matches!(leaf_w.as_deref(), Some("forge")) {
        let cond = words.next().and_then(|w| w.parse::<u8>().ok());
        match cond.and_then(cfe::world::survey::ForgeableGrade::new) {
            Some(g) => match world.forge_instrument(current, g) {
                Ok(()) => {
                    println!(
                        "{} forged instrument grade {} — detection now {}",
                        style.ok("◈"),
                        g.condition(),
                        world.detection_grade()
                    );
                }
                Err(e) => println!("{} forge refused: {e:?}", style.bad("✗")),
            },
            None => println!(
                "{} usage: {} — crystallized/crown condition only",
                style.bad("✗"),
                style.strong("bench research forge <condition>")
            ),
        }
        // re-show board after deed
    }
    if matches!(leaf_w.as_deref(), Some("practice")) {
        let code = words.next().and_then(|w| w.parse::<u8>().ok());
        match code {
            Some(p) => match world.adopt_practice(p) {
                Ok(()) => println!("{} practice {p} adopted", style.ok("◈")),
                Err(e) => println!("{} practice refused: {e:?}", style.bad("✗")),
            },
            None => println!(
                "{} usage: {}",
                style.bad("✗"),
                style.strong("bench research practice <1..5>")
            ),
        }
    }

    let board = world.research_board(current);
    banner(style, "BENCH › RESEARCH — constellation game");
    println!(
        "  held {} · forge-ready {} · instruments {} · practices {} · wants {}",
        board.held_count,
        board.forgeable_ready,
        board.instruments,
        board.practices,
        board.wants
    );
    if board.novelty_blocked {
        println!(
            "  {}",
            style.warn("NOVELTY WANT — crystallization halted until mind is fed")
        );
    }
    println!("  {} {}", style.dim("next →"), style.strong(&board.next_move));
    println!();
    println!("  {}", style.section("CONSTELLATION"));
    for n in &board.nodes {
        if n.threshold == 0 {
            continue;
        }
        let mark = if n.instrumented {
            "⚙"
        } else if n.held {
            "●"
        } else if n.progress > 0 {
            "○"
        } else {
            "·"
        };
        let bar = if n.threshold > 1 {
            format!(" {}/{}", n.progress, n.threshold)
        } else {
            String::new()
        };
        let forge = if n.forgeable { "  ← FORGE" } else { "" };
        println!(
            "  {mark} c{:<2} {:?}{bar}{forge}",
            n.condition, n.kind
        );
        if n.forgeable || (!n.held && n.progress > 0) {
            println!("      {}", style.dim(n.next_hint));
        }
    }
    println!();
    leaf(style, "bench research forge <c>", "deep: forge instrument at condition");
    leaf(style, "bench research practice <n>", "deep: adopt discipline");
    leaf(style, "forge <c> · practice · corpus · elevate", "same deeds outside hub");
    leaf(style, "bench link", "cross-pollinate tech via license");
    foot_bench(style, Some("research"));
}

fn intel_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let leaf_w = words.next();
    if is_back(leaf_w.as_deref()) {
        bench_index(style);
        return;
    }
    // Deep deeds on the pipeline
    match leaf_w.as_deref() {
        Some("probe") => match world.spy_probe(current) {
            Ok(d) => println!("{} probe depth {d}", style.ok("◈")),
            Err(e) => println!("{} probe refused: {e:?}", style.bad("✗")),
        },
        Some("exfil") => {
            let cond = words.next().and_then(|w| w.parse::<u8>().ok());
            match cond {
                Some(c) => match world.tech_exfil(current, c) {
                    Ok(h) => println!(
                        "{} exfil condition {c} · hash {:02x}{:02x}… (stash only)",
                        style.warn("EXFIL"),
                        h[0],
                        h[1]
                    ),
                    Err(e) => println!("{} exfil refused: {e:?}", style.bad("✗")),
                },
                None => println!(
                    "{} usage: {}",
                    style.bad("✗"),
                    style.strong("bench intel exfil <condition>")
                ),
            }
        }
        Some("sabotage") => match world.sabotage_instrument(current) {
            Ok(g) => println!("{} sabotaged instrument grade {g}", style.warn("SABOTAGE")),
            Err(e) => println!("{} sabotage refused: {e:?}", style.bad("✗")),
        },
        _ => {}
    }

    let board = world.intel_board(current);
    banner(style, "BENCH › INTEL — detection ladder · spy pipeline");
    println!(
        "  detection g{} · instruments {} · probes {} · stash {}",
        board.detection, board.instruments, board.probe_marks, board.intel_stash
    );
    println!(
        "  veiled={} · probe={} exfil={} sabotage={}",
        board.veiled, board.can_probe, board.can_exfil, board.can_sabotage
    );
    // Ladder visualization
    println!();
    println!("  {}", style.section("LADDER"));
    let steps = [
        (0u8, "surface"),
        (1, "probe-ready"),
        (2, "deep stratum"),
        (3, "optics"),
    ];
    for (need, label) in steps {
        let ok = board.detection >= need;
        println!(
            "  {} g≥{need} {label}",
            if ok {
                style.ok("✓")
            } else {
                style.dim("·")
            }
        );
    }
    println!();
    println!("  {}", style.section("PIPELINE"));
    println!(
        "  {} {}",
        style.warn("NEXT"),
        style.strong(&board.next_hint)
    );
    if !board.tech_conditions.is_empty() {
        println!(
            "  exfil targets (conditions held): {:?}",
            board.tech_conditions
        );
    }
    println!();
    leaf(style, "bench intel probe", "deep: scout mark (crystal fee)");
    leaf(style, "bench intel exfil <c>", "deep: hash → stash");
    leaf(style, "bench intel sabotage", "deep: remove instrument");
    leaf(style, "forge <g> · veil · scan", "raise ladder / OPSEC");
    leaf(style, "bench research", "forge path when detection thin");
    foot_bench(style, Some("intel"));
}

fn link_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    _current: u64,
) {
    let leaf_w = words.next();
    if is_back(leaf_w.as_deref()) {
        bench_index(style);
        return;
    }
    let board = world.link_board();
    banner(style, "BENCH › LINK — cross-pollination game");
    println!(
        "  score {} · crossed={} · councils {} (open {}) · bonds {} · forwards {}",
        amount(&board.score),
        board.crossed,
        board.councils,
        board.council_open,
        board.bonds,
        board.forwards
    );
    println!(
        "  treaties {} · war legs {} · licensable conditions {:?}",
        board.treaties, board.at_war_count, board.techs_licensable
    );
    println!("  {} {}", style.warn("NEXT"), style.strong(&board.next_hint));
    println!();
    println!("  {}", style.section("PLAY PATHS"));
    leaf(
        style,
        "council …",
        "open co-op (needed for license)",
    );
    leaf(
        style,
        "license <c> to <world> via <council> deal <n>",
        "tech leap across difference",
    );
    leaf(style, "absorb · bond · forward", "reciprocity mesh");
    leaf(style, "aperture · treaty", "value / oath (also TRADE)");
    println!();
    println!(
        "  {}",
        style.hint("Hail Mary: two different frames confirm — license is the play, not free mint.")
    );
    foot_bench(style, Some("link"));
    let _ = leaf_w;
}

fn doctrine_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    _current: u64,
) {
    let leaf_w = words.next();
    if is_back(leaf_w.as_deref()) {
        bench_index(style);
        return;
    }
    let board = world.doctrine_board();
    banner(style, "BENCH › DOCTRINE — domain ranks game");
    println!(
        "  primary {} · secondary {} · reoriented={}",
        board.primary, board.secondary, board.reoriented
    );
    println!(
        "  axes met={} dis={} per={} · kin {}",
        board.metabolism, board.disposition, board.perception, board.kin_voice
    );
    println!("  voice: {}", board.voice);
    println!();
    println!("  {}", style.section("RANKS CEL ATM COG CYB INF KIN MKT"));
    let names = ["CEL", "ATM", "COG", "CYB", "INF", "KIN", "MKT"];
    for (i, r) in board.ranks.iter().enumerate() {
        let bar = "█".repeat(*r as usize).to_string() + &"░".repeat(3usize.saturating_sub(*r as usize));
        println!("  {} {bar} ({r})", names[i]);
    }
    println!("  {} {}", style.warn("NEXT"), style.strong(&board.next_hint));
    println!();
    leaf(style, "posture set <dis> <per> [domain|clear]", "reorient doctrine");
    leaf(style, "defend escort|risk · suite convoy", "KIN play");
    leaf(style, "forge · market · veil", "raise weak domain ranks");
    foot_bench(style, Some("doctrine"));
    let _ = leaf_w;
}

fn estate_bench(
    world: &mut World,
    mut words: impl Iterator<Item = String>,
    style: &Style,
    current: u64,
) {
    let leaf_w = words.next();
    if is_back(leaf_w.as_deref()) {
        bench_index(style);
        return;
    }
    // Deep: survey terrain / estate
    if matches!(leaf_w.as_deref(), Some("chart") | Some("survey")) {
        match world.chart_terrain() {
            Ok(seed) => println!(
                "{} terrain charted · seed {:02x}{:02x}…",
                style.ok("◈"),
                seed[0],
                seed[1]
            ),
            Err(e) => println!("{} chart refused: {e:?}", style.bad("✗")),
        }
    }
    if matches!(leaf_w.as_deref(), Some("estate") | Some("ladder")) {
        match world.survey_estate(current) {
            Ok(h) => println!(
                "{} estate surveyed · {:02x}{:02x}…",
                style.ok("◈"),
                h[0],
                h[1]
            ),
            Err(e) => println!("{} estate refused: {e:?}", style.bad("✗")),
        }
    }

    banner(style, "BENCH › ESTATE — land game");
    let seeded = world.terrain_seed().is_some();
    println!("  terrain charted={}", seeded);
    if let Some(parcels) = world.parcels() {
        println!("  parcels {}", parcels.len());
        for (i, p) in parcels.iter().take(8).enumerate() {
            println!("    #{i} {:?}", p);
        }
    } else {
        println!("  {}", style.dim("no parcel map — survey estate first"));
    }
    println!();
    leaf(style, "bench estate chart", "deep: chart terrain seed");
    leaf(style, "bench estate ladder", "deep: survey_estate");
    leaf(style, "estate parcels · appraise …", "valuation deeds");
    foot_bench(style, Some("estate"));
}

// ── TRADE ────────────────────────────────────────────────────────────

/// `trade [domain]` hub when not in barter form.
pub fn trade(world: &World, mut words: impl Iterator<Item = String>, style: &Style, current: u64) {
    let domain = words.next();
    if is_back(domain.as_deref()) {
        trade_index(style);
        return;
    }
    match domain.as_deref() {
        None | Some("help") | Some("?") | Some("menu") | Some("index") => trade_index(style),
        Some("books") | Some("market") | Some("mkt") | Some("instruments") => {
            trade_books(world, style)
        }
        Some("fleet") | Some("hull") | Some("ships") => trade_fleet(world, style),
        Some("rails") | Some("rail") | Some("anchor") => trade_rails(world, style),
        Some("vault") | Some("custody") => trade_vault(world, style, current),
        Some("aperture") | Some("haul") | Some("cross") => trade_aperture(style),
        Some("diplo") | Some("diplomacy") | Some("treaty") => trade_diplo(world, style),
        Some("desk") => {
            println!(
                "  {}",
                style.hint("interactive desk: `desk enter` · panes t|y|r|a|f …")
            );
            foot_trade(style, None);
        }
        Some("barter") | Some("clear") => {
            println!(
                "{} barter form: {}",
                style.dim("↳"),
                style.strong("trade <my_offer> with <world> <their_offer> deal <n>")
            );
            foot_trade(style, None);
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
        style.dim("Run components the BENCH crafted. Nested views: `trade` or `back` home.")
    );
    println!();
    println!("  {}", style.section("DOMAINS"));
    row(style, "trade books", "offers · instruments · market war");
    row(style, "trade fleet", "scrap · hulls · capacity · escort");
    row(style, "trade rails", "R1–R4 · Anchor");
    row(style, "trade vault", "custody · margin");
    row(style, "trade aperture", "cross-world haul");
    row(style, "trade diplo", "treaties · war wall");
    row(style, "trade desk", "interactive suite");
    println!();
    leaf(
        style,
        "trade <my_id> with <world> <their_id> deal <n>",
        "barter clear",
    );
    leaf(style, "back · bench", "nav");
}

fn trade_books(world: &World, style: &Style) {
    banner(style, "TRADE › BOOKS");
    let mi = world.market_integrity();
    println!(
        "  sealed={} · Exchange={} · open note/fut/share/perp={}/{}/{}/{}",
        mi.sealed,
        world.exchange_seated(),
        mi.notes_open,
        mi.futures_open,
        mi.shares_open,
        mi.perps_open
    );
    leaf(style, "offer · note|future|share|perp · market · stamp", "deeds");
    foot_trade(style, Some("books"));
}

fn trade_fleet(world: &World, style: &Style) {
    banner(style, "TRADE › FLEET");
    let free = world.free_capacity(0);
    println!(
        "  hulls {} · free capacity (idle) {}",
        world.hulls().len(),
        amount(&free)
    );
    leaf(style, "fleet mint|assign · defend escort", "deeds");
    foot_trade(style, Some("fleet"));
}

fn trade_rails(world: &World, style: &Style) {
    banner(style, "TRADE › RAILS");
    let d = world.deep_rail_status();
    println!(
        "  Anchor={} · strand={} · court={} · element routes {}",
        d.anchor_seated,
        d.strand_enabled,
        d.court_enabled,
        world.element_routes().len()
    );
    leaf(style, "rail promote · anchor export|import", "deeds");
    foot_trade(style, Some("rails"));
}

fn trade_vault(world: &World, style: &Style, current: u64) {
    banner(style, "TRADE › VAULT");
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
    leaf(style, "vault deposit|withdraw", "deeds");
    foot_trade(style, Some("vault"));
}

fn trade_aperture(style: &Style) {
    banner(style, "TRADE › APERTURE");
    leaf(style, "aperture · element aperture · stamp aperture", "haul");
    leaf(style, "treaty · war wall", "diplo gates");
    foot_trade(style, Some("aperture"));
}

fn trade_diplo(world: &World, style: &Style) {
    banner(style, "TRADE › DIPLO");
    println!("  treaties standing: {}", world.committed().treaties.len());
    leaf(style, "treaty trade|defense|peace|war with …", "seal");
    leaf(style, "treaties · renounce", "list / drop leg");
    foot_trade(style, Some("diplo"));
}
