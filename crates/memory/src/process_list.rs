use std::{
    str,
    time::{Duration, Instant},
};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};

pub struct ProcessList {
    system: System,
    last_check: Instant,
}

impl ProcessList {
    pub fn new() -> Self {
        Self {
            system: System::new_with_specifics(
                RefreshKind::new().with_processes(ProcessRefreshKind::new()),
            ),
            last_check: Instant::now() - Duration::from_secs(1),
        }
    }

    pub fn refresh(&mut self) {
        let now = Instant::now();
        if now - self.last_check >= Duration::from_secs(1) {
            self.system
                .refresh_processes_specifics(ProcessesToUpdate::All, ProcessRefreshKind::new());
            self.last_check = now;
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

    pub fn is_open(&mut self, pid: Pid) -> bool {
        self.refresh();
        self.system.process(pid).is_some()
    }
}
