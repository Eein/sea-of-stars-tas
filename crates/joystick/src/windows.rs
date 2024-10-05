use log::error;
use vec2::clamp;
use vigem_client::{Client, TargetId, XButtons, XGamepad, Xbox360Wired};

use crate::common::{Button, JoystickInterface};

pub struct Joystick {
    device: Xbox360Wired<Client>, // may need to be Arc<Mutex>>
    gamepad: vigem_client::XGamepad,
}

impl Default for Joystick {
    fn default() -> Self {
        let client = Client::connect().unwrap();
        let id = TargetId::XBOX360_WIRED;
        let mut device = Xbox360Wired::new(client, id);

        device.plugin().unwrap();

        let gamepad = XGamepad {
            ..Default::default()
        };

        Joystick { device, gamepad }
    }
}

impl Joystick {
    fn map_button(button: Button) -> u16 {
        // TODO: Note, the left/right triggers are handled differently in XINPUT, they are u8 values.
        match button {
            Button::A => XButtons::A,
            Button::B => XButtons::B,
            Button::X => XButtons::X,
            Button::Y => XButtons::Y,
            Button::LB => XButtons::LB,
            Button::LTHUMB => XButtons::LTHUMB,
            Button::RB => XButtons::RB,
            Button::RTHUMB => XButtons::RTHUMB,
            Button::SELECT => XButtons::BACK,
            Button::START => XButtons::START,
            Button::UP => XButtons::UP,
            Button::DOWN => XButtons::DOWN,
            Button::LEFT => XButtons::LEFT,
            Button::RIGHT => XButtons::RIGHT,
            _ => 0,
        }
    }
}

impl JoystickInterface for Joystick {
    fn release_all(&mut self) {
        self.set_ljoy([0.0, 0.0]);
        self.set_rjoy([0.0, 0.0]);
        self.gamepad.left_trigger = 0;
        self.gamepad.right_trigger = 0;
        self.gamepad.buttons = vigem_client::XButtons!();
        match self.device.update(&self.gamepad) {
            Ok(_) => (),
            Err(e) => error!("Joystick error: {e:?}"),
        }
    }
    fn press(&mut self, button: Button) {
        match button {
            Button::LT(val) => self.gamepad.left_trigger = val,
            Button::RT(val) => self.gamepad.right_trigger = val,
            _ => {
                let code = Joystick::map_button(button);
                self.gamepad.buttons.raw |= code;
            }
        }
        match self.device.update(&self.gamepad) {
            Ok(_) => (),
            Err(e) => error!("Joystick error: {e:?}"),
        }
    }
    fn release(&mut self, button: Button) {
        match button {
            Button::LT(_) => self.gamepad.left_trigger = 0,
            Button::RT(_) => self.gamepad.right_trigger = 0,
            _ => {
                let code = Joystick::map_button(button);
                self.gamepad.buttons.raw &= !code;
            }
        }
        match self.device.update(&self.gamepad) {
            Ok(_) => (),
            Err(e) => error!("Joystick error: {e:?}"),
        }
    }
    fn set_ljoy(&mut self, dir: [f32; 2]) {
        let mut clamped_dir = dir;
        clamp(&mut clamped_dir, &[-1.0, -1.0], &[1.0, 1.0]);
        // Convert from range -1..1 to -32768..32767
        // Negative values are down/left, positive are up/right
        self.gamepad.thumb_lx = (clamped_dir[0] * i16::MAX as f32) as i16;
        self.gamepad.thumb_ly = (clamped_dir[1] * i16::MAX as f32) as i16;
        match self.device.update(&self.gamepad) {
            Ok(_) => (),
            Err(e) => error!("Joystick error: {e:?}"),
        }
    }
    fn set_rjoy(&mut self, dir: [f32; 2]) {
        let mut clamped_dir = dir;
        clamp(&mut clamped_dir, &[-1.0, -1.0], &[1.0, 1.0]);
        // Convert from range -1..1 to -32768..32767
        // Negative values are down/left, positive are up/right
        self.gamepad.thumb_rx = (clamped_dir[0] * i16::MAX as f32) as i16;
        self.gamepad.thumb_ry = (clamped_dir[1] * i16::MAX as f32) as i16;
        match self.device.update(&self.gamepad) {
            Ok(_) => (),
            Err(e) => error!("Joystick error: {e:?}"),
        }
    }
}
