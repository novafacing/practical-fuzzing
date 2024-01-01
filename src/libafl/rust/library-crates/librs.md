# lib.rs

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
you are not already a Rust programmer, and we'll cover them all in this tutorial.
