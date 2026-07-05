//! GUI-independent runtime core.
//!
//! [`TasCore`] owns everything the TAS needs at runtime that is *not* tied to a
//! window: the attached game process, the memory managers, the virtual
//! gamepads, and the optional running [`GameManager`]. Both the GUI ([`State`])
//! and the headless CLI drive the exact same logic through this struct so there
//! is a single source of truth for the attach/update/run loop.
//!
//! [`State`]: crate::state::State

use crate::config::Config;
use crate::game_manager::GameManager;
use crate::memory::MemoryManagers;
use crate::state::{GameState, StateContext};

use joystick::prelude::*;
use log::info;

use memory::game_engine::il2cpp::Module;
use memory::process::Process;
use memory::process_list::ProcessList;

/// Name of the game process to attach to.
pub const GAME_PROCESS_NAME: &str = "SeaOfStars.exe";

/// The GUI-free runtime core shared by the GUI and headless CLI front ends.
pub struct TasCore {
    pub context: StateContext,
    pub process_list: ProcessList,
    pub game_state: GameState,
    pub game_manager: Option<GameManager>,
}

impl TasCore {
    /// Build a fresh core with the given config and three (unattached) gamepads.
    pub fn new(config: Config) -> Self {
        Self {
            context: StateContext::default(),
            process_list: ProcessList::default(),
            game_state: GameState {
                gamepads: [
                    GenericJoystick::new(0),
                    GenericJoystick::new(1),
                    GenericJoystick::new(2),
                ],
                memory_managers: MemoryManagers::default(),
                config,
                game_launched_by_tas: false,
            },
            game_manager: None,
        }
    }

    /// Whether we currently have a live handle to the game process.
    pub fn is_attached(&self) -> bool {
        self.context.process.is_some()
    }

    /// Whether the game process is currently running, without attaching to it.
    /// Used to avoid launching a second instance via `game_start_command`.
    pub fn is_game_running(&mut self) -> bool {
        self.process_list.refresh();
        self.process_list
            .processes_by_name(GAME_PROCESS_NAME)
            .next()
            .is_some()
    }

    /// Drop the process handle if the game is no longer running so that memory
    /// addresses don't hang onto a zombie process.
    pub fn maybe_deregister_process(&mut self) {
        if let Some(process) = &self.context.process
            && !self
                .process_list
                .is_open(sysinfo::Pid::from(process.pid as usize))
        {
            self.context.process = None
        }
    }

    /// Find and attach to the game process if we aren't already attached.
    pub fn register_process(&mut self) {
        if self.context.process.is_none() {
            match Process::with_name(GAME_PROCESS_NAME, &mut self.process_list) {
                Ok(process) => {
                    // In case we change processes, default all the memory
                    // managers so addresses don't hang onto zombie processes.
                    self.game_state.memory_managers = MemoryManagers::default();
                    info!(
                        "- Attaching Process\nFound {} at pid {}",
                        GAME_PROCESS_NAME, process.pid
                    );
                    self.context.process = Some(process);
                }
                Err(_err) => {
                    self.context = StateContext::default();
                }
            }
        }
    }

    /// Attach to `GameAssembly.dll` once the process is known.
    pub fn register_module(&mut self) {
        if self.context.module.is_none()
            && let Some(process) = &mut self.context.process
        {
            info!("- Loading Module");
            self.context.module = Module::attach(process);
        }
    }

    /// Resolve the default IL2CPP image once the module is attached.
    pub fn register_image(&mut self) {
        if self.context.image.is_none()
            && let Some(process) = &self.context.process
            && let Some(module) = &self.context.module
        {
            info!("- Loading Image");
            self.context.image = module.get_default_image(process);
        }
    }

    /// Refresh every memory manager from the attached process.
    pub fn update_managers(&mut self) {
        self.game_state.memory_managers.update(&self.context);
    }

    /// One full attach/refresh cycle: (de)register the process/module/image and
    /// update all memory managers. Call this once per frame before running the
    /// game manager.
    pub fn poll(&mut self) {
        self.maybe_deregister_process();
        self.register_process();
        self.register_module();
        self.register_image();
        self.update_managers();
    }

    /// Start the currently-loaded game manager (enters the sequencer root).
    pub fn start_game_manager(&mut self) {
        if let Some(gm) = self.game_manager.as_mut() {
            gm.start(&mut self.game_state);
        }
    }

    /// Advance the running game manager by one frame, if any. Returns `true` on
    /// the frame the sequencer finishes.
    pub fn run_game_manager(&mut self) -> bool {
        if let Some(gm) = self.game_manager.as_mut()
            && gm.is_running()
        {
            return gm.run(&mut self.game_state);
        }
        false
    }

    /// Whether a game manager exists and its sequencer is still running.
    pub fn game_manager_running(&self) -> bool {
        self.game_manager.as_ref().is_some_and(|gm| gm.is_running())
    }
}
