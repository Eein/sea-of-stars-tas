use super::gui::GUI;
// Move these to preludes later
use super::memory::title_sequence_manager::TitleSequenceManager;
use memory::game_engine::il2cpp::Module;
use memory::process::Process;
use memory::process_list::ProcessList;
use memory::memory_manager::{MemoryManager, MemoryManagement};
use std::time::Instant;

pub struct StateDebug {
    pub last_update: Option<Instant>
}

pub struct State {
    pub process: Option<Process>,
    pub module: Option<Module>,
    pub process_list: ProcessList,
    pub memory_managers: Vec<Box<dyn MemoryManagement>>,
    pub debug: StateDebug
}

impl State {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self {
            process: None,
            module: None,
            process_list: ProcessList::new(),
            memory_managers: vec![],
            debug: StateDebug { last_update: None }
        }
    }

    pub fn maybe_deregister_process(&mut self) {
        if let Some(process) = &self.process {
            if !&self
                .process_list
                .is_open(sysinfo::Pid::from(process.pid as usize))
            {
                self.process = None
            }
        }
    }

    pub fn register_process(&mut self) {
        if self.process.is_none() {
            let process_name = "SeaOfStars.exe";
            match Process::with_name(process_name, &mut self.process_list) {
                Ok(mut process) => {
                    println!("Found {} at pid {}", process_name, process.pid);
                    self.module = Module::attach(&mut process);
                    self.process = Some(process);
                    self.memory_managers.push(Box::new(TitleSequenceManager::default()));
                }
                Err(_err) => (),
            }
        }
    }

    pub fn update_managers(&self) {
        for m in self.memory_managers.iter() {
            m.update()
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
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // Deregister the project in state if its not running anymore
        let _ = &self.maybe_deregister_process();
        // Register the process if its not in state
        let _ = &self.register_process();

        let _ = &self.update_managers();

        GUI::update(self, ctx, frame)
    }
}
