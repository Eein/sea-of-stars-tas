use crate::Node;

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

impl<State, Event> Node<State, Event> for SeqWait {
    fn print(&self) -> String {
        format!("Wait({}): {}/{}", self.name, self.timer, self.timeout)
    }
    // Execute the selected path until it terminates
    fn execute(&mut self, _state: &mut State, delta: f64) -> bool {
        self.timer += delta;
        self.timer >= self.timeout
    }
}
