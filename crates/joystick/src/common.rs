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
    fn press(&mut self, button: Button);
    fn release(&mut self, button: Button);
    // [x, y], where the values range from -1 to 1
    fn set_ljoy(&mut self, dir: [f32; 2]);
    fn set_rjoy(&mut self, dir: [f32; 2]);
}
