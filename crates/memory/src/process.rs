use bytemuck::{CheckedBitPattern, Pod};
use std::{
    io,
    time::{Duration, Instant},
};

use core::{
    mem::{self, MaybeUninit},
    slice,
};

use crate::process_list::ProcessList;
use proc_maps::{MapRange, Pid};
use read_process_memory::{CopyAddress, ProcessHandle};

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

pub struct Error;

pub struct Process {
    pub handle: ProcessHandle,
    pub pid: Pid,
    pub modules: Vec<MapRange>,
    pub last_check: Instant,
}

impl std::fmt::Debug for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Process").field("pid", &self.pid).finish()
    }
}

impl Process {
    pub fn with_name(name: &str, process_list: &mut ProcessList) -> Result<Self, OpenError> {
        process_list.refresh();
        let processes = process_list.processes_by_name(name);

        match &processes.max_by_key(|p| (p.start_time(), p.pid().as_u32())) {
            Some(process) => {
                let pid = process.pid().as_u32() as Pid;
                let handle = pid
                    .try_into()
                    .expect("This needs to error with an OpenError");
                Ok(Process {
                    handle,
                    pid,
                    modules: Vec::new(),
                    last_check: Instant::now() - Duration::from_secs(1),
                })
            }
            None => Err(OpenError::ProcessDoesntExist),
        }
    }

    pub fn module_address(&mut self, module: &str) -> Result<u64, ModuleError> {
        let now = Instant::now();
        if now - self.last_check >= Duration::from_secs(1) {
            self.modules = match proc_maps::get_process_maps(self.pid) {
                Ok(m) => m,
                Err(_source) => {
                    self.modules.clear();
                    return Err(ModuleError::ListModules);
                }
            };
            self.last_check = now;
        }

        match self
            .modules
            .iter()
            .find(|m| m.filename().map_or(false, |f| f.ends_with(module)))
            .map(|m| m.start() as u64)
        {
            Some(module) => Ok(module),
            None => Err(ModuleError::ModuleDoesntExist),
        }
    }

    pub fn read_mem(&self, address: u64, buf: &mut [u8]) -> io::Result<()> {
        self.handle.copy_address(address as usize, buf)
    }

    pub fn read<T: Pod>(&self, address: u64) -> Result<T, Error> {
        unsafe {
            let mut value = MaybeUninit::<T>::uninit();
            let _result = self.read_mem(
                address,
                slice::from_raw_parts_mut(value.as_mut_ptr().cast(), mem::size_of::<T>()),
            );
            Ok(value.assume_init())
        }
    }

    pub fn read_pointer<T: Pod>(&self, address: u64) -> Result<T, Error> {
        self.read::<T>(address)
    }

    pub fn read_into_uninit_buf<'buf>(
        &self,
        address: u64,
        buf: &'buf mut [u8],
    ) -> Result<&'buf mut [u8], Error> {
        // SAFETY: The process handle is guaranteed to be valid. We provide a
        // valid pointer and length to the buffer. We also do proper error
        // handling afterwards. The buffer is guaranteed to be initialized
        // afterwards, so we can safely return an u8 slice of it.
        unsafe {
            let buf_len = buf.len();
            if self.read_mem(address, buf).is_ok() {
                Ok(slice::from_raw_parts_mut(buf.as_mut_ptr().cast(), buf_len))
            } else {
                Err(Error {})
            }
        }
    }

    pub fn read_pointer_path<T: CheckedBitPattern + Pod>(
        &self,
        mut address: u64,
        path: &[u64],
    ) -> Result<T, Error> {
        let (&last, path) = path.split_last().ok_or(Error)?;
        for &offset in path {
            address = self.read_pointer::<u64>(address + offset)?;
        }

        self.read::<T>(address + last)
    }

    pub fn read_pointer_path_without_read(
        &self,
        mut address: u64,
        path: &[u64],
    ) -> Result<u64, Error> {
        let (&last, path) = path.split_last().ok_or(Error)?;
        for &offset in path {
            address = self.read_pointer::<u64>(address + offset)?;
        }

        Ok(address + last)
    }
}
