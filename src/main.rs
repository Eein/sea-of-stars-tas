mod gui;
mod state;

use eframe::egui;

use seq::{Sequencer, SeqLog, SeqList, SeqCheckpoint};

fn seq_test() {
    // Create a sequencer object
    let mut sequencer = Sequencer::create(
        SeqList::create("Root", vec![
            SeqLog::create("Starting TAS"),
            SeqLog::create("Selecting character"),
            SeqLog::create("Mountain Trail 1"),
            SeqCheckpoint::create("Mooncradle"),
            SeqList::create("Mooncradle", vec![
                SeqLog::create("Garl nooo..."),
                SeqLog::create("Training"),
                SeqLog::create("Final Trials"),
            ]),
            SeqList::create("Forbidden Cave", vec![
                SeqLog::create("Juking snails"),
                SeqLog::create("Big boss"),
                SeqLog::create("Leaving caves"),
            ]),
            SeqLog::create("Make jam"),
        ])
    );

    // Optionally, advance to a checkpoint in the sequence
    //sequencer.advance_to_checkpoint("Checkpoint");

    // Run the sequence (grabs control atm)
    sequencer.run();
}


fn main() {

    seq_test();

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
