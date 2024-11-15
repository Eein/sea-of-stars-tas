use seq::prelude::*;

use crate::game_manager::GameManager;
use crate::seq::relics::SeqRelicList;
use crate::seq::title::SeqTitleScreen;

use super::evermist_island;
use super::sleeper_island;

pub fn create_tas() -> GameManager {
    GameManager::new(SeqList::create(
        "TAS",
        vec![
            SeqLog::create("SEQ START"),
            SeqTitleScreen::create(),
            SeqRelicList::create(),
            evermist_island::create(),
            sleeper_island::create(),
            SeqLog::create("SEQ DONE"),
        ],
    ))
}
