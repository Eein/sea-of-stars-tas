use seq::prelude::*;

use crate::seq::relics::SeqRelicList;
use crate::seq::title::SeqTitleScreen;
use crate::seq::dialog::{SeqSelectOption, SeqSkipUntilIdle};
use crate::state::GameState;

use crate::seq::movement::*;

pub fn create_sequencer() -> Sequencer<GameState> {
    Sequencer::create(SeqList::create(
        "TAS",
        vec![
            SeqLog::create("SEQ START"),
            SeqTitleScreen::create(),
            SeqRelicList::create(),
            SeqLog::create("SEQ DONE"),
        ],
    ))
}

pub fn create_movement_test() -> Sequencer<GameState> {
    Sequencer::create(SeqList::create(
        "Move Test",
        vec![
            SeqLog::create("MOVE START"),
            SeqMove::create("Get Y'eeted", vec![
                // Note, after Elder Mist time tutorial
                Move::Log("Cross the bridges"),
                Move::ChangeTime(21.0),
                Move::To(46.352, 97.002, 170.300),
                Move::ChangeTime(9.0),
                Move::To(20.483, 97.000, 170.430),
                Move::To(7.655, 97.000, 172.225),
                Move::HoldDirWorld([-1.0, 0.0], [107.500, 6.002, 85.498]),
                Move::Log("Leaving Elder Mist"),
                Move::ToWorld(104.000, 6.002, 85.500),
                Move::Log("Enter Y'eet"),
                Move::Confirm,
                Move::To(-421.597, 27.002, 175.500),
                Move::To(-428.018, 27.002, 180.034),
            ]),
            SeqSelectOption::create(0, false),
            SeqSkipUntilIdle::create(),
            // Move this stuff elsewhere (route folders)
            SeqLog::create("X'Tol's Landing"),
            SeqMove::create("Move to chest",
                vec![
                    Move::To(-456.847, 1.002, -62.006),
                    Move::Interact(-459.978, -4.998, -65.169),
                    Move::To(-453.412, -4.998, -70.543),
                    Move::Interact(-453.412, -9.998, -72.458),
                    Move::To(-450.379, -9.998, -72.458),
                    Move::HoldDir([0.0, 1.0], [-491.448, 1.002, -219.789]),
                    Move::To(-491.448, 3.002, -207.100),
                    // Loot box
                ],
            ),
        ],
    ))
}
