# Solution Corpus

Unlike our working corpus, we want to keep any solutions we find, so we'll store them on
disk. We can use an
[`OnDiskCorpus`](https://docs.rs/libafl/latest/libafl/corpus/ondisk/struct.OnDiskCorpus.html)
to store solutions (crashes) that we discover. We'll add the `use` declaration:

```rust
use libafl::prelude::OnDiskCorpus;
```

We'll create our corpus at the path we specify in our program arguments, and panic with
an appropriate error message if the operation fails.

```rust
let solutions = OnDiskCorpus::new(&args.solutions).unwrap_or_else(|e| {
    panic!(
        "Unable to create OnDiskCorpus at {}: {}",
        args.solutions.display(),
        e
    )
});
```