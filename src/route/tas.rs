use seq::prelude::*;

use crate::seq::relics::SeqRelicList;
use crate::seq::title::SeqTitleScreen;
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
            SeqMove::create(vec![
                // Note, after Elder Mist time tutorial
                Move::ChangeTime(21.0),
                Move::To(46.352, 97.002, 170.300),
                Move::ChangeTime(9.0),
                Move::To(20.483, 97.000, 170.430),
                Move::To(7.655, 97.000, 172.225),
            ]),
            SeqLog::create("MOVE DONE"),
        ],
    ))
}
