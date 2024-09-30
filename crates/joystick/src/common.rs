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
    RIGHT,
}

pub trait JoystickInterface {
    fn release_all(&mut self);
    fn press(&mut self, button: Button);
    fn release(&mut self, button: Button);
    // [x, y], where the values range from -1 to 1
    fn set_joy(&mut self, dir: [f32; 2]);
}
