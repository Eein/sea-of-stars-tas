mod change_time_tutorial;
mod elder_mist;
mod forbidden_cave;
mod mooncradle;
mod mountain_trail;

use crate::seq::dialog::*;
use crate::state::{GameEvent, GameState};
use seq::prelude::*;

pub fn create() -> Box<dyn Node<GameState, GameEvent>> {
    SeqList::create(
        "Evermist Island",
        vec![
            mooncradle::create(),
            forbidden_cave::create(),
            mountain_trail::create(),
            elder_mist::create(),
        ],
    )
}
