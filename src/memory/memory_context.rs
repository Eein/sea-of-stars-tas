use bytemuck::CheckedBitPattern;
use memory::game_engine::il2cpp::{Image, Module};
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::{MemoryError, Process};

use crate::state::StateContext;

/// Per-frame handle bundling the attached process/module/image with a manager's
/// [`UnityPointer`](asr::game_engine::unity::il2cpp::UnityPointer) cache. Reads
/// resolve (and cache) their pointer paths through the manager.
pub struct MemoryContext<'a> {
    pub process: &'a Process,
    module: &'a Module,
    image: &'a Image,
    manager: &'a UnityMemoryManager,
}

impl<'a> MemoryContext<'a> {
    pub fn create(
        ctx: &'a StateContext,
        manager: &'a UnityMemoryManager,
    ) -> Result<MemoryContext<'a>, MemoryError> {
        if let (Some(process), Some(module), Some(image)) =
            (&ctx.process, &ctx.module, &ctx.image)
        {
            Ok(Self {
                process,
                module,
                image,
                manager,
            })
        } else {
            Err(MemoryError::Unset)
        }
    }

    /// Read `T` at a singleton-relative field `path`. Each entry is either a
    /// field name or a raw `"0x.."` / decimal offset; the static `instance`
    /// field is prepended automatically.
    pub fn read<T: CheckedBitPattern>(&self, path: &[&'static str]) -> Result<T, MemoryError> {
        self.manager.read::<T>(self.process, self.module, self.image, path)
    }

    /// Address of the manager singleton object.
    pub fn singleton(&self) -> Result<u64, MemoryError> {
        self.manager.singleton(self.process, self.module, self.image)
    }

    /// Raw pointer path anchored at the singleton object. Kept for the handful
    /// of call sites that walk runtime offset values (not compile-time field
    /// paths).
    pub fn read_pointer_path<T: CheckedBitPattern>(
        &self,
        path: &[u64],
    ) -> Result<T, MemoryError> {
        let base = self.singleton()?;
        self.process.read_pointer_path::<T>(base, path)
    }

    pub fn read_pointer<T: CheckedBitPattern>(&self, addr: u64) -> Result<T, MemoryError> {
        self.process.read_pointer::<T>(addr)
    }
}
