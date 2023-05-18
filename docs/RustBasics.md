# Table of Contents

- [Table of Contents](#table-of-contents)
- [Rust Lexicon](#rust-lexicon)
- [Rust Tooling](#rust-tooling)
  - [Rustup](#rustup)
  - [Cargo](#cargo)
- [Creating A Library Crate](#creating-a-library-crate)
  - [Cargo.toml](#cargotoml)
  - [lib.rs](#librs)
    - [Function Declarations](#function-declarations)
    - [Implicit Returns and Expression-Oriented Language](#implicit-returns-and-expression-oriented-language)
    - [Attributes](#attributes)
    - [Modules](#modules)
    - [Use Declarations](#use-declarations)
    - [Test Annotations and Macros](#test-annotations-and-macros)
- [Creating A Binary Crate](#creating-a-binary-crate)
  - [main.rs](#mainrs)

# Rust Lexicon

Rust has a few terms we should get familiar with before we get started using it. For any
language-specific terminology not listed here, you can consult the
[Rust glossary](https://doc.rust-lang.org/reference/glossary.html)

- `crate`: A `crate` is to Rust as a package is to Python or Java, or a library for
  C/C++. Unlike C/C++, a `crate` can be a library *or* a binary. Crates can have
  dependencies, and can import and export entities including functions, structures,
  types and enumerations.

# Rust Tooling

When we install Rust, it comes with several command line tools by default, which we will
use for various things.

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
- `cargo test`: Run tests in the current crate.
- `cargo rustc`: Pass arguments directly to `rustc`

# Creating A Library Crate

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

## Cargo.toml

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

## lib.rs

We created a Library crate when we passed `--lib` to `cargo new` earlier, and the
`src/lib.rs` file is the top level file in the library crate we have created. Cargo
gives us some default contents of this file:

```rust
$ cat /tmp/test-crate/src/lib.rs 
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

Generally when creating a crate, we will delete these defaults, but first let's take a
look at them. This short bit of code contains a lot of ideas that may be new to you if
you are not already a Rust programmer.

### Function Declarations

First, we have a function declaration `add`. This function is declared *public*, meaning
it is exported from our library. `pub fn` declares a publicly exported function, whereas
`fn` declares a private (to this *module*, more on that later) function. The function
takes two arguments, `left` and `right`, both of type `usize`. In Rust, type
declarations come after variable or parameter names, unlike C/C++ but similar to Python.
The function also returns a value of type `usize`, denoted by the "`->`" arrow.  After
that, we have an open brace to signify the start of the function body. 

### Implicit Returns and Expression-Oriented Language

The function body simply returns `left + right`, but if you are coming from C/C++ you
may notice we don't have a `return` statement. Before we get to that, we need a little
background. In Rust, it is important to understand the difference between *statements*
and *expressions*. In general, almost everything in the Rust programming language is an
*expression* unless it has a semicolon (`;`) after it. *Expressions* have *values*. In this
case, `left + right` is an expression with a value equal to the sum of the two numbers.
This leads us back to the lack of `return` statement. In Rust, the value of a *block*, or
anything inside a set of curly braces (`{ ... }`), is equal to the last expression in
that block. In this case, the *block* of the function's curly braces has a value equal to
the last *expression* `left + right` in that block, so the return is implicit.

You can read more about Rust's *expression-oriented* design in the
[Statements and expressions](https://doc.rust-lang.org/reference/statements-and-expressions.html)
section of the documentation.

### Attributes

Now that we understand the function declaration, let's look at the next section of code.

The first thing you'll notice is the `#[cfg(tests)]` line. This syntax is called an
*attribute*, and can be found in many places in the Rust language. In general,
*attributes* describe the next non-attribute construct after them in the source code, so
in this case `#[cfg(test)]` describes `mod tests`. The `cfg` attribute allows us to
conditionally compile code based on the current *build configuration*. In this case, we
are telling the compiler to only include `mod tests` if we are building in test mode,
i.e. we are running `cargo test`. You can read more about the `cfg` attribute
[here](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute)
and more about attributes in general
[here](https://doc.rust-lang.org/reference/attributes.html).

### Modules

The `mod tests` line declares a *module*. In Rust, modules are smaller units of
functionality inside a crate. Splitting a crate into discrete logical modules can be
useful for users of the crate as well as developers, by siloing functionality where it
makes sense to do so. You can read about modules
[here](https://doc.rust-lang.org/reference/items/modules.html). We will use modules in
various capacities throughout this workshop, but for now you can think of them as a way
to segment functionality similar to a `namespace` in C++, or a module in Python.

### Use Declarations

Inside the `tests` module, we have a `use` declaration, which is similar to an `import`
statement in Python or an `#include` statement in C/C++. You can read about `use`
declarations [here](https://doc.rust-lang.org/reference/items/use-declarations.html).
Here, we are `use`-ing `super::*`, which means everything (`*`) from the outer module
(`super`). In this case, the outer module is everything in `lib.rs`, specifically `fn
add`. The `*` means everything, but if we only wanted to `use` the `add` function we
could write `use super::add;` instead.

### Test Annotations and Macros

Finally, we have a function annotated with the `#[test]` attribute, which means it will
be run as a unit test case. This function makes sure that `add(2, 2)` equals `4`. Note
that `assert_eq!` has an exclamation point (`!`) in its name. This means it is a
*function-like macro*, not a function in the same way `add` is. Macros are used many
places in Rust, and are very powerful. We will encounter more macros as we progress, but
you can read about them [here](https://doc.rust-lang.org/reference/macros.html).

# Creating A Binary Crate

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
