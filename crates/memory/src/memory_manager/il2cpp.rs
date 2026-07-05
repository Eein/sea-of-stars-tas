use crate::game_engine::il2cpp::{Image, Module};
use crate::process::{MemoryError, Process};
use asr::game_engine::unity::il2cpp::UnityPointer;
use bytemuck::CheckedBitPattern;
use std::cell::RefCell;
use std::collections::HashMap;

/// Max pointer-path depth. Our longest singleton field path is ~5 entries; +1
/// for the implicit `instance` prefix. 16 leaves comfortable headroom.
const PTR_CAP: usize = 16;

/// Per-manager cache of resolved [`UnityPointer`]s, keyed by field path.
///
/// This replaces the old bespoke singleton walker. Each read lazily builds (and
/// then reuses) a `UnityPointer` anchored on the manager's IL2CPP class name,
/// walking the static `instance` field plus the requested path. The pointer
/// caches its resolved offsets internally, so metadata resolution happens once.
///
/// It mirrors the old `class -> parent -> static_table -> instance ->
/// follow_fields` flow. `nr_of_parents = 1` because the static `instance` field
/// lives on the singleton base class (the parent of the concrete manager type).
#[derive(Default)]
pub struct UnityMemoryManager {
    /// IL2CPP class name of the manager (e.g. `"CombatManager"`), set each frame
    /// by the owning `MemoryManager` before updates run.
    pub class_name: &'static str,
    cache: RefCell<HashMap<Vec<&'static str>, UnityPointer<PTR_CAP>>>,
}

impl UnityMemoryManager {
    /// Drop resolved offsets so they re-resolve (e.g. after a read failure or a
    /// process change). `class_name` is preserved; it is re-set each frame.
    pub fn reset(&mut self) {
        self.cache.borrow_mut().clear();
    }

    /// Read `T` at the field `path`, relative to the manager singleton. Entries
    /// may be field names or raw `"0x.."` / decimal offsets (asr resolves each).
    /// The static `instance` field is prepended automatically.
    pub fn read<T: CheckedBitPattern>(
        &self,
        process: &Process,
        module: &Module,
        image: &Image,
        path: &[&'static str],
    ) -> Result<T, MemoryError> {
        let mut cache = self.cache.borrow_mut();
        if !cache.contains_key(path) {
            let mut full: Vec<&'static str> = Vec::with_capacity(path.len() + 1);
            full.push("instance");
            full.extend_from_slice(path);
            cache.insert(path.to_vec(), UnityPointer::new(self.class_name, 1, &full));
        }
        cache
            .get(path)
            .unwrap()
            .deref::<T>(&process.proc, &module.0, &image.0)
            .map_err(|_| MemoryError::ReadError)
    }

    /// Address of the manager singleton object (value of the static `instance`
    /// field). `Ok(0)` when the field is null; `Err` when the class hasn't
    /// resolved yet.
    pub fn singleton(
        &self,
        process: &Process,
        module: &Module,
        image: &Image,
    ) -> Result<u64, MemoryError> {
        self.read::<u64>(process, module, image, &[])
    }
}
