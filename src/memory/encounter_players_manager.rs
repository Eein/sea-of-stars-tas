use crate::memory::memory_context::MemoryContext;
use crate::memory::single_player_plus_manager::Player;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;

use log::info;

use memory::game_engine::il2cpp::unity_list::UnityList;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;

/// Tracks the player currently taking their turn during an encounter. The
/// players here are `SinglePlayerPlusPlayer` objects (the same type read by
/// [`crate::memory::single_player_plus_manager`]), so we reuse [`Player`].
#[derive(Default, Debug)]
pub struct EncounterPlayersManagerData {
    /// Whether the manager has finished initializing.
    pub initialized: bool,
    /// The `index` of the player currently taking its turn
    /// (`currentPlayer->index`), or `None` when there is no active player. This
    /// is the reliable turn signal (the root `currentPlayerIndex` is not).
    pub current_player_index: Option<i32>,
    /// Every player in the encounter (`allPlayers`).
    pub all_players: UnityList<Player>,
}

impl Default for MemoryManager<EncounterPlayersManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "EncounterPlayersManager".to_string(),
            data: EncounterPlayersManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

impl MemoryManagerUpdate for EncounterPlayersManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_initialized(&memory_context)?;
        self.update_current_player(&memory_context)?;
        self.update_all_players(&memory_context)?;

        Ok(())
    }
}

impl EncounterPlayersManagerData {
    fn update_initialized(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        self.initialized = matches!(memory_context.follow_fields::<u8>(&["initialized"]), Ok(1));
        Ok(())
    }

    fn update_current_player(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        // Follow the full path so `currentPlayer` is dereferenced and `index` is
        // read from it. A null currentPlayer makes this fail, which we treat as
        // no active player.
        self.current_player_index = memory_context
            .follow_fields::<i32>(&["currentPlayer", "index"])
            .ok();

        Ok(())
    }

    fn update_all_players(&mut self, memory_context: &MemoryContext) -> Result<(), MemoryError> {
        if let Ok(players) = memory_context.follow_fields::<u64>(&["allPlayers"])
            && players != 0
        {
            self.all_players = UnityList::<Player>::read(memory_context.process, players)?;
        }
        Ok(())
    }
}
