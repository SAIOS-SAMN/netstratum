# Tools

These utilities operate outside the simulation engine. The Python scripts use
only the standard library and read the exported files under `var/` by default.

## `observer.py` — read-only terminal dashboard

Renders the exported universe as a terminal dashboard. It opens files for
reading only; it does not change game state. It can also show focused views
such as civilizations, worlds, tiers, stress, technologies, markets, routes,
refineries, ruins, and an individual world or civilization.

```sh
python3 tools/observer.py --root var
python3 tools/observer.py --root var --watch 2
python3 tools/observer.py --root var worlds
python3 tools/observer.py --root var world <world-name>
python3 tools/observer.py --root var civs --tier T0
python3 tools/observer.py --help
```

`--root` points to the data root containing `worlds/` and `registry/`. The
default is `var`. The script prints a notice if no exported worlds exist yet.
`--watch` refreshes the view every given number of seconds; `--plain` disables
color output.

## `steward.py` — queue directives

Lists worlds and queues a directive file in a world's `inbox/`. It never edits
world state directly; the game loop decides whether a queued directive can be
applied under the steward's scope. Its root defaults to `var/worlds`.

```sh
python3 tools/steward.py --root var/worlds list
python3 tools/steward.py --root var/worlds status <world-name>
python3 tools/steward.py --root var/worlds send <world-name> <verb> [args...]
python3 tools/steward.py --help
```

`send` writes a new binary `.dir` file to the target world's inbox, so use it
only when you intend to queue that action. `status` shows pending, applied, and
refused directives.

## `webserve.py` and `observatory.html` — browser observatory

`observatory.html` is the read-only browser dashboard. `webserve.py` serves it
and the supported exported data files, and streams changed file paths to the
page using server-sent events. The server supports GET requests only; it does
not expose a JSON API.

```sh
python3 tools/webserve.py --root var --port 8081
```

Open <http://localhost:8081/> in a browser. The server defaults to `var` and
port `8081`; it binds to `0.0.0.0`, so only run it on a network where that
read-only data access is appropriate. Stop it with Ctrl-C.

## systemd examples

The `.service` files are deployment examples, not installed services. Both
assume a checkout at `/opt/netstratum`; edit `WorkingDirectory` and executable
paths for the actual host before installing them.

- `netstratum-conductor.service` runs the G2 conductor heartbeat against
  `var-g2`, leaving the `var/` archive untouched. It expects the release
  `conductor` binary under `target/release/`.
- `netstratum-follow.service` describes the Mycelium hypha follower. It is
  intentionally disabled: the old standalone `:7520` court is retired, and
  this service must stay stopped until the coordinator names a court. The
  `ns-hypha` command has no default court.

Do not enable the follower just because its example unit is present. Confirm
the current coordinator and court configuration first.
