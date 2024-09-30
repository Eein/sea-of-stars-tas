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
            buttons: vigem_client::XButtons!(
                UP | DOWN | LEFT | RIGHT | LTHUMB | RTHUMB | LB | RB | GUIDE | A | B | X | Y
            ),
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
            Button::LT => XButtons::LB,
            Button::LT2 => XButtons::LTHUMB,
            Button::RT => XButtons::RB,
            Button::RT2 => XButtons::RTHUMB,
            Button::SELECT => XButtons::BACK,
            Button::START => XButtons::START,
            Button::UP => XButtons::UP,
            Button::DOWN => XButtons::DOWN,
            Button::LEFT => XButtons::LEFT,
            Button::RIGHT => XButtons::RIGHT,
        }
    }
}

impl JoystickInterface for Joystick {
    fn release_all(&mut self) {
        self.set_joy([0.0, 0.0]);
        self.gamepad.buttons = vigem_client::XButtons!();
        match self.device.update(&self.gamepad) {
            Ok(_) => (),
            Err(e) => println!("Joystick error: {e:?}"),
        }
    }
    fn press(&mut self, button: Button) {
        let code = Joystick::map_button(button);
        self.gamepad.buttons.raw |= code;
        match self.device.update(&self.gamepad) {
            Ok(_) => (),
            Err(e) => println!("Joystick error: {e:?}"),
        }
    }
    fn release(&mut self, button: Button) {
        let code = Joystick::map_button(button);
        self.gamepad.buttons.raw &= !code;
        match self.device.update(&self.gamepad) {
            Ok(_) => (),
            Err(e) => println!("Joystick error: {e:?}"),
        }
    }
    fn set_joy(&mut self, dir: [f32; 2]) {
        let mut clamped_dir = dir;
        clamp(&mut clamped_dir, &[-1.0, -1.0], &[1.0, 1.0]);
        // Convert from range -1..1 to -32768..32767
        // Negative values are down/left, positive are up/right
        self.gamepad.thumb_lx = (clamped_dir[0] * i16::MAX as f32) as i16;
        self.gamepad.thumb_ly = (clamped_dir[1] * i16::MAX as f32) as i16;
        match self.device.update(&self.gamepad) {
            Ok(_) => (),
            Err(e) => println!("Joystick error: {e:?}"),
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
    fn test_buttons() -> std::io::Result<()> {
        sleep(Duration::from_millis(2000));
        let mut joystick: Joystick = Joystick::default();
        joystick.release_all();
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
