use crate::Node;
use std::fmt::Display;

pub struct Sequencer<T, E> {
    root: Box<dyn Node<T, E>>,
    initialized: bool,
    finished: bool,
}

impl<T, E> Display for Sequencer<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Root: {}", self.root)
    }
}

impl<T, E> Sequencer<T, E> {
    pub fn new(root: Box<dyn Node<T, E>>) -> Self {
        Sequencer {
            root,
            initialized: false,
            finished: false,
        }
    }

    pub fn is_running(&self) -> bool {
        !self.finished
    }

    pub fn cutscene_control(&self) -> bool {
        self.root.cutscene_control()
    }

    pub fn on_event(&mut self, context: &mut T, event: &E) {
        if self.finished {
            return;
        }
        self.root.on_event(context, event);
    }

    pub fn run(&mut self, context: &mut T, delta: f64) -> bool {
        // Return early if the sequencer already finished
        if self.finished {
            return true;
        }
        // Perform initialization if needed
        if !self.initialized {
            self.initialized = true;
            self.root.enter(context);
        }
        // Update the sequencer root
        if self.root.execute(context, delta) {
            self.finished = true;
            self.root.exit(context);
        }
        self.finished
    }

    pub fn advance_to_checkpoint(&mut self, context: &mut T, checkpoint: &str) -> bool {
        self.root.advance_to_checkpoint(context, checkpoint)
    }
}

#[cfg(test)]
mod tests {
    use crate::flow::{SeqCheckpoint, SeqCondition, SeqIf, SeqList};
    use crate::logging::SeqLog;
    use crate::sequencer::Sequencer;
    use crate::Node;
    use std::fmt::Display;

    // Example state
    #[derive(Default)]
    struct State {
        value: u32,
    }

    // TODO(orkaboy): test
    #[derive(Default)]
    struct Event;

    // Example node
    #[derive(Default)]
    struct SeqTest {
        value: u32,
    }

    impl SeqTest {
        fn create(value: u32) -> Box<Self> {
            Box::new(Self { value })
        }
    }

    impl Node<State, Event> for SeqTest {
        fn enter(&mut self, state: &mut State) {
            assert_eq!(state.value, self.value);
        }
    }

    impl Display for SeqTest {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Test({})", self.value)
        }
    }

    // Example node
    #[derive(Default)]
    struct SeqAssert {
        value: bool,
    }

    impl SeqAssert {
        fn create(value: bool) -> Box<Self> {
            Box::new(Self { value })
        }
    }

    impl Node<State, Event> for SeqAssert {
        fn enter(&mut self, _state: &mut State) {
            assert!(self.value);
        }
    }

    impl Display for SeqAssert {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Assert({})", self.value)
        }
    }

    // Example node
    #[derive(Default)]
    struct SeqSetter {
        value: u32,
    }

    impl SeqSetter {
        fn create(value: u32) -> Box<Self> {
            Box::new(Self { value })
        }
    }

    impl Node<State, Event> for SeqSetter {
        fn enter(&mut self, state: &mut State) {
            state.value = self.value;
        }
    }

    impl Display for SeqSetter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Setter({})", self.value)
        }
    }

    // Test condition. Checks if value is greater than
    #[derive(Default)]
    struct CondGt {
        value: u32,
    }

    impl SeqCondition<State> for CondGt {
        fn evaluate(&self, state: &State) -> bool {
            state.value > self.value
        }
    }

    #[test]
    fn seq_test() -> std::io::Result<()> {
        // Create a sequencer object
        let mut sequencer: Sequencer<State, Event> = Sequencer::new(SeqList::create(
            "Root",
            vec![
                // Check that state has initialized to 0
                SeqTest::create(0),
                SeqLog::create("Starting TAS"),
                SeqLog::create("Selecting character"),
                // Set state to 2 and verify
                SeqSetter::create(2),
                SeqTest::create(2),
                // Create a branch, checking if state is > 3 (which it is not)
                SeqIf::create(
                    "Hidden Path, should trigger false path",
                    CondGt { value: 3 },
                    Some(SeqAssert::create(false)),
                    Some(SeqAssert::create(true)), // Should go here
                    false,
                ),
                // Set state to 4 and verify
                SeqSetter::create(4),
                SeqTest::create(4),
                // Create a branch, checking if state is > 3 (which it is)
                SeqIf::create(
                    "Path, value > 3, should trigger true path",
                    CondGt { value: 3 },
                    Some(SeqAssert::create(true)), // Should go here
                    Some(SeqAssert::create(false)),
                    false,
                ),
                SeqLog::create("Mountain Trail 1"),
                // Create a checkpoint, skipping things before this
                SeqCheckpoint::create("Mooncradle"),
                SeqList::create(
                    "Mooncradle",
                    vec![
                        SeqLog::create("Garl nooo..."),
                        SeqLog::create("Training"),
                        SeqLog::create("Final Trials"),
                    ],
                ),
                SeqList::create(
                    "Forbidden Cave",
                    vec![
                        SeqLog::create("Juking snails"),
                        SeqLog::create("Big boss"),
                        SeqLog::create("Leaving caves"),
                    ],
                ),
                SeqLog::create("Make jam"),
            ],
        ));

        // Optionally, advance to a checkpoint in the sequence
        //sequencer.advance_to_checkpoint("Mooncradle");

        let mut state = State { value: 0 };

        // Run the sequence until it's done
        loop {
            if sequencer.run(&mut state, 0.1) {
                break;
            }
        }

        Ok(())
    }
}
