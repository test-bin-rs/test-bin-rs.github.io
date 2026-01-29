//! This crate is used by `src/main.rs` and `examples/main.rs`
//!
//! ```
//! assert_true::test();
//! ```
//!

mod core;

pub fn test() {
    core::test();
}

#[cfg(test)]
#[test]
fn lib_unit_test() {
    core::test();
}
