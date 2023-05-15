# Rust Tooling

Rust comes with several command line tools by default, which we will use for various
things.

## Rustup

Rustup is run when we first install Rust, and generally you don't need to interact with
it very often. It performs first installation of the Rust toolchain on your system and
makes it easy to update or install alternate toolchains. We will only use it to install
specific toolchains for some more exotic targets like Windows.

## Cargo

The bedrock of the Rust ecosystem (for our purposes, at least) is `cargo`, the Rust
package manager. It is installed by default when you install Rust, and is responsible
for managing:

- Dependencies
- Build configurations and compilation
- Workspaces and repository layout
- Other Rust development tooling, as a frontend

`cargo` has a very robust help dialog, but the sub-commands we will use the most are:

- `cargo add`: Add a dependency to our *crate*, which is the name Rust uses for a package
  whether it is a library or binary.
- `cargo build`: Build the project and compile our code together with all our
  dependencies
- `cargo run`: Run our project's binary (or binaries)
- `cargo install`: Install a *binary* Rust package. Several developer tools we will use
  can be installed this way.
- `cargo check` or `cargo clippy`: We can run `check` to check for warnings and errors
  in our code, or `clippy` to check for even *more* warnings and errors.
- `cargo new` or `cargo init`: Create a new crate or initialize a directory as a crate.