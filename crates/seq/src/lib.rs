use std::fmt::Display;

pub mod prelude {
    pub use crate::Node;
    pub use crate::SeqTreeNode;
    pub use crate::flow::*;
    pub use crate::logging::*;
    pub use crate::sequencer::Sequencer;
    pub use crate::wait::*;
}

pub mod flow;
pub mod logging;
pub mod sequencer;
pub mod wait;

/// A display snapshot of a node subtree: a label, the child subtrees, and
/// which child is on the live execution path. Built by [`Node::tree`] so a
/// GUI can render the whole sequence with the running node highlighted.
///
/// Following `active_child` from the root yields the chain of currently
/// executing nodes; the concatenated child indices form a stable path to any
/// node (the future "play from here" jump target).
#[derive(Debug, Clone)]
pub struct SeqTreeNode {
    pub label: String,
    pub children: Vec<SeqTreeNode>,
    /// Index into `children` of the node currently executing (or selected),
    /// `None` for leaves and exhausted containers.
    pub active_child: Option<usize>,
    /// On the live execution chain (the root down to the running node).
    pub active: bool,
    /// This node's execution has fully finished — the GUI collapses these.
    pub completed: bool,
}

pub trait Node<State, Event>: Display {
    fn execute(&mut self, _state: &mut State, _delta: f64) -> bool {
        true
    }
    fn advance_to_checkpoint(&mut self, _state: &mut State, _checkpoint: &str) -> bool {
        false
    }
    fn on_event(&mut self, _state: &mut State, _event: &Event) {
        // Override
    }
    fn enter(&mut self, _state: &mut State) {
        // Override
    }
    fn exit(&self, _state: &mut State) {
        // Override
    }
    fn cutscene_control(&self) -> bool {
        false
    }
    /// One-line label for the tree display. Defaults to the first line of the
    /// node's `Display` (containers append their current child on later
    /// lines, which the tree renders as real children instead).
    fn label(&self) -> String {
        self.to_string()
            .lines()
            .next()
            .unwrap_or_default()
            .to_string()
    }
    /// Collect the name of every checkpoint in this subtree, in route order.
    /// Container nodes override to recurse into *all* their children (both
    /// branches of a conditional — the list enumerates what a run could
    /// reach); `SeqCheckpoint` pushes its name. Lets the checkpoint list be
    /// generated from the route definition instead of maintained by hand.
    fn checkpoints(&self, _out: &mut Vec<String>) {}

    /// Snapshot of this subtree for the GUI's sequence tree. Leaves are a
    /// bare label; container nodes override this to list their children and
    /// mark the one currently executing (passing `active` down only to that
    /// child). `active` = this node is on the live execution chain — nodes
    /// with expensive-to-list children (`SeqMove`'s coordinates) only include
    /// them when active, so building the snapshot stays cheap.
    fn tree(&self, active: bool) -> SeqTreeNode {
        SeqTreeNode {
            label: self.label(),
            children: Vec::new(),
            active_child: None,
            active,
            completed: false,
        }
    }
}
