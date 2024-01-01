# Corpus

We will keep our working corpus in memory for efficiency, so we'll want to create
a new [`InMemoryCorpus`](https://docs.rs/libafl/latest/libafl/corpus/inmemory/struct.InMemoryCorpus.html).

Add the `use` declaration for it:

```rust
use libafl::prelude::InMemoryCorpus;
```

And create a new corpus in your `main` function:

```rust
let corpus = InMemoryCorpus::new();
```
