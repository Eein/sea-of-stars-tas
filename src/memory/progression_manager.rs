//! `ProgressionManager` — the save-file progression singleton. Currently read
//! for `unlockedCombatMoves`, the set the game's `PlayerCombatMoveDefinition.
//! IsUnlocked` (RVA 0xA31D30) consults for `unlockable != 0` moves. This is
//! what separates a learned skill/combo from one merely present in the
//! fighter's loaded move lists.

use std::collections::HashSet;

use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;
use memory::string::ArrayWString;

#[derive(Default, Debug)]
pub struct ProgressionManagerData {
    /// `combatMoveId`s of every move the party has learned
    /// (`unlockedCombatMoves`, a `HashSet<PlayerCombatMoveDefinitionReference>`
    /// — the reference struct wraps the id string, so each set slot holds the
    /// string pointer directly).
    pub unlocked_combat_moves: HashSet<String>,
    /// Frame counter for throttling the set walk (unlocks change rarely).
    frame: u64,
}

impl Default for MemoryManager<ProgressionManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "ProgressionManager".to_string(),
            data: ProgressionManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

impl MemoryManagerUpdate for ProgressionManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        // Unlocks change on scroll pickups/level-ups, not per frame.
        const REFRESH_FRAMES: u64 = 60;
        self.frame = self.frame.wrapping_add(1);
        if self.frame.is_multiple_of(REFRESH_FRAMES) || self.unlocked_combat_moves.is_empty() {
            self.update_unlocked_moves(&memory_context)?;
        }

        Ok(())
    }
}

impl ProgressionManagerData {
    pub fn update_unlocked_moves(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        let Ok(set_ptr) = memory_context.follow_fields::<u64>(&["unlockedCombatMoves"]) else {
            return Ok(());
        };
        self.unlocked_combat_moves = memory_context
            .hashset_item_ptrs(set_ptr)
            .into_iter()
            .filter_map(|str_obj| {
                // Each slot value is a C# string (chars at +0x14).
                let chars = memory_context
                    .process
                    .read_pointer::<ArrayWString<64>>(str_obj + 0x14)
                    .ok()?;
                String::from_utf16(chars.as_slice()).ok()
            })
            .collect();
        Ok(())
    }
}
