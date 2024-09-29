pub mod helpers;
pub struct Gui;
use super::gui::helpers::{GuiHelper, GuiHelpers};
use super::memory::MemoryManagers;
use super::state::State;
use egui_dock::{DockArea, Style};
use std::time::Instant;

pub struct TabViewer<'a> {
    helpers: &'a mut GuiHelpers,
    memory_managers: &'a MemoryManagers,
}
impl TabViewer<'_> {
    fn draw(&mut self, ui: &mut egui::Ui, tab: &mut String) {
        match tab.as_str() {
            "Nav Helper" => self.helpers.nav_helper.draw(self.memory_managers, ui, tab),
            "Main Helper" => self.helpers.main_helper.draw(self.memory_managers, ui, tab),
            "Title Helper" => self
                .helpers
                .title_helper
                .draw(self.memory_managers, ui, tab),
            _ => {
                let label = format!("Tab: {} has not been initialized. Check gui/mod.rs and state.rs to initialize this tab.", tab.as_str());
                ui.label(label);
            }
        }
    }
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        self.draw(ui, tab);
    }
}

impl Gui {
    pub fn run() {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([800.0, 800.0])
                .with_min_inner_size([800.0, 800.0]),

            ..Default::default()
        };
        eframe::run_native(
            "Sea of Stars TAS",
            options,
            Box::new(|cc| Ok(Box::new(State::new(cc)))),
        )
        .expect("Error loading application");
    }
    pub fn update(state: &mut State, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match state.context.process.is_some() {
            true => Self::active(state, ctx, frame),
            false => Self::inactive(state, ctx, frame),
        }
    }

    pub fn active(state: &mut State, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                // FPS Counter
                let tnow = Instant::now();

                let tprev = state.debug.last_update;
                let fps_string = {
                    let dt = (tnow - tprev).as_secs_f64();
                    let fps = 1.0 / dt;
                    format!("FPS: {}", fps.round())
                };

                state.debug.last_update = tnow;

                egui::widgets::global_dark_light_mode_buttons(ui);

                ui.label(fps_string);
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            DockArea::new(&mut state.gui.dock_state)
                .style(Style::from_egui(ctx.style().as_ref()))
                .show_close_buttons(false)
                .show(
                    ctx,
                    &mut TabViewer {
                        memory_managers: &state.memory_managers,
                        helpers: &mut state.gui.helpers,
                    },
                );
        });

        // TODO(eein)
        // Request a repaint on every update:
        // This is probably not performant, and can be handled in other ways.
        ctx.request_repaint();
    }

    pub fn inactive(_state: &mut State, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Game is not running...");
        });
    }
}

// fn main_tab(_state: &mut State, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         ui.heading("Test");

//         ui.horizontal(|ui| {
//             ui.label("Testing the Application");
//         });
//     });
// }
