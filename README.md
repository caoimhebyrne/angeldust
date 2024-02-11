## `angeldust` 🌸 ✨

A Rust mini-kernel (maybe operating system at some point) for the Raspberry Pi 3B (through QEMU) and Raspberry Pi 4B (tested on real hardware).

### Building & Running

**1.** Install QEMU via your package manager of choice (optional!):
```
$ brew install qemu       # macOS
$ sudo dnf install qemu   # Fedora Linux
```

**2.** Install ``rust`` & ``rustup``, if you haven't already:
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
*(taken from [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install))*
    

**3.** Install the `aarch64-unknown-none-softfloat` target, if you haven't already:
```
$ rustup target add aarch64-unknown-none-softfloat
```

**4.** You can now use `cargo run` to run angeldust on a Raspberry Pi 3B in QEMU. 

*(TODO: Add script for copying to SD card)*

### License

This project uses the [MIT license](./LICENSE).