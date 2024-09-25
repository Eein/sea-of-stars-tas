use std::{
    str,
    time::{Duration, Instant},
};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System, UpdateKind};

pub struct ProcessList {
    system: System,
    next_check: Instant,
}

impl ProcessList {
    pub fn new() -> Self {
        Self {
            system: System::new_with_specifics(
                RefreshKind::new().with_processes(ProcessRefreshKind::new()),
            ),
            next_check: Instant::now() + Duration::from_secs(1),
        }
    }

 pub fn refresh(&mut self) {
        let now = Instant::now();
        if now >= self.next_check {
            self.system
                .refresh_processes_specifics(ProcessesToUpdate::All, multiple_processes());
            self.next_check = now + Duration::from_secs(1);
        }
    }

    pub fn refresh_single_process(&mut self, pid: sysinfo::Pid) {
        if self
            .system
            .refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), single_process())
            == 0
        {
            // FIXME: Unfortunately `refresh_process_specifics` doesn't remove
            // the process if it doesn't exist anymore. There also doesn't seem
            // to be a way to manually remove it. So we have to do a full
            // refresh of all processes.
            self.system
                .refresh_processes_specifics(ProcessesToUpdate::All, multiple_processes());
            self.next_check = Instant::now() + Duration::from_secs(1);
        }
    }

    pub fn processes_by_name<'process: 'both, 'both>(
        &'process self,
        name: &'both str,
    ) -> impl Iterator<Item = &'process sysinfo::Process> + 'both {
        let name = name.as_bytes();

        // On Linux the process name is limited to 15 bytes. So we ensure that
        // we don't compare more than that. This may lead to false positives
        // where we may find the wrong process, but it's better than not finding
        // it at all. We could also try to look at the entire path, but
        // apparently the path may be something entirely different in emulated
        // situations like with Wine.
        #[cfg(target_os = "linux")]
        let name = &name[..name.len().min(15)];

        self.system
            .processes()
            .values()
            .filter(move |p| p.name().as_encoded_bytes() == name)
    }

    pub fn is_open(&self, pid: sysinfo::Pid) -> bool {
        self.get(pid).is_some()
    }

    pub fn get(&self, pid: sysinfo::Pid) -> Option<&sysinfo::Process> {
        self.system.process(pid)
    }
}

#[inline]
fn multiple_processes() -> ProcessRefreshKind {
    ProcessRefreshKind::new().with_exe(UpdateKind::OnlyIfNotSet)
}

#[inline]
fn single_process() -> ProcessRefreshKind {
    ProcessRefreshKind::new()
}
