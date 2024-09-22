mod state;
use eframe::egui;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 300.0])
            .with_min_inner_size([300.0, 300.0]),

        ..Default::default()
    };
    eframe::run_native(
        "Sea of Stars TAS",
        options,
        Box::new(|cc| Ok(Box::new(state::State::new(cc)))),
    )
    .expect("Error loading application");
}
