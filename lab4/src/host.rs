use libc::c_void;
use nix::mount::{umount2, MntFlags};
use nix::sched::{clone, CloneFlags};
use nix::sys::mman::{mmap, munmap, MapFlags, ProtFlags};
use nix::sys::signal::Signal;
use nix::sys::wait::waitpid;
use nix::unistd::{pipe, Pid};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::{create_dir, remove_dir, File};
use std::io::{Read, Write};
use std::os::unix::io::{FromRawFd, RawFd};
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

pub fn mkpipe() -> (RawFd, RawFd) {
    pipe().unwrap()
}

pub fn umount_bind(bind_dir: &str, read_fd: RawFd) {
    let mut f: File = unsafe { File::from_raw_fd(read_fd) };

    let mut buf: [u8; 1] = [0; 1];

    f.read(&mut buf).unwrap();
    if buf[0] != 0 {
        panic!();
    }

    umount2(bind_dir, MntFlags::MNT_DETACH).unwrap();

    remove_dir(bind_dir).unwrap();
}

const CGROUP_NAME: &'static str = "cntr";
const MEM_LIMIT: u32 = 67108864;
const KMEM_LIMIT: u32 = 67108864;
const SWAP_LIMIT: u32 = 0;
const CPU_LIMIT: u32 = 256;
const PID_NUM_LIMIT: u32 = 64;

pub fn create_cgroup() {
    let mem_cgroup = String::from("/sys/fs/cgroup/memory/") + CGROUP_NAME;

    create_dir(&mem_cgroup).unwrap();

    let mut f = File::create(mem_cgroup.clone() + "/memory.limit_in_bytes").unwrap();
    f.write_all((MEM_LIMIT.to_string() + "\n").as_bytes())
        .unwrap();

    let mut f = File::create(mem_cgroup.clone() + "/memory.kmem.limit_in_bytes").unwrap();
    f.write_all((KMEM_LIMIT.to_string() + "\n").as_bytes())
        .unwrap();

    let mut f = File::create(mem_cgroup.clone() + "/memory.swappiness").unwrap();
    f.write_all((SWAP_LIMIT.to_string() + "\n").as_bytes())
        .unwrap();

    let cpu_cgroup = String::from("/sys/fs/cgroup/cpu/") + CGROUP_NAME;

    create_dir(&cpu_cgroup).unwrap();

    let mut f = File::create(cpu_cgroup.clone() + "/cpu.shares").unwrap();
    f.write_all((CPU_LIMIT.to_string() + "\n").as_bytes())
        .unwrap();

    let pid_num_cgroup = String::from("/sys/fs/cgroup/pids/") + CGROUP_NAME;

    create_dir(&pid_num_cgroup).unwrap();

    let mut f = File::create(pid_num_cgroup.clone() + "/pids.max").unwrap();
    f.write_all((PID_NUM_LIMIT.to_string() + "\n").as_bytes())
        .unwrap();
}

pub fn apply_cgroup_limit(pid: Pid) {
    let mem_cgroup = String::from("/sys/fs/cgroup/memory/") + CGROUP_NAME;

    let mut f = File::create(mem_cgroup + "/cgroup.procs").unwrap();
    f.write_all((pid.as_raw().to_string() + "\n").as_bytes())
        .unwrap();

    let cpu_cgroup = String::from("/sys/fs/cgroup/cpu/") + CGROUP_NAME;

    let mut f = File::create(cpu_cgroup.clone() + "/cgroup.procs").unwrap();
    f.write_all((pid.as_raw().to_string() + "\n").as_bytes())
        .unwrap();

    let pid_num_cgroup = String::from("/sys/fs/cgroup/pids/") + CGROUP_NAME;

    let mut f = File::create(pid_num_cgroup.clone() + "/cgroup.procs").unwrap();
    f.write_all((pid.as_raw().to_string() + "\n").as_bytes())
        .unwrap();
}

pub fn clear_cgroup_limit() {
    let mem_cgroup = String::from("/sys/fs/cgroup/memory/") + CGROUP_NAME;

    File::create(mem_cgroup + "/cgroup.procs").unwrap();

    let cpu_cgroup = String::from("/sys/fs/cgroup/cpu/") + CGROUP_NAME;

    File::create(cpu_cgroup.clone() + "/cgroup.procs").unwrap();

    let pid_num_cgroup = String::from("/sys/fs/cgroup/pids/") + CGROUP_NAME;

    File::create(pid_num_cgroup.clone() + "/cgroup.procs").unwrap();
}
