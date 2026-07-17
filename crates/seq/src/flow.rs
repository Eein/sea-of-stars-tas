use std::fmt::Display;

use crate::{Node, SeqTreeNode};
use log::{debug, info, warn};

pub struct SeqIf<State, Event, Cond: SeqCondition<State, Event>> {
    name: String,
    on_true: Option<Box<dyn Node<State, Event>>>,
    on_false: Option<Box<dyn Node<State, Event>>>,
    condition: Cond,
    selection: bool,
    default_selection: bool,
}

impl<State, Event, Cond: SeqCondition<State, Event>> SeqIf<State, Event, Cond> {
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

impl<State, Event, Cond: SeqCondition<State, Event>> Display for SeqIf<State, Event, Cond> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = format!("SeqIf({}), selecting path: {}", self.name, self.selection);
        match self.selection {
            true => {
                if let Some(child) = &self.on_true {
                    ret = format!("{}\n-> {}", ret, child);
                }
            }
            false => {
                if let Some(child) = &self.on_false {
                    ret = format!("{}\n-> {}", ret, child);
                }
            }
        }
        write!(f, "{}", ret)
    }
}

impl<State, Event, Cond: SeqCondition<State, Event>> Node<State, Event>
    for SeqIf<State, Event, Cond>
{
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

    fn checkpoints(&self, out: &mut Vec<String>) {
        if let Some(child) = &self.on_true {
            child.checkpoints(out);
        }
        if let Some(child) = &self.on_false {
            child.checkpoints(out);
        }
    }

    fn tree(&self, active: bool) -> SeqTreeNode {
        // Both branches are shown; the selected one is active. Selection is
        // only meaningful once the node has been entered (it defaults false).
        let mut children = Vec::new();
        let mut active_child = None;
        if let Some(child) = &self.on_true {
            if self.selection {
                active_child = Some(children.len());
            }
            let mut tree = child.tree(active && self.selection);
            tree.label = format!("then: {}", tree.label);
            children.push(tree);
        }
        if let Some(child) = &self.on_false {
            if !self.selection {
                active_child = Some(children.len());
            }
            let mut tree = child.tree(active && !self.selection);
            tree.label = format!("else: {}", tree.label);
            children.push(tree);
        }
        SeqTreeNode {
            label: format!("SeqIf({})", self.name),
            children,
            active_child,
            active,
            completed: false,
        }
    }
}

pub struct SeqFallback<State, Event, Cond: SeqCondition<State, Event>> {
    name: String,
    primary: Box<dyn Node<State, Event>>,
    fallback: Box<dyn Node<State, Event>>,
    condition: Cond,
    fallback_triggered: bool,
}

impl<State, Event, Cond: SeqCondition<State, Event>> SeqFallback<State, Event, Cond> {
    pub fn create(
        name: &str,
        condition: Cond,
        primary: Box<dyn Node<State, Event>>,
        fallback: Box<dyn Node<State, Event>>,
    ) -> Box<Self> {
        Box::new(SeqFallback {
            name: name.to_owned(),
            condition,
            primary,
            fallback,
            fallback_triggered: false,
        })
    }
}

impl<State, Event, Cond: SeqCondition<State, Event>> Display for SeqFallback<State, Event, Cond> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = format!(
            "SeqFallback({}), fallback triggered: {}",
            self.name, self.fallback_triggered
        );
        ret = if self.fallback_triggered {
            format!("{}\n-> {}", ret, self.fallback)
        } else {
            format!("{}\n-> {}", ret, self.primary)
        };
        write!(f, "{}", ret)
    }
}

impl<State, Event, Cond: SeqCondition<State, Event>> Node<State, Event>
    for SeqFallback<State, Event, Cond>
{
    // When first entering the node, run enter of the primary path
    fn enter(&mut self, state: &mut State) {
        self.primary.enter(state);
    }
    fn on_event(&mut self, state: &mut State, event: &Event) {
        match self.fallback_triggered {
            false => self.primary.on_event(state, event),
            true => self.fallback.on_event(state, event),
        }
        if !self.fallback_triggered {
            self.fallback_triggered = self.condition.on_event(event);
            // If triggered, exit the primary state and enter the fallback state
            if self.fallback_triggered {
                warn!("SeqFallback({}) Fallback triggered!", self.name);
                self.primary.exit(state);
                self.fallback.enter(state);
            }
        }
    }
    // Execute primary path until it terminates, or fallback condition triggers
    fn execute(&mut self, state: &mut State, delta: f64) -> bool {
        match self.fallback_triggered {
            true => self.fallback.execute(state, delta),
            false => {
                // Check condition on each frame
                self.fallback_triggered = self.condition.evaluate(state);
                // If triggered, exit the primary state and enter the fallback state
                if self.fallback_triggered {
                    warn!("SeqFallback({}) Fallback triggered!", self.name);
                    self.primary.exit(state);
                    self.fallback.enter(state);
                    false
                } else {
                    // Execute the primary state
                    self.primary.execute(state, delta)
                }
            }
        }
    }
    // If advancing past checkpoint, select the primary path
    // TODO: Select based on data instead?
    fn advance_to_checkpoint(&mut self, state: &mut State, checkpoint: &str) -> bool {
        self.primary.advance_to_checkpoint(state, checkpoint)
    }
    fn cutscene_control(&self) -> bool {
        match self.fallback_triggered {
            true => self.fallback.cutscene_control(),
            false => self.primary.cutscene_control(),
        }
    }
    fn exit(&self, state: &mut State) {
        match self.fallback_triggered {
            true => self.fallback.exit(state),
            false => self.primary.exit(state),
        }
    }

    fn checkpoints(&self, out: &mut Vec<String>) {
        self.primary.checkpoints(out);
        self.fallback.checkpoints(out);
    }

    fn tree(&self, active: bool) -> SeqTreeNode {
        let mut primary = self.primary.tree(active && !self.fallback_triggered);
        primary.label = format!("primary: {}", primary.label);
        let mut fallback = self.fallback.tree(active && self.fallback_triggered);
        fallback.label = format!("fallback: {}", fallback.label);
        SeqTreeNode {
            label: format!("SeqFallback({})", self.name),
            children: vec![primary, fallback],
            active_child: Some(usize::from(self.fallback_triggered)),
            active,
            completed: false,
        }
    }
}

pub trait SeqCondition<State, Event> {
    // Override
    fn evaluate(&self, _state: &State) -> bool {
        false
    }

    fn on_event(&self, _event: &Event) -> bool {
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

impl<State, Event> Display for SeqList<State, Event> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = format!("{}({}/{})", self.name, self.step + 1, self.children.len());
        if self.in_bounds() {
            ret = format!("{}\n-> {}", ret, self.children[self.step]);
        }
        write!(f, "{}", ret)
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

    fn checkpoints(&self, out: &mut Vec<String>) {
        for child in &self.children {
            child.checkpoints(out);
        }
    }

    fn tree(&self, active: bool) -> SeqTreeNode {
        SeqTreeNode {
            label: format!("{} ({}/{})", self.name, self.step, self.children.len()),
            children: self
                .children
                .iter()
                .enumerate()
                .map(|(i, c)| c.tree(active && i == self.step))
                .collect(),
            active_child: self.in_bounds().then_some(self.step),
            active,
            // The list ran off its end — every child has executed.
            completed: !self.in_bounds() && !self.children.is_empty(),
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

    fn checkpoints(&self, out: &mut Vec<String>) {
        out.push(self.checkpoint_name.clone());
    }
}

impl Display for SeqCheckpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Checkpoint({})", self.checkpoint_name)
    }
}
