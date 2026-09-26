// Copyright 2026 Oxide Computer Company

//! Compile-fail tests for `generate_api!`.
//!
//! The expected output tracks the current stable compiler; CI runs these
//! tests only on stable (see the trybuild job in `.github/workflows/rust.yml`).

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}
