use bytemuck::{CheckedBitPattern, Pod};
use memory::game_engine::il2cpp::{Class, Module};
use memory::memory_manager::unity::UnityMemoryManager;
use memory::process::{MemoryError, Process};

// TODO(eein): is it possible to make this more generic so it can be
// moved into memory crate?
use crate::state::StateContext;

pub struct MemoryContext<'a> {
    pub class: &'a Class,
    pub process: &'a Process,
    pub module: &'a Module,
    pub singleton: &'a Class,
}

impl<'a> MemoryContext<'a> {
    pub fn create(
        ctx: &'a StateContext,
        manager: &'a mut UnityMemoryManager,
    ) -> Result<MemoryContext<'a>, MemoryError> {
        if let Some(class) = &manager.class {
            if let Some(process) = &ctx.process {
                if let Some(module) = &ctx.module {
                    if let Some(singleton) = &manager.singleton {
                        return Ok(Self {
                            class,
                            process,
                            module,
                            singleton,
                        });
                    }
                }
            }
        }

        Err(MemoryError::Unset)
    }

    pub fn follow_fields<T: Pod>(&self, fields: &[&str]) -> Result<T, MemoryError> {
        self.class
            .follow_fields::<T>(*self.singleton, self.process, self.module, fields)
    }

    pub fn read_pointer_path_without_read(&self, path: &[u64]) -> Result<u64, MemoryError> {
        self.process
            .read_pointer_path_without_read(self.singleton.class, path)
    }

    pub fn read_pointer_path<T: CheckedBitPattern + Pod>(
        &self,
        path: &[u64],
    ) -> Result<T, MemoryError> {
        self.process
            .read_pointer_path::<T>(self.singleton.class, path)
    }

    pub fn read_pointer<T: Pod>(&self, addr: u64) -> Result<T, MemoryError> {
        self.process.read_pointer::<T>(addr)
    }

    pub fn get_field_offset(&self, field: &str) -> Option<u32> {
        self.class
            .get_field_offset(self.process, self.module, field)
    }
}
