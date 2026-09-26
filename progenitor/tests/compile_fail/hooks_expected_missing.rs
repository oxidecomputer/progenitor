// With `hooks = Expected`, omitting `impl ClientHooks for Client` must be a
// compile-time error that names the missing impl.
//
// trybuild builds this file as its own crate under
// `target/tests/trybuild/progenitor`, so the spec path is relative to that.
mod client {
    progenitor::generate_api!(
        spec = "../../../../progenitor/tests/compile_fail/no-operations.json",
        hooks = Expected,
    );
}

fn main() {}
