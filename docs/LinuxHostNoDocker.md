# Table of Contents

- [Table of Contents](#table-of-contents)
- [Install Dependencies](#install-dependencies)
- [System Packages](#system-packages)
- [Install Rust](#install-rust)
- [Test Rust](#test-rust)
- [Install VirtualBox](#install-virtualbox)
  - [Install On Ubuntu, Debian, SUSE](#install-on-ubuntu-debian-suse)
  - [Install On Fedora, RHEL](#install-on-fedora-rhel)
  - [Test VirtualBox](#test-virtualbox)


On Linux, installing the Rust toolchain and additional utilities we need is extremely
straightforward.

# Install Dependencies

The dependencies we need to install are:

- A Linker: `ld` or `ldd`
- The `clang` compiler
- `git`
- The Rust programming language
- (Optional) VirtualBox, for building and running Windows tutorials

#  System Packages

We can install `git`, `ld/ldd`, and `clang` from our system packages.

For `Ubuntu`, `Debian`, and other `Debian`-based systems:

```sh
$ sudo apt-get update -y && apt-get install -y git gcc g++ build-essential clang
```

For `Fedora`, `RHEL`, and other RPM-based systems:

```sh
$ sudo dnf install gcc g++ clang ldd
```

# Install Rust

To install Rust, run the installer from [rustup.rs](https://rustup.rs):

```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

# Test Rust

To check that your rust install is working, run:

```sh
$ cargo new --bin /tmp/rust-test
$ cargo run --manifest-path /tmp/rust-test/Cargo.toml
```

You should see output like:

```sh
Compiling rust-test v0.1.0 (/tmp/rust-test)
Finished dev [unoptimized + debuginfo] target(s) in 0.50s
    Running `/tmp/rust-test/target/debug/rust-test`
Hello, world!
```

If you do, your rust installation is complete!

# Install VirtualBox

When running on a Linux host, we will use `VirtualBox` run a Windows system for later
exercises for fuzzing Windows targets and on Windows platforms.

## Install On Ubuntu, Debian, SUSE

On `Ubuntu`, `Debian`, and `SUSE` host machines, you can install `VirtualBox` using the
instructions from [VirtualBox](https://www.virtualbox.org/wiki/Linux_Downloads).

## Install On Fedora, RHEL

On Fedora and RHEL, you can install `VirtualBox` with
[RPMFusion](https://rpmfusion.org/Howto/VirtualBox#Quick_install).

## Test VirtualBox

After installing `VirtualBox`, you can test it by running:

```sh
$ vboxmanage --version
```

If the command shows the version of `VirtualBox`, you are done! If not, please check the
[troubleshooting](./Troubleshooting.md) guide.
