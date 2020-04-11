# Lab2

## msh

msh is a shell written in Rust.

### Get Started

To run msh, get into `lab2` dir, build and run it with:

```bash
# Debug
cargo run
# Release
cargo run --release
```

The binary will lie as `target/debug/msh` or `target/release/msh` respectively.

### Features

- Piping
  - Single: `echo test | grep test`
  - Multi: `echo test | cat -n | grep test`
- Redirection: `>`, `<`, and `>>`
- Ctrl-C to abort by SIGINT
- Multiline Quoted str: `echo 'test<Enter>test'`
  - And Diff ' with "
- Variable: `echo $HOME`
- Tilde: `echo ~`
  - Also support other users: `echo ~root`

### Missed

- For struct problem, `$[varname]` in `''` will still be replace
- Also for struct problem, `<< EOF` and `<<<` missed
- Also, multi stdin: `echo test | cat < test.txt < test.md` missed

Use crate `os_pipe` may fix them but time is up.

- TCP redirection missed

Just Currently.

Others are not planned.

## strace

### `mmap`

`man mmap` gives:

> The mmap() function shall establish a mapping between an address space of a process and a memory object.

Belongs to memory management.

### `pread64`

`man pread64` gives:

> pread, pwrite - read from or write to a file descriptor at a given offset
>
> pread() reads up to count bytes from file descriptor fd at offset offset (from the start of the file) into the buffer starting at buf.  The file offset is not changed.

Belongs to file I/O management.

### `arch_prctl`

`man arch_prctl` gives:

> arch_prctl - set architecture-specific thread state
>
> arch_prctl()  sets architecture-specific process or thread state.  code selects a subfunction and passes argument addr to it; addr is interpreted as either an unsigned long
> for the "set" operations, or as an unsigned long *, for the "get" operations.

Belongs to process management.

In addition, as Linux uses 1-1 threading model, there is only semantic difference between process and thread.
In understanding, treating them the same is helpful.
