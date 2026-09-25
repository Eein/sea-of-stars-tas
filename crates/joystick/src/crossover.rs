//! File-backed XInput controller for CrossOver's app-local xinput1_3 shim.

use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

use crate::common::{Button, JoystickBtnInterface, JoystickInterface};

const STATE_FILE: &str = "tas-xinput-state.bin";

#[derive(Default)]
pub struct Joystick {
    index: usize,
    state: State,
    destination: Option<PathBuf>,
}

#[derive(Default)]
struct State {
    buttons: u16,
    left_trigger: u8,
    right_trigger: u8,
    left_x: i16,
    left_y: i16,
    right_x: i16,
    right_y: i16,
}

impl State {
    fn encode(&self) -> [u8; 16] {
        let mut bytes = [0; 16];
        bytes[..4].copy_from_slice(b"TAS1");
        bytes[4..6].copy_from_slice(&self.buttons.to_le_bytes());
        bytes[6] = self.left_trigger;
        bytes[7] = self.right_trigger;
        bytes[8..10].copy_from_slice(&self.left_x.to_le_bytes());
        bytes[10..12].copy_from_slice(&self.left_y.to_le_bytes());
        bytes[12..14].copy_from_slice(&self.right_x.to_le_bytes());
        bytes[14..16].copy_from_slice(&self.right_y.to_le_bytes());
        bytes
    }
}

impl Joystick {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            ..Self::default()
        }
    }

    pub fn is_attached(&self) -> bool {
        self.destination.is_some()
    }

    pub fn state_file_path(&self) -> Option<&Path> {
        self.destination.as_deref()
    }

    pub fn detach(&mut self) {
        self.release_all();
        self.destination = None;
    }

    /// Locate the attached process's executable module and write the shim's
    /// state file beside it. CrossOver exposes Windows paths in process maps.
    pub fn attach_to_process(&mut self, pid: u32, executable_name: &str) -> io::Result<()> {
        let pid = pid
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid process ID"))?;
        let maps = proc_maps::get_process_maps(pid)?;
        let directory = maps
            .iter()
            .filter_map(|map| map.filename())
            .map(Path::new)
            .find(|path| path.file_name() == Some(OsStr::new(executable_name)))
            .and_then(Path::parent)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "game executable module"))?;
        self.set_directory(directory)
    }

    /// Useful for a standalone shim probe and for callers with a known game directory.
    pub fn set_directory(&mut self, directory: &Path) -> io::Result<()> {
        if self.index != 0 {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "the current XInput shim exposes only controller slot 0",
            ));
        }
        let destination = directory.join(STATE_FILE);
        self.write_to(&destination)?;
        self.destination = Some(destination);
        Ok(())
    }

    fn write_to(&self, destination: &Path) -> io::Result<()> {
        let temporary =
            destination.with_file_name(format!("{STATE_FILE}.{}.tmp", std::process::id()));
        fs::write(&temporary, self.state.encode())?;
        fs::rename(temporary, destination)
    }

    fn publish(&self) {
        if let Some(destination) = &self.destination
            && let Err(error) = self.write_to(destination)
        {
            log::error!(
                "XInput state write failed at {}: {error}",
                destination.display()
            );
        }
    }
}

impl JoystickInterface for Joystick {
    fn release_all(&mut self) {
        self.state = State::default();
        self.publish();
    }

    fn set_ljoy(&mut self, dir: [f32; 2]) {
        self.state.left_x = axis(dir[0]);
        self.state.left_y = axis(dir[1]);
        self.publish();
    }

    fn set_rjoy(&mut self, dir: [f32; 2]) {
        self.state.right_x = axis(dir[0]);
        self.state.right_y = axis(dir[1]);
        self.publish();
    }
}

fn axis(value: f32) -> i16 {
    let value = value.clamp(-1.0, 1.0);
    if value < 0.0 {
        (value * 32768.0) as i16
    } else {
        (value * 32767.0) as i16
    }
}

impl JoystickBtnInterface<Button> for Joystick {
    fn press(&mut self, button: &Button) {
        match *button {
            Button::LT(value) => self.state.left_trigger = value,
            Button::RT(value) => self.state.right_trigger = value,
            _ => self.state.buttons |= button_bit(button),
        }
        self.publish();
    }

    fn release(&mut self, button: &Button) {
        match *button {
            Button::LT(_) => self.state.left_trigger = 0,
            Button::RT(_) => self.state.right_trigger = 0,
            _ => self.state.buttons &= !button_bit(button),
        }
        self.publish();
    }
}

fn button_bit(button: &Button) -> u16 {
    match button {
        Button::UP => 0x0001,
        Button::DOWN => 0x0002,
        Button::LEFT => 0x0004,
        Button::RIGHT => 0x0008,
        Button::START => 0x0010,
        Button::SELECT => 0x0020,
        Button::LTHUMB => 0x0040,
        Button::RTHUMB => 0x0080,
        Button::LB => 0x0100,
        Button::RB => 0x0200,
        Button::A => 0x1000,
        Button::B => 0x2000,
        Button::X => 0x4000,
        Button::Y => 0x8000,
        Button::LT(_) | Button::RT(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::{Joystick, axis};
    use crate::common::{Button, JoystickBtnInterface, JoystickInterface};

    #[test]
    fn writes_atomic_xinput_state_file() {
        let directory =
            std::env::temp_dir().join(format!("tas-xinput-test-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let mut pad = Joystick::new(0);
        pad.set_directory(&directory).unwrap();
        pad.press(&Button::A);
        pad.press(&Button::LT(128));
        pad.set_ljoy([-1.0, 1.0]);
        let bytes = std::fs::read(directory.join("tas-xinput-state.bin")).unwrap();
        assert_eq!(&bytes[..4], b"TAS1");
        assert_eq!(u16::from_le_bytes(bytes[4..6].try_into().unwrap()), 0x1000);
        assert_eq!(bytes[6], 128);
        assert_eq!(i16::from_le_bytes(bytes[8..10].try_into().unwrap()), -32768);
        assert_eq!(i16::from_le_bytes(bytes[10..12].try_into().unwrap()), 32767);
        pad.release_all();
        assert_eq!(
            &std::fs::read(directory.join("tas-xinput-state.bin")).unwrap()[4..],
            &[0; 12]
        );
        std::fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn axis_clamps() {
        assert_eq!(axis(-2.0), -32768);
        assert_eq!(axis(2.0), 32767);
    }
}
