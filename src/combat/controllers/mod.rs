pub mod basic_encounter_controller;

use crate::memory::combat_manager::CombatManagerData; 

pub struct CombatController<T: EncounterController> {
    pub controller: T,
    // joystick things?
    // maybe use an rc/arc to borrow a handle to the combat memory manager here?
}

pub trait EncounterController {
    /// If combat is done, just exit.
    fn encounter_done(&self) -> bool;

    /// Mashing confirm to dismiss dialog on screen.
    ///
    /// There are some fights with dialog mid-fight and
    /// this will get us through it until we regain control.
    fn execute_dialog(&self, manager: &CombatManagerData) -> bool;

    /// Generate an action.
    ///
    /// If we dont have an action or the current appraisal is complete, we make a new one.
    /// We also check if battle command has focus,
    /// so it doesn't start executing before we have control.
    fn generate_action(&self, manager: &CombatManagerData) -> bool;

    /// Handle block execution.
    ///
    /// Currently it will spam confirm to block.
    fn execute_block(&self, manager: &CombatManagerData) -> bool;

    /// Execute the consideration.
    ///
    /// If the consideration doesn't believe the situation is valid, execute changing the selected
    /// consideration (character). This will rotate the cursor to the next available consideration
    /// until it finds the one it expects, or if the character is currently benched. It will 
    /// perform the necessary actions to bring a character from off the bench.
    ///
    /// If the appraisal step is not the initial step, we should not be executing the consideration
    /// to prevent it from moving the cursor when we are in an actions lifecycle.
    fn execute_consideration(&self, manager: &CombatManagerData) -> bool;

    // Execute the appraisal of the action.
    //
    // If the appraisal completes its lifecycle it will set self.action to None
    // so that a new action can be generated.
    fn execute_appraisal(&self, manager: &CombatManagerData) -> bool;

    /// Check if an action has been assigned by utility and returns true if so.
    ///
    /// If there is no action to act on, it should be unable to continue with
    /// attempting to take control and attack an enemy.
    fn has_action(&self, manager: &CombatManagerData) -> bool;

    /// Check if we should be generating an action.
    /// This can happen if there is no action, the previous appraisal is complete
    fn _should_generate_action(&self, manager: &CombatManagerData) -> bool;

    /// Check if we should block. This is used to determine if we should be blocking.
    fn _should_block(&self, manager: &CombatManagerData) -> bool;

    /// Determine if we are actively blocking an attack.
    fn _is_blocking_attack(&self, manager: &CombatManagerData) -> bool;

    /// Determine if we are actively blocking a spell.
    fn _is_blocking_spell(&self, manager: &CombatManagerData) -> bool;

    /// Check the consideration to see if it's valid and return true if so.
    fn _consideration_valid(&self, manager: &CombatManagerData) -> bool;
}

