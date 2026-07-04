use std::fmt::Display;

use crate::control::SosAction;
use crate::memory::title_sequence_manager::TitleMenuOption;
use crate::seq::button::ButtonPress;
use crate::state::{GameEvent, GameState};
use data::prelude::*;
use joystick::prelude::*;
use log::info;
use seq::prelude::*;

struct KonamiCode {
    sequence: Vec<SosAction>,
    step: usize,
    timer: f64,
}

impl Default for KonamiCode {
    fn default() -> Self {
        Self {
            sequence: vec![
                SosAction::MenuUp,
                SosAction::MenuUp,
                SosAction::MenuDown,
                SosAction::MenuDown,
                SosAction::MenuLeft,
                SosAction::MenuRight,
                SosAction::MenuLeft,
                SosAction::MenuRight,
                SosAction::Cancel,
                SosAction::Confirm,
            ],
            step: 0,
            timer: 0.0,
        }
    }
}

impl KonamiCode {
    const PRESS_TIMEOUT: f64 = 0.25;
    const WAIT_TIMEOUT: f64 = 0.5;

    fn update(&mut self, gamepad: &mut GenericJoystick, delta: f64) -> bool {
        if self.step >= self.sequence.len() {
            return true;
        }

        if self.timer < KonamiCode::PRESS_TIMEOUT {
            gamepad.press(&self.sequence[self.step]);
        } else if self.timer < KonamiCode::WAIT_TIMEOUT {
            gamepad.release_all();
        } else {
            self.step += 1;
            self.timer = 0.0;
        }

        self.timer += delta;
        false
    }
}

/// Seconds to wait after the title screen appears before sending input, to let
/// the intro animation finish. Only applied when the TAS launched the game
/// itself (cold boot); a game already at the title screen needs no wait.
const TITLE_SETTLE_SECS: f64 = 8.0;

#[derive(Debug)]
enum TitleScreenFSM {
    WaitForTitle,
    SettleAnimation,
    Konami,
    ToMenu,
    NewGame,
    PressNewGame,
    WaitSelectHero,
    SelectHero,
    PressSelectHero,
}

pub struct SeqTitleScreen {
    fsm: TitleScreenFSM,
    btn: ButtonPress,
    kc: KonamiCode,
    settle_timer: f64,
}

impl SeqTitleScreen {
    pub fn create() -> Box<Self> {
        Box::new(Self {
            fsm: TitleScreenFSM::WaitForTitle,
            btn: ButtonPress::default(),
            kc: KonamiCode::default(),
            settle_timer: 0.0,
        })
    }
}

impl Display for SeqTitleScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TitleScreen({:?})", self.fsm)
    }
}

impl Node<GameState, GameEvent> for SeqTitleScreen {
    fn enter(&mut self, state: &mut GameState) {
        state.release_all();
        info!("Starting TAS! Focus the Sea of Stars window before the timer expires.");
    }

    fn execute(&mut self, state: &mut GameState, delta: f64) -> bool {
        let tsmd = &state.memory_managers.title_sequence_manager.data;
        let ngc = &tsmd.new_game_characters;

        match self.fsm {
            TitleScreenFSM::WaitForTitle => {
                // Don't feed inputs until the title screen singleton exists and
                // the title screen is actually showing. Otherwise the Konami
                // code (and Start presses) land on boot/splash screens and are
                // lost before the menu is interactive.
                if tsmd.active && tsmd.current_screen_name == "TitleScreen" {
                    if state.game_launched_by_tas {
                        // On a cold boot we triggered, the title screen shows
                        // before its intro animation finishes accepting input.
                        info!("Title screen ready; waiting {TITLE_SETTLE_SECS}s for intro animation");
                        self.fsm = TitleScreenFSM::SettleAnimation;
                    } else {
                        info!("Title screen ready");
                        self.fsm = TitleScreenFSM::Konami;
                    }
                }
            }
            TitleScreenFSM::SettleAnimation => {
                self.settle_timer += delta;
                if self.settle_timer >= TITLE_SETTLE_SECS {
                    info!("Intro animation settle complete");
                    self.fsm = TitleScreenFSM::Konami;
                }
            }
            TitleScreenFSM::Konami => {
                if state.config.konami_code {
                    if self.kc.update(&mut state.gamepads[0], delta) {
                        self.fsm = TitleScreenFSM::ToMenu;
                        self.btn = ButtonPress::new(SosAction::Start);
                    }
                } else {
                    self.fsm = TitleScreenFSM::ToMenu;
                    self.btn = ButtonPress::new(SosAction::Start);
                }
            }
            TitleScreenFSM::ToMenu => {
                if self.btn.update(&mut state.gamepads[0], delta) {
                    self.btn = ButtonPress::new(SosAction::Start);
                }
                if tsmd.pressed_start {
                    self.fsm = TitleScreenFSM::NewGame;
                    self.btn = ButtonPress::new(SosAction::MenuDown);
                    state.release_all();
                    info!("Entering main menu");
                }
            }
            TitleScreenFSM::NewGame => {
                if tsmd.title_menu_option_selected == TitleMenuOption::NewGame {
                    self.btn = ButtonPress::new(SosAction::Confirm);
                    self.fsm = TitleScreenFSM::PressNewGame;
                    state.release_all();
                    info!("Selecting New Game");
                } else if self.btn.update(&mut state.gamepads[0], delta) {
                    self.btn = ButtonPress::new(SosAction::MenuDown);
                }
            }
            TitleScreenFSM::PressNewGame => {
                if self.btn.update(&mut state.gamepads[0], delta) {
                    self.fsm = TitleScreenFSM::WaitSelectHero;
                }
            }
            TitleScreenFSM::WaitSelectHero => {
                if ngc.left.character == PlayerPartyCharacter::Valere {
                    self.btn = ButtonPress::new(SosAction::MenuLeft);
                    self.fsm = TitleScreenFSM::SelectHero;
                } else if ngc.right.character == PlayerPartyCharacter::Valere {
                    self.btn = ButtonPress::new(SosAction::MenuRight);
                    self.fsm = TitleScreenFSM::SelectHero;
                }
            }
            TitleScreenFSM::SelectHero => {
                if ngc.selected == PlayerPartyCharacter::Valere {
                    self.btn = ButtonPress::new(SosAction::Confirm);
                    self.fsm = TitleScreenFSM::PressSelectHero;
                } else if self.btn.update(&mut state.gamepads[0], delta) {
                    self.fsm = TitleScreenFSM::WaitSelectHero;
                }
            }
            TitleScreenFSM::PressSelectHero => {
                if self.btn.update(&mut state.gamepads[0], delta) {
                    info!("Selected Valere");
                    return true;
                }
            }
        }
        false
    }

    fn exit(&self, state: &mut GameState) {
        state.release_all();
    }
}

#[derive(Debug)]
enum LoadGameFSM {
    ToMenu,
    LoadGame,
    PressLoadGame,
    SelectSlot,
    SelectSlotPress,
    ManualSlot,
    ConfirmLoad,
}

pub struct SeqLoadGame {
    fsm: LoadGameFSM,
    btn: ButtonPress,
    save_slot: usize,
    auto_save_present: bool,
}

impl SeqLoadGame {
    pub fn new(save_slot: usize, auto_save_present: bool) -> Box<Self> {
        Box::new(Self {
            fsm: LoadGameFSM::ToMenu,
            btn: ButtonPress::new(SosAction::Start),
            save_slot,
            auto_save_present,
        })
    }
}

impl Display for SeqLoadGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LoadGame(Slot {}): {:?}", self.save_slot, self.fsm)
    }
}

impl Node<GameState, GameEvent> for SeqLoadGame {
    fn enter(&mut self, state: &mut GameState) {
        state.release_all();
        info!(
            "Loading TAS from checkpoint! Focus the Sea of Stars window before the timer expires."
        );
    }

    fn execute(&mut self, state: &mut GameState, delta: f64) -> bool {
        let tsmd = &state.memory_managers.title_sequence_manager.data;
        match self.fsm {
            LoadGameFSM::ToMenu => {
                if self.btn.update(&mut state.gamepads[0], delta) {
                    self.btn = ButtonPress::new(SosAction::Start);
                }
                if tsmd.pressed_start {
                    self.fsm = LoadGameFSM::LoadGame;
                    self.btn = ButtonPress::new(SosAction::MenuDown);
                    state.release_all();
                    info!("Entering main menu");
                }
            }
            LoadGameFSM::LoadGame => {
                if tsmd.title_menu_option_selected == TitleMenuOption::LoadGame {
                    self.btn = ButtonPress::new(SosAction::Confirm);
                    self.fsm = LoadGameFSM::PressLoadGame;
                    state.release_all();
                    info!("Selecting Load Game");
                } else if self.btn.update(&mut state.gamepads[0], delta) {
                    self.btn = ButtonPress::new(SosAction::MenuDown);
                }
            }
            LoadGameFSM::PressLoadGame => {
                if self.btn.update(&mut state.gamepads[0], delta) {
                    self.fsm = LoadGameFSM::SelectSlot;
                }
            }
            LoadGameFSM::SelectSlot => {
                let page = (self.save_slot - 1) / 3;
                if page > 0 {
                    self.save_slot -= 3;
                    self.btn = ButtonPress::new(SosAction::ShiftRight);
                    self.fsm = LoadGameFSM::PressLoadGame;
                } else {
                    let vertical = (self.save_slot - 1) % 3;
                    if vertical > 0 {
                        self.save_slot -= 1;
                        self.btn = ButtonPress::new(SosAction::MenuDown);
                        self.fsm = LoadGameFSM::PressLoadGame;
                    } else {
                        self.btn = ButtonPress::new(SosAction::Confirm);
                        self.fsm = LoadGameFSM::SelectSlotPress;
                    }
                }
            }
            LoadGameFSM::SelectSlotPress => {
                if self.btn.update(&mut state.gamepads[0], delta) {
                    if self.auto_save_present {
                        self.btn = ButtonPress::new(SosAction::MenuDown);
                        self.fsm = LoadGameFSM::ManualSlot;
                    } else {
                        // Done
                        self.btn = ButtonPress::new(SosAction::Confirm);
                        self.fsm = LoadGameFSM::ConfirmLoad;
                    }
                }
            }
            LoadGameFSM::ManualSlot => {
                if self.btn.update(&mut state.gamepads[0], delta) {
                    self.auto_save_present = false;
                    self.fsm = LoadGameFSM::SelectSlot;
                }
            }
            LoadGameFSM::ConfirmLoad => {
                if self.btn.update(&mut state.gamepads[0], delta) {
                    return true;
                }
            }
        }

        false
    }

    fn exit(&self, state: &mut GameState) {
        state.release_all();
    }
}
