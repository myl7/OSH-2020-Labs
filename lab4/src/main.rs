extern crate libc;
extern crate nix;
extern crate rand;

pub mod cntr;
pub mod host;

use std::env::current_dir;
use std::fs::read_dir;

fn main() {
    let (fd1, fd2) = host::mkpipe();
    let tmpdir = host::mktmpdir();
    let tmpdir_cntr = tmpdir.clone();

    let child = Box::new(move || {
        let tmpdir = tmpdir_cntr.as_str();

        let cwd = current_dir().unwrap();
        let root = cwd.to_str().unwrap();

        cntr::mount_all(root, tmpdir);
        cntr::mknod_all(root);
        cntr::pivot_root(tmpdir);
        cntr::umount_host("/oldroot");
        cntr::req_umount_bind(fd2);

        let files = read_dir("/")
            .unwrap()
            .map(|p| p.unwrap().path().to_str().unwrap().to_string())
            .collect::<Vec<String>>();

        println!("{:?}", files);

        0
    });

    let (pid, stack) = unsafe { host::isolated_clone(child) };

    host::umount_bind(tmpdir.as_str(), fd1);

    unsafe {
        host::isolated_wait(pid, stack);
    }
}
