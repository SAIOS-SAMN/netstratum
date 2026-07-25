# clients/ — the CLI/MUD client

Opened at landing N2. One command launches it (CHARTER §3.5):

    cargo run -p netstratum-cli

Interactive commands: `found <name>` · `zone <tile> <ext|hab>` · `tick <n>` ·
`collect` · `status` · `chronicle` · `verify` · `replay` · `quit`.
Scripted: `cargo run -p netstratum-cli -- --script "found meridian; zone 4 ext; tick 4; collect; verify"`.
