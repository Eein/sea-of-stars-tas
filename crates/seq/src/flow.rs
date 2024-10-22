use crate::Node;
use log::{debug, info};

pub struct SeqIf<State, Event, Cond: SeqCondition<State>> {
    name: String,
    on_true: Option<Box<dyn Node<State, Event>>>,
    on_false: Option<Box<dyn Node<State, Event>>>,
    condition: Cond,
    selection: bool,
    default_selection: bool,
}

impl<State, Event, Cond: SeqCondition<State>> SeqIf<State, Event, Cond> {
    pub fn create(
        name: &str,
        condition: Cond,
        on_true: Option<Box<dyn Node<State, Event>>>,
        on_false: Option<Box<dyn Node<State, Event>>>,
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

impl<State, Event, Cond: SeqCondition<State>> Node<State, Event> for SeqIf<State, Event, Cond> {
    fn print(&self) -> String {
        let mut ret = format!("SeqIf({}), selecting path: {}", self.name, self.selection);
        match self.selection {
            true => {
                if let Some(child) = &self.on_true {
                    ret = format!("{}\n-> {}", ret, child.print());
                }
            }
            false => {
                if let Some(child) = &self.on_false {
                    ret = format!("{}\n-> {}", ret, child.print());
                }
            }
        }
        ret
    }
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
    fn on_event(&mut self, state: &mut State, event: &Event) {
        match self.selection {
            true => {
                if let Some(child) = &mut self.on_true {
                    child.on_event(state, event);
                }
            }
            false => {
                if let Some(child) = &mut self.on_false {
                    child.on_event(state, event);
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
pub struct SeqList<State, Event> {
    name: String,
    children: Vec<Box<dyn Node<State, Event>>>,
    step: usize,
}

impl<State: Default, Event: Default> SeqList<State, Event> {
    pub fn create(name: &str, children: Vec<Box<dyn Node<State, Event>>>) -> Box<Self> {
        Box::new(SeqList::<State, Event> {
            name: name.to_owned(),
            children,
            ..Default::default()
        })
    }
}

impl<State, Event> SeqList<State, Event> {
    fn in_bounds(&self) -> bool {
        self.step < self.children.len()
    }
}

impl<State, Event> Node<State, Event> for SeqList<State, Event> {
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

    fn print(&self) -> String {
        let mut ret = format!("{}({}/{})", self.name, self.step + 1, self.children.len());
        if self.in_bounds() {
            ret = format!("{}\n-> {}", ret, self.children[self.step].print());
        }
        ret
    }

    fn exit(&self, _state: &mut State) {
        debug!("Leaving SeqList({})", self.name);
    }

    fn on_event(&mut self, state: &mut State, event: &Event) {
        if self.in_bounds() {
            self.children[self.step].on_event(state, event);
        }
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

impl<State, Event> Node<State, Event> for SeqCheckpoint {
    fn enter(&mut self, _state: &mut State) {
        info!("Checkpoint: {}", self.checkpoint_name);
    }

    fn advance_to_checkpoint(&mut self, _state: &mut State, checkpoint: &str) -> bool {
        self.checkpoint_name == checkpoint
    }
}
