use crate::seq::movement::*;
use crate::state::{GameEvent, GameState};
use seq::prelude::*;

pub fn flashback() -> Box<dyn Node<GameState, GameEvent>> {
    SeqList::create(
        "Mooncradle flashback",
        vec![
            SeqCheckpoint::create("intro_mooncradle"),
            SeqMove::create(
                "Mooncradle intro",
                vec![
                    Move::Log("Move to cliff"),
                    Move::To(-68.930, -10.998, 26.150),
                    Move::HoldDir([0.0, -1.0], [-13.969, -11.998, 38.757]),
                    Move::To(-13.969, -11.998, 36.392),
                    Move::To(-12.504, -11.998, 34.927),
                    Move::To(-9.406, -11.998, 34.927),
                    Move::Climb(-9.500, -6.998, 36.467),
                    Move::To(-13.482, -6.998, 40.543),
                    Move::Climb(-13.500, -0.998, 41.467),
                    Move::Log("Move to cutscene"),
                    Move::To(-9.222, -0.820, 53.985),
                    Move::To(-7.318, 1.010, 62.634),
                    Move::To(3.432, 1.002, 73.327),
                    Move::To(17.958, 1.002, 76.578),
                    Move::AwaitCutscene(Box::new(Move::HoldDir([1.0, 0.2], [0.0, 0.0, 0.0]))),
                    Move::Log("Move to Forbidden Cave"),
                    Move::To(31.373, 1.002, 89.671),
                    Move::To(31.471, 1.002, 114.862),
                    // Enter world map
                    Move::HoldDirWorld([0.0, 1.0], [109.500, 2.002, 61.998]),
                    Move::ToWorld(109.500, 2.002, 64.000),
                    Move::ToWorld(108.000, 2.002, 64.000),
                    Move::ToWorld(108.000, 2.002, 66.500),
                    Move::Log("Enter Forbidden Cave"),
                    Move::Confirm,
                    Move::To(14.000, 1.002, 17.396),
                    // Door
                    Move::Confirm,
                    Move::AwaitCutscene(Box::new(Move::HoldDir([0.0, 1.0], [0.0, 0.0, 0.0]))),
                    Move::Log("Move to dorms"),
                    Move::To(48.690, -8.990, -136.717),
                    Move::HoldDir([1.0, 1.0], [285.500, 5.002, 58.000]),
                    Move::To(290.419, 5.002, 61.872),
                    Move::To(295.647, 5.002, 63.663),
                    Move::HoldDir([1.0, 1.0], [72.657, -7.998, -133.640]),
                    Move::To(82.104, -7.998, -129.590),
                    Move::To(94.005, -11.998, -129.590),
                    Move::To(94.005, -11.998, -133.576),
                ],
            ),
            SeqCheckpoint::create("intro_dorms"),
        ],
    )
}
