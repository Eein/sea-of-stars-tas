#![allow(dead_code)]

pub mod appraisal;
mod controllers;
pub mod damage;
mod skills;

use joystick::common::JoystickBtnInterface;

use crate::combat::appraisal::Appraisal;
use crate::control::SosAction;
use crate::seq::button::ButtonPress;
use crate::state::GameState;

// The following imports are only used by the old WIP combat FSM below, kept
// commented out for reference:
// use data::prelude::PlayerPartyCharacter;
// use skills::{BasicAttack, Skill};
// use crate::combat::controllers::basic_encounter_controller::BasicEncounterController;
// use crate::combat::controllers::EncounterController;
// use crate::memory::combat_manager::{CombatControllerType, CombatDamageType};

// Old WIP combat state machine, kept for reference:
// #[derive(Debug)]
// enum CombatFsm {
//     Idle,
//     Dialog,
//     Action,
//     Blocking,
//     Consideration,
//     Appraisal,
// }

pub struct CombatManager {
    btn: ButtonPress,
    /// Controller index we were mashing on last frame, so we can release it if
    /// the active player changes mid-mash.
    last_gamepad: Option<usize>,
    /// Duty-cycle timer for mashing Confirm on all pads to dismiss mid-fight
    /// dialogue.
    dialog_timer: f64,
    /// Ranked candidate actions for the current combat state, recomputed every
    /// frame. Phase 1 only *decides* and surfaces these; acting on them is not
    /// wired up yet (we still mash Confirm to progress).
    pub appraisals: Vec<Appraisal>,
    /// The top-ranked appraisal, if any.
    pub chosen: Option<Appraisal>,
    // Old WIP FSM state, kept for reference:
    // fsm: CombatFsm,
    // controller: Option<Box<dyn EncounterController>>,
    // action: Option<Skill>,
}

impl Default for CombatManager {
    fn default() -> Self {
        Self {
            btn: ButtonPress::default(),
            last_gamepad: None,
            dialog_timer: 0.0,
            appraisals: Vec::new(),
            chosen: None,
            // Old WIP FSM state, kept for reference:
            // fsm: CombatFsm::Idle,
            // controller: None,
            // action: None,
        }
    }
}

impl CombatManager {
    pub fn update(&mut self, state: &mut GameState, dt: f64) -> bool {
        let encounter_active = state.memory_managers.combat_manager.data.encounter_active;

        // Decision layer: rank candidate actions for the current state. Phase 1
        // only decides and surfaces the result; the mashing below still does the
        // actual acting.
        self.appraise(state);

        // Mid-fight dialogue isn't tied to a player's turn, so mash Confirm on
        // every controller to dismiss it.
        if state.memory_managers.new_dialog_manager.data.dialog_visible {
            self.mash_all_confirm(state, dt);
            // Force a fresh per-player mash once dialogue clears.
            self.last_gamepad = None;
            return !encounter_active;
        }
        self.dialog_timer = 0.0;

        // Mash Confirm on the controller of the player whose turn it is. This is
        // re-read every frame, since the active player changes turn-to-turn. If
        // the encounter-players manager isn't active, assume a single player and
        // use controller 0.
        let gamepad = Self::active_gamepad(state);

        // If the active player changed, release every controller and restart the
        // mash so we never leave a button held on the previous one.
        if self.last_gamepad != Some(gamepad) {
            state.release_all();
            self.btn = Self::mash_press();
            self.last_gamepad = Some(gamepad);
        }

        if self.btn.update(&mut state.gamepads[gamepad], dt) {
            self.btn = Self::mash_press();
        }

        // Done once the encounter ends.
        !encounter_active
    }

    /// Recompute the ranked appraisals and the chosen (top) action from the
    /// current combat snapshot.
    fn appraise(&mut self, state: &GameState) {
        let cmd = &state.memory_managers.combat_manager.data;
        self.appraisals = appraisal::generate_appraisals(cmd);
        self.chosen = self.appraisals.first().cloned();
    }

    /// Tap Confirm on every controller in a duty cycle to dismiss mid-fight
    /// dialogue, regardless of which player owns the prompt.
    fn mash_all_confirm(&mut self, state: &mut GameState, dt: f64) {
        const PRESS: f64 = 0.1;
        const CYCLE: f64 = 0.3;
        self.dialog_timer += dt;
        if self.dialog_timer >= CYCLE {
            self.dialog_timer = 0.0;
        }
        let pressing = self.dialog_timer < PRESS;
        for gamepad in state.gamepads.iter_mut() {
            if pressing {
                gamepad.press(&SosAction::Confirm);
            } else {
                gamepad.release(&SosAction::Confirm);
            }
        }
    }

    /// A single Confirm tap used for mashing.
    fn mash_press() -> ButtonPress {
        ButtonPress {
            action: SosAction::Confirm,
            press_time: 0.1,
            release_time: 0.2,
            ..Default::default()
        }
    }

    /// Index of the controller to send combat input to.
    ///
    /// Controllers are mapped by *position* in the single-player-plus players
    /// list: the co-op join (`SeqMove` / `Move::Join`) binds `gamepads[i]` to
    /// `players.items[i]`. The active player's `index` field is a different
    /// numbering, so we find the list slot whose player matches the active one.
    /// Falls back to controller 0 when there's no active player or no match
    /// (single-player assumption).
    fn active_gamepad(state: &GameState) -> usize {
        let epmd = &state.memory_managers.encounter_players_manager.data;
        let Some(current_index) = epmd.current_player_index else {
            return 0;
        };

        let players = &state
            .memory_managers
            .single_player_plus_manager
            .data
            .players
            .items;
        if let Some(slot) = players.iter().position(|p| p.index == current_index)
            && slot < state.gamepads.len()
        {
            return slot;
        }
        0
    }

    // Old WIP combat behaviour (utility-AI action selection), kept for reference:
    //
    // pub fn update(&mut self, state: &mut GameState, dt: f64) -> bool {
    //     let combat_manager = &state.memory_managers.combat_manager.data;
    //
    //     if self.controller.is_none() {
    //         self.controller = Self::encounter_factory(state)
    //     }
    //
    //     // Execute Different states if controller is active
    //     if let Some(ref controller) = self.controller {
    //         // Always right before fsm - we never want to deal with dialog
    //         if controller.execute_dialog(state) {
    //             self.fsm = CombatFsm::Dialog
    //         }
    //     } else {
    //         // Otherwise set idle and wait for a controller
    //         self.fsm = CombatFsm::Idle
    //     }
    //
    //     match self.fsm {
    //         // intended to wait for acceptable parameters
    //         CombatFsm::Idle => {
    //             if self.action.is_none() && combat_manager.selected_character.is_some() {
    //                 println!("setting action");
    //                 self.fsm = CombatFsm::Action;
    //             }
    //         }
    //         CombatFsm::Dialog => {
    //             // this may not be needed - can probably handle this outside of combat
    //             // by delegating control to whoever can deal with this.
    //             // TODO(eein): button press here
    //         }
    //         CombatFsm::Action => {
    //             // Generate Action from Utility and select
    //             println!("action");
    //             self.action = Some(Skill {
    //                 character: PlayerPartyCharacter::Zale,
    //                 internal_name: "ZaleBasicAttack",
    //                 timing_type: skills::TimingType::MultiHit,
    //                 target_type: skills::TargetType::Enemy,
    //                 resource: skills::SkillResource::None,
    //                 damage_types: [CombatDamageType::Sword].to_vec(),
    //                 battle_command: skills::BattleCommand::Attack,
    //                 timing_controller: Box::new(BasicAttack),
    //                 cost: 0,
    //             });
    //
    //             if self.action.is_some() {
    //                 self.fsm = CombatFsm::Consideration;
    //             }
    //         }
    //         CombatFsm::Blocking => {
    //             // Blocking Behaviour
    //         }
    //         CombatFsm::Consideration => {
    //             // Execute Consideration from action (selection)
    //             if let Some(action) = &self.action {
    //                 if combat_manager.selected_character != Some(action.character.clone()) {
    //                     // press right if consideration is invalid
    //                     if self.btn.update(&mut state.gamepads[0], dt) {
    //                         self.btn = ButtonPress {
    //                             action: SosAction::MenuRight,
    //                             press_time: 0.05,
    //                             release_time: 0.05,
    //                             ..Default::default()
    //                         }
    //                     }
    //                 } else {
    //                     self.fsm = CombatFsm::Appraisal;
    //                 }
    //             }
    //         }
    //         CombatFsm::Appraisal => {
    //             // Execute Appraisal
    //         }
    //     }
    //
    //     !combat_manager.encounter_active
    // }
    //
    // fn encounter_factory(state: &GameState) -> Option<Box<dyn EncounterController>> {
    //     match &state
    //         .memory_managers
    //         .combat_manager
    //         .data
    //         .combat_controller_type
    //     {
    //         CombatControllerType::Basic => Some(Box::new(BasicEncounterController::default())),
    //         _ => Some(Box::new(BasicEncounterController::default())),
    //     }
    // }
}
