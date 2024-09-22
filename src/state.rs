use super::gui::GUI;
use memory::process::Process;
use memory::process_list::ProcessList;

pub struct State {
    pub process: Option<Process>,
    pub module: u64,
    pub process_list: ProcessList,
}

impl State {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self {
            process: None,
            module: 0,
            process_list: ProcessList::new(),
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
                    if let Ok(address) = process.module_address("GameAssembly.dll") {
                        self.process = Some(process);
                        self.module = address;
                    };
                }
                Err(_err) => (),
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

        GUI::update(self, ctx, frame)
    }
}
