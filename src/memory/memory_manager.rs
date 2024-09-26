use crate::state::StateContext;

pub trait MemoryManager {
    fn update(&mut self, ctx: &StateContext) {
        self.update_manager(ctx);
        self.update_memory(ctx);
    }

    fn update_manager(&mut self, state: &StateContext);
    fn update_memory(&mut self, state: &StateContext);
}
