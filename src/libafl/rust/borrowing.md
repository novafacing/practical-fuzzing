# Borrowing Mutably and Immutably

A *reference* is denoted with the `&` symbol, so a `String` is the actual string instance,
while an `&String` is a *reference* to the string instance. Like variables, *references*
can be mutable or immutable, and mutable references are written `&mut`. These are also
sometimes called *mutable borrows*, because *borrow* and *reference* are used somewhat
interchangeably in Rust literature.

We'll change our `foo` function to take a *mutable reference* to a `String` instead of
taking ownership of its parameter. This has another side effect of making the `clone()`
call unnecessary. Because we are passing a *reference* as a parameter, we need to
*dereference* the variable in order to modify it. We also need to change how we pass in
the parameter to `foo`. Instead of just passing `x`, which is the variable `x`, we want
to pass a *mutable reference* to `x`, so we write `&mut x`. `println!` doesn't need us to
dereference `x` for it, because a `&String`, `&mut String`, and `String` can all be
`Display`-ed with the same code.

```rust
fn foo(bar: &mut String) {
    *bar += " I'm here!";
    println!("Got {}", bar);
}

fn main() {
    let mut x: String = "Hello, World!".to_string();
    foo(&mut x);
    println!("X is {}", x);
}
```

<center>
<a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+foo%28bar%3A+%26mut+String%29+%7B%0A++++*bar+%2B%3D+%22+I%27m+here%21%22%3B%0A++++println%21%28%22Got+%7B%7D%22%2C+bar%29%3B%0A%7D%0A%0Afn+main%28%29+%7B%0A++++let+mut+x%3A+String+%3D+%22Hello%2C+World%21%22.to_string%28%29%3B%0A++++foo%28%26mut+x%29%3B%0A++++println%21%28%22X+is+%7B%7D%22%2C+x%29%3B%0A%7D">Playground</a>
</center>
<br>

We can run this now in the Playground and see that we print out the modified message
*both* times:

```
Got Hello, World! I'm here!
X is Hello, World! I'm here!
```

Hopefully you have a rough understanding of what it means to *own*, *reference*, *borrow*,
and *dereference* in Rust. Remember up [above](#ownership-and-moves) when we brushed
past using `.to_string()` to convert a `&str` to a `String`? The `.to_string()` method
is implemented for `str` references and returns a `String`!
