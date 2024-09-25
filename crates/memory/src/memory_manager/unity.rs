use crate::game_engine::il2cpp::*;
use crate::process::Process;

#[derive(Default)]
pub struct UnityMemoryManager {
    pub class: Option<Class>,
    pub parent: Option<Class>,
    pub instance: Option<u32>,
    pub static_table: Option<u64>,
}

impl UnityMemoryManagement for UnityMemoryManager {
    fn update(&mut self, process: &Process, module: &Module, image: &Image, name: String) {
        if self.class.is_none() || self.parent.is_none() || self.static_table.is_none() {
            self.class = image.get_class(process, module, &name);
            if let Some(class) = self.class {
              self.parent = class.get_parent(process, module);
                if let Some(parent) = self.parent {
                    self.instance = parent.get_field_offset(process, module, "instance");
                    self.static_table =parent.get_static_table(process, module);
                    println!("{:?}", self.class);
                    println!("{:?}", self.parent);
                    println!("{:?}", self.instance);
                    println!("{:?}", self.static_table);
                }
            }
        }
    }
}
pub trait UnityMemoryManagement {
    fn update(&mut self, process: &Process, module: &Module, image: &Image, name: String);
}
