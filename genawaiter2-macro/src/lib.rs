#![no_std]
#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![warn(clippy::cargo, clippy::pedantic)]

#[macro_export]
#[cfg(feature = "proc_macro")]
macro_rules! stack_let_gen {
    ($name:ident, $body:expr $(,)?) => {
        ::genawaiter2::stack::let_gen_using!($name, ::genawaiter2::stack_producer!($body),);
    };
}

#[macro_export]
macro_rules! stack_let_gen_using {
    ($name:ident, $producer:expr $(,)?) => {
        // Safety: The goal here is to ensure the safety invariants of `Gen::new`, i.e.,
        // the lifetime of the `Co` argument (in `$producer`) must not outlive `shelf`
        // or `generator`.
        //
        // We create two variables, `shelf` and `generator`, which cannot be named by
        // user-land code (because of macro hygiene). Because they are declared in the
        // same scope, and cannot be dropped before the end of the scope (because they
        // cannot be named), they have equivalent lifetimes. The type signature of
        // `Gen::new` ties the lifetime of `co` to that of `shelf`. This means it has
        // the same lifetime as `generator`, and so the invariant of `Gen::new` cannot
        // be violated.
        let mut shelf = ::genawaiter2::stack::Shelf::new();
        let mut generator = unsafe { ::genawaiter2::stack::Gen::new(&mut shelf, $producer) };
        let $name = &mut generator;
    };
}

#[macro_export]
#[cfg(feature = "proc_macro")]
macro_rules! rc_gen {
    ($body:expr) => {
        ::genawaiter2::rc::Gen::new(::genawaiter2::rc_producer!($body))
    };
}

#[macro_export]
#[cfg(feature = "proc_macro")]
macro_rules! sync_gen {
    ($body:expr) => {
        ::genawaiter2::sync::Gen::new(::genawaiter2::sync_producer!($body))
    };
}
