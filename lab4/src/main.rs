extern crate caps;
extern crate libc;
extern crate nix;
extern crate rand;
extern crate seccomp_sys;

pub mod cntr;
pub mod consts;
pub mod host;

use nix::unistd::Pid;
use std::env::current_dir;
use std::fs::{read_dir, File};
use std::io::Read;

fn main() {
    let (fd1, fd2) = host::mkpipe();

    let tmpdir = host::mktmpdir();
    let tmpdir_cntr = tmpdir.clone();

    host::create_cgroup();

    let child = Box::new(move || {
        let tmpdir = tmpdir_cntr.as_str();

        let cwd = current_dir().unwrap();
        let root = cwd.to_str().unwrap();

        cntr::mount_all(root, tmpdir);
        cntr::mknod_all(root);
        cntr::pivot_root(tmpdir);
        cntr::umount_host("/oldroot");
        cntr::req_umount_bind(fd2);
        cntr::limit_caps();
        cntr::limit_syscall();

        let files = read_dir("/")
            .unwrap()
            .map(|p| p.unwrap().path().to_str().unwrap().to_string())
            .collect::<Vec<String>>();

        println!("{:?}", files);

        let mut s = String::new();
        let mut f = File::open("/proc/1/cgroup").unwrap();
        f.read_to_string(&mut s).unwrap();
        println!("{}", &s);

        0
    });

    let host_pid = Pid::this();

    host::apply_cgroup_limit(host_pid);

    let (pid, stack) = unsafe { host::isolated_clone(child) };

    host::clear_cgroup_limit();
    host::apply_cgroup_limit(pid);

    host::umount_bind(tmpdir.as_str(), fd1);

    unsafe {
        host::isolated_wait(pid, stack);
    }
}
