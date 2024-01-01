# Add An Argument Parser

We want to be able to easily pass in a path to our input corpus of test cases as well
as a directory the fuzzer should put *solutions* (or crashing inputs) when it finds them.

We can implement argument parsing easily with the `clap` crate (documentation
[here](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)). Clap is super
flexible and powerful, but we'll just add two arguments, the two paths we mentioned
above. Add this to your `main.rs` above your main function.

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    /// Corpus directory
    corpus: PathBuf,
    #[arg(short, long)]
    /// Solutions directory
    solutions: PathBuf,
}
```

Now add the first line of our `main` function:

```rust
fn main() {
    let args = Args::parse();
}
```
