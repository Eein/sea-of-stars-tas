use memory::game_engine::il2cpp::*;
use crate::memory::memory_manager::MemoryManager;
use memory::memory_manager::unity::*;
use memory::process::Process;

#[derive(Default)]
pub struct TitleSequenceManager {
    pub name: String,
    pub manager: UnityMemoryManager
}

impl TitleSequenceManager {
    pub fn new() -> Box<Self> {
        Box::new(
            Self { 
                name: "TitleSequenceManager".to_string(),
                manager: UnityMemoryManager {
                    ..UnityMemoryManager::default() 
                }
            }
        )
    }
}

impl MemoryManager for TitleSequenceManager {
    fn update(&mut self, process: &Process, module: &Module, image: &Image) {
        self.update_manager(process, module, image);
        self.update_memory();
    }

    fn update_manager(&mut self, process: &Process, module: &Module, image: &Image) {
        self.manager.update(process, module, image, &self.name)
    }

    fn update_memory(&mut self) {
        if self.manager.instance.is_some() {
            println!("UPDATE MEMORY");
            let instance = self.manager.instance.unwrap() as *const u32;
            println!("{:?}", instance);
            let instance_ptr = unsafe { std::ptr::read(instance) };
            println!("{:?}", instance_ptr);
        }

    }
}

#[repr(C)]
#[derive(Default)]
pub struct TitleSequenceManagerStruct {
    m_cached_ptr: u64,
    initialized: bool
}
