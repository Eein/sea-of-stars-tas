use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AbsInfo, AbsoluteAxisCode, AbsoluteAxisEvent, AttributeSet, EventType, InputEvent, KeyCode,
    UinputAbsSetup,
};
use vec2::clamp;

use std::sync::Arc;
use std::sync::Mutex;

use crate::common::{Button, JoystickInterface};

pub struct Joystick {
    device: Arc<Mutex<VirtualDevice>>,
    keys: AttributeSet<KeyCode>,
}

impl JoystickInterface for Joystick {
    fn release_all(&mut self) {
        self.set_joy([0.0, 0.0]);
        let mut keys = vec![];
        for key in &self.keys {
            keys.push(InputEvent::new(EventType::KEY.0, key.code(), 0));
        }
        match self.device.lock().unwrap().emit(&keys) {
            Ok(_) => (),
            Err(e) => println!("Joystick error: {e:?}"),
        }
    }

    fn press(&mut self, button: Button) {
        let button = Joystick::get_button(button);
        let event = InputEvent::new(EventType::KEY.0, button.code(), 1);

        match self.device.lock().unwrap().emit(&[event]) {
            Ok(_) => (),
            Err(e) => println!("Joystick error: {e:?}"),
        }
    }

    fn release(&mut self, button: Button) {
        let button = Joystick::get_button(button);
        let event = InputEvent::new(EventType::KEY.0, button.code(), 0);

        match self.device.lock().unwrap().emit(&[event]) {
            Ok(_) => (),
            Err(e) => println!("Joystick error: {e:?}"),
        }
    }

    fn set_joy(&mut self, dir: [f32; 2]) {
        let mut clamped_dir = dir;
        clamp(&mut clamped_dir, &[-1.0, -1.0], &[1.0, 1.0]);
        // Convert from range -1..1 to -32768..32767
        // Negative values are down/left, positive are up/right
        let x_code = AbsoluteAxisCode::ABS_X.0;
        let y_code = AbsoluteAxisCode::ABS_Y.0;
        let abs_x = (clamped_dir[0] * i16::MAX as f32) as i16;
        let abs_y = (clamped_dir[1] * i16::MAX as f32) as i16;

        let x_event = *AbsoluteAxisEvent::new(AbsoluteAxisCode(x_code), abs_x.into());
        let y_event = *AbsoluteAxisEvent::new(AbsoluteAxisCode(y_code), abs_y.into());

        match self.device.lock().unwrap().emit(&[x_event, y_event]) {
            Ok(_) => (),
            Err(e) => println!("Joystick error: {e:?}"),
        }
    }
}

impl Joystick {
    fn get_button(button: Button) -> KeyCode {
        match button {
            Button::A => KeyCode::BTN_EAST,
            Button::B => KeyCode::BTN_SOUTH,
            Button::X => KeyCode::BTN_NORTH,
            Button::Y => KeyCode::BTN_WEST,
            Button::LT => KeyCode::BTN_TL,
            Button::RT => KeyCode::BTN_TR,
            Button::LT2 => KeyCode::BTN_TL2,
            Button::RT2 => KeyCode::BTN_TR2,
            Button::SELECT => KeyCode::BTN_SELECT,
            Button::START => KeyCode::BTN_START,
            Button::UP => KeyCode::BTN_DPAD_UP,
            Button::DOWN => KeyCode::BTN_DPAD_DOWN,
            Button::LEFT => KeyCode::BTN_DPAD_LEFT,
            Button::RIGHT => KeyCode::BTN_DPAD_RIGHT,
        }
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
            device: Arc::new(Mutex::new(device)),
            keys,
        }
    }
}

// To visually run these tests, use:
// `cargo test -- --test-threads 1`
// then focus https://hardwaretester.com/gamepad
#[cfg(test)]
mod tests {
    use crate::common::{Button, JoystickInterface};
    use crate::joystick::Joystick;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_joystick() -> std::io::Result<()> {
        sleep(Duration::from_millis(2000));
        let speed = 1000 / 360; // One rotation in 1s, CCW from -> to ->
        let mut joystick: Joystick = Joystick::default();
        for _ in 0..4 {
            for deg in 0..360 {
                let rad = f32::to_radians(deg as f32);
                let dir: [f32; 2] = [f32::cos(rad), f32::sin(rad)];
                joystick.set_joy(dir);
                sleep(Duration::from_millis(speed));
            }
        }
        Result::Ok(())
    }

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
