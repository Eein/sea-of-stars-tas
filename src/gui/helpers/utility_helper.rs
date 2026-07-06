use super::GuiHelper;
use crate::{state::GameState, tas_runner::TasRunner};

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
        _tas_runner: &mut Option<TasRunner>,
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

        for (i, enemy) in cmd.enemies.items.iter().enumerate() {
            let ctx = GameContext {
                combo_points: cmd.combo_points,
                enemy_cur_hp: enemy.current_hp,
                enemy_max_hp: enemy.max_hp,
            };
            ui.push_id(i, |ui| {
                appraisal.render(&ctx, ui);
            });
        }
    }
}
