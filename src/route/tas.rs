use seq::prelude::*;

use crate::game_manager::GameManager;
use crate::seq::movement::{Move, SeqMove};
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

// TODO: Temp
pub fn create_coop_test() -> GameManager {
    GameManager::new(SeqMove::create_coop(
        "Hills of intent",
        vec![
            vec![
                Move::WaitFor(3.0),
                Move::To(-0.940, 7.001, 10.111),
                Move::To(-0.583, 7.001, 8.625),
                Move::To(1.477, 7.001, 8.625),
                Move::To(1.477, 7.001, 10.842),
                Move::Log("Loop 1"),
                Move::To(-0.940, 7.001, 10.111),
                Move::To(-0.583, 7.001, 8.625),
                Move::To(1.477, 7.001, 8.625),
                Move::To(1.477, 7.001, 10.842),
                Move::Log("Loop 2"),
                Move::AwaitSync(vec![1, 2]),
                Move::To(-0.940, 7.001, 10.111),
                Move::To(-0.583, 7.001, 8.625),
                Move::To(1.477, 7.001, 8.625),
                Move::To(1.477, 7.001, 10.842),
                Move::Log("Loop 3"),
                Move::AwaitSync(vec![1, 2]),
            ],
            vec![
                Move::WaitFor(5.0),
                Move::Join,
                Move::AwaitSync(vec![0]),
                Move::To(5.976, 7.001, 10.202),
                Move::To(2.958, 7.001, 7.549),
                Move::To(5.934, 7.001, 7.549),
                Move::To(3.204, 7.001, 10.035),
                Move::Log("Loop 1"),
                Move::To(5.976, 7.001, 10.202),
                Move::To(2.958, 7.001, 7.549),
                Move::To(5.934, 7.001, 7.549),
                Move::To(3.204, 7.001, 10.035),
                Move::Log("Loop 2"),
                Move::To(5.976, 7.001, 10.202),
                Move::To(2.958, 7.001, 7.549),
                Move::To(5.934, 7.001, 7.549),
                Move::To(3.204, 7.001, 10.035),
                Move::Log("Loop 3"),
                Move::Leave,
                Move::AwaitSync(vec![0, 2]),
            ],
            vec![
                Move::WaitFor(7.0),
                Move::Join,
                Move::To(10.168, 7.001, 12.125),
                Move::To(10.168, 7.001, 8.290),
                Move::To(13.280, 7.001, 6.045),
                Move::To(8.361, 7.001, 6.045),
                Move::To(7.118, 7.001, 10.658),
                Move::Log("Loop 1"),
                Move::AwaitSync(vec![0]),
                Move::To(10.168, 7.001, 12.125),
                Move::To(10.168, 7.001, 8.290),
                Move::To(13.280, 7.001, 6.045),
                Move::To(8.361, 7.001, 6.045),
                Move::To(7.118, 7.001, 10.658),
                Move::Log("Loop 2"),
                Move::To(10.168, 7.001, 12.125),
                Move::To(10.168, 7.001, 8.290),
                Move::To(13.280, 7.001, 6.045),
                Move::To(8.361, 7.001, 6.045),
                Move::To(7.118, 7.001, 10.658),
                Move::Log("Loop 3"),
                Move::Leave,
                Move::AwaitSync(vec![0, 1]),
            ],
        ],
    ))
}
