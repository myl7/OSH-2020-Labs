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

## x86 bare metal MBR program

## Questions

[linux-src]: https://www.kernel.org/
