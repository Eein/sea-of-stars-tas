use std::fmt::Display;

pub mod prelude {
    pub use crate::flow::*;
    pub use crate::logging::*;
    pub use crate::sequencer::Sequencer;
    pub use crate::wait::*;
    pub use crate::Node;
}

pub mod flow;
pub mod logging;
pub mod sequencer;
pub mod wait;

pub trait Node<State, Event>: Display {
    fn execute(&mut self, _state: &mut State, _delta: f64) -> bool {
        true
    }
    fn advance_to_checkpoint(&mut self, _state: &mut State, _checkpoint: &str) -> bool {
        false
    }
    fn on_event(&mut self, _state: &mut State, _event: &Event) {
        // Override
    }
    fn enter(&mut self, _state: &mut State) {
        // Override
    }
    fn exit(&self, _state: &mut State) {
        // Override
    }
    fn cutscene_control(&self) -> bool {
        false
    }
}
