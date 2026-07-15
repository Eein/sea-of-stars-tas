//! Headless, script/AI-friendly runner for the Sea of Stars TAS.
//!
//! This module provides everything the `tas-cli` binary needs: argument
//! parsing ([`CliArgs`]), a deterministic-ish structured logger
//! ([`init_logger`]), a one-shot state snapshot ([`dump_state`]), and the
//! headless run loop ([`run`]). No window is ever created, output goes to
//! stdout/stderr as stable plaintext or JSON-lines, and the process exits with
//! a well-defined [status code](ExitCodes).

use std::process::ExitCode;
use std::time::{Duration, Instant};

use clap::{Parser, ValueEnum};
use fps_clock::FpsClock;
use log::{LevelFilter, Metadata, Record, error, info, warn};

use crate::config::{Config, load_config};
use crate::core::{GAME_PROCESS_NAME, TasCore};
use crate::route::tas;
use crate::util::vec3_ext::Vector3Ext;

/// Process exit codes. Kept small and stable so callers/agents can branch on them.
pub mod exit {
    /// Sequence finished, or one-shot command succeeded.
    pub const SUCCESS: u8 = 0;
    /// Bad usage / config error (clap handles most usage errors itself with 2).
    pub const USAGE: u8 = 1;
    /// Could not attach to the game process (not found / timed out / lost).
    pub const NO_GAME: u8 = 2;
    /// Aborted by the `--max-secs` watchdog before finishing.
    pub const TIMEOUT: u8 = 3;
}

/// Which pre-built sequence to run.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Route {
    /// The full TAS route (title screen -> relics -> islands).
    Tas,
    /// Load an existing save and hand off to the route.
    Load,
    /// Combat-only test sequence.
    Combat,
    /// Relic-selection-only test sequence.
    Relic,
}

/// Log output format.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum LogFormat {
    /// Stable, uncolored `LEVEL target: message` lines. Greppable by default.
    Plain,
    /// One JSON object per line (`{"level":..,"target":..,"msg":..}`).
    Json,
}

/// Headless runner arguments.
#[derive(Parser, Debug)]
#[command(
    name = "tas-cli",
    about = "Headless runner for the Sea of Stars TAS (no GUI).",
    long_about = "Drives the Sea of Stars TAS without a window. Produces stable, \
structured logs and deterministic exit codes so it can be scripted or driven by \
an AI agent for debugging."
)]
pub struct CliArgs {
    /// Which sequence to run.
    #[arg(long, value_enum, default_value_t = Route::Tas)]
    pub route: Route,

    /// Advance to this checkpoint before running (only meaningful for `--route tas`).
    #[arg(long)]
    pub checkpoint: Option<String>,

    /// Save slot to load before running. When unset, starts a new game (no load).
    #[arg(long)]
    pub save_slot: Option<usize>,

    /// Whether an auto-save is present (only for `--route load`).
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub auto_save_present: bool,

    /// Path to the config.toml file.
    #[arg(long, default_value = "./config.toml")]
    pub config: String,

    /// Override the Konami Code config flag.
    #[arg(long)]
    pub konami: Option<bool>,

    /// Override the Solstice Diploma config flag.
    #[arg(long)]
    pub solstice: Option<bool>,

    /// Shell command used to launch the game if it isn't already running.
    /// Overrides the `game_start_command` config setting.
    #[arg(long)]
    pub game_start_command: Option<String>,

    /// Wait for the game process to appear instead of exiting immediately.
    #[arg(long)]
    pub wait_for_game: bool,

    /// Give up waiting for the game after this many seconds (0 = wait forever).
    #[arg(long, default_value_t = 30)]
    pub attach_timeout: u64,

    /// Seconds to wait after attaching before starting the sequence.
    #[arg(long, default_value_t = 3.0)]
    pub start_delay: f64,

    /// Abort the run after this many seconds (0 = no limit). A watchdog for
    /// unattended/AI runs so a stuck sequence can't hang forever.
    #[arg(long, default_value_t = 0)]
    pub max_secs: u64,

    /// Target frames-per-second for the run loop.
    #[arg(long, default_value_t = 60)]
    pub fps: u32,

    /// Log output format.
    #[arg(long, value_enum, default_value_t = LogFormat::Plain)]
    pub log_format: LogFormat,

    /// Log level: error, warn, info, debug, or trace.
    #[arg(long, default_value = "info")]
    pub log_level: String,

    /// Attach, print a one-shot state snapshot, and exit (does not run the TAS).
    #[arg(long)]
    pub dump_state: bool,

    /// Print a final state snapshot when the run ends, whatever the reason
    /// (finished, lost game, or `--max-secs` timeout). Useful for AI debugging.
    #[arg(long)]
    pub dump_on_close: bool,
}

/// A minimal `log` backend that emits stable, uncolored lines in either plain
/// or JSON-lines format. No wall-clock timestamps, so identical runs produce
/// diff-stable output.
struct CliLogger {
    format: LogFormat,
}

impl log::Log for CliLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let level = record.level();
        let target = record.target();
        let msg = record.args().to_string();
        match self.format {
            LogFormat::Plain => {
                // e.g. "INFO  seq: SeqLog: SEQ START"
                eprintln!("{:<5} {}: {}", level, target, msg);
            }
            LogFormat::Json => {
                let obj = serde_json::json!({
                    "level": level.as_str(),
                    "target": target,
                    "msg": msg,
                });
                eprintln!("{}", obj);
            }
        }
    }

    fn flush(&self) {}
}

/// Install the CLI logger. Call once, before any logging happens.
pub fn init_logger(format: LogFormat, level: &str) {
    let filter = level.parse::<LevelFilter>().unwrap_or(LevelFilter::Info);
    // Ignore errors: a second install (e.g. in tests) is harmless.
    let _ = log::set_boxed_logger(Box::new(CliLogger { format }));
    log::set_max_level(filter);
}

/// Apply CLI config overrides on top of a loaded/default config.
fn resolve_config(args: &CliArgs) -> Config {
    let mut config = match load_config(&args.config) {
        Ok(config) => config,
        Err(_err) => {
            warn!("No config loaded from {}, using defaults.", args.config);
            Config::default()
        }
    };
    if let Some(konami) = args.konami {
        config.konami_code = konami;
    }
    if let Some(solstice) = args.solstice {
        config.solstice_diploma = solstice;
    }
    if let Some(command) = &args.game_start_command {
        config.game_start_command = Some(command.clone());
    }
    config
}

/// Spawn the game via a shell so `game_start_command` can be an arbitrary
/// command line. The child is left detached; we don't wait on it.
fn launch_game(command: &str) -> std::io::Result<()> {
    use std::process::Command;
    #[cfg(unix)]
    let mut cmd = {
        let mut c = Command::new("sh");
        c.arg("-c").arg(command);
        c
    };
    #[cfg(windows)]
    let mut cmd = {
        let mut c = Command::new("cmd");
        c.arg("/C").arg(command);
        c
    };
    cmd.spawn().map(|_child| ())
}

/// Launch the game via `game_start_command` if it is configured and the game
/// isn't already running. Returns `true` if we started it.
fn start_game_if_needed(core: &mut TasCore, command: Option<&str>) -> bool {
    let Some(command) = command else {
        return false;
    };
    if core.is_game_running() {
        info!("Game already running; not launching game_start_command.");
        return false;
    }
    info!("Game not running; launching game_start_command: {command}");
    match launch_game(command) {
        Ok(()) => true,
        Err(err) => {
            error!("Failed to launch game_start_command: {err}");
            false
        }
    }
}

/// Attach to the game, honoring `wait_for_game` / `--attach-timeout`.
/// Returns `true` once attached, `false` if we gave up.
fn attach(core: &mut TasCore, wait_for_game: bool, attach_timeout: u64) -> bool {
    let start = Instant::now();
    loop {
        core.poll();
        if core.is_attached() {
            return true;
        }
        if !wait_for_game {
            return false;
        }
        if attach_timeout > 0 && start.elapsed().as_secs() >= attach_timeout {
            return false;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// Join a list field as an indented block — one item per line, under the
/// field's `key:` header — so dumps read without horizontal scrolling.
fn indented_block(items: Vec<String>) -> String {
    if items.is_empty() {
        "(none)".to_string()
    } else {
        format!("\n  {}", items.join("\n  "))
    }
}

/// The state-snapshot fields derivable from `GameState` alone, shared by the
/// CLI's `--dump-state` and the GUI's "Dump State" button.
pub fn snapshot_fields(game_state: &crate::state::GameState) -> Vec<(&'static str, String)> {
    let mm = &game_state.memory_managers;
    let title = &mm.title_sequence_manager.data;
    let party = &mm.player_party_manager.data;
    let combat = &mm.combat_manager.data;
    let cutscene = &mm.cutscene_manager.data;
    let level_up = &mm.level_up_manager.data;
    let speedrun = &mm.speedrun_manager.data;

    let pos = &party.position;

    vec![
        ("title_active", title.active.to_string()),
        ("title_screen", title.current_screen_name.clone()),
        ("leader", format!("{:?}", party.leader_character)),
        ("movement_state", format!("{:?}", party.movement_state)),
        (
            "position",
            format!("{:.2},{:.2},{:.2}", pos.get_x(), pos.get_y(), pos.get_z()),
        ),
        ("in_cutscene", cutscene.is_in_cutscene.to_string()),
        ("encounter_active", combat.encounter_active.to_string()),
        ("enemy_count", combat.enemies.items.len().to_string()),
        ("player_count", combat.players.items.len().to_string()),
        (
            "player_aoe_radius",
            combat
                .player_aoe_radius
                .map(|r| format!("{r:.3}"))
                .unwrap_or_else(|| "none".to_string()),
        ),
        (
            "enemies",
            indented_block(
                combat
                .enemies
                .items
                .iter()
                .map(|e| {
                    let pos = e
                        .position
                        .map(|p| format!("{:.2},{:.2},{:.2}", p.get_x(), p.get_y(), p.get_z()))
                        .unwrap_or_else(|| "?".to_string());
                    let mods = e
                        .damage_type_modifiers
                        .items
                        .iter()
                        .map(|(k, v)| format!("{:?}={}", k.key, v.value))
                        .collect::<Vec<_>>()
                        .join(",");
                    format!(
                        "{} hp={} pdef={} mdef={} pos=({pos}) mods=[{mods}]",
                        e.unique_id, e.current_hp, e.physical_defense, e.magical_defense
                    )
                })
                .collect(),
            ),
        ),
        (
            "players",
            indented_block(
                combat
                .players
                .items
                .iter()
                .map(|p| {
                    format!(
                        "{:?} hp={} mp={} patk={} matk={}",
                        p.character,
                        p.current_hp,
                        p.current_mp,
                        p.physical_attack,
                        p.magical_attack
                    )
                })
                .collect(),
            ),
        ),
        (
            "moves",
            indented_block(
                combat
                .moves
                .iter()
                .flat_map(|cm| {
                    // Loaded moves (skills/attacks) plus combos, which never
                    // have a live component but do cost combo points.
                    cm.moves
                        .iter()
                        .filter(|m| m.loaded || m.combo_point_cost.unwrap_or(0) > 0)
                        .map(|m| {
                            format!(
                                "{:?}:{} power={:?} req={:?} loaded={} unlocked={} unlockable={:?} damaging={} cp={:?} sp={:?}",
                                cm.character,
                                m.move_id.as_deref().unwrap_or("?"),
                                m.special_move_power,
                                m.required_characters,
                                m.loaded,
                                m.unlocked,
                                m.unlockable,
                                m.is_damaging,
                                m.combo_point_cost,
                                m.skill_point_cost,
                            )
                        })
                })
                .collect(),
            ),
        ),
        (
            "appraisals",
            indented_block(
                crate::combat::appraisal::generate_appraisals(combat)
                    .iter()
                    .map(crate::combat::appraisal::Appraisal::describe)
                    .collect(),
            ),
        ),
        ("level_up_active", level_up.active.to_string()),
        ("is_speedrunning", speedrun.is_speedrunning.to_string()),
        ("speedrun_timer", format!("{}", speedrun.speedrun_timer)),
    ]
}

/// Print a one-shot, structured snapshot of the current game state.
fn dump_state(core: &TasCore, format: LogFormat) {
    let pid = core
        .context
        .process
        .as_ref()
        .map(|p| p.pid.to_string())
        .unwrap_or_else(|| "none".to_string());

    let mut fields: Vec<(&str, String)> =
        vec![("attached", core.is_attached().to_string()), ("pid", pid)];
    fields.extend(snapshot_fields(&core.game_state));

    match format {
        LogFormat::Plain => {
            println!("=== state snapshot ===");
            for (key, value) in &fields {
                println!("{key}: {value}");
            }
        }
        LogFormat::Json => {
            let map: serde_json::Map<String, serde_json::Value> = fields
                .into_iter()
                .map(|(k, v)| (k.to_string(), serde_json::Value::String(v)))
                .collect();
            println!("{}", serde_json::Value::Object(map));
        }
    }
}

/// Minimal shell-quoting: wrap in double quotes only if the value needs it.
fn shell_quote(s: &str) -> String {
    if s.is_empty() || s.contains(char::is_whitespace) {
        format!("\"{s}\"")
    } else {
        s.to_string()
    }
}

/// Render a `tas-cli` command line that reproduces a run, so a GUI or CLI
/// session can be replayed headlessly. Config-derived behavior (konami,
/// solstice) is emitted explicitly so the command doesn't depend on the local
/// `config.toml`. `--log-format json --start-delay 0` are canonical, agent
/// friendly reproduction defaults.
pub fn repro_command(
    route: Route,
    checkpoint: Option<&str>,
    save_slot: Option<usize>,
    auto_save_present: bool,
    config: &Config,
) -> String {
    let route_name = route
        .to_possible_value()
        .map(|v| v.get_name().to_string())
        .unwrap_or_else(|| "tas".to_string());

    let mut parts = vec!["tas-cli".to_string(), format!("--route {route_name}")];

    match route {
        Route::Tas => {
            if let Some(checkpoint) = checkpoint
                && checkpoint != "New Game"
            {
                parts.push(format!("--checkpoint {}", shell_quote(checkpoint)));
            }
            // A save slot means we load before running; include the save options.
            if let Some(slot) = save_slot {
                parts.push(format!("--save-slot {slot}"));
                parts.push(format!("--auto-save-present {auto_save_present}"));
            }
        }
        Route::Load => {
            if let Some(slot) = save_slot {
                parts.push(format!("--save-slot {slot}"));
            }
            parts.push(format!("--auto-save-present {auto_save_present}"));
        }
        Route::Combat | Route::Relic => {}
    }

    parts.push(format!("--konami {}", config.konami_code));
    parts.push(format!("--solstice {}", config.solstice_diploma));
    if let Some(command) = &config.game_start_command {
        parts.push(format!("--game-start-command {}", shell_quote(command)));
    }
    parts.push("--wait-for-game".to_string());
    parts.push("--log-format json".to_string());
    parts.push("--start-delay 0".to_string());

    parts.join(" ")
}

/// Build the requested game manager, applying route-specific options.
fn build_tas_runner(core: &mut TasCore, args: &CliArgs) {
    let gm = match args.route {
        Route::Tas => {
            let mut gm = tas::create_tas();
            if let Some(checkpoint) = &args.checkpoint
                && checkpoint != "New Game"
            {
                info!("Advancing to checkpoint: {checkpoint}");
                gm.advance_to_checkpoint(&mut core.game_state, checkpoint);
            }
            gm
        }
        Route::Load => {
            tas::create_load_sequence(args.save_slot.unwrap_or(1), args.auto_save_present)
        }
        Route::Combat => tas::create_combat_test(),
        Route::Relic => tas::create_relic_test(),
    };
    core.tas_runner = Some(gm);
}

/// Run the headless TAS. Returns a process exit code.
pub fn run(args: CliArgs) -> ExitCode {
    init_logger(args.log_format, &args.log_level);

    let config = resolve_config(&args);
    info!(
        "config: konami_code={} solstice_diploma={}",
        config.konami_code, config.solstice_diploma
    );

    let mut core = TasCore::new(config);

    // Launch the game first if configured and it isn't already running.
    let start_command = core.game_state.config.game_start_command.clone();
    let launched = start_game_if_needed(&mut core, start_command.as_deref());
    // Let the title sequence know it should wait out the intro animation.
    core.game_state.game_launched_by_tas = launched;

    info!("Waiting to attach to {GAME_PROCESS_NAME}...");
    // If we just launched the game, wait for it to appear even without
    // --wait-for-game, since it won't be ready instantly.
    let wait_for_game = args.wait_for_game || launched;
    if !attach(&mut core, wait_for_game, args.attach_timeout) {
        error!("Could not attach to {GAME_PROCESS_NAME} (game not running?).");
        return ExitCode::from(exit::NO_GAME);
    }
    info!("Attached to game process.");

    if args.dump_state {
        // Refresh a few frames so the managers have populated before we read.
        for _ in 0..5 {
            core.poll();
            std::thread::sleep(Duration::from_millis(16));
        }
        dump_state(&core, args.log_format);
        return ExitCode::from(exit::SUCCESS);
    }

    info!(
        "Reproduce this run with: {}",
        repro_command(
            args.route,
            args.checkpoint.as_deref(),
            args.save_slot,
            args.auto_save_present,
            &core.game_state.config,
        )
    );

    let code = run_sequence(&mut core, &args);

    // Dump a final snapshot on close, whatever the exit reason, so an agent can
    // inspect where the run ended up. Reads last-known manager data even if the
    // game process has since vanished.
    if args.dump_on_close {
        info!("Dumping final state on close.");
        dump_state(&core, args.log_format);
    }

    ExitCode::from(code)
}

/// Run the loaded route to completion (or until it errors/times out) and return
/// the corresponding [exit] code. Split out from [`run`] so every exit path
/// funnels through a single dump-on-close point.
fn run_sequence(core: &mut TasCore, args: &CliArgs) -> u8 {
    if args.start_delay > 0.0 {
        info!("Starting sequence in {:.1}s...", args.start_delay);
        let delay_start = Instant::now();
        while delay_start.elapsed().as_secs_f64() < args.start_delay {
            // Keep polling so we detect the game vanishing during the countdown.
            core.poll();
            if !core.is_attached() {
                error!("Lost the game process during start delay.");
                return exit::NO_GAME;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    let run_start = Instant::now();

    // A save slot means: load that save first, then run the route (the TAS route
    // additionally advances to a checkpoint if given). No save slot = new game.
    // `--route load` is itself the load, so it's excluded.
    if args.route != Route::Load
        && let Some(slot) = args.save_slot
    {
        info!("Loading save slot {slot}...");
        core.tas_runner = Some(tas::create_load_sequence(slot, args.auto_save_present));
        core.start_tas();
        if let Err(code) = run_active(core, args, run_start) {
            return code;
        }
    }

    // Build and run the requested route (TAS advances to the checkpoint if set).
    build_tas_runner(core, args);
    info!("Starting sequence (route={:?}).", args.route);
    core.start_tas();
    match run_active(core, args, run_start) {
        Ok(()) => exit::SUCCESS,
        Err(code) => code,
    }
}

/// Drive the currently-loaded game manager until it finishes. Returns `Ok(())`
/// on normal completion, or `Err(exit_code)` if the game vanished or the
/// `--max-secs` watchdog fired.
fn run_active(core: &mut TasCore, args: &CliArgs, run_start: Instant) -> Result<(), u8> {
    let mut fps = FpsClock::new(args.fps);
    loop {
        core.poll();
        core.run_tas();

        if !core.tas_running() {
            info!("Sequence finished.");
            return Ok(());
        }
        if !core.is_attached() {
            error!("Lost the game process during the run.");
            return Err(exit::NO_GAME);
        }
        if args.max_secs > 0 && run_start.elapsed().as_secs() >= args.max_secs {
            warn!("Aborting: exceeded --max-secs={}.", args.max_secs);
            core.game_state.release_all();
            return Err(exit::TIMEOUT);
        }

        fps.tick();
    }
}
