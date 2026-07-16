use seq::prelude::*;

use crate::seq::movement::{Move, SeqMove};
use crate::seq::relics::SeqRelicList;
use crate::seq::title::{SeqLoadGame, SeqTitleScreen};
use crate::tas_runner::TasRunner;

use super::combat;
use super::evermist_island;
use super::sleeper_island;

pub fn create_tas() -> TasRunner {
    TasRunner::new(SeqList::create(
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

/// Debug route that runs only the relic-selection sequence, assuming the game
/// is already sitting on the Difficulty Selection screen. Useful for iterating
/// on the relic screen logic without replaying the whole title sequence.
pub fn create_relic_test() -> TasRunner {
    TasRunner::new(SeqList::create(
        "RELIC TEST",
        vec![
            SeqLog::create("RELIC TEST START"),
            SeqRelicList::create(),
            SeqLog::create("RELIC TEST DONE"),
        ],
    ))
}

pub fn create_combat_test() -> TasRunner {
    TasRunner::new(SeqList::create(
        "TAS",
        vec![
            SeqLog::create("COMBAT START"),
            combat::create(),
            SeqLog::create("COMBAT DONE"),
        ],
    ))
}

/// Debug route for the campfire cooking interaction: assumes the game is
/// standing next to the Mountain Trail campfire with berries picked. Walks up,
/// picks Cook in the dialog, waits for the cooking screen, and holds Confirm
/// to cook.
pub fn create_cook_test() -> TasRunner {
    TasRunner::new(SeqList::create(
        "COOK TEST",
        vec![
            SeqLog::create("COOK TEST START"),
            SeqMove::create(
                "Campfire cook",
                vec![
                    Move::Interact(28.137, 21.002, 13.178),
                    Move::Confirm,
                    Move::WaitFor(2.5),
                    Move::Confirm,
                    Move::AwaitView("CookingScreen"),
                    Move::HoldConfirm(3.5), // TODO: How long is this
                ],
            ),
            SeqLog::create("COOK TEST DONE"),
        ],
    ))
}

pub fn create_load_sequence(save_slot: usize, auto_save_present: bool) -> TasRunner {
    TasRunner::new(SeqLoadGame::new(save_slot, auto_save_present))
}
