use seq::prelude::*;

use crate::game_manager::GameManager;
use crate::seq::relics::SeqRelicList;
use crate::seq::title::{SeqLoadGame, SeqTitleScreen};

use super::combat;
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

pub fn create_combat_test() -> GameManager {
    GameManager::new(SeqList::create(
        "TAS",
        vec![
            SeqLog::create("COMBAT START"),
            combat::create(),
            SeqLog::create("COMBAT DONE"),
        ],
    ))
}

pub fn create_load_sequence(save_slot: usize, auto_save_present: bool) -> GameManager {
    GameManager::new(SeqLoadGame::new(save_slot, auto_save_present))
}
