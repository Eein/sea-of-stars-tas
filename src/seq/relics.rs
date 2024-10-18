use crate::control::SosAction;
use crate::memory::title_sequence_manager::{RelicButton, TitleSequenceManagerData};
use crate::state::GameState;
use joystick::prelude::*;
use log::{info, warn};
use seq::prelude::*;

use crate::seq::button::ButtonPress;

#[derive(Default)]
enum RelicScreenFSM {
    #[default]
    Eval,
    PressButton,
}

#[derive(Default)]
pub struct SeqRelicList {
    fsm: RelicScreenFSM,
    btn: ButtonPress,
    first: Option<String>,
    prev: Option<String>,
    done: bool,
}

impl SeqRelicList {
    pub fn create() -> Box<Self> {
        Box::new(Self::default())
    }

    // Shorthand to get the currently selected relic
    fn get_current_relic<'a>(&self, tsmd: &'a TitleSequenceManagerData) -> Option<&'a RelicButton> {
        tsmd.relic_buttons
            .items
            .iter()
            .find(|&button| button.selected)
    }

    // Set up FSM to move to next relic in the list
    fn next_relic(&mut self) {
        self.fsm = RelicScreenFSM::PressButton;
        self.btn = ButtonPress {
            action: SosAction::MenuDown,
            press_time: 0.1,
            release_time: 0.1,
            ..Default::default()
        };
    }

    // Set up FSM to toggle relic state
    fn toggle_relic(&mut self) {
        self.fsm = RelicScreenFSM::PressButton;
        self.btn = ButtonPress {
            action: SosAction::Confirm,
            press_time: 0.1,
            release_time: 0.1,
            ..Default::default()
        };
    }

    // When done, press start
    fn press_start(&mut self) {
        self.done = true;
        self.fsm = RelicScreenFSM::PressButton;
        self.btn = ButtonPress {
            action: SosAction::Start,
            ..Default::default()
        };
    }

    // Check if we have wrapped by comparing current selected relic to the first in the list.
    // If there is no first relic, store its name now. We must have moved from a different
    // relic before (a previous relic name is held and updated).
    fn check_done(&mut self, cur_relic: &str) -> bool {
        if let Some(prev_name) = &self.prev {
            if let Some(first_name) = &self.first {
                let wrapped = cur_relic == first_name && cur_relic != prev_name;
                self.prev = Some(cur_relic.to_owned());
                return wrapped;
            }
        } else {
            self.prev = Some(cur_relic.to_owned());
            self.first = Some(cur_relic.to_owned());
        }
        false
    }
}

impl Node<GameState> for SeqRelicList {
    fn enter(&mut self, state: &mut GameState) {
        state.gamepad.release_all();
    }

    fn execute(&mut self, state: &mut GameState, delta: f64) -> bool {
        let tsmd = &state.memory_managers.title_sequence_manager.data;

        match self.fsm {
            RelicScreenFSM::Eval => {
                // Check if we can get the currently selected relic
                if let Some(cur_relic) = self.get_current_relic(tsmd) {
                    // Check if we have wrapped
                    if self.check_done(&cur_relic.name) {
                        info!("Relic sequence done!");
                        self.press_start();
                    } else {
                        match state.config.relics.get(&cur_relic.name) {
                            Some(relic) => {
                                if *relic != cur_relic.enabled {
                                    info!("Toggling relic '{}' to {}", &cur_relic.name, relic);
                                    self.toggle_relic();
                                } else {
                                    self.next_relic();
                                }
                            }
                            None => {
                                warn!("Relic '{}' not in config, ignored!", cur_relic.name);
                                self.next_relic();
                            }
                        }
                    }
                }
            }
            RelicScreenFSM::PressButton => {
                if self.btn.update(&mut state.gamepad, delta) {
                    if self.done {
                        return true;
                    }
                    self.fsm = RelicScreenFSM::Eval;
                }
            }
        }

        false
    }
}
