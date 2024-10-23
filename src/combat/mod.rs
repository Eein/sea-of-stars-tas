#![allow(dead_code)]

mod controllers;

use crate::combat::controllers::basic_encounter_controller::BasicEncounterController;
use crate::combat::controllers::EncounterController;
use crate::control::SosAction;
use crate::memory::combat_manager::CombatControllerType;
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
            self.controller = Self::encounter_factory(state)
        }

        if self.btn.update(&mut state.gamepad, dt) {
            self.btn = ButtonPress {
                action: SosAction::Confirm,
                press_time: 0.1,
                release_time: 0.2,
                ..Default::default()
            };
        }

        // Execute Different states if controller is active
        if let Some(ref controller) = self.controller {
            // Always right before fsm - we never want to deal with dialog
            if controller.execute_dialog(state) {
                self.fsm = CombatFsm::Dialog
            }
        } else {
            // Otherwise set idle and wait for a controller
            self.fsm = CombatFsm::Idle
        }

        match self.fsm {
            CombatFsm::Idle => {
                // intended to wait for acceptable parameters
            }
            CombatFsm::Dialog => {
                // this may not be needed - can probably handle this outside of combat
                // by delegating control to whoever can deal with this.
                // TODO(eein): button press here
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

    fn encounter_factory(state: &GameState) -> Option<Box<dyn EncounterController>> {
        match &state
            .memory_managers
            .combat_manager
            .data
            .combat_controller_type
        {
            CombatControllerType::Basic => Some(Box::new(BasicEncounterController::default())),
            // CombatControllerType::FirstEncounter => Some(Box::new(FirstEncounterController::default())),
            // CombatControllerType::SecondEncounter => Some(Box::new(SecondEncounterController::default())),
            // CombatControllerType::DwellerOfStrife => Some(Box::new(DwellerOfStrifeEncounterController::default())),
            // CombatControllerType::DwellerOfDread => Some(Box::new(DwellerOfDreadEncounterController::default())),
            // CombatControllerType::KOTutorial => Some(Box::new(KOTutorialEncounterController::default())),
            // CombatControllerType::LiveManaTutorial => Some(Box::new(LiveManaTutorialEncounterController::default())),
            // CombatControllerType::ManaRegenTutorial => Some(Box::new(ManaRegenTutorialEncounterController::default())),
            // CombatControllerType::RoundsTutorial => Some(Box::new(RoundsTutorialEncounterController::default())),
            // CombatControllerType::SpellLockTutorial => Some(Box::new(SpellLockTutorialEncounterController::default())),
            // CombatControllerType::TimedBlocksTutorial => Some(Box::new(TimedBlocksTutorialEncounterController::default())),
            // CombatControllerType::TimedHitsTutorial => Some(Box::new(TimedHitsTutorialEncounterController::default())),
            _ => Some(Box::new(BasicEncounterController::default())),
        }
    }
}
