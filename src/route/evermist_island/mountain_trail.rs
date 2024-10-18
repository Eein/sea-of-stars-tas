use crate::seq::movement::*;
use crate::state::GameState;
use seq::prelude::*;

pub fn intro() -> Box<dyn Node<GameState>> {
    SeqList::create(
        "Evermist Island",
        vec![
            SeqLog::create("TAS START"),
            // TODO: Combat
            SeqMove::create(
                "Navigate tutorial fights",
                vec![
                    Move::Interact(31.524, 6.002, 19.951),
                    Move::To(36.021, 5.842, 19.951),
                    Move::To(49.921, 6.002, 6.540),
                    Move::To(54.534, 6.002, 6.543),
                    Move::Interact(55.458, 10.002, 9.467),
                    Move::To(57.051, 10.002, 12.404),
                    Move::Interact(43.963, 13.010, 26.059),
                    // TODO: This part is janky
                    Move::Interact(35.870, 13.010, 28.070), // after fight
                    Move::Climb(34.448, 6.002, 25.407),
                    Move::Log("Move to cavern"),
                    Move::To(31.505, 6.002, 19.783),
                    Move::Interact(23.612, 5.001, 11.401),
                    Move::To(19.979, 5.002, 7.421),
                    Move::To(14.948, 5.001, 7.870),
                    Move::To(-3.976, 5.001, 15.624),
                    Move::To(-8.323, 5.001, 17.491),
                    Move::To(-7.740, 5.002, 21.092),
                    Move::Interact(-5.086, 9.002, 23.746),
                    Move::To(-1.139, 9.002, 24.543),
                    Move::Interact(-1.223, 13.002, 26.974),
                    Move::To(-29.325, 13.112, 26.818),
                    Move::To(-33.072, 13.002, 23.247),
                    Move::To(-39.808, 11.002, 23.278),
                    Move::HoldDir([0.0, 1.0], [-88.500, 10.002, 32.058]),
                    Move::Log("Move through cavern"),
                    Move::To(-82.388, 10.002, 37.691),
                    Move::To(-82.254, 10.002, 42.898),
                    Move::To(-88.219, 10.002, 44.546),
                    Move::Log("Cliff section"),
                    Move::Climb(-88.219, 16.000, 45.000),
                    Move::To(-89.280, 16.000, 45.000),
                    Move::To(-92.238, 16.000, 47.673),
                    Move::To(-92.944, 16.000, 48.000),
                    Move::Climb(-93.016, 22.000, 48.000),
                    Move::To(-88.799, 22.000, 45.000),
                    Move::Log("Move to campfire"),
                    Move::To(-87.284, 22.002, 44.522),
                    Move::To(-73.903, 22.002, 34.029),
                    Move::HoldDir([0.0, -1.0], [-35.819, 21.002, 27.816]),
                    Move::To(-32.177, 21.002, 27.816),
                    Move::To(-30.369, 21.002, 32.300),
                    Move::Interact(-20.660, 21.002, 32.300),
                    Move::Interact(-17.865, 21.002, 35.382),
                    Move::Interact(-7.719, 21.002, 35.382),
                    Move::To(3.729, 21.002, 30.631),
                    Move::Interact(16.907, 21.002, 17.446),
                    Move::To(18.852, 21.002, 16.665),
                    // TODO: Hold dir
                ],
            ),
        ],
    )
}
