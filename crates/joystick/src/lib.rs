/* Usage:
use joystick::prelude::*;

#[derive(PartialEq, Eq, Hash)]
enum Action {
    Confirm,
    Cancel,
    Fire,
    Jump,
    Aim,
}

let mut gamepad = GenericJoystick::new(HashMap::from([
    (Action::Confirm, Button::A),
    (Action::Cancel, Button::B),
    (Action::Fire, Button::X),
    (Action::Jump, Button::Y),
    (Action::Aim, Button::RT(255)),
]));

gamepad.press(&Action::Confirm);
gamepad.release(&Action::Fire);
gamepad.release_all();
gamepad.set_ljoy([1.0, 1.0]);
gamepad.set_rjoy([-1.0, -1.0]);
*/

pub mod prelude {
    pub use crate::common::Button;
    pub use crate::mapping::GenericJoystick;
    pub use std::collections::HashMap;
}

// The modules can also be individually included
pub mod common;
pub mod mapping;

#[cfg(target_os = "linux")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
pub mod joystick;
#[cfg(target_os = "windows")]
#[cfg_attr(target_os = "windows", path = "windows.rs")]
pub mod joystick;
