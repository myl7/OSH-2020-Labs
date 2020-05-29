use libc::c_void;
use nix::sched::{clone, CloneFlags};
use nix::sys::mman::{mmap, munmap, MapFlags, ProtFlags};
use nix::sys::signal::Signal;
use nix::sys::wait::waitpid;
use nix::unistd::Pid;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::create_dir;
use std::ptr::{null_mut, slice_from_raw_parts_mut};

const STACK_SIZE: usize = 1000 * 1000;

/// As chroot is not applied, the root dir has not changed.
/// Following `pivot_root` will change it.
///
/// Stack allocated by `mmap` should be returned.
/// So this is unsafe.
pub unsafe fn isolated_clone(main: Box<dyn FnMut() -> isize>) -> (Pid, *mut c_void) {
    let stack_bottom = mmap(
        null_mut(),
        STACK_SIZE,
        ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
        MapFlags::MAP_PRIVATE | MapFlags::MAP_ANONYMOUS | MapFlags::MAP_STACK,
        -1,
        0,
    )
    .unwrap();

    let stack = slice_from_raw_parts_mut(stack_bottom as *mut u8, STACK_SIZE);

    (
        clone(
            main,
            stack.as_mut().unwrap(),
            CloneFlags::CLONE_NEWPID
                | CloneFlags::CLONE_NEWNS
                | CloneFlags::CLONE_NEWUTS
                | CloneFlags::CLONE_NEWIPC
                | CloneFlags::CLONE_NEWCGROUP,
            Some(Signal::SIGCHLD as i32),
        )
        .unwrap(),
        stack_bottom,
    )
}

pub unsafe fn isolated_wait(pid: Pid, stack_bottom: *mut c_void) {
    waitpid(pid, None).unwrap();
    munmap(stack_bottom, STACK_SIZE).unwrap();
}

pub fn mktmpdir() -> String {
    let rand_s = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>();
    let tmp_root = "/tmp/root-".to_string() + rand_s.as_str();
    create_dir(&tmp_root).unwrap();
    tmp_root
}

// pub fn umount_bind(bind_dir: &str) {}
