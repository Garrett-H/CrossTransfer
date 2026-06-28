# dir_zipper

<!-- TODO: replace if crate is created -->
<!-- [![Crates.io](https://img.shields.io/crates/v/dir_zipper.svg)](https://crates.io/crates/dir_zipper) -->
<!-- [![Docs.rs](https://docs.rs/dir_zipper/badge.svg)](https://docs.rs/dir_zipper)
[![CI](https://github.com/your-username/dir_zipper/actions/workflows/ci.yml/badge.svg)](https://github.com/your-username/dir_zipper/actions)
[![License](https://img.shields.io/crates/l/dir_zipper.svg)](#license) -->

> Directory zipper is a library used to archive & compress a filesystem directory to a single zipped file.

A longer paragraph that expands on the elevator pitch. Explain the key use cases,
what makes this crate useful, and who should reach for it. Keep it focused.

## Features

- Archiving - standard **tar** format with "PAX", "xattrs", & "selinux" support
  - For windows archiving, planned support for ACL (DACL) saving via `icacls` export
- Compression - Primarily planned to use **zstd** format. Future formats will be added as needed.

## Installation
<!-- TODO 
Add this to your `Cargo.toml`:

```toml
[dependencies]
dir_zipper = "0.1"
```

Or use `cargo add`:

```sh
cargo add dir_zipper
```
 -->

## Usage
<!-- TODO
```rust
use dir_zipper::Thing;

fn main() {
    let thing = Thing::new("hello");
    println!("{}", thing.value());
}
```
 -->

## Building & Testing
<!-- TODO
```sh
# Build the library
cargo build

# Run the test suite
cargo test

# Run lints and formatting checks
cargo clippy --all-targets --all-features
cargo fmt --all -- --check

# Build the docs locally
cargo doc --open
```
 -->
## Contributing

Contributions are welcome!

1. Create your feature branch (`git checkout -b feature/my-feature`)
2. Commit your changes (`git commit -am 'Add my feature'`)
3. Push to the branch (`git push origin feature/my-feature`)
4. Open a Pull Request

## License

This project is licensed under the **GNU General Public License v3.0** — see the
[LICENSE](LICENSE) file for details, or visit
<https://www.gnu.org/licenses/gpl-3.0>.
