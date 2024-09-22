#[cfg_attr(target_os = "linux", path = "linux.rs")]
pub mod linux;
#[cfg_attr(windows, path = "windows.rs")]
pub mod windows;
