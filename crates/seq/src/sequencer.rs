use crate::Node;

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
        self.root.enter();
        loop {
            let deltatime = timer.mark_secs();
            if self.root.execute(deltatime) {
                break;
            }
        }
        self.root.exit();
    }

    pub fn advance_to_checkpoint(&mut self, checkpoint: &str) -> bool {
        self.root.advance_to_checkpoint(checkpoint)
    }
}

#[cfg(test)]
mod tests {
    use crate::flow::{SeqCheckpoint, SeqCondition, SeqIf, SeqList};
    use crate::logging::SeqLog;
    use crate::sequencer::Sequencer;

    #[derive(Default)]
    struct TestCond;
    impl SeqCondition for TestCond {
        fn evaluate(&self) -> bool {
            true
        }
    }

    #[test]
    fn seq_test() -> std::io::Result<()> {
        // Create a sequencer object
        let mut sequencer = Sequencer::create(SeqList::create(
            "Root",
            vec![
                SeqLog::create("Starting TAS"),
                SeqLog::create("Selecting character"),
                SeqIf::create(
                    "Path",
                    TestCond,
                    Some(SeqLog::create("True path")),
                    Some(SeqLog::create("False path")),
                    false,
                ),
                SeqLog::create("Mountain Trail 1"),
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

        // Run the sequence (grabs control atm)
        sequencer.run();

        Ok(())
    }
}
