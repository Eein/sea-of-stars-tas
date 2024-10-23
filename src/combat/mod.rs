#![allow(dead_code)]

mod controllers;

use crate::combat::controllers::basic_encounter_controller::BasicEncounterController;
use crate::combat::controllers::EncounterController;
use crate::control::SosAction;
use crate::seq::button::ButtonPress;
use crate::state::GameState;

#[derive(Debug)]
enum CombatFsm {
    Idle,
    Dialog,
    Action,
    Blocking,
    Consideration,
    Appraisal,
}

pub struct CombatManager {
    fsm: CombatFsm,
    btn: ButtonPress,
    controller: Option<Box<dyn EncounterController>>,
}

impl Default for CombatManager {
    fn default() -> Self {
        Self {
            btn: ButtonPress::new(SosAction::Confirm),
            fsm: CombatFsm::Idle,
            controller: None,
        }
    }
}

impl CombatManager {
    pub fn update(&mut self, state: &mut GameState, dt: f64) -> bool {
        let combat_manager = &state.memory_managers.combat_manager.data;

        if self.controller.is_none() {
            // determine controller
            self.controller = Some(Box::new(BasicEncounterController::default()));
        }

        if self.btn.update(&mut state.gamepad, dt) {
            self.btn = ButtonPress {
                action: SosAction::Confirm,
                press_time: 0.1,
                release_time: 0.2,
                ..Default::default()
            };
        }
        match self.fsm {
            CombatFsm::Idle => {
                // intended to wait for acceptable parameters
            }
            CombatFsm::Dialog => {
                // this may not be needed - can probably handle this outside of combat
                // by delegating control to whoever can deal with this.
            }
            CombatFsm::Action => {
                // Generate Action
            }
            CombatFsm::Blocking => {
                // Blocking Behaviour
            }
            CombatFsm::Consideration => {
                // Execute Consideration from action
            }
            CombatFsm::Appraisal => {
                // Execute Appraisal
            }
        }

        !combat_manager.encounter_active
    }
}
