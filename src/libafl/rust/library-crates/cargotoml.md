# Cargo.toml

The first file you'll notice is `Cargo.toml`, which is in `TOML` format. You can learn
more about the `TOML` format from the [TOML docs](https://toml.io/en/). This file
describes metadata about your crate, including its dependencies. You can find the full
list of everything `Cargo.toml` can contain in [the manifest format
docs](https://doc.rust-lang.org/cargo/reference/manifest.html), but we are only
concerned with a few fields at this point. Take a look at the `Cargo.toml` file:

```sh
$ cat /tmp/test-crate/Cargo.toml
[package]
name = "test-crate"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

You will notice two sections `package` and `dependencies` (which is empty for now). The
`package` section lists the name of the crate (`test-crate`), the version of the crate
in [Semantic Version](https://semver.org/) format (all Rust crates use Semantic Versions
in their `Cargo.toml`, if you aren't familiar with Semantic Versioning, you can learn about
it in the [Semantic Version Spec](https://semver.org/)), and the Rust edition the crate
uses which you can typically leave as-is.

The `dependencies` section lists dependencies of our crate. We can add a dependency by
running:

```sh
$ cd /tmp/test-crate/
$ cargo add regex
$ cat /tmp/test-crate/Cargo.toml
[package]
name = "test-crate"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
regex = "1.8.1"
```

You'll notice our `Cargo.toml` file now includes a listing for the `regex` crate as a
dependency, including the specific version of `regex` we depend on. Later, we'll explore
some slightly more advanced features of this configuration file, but for now we just
need to know that it describes our metadata and dependencies.
