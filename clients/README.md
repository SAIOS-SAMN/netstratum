# clients/ — the CLI/MUD client

Opened at landing N2. One command launches it (CHARTER §3.5):

    cargo run -p netstratum-cli

Interactive commands: `found <name>` · `zone <tile|row:col> <ext|hab>` ·
`tick <n>` · `collect` · `map` · `status` · `chronicle` · `verify` ·
`replay` · `help` · `quit`. The world is an 8×8 tile grid; the map renders
after every build. Colors mark closures (✓), refusals (✗ with the cure
named), and uncollected workspace (◔ — collect to keep). `--plain` or
`NO_COLOR` disables color.
Scripted: `cargo run -p netstratum-cli -- --script "found meridian; zone 0:0 ext; tick 4; collect; verify"`.
