use crate::control::SosAction;
use log::info;

use crate::{memory::level_up_manager::LevelUpUpgrade, seq::button::ButtonPress, state::GameState};

#[derive(Debug)]
enum LevelUpFsm {
    Select,
    Press,
}

pub struct LevelUpManager {
    btn: ButtonPress,
    fsm: LevelUpFsm,
}

impl Default for LevelUpManager {
    fn default() -> Self {
        Self {
            btn: ButtonPress::new(crate::control::SosAction::MenuRight),
            fsm: LevelUpFsm::Select,
        }
    }
}

const STAT_PRIORITY: [LevelUpUpgrade; 6] = [
    LevelUpUpgrade::SkillPoint,
    LevelUpUpgrade::MagicalAttack,
    LevelUpUpgrade::PhysicalAttack,
    LevelUpUpgrade::HitPoint,
    LevelUpUpgrade::MagicalDefense,
    LevelUpUpgrade::PhysicalDefense,
];

impl LevelUpManager {
    pub fn update(&mut self, state: &mut GameState, dt: f64) -> bool {
        let lumd = &state.memory_managers.level_up_manager.data;

        let active_char = &lumd.current_character;

        let mut best_option = LevelUpUpgrade::None;
        let mut best_index = 99;

        for upgrade in &lumd.current_upgrades.items {
            if let Some(considered_index) = STAT_PRIORITY.iter().position(|u| *u == upgrade.upgrade)
            {
                if considered_index < best_index {
                    best_index = considered_index;
                    best_option = upgrade.upgrade.clone();
                }
            }
        }

        match self.fsm {
            LevelUpFsm::Select => {
                if self.btn.update(&mut state.gamepad, dt) {
                    self.btn = ButtonPress::new(SosAction::MenuRight);
                }
                for upgrade in &lumd.current_upgrades.items {
                    if upgrade.upgrade == best_option && upgrade.selected {
                        self.btn = ButtonPress::new(SosAction::Confirm);
                        self.fsm = LevelUpFsm::Press;
                        break;
                    }
                }
            }
            LevelUpFsm::Press => {
                if self.btn.update(&mut state.gamepad, dt) {
                    self.btn = ButtonPress::new(SosAction::MenuRight);
                    self.fsm = LevelUpFsm::Select;
                    info!("LevelUp: selecting {:?} for {:?}", best_option, active_char);
                }
            }
        }

        !lumd.active
    }
}
