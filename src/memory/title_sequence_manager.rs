use memory::memory_manager::{MemoryManagement, MemoryManager};

pub struct TitleSequenceManager {
    pub manager: MemoryManager,
}

impl Default for TitleSequenceManager {
    fn default() -> TitleSequenceManager {
        TitleSequenceManager {
            manager: MemoryManager {
                name: "TitleSequenceManager".to_string(),
                ..MemoryManager::default()
            }
        }
    }
}

impl MemoryManagement for TitleSequenceManager {
    fn update(&self) {
        self.manager.dbg("This should work?");
        self.manager.dbg(&self.manager.name);
    }
}
