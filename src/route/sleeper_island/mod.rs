use crate::seq::{loot::SeqLoot, movement::*};
use crate::state::GameState;
use seq::prelude::*;

pub fn create() -> Box<dyn Node<GameState>> {
    SeqList::create(
        "Sleeper Island",
        vec![
            SeqLog::create("X'Tol's Landing"),
            // TODO(orkaboy): Note, branch check Solstice Ring here
            SeqMove::create(
                "Move to chest",
                vec![
                    Move::To(-456.847, 1.002, -62.006),
                    Move::Interact(-459.978, -4.998, -65.169),
                    Move::To(-453.412, -4.998, -70.543),
                    Move::Interact(-453.412, -9.998, -72.458),
                    Move::To(-450.379, -9.998, -72.458),
                    Move::HoldDir([0.0, 1.0], [-491.448, 1.002, -219.789]),
                    Move::To(-491.448, 3.002, -207.100),
                ],
            ),
            // TODO(orkaboy): Loot Solstice Ring + equip
            SeqLoot::create(),
            SeqMove::create(
                "Move to chest",
                vec![
                    Move::To(-491.448, 1.010, -220.681),
                    Move::HoldDir([0.0, -1.0], [-450.196, -9.998, -72.732]),
                    Move::To(-446.604, -9.998, -72.457),
                    Move::Interact(-445.176, -14.998, -73.460),
                    Move::To(-444.577, -14.998, -73.867),
                ],
            ),
            // TODO(orkaboy): Loot 90 gold
            SeqLoot::create(),
            SeqMove::create(
                "Return to route",
                vec![
                    Move::Confirm,
                    Move::To(-447.621, -14.998, -76.229),
                    Move::Interact(-450.221, -14.998, -79.093),
                    Move::To(-449.642, -14.998, -81.495),
                    Move::Interact(-443.687, -14.998, -87.190),
                    Move::To(-443.687, -14.998, -90.633),
                    Move::To(-440.434, -14.998, -93.589),
                ],
            ),
            // Merging branch
            SeqMove::create(
                "Move to fight",
                vec![
                    Move::To(-426.310, -14.998, -98.160),
                    Move::Climb(-425.293, -20.566, -98.457),
                    Move::HoldDir([0.0, -1.0], [-425.293, -22.998, -100.725]),
                ],
            ),
        ],
    )
}
