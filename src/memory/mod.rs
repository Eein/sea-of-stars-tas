pub mod boat_manager;
pub mod combat_manager;
pub mod currency_manager;
pub mod cutscene_manager;
pub mod inventory_manager;
pub mod level_manager;
pub mod level_up_manager;
pub mod memory_context;
pub mod new_dialog_manager;
pub mod player_party_manager;
pub mod shop_manager;
pub mod speedrun_manager;
pub mod time_of_day_manager;
pub mod title_sequence_manager;

use log::error;

use crate::state::StateContext;

use boat_manager::BoatManagerData;
use combat_manager::CombatManagerData;
use currency_manager::CurrencyManagerData;
use cutscene_manager::CutsceneManagerData;
use inventory_manager::InventoryManagerData;
use level_manager::LevelManagerData;
use level_up_manager::LevelUpManagerData;
use memory::memory_manager::il2cpp::{UnityMemoryManagement, UnityMemoryManager};
use memory::process::MemoryError;
use new_dialog_manager::NewDialogManagerData;
use player_party_manager::PlayerPartyManagerData;
use shop_manager::ShopManagerData;
use speedrun_manager::SpeedrunManagerData;
use time_of_day_manager::TimeOfDayManagerData;
use title_sequence_manager::TitleSequenceManagerData;

pub trait MemoryManagerUpdate {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError>;
}

pub struct MemoryManager<T: MemoryManagerUpdate> {
    pub name: String,
    pub manager: UnityMemoryManager,
    pub data: T,
}

#[derive(Default)]
pub struct MemoryManagers {
    pub title_sequence_manager: MemoryManager<TitleSequenceManagerData>,
    pub player_party_manager: MemoryManager<PlayerPartyManagerData>,
    pub time_of_day_manager: MemoryManager<TimeOfDayManagerData>,
    pub boat_manager: MemoryManager<BoatManagerData>,
    pub level_manager: MemoryManager<LevelManagerData>,
    pub currency_manager: MemoryManager<CurrencyManagerData>,
    pub new_dialog_manager: MemoryManager<NewDialogManagerData>,
    pub combat_manager: MemoryManager<CombatManagerData>,
    pub cutscene_manager: MemoryManager<CutsceneManagerData>,
    pub shop_manager: MemoryManager<ShopManagerData>,
    pub speedrun_manager: MemoryManager<SpeedrunManagerData>,
    pub inventory_manager: MemoryManager<InventoryManagerData>,
    pub level_up_manager: MemoryManager<LevelUpManagerData>,
}

impl MemoryManagers {
    pub fn update(&mut self, ctx: &StateContext) {
        if self.ready_for_updates(ctx) {
            self.title_sequence_manager.update(ctx);
            self.player_party_manager.update(ctx);
            self.time_of_day_manager.update(ctx);
            self.boat_manager.update(ctx);
            self.level_manager.update(ctx);
            self.currency_manager.update(ctx);
            self.new_dialog_manager.update(ctx);
            self.combat_manager.update(ctx);
            self.cutscene_manager.update(ctx);
            self.shop_manager.update(ctx);
            self.speedrun_manager.update(ctx);
            self.inventory_manager.update(ctx);
            self.level_up_manager.update(ctx);
        }
    }
    pub fn ready_for_updates(&mut self, ctx: &StateContext) -> bool {
        ctx.process.is_some() && ctx.module.is_some() && ctx.image.is_some()
    }
}

impl<T: MemoryManagerUpdate> MemoryManager<T> {
    fn ready_for_updates(&mut self, _ctx: &StateContext) -> bool {
        matches!(self.manager.singleton, Some(_val))
    }

    fn update_manager(&mut self, ctx: &StateContext) {
        if let Some(process) = &ctx.process {
            if let Some(module) = &ctx.module {
                if let Some(image) = &ctx.image {
                    self.manager.update(process, module, image, &self.name);
                }
            }
        }
    }

    fn update_memory(&mut self, ctx: &StateContext) {
        match self.data.update(ctx, &mut self.manager) {
            Ok(_) => (),
            Err(error) => {
                error!(
                    "Memory Update Error in {} with error {:?}",
                    self.name, error
                );
                match error {
                    MemoryError::ReadError => self.manager.reset(),
                    MemoryError::NullPointer => {}
                    MemoryError::Unset => self.manager.reset(),
                    MemoryError::InvalidParameters => {}
                }
            }
        }
    }

    fn update(&mut self, ctx: &StateContext) {
        self.update_manager(ctx);
        if self.ready_for_updates(ctx) {
            self.update_memory(ctx);
        }
    }
}
