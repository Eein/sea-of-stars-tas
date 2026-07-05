use asr::{Address, PointerSize, Process as AsrProcess, ProcessId};
use bytemuck::CheckedBitPattern;

use crate::process_list::ProcessList;

/// Every process we attach to is a 64-bit Proton/Wine target.
const POINTER_SIZE: PointerSize = PointerSize::Bit64;

#[derive(Debug)]
pub enum OpenError {
    ProcessDoesntExist,
    InvalidHandle,
}

#[derive(Debug)]
pub enum ModuleError {
    ModuleDoesntExist,
    ListModules,
}

#[derive(Debug)]
pub enum MemoryError {
    InvalidParameters, // When some parameters are invalid
    NullPointer,       // When a ptr is 0
    Unset,             // When a value is unset (ie None)
    ReadError,         // when memory cant be read
}

impl From<asr::Error> for MemoryError {
    fn from(_: asr::Error) -> Self {
        // asr's `Error` is opaque, so every read/attach failure collapses to
        // `ReadError`. The other variants are produced only by our own guards.
        MemoryError::ReadError
    }
}

/// A thin wrapper over [`asr::Process`]. asr owns the native process handle,
/// module-range cache (1s throttle) and pointer reads; we keep the same method
/// surface the rest of the crate + the 15 memory managers already call.
pub struct Process {
    pub proc: AsrProcess,
    pub pid: u32,
}

impl std::fmt::Debug for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Process").field("pid", &self.pid).finish()
    }
}

impl Process {
    /// Attach to the newest running instance of `name`. We still pick the
    /// process via sysinfo (newest by start time / pid) so relaunches attach to
    /// the fresh instance, then hand the pid to asr to open the handle.
    pub fn with_name(name: &str, process_list: &mut ProcessList) -> Result<Self, OpenError> {
        process_list.refresh();
        let processes = process_list.processes_by_name(name);

        match &processes.max_by_key(|p| (p.start_time(), p.pid().as_u32())) {
            Some(process) => {
                let pid = process.pid().as_u32();
                let proc = AsrProcess::attach_by_pid(ProcessId(pid as u64))
                    .ok_or(OpenError::InvalidHandle)?;
                Ok(Process { proc, pid })
            }
            None => Err(OpenError::ProcessDoesntExist),
        }
    }

    pub fn module_address(&self, module: &str) -> Result<u64, ModuleError> {
        self.proc
            .get_module_address(module)
            .map(|a| a.value())
            .map_err(|_| ModuleError::ModuleDoesntExist)
    }

    pub fn read<T: CheckedBitPattern>(&self, address: u64) -> Result<T, MemoryError> {
        self.proc
            .read::<T>(Address::new(address))
            .map_err(|_| MemoryError::ReadError)
    }

    pub fn read_pointer<T: CheckedBitPattern>(&self, address: u64) -> Result<T, MemoryError> {
        self.read::<T>(address)
    }

    /// Walk `path`, dereferencing all but the last offset from `address`, then
    /// read `T` at the final offset. Keeps our explicit `InvalidParameters`
    /// guard on empty paths so the manager reset logic stays intact.
    pub fn read_pointer_path<T: CheckedBitPattern>(
        &self,
        address: u64,
        path: &[u64],
    ) -> Result<T, MemoryError> {
        let (&last, path) = path.split_last().ok_or(MemoryError::InvalidParameters)?;
        let mut address = address;
        for &offset in path {
            address = self
                .proc
                .read_pointer(Address::new(address + offset), POINTER_SIZE)
                .map_err(|_| MemoryError::ReadError)?
                .value();
        }

        self.read::<T>(address + last)
    }
}
