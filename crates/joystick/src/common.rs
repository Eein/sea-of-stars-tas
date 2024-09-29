pub static TAP_DURATION: u64 = 50;

pub enum KeyAction {
    Press,
    Release,
}

pub enum Button {
    A,
    B,
    X,
    Y,
    LT,
    RT,
    LT2,
    RT2,
    SELECT,
    START,
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub trait JoystickInterface {
    fn release_all(&mut self);
    fn press(&mut self, button: Button);
    fn release(&mut self, button: Button);
}
