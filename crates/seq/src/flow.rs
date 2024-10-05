use crate::Node;
use log::{debug, info};
use std::fmt::Display;

pub struct SeqIf<T: SeqCondition> {
    name: String,
    on_true: Option<Box<dyn Node>>,
    on_false: Option<Box<dyn Node>>,
    condition: T,
    selection: bool,
    default_selection: bool,
}

impl<T: SeqCondition> SeqIf<T> {
    pub fn create(
        name: &str,
        condition: T,
        on_true: Option<Box<dyn Node>>,
        on_false: Option<Box<dyn Node>>,
        default_selection: bool,
    ) -> Box<Self> {
        Box::new(SeqIf {
            name: name.to_owned(),
            condition,
            on_true,
            on_false,
            selection: false,
            default_selection,
        })
    }
}

impl<T: SeqCondition> Node for SeqIf<T> {
    // When first entering the node, evaluate the conditional
    fn enter(&mut self) {
        self.selection = self.condition.evaluate();
        info!("SeqIf({}), selecting path: {}", self.name, self.selection);
        match self.selection {
            true => {
                if let Some(child) = &mut self.on_true {
                    child.enter();
                }
            }
            false => {
                if let Some(child) = &mut self.on_false {
                    child.enter();
                }
            }
        }
    }
    // Execute the selected path until it terminates
    fn execute(&mut self, delta: f64) -> bool {
        match self.selection {
            true => {
                if let Some(child) = &mut self.on_true {
                    return child.execute(delta);
                }
                true
            }
            false => {
                if let Some(child) = &mut self.on_false {
                    return child.execute(delta);
                }
                true
            }
        }
    }
    // If advancing past checkpoint, select the default path
    // TODO: Select based on data instead?
    fn advance_to_checkpoint(&mut self, checkpoint: &str) -> bool {
        self.selection = self.default_selection;
        match self.selection {
            true => {
                if let Some(child) = &mut self.on_true {
                    return child.advance_to_checkpoint(checkpoint);
                }
                false
            }
            false => {
                if let Some(child) = &mut self.on_false {
                    return child.advance_to_checkpoint(checkpoint);
                }
                false
            }
        }
    }
    fn exit(&self) {
        match self.selection {
            true => {
                if let Some(child) = &self.on_true {
                    child.exit();
                }
            }
            false => {
                if let Some(child) = &self.on_false {
                    child.exit();
                }
            }
        }
    }
}

pub trait SeqCondition {
    // TODO(orkaboy): Override. Also needs state/data here!
    fn evaluate(&self) -> bool {
        false
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
