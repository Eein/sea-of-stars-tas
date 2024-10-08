use joystick::prelude::*;

// For now, define these statically
#[derive(Clone)]
pub enum SosAction {
    Confirm,
    Bracelet,
    Graplou,
    Cancel,
    Menu,
    Start,
    Pause,
    Turbo,
    Boost,
    ShiftLeft,
    ShiftRight,
    TimeInc,
    TimeDec,
    MenuUp,
    MenuDown,
    MenuLeft,
    MenuRight,
}

impl From<SosAction> for Button {
    fn from(value: SosAction) -> Self {
        match value {
            SosAction::Confirm => Button::A,
            SosAction::Cancel => Button::B,
            SosAction::Graplou => Button::B,
            SosAction::Bracelet => Button::X,
            SosAction::Menu => Button::Y,
            SosAction::Start => Button::START,
            SosAction::Pause => Button::START,
            SosAction::Turbo => Button::RB,
            SosAction::Boost => Button::RT(255),
            SosAction::ShiftLeft => Button::LB,
            SosAction::ShiftRight => Button::RB,
            SosAction::TimeInc => Button::RT(255),
            SosAction::TimeDec => Button::LT(255),
            SosAction::MenuUp => Button::UP,
            SosAction::MenuDown => Button::DOWN,
            SosAction::MenuLeft => Button::LEFT,
            SosAction::MenuRight => Button::RIGHT,
        }
    }
}
