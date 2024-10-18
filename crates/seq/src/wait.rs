use crate::Node;
use std::fmt::Display;

pub struct SeqWait {
    name: String,
    timer: f64,
    timeout: f64,
}

impl SeqWait {
    pub fn create(name: &str, timeout: f64) -> Box<Self> {
        Box::new(SeqWait {
            name: name.to_owned(),
            timer: 0.0,
            timeout,
        })
    }
}

impl Display for SeqWait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}/{}", self.name, self.timer, self.timeout)
    }
}

impl<State, Event> Node<State, Event> for SeqWait {
    // Execute the selected path until it terminates
    fn execute(&mut self, _state: &mut State, delta: f64) -> bool {
        self.timer += delta;
        self.timer >= self.timeout
    }
}
