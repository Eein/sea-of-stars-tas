use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AbsInfo, AbsoluteAxisCode, AbsoluteAxisEvent, AttributeSet, BusType, EventType, InputEvent,
    InputId, KeyCode, UinputAbsSetup,
};
use vec2::clamp;

use log::error;
use std::sync::Arc;
use std::sync::Mutex;

use crate::common::{Button, JoystickBtnInterface, JoystickInterface};

pub struct Joystick {
    device: Arc<Mutex<VirtualDevice>>,
    keys: AttributeSet<KeyCode>,
}

const ABS_MAX: i32 = 512;
const ABS_MIN: i32 = 0;

impl JoystickInterface for Joystick {
    fn release_all(&mut self) {
        self.set_ljoy([0.0, 0.0]);
        self.set_rjoy([0.0, 0.0]);
        let mut keys = vec![];
        for key in &self.keys {
            keys.push(InputEvent::new(EventType::KEY.0, key.code(), 0));
        }
        match self.device.lock().unwrap().emit(&keys) {
            Ok(_) => (),
            Err(e) => error!("Joystick error: {e:?}"),
        }
    }

    fn set_ljoy(&mut self, dir: [f32; 2]) {
        self.set_joy(dir, AbsoluteAxisCode::ABS_X, AbsoluteAxisCode::ABS_Y);
    }

    fn set_rjoy(&mut self, dir: [f32; 2]) {
        self.set_joy(dir, AbsoluteAxisCode::ABS_RX, AbsoluteAxisCode::ABS_RY);
    }
}

impl JoystickBtnInterface<Button> for Joystick {
    fn press(&mut self, button: &Button) {
        let button = Joystick::get_button(button);
        let event = InputEvent::new(EventType::KEY.0, button.code(), 1);

        match self.device.lock().unwrap().emit(&[event]) {
            Ok(_) => (),
            Err(e) => error!("Joystick error: {e:?}"),
        }
    }

    fn release(&mut self, button: &Button) {
        let button = Joystick::get_button(button);
        let event = InputEvent::new(EventType::KEY.0, button.code(), 0);

        match self.device.lock().unwrap().emit(&[event]) {
            Ok(_) => (),
            Err(e) => error!("Joystick error: {e:?}"),
        }
    }
}

impl Joystick {
    fn set_joy(&mut self, dir: [f32; 2], x_code: AbsoluteAxisCode, y_code: AbsoluteAxisCode) {
        let mut clamped_dir = dir;
        clamp(&mut clamped_dir, &[-1.0, -1.0], &[1.0, 1.0]);
        // Convert from range -1..1 to -32768..32767
        // Negative values are down/left, positive are up/right
        let x_code = x_code.0;
        let y_code = y_code.0;
        let abs_x = (clamped_dir[0] * i16::MAX as f32) as i16;
        let abs_y = -(clamped_dir[1] * i16::MAX as f32) as i16;
        
        let n_x = normalize(abs_x.into());
        let n_y = normalize(abs_y.into());

        let x_event = *AbsoluteAxisEvent::new(AbsoluteAxisCode(x_code), n_x.into());
        let y_event = *AbsoluteAxisEvent::new(AbsoluteAxisCode(y_code), n_y.into());

        match self.device.lock().unwrap().emit(&[x_event, y_event]) {
            Ok(_) => (),
            Err(e) => error!("Joystick error: {e:?}"),
        }
    }

    fn get_button(button: &Button) -> KeyCode {
        match button {
            Button::A => KeyCode::BTN_EAST,
            Button::B => KeyCode::BTN_SOUTH,
            Button::X => KeyCode::BTN_NORTH,
            Button::Y => KeyCode::BTN_WEST,
            Button::LT(_) => KeyCode::BTN_TL,
            Button::RT(_) => KeyCode::BTN_TR,
            Button::LB => KeyCode::BTN_TL2,
            Button::RB => KeyCode::BTN_TR2,
            Button::LTHUMB => KeyCode::BTN_THUMB,
            Button::RTHUMB => KeyCode::BTN_THUMB2,
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
        let center = ABS_MAX / 2;
        let abs_setup = AbsInfo::new(center, ABS_MIN, ABS_MAX, 20, 20, 1);
        let abs_x = UinputAbsSetup::new(AbsoluteAxisCode::ABS_X, abs_setup);
        let abs_y = UinputAbsSetup::new(AbsoluteAxisCode::ABS_Y, abs_setup);

        let abs_r_x = UinputAbsSetup::new(AbsoluteAxisCode::ABS_RX, abs_setup);
        let abs_r_y = UinputAbsSetup::new(AbsoluteAxisCode::ABS_RY, abs_setup);

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
        keys.insert(KeyCode::BTN_THUMB);
        keys.insert(KeyCode::BTN_THUMB2);
        keys.insert(KeyCode::BTN_TR);
        keys.insert(KeyCode::BTN_TL);
        keys.insert(KeyCode::BTN_TR2);
        keys.insert(KeyCode::BTN_TL2);

        let input_id = InputId::new(BusType::BUS_VIRTUAL, 1234, 5678, 1);
        let device = VirtualDeviceBuilder::new()
            .unwrap()
            .input_id(input_id)
            .name(name)
            .with_keys(&keys)
            .unwrap()
            .with_absolute_axis(&abs_x)
            .unwrap()
            .with_absolute_axis(&abs_y)
            .unwrap()
            .with_absolute_axis(&abs_r_x)
            .unwrap()
            .with_absolute_axis(&abs_r_y)
            .unwrap()
            .build()
            .unwrap();

        Joystick {
            device: Arc::new(Mutex::new(device)),
            keys,
        }
    }
}

// Normalizing a value for linux should normalize against the abs values
// TODO(eein): you should have removed this message and taken the magic values to const
pub fn normalize(value: f32) -> u16 {
    let val = ((value - -1.0) / (1.0 - -1.0)) * ABS_MAX as f32;
    val.round() as u16
}

#[cfg(test)]
mod tests {
    use crate::joystick::normalize;
    #[test]
    fn test_normalize() -> std::io::Result<()> {
        assert_eq!(normalize(-1.0), 0);
        assert_eq!(normalize(1.0), 512);
        assert_eq!(normalize(0.0), 256);
        Ok(())
    }
}
