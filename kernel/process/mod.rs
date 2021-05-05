use crate::{
    arch::{self, SpinLock, SpinLockGuard},
    fs::opened_file::*,
    mm::vm::{Vm, VmAreaType},
};

use alloc::sync::Arc;

use arch::{UserVAddr, KERNEL_STACK_SIZE, PAGE_SIZE, USER_STACK_TOP};

use core::cmp::max;
use core::mem::size_of;

use penguin_utils::once::Once;
use penguin_utils::{alignment::align_up, lazy::Lazy};

mod elf;
mod execve;
mod fork;
mod init_stack;
#[allow(clippy::module_inception)]
mod process;
mod scheduler;
pub mod signal;
mod switch;
mod wait_queue;

pub use execve::*;
pub use fork::*;
pub use init_stack::*;
pub use process::*;
pub use scheduler::*;
pub use switch::*;
pub use wait_queue::*;

cpu_local! {
    static ref CURRENT: Lazy<Arc<SpinLock<Process>>> = Lazy::new();
}

cpu_local! {
    // TODO: Should be pub(super)
    pub static ref IDLE_THREAD: Lazy<Arc<SpinLock<Process>>> = Lazy::new();
}

static SCHEDULER: Once<SpinLock<Scheduler>> = Once::new();
pub static JOIN_WAIT_QUEUE: Once<WaitQueue> = Once::new();

pub fn current_process() -> SpinLockGuard<'static, Process> {
    CURRENT.get().lock()
}

pub fn current_process_arc() -> &'static Arc<SpinLock<Process>> {
    CURRENT.get()
}

pub fn kill_current_process() -> ! {
    // TODO:
    todo!()
}

pub fn init() {
    JOIN_WAIT_QUEUE.init(WaitQueue::new);
    SCHEDULER.init(|| SpinLock::new(Scheduler::new()));
    let idle_thread = Process::new_idle_thread().unwrap();
    IDLE_THREAD.as_mut().set(idle_thread.clone());
    CURRENT.as_mut().set(idle_thread);
}
