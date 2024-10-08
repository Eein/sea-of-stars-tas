use super::memory::MemoryManagers;
use crate::gui::Gui;

use crate::gui::helpers::*;
// Move these to preludes later
use memory::game_engine::il2cpp::{Image, Module};
use memory::process::Process;
use memory::process_list::ProcessList;

use egui_dock::DockState;
use log::info;
use std::time::Instant;

use joystick::prelude::*;
use seq::prelude::*;

pub struct StateDebug {
    pub pinned_fps: f64,
    pub last_pinned_update: Instant,
    pub last_update: Instant,
    pub last_memory_update: Instant,
}

pub struct StateGui {
    pub helpers: GuiHelpers,
    pub dock_state: DockState<String>,
}

// TODO(orkaboy): Anything the sequencer needs to access has to go in here
#[derive(Default)]
pub struct GameState {
    // TODO(orkaboy): Create multiple gamepads, one for each player
    pub gamepad: GenericJoystick,
    pub memory_managers: MemoryManagers,
}

pub struct State {
    pub context: StateContext,
    pub process_list: ProcessList,
    pub debug: StateDebug,
    pub gui: StateGui,
    pub game_state: GameState,
    pub sequencer: Sequencer<GameState>,
}
#[derive(Default)]
pub struct StateContext {
    pub process: Option<Process>,
    pub module: Option<Module>,
    pub image: Option<Image>,
}

impl State {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Register any GUI helpers here
        let gui_helpers = GuiHelpers::default();

        let tree_names = gui_helpers.tree_names();

        Self {
            context: StateContext {
                process: None,
                module: None,
                image: None,
            },
            process_list: ProcessList::default(),
            gui: StateGui {
                helpers: gui_helpers,
                dock_state: DockState::new(tree_names),
            },
            debug: StateDebug {
                pinned_fps: 0.0,
                last_pinned_update: Instant::now(),
                last_update: Instant::now(),
                last_memory_update: Instant::now(),
            },
            game_state: GameState {
                gamepad: GenericJoystick::default(),
                memory_managers: MemoryManagers::default(),
            },
            // TODO(orkaboy): Temp code, should not be here
            // TODO(orkaboy): Where do we put sequencer.run()? Might need to refactor that as well.
            sequencer: Sequencer::create(SeqLog::create("SEQ START")),
        }
    }

    pub fn maybe_deregister_process(&mut self) {
        if let Some(process) = &self.context.process {
            if !&self
                .process_list
                .is_open(sysinfo::Pid::from(process.pid as usize))
            {
                self.context.process = None
            }
        }
    }

    pub fn register_process(&mut self) {
        let process_name = "SeaOfStars.exe";
        // Find the Process
        match Process::with_name(process_name, &mut self.process_list) {
            Ok(process) => {
                if self.context.process.is_none() {
                    info!(
                        "- Attaching Process\nFound {} at pid {}",
                        process_name, process.pid
                    );
                }
                self.context.process = Some(process);
            }
            Err(_err) => {
                self.context = StateContext::default();
            }
        }
    }

    pub fn register_module(&mut self) {
        if self.context.module.is_none() {
            if let Some(ref mut process) = &mut self.context.process {
                info!("- Loading Module");
                // Attach to GameAssembly.dll
                self.context.module = Module::attach(process);
            }
        }
    }
    pub fn register_image(&mut self) {
        // If the module attached, set the default image (usually Assembly-Csharp)
        if self.context.image.is_none() {
            if let Some(process) = &self.context.process {
                if let Some(module) = &self.context.module {
                    info!("- Loading Image");
                    self.context.image = module.get_default_image(process);
                }
            }
        }
    }

    pub fn update_managers(&mut self) {
        let now = Instant::now();
        // Update managers at 100 fps >> 10ms
        if now
            .duration_since(self.debug.last_memory_update)
            .as_millis()
            >= 10
        {
            self.game_state.memory_managers.update(&self.context);
            self.debug.last_memory_update = now;
        }
    }
}

impl eframe::App for State {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Uncomment for puffin profiler
        // use puffin_egui::puffin;
        // puffin::set_scopes_on(true);
        // puffin_egui::profiler_window(ctx);
        // use
        // puffin::profile_function!();
        // if you want to profile a function

        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // Deregister the project in state if its not running anymore
        let _ = &self.maybe_deregister_process();

        // Register the process if its not in state
        let _ = &self.register_process();
        let _ = &self.register_module();
        let _ = &self.register_image();

        // Run self.update() on each manager
        let _ = &self.update_managers();

        // TODO(orkaboy): Should probably not be here
        if self.sequencer.is_running() {
            let _ = self.sequencer.run(&mut self.game_state);
        }

        Gui::update(self, ctx, frame)
    }
}
