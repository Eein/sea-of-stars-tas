# Headless CLI (`tas-cli`)

`tas-cli` runs the Sea of Stars TAS **without a GUI**. It shares 100% of the
runtime logic with the GUI (both drive the same `TasCore` in `src/core.rs`), but
produces stable, structured logs and deterministic exit codes so it can be
scripted or driven by an AI agent for debugging.

## Building & running

The toolchain (nightly Rust + native libs) comes from the Nix dev shell:

```sh
nix develop --command cargo build            # builds gui + tas-cli
nix develop --command cargo run --bin tas-cli -- --help
```

The virtual gamepad uses `uinput` (Linux), so the process needs write access to
`/dev/uinput` (typically run as a user in the `input` group, or via `sudo`).

## What it does

1. Attaches to the running `SeaOfStars.exe` process and loads its memory managers.
2. Optionally prints a one-shot state snapshot (`--dump-state`) and exits.
3. Otherwise builds the requested sequence, waits `--start-delay` seconds, runs
   the TAS loop at `--fps`, and exits when the sequence finishes.

## Exit codes

| Code | Meaning                                                        |
|------|---------------------------------------------------------------|
| 0    | Sequence finished, or `--dump-state` succeeded                |
| 1    | Usage/config error (reserved)                                 |
| 2    | Could not attach to the game (not found / timed out / lost)   |
| 3    | Aborted by the `--max-secs` watchdog                          |

(`clap` itself exits with code 2 on invalid arguments.)

## Logging

Every internal `info!/warn!/error!` flows through one logger with a stable,
uncolored format and **no wall-clock timestamps** (so repeated runs diff
cleanly). Choose the format with `--log-format`:

- `plain` (default): `LEVEL  target: message` — greppable by eye and by regex.
- `json`: one JSON object per line, e.g.
  `{"level":"INFO","target":"...","msg":"..."}` — for machine parsing.

Logs go to **stderr**; `--dump-state` output goes to **stdout**.

## Common invocations

```sh
# Full route, wait up to 60s for the game, abort if it hangs past 20 min:
tas-cli --wait-for-game --attach-timeout 60 --max-secs 1200

# Resume from a checkpoint:
tas-cli --route tas --checkpoint "Before Bosslug"

# Load an existing save (slot 3) then run:
tas-cli --route load --save-slot 3 --auto-save-present true

# Iterate on just the relic-selection logic (game sitting on Difficulty Select):
tas-cli --route relic --start-delay 0

# Combat-only test, JSON logs, no start countdown:
tas-cli --route combat --log-format json --start-delay 0

# Inspect live game state once and exit (JSON):
tas-cli --dump-state --log-format json

# Override config flags without editing config.toml:
tas-cli --konami false --solstice true
```

## Flags

Run `tas-cli --help` for the authoritative list. Highlights:

| Flag                       | Default          | Purpose                                             |
|----------------------------|------------------|-----------------------------------------------------|
| `--route <tas\|load\|combat\|relic>` | `tas`   | Which sequence to run                               |
| `--checkpoint <name>`      | –                | Advance to a checkpoint before running (`tas`)      |
| `--save-slot <N>`          | `1`              | Save slot for `--route load`                        |
| `--auto-save-present <b>`  | `true`           | Auto-save present, for `--route load`               |
| `--config <path>`          | `./config.toml`  | Config file to load                                 |
| `--konami <bool>`          | (from config)    | Override the Konami Code flag                        |
| `--solstice <bool>`        | (from config)    | Override the Solstice Diploma flag                   |
| `--wait-for-game`          | off              | Poll for the game instead of exiting immediately    |
| `--attach-timeout <secs>`  | `30`             | Give up waiting for the game (0 = forever)          |
| `--start-delay <secs>`     | `3`              | Countdown after attach before starting              |
| `--max-secs <secs>`        | `0`              | Watchdog: abort a stuck run (0 = no limit)          |
| `--fps <N>`                | `60`             | Run-loop tick rate                                  |
| `--log-format <plain\|json>` | `plain`        | Log output format                                   |
| `--log-level <lvl>`        | `info`           | error / warn / info / debug / trace                 |
| `--dump-state`             | off              | Print a state snapshot and exit                     |

## Notes on determinism

This tool drives a **live game process** in real time: it reads live memory,
depends on frame timing, and the game has its own RNG. Byte-for-byte replay
determinism is therefore *not* a goal here — what the CLI guarantees is no GUI
dependency, stable/greppable/parseable log output, scriptable flags, and clear
exit codes, which is what makes automated debugging tractable.
