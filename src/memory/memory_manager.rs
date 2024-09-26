use crate::state::StateContext;

pub trait MemoryManager {
    fn update(&mut self, state: &StateContext);
    fn update_manager(&mut self, state: &StateContext);
    fn update_memory(&mut self, state: &StateContext);
}
