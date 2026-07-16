//! `UIManager` — the game's screen/view singleton. Read for `screensByType`
//! (a `Dictionary<Type, List<View>>` of every *live* view instance), which is
//! how the TAS can tell a specific UI screen — e.g. the campfire's
//! `CookingScreen` — is currently up.

use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;

#[derive(Default, Debug)]
pub struct UiManagerData {
    /// Class names of the views currently instantiated (alive) in
    /// `screensByType`, e.g. `"CookingScreen"`, `"PauseMenu"`.
    pub open_views: Vec<String>,
    /// Frame counter for throttling the dictionary walk.
    frame: u64,
}

impl UiManagerData {
    /// Whether a view of the given class name is currently up.
    pub fn view_open(&self, name: &str) -> bool {
        self.open_views.iter().any(|v| v == name)
    }

    /// Whether the pause *menu* is up. This — not `PauseManager.isPaused` —
    /// is the "a player paused the game" signal: other screens (the campfire
    /// `CookingScreen`, shops) also pause gameplay via pause requests, and
    /// the TAS must keep driving those.
    pub fn pause_menu_open(&self) -> bool {
        self.view_open("PauseMenu")
    }
}

impl Default for MemoryManager<UiManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "UIManager".to_string(),
            data: UiManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

impl MemoryManagerUpdate for UiManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;
        // The view dictionary walk (type-name resolution per entry) is
        // expensive and its consumers (AwaitView, pause detection) tolerate a
        // few frames of latency — throttle it.
        const REFRESH_FRAMES: u64 = 6;
        self.frame = self.frame.wrapping_add(1);
        if self.frame.is_multiple_of(REFRESH_FRAMES) || self.open_views.is_empty() {
            self.update_open_views(&memory_context)?;
        }
        Ok(())
    }
}

impl UiManagerData {
    /// Walk `screensByType` (`Dictionary<Type, List<View>>`): each live entry
    /// pairs a `System.Type` key with the list of instantiated views of that
    /// type. A type with at least one *alive* view (non-zero native handle —
    /// destroyed Unity objects zero it) counts as open.
    pub fn update_open_views(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        /// Dictionary entry stride: `{i32 hash, i32 next, ptr key, ptr value}`.
        const ENTRY_STRIDE: u64 = 0x18;
        const ENTRY_KEY: u64 = 0x8;
        const ENTRY_VALUE: u64 = 0x10;
        /// Array elements start (after klass/monitor/bounds/length header).
        const ARRAY_ELEMS: u64 = 0x20;
        const MAX_ENTRIES: i32 = 64;

        self.open_views.clear();
        let Ok(dict) = memory_context.follow_fields::<u64>(&["screensByType"]) else {
            return Ok(());
        };
        let (Some(entries), Some(count)) = (
            memory_context.read_named_ptr(dict, "_entries"),
            memory_context.read_named::<i32>(dict, "_count"),
        ) else {
            return Ok(());
        };
        for i in 0..count.clamp(0, MAX_ENTRIES) {
            let entry = entries + ARRAY_ELEMS + i as u64 * ENTRY_STRIDE;
            // Freed slots have their key reference nulled.
            let Ok(key) = memory_context
                .process
                .read_pointer::<u64>(entry + ENTRY_KEY)
            else {
                continue;
            };
            if key == 0 {
                continue;
            }
            let Ok(views) = memory_context
                .process
                .read_pointer::<u64>(entry + ENTRY_VALUE)
            else {
                continue;
            };
            // At least one view instance that is still alive (its native
            // handle at +0x10 is non-zero; destroyed objects zero it).
            let has_live_view = memory_context
                .list_item_ptrs(views)
                .into_iter()
                .any(|view| {
                    memory_context
                        .process
                        .read_pointer::<u64>(view + 0x10)
                        .is_ok_and(|native| native != 0)
                });
            if has_live_view && let Some(name) = memory_context.type_class_name::<64>(key) {
                self.open_views.push(name);
            }
        }
        Ok(())
    }
}
