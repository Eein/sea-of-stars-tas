use evdev::KeyCode;
use std::time::Duration;

pub trait JoystickInterface {
    fn press_a(&mut self);
    fn press_b(&mut self);
    fn press_x(&mut self);
    fn press_y(&mut self);
    fn press_lt(&mut self);
    fn press_rt(&mut self);
    fn press_lt2(&mut self);
    fn press_rt2(&mut self);
    fn press_select(&mut self);
    fn press_start(&mut self);
    fn press_dpad_up(&mut self);
    fn press_dpad_down(&mut self);
    fn press_dpad_left(&mut self);
    fn press_dpad_right(&mut self);

    fn release_a(&mut self);
    fn release_b(&mut self);
    fn release_x(&mut self);
    fn release_y(&mut self);
    fn release_lt(&mut self);
    fn release_rt(&mut self);
    fn release_lt2(&mut self);
    fn release_rt2(&mut self);
    fn release_select(&mut self);
    fn release_start(&mut self);
    fn release_dpad_up(&mut self);
    fn release_dpad_down(&mut self);
    fn release_dpad_left(&mut self);
    fn release_dpad_right(&mut self);

    fn tap_a(&mut self);
    fn tap_b(&mut self);
    fn tap_x(&mut self);
    fn tap_y(&mut self);
    fn tap_lt(&mut self);
    fn tap_rt(&mut self);
    fn tap_lt2(&mut self);
    fn tap_rt2(&mut self);
    fn tap_select(&mut self);
    fn tap_start(&mut self);
    fn tap_dpad_up(&mut self);
    fn tap_dpad_down(&mut self);
    fn tap_dpad_left(&mut self);
    fn tap_dpad_right(&mut self);
    fn release_all(&mut self);
    fn run(&mut self);

    #[cfg(target_os = "linux")]
    fn tap(&mut self, button: KeyCode);
    #[cfg(target_os = "linux")]
    fn press(&mut self, button: KeyCode);
    #[cfg(target_os = "linux")]
    fn release(&mut self, button: KeyCode);
    #[cfg(target_os = "linux")]
    fn release_later(&mut self, button: KeyCode, duration: Duration);
    #[cfg(target_os = "windows")]
    fn tap(&mut self, button: u16);
    #[cfg(target_os = "windows")]
    fn press(&mut self, button: u16);
    #[cfg(target_os = "windows")]
    fn release(&mut self, button: u16);
    #[cfg(target_os = "windows")]
    fn release_later(&mut self, button: u16, duration: Duration);
}
