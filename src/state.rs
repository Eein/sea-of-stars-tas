use super::memory::MemoryManagers;
use crate::core::TasCore;
use crate::gui::Gui;
// use puffin_egui::puffin;

use crate::gui::helpers::*;
// Move these to preludes later
use memory::game_engine::il2cpp::{Image, Module};
use memory::process::Process;

use crate::config::Config;
use egui_dock::DockState;

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
    pub gamepads: [GenericJoystick; 3],
    pub memory_managers: MemoryManagers,
    pub config: Config,
    /// True when the TAS launched the game itself (via `game_start_command`).
    /// Used to add a settle delay on the title screen for the intro animation,
    /// which is only relevant on a cold boot we triggered.
    pub game_launched_by_tas: bool,
}

impl GameState {
    pub fn release_all(&mut self) {
        for gamepad in self.gamepads.iter_mut() {
            gamepad.release_all();
        }
    }
}

#[derive(Default)]
pub enum GameEvent {
    #[default]
    Cutscene,
    Combat,
    CoopSync(usize), // Param is player ID that originated signal
}

pub struct State {
    /// GUI-free runtime core (process, memory managers, gamepads, game manager).
    pub core: TasCore,
    pub debug: StateDebug,
    pub gui: StateGui,
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
        if conf.persist_tab_state
            && let Some(storage) = cc.storage
            && let Some(surface_store) = storage.get_string("dock_state")
        {
            let main_surface = dock_state.main_surface_mut();
            use egui_dock::Tree;
            if let Ok(deserialized_surface) = ron::from_str::<Tree<String>>(&surface_store) {
                *main_surface = deserialized_surface;
            }
        }

        Self {
            core: TasCore::new(conf),
            gui: StateGui {
                helpers: gui_helpers,
                dock_state,
            },
            debug: StateDebug {
                fps: FpsClock::new(100),
            },
        }
    }
}

impl eframe::App for State {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if self.core.game_state.config.persist_tab_state {
            eframe::set_value(storage, "dock_state", &self.gui.dock_state.main_surface());
        }
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    ///
    /// eframe 0.34 renamed the old `update(ctx, ..)` entry point to `ui(ui, ..)`;
    /// we build our own panels from the context, so we just pull it back out of
    /// the provided `Ui`.
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // Uncomment for puffin profiler
        // puffin::profile_function!();
        // puffin::set_scopes_on(true);
        // puffin_egui::profiler_window(ui.ctx());

        // Attach to the game (if needed) and refresh all memory managers.
        self.core.poll();

        // Advance the running game manager, if any.
        // TODO(orkaboy): Should probably not be here
        let _ = self.core.run_game_manager();

        // puffin::GlobalProfiler::lock().new_frame();
        Gui::update(self, ui, frame);
        self.debug.fps.tick();
    }
}
