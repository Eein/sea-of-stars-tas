#[derive(Debug)]
pub struct MemoryManager {
    pub name: String,
    pub base: Option<u64>,
    pub fields_base: Option<u64>,
    pub singleton_base: Option<u64>,
    pub debug_enabled: bool,
}

impl MemoryManager {
    /// Toggles debug for this manager
    pub fn toggle_debug(&mut self) {
        self.debug_enabled = !self.debug_enabled
    }

    /// Helper debug methods only if debug is enabled
    pub fn dbg(&self, str: &str) {
        if self.debug_enabled {
            println!("{:?}", str);
        }
    }

    /// Logs out current address information about the manager
    pub fn log_manager_information(&self) {
        let base = self.base.unwrap_or(0);
        let fields_base = self.fields_base.unwrap_or(0);
        let singleton_base = self.singleton_base.unwrap_or(0);
        println!(
            "Memory Manager: {} :: Base 0x{:x} :: Fields Base 0x{:x} :: Singleton Base 0x{:x} :: Debug {:?}", 
            self.name,
            base,
            fields_base,
            singleton_base,
            self.debug_enabled
        );
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
