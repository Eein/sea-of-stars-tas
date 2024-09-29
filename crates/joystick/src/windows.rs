use std::time::{Duration, Instant};
use vigem_client::{Client, XButtons, Xbox360Wired};

static TAP_DURATION: u64 = 50;

enum KeyAction {
    Press,
    Release,
}

pub struct Joystick {
    device: Xbox360Wired<Client>, // may need to be Arc<Mutex>>
    gamepad: vigem_client::XGamepad,
    events: Vec<JoystickEvent>,
    mask: u16, // current mask for the joystick
    pub instant: Instant,
}

struct JoystickEvent {
    key: u16,
    duration: Duration,
    action: KeyAction,
}

impl Default for Joystick {
    fn default() -> Self {
        let client = Client::connect().unwrap();
        let id = vigem_client::TargetId::XBOX360_WIRED;
        let mut device = Xbox360Wired::new(client, id);

        device.plugin().unwrap();

        let gamepad = vigem_client::XGamepad {
            buttons: vigem_client::XButtons!(
                UP | DOWN | LEFT | RIGHT | LTHUMB | RTHUMB | LB | RB | GUIDE | A | B | X | Y
            ),
            ..Default::default()
        };

        Joystick {
            device,
            events: vec![],
            mask: 0x0,
            instant: Instant::now(),
            gamepad,
        }
    }
}

impl Joystick {
    pub fn new(&self) -> Self {
        Joystick::default()
    }

    pub fn tap_a(&mut self) {
        self.tap(XButtons::A)
    }

    pub fn tap_b(&mut self) {
        self.tap(XButtons::B)
    }

    pub fn tap_x(&mut self) {
        self.tap(XButtons::X)
    }

    pub fn tap_y(&mut self) {
        self.tap(XButtons::Y)
    }

    pub fn tap_lt(&mut self) {
        self.tap(XButtons::LB)
    }

    pub fn tap_rt(&mut self) {
        self.tap(XButtons::RB)
    }

    // TODO(eein): We dont rear triggers yet
    pub fn tap_lt2(&mut self) {
        // self.tap(XButtons::LB)
    }

    pub fn tap_rt2(&mut self) {
        // self.tap(XButtons::RB)
    }

    pub fn tap_select(&mut self) {
        self.tap(XButtons::BACK)
    }

    pub fn tap_start(&mut self) {
        self.tap(XButtons::START)
    }

    pub fn tap_dpad_up(&mut self) {
        self.tap(XButtons::UP)
    }

    pub fn tap_dpad_down(&mut self) {
        self.tap(XButtons::DOWN)
    }

    pub fn tap_dpad_left(&mut self) {
        self.tap(XButtons::LEFT)
    }

    pub fn tap_dpad_right(&mut self) {
        self.tap(XButtons::RIGHT)
    }

    pub fn release_all(&mut self) {
        self.gamepad.buttons = vigem_client::XButtons!();
    }

    pub fn tap(&mut self, button: u16) {
        // send in press and release
        let time = self.instant.elapsed();
        let release_duration = Duration::from_millis(TAP_DURATION);

        self.events.push(JoystickEvent {
            key: button,
            duration: time,
            action: KeyAction::Press,
        });
        self.events.push(JoystickEvent {
            key: button,
            duration: time + release_duration,
            action: KeyAction::Release,
        });
    }

    pub fn run(&mut self) {
        if !self.events.is_empty() {
            let timer_time = self.instant.elapsed();

            for event in &self.events {
                if event.duration <= timer_time {
                    match event.action {
                        KeyAction::Release => {
                            // bitwise AND NOT (removes the button from the bitflags)
                            self.mask = self.mask & !event.key
                        }
                        KeyAction::Press => {
                            // bitwise OR (adds the button to the bitflags)
                            self.mask = self.mask | event.key
                        }
                    };
                    self.gamepad.buttons.raw = self.mask;
                    let _ = self.device.update(&self.gamepad);
                }
            }
            self.events.retain(|event| event.duration > timer_time);
        } else {
            self.instant = Instant::now();
        }
    }
}
