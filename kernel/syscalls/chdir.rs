use crate::fs::path::Path;
use crate::result::Result;
use crate::{process::current_process, syscalls::SyscallDispatcher};

impl<'a> SyscallDispatcher<'a> {
    pub fn sys_chdir(&mut self, path: &Path) -> Result<isize> {
        current_process().root_fs.lock().chdir(path)?;
        Ok(0)
    }
}
