use crate::common::{Button, JoystickInterface};
use crate::joystick::Joystick;
use std::collections::HashMap;
use std::hash::Hash;

pub struct GenericJoystick<KeyType: PartialEq + Eq + Hash> {
    joystick: Joystick,
    mappings: HashMap<KeyType, Button>,
}

impl<KeyType: PartialEq + Eq + Hash> Default for GenericJoystick<KeyType> {
    fn default() -> Self {
        GenericJoystick::new(HashMap::new())
    }
}

impl<KeyType: PartialEq + Eq + Hash> GenericJoystick<KeyType> {
    pub fn new(mappings: HashMap<KeyType, Button>) -> Self {
        Self {
            joystick: Joystick::default(),
            mappings,
        }
    }

    pub fn release_all(&mut self) {
        self.joystick.release_all();
    }
    pub fn press(&mut self, key: &KeyType) {
        if let Some(button) = self.mappings.get(key) {
            self.joystick.press(button);
        } // Note: could panic! here on None?
    }
    pub fn release(&mut self, key: &KeyType) {
        if let Some(button) = self.mappings.get(key) {
            self.joystick.release(button);
        } // Note: could panic! here on None?
    }
    // [x, y], where the values range from -1 to 1
    pub fn set_ljoy(&mut self, dir: [f32; 2]) {
        self.joystick.set_ljoy(dir);
    }

    pub fn set_rjoy(&mut self, dir: [f32; 2]) {
        self.joystick.set_rjoy(dir);
    }
}

// To visually run these tests, use:
// `cargo test -- --test-threads 1`
// then focus https://hardwaretester.com/gamepad
#[cfg(test)]
mod tests {
    // Used in the test
    use std::thread::sleep;
    use std::time::Duration;

    // Include the library
    use crate::prelude::*;

    // This test doubles as an example on how to use the joystick library with custom actions
    #[derive(PartialEq, Eq, Hash)]
    enum Action {
        Confirm,
        Cancel,
        Fire,
        Jump,
        Aim,
    }

    #[test]
    fn test_mappings() -> std::io::Result<()> {
        sleep(Duration::from_millis(2000));
        let mut gamepad = GenericJoystick::new(HashMap::from([
            (Action::Confirm, Button::A),
            (Action::Cancel, Button::B),
            (Action::Fire, Button::X),
            (Action::Jump, Button::Y),
            (Action::Aim, Button::RT(255)),
        ]));
        sleep(Duration::from_millis(500));
        for _ in 0..3 {
            gamepad.press(&Action::Confirm);
            sleep(Duration::from_millis(250));
            gamepad.release(&Action::Confirm);
            sleep(Duration::from_millis(250));
        }
        for _ in 0..3 {
            gamepad.press(&Action::Cancel);
            sleep(Duration::from_millis(250));
            gamepad.release(&Action::Cancel);
            sleep(Duration::from_millis(250));
        }
        for _ in 0..3 {
            gamepad.press(&Action::Fire);
            sleep(Duration::from_millis(250));
            gamepad.release(&Action::Fire);
            sleep(Duration::from_millis(250));
        }
        for _ in 0..3 {
            gamepad.press(&Action::Jump);
            sleep(Duration::from_millis(250));
            gamepad.release(&Action::Jump);
            sleep(Duration::from_millis(250));
        }
        for _ in 0..3 {
            gamepad.press(&Action::Aim);
            sleep(Duration::from_millis(250));
            gamepad.release(&Action::Aim);
            sleep(Duration::from_millis(250));
        }
        sleep(Duration::from_millis(250));
        gamepad.press(&Action::Confirm);
        gamepad.press(&Action::Cancel);
        gamepad.press(&Action::Fire);
        gamepad.press(&Action::Jump);
        gamepad.press(&Action::Aim);
        sleep(Duration::from_millis(1000));
        gamepad.release_all();

        let speed = 1000 / 360; // One rotation in 1s, CCW from -> to ->
        for _ in 0..4 {
            for deg in 0..360 {
                let rad = f32::to_radians(deg as f32);
                let ldir: [f32; 2] = [f32::cos(rad), f32::sin(rad)];
                gamepad.set_ljoy(ldir);
                let rdir: [f32; 2] = [f32::sin(rad), f32::cos(rad)];
                gamepad.set_rjoy(rdir);
                sleep(Duration::from_millis(speed));
            }
        }

        Result::Ok(())
    }
}
