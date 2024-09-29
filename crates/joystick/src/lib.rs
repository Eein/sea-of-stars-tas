pub mod common;

#[cfg(target_os = "linux")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
pub mod joystick;
#[cfg(target_os = "windows")]
#[cfg_attr(windows, path = "windows.rs")]
pub mod joystick;
