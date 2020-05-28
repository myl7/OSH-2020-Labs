use nix::sched::{clone, CloneFlags};
use nix::sys::mman::{mmap, MapFlags, ProtFlags};
use nix::sys::signal::Signal;
use nix::sys::wait::{waitpid, WaitStatus};
use std::ptr::null_mut;

const CHILD_STACK_SIZE: usize = 1000 * 1000;

unsafe fn isolated_clone<F>(child_main: Box<dyn FnOnce() -> ()>) -> WaitStatus {
    let child_stack = mmap(
        null_mut(),
        CHILD_STACK_SIZE,
        ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
        MapFlags::MAP_PRIVATE | MapFlags::MAP_ANONYMOUS | MapFlags::MAP_STACK,
        -1,
        0,
    )
    .unwrap();

    let child_stack_start = child_stack.offset(CHILD_STACK_SIZE as isize);

    let child = clone(
        child_main,
        (child_stack_start as *mut [u8]).as_mut().unwrap(),
        CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWIPC
            | CloneFlags::CLONE_NEWCGROUP,
        Some(Signal::SIGCHLD as i32),
    )
    .unwrap();

    return waitpid(child, None).unwrap();
}
