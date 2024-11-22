use super::GuiHelper;
use crate::{game_manager::GameManager, state::GameState};

use utility::prelude::*;

pub const NAME: &str = "Utility Helper";

struct GameContext {
    combo_points: u32,
    enemy_cur_hp: u32,
    enemy_max_hp: u32,
}

#[derive(Debug)]
pub struct UtilityHelper;

impl UtilityHelper {
    pub fn create() -> Box<Self> {
        Box::new(Self)
    }
}

impl GuiHelper for UtilityHelper {
    fn draw(
        &mut self,
        game_state: &mut GameState,
        _game_manager: &mut Option<GameManager>,
        ui: &mut egui::Ui,
        _tab: &mut String,
    ) {
        let cmd = &game_state.memory_managers.combat_manager.data;

        let appraisal = WeightedSum::<GameContext>::new(
            vec![
                (
                    1.0,
                    Lambda::<_, GameContext>::new(|ctx| ctx.combo_points as f64 / 3.0),
                ),
                (
                    2.0,
                    Lambda::<_, GameContext>::new(|ctx| {
                        ctx.enemy_cur_hp as f64 / ctx.enemy_max_hp as f64
                    }),
                ),
            ],
            true,
        );

        for enemy in &cmd.enemies.items {
            let ctx = GameContext {
                combo_points: cmd.combo_points,
                enemy_cur_hp: enemy.current_hp,
                enemy_max_hp: enemy.max_hp,
            };
            appraisal.render(&ctx, ui);
        }
    }
}
