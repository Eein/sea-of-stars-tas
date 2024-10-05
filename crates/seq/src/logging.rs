use log::info;
use std::fmt::Display;

use crate::Node;

pub struct SeqLog {
    text: String,
}

impl SeqLog {
    pub fn create(text: &str) -> Box<Self> {
        Box::new(SeqLog {
            text: text.to_owned(),
        })
    }
}

impl Display for SeqLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl<State> Node<State> for SeqLog {
    fn enter(&mut self, _state: &mut State) {
        info!("SeqLog: {}", self);
    }
}
