# Lab4: cntrs

## Build

Use Cargo to build, test and run:

```bash
# To build and test
cargo test
# To build and run
cargo run
```

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
