use super::memory::MemoryManagers;
use crate::game_manager::GameManager;
use crate::gui::Gui;
// use puffin_egui::puffin;

use crate::gui::helpers::*;
// Move these to preludes later
use memory::game_engine::il2cpp::{Image, Module};
use memory::process::Process;
use memory::process_list::ProcessList;

use crate::config::Config;
use egui_dock::DockState;
use log::info;

use joystick::prelude::*;

use fps_clock::FpsClock;

pub struct StateDebug {
    pub fps: FpsClock,
}

pub struct StateGui {
    pub helpers: GuiHelpers,
    pub dock_state: DockState<String>,
}

// Anything the sequencer needs to access has to go in here
#[derive(Default)]
pub struct GameState {
    // TODO(orkaboy): Create multiple gamepads, one for each player
    pub gamepad: GenericJoystick,
    pub memory_managers: MemoryManagers,
    pub config: Config,
}

#[derive(Default)]
pub enum GameEvent {
    #[default]
    Cutscene,
    Combat,
}

pub struct State {
    pub context: StateContext,
    pub process_list: ProcessList,
    pub debug: StateDebug,
    pub gui: StateGui,
    pub game_state: GameState,
    pub game_manager: Option<GameManager>,
}
#[derive(Default)]
pub struct StateContext {
    pub process: Option<Process>,
    pub module: Option<Module>,
    pub image: Option<Image>,
}

impl State {
    pub fn new(cc: &eframe::CreationContext<'_>, conf: Config) -> Self {
        // Install Image Loaders
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // Register any GUI helpers here
        let gui_helpers = GuiHelpers::default();

        // Setup Dock State
        let tree_names = gui_helpers.tree_names();
        let mut dock_state = DockState::new(tree_names);

        // Restore Dock State from Store
        if conf.persist_tab_state {
            if let Some(storage) = cc.storage {
                if let Some(surface_store) = storage.get_string("dock_state") {
                    let main_surface = dock_state.main_surface_mut();
                    use egui_dock::Tree;
                    if let Ok(deserialized_surface) = ron::from_str::<Tree<String>>(&surface_store)
                    {
                        *main_surface = deserialized_surface;
                    }
                }
            }
        }

        Self {
            context: StateContext {
                process: None,
                module: None,
                image: None,
            },
            process_list: ProcessList::default(),
            gui: StateGui {
                helpers: gui_helpers,
                dock_state,
            },
            debug: StateDebug {
                fps: FpsClock::new(100),
            },
            game_state: GameState {
                gamepad: GenericJoystick::default(),
                memory_managers: MemoryManagers::default(),
                config: conf,
            },
            // TODO(orkaboy): Temp code, should not be here
            // TODO(orkaboy): Where do we put sequencer.run()? Might need to refactor that as well.
            game_manager: None,
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
        if self.context.process.is_none() {
            match Process::with_name(process_name, &mut self.process_list) {
                Ok(process) => {
                    // In case we change processes, default all the memory
                    // managers so addresses dont hang onto zombie processes.
                    // NOTE(eein): it may make more sense to do the entire
                    // game_state here
                    self.game_state.memory_managers = MemoryManagers::default();
                    info!(
                        "- Attaching Process\nFound {} at pid {}",
                        process_name, process.pid
                    );
                    self.context.process = Some(process);
                }
                Err(_err) => {
                    self.context = StateContext::default();
                }
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
        self.game_state.memory_managers.update(&self.context);
    }
}

impl eframe::App for State {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if self.game_state.config.persist_tab_state {
            eframe::set_value(storage, "dock_state", &self.gui.dock_state.main_surface());
        }
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Uncomment for puffin profiler
        // puffin::profile_function!();
        // puffin::set_scopes_on(true);
        // puffin_egui::profiler_window(ctx);

        // Deregister the project in state if its not running anymore
        let _ = &self.maybe_deregister_process();

        // Register the process if its not in state
        let _ = &self.register_process();
        let _ = &self.register_module();
        let _ = &self.register_image();

        // Run self.update() on each manager
        let _ = &self.update_managers();

        // TODO(orkaboy): Should probably not be here
        if let Some(gm) = self.game_manager.as_mut() {
            if gm.is_running() {
                let _ = gm.run(&mut self.game_state);
            }
        }
        // puffin::GlobalProfiler::lock().new_frame();
        Gui::update(self, ctx, frame);
        self.debug.fps.tick();
    }
}
