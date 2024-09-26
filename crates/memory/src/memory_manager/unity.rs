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
    fn update(&mut self, process: &Process, module: &Module, image: &Image, name: &str) {
        println!("Unity MEMORY UPDATE");
        if self.class.is_none() {
            println!("{}", &name);
            self.class = image.get_class(process, module, name);
            println!("CLASS {:?}", self.class);
            if let Some(class) = self.class {
              self.parent = class.get_parent(process, module);
                println!("PARENT {:?}", self.parent);
                if let Some(parent) = self.parent {
                    self.instance = parent.get_field_offset(process, module, "instance");
                    println!("INSTANCE {:?}", self.instance);
                    self.static_table =parent.get_static_table(process, module);
                    println!("STABLE: {:?}", self.static_table);
                }
            }
        }
    }
}

pub trait UnityMemoryManagement {
    fn update(&mut self, process: &Process, module: &Module, image: &Image, name: &str);
}
