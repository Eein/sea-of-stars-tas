use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AbsInfo, AbsoluteAxisCode, AttributeSet, EventType, InputEvent, KeyCode, UinputAbsSetup,
};

use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::common::JoystickInterface;
static TAP_DURATION: u64 = 50;

enum KeyAction {
    Press,
    Release,
}

struct JoystickEvent {
    key: KeyCode,
    event_type: EventType,
    duration: Duration,
    action: KeyAction,
}

pub struct Joystick {
    device: Arc<Mutex<VirtualDevice>>,
    keys: AttributeSet<KeyCode>,
    events: Vec<JoystickEvent>,
    instant: Instant,
}

impl JoystickInterface for Joystick {
    fn press_a(&mut self) {
        self.press(KeyCode::BTN_EAST)
    }

    fn press_b(&mut self) {
        self.press(KeyCode::BTN_SOUTH)
    }

    fn press_x(&mut self) {
        self.press(KeyCode::BTN_NORTH)
    }

    fn press_y(&mut self) {
        self.press(KeyCode::BTN_WEST)
    }

    fn press_lt(&mut self) {
        self.press(KeyCode::BTN_TL)
    }

    fn press_rt(&mut self) {
        self.press(KeyCode::BTN_TR)
    }

    fn press_lt2(&mut self) {
        self.press(KeyCode::BTN_TL2)
    }

    fn press_rt2(&mut self) {
        self.press(KeyCode::BTN_TR2)
    }

    fn press_select(&mut self) {
        self.press(KeyCode::BTN_SELECT)
    }

    fn press_start(&mut self) {
        self.press(KeyCode::BTN_START)
    }

    fn press_dpad_up(&mut self) {
        self.press(KeyCode::BTN_DPAD_UP)
    }

    fn press_dpad_down(&mut self) {
        self.press(KeyCode::BTN_DPAD_DOWN)
    }

    fn press_dpad_left(&mut self) {
        self.press(KeyCode::BTN_DPAD_LEFT)
    }

    fn press_dpad_right(&mut self) {
        self.press(KeyCode::BTN_DPAD_RIGHT)
    }

    fn release_a(&mut self) {
        self.release(KeyCode::BTN_EAST)
    }

    fn release_b(&mut self) {
        self.release(KeyCode::BTN_SOUTH)
    }

    fn release_x(&mut self) {
        self.release(KeyCode::BTN_NORTH)
    }

    fn release_y(&mut self) {
        self.release(KeyCode::BTN_WEST)
    }

    fn release_lt(&mut self) {
        self.release(KeyCode::BTN_TL)
    }

    fn release_rt(&mut self) {
        self.release(KeyCode::BTN_TR)
    }

    fn release_lt2(&mut self) {
        self.release(KeyCode::BTN_TL2)
    }

    fn release_rt2(&mut self) {
        self.release(KeyCode::BTN_TR2)
    }

    fn release_select(&mut self) {
        self.release(KeyCode::BTN_SELECT)
    }

    fn release_start(&mut self) {
        self.release(KeyCode::BTN_START)
    }

    fn release_dpad_up(&mut self) {
        self.release(KeyCode::BTN_DPAD_UP)
    }

    fn release_dpad_down(&mut self) {
        self.release(KeyCode::BTN_DPAD_DOWN)
    }

    fn release_dpad_left(&mut self) {
        self.release(KeyCode::BTN_DPAD_LEFT)
    }

    fn release_dpad_right(&mut self) {
        self.release(KeyCode::BTN_DPAD_RIGHT)
    }

    fn tap_a(&mut self) {
        self.tap(KeyCode::BTN_EAST)
    }

    fn tap_b(&mut self) {
        self.tap(KeyCode::BTN_SOUTH)
    }

    fn tap_x(&mut self) {
        self.tap(KeyCode::BTN_NORTH)
    }

    fn tap_y(&mut self) {
        self.tap(KeyCode::BTN_WEST)
    }

    fn tap_lt(&mut self) {
        self.tap(KeyCode::BTN_TL)
    }

    fn tap_rt(&mut self) {
        self.tap(KeyCode::BTN_TR)
    }

    fn tap_lt2(&mut self) {
        self.tap(KeyCode::BTN_TL2)
    }

    fn tap_rt2(&mut self) {
        self.tap(KeyCode::BTN_TR2)
    }

    fn tap_select(&mut self) {
        self.tap(KeyCode::BTN_SELECT)
    }

    fn tap_start(&mut self) {
        self.tap(KeyCode::BTN_START)
    }

    fn tap_dpad_up(&mut self) {
        self.tap(KeyCode::BTN_DPAD_UP)
    }

    fn tap_dpad_down(&mut self) {
        self.tap(KeyCode::BTN_DPAD_DOWN)
    }

    fn tap_dpad_left(&mut self) {
        self.tap(KeyCode::BTN_DPAD_LEFT)
    }

    fn tap_dpad_right(&mut self) {
        self.tap(KeyCode::BTN_DPAD_RIGHT)
    }

    fn release_all(&mut self) {
        let mut keys = vec![];
        for key in &self.keys {
            keys.push(InputEvent::new(EventType::KEY.0, key.code(), 0));
        }
        self.device.lock().unwrap().emit(&keys).unwrap();
    }

    fn run(&mut self) {
        if !self.events.is_empty() {
            let timer_time = self.instant.elapsed();

            for event in &self.events {
                if event.duration <= timer_time {
                    let action = match event.action {
                        KeyAction::Release => 0,
                        KeyAction::Press => 1,
                    };
                    let event = InputEvent::new(event.event_type.0, event.key.code(), action);

                    self.device.lock().unwrap().emit(&[event]).unwrap();
                }
            }
            self.events.retain(|event| event.duration > timer_time);
        } else {
            self.instant = Instant::now();
        }
    }

    fn press(&mut self, button: KeyCode) {
        let time = self.instant.elapsed();

        self.events.push(JoystickEvent {
            key: button,
            event_type: EventType::KEY,
            duration: time,
            action: KeyAction::Press,
        });
    }

    fn release(&mut self, button: KeyCode) {
        let time = self.instant.elapsed();

        self.events.push(JoystickEvent {
            key: button,
            event_type: EventType::KEY,
            duration: time,
            action: KeyAction::Release,
        });
    }

    fn release_later(&mut self, button: KeyCode, duration: Duration) {
        let time = self.instant.elapsed();

        self.events.push(JoystickEvent {
            key: button,
            event_type: EventType::KEY,
            duration: time + duration,
            action: KeyAction::Release,
        });
    }

    fn tap(&mut self, button: KeyCode) {
        // send in press and release
        let release_duration = Duration::from_millis(TAP_DURATION);
        self.press(button);
        self.release_later(button, release_duration);
    }
}

impl Default for Joystick {
    fn default() -> Self {
        let name = "Future TAS Joystick Linux";
        let abs_setup = AbsInfo::new(256, 0, 512, 20, 20, 1);
        let abs_x = UinputAbsSetup::new(AbsoluteAxisCode::ABS_X, abs_setup);
        let abs_y = UinputAbsSetup::new(AbsoluteAxisCode::ABS_Y, abs_setup);
        let mut keys = AttributeSet::<KeyCode>::new();

        keys.insert(KeyCode::BTN_DPAD_DOWN);
        keys.insert(KeyCode::BTN_DPAD_LEFT);
        keys.insert(KeyCode::BTN_DPAD_RIGHT);
        keys.insert(KeyCode::BTN_DPAD_UP);
        keys.insert(KeyCode::BTN_NORTH);
        keys.insert(KeyCode::BTN_SOUTH);
        keys.insert(KeyCode::BTN_EAST);
        keys.insert(KeyCode::BTN_WEST);
        keys.insert(KeyCode::BTN_START);
        keys.insert(KeyCode::BTN_SELECT);
        keys.insert(KeyCode::BTN_TR);
        keys.insert(KeyCode::BTN_TL);
        keys.insert(KeyCode::BTN_TR2);
        keys.insert(KeyCode::BTN_TL2);

        let device = VirtualDeviceBuilder::new()
            .unwrap()
            .name(name)
            .with_keys(&keys)
            .unwrap()
            .with_absolute_axis(&abs_x)
            .unwrap()
            .with_absolute_axis(&abs_y)
            .unwrap()
            .build()
            .unwrap();

        Joystick {
            events: vec![],
            device: Arc::new(Mutex::new(device)),
            instant: Instant::now(),
            keys,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::JoystickInterface;
    use crate::joystick::Joystick;
    use std::thread::sleep;
    use std::time::{Duration, Instant};

    #[test]
    fn test_event_system() -> std::io::Result<()> {
        sleep(Duration::from_millis(1000));
        let mut joystick: Joystick = Joystick::default();
        joystick.tap_dpad_up();
        sleep(Duration::from_millis(500));
        joystick.tap_dpad_down();
        sleep(Duration::from_millis(500));
        joystick.tap_dpad_left();
        sleep(Duration::from_millis(500));
        joystick.tap_dpad_right();
        sleep(Duration::from_millis(500));
        joystick.tap_a();
        sleep(Duration::from_millis(500));
        joystick.tap_b();
        sleep(Duration::from_millis(500));
        joystick.tap_x();
        sleep(Duration::from_millis(500));
        joystick.tap_y();
        sleep(Duration::from_millis(500));
        joystick.tap_lt();
        sleep(Duration::from_millis(500));
        joystick.tap_rt();
        sleep(Duration::from_millis(500));
        joystick.tap_lt2();
        sleep(Duration::from_millis(500));
        joystick.tap_rt2();
        sleep(Duration::from_millis(500));
        joystick.instant = Instant::now();
        assert!(!joystick.events.is_empty());

        while !joystick.events.is_empty() {
            joystick.run();
        }

        assert!(joystick.events.is_empty());
        Result::Ok(())
    }

    #[test]
    fn ensure_instants_reset() -> std::io::Result<()> {
        let mut joystick: Joystick = Joystick::default();
        joystick.tap_a();
        assert!(!joystick.events.is_empty());

        while !joystick.events.is_empty() {
            joystick.run();
        }

        assert!(joystick.instant < Instant::now() + Duration::from_secs(1));
        assert!(joystick.events.is_empty());
        Result::Ok(())
    }
}
