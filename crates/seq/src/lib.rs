pub mod flow;
pub mod logging;
pub mod sequencer;

pub trait Node {
    fn execute(&mut self, _delta: f64) -> bool {
        true
    }
    fn advance_to_checkpoint(&mut self, _checkpoint: &str) -> bool {
        false
    }
    fn enter(&mut self) {
        // Override
    }
    fn exit(&self) {
        // Override
    }
}
