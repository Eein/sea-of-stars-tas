use std::fmt::Display;

pub trait Node {
    fn execute(&mut self, _delta: f64) -> bool {
        true
    }
    fn advance_to_checkpoint(&mut self, _checkpoint: &str) -> bool {
        false
    }
}

pub struct SeqLog {
    text: String,
    // TODO: More log info, like log level?
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

impl Node for SeqLog {
    fn execute(&mut self, _delta: f64) -> bool {
        // TODO: better logging lib?
        println!("SeqLog: {}", self);
        true
    }
}

#[derive(Default)]
pub struct SeqList {
    name: String,
    children: Vec<Box<dyn Node>>,
    step: usize,
}

impl SeqList {
    pub fn create(name: &str, children: Vec<Box<dyn Node>>) -> Box<Self> {
        Box::new(SeqList {
            name: name.to_owned(),
            children,
            ..Default::default()
        })
    }
}

impl Display for SeqList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({}/{})",
            self.name,
            self.step + 1,
            self.children.len()
        )
    }
}

impl Node for SeqList {
    fn execute(&mut self, delta: f64) -> bool {
        if self.step >= self.children.len() {
            true
        } else {
            if self.children[self.step].execute(delta) {
                self.step = self.step + 1;
            }
            false
        }
    }

    fn advance_to_checkpoint(&mut self, checkpoint: &str) -> bool {
        loop {
            if self.step >= self.children.len() {
                return false;
            }
            if self.children[self.step].advance_to_checkpoint(checkpoint) {
                return true;
            } else {
                self.step = self.step + 1;
            }
        }
    }
}

#[derive(Default)]
pub struct SeqCheckpoint {
    pub checkpoint_name: String,
}

impl SeqCheckpoint {
    pub fn create(name: &str) -> Box<Self> {
        Box::new(SeqCheckpoint {
            checkpoint_name: name.to_owned(),
        })
    }
}

impl Node for SeqCheckpoint {
    fn advance_to_checkpoint(&mut self, checkpoint: &str) -> bool {
        self.checkpoint_name == checkpoint
    }
}

pub struct Sequencer {
    root: Box<dyn Node>,
}

impl Sequencer {
    pub fn create(root: Box<dyn Node>) -> Self {
        Sequencer { root }
    }
}

impl Sequencer {
    pub fn run(&mut self) {
        let mut timer = delta::Timer::new();
        loop {
            let deltatime = timer.mark_secs();
            if self.root.execute(deltatime) {
                break;
            }
        }
    }

    pub fn advance_to_checkpoint(&mut self, checkpoint: &str) -> bool {
        self.root.advance_to_checkpoint(checkpoint)
    }
}
