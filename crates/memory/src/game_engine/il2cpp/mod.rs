//! IL2CPP access for Unity games, backed by livesplit `asr`.
//!
//! Our old hand-rolled metadata walker (`Class`/`follow_fields`/`Offsets` +
//! signature scanning) has been replaced by asr's `il2cpp` module. `Module` and
//! `Image` are thin newtypes over asr's types so the rest of the crate keeps the
//! same surface; actual class/field resolution now flows through
//! [`asr::game_engine::unity::il2cpp::UnityPointer`] (see
//! [`crate::memory_manager::il2cpp`]).
pub mod unity_list;
pub mod unity_serializable_dictionary;

use crate::process::Process;
use asr::game_engine::unity::il2cpp::{Image as AsrImage, Module as AsrModule, Version};

/// Represents access to a Unity game that is using the IL2CPP backend.
pub struct Module(pub AsrModule);

impl Module {
    /// Attach to the IL2CPP backend, forcing [`Version::V2020`] (Sea of Stars is
    /// Unity 2020.x; its offsets are byte-identical to asr's V2020 table). We
    /// avoid `attach_auto_detect` because `Version::detect` reads
    /// `UnityPlayer.dll`'s PE `FileVersion`, which is unreliable under
    /// Proton/Wine.
    ///
    /// KNOWN BLOCKER: asr PR #119 (Sep 2025) replaced the direct
    /// `type_info_definition_table` signature with a `global-metadata.dat`
    /// string -> LEA/SHR/RAX instruction walk that does NOT match this Sea of
    /// Stars build. `assemblies` still resolves; only `type_info` fails, so
    /// `attach` returns `None` here. Our old (pre-#119) signature
    /// `48 83 3C ?? 00 75 ?? 8B C? E8` resolves it correctly. The fix lives in
    /// the asr fork: add that signature as a fallback in asr's `attach` (see
    /// `~/code/TAS_PLAN.md`). Once the fork is pinned, this call works unchanged.
    pub fn attach(process: &Process) -> Option<Self> {
        AsrModule::attach(&process.proc, Version::V2020).map(Module)
    }

    /// Resolve the default `Assembly-CSharp` image.
    pub fn get_default_image(&self, process: &Process) -> Option<Image> {
        self.0.get_default_image(&process.proc).map(Image)
    }
}

/// A .NET image (DLL) loaded by the game — `Assembly-CSharp` holds the game logic.
#[derive(Copy, Clone)]
pub struct Image(pub AsrImage);
