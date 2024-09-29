use std::time::{Duration, Instant};
use vigem_client::{Client, TargetId, XButtons, XGamepad, Xbox360Wired};

use crate::common::JoystickInterface;
use crate::common::{JoystickInterface, KeyAction, TAP_DURATION};

pub struct Joystick {
    device: Xbox360Wired<Client>, // may need to be Arc<Mutex>>
    gamepad: vigem_client::XGamepad,
    events: Vec<JoystickEvent>,
    button_mask: u16, // current mask for the joystick
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
        let id = TargetId::XBOX360_WIRED;
        let mut device = Xbox360Wired::new(client, id);

        device.plugin().unwrap();

        let gamepad = XGamepad {
            buttons: vigem_client::XButtons!(
                UP | DOWN | LEFT | RIGHT | LTHUMB | RTHUMB | LB | RB | GUIDE | A | B | X | Y
            ),
            ..Default::default()
        };

        Joystick {
            device,
            events: vec![],
            button_mask: 0x0,
            instant: Instant::now(),
            gamepad,
        }
    }
}

impl JoystickInterface for Joystick {
    fn press_a(&mut self) {
        self.press(XButtons::A)
    }

    fn press_b(&mut self) {
        self.press(XButtons::B)
    }

    fn press_x(&mut self) {
        self.press(XButtons::X)
    }

    fn press_y(&mut self) {
        self.press(XButtons::Y)
    }

    fn press_lt(&mut self) {
        self.press(XButtons::LB)
    }

    fn press_rt(&mut self) {
        self.press(XButtons::RB)
    }

    // TODO(eein): We dont rear triggers yet
    fn press_lt2(&mut self) {
        // self.press(XButtons::LB)
    }

    fn press_rt2(&mut self) {
        // self.press(XButtons::RB)
    }

    fn press_select(&mut self) {
        self.press(XButtons::BACK)
    }

    fn press_start(&mut self) {
        self.press(XButtons::START)
    }

    fn press_dpad_up(&mut self) {
        self.press(XButtons::UP)
    }

    fn press_dpad_down(&mut self) {
        self.press(XButtons::DOWN)
    }

    fn press_dpad_left(&mut self) {
        self.press(XButtons::LEFT)
    }

    fn press_dpad_right(&mut self) {
        self.press(XButtons::RIGHT)
    }

    fn release_a(&mut self) {
        self.release(XButtons::A)
    }

    fn release_b(&mut self) {
        self.release(XButtons::B)
    }

    fn release_x(&mut self) {
        self.release(XButtons::X)
    }

    fn release_y(&mut self) {
        self.release(XButtons::Y)
    }

    fn release_lt(&mut self) {
        self.release(XButtons::LB)
    }

    fn release_rt(&mut self) {
        self.release(XButtons::RB)
    }

    // TODO(eein): We dont rear triggers yet
    fn release_lt2(&mut self) {
        // self.release(XButtons::LB)
    }

    fn release_rt2(&mut self) {
        // self.release(XButtons::RB)
    }

    fn release_select(&mut self) {
        self.release(XButtons::BACK)
    }

    fn release_start(&mut self) {
        self.release(XButtons::START)
    }

    fn release_dpad_up(&mut self) {
        self.release(XButtons::UP)
    }

    fn release_dpad_down(&mut self) {
        self.release(XButtons::DOWN)
    }

    fn release_dpad_left(&mut self) {
        self.release(XButtons::LEFT)
    }

    fn release_dpad_right(&mut self) {
        self.release(XButtons::RIGHT)
    }

    fn tap_a(&mut self) {
        self.tap(XButtons::A)
    }

    fn tap_b(&mut self) {
        self.tap(XButtons::B)
    }

    fn tap_x(&mut self) {
        self.tap(XButtons::X)
    }

    fn tap_y(&mut self) {
        self.tap(XButtons::Y)
    }

    fn tap_lt(&mut self) {
        self.tap(XButtons::LB)
    }

    fn tap_rt(&mut self) {
        self.tap(XButtons::RB)
    }

    // TODO(eein): We dont rear triggers yet
    fn tap_lt2(&mut self) {
        // self.tap(XButtons::LB)
    }

    fn tap_rt2(&mut self) {
        // self.tap(XButtons::RB)
    }

    fn tap_select(&mut self) {
        self.tap(XButtons::BACK)
    }

    fn tap_start(&mut self) {
        self.tap(XButtons::START)
    }

    fn tap_dpad_up(&mut self) {
        self.tap(XButtons::UP)
    }

    fn tap_dpad_down(&mut self) {
        self.tap(XButtons::DOWN)
    }

    fn tap_dpad_left(&mut self) {
        self.tap(XButtons::LEFT)
    }

    fn tap_dpad_right(&mut self) {
        self.tap(XButtons::RIGHT)
    }

    fn release_all(&mut self) {
        self.gamepad.buttons = vigem_client::XButtons!();
    }

    fn run(&mut self) {
        if !self.events.is_empty() {
            let timer_time = self.instant.elapsed();

            for event in &self.events {
                if event.duration <= timer_time {
                    match event.action {
                        KeyAction::Release => {
                            // bitwise AND NOT (removes the button from the bitflags)
                            self.button_mask &= !event.key
                        }
                        KeyAction::Press => {
                            // bitwise OR (adds the button to the bitflags)
                            self.button_mask |= event.key
                        }
                    };
                    self.gamepad.buttons.raw = self.button_mask;
                    let _ = self.device.update(&self.gamepad);
                }
            }
            self.events.retain(|event| event.duration > timer_time);
        } else {
            self.instant = Instant::now();
        }
    }

    fn press(&mut self, button: u16) {
        let time = self.instant.elapsed();

        self.events.push(JoystickEvent {
            key: button,
            event_type: EventType::KEY,
            duration: time,
            action: KeyAction::Press,
        });
    }

    fn release(&mut self, button: u16) {
        let time = self.instant.elapsed();

        self.events.push(JoystickEvent {
            key: button,
            event_type: EventType::KEY,
            duration: time,
            action: KeyAction::Release,
        });
    }

    fn release_later(&mut self, button: u16, duration: Duration) {
        let time = self.instant.elapsed();

        self.events.push(JoystickEvent {
            key: button,
            event_type: EventType::KEY,
            duration: time + duration,
            action: KeyAction::Release,
        });
    }

    fn tap(&mut self, button: u16) {
        // send in press and release
        let release_duration = Duration::from_millis(TAP_DURATION);
        self.press(button);
        self.release_later(button, release_duration);
    }
}

// To visually run these tests, use:
// `cargo test -- --test-threads 1`
// then focus https://hardwaretester.com/gamepad 
#[cfg(test)]
mod tests {
    use crate::common::{JoystickInterface, Button};
    use crate::joystick::Joystick;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_event_system() -> std::io::Result<()> {
        sleep(Duration::from_millis(2000));
        let mut joystick: Joystick = Joystick::default();
        sleep(Duration::from_millis(500));
        joystick.press(Button::UP);
        sleep(Duration::from_millis(500));
        joystick.press(Button::DOWN);
        sleep(Duration::from_millis(500));
        joystick.press(Button::LEFT);
        sleep(Duration::from_millis(500));
        joystick.press(Button::RIGHT);
        sleep(Duration::from_millis(500));
        joystick.press(Button::A);
        sleep(Duration::from_millis(500));
        joystick.press(Button::B);
        sleep(Duration::from_millis(500));
        joystick.press(Button::X);
        sleep(Duration::from_millis(500));
        joystick.press(Button::Y);
        sleep(Duration::from_millis(500));
        joystick.press(Button::LT);
        sleep(Duration::from_millis(500));
        joystick.press(Button::RT);
        sleep(Duration::from_millis(500));
        joystick.press(Button::LT2);
        sleep(Duration::from_millis(500));
        joystick.press(Button::RT2);
        sleep(Duration::from_millis(500));
        joystick.press(Button::SELECT);
        sleep(Duration::from_millis(500));
        joystick.press(Button::START);
        sleep(Duration::from_millis(500));

        joystick.release(Button::UP);
        sleep(Duration::from_millis(500));
        joystick.release(Button::DOWN);
        sleep(Duration::from_millis(500));
        joystick.release(Button::LEFT);
        sleep(Duration::from_millis(500));
        joystick.release(Button::RIGHT);
        sleep(Duration::from_millis(500));
        joystick.release(Button::A);
        sleep(Duration::from_millis(500));
        joystick.release(Button::B);
        sleep(Duration::from_millis(500));
        joystick.release(Button::X);
        sleep(Duration::from_millis(500));
        joystick.release(Button::Y);
        sleep(Duration::from_millis(500));
        joystick.release(Button::LT);
        sleep(Duration::from_millis(500));
        joystick.release(Button::RT);
        sleep(Duration::from_millis(500));
        joystick.release(Button::LT2);
        sleep(Duration::from_millis(500));
        joystick.release(Button::RT2);
        sleep(Duration::from_millis(500));
        joystick.release(Button::SELECT);
        sleep(Duration::from_millis(500));
        joystick.release(Button::START);
        sleep(Duration::from_millis(500));

        Result::Ok(())
    }

    #[test]
    fn test_release_all() -> std::io::Result<()> {
        sleep(Duration::from_millis(2000));
        let mut joystick: Joystick = Joystick::default();
        sleep(Duration::from_millis(500));
        joystick.press(Button::UP);
        joystick.press(Button::DOWN);
        joystick.press(Button::LEFT);
        joystick.press(Button::RIGHT);
        joystick.press(Button::A);
        joystick.press(Button::B);
        joystick.press(Button::X);
        joystick.press(Button::Y);
        joystick.press(Button::LT);
        joystick.press(Button::RT);
        joystick.press(Button::LT2);
        joystick.press(Button::RT2);
        joystick.press(Button::START);
        joystick.press(Button::SELECT);
        sleep(Duration::from_millis(1000));
        joystick.release_all();
        sleep(Duration::from_millis(500));

        Result::Ok(())
    }
}
