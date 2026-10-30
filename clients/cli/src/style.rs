//! CLI presentation — full ansi attribute palette (Charter client voice).
//!
//! Respects `--plain` / `NO_COLOR`. Every semantic role has a color so
//! help menus and the trade desk stay readable under full color.

/// Presentation style for the Netstratum CLI.
#[derive(Clone, Copy)]
pub struct Style {
    pub plain: bool,
}

impl Style {
    pub fn new(plain: bool) -> Self {
        Self { plain }
    }

    fn wrap(self, code: &str, text: &str) -> String {
        if self.plain {
            text.to_string()
        } else {
            format!("\x1b[{code}m{text}\x1b[0m")
        }
    }

    // ── core roles (legacy names kept) ───────────────────────────
    pub fn ok(self, text: &str) -> String {
        self.wrap("1;32", text) // bold green — sealed success
    }
    pub fn warn(self, text: &str) -> String {
        self.wrap("1;33", text) // bold yellow — caution
    }
    pub fn bad(self, text: &str) -> String {
        self.wrap("1;31", text) // bold red — refusal
    }
    pub fn head(self, text: &str) -> String {
        self.wrap("1;36", text) // bold cyan — section headers
    }
    pub fn strong(self, text: &str) -> String {
        self.wrap("1", text) // bold
    }
    pub fn dim(self, text: &str) -> String {
        self.wrap("2", text) // dim structure
    }
    pub fn prompt(self, text: &str) -> String {
        self.wrap("1;35", text) // bold magenta
    }

    // ── extended palette (help + desk) ───────────────────────────
    pub fn title(self, text: &str) -> String {
        self.wrap("1;96", text) // bright cyan title
    }
    pub fn section(self, text: &str) -> String {
        self.wrap("1;94", text) // bright blue section
    }
    pub fn verb(self, text: &str) -> String {
        self.wrap("1;93", text) // bright yellow command verb
    }
    pub fn key(self, text: &str) -> String {
        self.wrap("1;95", text) // bright magenta keybind
    }
    pub fn value(self, text: &str) -> String {
        self.wrap("1;97", text) // bright white value
    }
    pub fn label(self, text: &str) -> String {
        self.wrap("1;37", text) // bold white label
    }
    pub fn hint(self, text: &str) -> String {
        self.wrap("2;36", text) // dim cyan hint
    }
    pub fn border(self, text: &str) -> String {
        self.wrap("36", text) // cyan border
    }
    pub fn accent(self, text: &str) -> String {
        self.wrap("1;35", text) // magenta accent
    }
    pub fn info(self, text: &str) -> String {
        self.wrap("34", text) // blue info
    }
    pub fn money(self, text: &str) -> String {
        self.wrap("1;32", text) // green money/STAMP
    }
    pub fn cargo(self, text: &str) -> String {
        self.wrap("1;33", text) // yellow cargo
    }
    pub fn route(self, text: &str) -> String {
        self.wrap("1;34", text) // blue route
    }
    pub fn door_open(self, text: &str) -> String {
        self.wrap("1;32", text)
    }
    pub fn door_wall(self, text: &str) -> String {
        self.wrap("1;31", text)
    }
    pub fn door_shield(self, text: &str) -> String {
        self.wrap("1;33", text)
    }
    pub fn align_cheap(self, text: &str) -> String {
        self.wrap("1;32", text)
    }
    pub fn align_fair(self, text: &str) -> String {
        self.wrap("1;33", text)
    }
    pub fn align_dear(self, text: &str) -> String {
        self.wrap("1;31", text)
    }
    #[allow(dead_code)]
    pub fn underline(self, text: &str) -> String {
        self.wrap("4;37", text)
    }
    #[allow(dead_code)]
    pub fn inverse(self, text: &str) -> String {
        self.wrap("7", text)
    }
}
