# bullet-sys

Bullet3 wrapper for the Rust language. Inspired by [bulletrs](https://github.com/not-fl3/bulletrs/).

## Building

Install [rustup](https://rustup.rs/).

Clone and compile project:

```bash
git clone https://github.com/HaronK/bullet-sys.git
cd ./bullet-sys
cargo build --release
```

### Linux dependencies

```bash
sudo apt install cmake libglfw3-dev libxinerama-dev libxcursor-dev libxi-dev libcsfml-dev libclang-dev
```

### Windows dependencies

 * [CMake](https://cmake.org/)
 * [MSYS2](https://www.msys2.org/)

**NOTE**: add path to the MSYS2 binaries folder to the **PATH** environment variable.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
