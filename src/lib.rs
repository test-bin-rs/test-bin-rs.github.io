//! ```
//! assert_true::test();
//! ```
//!
//! This crate is used by `src/main.rs` for the `assert_true` bin
//! that you can run via
//! ```text
//! cargo install \
//!     --git https://github.com/test-bin-rs/test-bin-rs.github.io \
//!     --branch assert-true \
//!     --root /tmp/.cargo 
//! /tmp/.cargo/bin/assert-true
//! ```
//!
//! (Read it carefully before you copy and paste to run it on your computer.) 
//!
//! Then you can delete the `/tmp/.cargo` directory via `rm -rf /tmp/.cargo` 	
//!
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
