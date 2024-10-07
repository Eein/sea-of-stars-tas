pub enum Button {
    A,
    B,
    X,
    Y,
    LT(u8),
    RT(u8),
    LB,
    RB,
    LTHUMB,
    RTHUMB,
    SELECT,
    START,
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub trait JoystickInterface {
    fn release_all(&mut self);
    fn press(&mut self, button: &Button);
    fn release(&mut self, button: &Button);
    // [x, y], where the values range from -1 to 1
    fn set_ljoy(&mut self, dir: [f32; 2]);
    fn set_rjoy(&mut self, dir: [f32; 2]);
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
                let ldir: [f32; 2] = [f32::cos(rad), f32::sin(rad)];
                joystick.set_ljoy(ldir);
                let rdir: [f32; 2] = [f32::sin(rad), f32::cos(rad)];
                joystick.set_rjoy(rdir);
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
        joystick.press(&Button::UP);
        sleep(Duration::from_millis(500));
        joystick.press(&Button::DOWN);
        sleep(Duration::from_millis(500));
        joystick.press(&Button::LEFT);
        sleep(Duration::from_millis(500));
        joystick.press(&Button::RIGHT);
        sleep(Duration::from_millis(500));
        joystick.press(&Button::A);
        sleep(Duration::from_millis(500));
        joystick.press(&Button::B);
        sleep(Duration::from_millis(500));
        joystick.press(&Button::X);
        sleep(Duration::from_millis(500));
        joystick.press(&Button::Y);
        sleep(Duration::from_millis(500));
        joystick.press(&Button::LT(255));
        sleep(Duration::from_millis(500));
        joystick.press(&Button::RT(255));
        sleep(Duration::from_millis(500));
        joystick.press(&Button::LB);
        sleep(Duration::from_millis(500));
        joystick.press(&Button::RB);
        sleep(Duration::from_millis(500));
        joystick.press(&Button::LTHUMB);
        sleep(Duration::from_millis(500));
        joystick.press(&Button::RTHUMB);
        sleep(Duration::from_millis(500));
        joystick.press(&Button::SELECT);
        sleep(Duration::from_millis(500));
        joystick.press(&Button::START);
        sleep(Duration::from_millis(500));

        joystick.release(&Button::UP);
        sleep(Duration::from_millis(500));
        joystick.release(&Button::DOWN);
        sleep(Duration::from_millis(500));
        joystick.release(&Button::LEFT);
        sleep(Duration::from_millis(500));
        joystick.release(&Button::RIGHT);
        sleep(Duration::from_millis(500));
        joystick.release(&Button::A);
        sleep(Duration::from_millis(500));
        joystick.release(&Button::B);
        sleep(Duration::from_millis(500));
        joystick.release(&Button::X);
        sleep(Duration::from_millis(500));
        joystick.release(&Button::Y);
        sleep(Duration::from_millis(500));
        joystick.release(&Button::LT(0));
        sleep(Duration::from_millis(500));
        joystick.release(&Button::RT(0));
        sleep(Duration::from_millis(500));
        joystick.release(&Button::LB);
        sleep(Duration::from_millis(500));
        joystick.release(&Button::RB);
        sleep(Duration::from_millis(500));
        joystick.release(&Button::LTHUMB);
        sleep(Duration::from_millis(500));
        joystick.release(&Button::RTHUMB);
        sleep(Duration::from_millis(500));
        joystick.release(&Button::SELECT);
        sleep(Duration::from_millis(500));
        joystick.release(&Button::START);
        sleep(Duration::from_millis(500));

        Result::Ok(())
    }

    #[test]
    fn test_release_all() -> std::io::Result<()> {
        sleep(Duration::from_millis(2000));
        let mut joystick: Joystick = Joystick::default();
        sleep(Duration::from_millis(500));
        joystick.press(&Button::UP);
        joystick.press(&Button::DOWN);
        joystick.press(&Button::LEFT);
        joystick.press(&Button::RIGHT);
        joystick.press(&Button::A);
        joystick.press(&Button::B);
        joystick.press(&Button::X);
        joystick.press(&Button::Y);
        joystick.press(&Button::LT(255));
        joystick.press(&Button::RT(255));
        joystick.press(&Button::LB);
        joystick.press(&Button::RB);
        joystick.press(&Button::START);
        joystick.press(&Button::SELECT);
        sleep(Duration::from_millis(1000));
        joystick.release_all();
        sleep(Duration::from_millis(500));

        Result::Ok(())
    }
}
