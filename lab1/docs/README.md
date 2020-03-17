# Lab1

## Linux kernel

Download Linux 5.4.22 src from [The Linux Kernel Archives][linux-src].

Extract the `.tar.xz` file:

```bash
tar xf linux-5.4.22.tar.xz
```

Get into the src folder and compile the kernel.

```bash
cd linux-5.4.22
make defconfig  # Use default config according to current OS and hardware environment.
make xconfig  # As I am using KDE, use QT-based GUI config windows.
# Remove some unused functions, such as network, sound and virtualization.
make -j6  # I am usng i5-8250U, which is a 4C8T low-voltage CPU.
# Then we can found the `bzImage` file in `arch/x86/boot`, which is 2113 kB large.
```

## initrd and init

### Simple initrd

Create `init.c` with respective code.

Compile and staticly link it with `gcc`.

Create the gzip initrd with `cpio` and `gzip`.

Boot the kernel with the initrd with:

```bash
qemu-system-x86_64 -kernel bzImage -initrd initrd.cpio.gz
```

We will see:

![initrd1-qemu](./images/initrd1-qemu.png)

Which has no "Hello, Linux!".

But this is because we try to kill the init process, which caused kernel panic.
The error info full the VGA memory-mapped screen so we can not see the required output.
Later in initrd2 we will see it.

### initrd with three programs

Create the new `init.c` to use `fork`, `execl`, `waitpid` to run given programs.

Add an endless loop in the tail to prevent the exiting of init process.

create the device files.

As I am using Manjaro, which is a kind of Linux distro, I just use `mknod` to create them.

```bash
sudo mknod dev/ttyS0 c 4 64
sudo mknod dev/fb0 c 29 0
```

Then create initrd like above.

The boot and execution is successful.

## x86 bare metal MBR program

Like ordinary programing, write source, compile it and run it.

In the source, the start part is options, then code, finally data.

As nasm does not support `dup`, I use `times` to build the constants

To wrap lines, use 13 ('\r') carriage return and 10 ('\n') line break.

Print the line text every 18 updates which is similar to 18.2.

## Questions

### 1

Means that the function will be loaded with modules, and you will find the compiled modules after
kernel compilation.

No.

They will be loaded in userspace by the manager (e.g. udev).

### 2

For after that the `init` process ends, while it is the only process on the system.

If it ends, the kernel can never create a new process via `fork`, so it panic.

### 3

For we need to ensure all used instructions should be in the executable file.

No. If we do that, the call to the functions that requires dynamical library will failed,
causing exceptions.

### 4

`man fakeroot` gives that:

> fakeroot - run a command in an environment faking root privileges for file manipulation

Which also allow ones to manipulate device file, to set the major and minor.

### 5

For the kernel can parse the script with a shebang, and use the right program to run the script.

To make it, the function in kernel config should be enabled.

### 6

See [how does linux kernel prevents the BIOS system calls? - Stack Overflow][stackoverflow-1]

> The BIOS is mostly available in 16 bits mode, not in the 32 or 64 bits mode of x86 on which Linux runs.
>
> A Linux process is running in user mode, using virtual memory, it has its own virtual address space.
>
> Some machine instructions (notably INT which is used to go to BIOS) are privileged so cannot be run in user mode. If you try running them in user mode, the processor makes a machine exception, and the kernel handles it by sending some signal. (some INT is also used for syscalls, but the SYSENTER instruction is preferred).

### 7

For GRUB2, it provides a `boot.img` image which is exactly 512 bytes.
The image can be written to MBR and loaded when boot.
Then as the sole function in the image is to read the first sector of the core image
from a local disk and jump to it, GRUB2 loads other instructions to serve users.

[linux-src]: https://www.kernel.org/
[stackoverflow-1]: https://stackoverflow.com/questions/19535056/how-does-linux-kernel-prevents-the-bios-system-calls
