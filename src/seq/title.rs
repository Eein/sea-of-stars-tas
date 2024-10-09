use crate::control::SosAction;
use crate::memory::objects::character::PlayerPartyCharacter;
use crate::memory::title_sequence_manager::TitleMenuOption;
use crate::state::GameState;
use joystick::prelude::*;
use log::info;
use seq::prelude::*;

struct ButtonPress {
    action: SosAction,
    press_time: f64,
    release_time: f64,
    timer: f64,
}

impl Default for ButtonPress {
    fn default() -> Self {
        Self {
            action: SosAction::Confirm,
            press_time: 0.5,
            release_time: 1.0,
            timer: 0.0,
        }
    }
}

impl ButtonPress {
    pub fn new(action: SosAction) -> Self {
        Self {
            action,
            ..Default::default()
        }
    }

    pub fn update(&mut self, gamepad: &mut GenericJoystick, delta: f64) -> bool {
        self.timer += delta;
        // First press the button
        if self.timer < self.press_time {
            gamepad.press(&self.action);
        } else {
            gamepad.release(&self.action);
            if self.done() {
                return true;
            }
        }
        false
    }

    pub fn done(&self) -> bool {
        self.timer >= self.release_time
    }
}

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

enum TitleScreenFSM {
    Countdown,
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
    timer: f64,
}

// ! Countdown to let the user focus the TAS window
const COUNTDOWN_TIMEOUT: f64 = 5.0;

impl SeqTitleScreen {
    pub fn create() -> Box<Self> {
        Box::new(Self {
            fsm: TitleScreenFSM::Countdown,
            btn: ButtonPress::default(),
            kc: KonamiCode::default(),
            timer: COUNTDOWN_TIMEOUT,
        })
    }
}

impl Node<GameState> for SeqTitleScreen {
    fn enter(&mut self, state: &mut GameState) {
        state.gamepad.release_all();
        info!("Starting TAS! Focus the Sea of Stars window before the timer expires.");
    }

    fn execute(&mut self, state: &mut GameState, delta: f64) -> bool {
        let tsmd = &state.memory_managers.title_sequence_manager.data;
        let ngc = &tsmd.new_game_characters;

        match self.fsm {
            TitleScreenFSM::Countdown => {
                if self.timer.floor() != (self.timer - delta).floor() {
                    info!("Counting down to TAS start: {}", self.timer.floor());
                }
                self.timer -= delta;
                if self.timer <= 0.0 {
                    if state.config.konami_code {
                        self.fsm = TitleScreenFSM::Konami;
                        info!("Entering Konami code");
                    } else {
                        self.fsm = TitleScreenFSM::ToMenu;
                        self.btn = ButtonPress::new(SosAction::Start);
                    }
                }
            }
            TitleScreenFSM::Konami => {
                if self.kc.update(&mut state.gamepad, delta) {
                    self.fsm = TitleScreenFSM::ToMenu;
                    self.btn = ButtonPress::new(SosAction::Start);
                }
            }
            TitleScreenFSM::ToMenu => {
                self.btn.update(&mut state.gamepad, delta);
                if tsmd.pressed_start {
                    self.fsm = TitleScreenFSM::NewGame;
                    self.btn = ButtonPress::new(SosAction::MenuDown);
                    state.gamepad.release_all();
                }
            }
            TitleScreenFSM::NewGame => {
                if tsmd.title_menu_option_selected == TitleMenuOption::NewGame {
                    self.btn = ButtonPress::new(SosAction::Confirm);
                    self.fsm = TitleScreenFSM::PressNewGame;
                    state.gamepad.release_all();
                } else if self.btn.update(&mut state.gamepad, delta) {
                    self.btn = ButtonPress::new(SosAction::MenuDown);
                }
            }
            TitleScreenFSM::PressNewGame => {
                if self.btn.update(&mut state.gamepad, delta) {
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
                } else if self.btn.update(&mut state.gamepad, delta) {
                    self.fsm = TitleScreenFSM::WaitSelectHero;
                }
            }
            TitleScreenFSM::PressSelectHero => {
                if self.btn.update(&mut state.gamepad, delta) {
                    return true;
                }
            }
        }
        false
    }

    fn exit(&self, state: &mut GameState) {
        state.gamepad.release_all();
    }
}