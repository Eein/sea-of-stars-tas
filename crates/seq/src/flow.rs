use crate::Node;
use log::info;

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
