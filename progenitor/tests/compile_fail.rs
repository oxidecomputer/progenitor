// Copyright 2026 Oxide Computer Company

//! Compile-fail tests for `generate_api!`.

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}
