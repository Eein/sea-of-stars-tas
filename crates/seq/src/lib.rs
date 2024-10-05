use log::{debug, info};
use std::fmt::Display;
pub mod flow;
pub mod logging;
pub mod sequencer;

pub trait Node {
    fn execute(&mut self, _delta: f64) -> bool {
        true
    }
    fn advance_to_checkpoint(&mut self, _checkpoint: &str) -> bool {
        false
    }
    fn enter(&mut self) {
        // Override
    }
    fn exit(&self) {
        // Override
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

impl SeqList {
    fn in_bounds(&self) -> bool {
        self.step < self.children.len()
    }
}

impl Node for SeqList {
    fn enter(&mut self) {
        // Run enter for the first child
        debug!(
            "Enter SeqList({}), {} children",
            self.name,
            self.children.len()
        );
        if self.in_bounds() {
            self.children[self.step].enter();
        }
    }

    fn exit(&self) {
        debug!("Leaving SeqList({})", self.name);
    }

    fn execute(&mut self, delta: f64) -> bool {
        if !self.in_bounds() {
            true
        } else {
            if self.children[self.step].execute(delta) {
                self.children[self.step].exit();
                self.step += 1;
                if self.in_bounds() {
                    self.children[self.step].enter();
                }
            }
            false
        }
    }

    fn advance_to_checkpoint(&mut self, checkpoint: &str) -> bool {
        loop {
            if !self.in_bounds() {
                return false;
            }
            if self.children[self.step].advance_to_checkpoint(checkpoint) {
                return true;
            } else {
                self.step += 1;
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
    fn enter(&mut self) {
        info!("Checkpoint: {}", self.checkpoint_name);
    }

    fn advance_to_checkpoint(&mut self, checkpoint: &str) -> bool {
        self.checkpoint_name == checkpoint
    }
}
