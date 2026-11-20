//! Rust types generated for embedded targets from the shared JSON test bundle.
//!
//! The crate is `no_std`, so building it for a target without an allocator
//! checks that the generated code needs neither `std` nor `alloc`.

#![no_std]

sidex_embedded::include_bundle!(pub json_test_bundle as generated);
