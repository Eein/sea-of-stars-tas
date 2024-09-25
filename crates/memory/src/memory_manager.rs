use super::process::Process;
use super::game_engine::il2cpp::*;

#[derive(Debug)]
pub struct UnityMemoryManager {
    pub name: String,   
    pub base: Option<u64>,
    pub parent: Option<u64>,
    pub parent_instance: Option<u64>,
    pub static_table: Option<u64>,
    pub debug_enabled: bool,
}

impl UnityMemoryManager {
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
        let parent = self.parent.unwrap_or(0);
        let parent_instance = self.parent_instance.unwrap_or(0);
        let static_table = self.static_table.unwrap_or(0);
        println!(
            "Memory Manager: {} | Base: 0x{:x} | P: 0x{:x} | PI: 0x{:x} | STable: 0x{:x} | Debug {:?}", 
            self.name,
            base,
            parent,
            parent_instance,
            static_table,
            self.debug_enabled
        );
    }
}

// This needs to be unity specific
impl Default for UnityMemoryManager {
    fn default() -> UnityMemoryManager {
        UnityMemoryManager {
            name: "UnityMemoryManagerStub".to_string(),
            base: None,
            parent: None,
            parent_instance: None,
            static_table: None,
            debug_enabled: false
        }
    }
}

pub trait MemoryManagement {
    fn update(&mut self, process: &Process, image: &Image);
}
