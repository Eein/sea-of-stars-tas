use memory::game_engine::il2cpp::*;
use memory::process::Process;

pub trait MemoryManager {
    fn update(&mut self, process: &Process, module: &Module, image: &Image);
    fn update_manager(&mut self, process: &Process, module: &Module, image: &Image);
    fn update_memory(&mut self);
}
