# Triage The Crash

Run the fuzzer a few times and confirm you see an objective each time. We can then view
the crashes in the `solutions` directory:

```sh
$ ls solutions
26c0aa3cb05a20f7  6b47dcc6e8baf69e  6f40a2bea93ee0a3  71fd31f6e02fb1d5  a3ee050a83870a18  e54c970569a44481  e552cbba2b3e2764
```

We'll create a test case for all these crashes. Create a `testcases` directory in your
`first-target` crate, and copy all the inputs into it:

```sh
$ cd /path/to/first-target
$ mkdir testcases
$ cp ../first-fuzzer/solutions/* ./testcases
```


Then, add a test case to `mod tests` in your `lib.rs`:


```rust
    #[test]
    fn it_crashed_by_fuzzer() {
        let test = include_bytes!("../testcases/26c0aa3cb05a20f7");
        decode(test);
        let test = include_bytes!("../testcases/6b47dcc6e8baf69e");
        decode(test);
        let test = include_bytes!("../testcases/6f40a2bea93ee0a3");
        decode(test);
        let test = include_bytes!("../testcases/71fd31f6e02fb1d5");
        decode(test);
        let test = include_bytes!("../testcases/a3ee050a83870a18");
        decode(test);
        let test = include_bytes!("../testcases/e54c970569a44481");
        decode(test);
        let test = include_bytes!("../testcases/e552cbba2b3e2764");
        decode(test);
    }
```

Running `cargo test` should crash with these inputs (you can separate them into individual
test functions to narrow down which inputs are problematic), and as a developer you could
then debug your program to find the root cause using these crashing inputs.
