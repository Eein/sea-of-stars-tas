mod gui;
mod memory;
mod state;

use crate::gui::Gui;

use seq::{SeqCheckpoint, SeqList, SeqLog, Sequencer};

fn seq_test() {
    // Create a sequencer object
    let mut sequencer = Sequencer::create(SeqList::create(
        "Root",
        vec![
            SeqLog::create("Starting TAS"),
            SeqLog::create("Selecting character"),
            SeqLog::create("Mountain Trail 1"),
            SeqCheckpoint::create("Mooncradle"),
            SeqList::create(
                "Mooncradle",
                vec![
                    SeqLog::create("Garl nooo..."),
                    SeqLog::create("Training"),
                    SeqLog::create("Final Trials"),
                ],
            ),
            SeqList::create(
                "Forbidden Cave",
                vec![
                    SeqLog::create("Juking snails"),
                    SeqLog::create("Big boss"),
                    SeqLog::create("Leaving caves"),
                ],
            ),
            SeqLog::create("Make jam"),
        ],
    ));

    // Optionally, advance to a checkpoint in the sequence
    //sequencer.advance_to_checkpoint("Checkpoint");

    // Run the sequence (grabs control atm)
    sequencer.run();
}

fn main() {
    colog::init();
    seq_test();
    Gui::run();
}
