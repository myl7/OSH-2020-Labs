# Lab4: cntrs

## Build

Use Cargo to build, test and run:

```bash
# To build and test
cargo test
# To build and run
cargo run
```

## Impl

Ues `clone` to create the isolated child process.

Between parent and child, sync with piping, for the file system (not mount points) is not isolated.

Under `/dev`, `/dev/null`, `/dev/zero`, `/dev/urandom` and `/dev/tty` are created.

Capability management uses `caps` crate, which has been download about 20w times,
and exposed raw capability APIs like libcap, but impl them directly by syscalls.

Syscall management uses `seccomp-sys` crate, which is not hot, downloaded about 2w times,
but a raw FFI crate that is a simple wrapper of libseccomp, and has the same functions as C lib.

To limit cgroup root, for the container limitation is not strict, I first add the parent to the
cgroup, then clone out the child, and then add child to the cgroup. Using `unshare` to create a
new cgroup namespace in the child is wiser, but time is up, and current way works.

Command line argument design follows the suggestion of the doc.

## Questions

### 1

seccomp uses `mmap`, `pread64` and other memory syscalls to build a filter in the memory,
and then uses `seccomp` to set basic config

capabilities uses `capget`, `capset` to set basic config,
and then uses `prctl` to set every rule.

Both of them requires lots of syscalls to provide filter information,
so using a library is wirer.

### 2

To solve it, we can provide a new namespace for resource monitoring.
Relative to the host, monitoring syscalls of the process in the namespace can
only see the resource owned by it.
