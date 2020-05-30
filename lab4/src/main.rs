extern crate caps;
extern crate libc;
extern crate nix;
extern crate rand;
extern crate seccomp_sys;

pub mod cntr;
pub mod consts;
pub mod host;

use nix::unistd::Pid;
use std::env::args;
use std::process::Command;

fn main() {
    let mut args = args().collect::<Vec<String>>();

    if args.len() < 3 {
        panic!("Too few arguments.");
    }

    args.remove(0);

    let root_s = args.remove(0);
    let prog = args.remove(0);

    let (fd1, fd2) = host::mkpipe();

    let tmpdir = host::mktmpdir();
    let tmpdir_s = tmpdir.clone();

    host::create_cgroup();

    let child = Box::new(move || {
        let root = root_s.as_str();
        let tmpdir = tmpdir_s.as_str();

        cntr::mount_all(root, tmpdir);
        cntr::mknod_all(root);
        cntr::pivot_root(tmpdir);
        cntr::umount_host("/oldroot");
        cntr::req_umount_bind(fd2);
        cntr::limit_caps();
        cntr::limit_syscall();

        // Debug code
        // let files = read_dir("/")
        //     .unwrap()
        //     .map(|p| p.unwrap().path().to_str().unwrap().to_string())
        //     .collect::<Vec<String>>();
        //
        // println!("{:?}", files);
        //
        // let mut s = String::new();
        // let mut f = File::open("/proc/1/cgroup").unwrap();
        // f.read_to_string(&mut s).unwrap();
        // println!("{}", &s);

        Command::new(&prog).args(&args).status().unwrap();

        0
    });

    let host_pid = Pid::this();

    host::apply_cgroup_limit(host_pid);

    let (pid, stack) = unsafe { host::isolated_clone(child) };

    host::apply_cgroup_limit(pid);

    host::umount_bind(tmpdir.as_str(), fd1);

    unsafe {
        host::isolated_wait(pid, stack);
    }

    host::remove_cgroup_limit();
}
