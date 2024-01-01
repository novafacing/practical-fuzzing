# Add the Target Crate as a Dependency

In order to link in our fuzz target and call it from our fuzzer, we need to add it as a
dependency. Run:

```sh
$ cargo add --path ../first-target
```

You should see the following line appear in your `Cargo.toml` file under
`[dependencies]`:

```toml
first-target = { version = "0.1.0", path = "../first-target" }
```

