use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AbsInfo, AbsoluteAxisCode, AttributeSet, EventType, InputEvent, KeyCode, UinputAbsSetup,
};

use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

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
    name: String,
    keys: AttributeSet<KeyCode>,
    events: Vec<JoystickEvent>,
    instant: Instant,
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
            name: name.to_string(),
            instant: Instant::now(),
            keys,
        }
    }
}

impl Joystick {
    pub fn new(&self) -> Self {
        Joystick::default()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tap_a(&mut self) {
        self.tap(KeyCode::BTN_EAST)
    }

    pub fn tap_b(&mut self) {
        self.tap(KeyCode::BTN_SOUTH)
    }

    pub fn tap_x(&mut self) {
        self.tap(KeyCode::BTN_NORTH)
    }

    pub fn tap_y(&mut self) {
        self.tap(KeyCode::BTN_WEST)
    }

    pub fn tap_lt(&mut self) {
        self.tap(KeyCode::BTN_TL)
    }

    pub fn tap_rt(&mut self) {
        self.tap(KeyCode::BTN_TR)
    }

    pub fn tap_lt2(&mut self) {
        self.tap(KeyCode::BTN_TL2)
    }

    pub fn tap_rt2(&mut self) {
        self.tap(KeyCode::BTN_TR2)
    }

    pub fn tap_select(&mut self) {
        self.tap(KeyCode::BTN_SELECT)
    }

    pub fn tap_start(&mut self) {
        self.tap(KeyCode::BTN_START)
    }

    pub fn tap_dpad_up(&mut self) {
        self.tap(KeyCode::BTN_DPAD_UP)
    }

    pub fn tap_dpad_down(&mut self) {
        self.tap(KeyCode::BTN_DPAD_DOWN)
    }

    pub fn tap_dpad_left(&mut self) {
        self.tap(KeyCode::BTN_DPAD_LEFT)
    }

    pub fn tap_dpad_right(&mut self) {
        self.tap(KeyCode::BTN_DPAD_RIGHT)
    }

    pub fn release_all(&mut self) {
        let mut keys = vec![];
        for key in &self.keys {
            keys.push(InputEvent::new(EventType::KEY.0, key.code(), 0));
        }
        self.device.lock().unwrap().emit(&keys).unwrap();
    }

    pub fn tap(&mut self, button: KeyCode) {
        // send in press and release
        let time = self.instant.elapsed();
        let release_duration = Duration::from_millis(TAP_DURATION);

        self.events.push(JoystickEvent {
            key: button,
            event_type: EventType::KEY,
            duration: time,
            action: KeyAction::Press,
        });
        self.events.push(JoystickEvent {
            key: button,
            event_type: EventType::KEY,
            duration: time + release_duration,
            action: KeyAction::Release,
        });
    }

    pub fn run(&mut self) {
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
}

#[cfg(test)]
mod tests {
    use crate::Joystick;
    use std::thread::sleep;
    use std::time::{Duration, Instant};

    #[test]
    fn test_event_system() -> std::io::Result<()> {
        let mut joystick: Joystick = Joystick::default();
        joystick.tap_a();
        sleep(Duration::from_millis(200));
        joystick.tap_b();
        sleep(Duration::from_millis(200));
        joystick.tap_x();
        sleep(Duration::from_millis(200));
        joystick.tap_y();
        sleep(Duration::from_millis(200));
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
