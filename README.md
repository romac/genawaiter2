# genawaiter2

[![crate-badge]][crate-link] [![docs-badge]][docs-link] [![ci-badge]][ci-link]

[crate-badge]: https://img.shields.io/crates/v/genawaiter2.svg
[crate-link]: https://crates.io/crates/genawaiter2
[docs-badge]: https://docs.rs/genawaiter2/badge.svg
[docs-link]: https://docs.rs/genawaiter2
[ci-badge]: https://github.com/romac/genawaiter2/workflows/CI/badge.svg
[ci-link]: https://github.com/romac/genawaiter2/actions

This crate is a fork of [`genawaiter`][genawaiter] a crate which implements stackless generators (aka coroutines) in stable Rust. Instead of using `yield`, which [won't be stabilized anytime soon][yield-unstable], you use `async`/`await`, which is stable today.

[genawaiter]: https://github.com/whatisaphone/genawaiter
[yield-unstable]: https://doc.rust-lang.org/nightly/unstable-book/language-features/generators.html

Features:

- Supports resume arguments and completion values
- Supports async generators (e.g., `Stream`s)
- Allocation-free
- No runtime dependencies
  - No compile-time dependencies either, with `default-features = false`
- Built on top of standard language constructs, which means there are no platform-specific shenanigans

Example:

```rust
let odd_numbers_less_than_ten = gen!({
    let mut n = 1;
    while n < 10 {
        yield_!(n); // Suspend a function at any point with a value.
        n += 2;
    }
});

// Generators can be used as ordinary iterators.
for num in odd_numbers_less_than_ten {
    println!("{}", num);
}
```

Result:

```text
1
3
5
7
9
```

And here is the same generator, this time without macros. This is how you do things with `default-features = false` (which eliminates the proc macro dependencies).

```rust
let odd_numbers_less_than_ten = Gen::new(|co| async move {
    let mut n = 1;
    while n < 10 {
        co.yield_(n).await;
        n += 2;
    }
});
```

[See the docs for more.](https://docs.rs/genawaiter2)

## Development

### Install prerequisites

- [Rust]
- [pre-commit]

[Rust]: https://www.rust-lang.org/
[pre-commit]: https://pre-commit.com/

### Install the pre-commit hook

```sh
pre-commit install
```

This installs a Git hook that runs a quick sanity check before every commit.

### Run the app

```sh
cargo run
```

### Run the tests

```sh
cargo test
```
