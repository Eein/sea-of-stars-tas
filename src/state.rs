use super::gui::GUI;
// Move these to preludes later
use super::memory::title_sequence_manager::*;
use super::memory::memory_manager::MemoryManager;
use memory::game_engine::il2cpp::{Image, Module};
use memory::memory_manager::unity::*;
use memory::process::Process;
use memory::process_list::ProcessList;
use std::time::Instant;

pub struct StateDebug {
    pub last_update: Option<Instant>,
    pub last_memory_update: Option<Instant>,
}

pub struct State {
    pub process: Option<Process>,
    pub module: Option<Module>,
    pub image: Option<Image>,
    pub process_list: ProcessList,
    pub memory_managers: Vec<Box<dyn MemoryManager>>,
    pub debug: StateDebug,
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
            image: None,
            process_list: ProcessList::new(),
            memory_managers: Vec::new(),
            debug: StateDebug { last_update: None, last_memory_update: None },
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
            println!("- Attaching Process");
            let process_name = "SeaOfStars.exe";
            // Find the Process
            match Process::with_name(process_name, &mut self.process_list) {
                Ok(process) => {
                    println!("Found {} at pid {}", process_name, process.pid);
                    self.process = Some(process);
                    println!("{:?}", self.process);
                }
                Err(_err) => (),
            }
        }
    }

    pub fn register_module(&mut self) {
        if self.module.is_none() {
            if let Some(ref mut process) = &mut self.process {
                println!("- Loading Module");
                // Attach to GameAssembly.dll
                self.module = Module::attach(process);
                println!("{:?}", self.module);
            }
        }
    }
    pub fn register_image(&mut self) {
        // If the module attached, set the default image (usually Assembly-Csharp)
        if self.image.is_none() {
            if let Some(process) = &self.process {
                if let Some(module) = &self.module {
                    println!("- Loading Image");
                    self.image = module.get_default_image(process);
                    println!("{:?}", self.image);
                }
            }
        }
    }

    pub fn register_managers(&mut self) {
        if self.process.is_some() && self.image.is_some() && self.memory_managers.is_empty() {
            println!("- Pushing Managers");
            self.memory_managers.push(TitleSequenceManager::new());
        }
    }

    pub fn update_managers(&mut self) {
        if let Some(last_memory_update) = self.debug.last_memory_update {
            let now = Instant::now();
            // Update managers at 100 fps >> 10ms
            if now.duration_since(last_memory_update).as_millis() > 10 {
                self.debug.last_memory_update = Some(Instant::now());
                if let Some(process) = &self.process {
                    if let Some(module) = &self.module {
                        if let Some(image) = &self.image {
                            for m in self.memory_managers.iter_mut() {
                                m.update(process, module, image)
                            }
                        }
                    }
                }
            }
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
        let _ = &self.register_module();
        let _ = &self.register_image();
        let _ = &self.register_managers();

        // Run self.update() on each manager
        let _ = &self.update_managers();

        GUI::update(self, ctx, frame)
    }
}
