# Progenitor

Progenitor is a Rust crate for generating opinionated clients from API
descriptions specified in the OpenAPI 3.0.x format. It makes use of Rust
futures for async API calls and `Streams` for paginated interfaces.

It generates a type called `Client` with methods that correspond to the
operations specified in the OpenAPI document.

## Using Progenitor

There are three different ways of using the `progenitor` crate. The one you
choose will depend on your use case and preferences.

### Macro

The simplest way to use Progenitor is via its `generate_api!` macro.

In a source file (often `main.rs`, `lib.rs`, or `mod.rs`) simply invoke the
macro:

```rust
generate_api("path/to/openapi_document.json");
```

You'll need to add add the following to `Cargo.toml`:

```diff
[dependencies]
+progenitor = { git = "https://github.com/oxidecomputer/progenitor" }
+reqwest = { version = "0.11", features = ["json", "stream"] }
+serde = { version = "1.0", features = ["derive"] }
```

In addition, if the OpenAPI document contains string types with the `format`
field set to `date` or `date-time`, include

```diff
[dependencies]
+chrono = { version = "0.4", features = ["serde"] }
```

Similarly if there is a `format` field set to `uuid`:

```diff
[dependencies]
+uuid = { version = "0.8", features = ["serde", "v4"] }
```

Note that the macro will be re-evaluated when the OpenAPI json document
changes (when its mtime is updated).

### Builder

Progenitor includes an interface appropriate for use in a
[`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
file. While slightly more onerous than the macro, a builder has the advantage of making the generated code visible.

The `build.rs` file should look something like this:

```rust
fn main() {
    let src = "../sample_openapi/keeper.json";
    println!("cargo:rerun-if-changed={}", src);
    let file = File::open(src).unwrap();
    let spec = serde_json::from_reader(file).unwrap();
    let mut generator = progenitor::Generator::new();

    let content = generator.generate_text(&spec).unwrap();

    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("codegen.rs");

    fs::write(out_file, content).unwrap();
}
```

In a source file (often `main.rs`, `lib.rs`, or `mod.rs`) include the generated
code:

```rust
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
```

You'll need to add add the following to `Cargo.toml`:

```diff
[dependencies]
+progenitor-client = { git = "https://github.com/oxidecomputer/progenitor" }
+reqwest = { version = "0.11", features = ["json", "stream"] }
+serde = { version = "1.0", features = ["derive"] }

[build-dependencies]
+progenitor = { git = "https://github.com/oxidecomputer/progenitor" }
+serde_json = "1.0"
```

(`chrono` and `uuid` as above)

Note that `progenitor` is used by `build.rs`, but the generated code required
`progenitor-client`.


### Static Crate

Progenitor can be run to emit a stand-alone crate for the generated client.
This ensures no unexpected changes (e.g. from updates to progenitor). It is
however, the most manual way to use Progenitor.

Usage:

```
progenitor

Options:
    -i INPUT            OpenAPI definition document (JSON)
    -o OUTPUT           Generated Rust crate directory
    -n CRATE            Target Rust crate name
    -v VERSION          Target Rust crate version
```

For example:

`cargo run --bin progenitor -- -i sample_openapi/keeper.json -o keeper -n keeper -v 0.1.0`

This will produce a package in the specified directory. The output has no
persistent dependency on Progenitor including the `progenitor-client` crate.
Here's a excerpt from the emitted `Cargo.toml`:

```toml
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
futures = "0.3"
percent-encoding = "2.1"
reqwest = { version = "0.11", features = ["json", "stream"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "0.8", features = ["serde", "v4"] }
```

Note that there is a dependency on `percent-encoding` which macro- and
build.rs-generated clients is included from `progenitor-client`.
