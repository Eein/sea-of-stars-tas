mod controllers;

use crate::combat::controllers::CombatController; 
use crate::{state::GameState};

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
    controller: Option<&dyn CombatController>
}

impl Default for CombatManager {
    fn default() -> Self {
        Self {
            fsm: CombatFsm::Idle,
        }
    }
}

impl CombatManager {
    pub fn update(&mut self, state: &mut GameState, _dt: f64) -> bool {
        let combat_manager = &state.memory_managers.combat_manager.data;

        match self.fsm {
            CombatFsm::Idle => {
                // Nothing to do, but we dont want to give control back
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
            CombatFsm::Appraisal=> {
                // Execute Appraisal
            }
        }

        !combat_manager.encounter_active
    }
}
