use memory::game_engine::il2cpp::*;
use memory::memory_manager::unity::*;
use memory::process::Process;

#[derive(Default)]
pub struct MemoryManager {
    pub name: String,
    pub manager: UnityMemoryManager,
}

impl MemoryManager {
    pub fn update(&mut self, process: &Process, module: &Module, image: &Image) {
        self.manager.update(process, module, image, self.name.clone())
    }
}

// #[repr(C)]
// pub struct TitleSequenceManagerStruct {
//     m_CachedPtr: 0x10,
//     initialized: 0x18
// }
