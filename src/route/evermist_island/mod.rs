mod change_time_tutorial;
mod diploma;
mod elder_mist;
mod mooncradle;
mod mountain_trail;

use self::diploma::SeqDiploma;

use crate::seq::branch::CondRelic;
use crate::seq::dialog::*;
use crate::state::{GameEvent, GameState};
use seq::prelude::*;

pub fn create() -> Box<dyn Node<GameState, GameEvent>> {
    SeqList::create(
        "Evermist Island",
        vec![
            SeqLog::create("MOVE START"),
            mountain_trail::intro(),
            SeqIf::create(
                "Solstice Diploma",
                CondRelic {
                    name: "Solstice Diploma",
                },
                // TODO(orkaboy): Currently doesn't work since control is grabbed by cutscene
                // TODO: Need to enter this node before the cutscene triggers
                Some(SeqDiploma::create()),
                Some(mooncradle::flashback()),
                false,
            ),
            SeqCheckpoint::create("forbidden_cave"),
            SeqLog::create("TODO BREAK"),
            elder_mist::create(),
        ],
    )
}
