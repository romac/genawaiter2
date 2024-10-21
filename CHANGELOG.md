# Changelog

## v0.99.1 – 2020-03-08

### Fixed

- Compile error due to missing features ([#12](https://github.com/whatisaphone/genawaiter/issues/12))

## v0.99.0 – 2020-02-16

### Added

- Proc macros `gen!`, `yield_!`, and friends (Thanks [@DevinR528](https://github.com/DevinR528)!)
- Example showing error propagation

### Deprecated

- `generator_mut!` and `unsafe_create_generator!` macros. Use `let_gen_using!` instead.

## v0.2.2 – 2019-11-25

### Added

- Implement `Stream` for generators. (requires opting in to the dependency with the feature `stream`)

## v0.2.1 – 2019-11-11

### Added

- `generator_mut!`, a safe wrapper on top of `unsafe_create_generator!`.
- A version of `Gen` which can be shared between threads.
- `Co` now detects when it's used after its generator has completed, and panics (in debug builds).

## v0.2.0 – 2019-11-07

### Added

- Support for resume arguments, via `Coroutine` and `resume_with`.
- The backing state of stack-based generators is now public (`Shelf`), so you can avoid using macros if you wish.

### Changed

- Improved panic messages (in debug builds) which try to teach correct usage of the library.
- Stack-based generators are now "less unsafe". The lifetime of `co` is now bound by the lifetime of the generator's state, instead of `'static`. It's not fully safe yet, but it's much better.
- Improved the docs.
- Moved CI from GitLab to GitHub Actions.
