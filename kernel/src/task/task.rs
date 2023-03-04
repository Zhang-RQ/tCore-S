use alloc::{
    string::String,
    sync::{Arc, Weak},
    vec::Vec,
};
use core::{cell::SyncUnsafeCell, fmt, mem::size_of};
use errno::Errno;
use id_alloc::{IDAllocator, RecycleAllocator};
use kernel_sync::{SleepLockSched, SpinLock, SpinLockGuard};
use log::{debug, trace};
use signal_defs::{SigActions, SigInfo, SigPending, SigSet, NSIG};
use syscall_interface::{IoVec, SyscallResult, AT_FDCWD, AT_REMOVEDIR};
use vfs::{File, OpenFlags, Path, StatMode};

use crate::{
    arch::{
        mm::{PTEFlags, PhysAddr, VirtAddr, PAGE_SIZE},
        trap::{user_trap_handler, user_trap_return, TrapFrame},
    },
    config::*,
    error::{KernelError, KernelResult},
    fs::{open, unlink, FDManager},
    loader::from_elf,
    mm::{pma::FixedPMA, BackendFile, MmapFlags, MmapProt, KERNEL_MM, MM},
    task::{kstack_alloc, pid_alloc, schedule::Scheduler},
};

use super::{
    context::{TaskContext, __switch},
    cpu, curr_ctx, curr_task, idle_ctx,
    manager::{kstack_dealloc, kstack_vm_alloc, PID},
    TASK_MANAGER,
};

bitflags::bitflags! {
    /// Five-state model:
    ///
    /// - **Running** or **Runnable** (R)
    /// - **Sleeping** states: **Interruptible** (S) and **Uninterruptible** (D).
    /// - **Stopped** (T)
    /// - **Zombie** (Z)
    pub struct TaskState: u8 {
        /// The task is waiting in scheduler.
        const RUNNABLE = 1 << 0;

        /// The task takes up a CPU core to execute its code.
        const RUNNING = 1  << 1;

        /// A task will react to `SIGSTOP` or `SIGTSTP` signals and be brought back
        /// to running or runnable by `SIGCONT` signal.
        const STOPPED = 1 << 2;

        /// Task will only for resources to be available.
        const INTERRUPTIBLE = 1 << 3;

        /// Task will react to both signals and the availability of resources.
        const UNINTERRUPTIBLE = 1 << 4;

        /// When a task has completed its execution or is terminated, it will send the
        /// `SIGCHLD` signal to the parent task and go into the zombie state.
        const ZOMBIE = 1 << 5;
    }
}

/// Mutable inner data of the task, not protected by lock.
pub struct TaskInner {
    /// Task exit code, known as the number returned to a parent process by an executable.
    pub exit_code: i32,

    /// Task context
    pub ctx: TaskContext,

    /// Current working directory.
    pub curr_dir: String,

    /// If a thread is started using `clone(2)` with the `CLONE_CHILD_SETTID` flag,
    /// set_child_tid is set to the value passed in the ctid argument of that system call.
    ///
    /// When set_child_tid is set, the very first thing the new thread does is to write
    /// its thread ID at this address.
    pub set_child_tid: usize,

    /// If a thread is started using `clone(2)` with the `CLONE_CHILD_CLEARTID` flag,
    /// clear_child_tid is set to the value passed in the ctid argument of that system call.
    pub clear_child_tid: usize,

    /// Pending signals.
    pub sig_pending: SigPending,

    /// Blocked signals.
    pub sig_blocked: SigSet,
}

unsafe impl Send for TaskInner {}

/// Mutable inner data of the task, protected by lock.
pub struct TaskLockedInner {
    /// Task state, using five-state model.
    pub state: TaskState,

    /// Sleep lock id.
    pub sleeping_on: Option<usize>,

    /// Hierarchy pointers in task management.
    /// INIT task has no parent task.
    pub parent: Option<Weak<Task>>,

    /// Pointers to child tasks.
    /// When a parent task exits before its children, they will become orphans.
    /// These tasks will be adopted by INIT task to avoid being dropped when the reference
    /// counter becomes 0.
    pub children: Vec<Arc<Task>>,
}

unsafe impl Send for TaskLockedInner {}

/// In conventional opinion, process is the minimum unit of resource allocation, while task (or
/// thread) is the minimum unit of scheduling. Process is always created with a main task. On
/// the one hand, a process may have several tasks; on the other hand, these tasks shared the
/// same information belonging to the process, such as virtual memory handler, process
/// identification (called pid) and etc.
///
/// We use four types of regions to maintain the task metadata:
/// - Shared with other tasks and immutable: uses [`Arc<T>`]
/// - Shared with other takss and mutable: uses [`Arc<SpinLock<T>>`]
/// - Local and immutable: data initialized once when task created
/// - Local and mutable fields that might be changed by other harts: uses [`SpinLock<TaskLockedInner>`] to wrap
/// the data together
/// - Local and mutable files that cannot be accessed by multiple harts at the same time: uses
/// [`SyncUnsafeCell<TaskInner>`]
pub struct Task {
    /* Local and immutable */
    /// Name of this task (for debug).
    pub name: String,

    /// Kernel stack identification.
    pub kstack: usize,

    /// Task identification.
    pub tid: usize,

    /// Trapframe physical address.
    pub trapframe_pa: PhysAddr,

    /* Shared and immutable */
    /// Process identification.
    ///
    /// Use `Arc` to track the ownership of pid. If all tasks holding
    /// this pid exit and parent process release the resources through `wait()`,
    /// the pid will be released.
    pub pid: Arc<PID>,

    /* Shared and mutable */
    /// Task identification allocator.
    pub tid_allocator: Arc<SpinLock<RecycleAllocator>>,

    /// Address space metadata.
    pub mm: Arc<SpinLock<MM>>,

    /// File descriptor table.
    pub fd_manager: Arc<SpinLock<FDManager>>,

    /// Signal actions.
    pub sig_actions: Arc<SpinLock<SigActions>>,

    /* Local and mutable */
    /// Inner data wrapped by [`SpinLock`].
    pub locked_inner: SpinLock<TaskLockedInner>,

    /// Inner data wrapped by [`SyncUnsafeCell`].
    pub inner: SyncUnsafeCell<TaskInner>,
}

impl Task {
    /// Create a new task from ELF data.
    pub fn new(dir: String, elf_data: &[u8], args: Vec<String>) -> KernelResult<Self> {
        // Get task name
        let name = args.join(" ");

        // Init address space
        let mut mm = MM::new()?;
        let sp = from_elf(elf_data, args, &mut mm)?;
        trace!("\nTask [{}]\n{:#?}", &name, mm);

        // New process identification
        let pid = pid_alloc();

        // New kernel stack for user task
        let kstack = kstack_alloc();
        let kstack_base = kstack_vm_alloc(kstack)?;

        // Init trapframe
        let trapframe_base: VirtAddr = trapframe_base(MAIN_TASK).into();
        mm.alloc_write_vma(
            None,
            trapframe_base,
            trapframe_base + PAGE_SIZE,
            PTEFlags::READABLE | PTEFlags::WRITABLE,
            Arc::new(SpinLock::new(FixedPMA::new(1)?)),
        )?;
        let trapframe_pa = mm.translate(trapframe_base)?;
        let trapframe = TrapFrame::from(trapframe_pa);
        *trapframe = TrapFrame::new(
            KERNEL_MM.lock().page_table.satp(),
            kstack_base,
            user_trap_handler as usize,
            mm.entry.value(),
            sp.into(),
            // CPU id will be saved when the user task is restored.
            usize::MAX,
        );

        // Init file descriptor table
        let fd_manager = FDManager::new();

        let task = Self {
            kstack,
            tid: MAIN_TASK,
            trapframe_pa,
            inner: SyncUnsafeCell::new(TaskInner {
                exit_code: 0,
                ctx: TaskContext::new(user_trap_return as usize, kstack_base),
                set_child_tid: 0,
                clear_child_tid: 0,
                curr_dir: dir,
                sig_pending: SigPending::new(),
                sig_blocked: SigSet::new(),
            }),
            locked_inner: SpinLock::new(TaskLockedInner {
                state: TaskState::RUNNABLE,
                sleeping_on: None,
                parent: None,
                children: Vec::new(),
            }),
            pid: Arc::new(PID(pid)),
            tid_allocator: Arc::new(SpinLock::new(RecycleAllocator::new(MAIN_TASK + 1))),
            mm: Arc::new(SpinLock::new(mm)),
            fd_manager: Arc::new(SpinLock::new(fd_manager)),
            sig_actions: Arc::new(SpinLock::new(SigActions::new())),
            name,
        };
        Ok(task)
    }

    /// Returns the [`TrapFrame`] of this task
    pub fn trapframe(&self) -> &'static mut TrapFrame {
        TrapFrame::from(self.trapframe_pa)
    }

    /// Mutable access to [`TaskInner`].
    pub fn inner(&self) -> &mut TaskInner {
        unsafe { &mut *self.inner.get() }
    }

    /// Acquires inner lock to modify the metadata in [`TaskLockedInner`].
    pub fn locked_inner(&self) -> SpinLockGuard<TaskLockedInner> {
        self.locked_inner.lock()
    }

    /// Gets the reference of a file object by file descriptor `fd`.
    pub fn get_file(&self, fd: usize) -> KernelResult<Arc<dyn File>> {
        let fd_manager = self.fd_manager.lock();
        fd_manager.get(fd)
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        kstack_dealloc(self.kstack);
        // We don't release the memory resource occupied by the kernel stack.
        // This memory area might be used agian when a new task calls for a
        // new kernel stack.
        self.tid_allocator.lock().dealloc(self.tid);
    }
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Task [{} {}:{}]", self.name, self.pid.0, self.tid)
    }
}

/// Returns trapframe base of the task in the address space by task identification.
///
/// Trapframes are located right below the Trampoline in each address space.
pub fn trapframe_base(tid: usize) -> usize {
    TRAMPOLINE_VA - PAGE_SIZE - tid * PAGE_SIZE
}

/// Returns task stack layout [top, base) by task identification.
///
/// Stack grows from high address to low address.
pub fn ustack_layout(tid: usize) -> (usize, usize) {
    let ustack_base = USER_STACK_BASE - tid * (USER_STACK_SIZE + PAGE_SIZE);
    let ustack_top = ustack_base - USER_STACK_SIZE;
    (ustack_top, ustack_base - ADDR_ALIGN)
}

/* Syscall helpers */

impl Task {
    /// A helper for [`syscall_interface::SyscallProc::mmap`].
    ///
    /// TODO: MAP_SHARED and MAP_PRIVATE
    pub fn do_mmap(
        &self,
        hint: VirtAddr,
        len: usize,
        prot: MmapProt,
        flags: MmapFlags,
        fd: usize,
        off: usize,
    ) -> SyscallResult {
        trace!(
            "MMAP {:?}, 0x{:X?} {:#?} {:#?} 0x{:X} 0x{:X}",
            hint,
            len,
            prot,
            flags,
            fd,
            off
        );

        if len == 0
            || !hint.is_aligned()
            || !(hint + len).is_aligned()
            || hint + len > VirtAddr::from(LOW_MAX_VA)
            || hint == VirtAddr::zero() && flags.contains(MmapFlags::MAP_FIXED)
        {
            return Err(Errno::EINVAL);
        }

        let mut mm = self.mm.lock();
        if mm.map_count() >= MAX_MAP_COUNT {
            return Err(Errno::ENOMEM);
        }

        // Find an available area by kernel.
        let anywhere = hint == VirtAddr::zero() && !flags.contains(MmapFlags::MAP_FIXED);

        // Handle different cases indicated by `MmapFlags`.
        if flags.contains(MmapFlags::MAP_ANONYMOUS) {
            if fd as isize == -1 && off == 0 {
                if let Ok(start) = mm.alloc_vma(hint, hint + len, prot.into(), anywhere, None) {
                    return Ok(start.value());
                } else {
                    return Err(Errno::ENOMEM);
                }
            }
            return Err(Errno::EINVAL);
        }

        // Map to backend file.
        if let Ok(file) = self.fd_manager.lock().get(fd) {
            if !file.is_reg() || !file.read_ready() {
                return Err(Errno::EACCES);
            }
            if let Some(_) = file.seek(off, vfs::SeekWhence::Set) {
                let backend = BackendFile::new(file, off);
                if let Ok(start) =
                    mm.alloc_vma(hint, hint + len, prot.into(), anywhere, Some(backend))
                {
                    return Ok(start.value());
                } else {
                    return Err(Errno::ENOMEM);
                }
            } else {
                return Err(Errno::EACCES);
            }
        } else {
            return Err(Errno::EBADF);
        }

        // Invalid arguments or unimplemented cases
        // flags contained none of MAP_PRIVATE, MAP_SHARED, or MAP_SHARED_VALIDATE.
        // Err(Errno::EINVAL)
    }

    /// Gets the directory name from a file descriptor.
    pub fn get_dir(&self, dirfd: usize) -> KernelResult<Path> {
        if dirfd == AT_FDCWD {
            Ok(Path::new(self.inner().curr_dir.as_str()))
        } else {
            let dir = self.fd_manager.lock().get(dirfd)?;
            if dir.is_dir() {
                Ok(dir.get_path().unwrap())
            } else {
                Err(KernelError::Errno(Errno::ENOTDIR))
            }
        }
    }

    /// Resolves absolute path with directory file descriptor and pathname.
    ///
    /// If the pathname is relative, then it is interpreted relative to the directory
    /// referred to by the file descriptor dirfd .
    ///
    /// If pathname is relative and dirfd is the special value [`AT_FDCWD`], then pathname
    /// is interpreted relative to the current working directory of the calling process.
    ///
    /// If pathname is absolute, then dirfd is ignored.
    pub fn resolve_path(&self, dirfd: usize, pathname: String) -> KernelResult<Path> {
        if pathname.starts_with("/") {
            Ok(Path::new(pathname.as_str()))
        } else {
            let mut path = self.get_dir(dirfd)?;
            path.extend(pathname.as_str());
            Ok(path)
        }
    }

    /// A helper for [`syscall_interface::SyscallFile::openat`].
    pub fn do_open(
        &self,
        dirfd: usize,
        pathname: *const u8,
        flags: OpenFlags,
        mode: Option<StatMode>,
    ) -> KernelResult<usize> {
        if flags.contains(OpenFlags::O_CREAT) && mode.is_none()
            || flags.contains(OpenFlags::O_WRONLY | OpenFlags::O_RDWR)
        {
            return Err(KernelError::Errno(Errno::EINVAL));
        }

        let mut mm = self.mm.lock();
        let path = self.resolve_path(dirfd, mm.get_str(VirtAddr::from(pathname as usize))?)?;

        trace!("OPEN {:?} {:?}", path, flags);

        self.fd_manager
            .lock()
            .push(open(path, flags).map_err(|errno| KernelError::Errno(errno))?)
    }

    /// A helper for [`syscall_interface::SyscallFile::readv`] and [`syscall_interface::SyscallFile::writev`].
    pub fn for_each_iov(
        &self,
        iov: VirtAddr,
        iovcnt: usize,
        mut op: impl FnMut(usize, usize) -> bool,
    ) -> KernelResult {
        let size = size_of::<IoVec>();
        let mut mm = self.mm.lock();
        let buf = mm.get_buf_mut(iov, iovcnt * size)?;
        for bytes in buf.into_iter().step_by(size) {
            let iov = unsafe { &*(bytes as *const IoVec) };
            if !op(iov.iov_base, iov.iov_len) {
                break;
            }
        }
        Ok(())
    }

    /// A helper for [`syscall_interface::SyscallFile::unlinkat`].
    pub fn do_unlinkat(&self, dirfd: usize, pathname: *const u8, flags: usize) -> KernelResult {
        if flags == AT_REMOVEDIR {
            unimplemented!()
        } else if flags == 0 {
            let mut mm = self.mm.lock();
            let path = self.resolve_path(dirfd, mm.get_str(VirtAddr::from(pathname as usize))?)?;

            trace!("UNLINKAT {:?}", path);

            unlink(path).map_err(|err| KernelError::Errno(err))
        } else {
            Err(KernelError::InvalidArgs)
        }
    }
}

/* Signal Helpers */

impl Task {
    pub fn send_sig_info(sig: usize, info: &SigInfo) -> SyscallResult {
        if (sig as isize) < 0 || sig > NSIG {
            return Err(Errno::EINVAL);
        }

        Ok(0)
    }
}

/* Sleep lock */

impl kernel_sync::SleepLockSched for TaskLockedInner {
    unsafe fn sched(guard: SpinLockGuard<Self>) {
        // Lock might be released after the task is pushed back to the scheduler.
        TASK_MANAGER.lock().add(curr_task().take().unwrap());
        drop(guard);

        __switch(curr_ctx(), idle_ctx());
    }

    fn set_id(task: &mut Self, id: Option<usize>) {
        task.sleeping_on = id;
    }

    fn sleep(task: &mut Self) {
        task.state = TaskState::INTERRUPTIBLE;
    }

    /// Wakes up tasks sleeping on this lock.
    fn wakeup(id: usize) {
        TASK_MANAGER.lock().iter().for_each(|task| {
            let mut inner = task.locked_inner();
            if inner.state == TaskState::INTERRUPTIBLE
                && inner.sleeping_on.is_some()
                && inner.sleeping_on.unwrap() == id
            {
                inner.state = TaskState::RUNNABLE;
            }
        });
    }
}
