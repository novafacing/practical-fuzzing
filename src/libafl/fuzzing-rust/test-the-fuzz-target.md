# Test the Fuzz Target

Like any good developers, we will create a few unit tests for our code before we worry
about fuzzing or any additional security testing.

We can add our tests directly in `lib.rs`. The `#[cfg(test)]` annotation ensures this
code is only compiled when testing (i.e. it will not take up any space in any final
binaries using this code). The `#[test]` annotation marks each function as a test entry
point, so the Cargo test harness will run each in a separate thread by default.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decodes_successfully_emoji() {
        let expected = String::from("👾 Exterminate!").as_bytes().to_vec();
        let encoded = "%F0%9F%91%BE%20Exterminate%21".as_bytes();
        assert_eq!(expected, decode(encoded));
    }

    #[test]
    fn it_decodes_misc() {
        assert_eq!("".as_bytes().to_vec(), decode("".as_bytes()));
        assert_eq!(
            "pureascii".as_bytes().to_vec(),
            decode("pureascii".as_bytes())
        );
        assert_eq!("\0".as_bytes().to_vec(), decode("\0".as_bytes()));
        assert_eq!(
            "this that".as_bytes().to_vec(),
            decode("this%20that".as_bytes())
        );
    }

    // #[test]
    // fn it_crashes() {
    //     println!("{:?}", decode("aaaaaaaaaaaaaaaa%%%%%%%%%%%%".as_bytes()));
    // }
}
```

Note that we have a third test here commented out. Run `cargo test` in your crate.

```sh
$ cargo test
   Compiling first-targetv0.1.0 (/workspaces/documentation.security.fuzzing.libafl/first-target)
    Finished test [unoptimized + debuginfo] target(s) in 0.33s
     Running unittests src/lib.rs (target/debug/deps/first_target-49e422aa7de12c55)

running 2 tests
test tests::it_decodes_misc ... ok
test tests::it_decodes_successfully_emoji ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
