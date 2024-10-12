pub mod helpers;
pub struct Gui;
use super::gui::helpers::GuiHelpers;
use super::state::{GameState, State};
use crate::config::Config;
use crate::util::time_formatter::TimeFormatter;
use egui_dock::{DockArea, Style};
use seq::prelude::*;
use std::time::{Duration, Instant};

pub struct TabViewer<'a> {
    helpers: &'a mut GuiHelpers,
    game_state: &'a mut GameState,
    sequencer: &'a mut Sequencer<GameState>,
}
impl TabViewer<'_> {
    fn draw(&mut self, ui: &mut egui::Ui, tab: &mut String) {
        self.helpers.draw(self.game_state, self.sequencer, ui, tab);
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
    pub fn run(conf: Config) {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([800.0, 800.0])
                .with_min_inner_size([800.0, 800.0]),

            ..Default::default()
        };
        eframe::run_native(
            "Sea of Stars TAS",
            options,
            Box::new(|cc| Ok(Box::new(State::new(cc, conf)))),
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
        let speedrun_manager = &state.game_state.memory_managers.speedrun_manager.data;

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
                let tlastpinned = state.debug.last_pinned_update;

                let fps_string = {
                    let since = tnow.duration_since(tlastpinned);
                    if since >= Duration::from_millis(100) {
                        let dt = (tnow - tprev).as_secs_f64();
                        let fps = 1.0 / dt;
                        let out = fps.round();
                        state.debug.last_pinned_update = tnow;
                        state.debug.pinned_fps = out;
                    }
                    format!("FPS: {: >3}", state.debug.pinned_fps)
                };

                state.debug.last_update = tnow;

                egui::widgets::global_theme_preference_buttons(ui);

                // Speedrun Timers
                if speedrun_manager.is_speedrunning {
                    ui.label(format!(
                        "Run: {}",
                        TimeFormatter::format(speedrun_manager.speedrun_timer.timer_in_second)
                    ));
                    ui.label(format!(
                        "Pause: {}",
                        TimeFormatter::format(speedrun_manager.pause_timer.timer_in_second)
                    ));
                }

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
                        game_state: &mut state.game_state,
                        sequencer: &mut state.sequencer,
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
