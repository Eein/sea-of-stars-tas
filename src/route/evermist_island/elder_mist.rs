//! Elder Mist trials + boss, ported from shenef/SoS-TAS `elder_mist.py`
//! (fights are engaged automatically by the combat FSM; dialog/option boxes
//! go through [`SeqSelectOption`], which owns the cutscene flag).

use super::change_time_tutorial::SeqChangeTimeTutorial;

use crate::seq::dialog::*;
use crate::seq::movement::*;
use crate::state::{GameEvent, GameState};
use seq::prelude::*;

/// The right section of the Elder Mist Trials. A few fights.
fn right_trial() -> Box<dyn Node<GameState, GameEvent>> {
    SeqMove::create(
        "Right trial",
        vec![
            Move::To(87.691, -5.998, 80.466),
            Move::To(92.032, -5.998, 80.164),
            Move::Interact(94.589, -8.998, 78.182),
            Move::Interact(97.268, -5.998, 75.787),
            Move::To(101.543, -3.998, 76.107),
            Move::Interact(108.939, -3.998, 76.190),
            Move::To(118.863, -5.998, 76.819),
            Move::Interact(118.863, -10.998, 74.542),
            Move::To(120.543, -10.998, 73.459),
            Move::Interact(123.539, -10.998, 73.459),
            Move::Interact(129.062, -10.998, 78.701),
            Move::Interact(127.996, -7.998, 80.362),
            Move::To(127.004, -7.998, 81.354),
            Move::Climb(126.832, -3.956, 81.168),
            Move::Climb(126.170, 8.002, 81.830),
            Move::To(127.200, 8.002, 88.541),
            Move::Interact(127.200, 8.002, 91.673),
            Move::To(127.200, 8.002, 95.574),
            Move::To(125.267, 8.002, 100.462),
            Move::To(121.495, 8.002, 104.859),
            Move::Climb(121.168, 11.253, 105.168),
            Move::Climb(121.830, 16.002, 105.830),
            // Fights trigger along here; combat takes over automatically.
            Move::Log("Navigate to lever"),
            Move::To(116.468, 16.002, 111.341),
            Move::To(109.964, 16.002, 111.458),
            Move::To(108.043, 16.002, 110.545),
            Move::Interact(102.052, 16.002, 110.545),
            Move::To(99.781, 16.002, 110.523),
            Move::Interact(99.781, 8.002, 107.825),
            Move::To(99.453, 8.002, 99.120),
            Move::Interact(99.453, 1.002, 91.542),
            Move::To(96.941, 1.002, 93.634),
            Move::Confirm, // Flip lever
            Move::To(102.188, 1.002, 83.490),
            Move::Interact(114.565, 1.002, 83.490),
            Move::To(117.486, 1.002, 87.864),
            Move::Climb(117.168, 3.847, 88.168),
            Move::Climb(117.830, 8.002, 88.830),
            Move::Log("Move to trigger"),
            Move::To(116.617, 8.002, 98.078),
            Move::To(114.460, 8.002, 100.543),
            Move::To(93.543, 8.002, 100.543),
            Move::To(85.471, 8.002, 92.376),
            Move::To(85.015, 8.002, 90.035),
            Move::Confirm, // Doodad
        ],
    )
}

/// The central section of the Elder Mist Trials: the quiz questions.
fn center_trial() -> Box<dyn Node<GameState, GameEvent>> {
    SeqList::create(
        "Center trial",
        vec![
            SeqMove::create(
                "Move to first question",
                vec![
                    Move::To(50.274, 1.002, 103.499),
                    Move::To(54.201, 1.002, 109.420),
                    Move::To(54.201, 1.002, 115.764),
                    Move::To(50.341, 1.002, 119.543),
                    Move::Climb(50.341, 5.811, 119.530),
                    Move::Climb(49.441, 11.222, 119.530),
                    Move::Climb(47.783, 11.222, 119.530),
                    Move::To(47.129, 10.002, 118.084),
                    Move::To(45.391, 10.002, 118.741),
                ],
            ),
            // Assumes top is the correct answer
            SeqSelectOption::create(vec![0], false),
            SeqMove::create(
                "Move to second question",
                vec![
                    Move::To(47.866, 10.002, 115.756),
                    Move::Interact(43.912, 10.002, 111.596),
                    Move::To(41.175, 10.002, 110.298),
                    Move::To(41.204, 10.002, 107.443),
                    Move::To(42.466, 10.002, 105.762),
                    Move::To(42.457, 10.002, 101.229),
                    Move::To(43.476, 10.002, 100.882),
                    Move::Climb(43.168, 13.664, 101.168),
                    Move::Climb(43.851, 17.002, 101.823),
                    Move::To(45.126, 17.002, 101.581),
                    Move::Interact(53.540, 17.002, 101.565),
                    Move::To(53.540, 17.002, 105.540),
                    Move::Interact(56.450, 14.002, 105.540),
                    Move::Interact(59.651, 7.002, 105.540),
                    Move::To(59.860, 7.002, 108.213),
                ],
            ),
            // Assumes top is the correct answer
            SeqSelectOption::create(vec![0], false),
            SeqMove::create(
                "Move to third question",
                vec![
                    Move::To(58.584, 7.002, 106.941),
                    Move::Climb(58.707, 10.284, 107.043),
                    Move::Climb(58.045, 14.002, 107.705),
                    Move::To(58.791, 14.002, 110.940),
                    Move::To(56.889, 14.002, 113.242),
                    Move::Confirm,
                    Move::To(56.549, 14.002, 117.804),
                    Move::Climb(56.168, 19.161, 118.168),
                    Move::Climb(56.830, 28.002, 118.830),
                    Move::To(59.421, 28.002, 115.745),
                    Move::To(59.461, 28.002, 109.460),
                    Move::Interact(40.986, 28.002, 109.500),
                    Move::To(40.171, 28.002, 110.857),
                    Move::To(40.540, 28.002, 113.208),
                    Move::Interact(41.917, 20.002, 113.208),
                    Move::To(43.311, 20.002, 116.665),
                ],
            ),
            // Assumes second is the correct answer
            SeqSelectOption::create(vec![1], false),
            SeqMove::create(
                "Move to pillar",
                vec![
                    Move::To(42.037, 20.002, 113.742),
                    Move::To(41.725, 20.002, 111.109),
                    Move::Confirm, // Doodad
                ],
            ),
        ],
    )
}

/// The left section of the Elder Mist Trials: moving platform and fights.
fn left_trial() -> Box<dyn Node<GameState, GameEvent>> {
    SeqMove::create(
        "Left trial",
        vec![
            Move::To(3.565, 1.002, 88.080),
            Move::To(-2.886, 6.010, 95.497),
            Move::To(-9.841, 6.002, 94.938),
            Move::To(-10.918, 6.002, 84.260),
            Move::Confirm, // Lever
            Move::Log("Drop ladder"),
            Move::To(-17.307, 6.002, 94.302),
            Move::Interact(-22.814, 6.002, 94.302),
            Move::To(-24.680, 6.002, 98.165),
            Move::Interact(-24.680, 8.087, 103.891),
            Move::To(-24.680, 14.010, 110.938),
            Move::To(-16.499, 14.002, 110.815),
            Move::To(-10.784, 14.002, 104.157),
            Move::To(-6.991, 14.002, 104.157),
            Move::Interact(-7.000, 6.002, 100.389),
            Move::To(-10.940, 6.002, 84.260),
            Move::Confirm, // Lever
            Move::To(-6.982, 6.002, 102.546),
            Move::Climb(-7.000, 10.414, 102.530),
            Move::Climb(-7.000, 14.002, 103.467),
            Move::Log("Move to doodad"),
            Move::To(-10.443, 14.002, 103.467),
            Move::Interact(-10.454, 14.002, 97.185),
            Move::To(-9.190, 14.002, 92.457),
            Move::Confirm,
            // Move::Interact(-7.124, 14.002, 90.925),
            Move::To(-4.649, 14.002, 93.002),
            Move::Confirm,
            Move::Interact(-0.431, 15.589, 98.986),
            Move::To(1.930, 18.888, 103.072),
            Move::To(8.359, 19.002, 107.734),
            Move::To(12.489, 19.002, 99.108),
            // Double back to catch the moving platform's timing.
            Move::To(8.359, 19.002, 107.734),
            Move::To(12.489, 19.002, 99.108),
            Move::Confirm, // Doodad
        ],
    )
}

pub fn create() -> Box<dyn Node<GameState, GameEvent>> {
    SeqList::create(
        "Elder Mist",
        vec![
            SeqMove::create(
                "Enter dream world",
                vec![
                    Move::To(48.570, 1.002, -6.019),
                    Move::Interact(48.533, 1.002, -1.519),
                    Move::To(50.541, 1.002, 0.470),
                    Move::Interact(50.541, 1.002, 3.481),
                    Move::To(51.546, 1.002, 4.541),
                    Move::Interact(54.481, 1.002, 4.541),
                    Move::Interact(54.481, 1.002, 12.923),
                    Move::To(54.441, 1.002, 15.912),
                    Move::To(49.410, 1.002, 26.971),
                    Move::Interact(49.432, 1.002, 30.748),
                ],
            ),
            SeqCheckpoint::create("Trials in the Mist"),
            SeqMove::create(
                "Move to right trial",
                vec![
                    Move::To(52.248, 1.002, 33.803),
                    Move::To(53.425, 1.002, 46.139),
                    Move::To(59.055, 4.002, 52.155),
                    Move::Confirm, // Teleporter
                ],
            ),
            right_trial(),
            SeqMove::create(
                "Move to center trial",
                vec![
                    Move::To(49.704, 4.010, 55.624),
                    Move::Confirm, // Teleporter
                ],
            ),
            center_trial(),
            SeqMove::create(
                "Move to left trial",
                vec![
                    Move::To(39.774, 4.002, 52.453),
                    Move::Confirm, // Teleporter
                ],
            ),
            left_trial(),
            SeqCheckpoint::create("Elder Mist Boss"),
            SeqMove::create(
                "Move to boss",
                vec![
                    Move::To(49.580, 1.002, 43.000),
                    Move::To(49.580, 1.002, 47.540),
                ],
            ),
            // Start the fight (dialog option), then wait it out: the inner
            // WaitFor never finishes on its own, so the step advances on the
            // combat-done event; intro/outro cutscenes are auto-mashed.
            SeqSelectOption::create(vec![0], false),
            SeqMove::create(
                "Elder Mist boss",
                vec![Move::AwaitCombat(Box::new(Move::WaitFor(9999.0)))],
            ),
            SeqMove::create("Move to save point", vec![Move::To(52.413, 1.002, 33.476)]),
            SeqCheckpoint::create("After Elder Mist Boss-fight"),
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
