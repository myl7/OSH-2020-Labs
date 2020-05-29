use libc::{syscall, SYS_pivot_root};
use nix::mount::{mount, umount2, MntFlags, MsFlags};
use nix::sys::stat::{makedev, mknod, Mode, SFlag};
use std::ffi::CString;
use std::fs::{create_dir, remove_dir, File};
use std::io::Write;
use std::os::unix::io::{FromRawFd, RawFd};
use std::process::Command;

const NONE: Option<&str> = None;

pub fn mount_all(root: &str, bind_root: &str) {
    // Bind mount container root to a tmpdir.
    mount(Some(root), bind_root, NONE, MsFlags::MS_BIND, NONE).unwrap();

    // Mount `/dev`, `/proc`, `/sys`, `/tmp`.
    mount(
        NONE,
        (root.to_string() + "/dev").as_str(),
        Some("tmpfs"),
        MsFlags::MS_NOSUID,
        NONE,
    )
    .unwrap();

    mount(
        NONE,
        (root.to_string() + "/proc").as_str(),
        Some("proc"),
        MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_NOEXEC | MsFlags::MS_RELATIME,
        NONE,
    )
    .unwrap();

    mount(
        NONE,
        (root.to_string() + "/sys").as_str(),
        Some("sysfs"),
        MsFlags::MS_RDONLY
            | MsFlags::MS_NOSUID
            | MsFlags::MS_NODEV
            | MsFlags::MS_NOEXEC
            | MsFlags::MS_RELATIME,
        NONE,
    )
    .unwrap();

    mount(
        NONE,
        (root.to_string() + "/tmp").as_str(),
        Some("tmpfs"),
        MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
        NONE,
    )
    .unwrap();

    // Change host root in container namespace to private.
    mount(NONE, "/", NONE, MsFlags::MS_PRIVATE | MsFlags::MS_REC, NONE).unwrap();
}

pub fn mknod_all(root: &str) {
    mknod(
        (root.to_string() + "/dev/null").as_str(),
        SFlag::S_IFCHR,
        Mode::S_IRUSR | Mode::S_IWUSR,
        makedev(1, 3),
    )
    .unwrap();

    mknod(
        (root.to_string() + "/dev/zero").as_str(),
        SFlag::S_IFCHR,
        Mode::S_IRUSR | Mode::S_IWUSR,
        makedev(1, 5),
    )
    .unwrap();

    mknod(
        (root.to_string() + "/dev/urandom").as_str(),
        SFlag::S_IFCHR,
        Mode::S_IRUSR | Mode::S_IWUSR,
        makedev(1, 9),
    )
    .unwrap();

    mknod(
        (root.to_string() + "/dev/tty").as_str(),
        SFlag::S_IFCHR,
        Mode::S_IRUSR | Mode::S_IWUSR,
        makedev(5, 0),
    )
    .unwrap();
}

pub fn pivot_root(tmp_root: &str) {
    let new_root = tmp_root.to_string();
    let put_old = new_root.clone() + "/oldroot";

    create_dir(&put_old).unwrap();

    Command::new("mount").status().unwrap();

    let c_new_root = CString::new(new_root.as_str()).unwrap();
    let c_put_old = CString::new(put_old.as_str()).unwrap();

    let ret = unsafe { syscall(SYS_pivot_root, c_new_root.as_ptr(), c_put_old.as_ptr()) };
    if ret != 0 {
        panic!();
    }
}

pub fn umount_host(put_old: &str) {
    umount2(put_old, MntFlags::MNT_DETACH).unwrap();

    remove_dir(put_old).unwrap();
}

pub fn req_umount_bind(write_fd: RawFd) {
    let mut f: File = unsafe { File::from_raw_fd(write_fd) };

    let buf: [u8; 2] = [0, '\n' as u8];

    f.write(&buf).unwrap();
}
