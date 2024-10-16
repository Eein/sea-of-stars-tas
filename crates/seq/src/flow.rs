use crate::Node;
use log::{debug, info};
use std::fmt::Display;

pub struct SeqIf<State, Cond: SeqCondition<State>> {
    name: String,
    on_true: Option<Box<dyn Node<State>>>,
    on_false: Option<Box<dyn Node<State>>>,
    condition: Cond,
    selection: bool,
    default_selection: bool,
}

impl<State, Cond: SeqCondition<State>> SeqIf<State, Cond> {
    pub fn create(
        name: &str,
        condition: Cond,
        on_true: Option<Box<dyn Node<State>>>,
        on_false: Option<Box<dyn Node<State>>>,
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

impl<State, Cond: SeqCondition<State>> Node<State> for SeqIf<State, Cond> {
    // When first entering the node, evaluate the conditional
    fn enter(&mut self, state: &mut State) {
        self.selection = self.condition.evaluate(state);
        info!("SeqIf({}), selecting path: {}", self.name, self.selection);
        match self.selection {
            true => {
                if let Some(child) = &mut self.on_true {
                    child.enter(state);
                }
            }
            false => {
                if let Some(child) = &mut self.on_false {
                    child.enter(state);
                }
            }
        }
    }
    // Execute the selected path until it terminates
    fn execute(&mut self, state: &mut State, delta: f64) -> bool {
        match self.selection {
            true => {
                if let Some(child) = &mut self.on_true {
                    return child.execute(state, delta);
                }
                true
            }
            false => {
                if let Some(child) = &mut self.on_false {
                    return child.execute(state, delta);
                }
                true
            }
        }
    }
    // If advancing past checkpoint, select the default path
    // TODO: Select based on data instead?
    fn advance_to_checkpoint(&mut self, state: &mut State, checkpoint: &str) -> bool {
        self.selection = self.default_selection;
        match self.selection {
            true => {
                if let Some(child) = &mut self.on_true {
                    return child.advance_to_checkpoint(state, checkpoint);
                }
                false
            }
            false => {
                if let Some(child) = &mut self.on_false {
                    return child.advance_to_checkpoint(state, checkpoint);
                }
                false
            }
        }
    }
    fn cutscene_control(&self) -> bool {
        match self.selection {
            true => {
                if let Some(child) = &self.on_true {
                    return child.cutscene_control();
                }
            }
            false => {
                if let Some(child) = &self.on_false {
                    return child.cutscene_control();
                }
            }
        }
        false
    }
    fn exit(&self, state: &mut State) {
        match self.selection {
            true => {
                if let Some(child) = &self.on_true {
                    child.exit(state);
                }
            }
            false => {
                if let Some(child) = &self.on_false {
                    child.exit(state);
                }
            }
        }
    }
}

pub trait SeqCondition<State> {
    // Override
    fn evaluate(&self, _state: &State) -> bool {
        false
    }
}

#[derive(Default)]
pub struct SeqList<State> {
    name: String,
    children: Vec<Box<dyn Node<State>>>,
    step: usize,
}

impl<State: Default> SeqList<State> {
    pub fn create(name: &str, children: Vec<Box<dyn Node<State>>>) -> Box<Self> {
        Box::new(SeqList::<State> {
            name: name.to_owned(),
            children,
            ..Default::default()
        })
    }
}

impl<State> Display for SeqList<State> {
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

impl<State> SeqList<State> {
    fn in_bounds(&self) -> bool {
        self.step < self.children.len()
    }
}

impl<State> Node<State> for SeqList<State> {
    fn enter(&mut self, state: &mut State) {
        // Run enter for the first child
        debug!(
            "Enter SeqList({}), {} children",
            self.name,
            self.children.len()
        );
        if self.in_bounds() {
            self.children[self.step].enter(state);
        }
    }

    fn exit(&self, _state: &mut State) {
        debug!("Leaving SeqList({})", self.name);
    }

    fn execute(&mut self, state: &mut State, delta: f64) -> bool {
        if !self.in_bounds() {
            true
        } else {
            if self.children[self.step].execute(state, delta) {
                self.children[self.step].exit(state);
                self.step += 1;
                if self.in_bounds() {
                    self.children[self.step].enter(state);
                }
            }
            false
        }
    }

    fn cutscene_control(&self) -> bool {
        if !self.in_bounds() {
            false
        } else {
            self.children[self.step].cutscene_control()
        }
    }

    fn advance_to_checkpoint(&mut self, state: &mut State, checkpoint: &str) -> bool {
        loop {
            if !self.in_bounds() {
                return false;
            }
            if self.children[self.step].advance_to_checkpoint(state, checkpoint) {
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

impl<State> Node<State> for SeqCheckpoint {
    fn enter(&mut self, _state: &mut State) {
        info!("Checkpoint: {}", self.checkpoint_name);
    }

    fn advance_to_checkpoint(&mut self, _state: &mut State, checkpoint: &str) -> bool {
        self.checkpoint_name == checkpoint
    }
}
