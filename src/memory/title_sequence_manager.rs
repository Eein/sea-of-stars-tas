use memory::memory_manager::{UnityMemoryManager, MemoryManagement};
use memory::game_engine::il2cpp::*;
use memory::process::Process;

pub struct TitleSequenceManager {
    pub manager: UnityMemoryManager,
    pub class: Option<Class>,
    pub parent: Option<Class>,
    pub static_table: Option<u64>
}

impl Default for TitleSequenceManager {
    fn default() -> TitleSequenceManager {
        let module = TitleSequenceManager {
            manager: UnityMemoryManager {
                name: "TitleSequenceManager".to_string(),
                // debug_enabled: true,
                ..UnityMemoryManager::default()
            },
            class: None,
            parent: None,
            static_table: None,


        };
        module.manager.log_manager_information();
        module
    }
}

impl MemoryManagement for TitleSequenceManager {
    fn update(&mut self, _process: &Process, _image: &Image) {
        if self.class.is_none() {
            // reset parents, since the class must exist to update
            self.parent = None;
            self.static_table = None;

            // find the class 
            // or fail
            if self.class.is_none() { return; }
            
            if self.class.is_none() {
                // find the class or fail
            }
        }

    }
}

// #[repr(C)]
// pub struct TitleSequenceManagerStruct {
//     m_CachedPtr: 0x10,
//     initialized: 0x18
// }
