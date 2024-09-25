pub struct MemoryManager {
    pub name: String,
    pub base: Option<u64>,
    pub fields_base: Option<u64>,
    pub singleton_base: Option<u64>,
    pub debug_enabled: bool,
}

impl MemoryManager {
    pub fn toggle_debug(&mut self) {
        self.debug_enabled = !self.debug_enabled
    }
    pub fn dbg(&self, str: &str) {
        if self.debug_enabled {
            println!("{:?}", str);
        }
    }
}

impl Default for MemoryManager {
    fn default() -> MemoryManager {
        MemoryManager {
            name: "MemoryManagerStub".to_string(),
            base: None,
            fields_base: None,
            singleton_base: None,
            debug_enabled: false
        }
    }
}

pub trait MemoryManagement {
    fn update(&self);
    fn toggle_debug(&self, manager: &mut MemoryManager) {
        manager.toggle_debug()
    }
}
