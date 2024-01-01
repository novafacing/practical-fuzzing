# Binary Crates

We've already created a Library crate, but we will also go ahead and create a Binary
crate. You can change a Library crate to a Binary crate after you create it, the only
difference is the default files `cargo` creates for you.

We'll create a new Binary crate with the `cargo new` command, again in `/tmp`.

```sh
$ cd /tmp/
$ cargo new --bin /tmp/test-crate-bin/
$ ls -R /tmp/test-crate-bin/
/tmp/test-crate-bin:
Cargo.toml  src

/tmp/test-crate-bin/src:
main.rs
```

## main.rs

Notice that this time, instead of a `lib.rs` file, we got a `main.rs` file. This `main`
file is much less complex than the default Library file:

```sh
$ cat /tmp/test-crate-bin/src/main.rs 
fn main() {
    println!("Hello, world!");
}
```

Here, we have a `main` function (which is the default entrypoint of Rust programs the
same way as in C/C++) that prints "Hello, world!". Notice that like `assert_eq!`,
`println!` is a macro.

