use delta::Timer;

use crate::Node;

pub struct Sequencer<T> {
    root: Box<dyn Node<T>>,
    initialized: bool,
    finished: bool,
    timer: Timer,
}

impl<T> Sequencer<T> {
    pub fn create(root: Box<dyn Node<T>>) -> Self {
        Sequencer {
            root,
            initialized: false,
            finished: false,
            timer: delta::Timer::new(),
        }
    }

    pub fn run(&mut self, context: &mut T) -> bool {
        // Return early if the sequencer already finished
        if self.finished {
            return true;
        }
        // Perform initialization if needed
        if !self.initialized {
            self.initialized = true;
            let _ = self.timer.mark();
            self.root.enter(context);
        }
        // Update the sequencer root
        let dt = self.timer.mark_secs();
        if self.root.execute(context, dt) {
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

    // Example state
    #[derive(Default)]
    struct State {
        value: u32,
    }

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

    impl Node<State> for SeqTest {
        fn enter(&mut self, state: &mut State) {
            assert_eq!(state.value, self.value);
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

    impl Node<State> for SeqAssert {
        fn enter(&mut self, _state: &mut State) {
            assert!(self.value);
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

    impl Node<State> for SeqSetter {
        fn enter(&mut self, state: &mut State) {
            state.value = self.value;
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
        let mut sequencer: Sequencer<State> = Sequencer::create(SeqList::create(
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
            if sequencer.run(&mut state) {
                break;
            }
        }

        Ok(())
    }
}
