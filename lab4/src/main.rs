extern crate libc;
extern crate nix;
extern crate rand;

pub mod cntr;
pub mod host;

use std::env::current_dir;
use std::fs::read_dir;

fn main() {
    let child = Box::new(move || {
        let tmpdir_s = host::mktmpdir();
        let tmpdir = tmpdir_s.as_str();

        let cwd = current_dir().unwrap();
        let root = cwd.to_str().unwrap();

        cntr::mount_all(root, tmpdir);
        cntr::mknod_all(root);
        cntr::pivot_root(tmpdir);
        cntr::umount_host("/oldroot");

        let files = read_dir("/")
            .unwrap()
            .map(|p| p.unwrap().path().to_str().unwrap().to_string())
            .collect::<Vec<String>>();

        println!("{:?}", files);

        0
    });

    let (pid, stack) = unsafe { host::isolated_clone(child) };

    unsafe {
        host::isolated_wait(pid, stack);
    }
}
