use crate::memory::objects::character::PlayerPartyCharacter;
use crate::seq::branch::CondMainChar;
use crate::seq::movement::*;
use crate::state::{GameEvent, GameState};
use seq::prelude::*;

use super::SeqSelectOption;

fn looms_to_center() -> Box<dyn Node<GameState, GameEvent>> {
    SeqIf::create(
        "Main Character",
        CondMainChar {
            leader: PlayerPartyCharacter::Valere,
        },
        Some(SeqMove::create(
            "Valere path",
            vec![
                Move::To(91.478, -18.998, -156.476),
                Move::To(88.309, -18.998, -156.476),
                Move::To(88.127, -14.998, -134.674),
            ],
        )),
        Some(SeqMove::create(
            "Zale path",
            vec![
                Move::To(97.745, -18.998, -156.127),
                Move::To(100.573, -18.998, -156.149),
                Move::To(101.496, -14.998, -142.680),
                Move::To(100.851, -14.998, -134.661),
            ],
        )),
        true,
    )
}

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
            SeqIf::create(
                "Main Character",
                CondMainChar {
                    leader: PlayerPartyCharacter::Valere,
                },
                Some(SeqMove::create(
                    "Valere path",
                    vec![
                        Move::To(94.005, -11.998, -133.576),
                        Move::To(86.621, -14.998, -135.924),
                        Move::To(84.091, -14.960, -140.766),
                        Move::To(82.085, -18.998, -147.421),
                        Move::To(78.360, -18.998, -147.465),
                    ],
                )),
                Some(SeqMove::create(
                    "Zale path",
                    vec![
                        Move::To(94.005, -11.998, -133.576),
                        Move::To(100.395, -14.998, -134.377),
                        Move::To(104.751, -14.998, -139.367),
                        Move::To(106.596, -18.998, -147.793),
                        Move::To(109.640, -18.998, -147.793),
                    ],
                )),
                true,
            ),
            SeqSelectOption::create(vec![0], false),
            SeqMove::create(
                "Training",
                vec![
                    Move::Log("Move to Erlina"),
                    Move::To(-17.700, -13.998, -136.900),
                    Move::Interact(-11.000, -14.998, -143.700),
                    Move::HoldDir([1.0, -1.0], [248.762, 5.002, 56.959]),
                    Move::To(253.865, 5.002, 56.959),
                    Move::To(257.891, 5.002, 58.706),
                    Move::To(260.734, 5.002, 58.706),
                    Move::HoldDir([1.0, -1.0], [16.980, -8.998, -135.817]),
                    Move::To(32.789, -8.930, -151.592),
                    Move::To(32.789, -8.990, -176.218),
                    Move::AwaitCutscene(Box::new(Move::HoldDir([0.0, -1.0], [0.0, 0.0, 0.0]))),
                    Move::Log("Train with Erlina"),
                ],
            ),
            looms_to_center(),
            SeqMove::create(
                "Eavesdrop",
                vec![
                    Move::To(94.160, -11.998, -133.319),
                    Move::To(94.382, -11.998, -129.766),
                    Move::To(81.351, -7.998, -129.428),
                    Move::To(72.903, -7.998, -133.028),
                    Move::HoldDir([-1.0, -1.0], [295.354, 5.002, 64.422]),
                    Move::To(292.408, 5.002, 62.065),
                    Move::To(286.460, 5.002, 59.299),
                    Move::AwaitCutscene(Box::new(Move::HoldDir([-1.0, -1.0], [0.0, 0.0, 0.0]))),
                ],
            ),
            SeqIf::create(
                "Main Character",
                CondMainChar {
                    leader: PlayerPartyCharacter::Valere,
                },
                Some(SeqMove::create(
                    "Valere path",
                    vec![
                        Move::To(82.936, -18.998, -147.362),
                        Move::To(84.538, -14.990, -138.768),
                        Move::To(89.372, -14.998, -133.939),
                    ],
                )),
                Some(SeqMove::create(
                    "Zale path",
                    vec![
                        Move::To(106.395, -18.998, -147.455),
                        Move::To(104.039, -14.998, -137.504),
                        Move::To(99.935, -14.998, -133.890),
                    ],
                )),
                true,
            ),
            SeqMove::create(
                "Eavesdrop",
                vec![
                    Move::To(94.014, -11.998, -133.488),
                    Move::To(93.947, -11.998, -129.872),
                    Move::To(80.214, -7.998, -129.872),
                    Move::To(73.745, -7.998, -132.473),
                    Move::HoldDir([-1.0, -1.0], [295.354, 5.002, 64.422]),
                    Move::To(292.408, 5.002, 62.065),
                    Move::To(286.460, 5.002, 59.299),
                    Move::HoldDir([-1.0, -1.0], [49.334, -8.998, -135.486]),
                    Move::To(33.021, -8.932, -151.461),
                    Move::To(33.021, -8.990, -176.434),
                    Move::HoldDir([0.0, -1.0], [273.463, 5.002, 48.071]),
                    Move::Log("Cookies"),
                    Move::AwaitCutscene(Box::new(Move::HoldDir([0.0, -1.0], [0.0, 0.0, 0.0]))),
                ],
            ),
            looms_to_center(),
            SeqCheckpoint::create("intro_dorms2"),
            SeqMove::create(
                "Move to Moraine",
                vec![
                    Move::To(94.014, -11.998, -133.488),
                    Move::To(93.947, -11.998, -129.872),
                    Move::To(80.214, -7.998, -129.872),
                    Move::To(73.745, -7.998, -132.473),
                    Move::HoldDir([-1.0, -1.0], [295.354, 5.002, 64.422]),
                    Move::To(292.408, 5.002, 62.065),
                    Move::To(286.460, 5.002, 59.299),
                    Move::HoldDir([-1.0, -1.0], [49.334, -8.998, -135.486]),
                    Move::To(43.613, -8.998, -141.418),
                    Move::To(35.035, -8.998, -141.418),
                    Move::To(32.954, -8.998, -136.402),
                    Move::Confirm,                      // Talk to Moraine
                    Move::To(31.747, -8.998, -141.005), // Move to Erlina
                ],
            ),
            // Skip Erlina Tutorial
            SeqSelectOption::create(vec![1, 1], false),
            SeqMove::create("Move to Brugaves", vec![Move::To(33.529, -8.932, -141.817)]),
            // Skip Brugaves Tutorial
            SeqSelectOption::create(vec![1, 1], false),
            SeqMove::create("Move to Moraine", vec![Move::To(33.071, -8.998, -136.126)]),
            // Talk with Moraine
            SeqSelectOption::create(vec![0], false),
            // TODO(orkaboy): MoveToward
        ],
    )
}
