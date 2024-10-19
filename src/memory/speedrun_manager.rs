use crate::memory::memory_context::MemoryContext;
use crate::memory::{MemoryManager, MemoryManagerUpdate};
use crate::state::StateContext;
use log::info;
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::MemoryError;
use std::fmt::Display;
use std::time::Duration;

const SECONDS_PER_MINUTE: u64 = 60;
const SECONDS_PER_HOUR: u64 = 60 * SECONDS_PER_MINUTE;

#[derive(Default, Debug)]
pub struct SpeedrunManagerData {
    // The Main Timer for speedrun mode
    pub speedrun_timer: SpeedrunTimer,

    // The Pause Timer for speedrun mode - this uses the same struct as above
    pub pause_timer: SpeedrunTimer,

    // Unsure if this is for the statemachine or something else.
    // TODO(eein): determine this fields true use
    pub is_speedrunning: bool,

    // Looks like this is an alias to the speedrunTimer -> isPaused
    // and is used for ui or statemachine. The next field in the struct
    // is resumeSpeedrunNextFrame, which likely is for their statemachine.
    pub speedrun_timer_pause_lock: bool,
}

impl Default for MemoryManager<SpeedrunManagerData> {
    fn default() -> Self {
        let manager = Self {
            name: "SpeedrunManager".to_string(),
            data: SpeedrunManagerData::default(),
            manager: UnityMemoryManager::default(),
        };
        info!("Memory: {} Loaded", manager.name);
        manager
    }
}

#[derive(Default, Debug)]
pub struct SpeedrunTimer {
    pub is_started: bool,
    pub is_paused: bool,
    pub timer_in_second: f64, // or f32?
    // pub time_char_array_hours_2_digits 0x20
    // pub time_char_array_hours_3_digits 0x28

    // The time since opening the game
    pub realtime_delta_time: f64,
}

impl Display for SpeedrunTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time = self.timer_in_second;
        let duration = Duration::from_secs_f64(time);
        let nanoseconds = duration.subsec_millis();
        let total_seconds = duration.as_secs();

        let seconds = (total_seconds % SECONDS_PER_MINUTE) as u8;
        let minutes = ((total_seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE) as u8;
        let hours = total_seconds / SECONDS_PER_HOUR;

        let mut buffer = itoa::Buffer::new();
        let hours = buffer.format(hours);
        let out = format!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>3}",
            hours, minutes, seconds, nanoseconds
        );
        write!(f, "{}", out)
    }
}

impl MemoryManagerUpdate for SpeedrunManagerData {
    fn update(
        &mut self,
        ctx: &StateContext,
        manager: &mut UnityMemoryManager,
    ) -> Result<(), MemoryError> {
        let memory_context = MemoryContext::create(ctx, manager)?;

        self.update_speedrun_timer(&memory_context)?;
        self.update_pause_timer(&memory_context)?;
        self.update_is_speedrunning(&memory_context)?;
        self.update_speedrun_timer_pause_lock(&memory_context)?;

        Ok(())
    }
}

impl SpeedrunManagerData {
    pub fn update_speedrun_timer_pause_lock(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(current_time) = memory_context.follow_fields::<u8>(&["speedrunTimerPauseLock"]) {
            self.speedrun_timer_pause_lock = match current_time {
                0 => false,
                1 => true,
                _ => false,
            };
        }

        Ok(())
    }
    pub fn update_is_speedrunning(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(current_time) = memory_context.follow_fields::<u8>(&["isSpeedRunning"]) {
            self.is_speedrunning = match current_time {
                0 => false,
                1 => true,
                _ => false,
            };
        }

        Ok(())
    }
    pub fn update_speedrun_timer(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(speedrun_timer) =
            Self::update_speedrun_timer_struct(memory_context, "speedrunTimer")
        {
            self.speedrun_timer = speedrun_timer
        }

        Ok(())
    }
    pub fn update_pause_timer(
        &mut self,
        memory_context: &MemoryContext,
    ) -> Result<(), MemoryError> {
        if let Ok(speedrun_timer) = Self::update_speedrun_timer_struct(memory_context, "pauseTimer")
        {
            self.pause_timer = speedrun_timer
        }

        Ok(())
    }
    fn update_speedrun_timer_struct(
        memory_context: &MemoryContext,
        field_name: &str,
    ) -> Result<SpeedrunTimer, MemoryError> {
        if memory_context
            .follow_fields_without_read(&[field_name])
            .is_ok()
        {
            let is_started = match memory_context.follow_fields::<u8>(&[field_name, "isStarted"])? {
                0 => false,
                1 => true,
                _ => false,
            };
            let is_paused = match memory_context.follow_fields::<u8>(&[field_name, "isPaused"])? {
                0 => false,
                1 => true,
                _ => false,
            };
            let timer_in_second =
                memory_context.follow_fields::<f64>(&[field_name, "timerInSecond"])?;
            let realtime_delta_time =
                memory_context.follow_fields::<f64>(&[field_name, "realtimeDeltaTime"])?;

            Ok(SpeedrunTimer {
                is_started,
                is_paused,
                timer_in_second,
                realtime_delta_time,
            })
        } else {
            Err(MemoryError::NullPointer)
        }
    }
}
