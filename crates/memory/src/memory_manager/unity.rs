use crate::game_engine::il2cpp::*;
use crate::process::Process;

#[derive(Default)]
pub struct UnityMemoryManager {
    pub class: Option<Class>,
    pub parent: Option<Class>,
    pub instance: Option<u32>,
    pub singleton: Option<Class>,
    pub static_table: Option<u64>,
}
impl UnityMemoryManager {
    pub fn reset(&mut self) {
        self.class = None;
        self.parent = None;
        self.instance = None;
        self.singleton = None;
        self.static_table = None;
    }
}

impl UnityMemoryManagement for UnityMemoryManager {
    fn update(&mut self, process: &Process, module: &Module, image: &Image, name: &str) {
        if self.class.is_none() {
        self.class = image.get_class(process, module, name);
        if let Some(class) = self.class {
            self.parent = class.get_parent(process, module);
            if let Some(parent) = self.parent {
                self.static_table = parent.get_static_table(process, module);
                if let Some(static_table) = self.static_table {
                    self.instance = parent.get_field_offset(process, module, "instance");
                    if let Some(instance) = self.instance {
                        if let Ok(addr) =
                        process.read_pointer::<u64>(static_table + instance as u64)
                        {
                            self.singleton = Some(Class { class: addr });
                        }
                    }
                }
            }
            }
        }
    }
}

pub trait UnityMemoryManagement {
    fn update(&mut self, process: &Process, module: &Module, image: &Image, name: &str);
}
