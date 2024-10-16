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
            SeqLog::create("SEQ DONE"),
        ],
    ))
}

pub fn create_movement_test() -> GameManager {
    GameManager::new(SeqList::create(
        "Move Test",
        vec![evermist_island::create(), sleeper_island::create()],
    ))
}
