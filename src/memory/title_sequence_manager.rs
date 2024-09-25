use memory::memory_manager::{MemoryManagement, MemoryManager};

#[derive(Debug)]
pub struct TitleSequenceManager {
    pub manager: MemoryManager,
}

impl Default for TitleSequenceManager {
    fn default() -> TitleSequenceManager {
        let module = TitleSequenceManager {
            manager: MemoryManager {
                name: "TitleSequenceManager".to_string(),
                // debug_enabled: true,
                ..MemoryManager::default()
            }
        };
        module.manager.log_manager_information();
        module
    }
}

impl MemoryManagement for TitleSequenceManager {
    fn update(&self) {
        self.manager.dbg("This should work?");
        self.manager.dbg(&self.manager.name);
    }
}
