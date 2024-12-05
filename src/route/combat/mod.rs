use crate::seq::movement::*;
use crate::state::{GameEvent, GameState};
use seq::prelude::*;

pub fn create() -> Box<dyn Node<GameState, GameEvent>> {
    SeqMove::create(
        "Combat Test",
        vec![Move::AwaitCombat(Box::new(Move::HoldDir(
            [0.0, 0.0],
            [0.0, 0.0, 0.0],
        )))],
    )
}
