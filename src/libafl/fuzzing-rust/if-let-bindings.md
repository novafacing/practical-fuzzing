### `if let` Bindings

The first concept is `if let` bindings, which you can read about
[here](https://doc.rust-lang.org/book/ch06-03-if-let.html). This control flow construct
allows us to check if the value of an expression *matches* some pattern, and bind it to
a new variable if so. We can have an `else` branch on this `if let` statement as well.

We can check out a brief demo of this pattern on both `Option`s and `Result`s. In the
code below, we have two `Option`s and two `Result`s. The first option is `None`, while
the second is `Some(T)` (where `T` is `String` in this case). We can pattern match both
of these variables using `if let` to bind them to a new variable if they match the
pattern (`Some(_)`) that we are checking for. Notice in the first check, we bind `x` to
a new variable `x`, and in the second we bind `x` to a new variable `e`. We can do the
same for `Result`.

```rust
fn main() {
  let x: Option<String> = None;
  let y: Option<String> = Some("Hello, World!".to_string());

  if let Some(x) = x {
      println!("x is some: {}!", x);
  } else {
      println!("x is none");
  }

  if let Some(e) = y {
      println!("y is some: {}!", e);
  } else {
      println!("y is none");
  }

  let w: Result<String, String> = Err("There was something wrong...".into());
  let z: Result<String, String> = Ok("Hello, World!".to_string());

  if let Ok(z) = z {
      println!("z was ok: {}!", z);
  } else {
      println!("z was not ok");
  }

  if let Err(e) = w {
      println!("w was err: {}!", e);
  } else {
      println!("w was ok");
  }
}
```

<center>
<a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn+main%28%29+%7B%0A++++let+x%3A+Option%3CString%3E+%3D+None%3B%0A++++let+y%3A+Option%3CString%3E+%3D+Some%28%22Hello%2C+World%21%22.to_string%28%29%29%3B%0A++++%0A++++if+let+Some%28x%29+%3D+x+%7B%0A++++++++println%21%28%22x+is+some%3A+%7B%7D%21%22%2C+x%29%3B%0A++++%7D+else+%7B%0A++++++++println%21%28%22x+is+none%22%29%3B%0A++++%7D%0A++++%0A++++if+let+Some%28y%29+%3D+y+%7B%0A++++++++println%21%28%22y+is+some%3A+%7B%7D%21%22%2C+y%29%3B%0A++++%7D+else+%7B%0A++++++++println%21%28%22y+is+none%22%29%3B%0A++++%7D%0A%7D">Playground</a>
</center>
<br>

In the first check in our code, we checked if `rest` is `Some` (`Option<T>` values
in rust are either `Some(T)` or `None`). If so, we bind it to a new variable (also
called `rest`, there is no need to choose a new name, this newly bound variable has a
smaller scope). We use the `if let` pattern again, to check if we have at least 2
characters at the beginning of `rest`:

```rust
if let Some(rest) = rest {
    if let Some(&[first, second]) = rest.get(0..2) {
```
