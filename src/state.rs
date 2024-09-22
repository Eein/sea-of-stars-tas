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
}

impl eframe::App for State {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if let Some(process) = &self.process {
            if !self.process_list.is_open(sysinfo::Pid::from(process.pid as usize)) {
                self.process = None
            }
        };

        if let Some(process) = &self.process  {
            // If process is loaded then:
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                // The top panel is often a good place for a menu bar:

                egui::menu::bar(ui, |ui| {
                    // NOTE: no File->Quit on web pages!
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);

                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Test");

                ui.horizontal(|ui| {
                    ui.label("Testing the Application");
                });
            });
        } else {
        // If process needs to be loaded/reloaded:
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

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Game is not running...");
            });
        }


    }
}

