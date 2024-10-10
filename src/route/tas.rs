use seq::prelude::*;

use crate::seq::relics::SeqRelicList;
use crate::seq::title::SeqTitleScreen;
use crate::state::GameState;

pub fn create_sequencer() -> Sequencer<GameState> {
    Sequencer::create(SeqList::create(
        "TEMP",
        vec![
            SeqLog::create("SEQ START"),
            SeqTitleScreen::create(),
            SeqRelicList::create(),
            SeqLog::create("SEQ DONE"),
        ],
    ))
}
