# Creating the Crate

To create a new crate, you can use the `cargo new` command. Let's create a crate in
`/tmp` and take a look at what `cargo` sets up for us. We'll create a Library crate.

```sh
$ cd /tmp/
$ cargo new --lib /tmp/test-crate
$ ls -R /tmp/test-crate/
/tmp/test-crate:
Cargo.toml  src

/tmp/test-crate/src:
lib.rs
```

