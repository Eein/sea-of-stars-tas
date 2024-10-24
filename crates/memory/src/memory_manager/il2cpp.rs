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
        *self = Self::default();
    }
}

impl UnityMemoryManagement for UnityMemoryManager {
    fn update(&mut self, process: &Process, module: &Module, image: &Image, name: &str) {
        // if our base classes are none, do things to them.
        if self.class.is_none()
            || self.parent.is_none()
            || self.static_table.is_none()
            || self.instance.is_none()
        {
            self.class = image.get_class(process, module, name);
            if let Some(class) = self.class {
                self.parent = class.get_parent(process, module);
                if let Some(parent) = self.parent {
                    self.static_table = parent.get_static_table(process, module);
                    self.instance = parent.get_field_offset(process, module, "instance");
                }
            }
        }
        // update our singleton
        if let Some(instance) = self.instance {
            if let Some(static_table) = self.static_table {
                if let Ok(addr) = process.read_pointer::<u64>(static_table + instance as u64) {
                    self.singleton = Some(Class { class: addr });
                } else {
                    self.singleton = Some(Class { class: 0 });
                }
            }
        }
    }
}

pub trait UnityMemoryManagement {
    fn update(&mut self, process: &Process, module: &Module, image: &Image, name: &str);
}
