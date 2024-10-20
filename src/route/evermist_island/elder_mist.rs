use super::change_time_tutorial::SeqChangeTimeTutorial;

use crate::seq::dialog::*;
use crate::seq::movement::*;
use crate::state::{GameEvent, GameState};
use seq::prelude::*;

pub fn create() -> Box<dyn Node<GameState, GameEvent>> {
    SeqList::create(
        "Elder Mist",
        vec![
            // TODO: more stuff
            SeqCheckpoint::create("elder_mist_boss2"),
            SeqMove::create(
                "Leave dream world",
                vec![
                    Move::To(49.555, 1.002, 31.458),
                    Move::Interact(49.555, 1.002, 25.086),
                    Move::To(54.487, 1.002, 12.457),
                    Move::Interact(54.487, 1.002, 4.794),
                    Move::Interact(51.519, 1.002, 4.493),
                    Move::To(50.460, 1.002, 3.457),
                    Move::Interact(50.460, 1.002, 0.519),
                    Move::To(48.460, 1.002, -1.543),
                    Move::Interact(48.460, 1.002, -4.481),
                    Move::To(49.572, 1.002, -10.738),
                ],
            ),
            SeqSelectOption::create(vec![0], false),
            SeqChangeTimeTutorial::create(),
            SeqMove::create(
                "Change time",
                vec![
                    // Note, after Elder Mist time tutorial
                    Move::ChangeTime(21.0),
                    Move::WaitFor(3.0),
                    Move::ChangeTime(15.0),
                    Move::Log("Cross the bridges"),
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
                ],
            ),
            SeqSelectOption::create(vec![0], false),
        ],
    )
}
