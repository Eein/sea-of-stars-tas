pub struct GUI;
use super::state::State;
use std::time::Instant;

impl GUI {
    pub fn update(state: &mut State, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match state.process.is_some() {
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Test");

            ui.horizontal(|ui| {
                ui.label("Testing the Application");
            });
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
