use std::time::Duration;

// Code around this was borrowed from:
// https://github.com/LiveSplit/livesplit-core/blob/master/src/timing/formatter/timer.rs
// Modified to fit our requirements, even if it is lossy

pub struct TimeFormatter;

const SECONDS_PER_MINUTE: u64 = 60;
const SECONDS_PER_HOUR: u64 = 60 * SECONDS_PER_MINUTE;

impl TimeFormatter {
    pub fn format(time: f64) -> String {
        let duration = Duration::from_secs_f64(time);
        let nanoseconds = duration.subsec_millis();
        let total_seconds = duration.as_secs();

        let seconds = (total_seconds % SECONDS_PER_MINUTE) as u8;
        let minutes = ((total_seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE) as u8;
        let hours = total_seconds / SECONDS_PER_HOUR;

        let mut buffer = itoa::Buffer::new();
        let hours = buffer.format(hours);
        format!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>3}",
            hours, minutes, seconds, nanoseconds
        )
    }
}
